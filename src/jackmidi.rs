const MAX_MIDI: usize = 3;

pub trait MidiMsgBase: Send + std::fmt::Display {
    fn type_of(&self) -> &str;
    fn get_data(&self) -> Vec<u8>;
    fn get_id(&self) -> u16;
    fn get_value(&self) -> u16;
    fn get_time(&self) -> u64;
}

//a fixed size container to copy data out of real-time thread
#[derive(Copy, Clone)]
pub struct MidiMsgGeneric {
    pub len: usize,
    pub data: [u8; MAX_MIDI],
    pub time: u64,
}

impl MidiMsgBase for MidiMsgGeneric {
    fn type_of(&self) -> &str {
        "MidiMsgGeneric"
    }
    fn get_data(&self) -> Vec<u8> {
        self.data.into_iter().collect()
    }
    fn get_id(&self) -> u16 {
        u16::MAX
    }
    fn get_value(&self) -> u16 {
        0
    }
    fn get_time(&self) -> u64 {
        self.time
    }
}

impl std::fmt::Debug for MidiMsgGeneric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MidiGeneric: time: {}, len: {}, data: {:?}",
            self.time,
            self.len,
            &self.data[..self.len]
        )
    }
}

impl std::fmt::Display for MidiMsgGeneric {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MidiGeneric: time: {}, len: {}, data: {:?}",
            self.time,
            self.len,
            &self.data[..self.len]
        )
    }
}

impl From<jack::RawMidi<'_>> for MidiMsgGeneric {
    fn from(midi: jack::RawMidi<'_>) -> MidiMsgGeneric {
        let mut data: [u8; MAX_MIDI] = [0, 0, 0];
        data[..MAX_MIDI].copy_from_slice(&midi.bytes[..MAX_MIDI]);
        MidiMsgGeneric {
            len: MAX_MIDI,
            data,
            time: midi.time as u64 + jack::get_time(),
        }
    }
}
