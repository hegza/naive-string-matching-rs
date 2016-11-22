use time::{PreciseTime,Duration};

pub struct Measure {
    pub start: PreciseTime,
    end: Option<PreciseTime>,
    duration: Option<Duration>,
}

impl Measure {
    pub fn start() -> Measure {
        let start = PreciseTime::now();
        Measure{ start: start, end: None, duration: None }
    }

    pub fn stop(&mut self) -> Duration {
        self.end = Some( PreciseTime::now() );
        self.duration = Some( self.start.to(self.end.unwrap()) );
        self.duration.unwrap()
    }

    pub fn duration(&self) -> Duration {
        self.duration.unwrap()
    }
}

