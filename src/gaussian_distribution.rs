use crate::{normal_distribution::NormalDistribution, RandomNumberGenerator};

pub trait GaussianDistribution<T> {
    fn gaussian_distribution(&mut self, mu: T, sigme: T) -> T;
}

impl<T: RandomNumberGenerator> GaussianDistribution<f32> for T
where
    T: NormalDistribution<f32>,
{
    fn gaussian_distribution(&mut self, mu: f32, sigme: f32) -> f32 {
        (NormalDistribution::normal_distribution(self) * sigme) + mu
    }
}
impl<T: RandomNumberGenerator> GaussianDistribution<f64> for T
where
    T: NormalDistribution<f64>,
{
    fn gaussian_distribution(&mut self, mu: f64, sigme: f64) -> f64 {
        (NormalDistribution::normal_distribution(self) * sigme) + mu
    }
}
