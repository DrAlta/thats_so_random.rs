use super::Pcg32;

pub trait GaussianDistribution<T>{
    fn gaussian_distribution(&mut self, mu: T, sigme: T) -> T;
}

impl GaussianDistribution<f32> for Pcg32 {
    fn gaussian_distribution(&mut self, mu: f32, sigme: f32) -> f32 {
        (self.normal_distribution::<f32>() * sigme) + mu
    }
}
impl GaussianDistribution<f64> for Pcg32 {
    fn gaussian_distribution(&mut self, mu: f64, sigme: f64) -> f64 {
        (self.normal_distribution::<f64>() * sigme) + mu
    }
}
