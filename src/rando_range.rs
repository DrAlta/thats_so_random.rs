use crate::RandomNumberGenerator;

pub trait RandoRange<T> {
    fn random_range(&mut self, low: T, high: T) -> T;
}
impl<T: RandomNumberGenerator> RandoRange<i32> for T {
    fn random_range(&mut self, low: i32, high: i32) -> i32 {
        (self.next_u32() % i32::MAX as u32) as i32 % (high - low) + low
    }
}
impl<T: RandomNumberGenerator> RandoRange<u32> for T {
    fn random_range(&mut self, low: u32, high: u32) -> u32 {
        (self.next_u32() % (high - low)) + low
    }
}
impl<T: RandomNumberGenerator> RandoRange<f64> for T {
    fn random_range(&mut self, low: f64, high: f64) -> f64 {
        let diff = high - low;
        low + (diff * self.random::<f64>())
    }
}

impl<T: RandomNumberGenerator> RandoRange<usize> for T {
    fn random_range(&mut self, low: usize, high: usize) -> usize {
        (self.next_usize() % (high - low)) + low
    }
}
