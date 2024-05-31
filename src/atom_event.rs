enum Type {
    Recording,
    ChangeStartAdress,
    ChangeEndAdress,
    Playback,
}

struct AtomEvent {
    event_type: Type,
}
