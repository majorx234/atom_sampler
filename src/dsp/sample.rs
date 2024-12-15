use crate::error::{Error, Result};
use std::fs::File;
use std::path::{Path, PathBuf};
use wav::{bit_depth::BitDepth, read};
struct Sample {
    data: Vec<f32>,
    path: Option<Box<Path>>,
    gain: f32,
    speed: f32,
    pan: f32,
}

impl Sample {
    fn new() -> Self {
        Sample {
            data: Vec::new(),
            path: None,
            gain: 1.0f32,
            speed: 1.0f32,
            pan: 0.5f32,
        }
    }
    fn load_from_wav() -> Result<Self> {
        let mut inp_file = File::open(Path::new("data/sine.wav"))?;
        let (header, data) = wav::read(&mut inp_file)?;
        match data {
            BitDepth::ThirtyTwoFloat(data) => Ok(Sample {
                data: Vec::new(),
                path: None,
                gain: 1.0f32,
                speed: 1.0f32,
                pan: 0.5f32,
            }),
            _ => Err(Error::IoWrongDatatyp),
        }
    }
}
