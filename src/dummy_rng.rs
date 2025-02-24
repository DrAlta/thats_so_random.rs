use crate::RandomNumberGenerator;

pub struct DummyRNG {
    pub values: Vec<u32>,
    pub idx: usize,
}

impl RandomNumberGenerator for DummyRNG {
    fn advance(&mut self, delta: u64) {
        self.idx = (self.idx + delta as usize) & self.values.len();
    }

    fn next_u32(&mut self) -> u32 {
        let ret = self.values.get(self.idx).cloned().unwrap_or(0);
        self.idx = (self.idx + 1) & self.values.len();
        ret
    }
}
impl DummyRNG {
    pub fn new<T: Into<Vec<u32>>>(values: T) -> Self {
        Self {
            values: values.into(),
            idx: 0,
        }
    }
}
