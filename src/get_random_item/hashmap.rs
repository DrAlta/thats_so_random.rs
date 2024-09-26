use std::collections::HashMap;

use super::GetRandomItem;

impl<'a, K: Eq + std::hash::Hash, V> GetRandomItem<'a, (&'a K, &'a V)> for HashMap<K,V> {
    fn get_random_item(&self, rng:&mut crate::Pcg32) -> Option<(&K, &V)> {
        if self.is_empty() {
            return None
        }
        let max = self.len() as usize;
        let nth = rng.random_range(0, max); 
        self.iter().nth(nth)
    }
}
#[cfg(test)]
mod tests {
    use crate::Pcg32;

    use super::*;

    #[test]
    fn remove_random_item() {
        let mut rng = Pcg32::new(crate::DEFAULT_STATE, crate::DEFAULT_STREAM);
        let a = HashMap::from([('c', 1_i32)]);
        assert_eq!(
            a.get_random_item(&mut rng),
            Some((&'c', &1_i32))
        )
    }
}