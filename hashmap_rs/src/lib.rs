#![allow(dead_code)]
#![allow(unused_variables)]

use core::hash::Hash;
use core::marker::Copy;
use std::collections::hash_map::DefaultHasher;
//use std::hash::Hasher;
const BUCKET_SIZE: usize = 20;

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// struct Bucket<K, V> {
//     vec: Vec<(K, V)>,
// }
type Bucket<K, V> = Vec<(K, V)>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HashMap<K, V> {
    bucket: Vec<Bucket<K, V>>,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + PartialEq + PartialOrd + Copy,
    V: Ord,
{
    fn new() -> Self {
        let mut bucket = Vec::with_capacity(BUCKET_SIZE);
        for _ in 0..BUCKET_SIZE {
            bucket.push(Vec::new());
        }
        Self { bucket }
    }

    // fn from(bucket: Vec<(K, V)>) -> Self {
    //     /*  HashMap { bucket:  } */
    // }

    fn size(&self) -> usize {
        /*  self.bucket.len() */
        0
    }

    fn key_hash(&self, key: K) -> u64 {
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let index = hash % (BUCKET_SIZE as u64);
        index
    }

    fn clear(&mut self) {
        self.bucket.clear();
    }
    fn insert(&mut self, key: K, value: V) -> bool {
        // self.bucket.push((key, value));
        // !self.bucket.is_empty()
        // TODO: calculate hash
        // TODO: handle collision
        false
    }
    fn is_empty(&self) -> bool {
        self.bucket.is_empty()
    }
    fn remove_by_key(key: K) {}
    // TODO: add resize method
    // TODO: add get method
    // TODO: add remove_by_key method
    // TODO: add find value_value_by_key
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut map = HashMap::new();
        map.insert(10, 14);
        let x: i32 = 10;
        let expected = map.is_empty();
        assert_eq!(expected, false);
    }
    #[test]
    fn test_initilizating_using_from() {
        let n: HashMap<&str, i32> = HashMap::from(vec![("8", 2), ("9", 4), ("8", 6), ("0", 8)]);
        assert_eq!(n.size(), 4)
    }
}
