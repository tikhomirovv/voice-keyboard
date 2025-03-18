use crate::modules::audio::SampleType;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    FromSample, Sample, SizedSample,
};
// use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

// // Explicitly implement Send and Sync
unsafe impl Send for RecordingSession {}
unsafe impl Sync for RecordingSession {}

const BUFFER_SIZE: usize = 48000 * 5;

pub struct RecordingSession {
    pub id: String,
    sender: broadcast::Sender<Vec<SampleType>>,
    stream: Option<cpal::Stream>,
}

impl RecordingSession {
    pub fn new() -> Self {
        let id = Uuid::new_v4().to_string();
        let (sender, _) = broadcast::channel::<Vec<SampleType>>(BUFFER_SIZE);
        Self {
            id,
            sender,
            stream: None,
        }
    }

    pub fn start(&mut self, device: &cpal::Device) -> Result<(), anyhow::Error> {
        let config = device.default_input_config()?;
        let sample_format = config.sample_format();
        let sender = self.sender.clone();

        let stream = match sample_format {
            cpal::SampleFormat::I8 => self.build_typed_input_stream::<i8>(device, sender)?,
            cpal::SampleFormat::I16 => self.build_typed_input_stream::<i16>(device, sender)?,
            cpal::SampleFormat::I32 => self.build_typed_input_stream::<i32>(device, sender)?,
            cpal::SampleFormat::F32 => self.build_typed_input_stream::<f32>(device, sender)?,
            format => return Err(anyhow::anyhow!("Неподдерживаемый формат: {format}")),
        };
        stream.play()?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            // println!("Stream stopped and resources freed");
            drop(stream);
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Vec<SampleType>> {
        self.sender.subscribe()
    }

    fn build_typed_input_stream<T>(
        &self,
        device: &cpal::Device,
        sender: broadcast::Sender<Vec<SampleType>>,
    ) -> Result<cpal::Stream, anyhow::Error>
    where
        T: Sample + Send + SizedSample + 'static,
        SampleType: Sample + FromSample<T>,
    {
        let config = device.default_input_config()?;

        let err_fn = move |err| {
            eprintln!("Ошибка потока: {}", err);
            println!("Запись остановлена из-за ошибки потока");
        };
        // static COUNTER: AtomicUsize = AtomicUsize::new(0);

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[T], _| {
                // Преобразуем весь чанк
                let samples: Vec<SampleType> = data
                    .iter()
                    .map(|&sample| SampleType::from_sample(sample))
                    .collect();

                // Отправляем весь чанк целиком
                if let Err(e) = sender.send(samples) {
                    eprintln!("Ошибка отправки данных: {}", e);
                }
            },
            err_fn,
            None,
        )?;
        Ok(stream)
    }
}

impl Drop for RecordingSession {
    fn drop(&mut self) {
        self.stop();
        println!(
            "RecordingSession {} is being dropped. Cleaning up resources.",
            self.id
        );
    }
}
