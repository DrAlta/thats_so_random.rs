mod hashmap;
mod vec;

pub trait GetRandomItem<'a, T, R> {
    fn get_random_item<'b>(&'a self, rng: &'b mut R) -> Option<T>;
}
