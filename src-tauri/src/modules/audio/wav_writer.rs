use crate::modules::audio::SampleType;
use anyhow::Result;
use hound::{WavSpec, WavWriter};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const BITS_PER_SAMPLE: u16 = 8;

pub struct AudioFileWriter {
    writer: WavWriter<BufWriter<File>>,
}

impl AudioFileWriter {
    /// Создает новый WAV файл с заданными параметрами
    pub fn create(id: String, sample_rate: u32) -> Result<Self> {
        let spec = WavSpec {
            channels: 1,
            sample_rate: sample_rate, // Стандартная частота дискретизации
            bits_per_sample: BITS_PER_SAMPLE, // Соответствует SampleType = i8
            sample_format: hound::SampleFormat::Int,
        };

        let wav_path = format!("{}/recordings/{}.wav", env!("CARGO_MANIFEST_DIR"), id); // Используем format вместо concat
        let path = Path::new(&wav_path); // Создаем путь к файлу
        println!("path: {}", wav_path);

        // Создаем директорию для записей если её нет
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent)?;
        }

        let writer = WavWriter::create(path, spec)?;

        Ok(Self { writer })
    }

    /// Записывает блок сэмплов в файл
    pub fn write_samples(&mut self, samples: &[SampleType]) -> Result<()> {
        for &sample in samples {
            self.writer.write_sample(sample)?;
        }
        Ok(())
    }

    /// Завершает запись и закрывает файл
    pub fn finalize(self) -> Result<()> {
        self.writer.finalize()?;
        Ok(())
    }
}
