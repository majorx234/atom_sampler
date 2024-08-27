#[derive(PartialEq)]
pub enum Type {
    Recording,
    ChangeStartAdress(usize),
    ChangeEndAdress(usize),
    Playback,
}

pub struct AtomEvent {
    pub event_type: Type,
    pub start: bool,
}
