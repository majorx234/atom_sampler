#[derive(PartialEq, Clone)]
pub enum Type {
    Recording,
    ChangeStartAdress(usize),
    ChangeEndAdress(usize),
    Playback,
}

#[derive(Clone)]
pub struct AtomEvent {
    pub event_type: Type,
    pub start: bool,
}
