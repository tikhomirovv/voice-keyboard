use crate::app::get_local_data_dir;
use crate::modules::audio::SampleType;
use crate::utils::get_current_timestamp;
use anyhow::Result;
use hound::{WavSpec, WavWriter};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

const BITS_PER_SAMPLE: u16 = 8;

pub struct AudioFileWriter {
    writer: WavWriter<BufWriter<File>>,
}

impl AudioFileWriter {
    pub fn create(id: String, sample_rate: u32) -> Result<Self> {
        let path = Self::generate_wav_path(id)?;
        let writer = WavWriter::create(
            &path,
            WavSpec {
                channels: 1,
                sample_rate, // Частота дискретизации
                bits_per_sample: BITS_PER_SAMPLE,
                sample_format: hound::SampleFormat::Int,
            },
        )?;

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

    fn generate_wav_path(id: String) -> Result<PathBuf> {
        let timestamp = get_current_timestamp();
        let wav_path = format!("{}/{}_{}.wav", "records", timestamp, id);
        let full_path = get_local_data_dir(&wav_path)?;
        let path = PathBuf::from(full_path);
        // Создаем директорию для записей если её нет
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent)?;
        }
        println!("path: {}", path.display());
        Ok(path)
    }
}
