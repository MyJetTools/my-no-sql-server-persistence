use std::time::Duration;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::data_to_persist::DataToPersist;

pub struct PersistMarkers {
    pub data_to_persist: DataToPersist,
    pub persist_duration: Vec<usize>,
    pub last_persist_time: Option<DateTimeAsMicroseconds>,
}

impl PersistMarkers {
    pub fn new() -> Self {
        Self {
            data_to_persist: DataToPersist::new(),
            persist_duration: Vec::new(),
            last_persist_time: None,
        }
    }

    pub fn add_persist_duration(&mut self, dur: Duration) {
        self.persist_duration.push(dur.as_micros() as usize);

        while self.persist_duration.len() > 120 {
            self.persist_duration.remove(0);
        }

        self.last_persist_time = DateTimeAsMicroseconds::now().into();
    }
}
