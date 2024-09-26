use crate::Pcg32;
mod vec;
mod hashmap;

pub trait RemoveRandomItem<T>{
    fn remove_random_item(&mut self, rng:&mut Pcg32) -> Option<T>;

}