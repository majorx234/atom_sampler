#[derive(PartialEq)]
pub enum Type {
    Recording,
    ChangeStartAdress,
    ChangeEndAdress,
    Playback,
}

pub struct AtomEvent {
    pub event_type: Type,
    pub start: bool,
}
