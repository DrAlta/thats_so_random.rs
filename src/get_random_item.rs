use crate::Pcg32;
mod vec;
mod hashmap;

pub trait GetRandomItem<'a, T>{
    fn get_random_item<'b>(&'a self, rng:&'b mut Pcg32) -> Option<T>;
}