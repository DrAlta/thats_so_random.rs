use super::Pcg32;

pub trait NormalDistribution<T>{
    fn normal_distribution(&mut self) -> T;
}

impl NormalDistribution<f32> for Pcg32 {
    fn normal_distribution(&mut self) -> f32 {
        let u1 = self.random();
        let u2: f32 = self.random();
        f32::sqrt(-2.0 * f32::ln(u1)) * f32::cos(2.0 * std::f32::consts::PI * u2)
    }
}

impl NormalDistribution<f64> for Pcg32 {
    fn normal_distribution(&mut self) -> f64 {
        let u1 = self.random();
        let u2: f64 = self.random();
        f64::sqrt(-2.0 * f64::ln(u1)) * f64::cos(2.0 * std::f64::consts::PI * u2)
    }
}