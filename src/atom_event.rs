#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Recording(bool),
    ChangeStartAdress(usize),
    ChangeEndAdress(usize),
    Playback(bool),
}

#[derive(Clone, Debug)]
pub struct AtomEvent {
    pub channel: usize,
    pub event_type: Type,
    pub start: bool,
}
