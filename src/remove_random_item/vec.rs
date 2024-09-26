use super::RemoveRandomItem;

impl<T> RemoveRandomItem<T> for Vec<T> {
    fn remove_random_item(&mut self, rng:&mut crate::Pcg32) -> Option<T> {
        if self.is_empty() {
            return None
        }
        Some(self.remove(rng.random_range(0, self.len())))
    }
}

#[cfg(test)]
mod tests {
    use crate::Pcg32;

    use super::*;

    #[test]
    fn remove_random_item() {
        let mut rng = Pcg32::new(crate::DEFAULT_STATE, crate::DEFAULT_STREAM);
        let mut a = Vec::from([1_i32]);
        assert_eq!(
            a.remove_random_item(&mut rng),
            Some(1_i32)
        )
    }
}