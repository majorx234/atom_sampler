use std::path::PathBuf;

pub enum Commands {
    LOAD,
    PLAY,
    STARTADRESS(usize),
    ENDADDRESS(usize),
}

pub struct WaveCommands {
    pub command: Commands,
    pub path: PathBuf,
}
