#[derive(PartialEq, Clone)]
pub enum Type {
    Recording(bool),
    ChangeStartAdress(usize),
    ChangeEndAdress(usize),
    Playback(bool),
}

#[derive(Clone)]
pub struct AtomEvent {
    pub event_type: Type,
    pub start: bool,
}
