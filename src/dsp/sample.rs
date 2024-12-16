use crate::error::{Error, Result};
use hound;
use std::fs::File;
use std::path::{Path, PathBuf};

struct Sample {
    data_left: Vec<f32>,
    data_right: Vec<f32>,
    path: Option<Box<Path>>,
    gain: f32,
    speed: f32,
    pan: f32,
}

impl Sample {
    fn new() -> Self {
        Sample {
            data_left: Vec::new(),
            data_right: Vec::new(),
            path: None,
            gain: 1.0f32,
            speed: 1.0f32,
            pan: 0.5f32,
        }
    }
    fn load_from_wav() -> Result<Self> {
        let mut reader = hound::WavReader::open("testsamples/pop.wav")?;
        let format = reader.spec().sample_format;
        let mut nsamples = reader.len() as usize;
        match format {
            hound::SampleFormat::Float => {
                let data: Vec<f32> = reader
                    .samples::<f32>()
                    .map(|x| x.unwrap())
                    .collect::<Vec<f32>>();
                Ok(Sample {
                    data_left: Vec::new(),
                    data_right: Vec::new(),
                    path: None,
                    gain: 1.0f32,
                    speed: 1.0f32,
                    pan: 0.5f32,
                })
            }
            _ => Err(Error::IoWrongDatatyp),
        }
    }
}
