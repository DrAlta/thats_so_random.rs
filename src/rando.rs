use super::Pcg32;

pub trait Rando<T> {
    fn random(&mut self) -> T;
}

impl Rando<i32> for Pcg32 {
    fn random(&mut self) -> i32 {
        u32_to_i32(self.next_u32())
    }
}


impl Rando<u32> for Pcg32 {
    fn random(&mut self) -> u32 {
        self.next_u32()
    }
}

impl Rando<i64> for Pcg32 {
    fn random(&mut self) -> i64 {
       u64_to_i64(self.next_u64())
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

////
const I32_MAX: u32 = i32::MAX as u32; 
fn u32_to_i32(x:u32) -> i32 {
    if x == u32::MAX {
        return i32::MIN
    }
    if x > I32_MAX {
        -1 - (( x - I32_MAX - 1) as i32)
    } else {
        x as i32
    }

}
const I64_MAX: u64 = i64::MAX as u64; 
fn u64_to_i64(x:u64) -> i64 {
    if x > I64_MAX {
        -1 - (( x - I64_MAX - 1) as i64)
    } else {
        x as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_to_i32_test(){
        assert_eq!(i32::MIN , u32_to_i32(u32::MAX));
    }
    #[test]
    fn u64_to_i64_test(){
        assert_eq!(i64::MIN, u64_to_i64(u64::MAX));
    }
}