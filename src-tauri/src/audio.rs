extern crate anyhow;
extern crate cpal;
extern crate serde;
extern crate serde_json;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// use serde_json::json;

fn get_device_hash(device_name: &str) -> String {
    let mut hasher = DefaultHasher::new();
    device_name.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:x}", hash)
}

#[derive(Serialize, Deserialize)]
struct AudioDevice {
    id: String,
    name: String,
}

pub fn get_microphones() -> Result<String, anyhow::Error> {
    let mut devices_list = Vec::new();
    let host = cpal::default_host();
    let devices = host.input_devices()?;

    for (_, device) in devices.enumerate() {
        let name = device.name().unwrap_or_else(|_| "Unknown".to_string());
        devices_list.push(AudioDevice {
            id: get_device_hash(&name),
            name,
        });
        println!("Input device: {}", device.name()?);
    }

    Ok(serde_json::to_string(&devices_list)?)
}

use cpal::{FromSample, Sample, SupportedStreamConfig};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

pub fn record(device_id: &str) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = if device_id != "" {
        host.input_devices()?.find(|x| {
            x.name()
                .map(|y| get_device_hash(&y) == device_id)
                .unwrap_or(false)
        })
    } else {
        host.default_input_device()
    }
    .expect("failed to find input device");

    println!("Input device: {}", device.name()?);

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // The WAV file we're recording to.
    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../recorded.wav");
    let spec = wav_spec_from_config(&config);
    println!("Recording to: {:?}", spec);

    let writer = hound::WavWriter::create(PATH, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32, i32>(data, &writer_2),
            err_fn,
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };

    stream.play()?;

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize()?;
    println!("Recording {} complete!", PATH);
    Ok(())
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: hound::SampleFormat::Int,
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
