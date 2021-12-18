use std::cmp::min;

#[derive(Default, Clone, Copy)]
pub struct Range {
    start: u64,
    offset: u64,
}

impl Range {
    pub fn new(start: u64, offset: u64) -> Self {
        Self { start, offset }
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn apply_filesize(&self, file_size: u64) -> Self {
        let start = min(file_size - 1, self.start);
        Self {
            start,
            offset: min(file_size - start, self.offset),
        }
    }
}
