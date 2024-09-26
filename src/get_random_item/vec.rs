use super::GetRandomItem;

impl<'a, T> GetRandomItem<'a, &'a T> for Vec<T> {
    fn get_random_item(&'a self, rng:& mut crate::Pcg32) -> Option<&'a T> {
        if self.is_empty() {
            return None
        }
        self.get(rng.random_range(0, self.len()))
    }
}
#[cfg(test)]
mod tests {
    use crate::Pcg32;

    use super::*;

    #[test]
    fn get_random_item() {
        let mut rng = Pcg32::new(crate::DEFAULT_STATE, crate::DEFAULT_STREAM);
        let a = Vec::from([1_i32]);
        assert_eq!(
            a.get_random_item(&mut rng),
            Some(&1_i32)
        )
    }
}