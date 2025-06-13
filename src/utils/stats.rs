use leptos::prelude::{ReadSignal, WriteSignal, signal};

pub struct SignalPair<T> {
    pub read: ReadSignal<T>,
    pub write: WriteSignal<T>
}

pub fn signalpair<T: std::marker::Send + std::marker::Sync + 'static>(v: T) -> SignalPair<T>{
    let (read, write) = signal(v);
    return SignalPair {
        read: read,
        write: write
    }
}

pub struct Stats {
    // the current total time game has been unpaused
    pub active_time_ms: SignalPair<f64>,
    // Whether or not to increment active_time_ms
    pub paused: SignalPair<bool>,
    // how many days pass per millisecond
    pub gamedays_per_ms: SignalPair<f64>,
    // maximum lifespan in terms of game
    pub max_lifespan_gamedays: SignalPair<f64>,
}

pub fn stats() -> Stats {
    return Stats {
        paused: signalpair(false), // start unpaused
        active_time_ms: signalpair(0.0), // start with no active time
        gamedays_per_ms: signalpair(0.03), // ~1 month a second,
        max_lifespan_gamedays: signalpair(36500.0), // 100 years by default
    }
}