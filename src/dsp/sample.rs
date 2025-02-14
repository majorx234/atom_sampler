use crate::error::{Error, Result};
use hound;
use std::path::PathBuf;

pub struct Sample {
    pub data_left: Vec<f32>,
    pub data_right: Vec<f32>,
    gain: f32,
    speed: f32,
    pan: f32,
}

impl Default for Sample {
    fn default() -> Self {
        Self::new()
    }
}

impl Sample {
    pub fn new() -> Self {
        Sample {
            data_left: Vec::new(),
            data_right: Vec::new(),
            gain: 1.0f32,
            speed: 1.0f32,
            pan: 0.5f32,
        }
    }

    pub fn load_from_data(&mut self, data_left: Vec<f32>, data_right: Vec<f32>) -> Result<()> {
        self.data_left = data_left;
        self.data_right = data_right;
        Ok(())
    }

    pub fn load_from_wav(&mut self, path: PathBuf) -> Result<()> {
        let mut reader = hound::WavReader::open(path)?;
        let format = reader.spec().sample_format;
        let nsamples = reader.len() as usize;
        match format {
            hound::SampleFormat::Float => {
                let data: Vec<f32> = reader
                    .samples::<f32>()
                    .map(|x| x.unwrap())
                    .collect::<Vec<f32>>();
                self.data_left = data.clone();
                self.data_right = data;
                Ok(())
            }
            _ => Err(Error::IoWrongDatatyp),
        }
    }
}
