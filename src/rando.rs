use super::Pcg32;

pub trait Rando<T> {
    fn random(&mut self) -> T;
}

const I32_MAX: u32 = i32::MAX as u32; 
impl Rando<i32> for Pcg32 {
    fn random(&mut self) -> i32 {
        let x = self.next_u32();
        if x > I32_MAX {
            0 - (( x - I32_MAX) as i32)
        } else {
            x as i32
        }
    }
}

impl Rando<u32> for Pcg32 {
    fn random(&mut self) -> u32 {
        self.next_u32()
    }
}

const I64_MAX: u64 = i64::MAX as u64; 
impl Rando<i64> for Pcg32 {
    fn random(&mut self) -> i64 {
        let x = self.next_u64();
        if x > I64_MAX {
            0 - (( x - I64_MAX) as i64)
        } else {
            x as i64
        }
    }
}

impl Rando<u64> for Pcg32{
    fn random(&mut self) -> u64 {
        self.next_u64()
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
