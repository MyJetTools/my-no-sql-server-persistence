use std::collections::VecDeque;

use super::PersistEvent;

pub struct PersistQueue {
    pub data: VecDeque<PersistEvent>,
}

impl PersistQueue {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, event: PersistEvent) {
        self.data.push_back(event);
    }

    pub fn get(&mut self, max_amount: usize) -> Vec<PersistEvent> {
        let mut result = Vec::new();

        while result.len() < max_amount {
            match self.data.pop_front() {
                Some(element) => result.push(element),
                None => break,
            }
        }

        result
    }
}
