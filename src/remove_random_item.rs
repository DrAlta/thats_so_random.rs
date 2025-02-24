mod hashmap;
mod vec;

pub trait RemoveRandomItem<T, R> {
    fn remove_random_item(&mut self, rng: &mut R) -> Option<T>;
}
