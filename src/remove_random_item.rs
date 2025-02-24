mod vec;
mod hashmap;

pub trait RemoveRandomItem<T, R>{
    fn remove_random_item(&mut self, rng:&mut R) -> Option<T>;

}