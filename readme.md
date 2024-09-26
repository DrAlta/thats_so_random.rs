    /// Generate a random number from a Gaussian distribution.
    pub fn gaussian_distribution<T>(&mut self, mu: T, sigme: T) -> T

    /// Implement `next_u64` via `next_u32`, little-endian order.
    pub fn next_u64(&mut self) -> u64 {

    /// Implement `next_usize` via `next_u32`, little-endian order.
    pub fn next_usize(&mut self) -> usize {

    /// Generate a random number from a normal distribution.
    pub fn normal_distribution<T>(&mut self) -> T 

    /// Generate a random number.
    /// floats are between 0.0 and 1.0
    /// integers are be T::MIN and T::MAX
    pub fn random<T>(&mut self) -> T 

    /// Remove and return a random item from a collection.
    pub fn remove_random_item<T, C>(&mut self, vec: &mut C) -> Option<T> {

    // return a random item from a collection
    pub fn get_random_item<'a, 'b, T, C>(&'a mut self, collection: &'b C) -> Option<&'b T> {

    /// Generate a random number within a specified range.
    pub fn random_range<T>(&mut self, low: T, high : T) -> T 
