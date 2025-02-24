use std::u16;

mod dummy_rng;
pub use dummy_rng::DummyRNG;
mod gaussian_distribution;
use gaussian_distribution::GaussianDistribution;
pub mod get_random_item;
use get_random_item::GetRandomItem;
mod normal_distribution;
use normal_distribution::NormalDistribution;

mod pcg32;
pub use pcg32::Pcg32;
mod rando;
pub use rando::Rando;
mod rando_range;
pub use rando_range::RandoRange;
pub mod remove_random_item;
use remove_random_item::RemoveRandomItem;

/// The default state for the PCG32 generator.
pub const DEFAULT_STATE:u64 = 0xcafef00dd15ea5e5;
/// The default stream for the PCG32 generator.
pub const DEFAULT_STREAM:u64 = 0xa02bdbf7bb3c0a7;


pub trait RandomNumberGenerator{
    fn advance(&mut self, delta: u64);
    fn next_u32(&mut self) -> u32;
   /// Generate a random number from a Gaussian distribution.
    fn gaussian_distribution<T>(&mut self, mu: T, sigme: T) -> T
    where Self: GaussianDistribution<T>
    {
        GaussianDistribution::<T>::gaussian_distribution(self, mu, sigme)
    }

    /// Implement `next_u64` via `next_u32`, little-endian order.
    fn next_u64(&mut self) -> u64 {
        // Use LE; we explicitly generate one value before the next.
        let x = u64::from(self.next_u32());
        let y = u64::from(self.next_u32());
        (y << 32) | x
    }

    /// Implement `next_usize` via `next_u32`, little-endian order.
    fn next_usize(&mut self) -> usize {
        // Use LE; we explicitly generate one value before the next.
        let x = self.next_u32();
        let y = u64::from(self.next_u32());
        let z = (y << 32) | u64::from(x);
        if let Ok(thing) =usize::try_from(z) {
            return thing;
        }
        if let Ok(thing) =usize::try_from(x) {
            return thing;
        }
        usize::from((x % u16::MAX as u32) as u16 )

    }

    /// Generate a random number from a normal distribution.
    fn normal_distribution<T>(&mut self) -> T 
    where Self: NormalDistribution<T>
    {
        NormalDistribution::<T>::normal_distribution(self)

    }
     /// Generate a random number.
     /// floats are between 0.0 and 1.0
     /// integers are be T::MIN and T::MAX
    fn random<T>(&mut self) -> T 
    where Self: Rando<T>
    {
        Rando::<T>::random(self)

    }

    /// Remove and return a random item from a vector.
    fn remove_random_item<T, C:RemoveRandomItem<T, Self>>(&mut self, collection: &mut C) -> Option<T> where Self: Sized {
        collection.remove_random_item(self)
    }


    fn get_random_item<'a, 'b, T, C: GetRandomItem<'b, T, Self>>(&'a mut self, collection: &'b C) -> Option<T> where Self: Sized{
        collection.get_random_item(self)
    }


    /// Generate a random number within a specified range.
    fn random_range<T>(&mut self, low: T, high : T) -> T 
    where Self: RandoRange<T>
    {
        RandoRange::<T>::random_range(self, low, high)

    }
}

