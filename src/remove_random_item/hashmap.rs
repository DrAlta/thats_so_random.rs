use std::collections::HashMap;

use super::RemoveRandomItem;

impl<K: Eq + std::hash::Hash + Clone, V> RemoveRandomItem<(K,V)> for HashMap<K,V> {
    fn remove_random_item(&mut self, rng:&mut crate::Pcg32) -> Option<(K,V)> {
        if self.is_empty() {
            return None
        }
        let max = self.len() as usize;
        let nth = rng.random_range(0, max); 
        let key = self.iter().nth(nth)?.0.clone();
        let v= self.remove(&key)?;
        Some((key, v))
    }
}
#[cfg(test)]
mod tests {
    use crate::Pcg32;

    use super::*;

    #[test]
    fn remove_random_item() {
        let mut rng = Pcg32::new(crate::DEFAULT_STATE, crate::DEFAULT_STREAM);
        let mut a = HashMap::from([('c', 1_i32)]);
        assert_eq!(
            a.remove_random_item(&mut rng),
            Some(('c', 1_i32))
        )
    }
}