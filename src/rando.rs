use super::Pcg32;

pub trait Rando<T> {
    fn random(&mut self) -> T;
}

impl Rando<i32> for Pcg32 {
    fn random(&mut self) -> i32 {
        (self.next_u32() % i32::MAX as u32) as i32
    }
}
impl Rando<u32> for Pcg32 {
    fn random(&mut self) -> u32 {
        self.next_u32()
    }
}
impl Rando<f32> for Pcg32{
    fn random(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }
}
impl Rando<f64> for Pcg32{
    fn random(&mut self) -> f64 {
        self.next_u32() as f64 / u32::MAX as f64
    }
}
