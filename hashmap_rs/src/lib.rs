#![allow(dead_code)]
#![allow(unused_variables)]

use core::hash::Hash;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};
// use core::marker::Copy;
// use std::collections::hash_map::DefaultHasher;
//use std::hash::Hasher;

const BUCKET_SIZE: usize = 32;
const THRESHOLD: f32 = 0.75;
// 0.75 * 32 = 24
const MAX_SIZE: usize = 24;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Bucket<K, V> {
    vec: Vec<(K, V)>,
}

impl<K, V> Bucket<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Self {
        Self { vec: vec![] }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        // position() returns the index of (k,v) if k == key
        match self.vec.iter().position(|(k, _)| *k == key) {
            Some(i) => {
                let (_, replaced_val) = std::mem::replace(&mut self.vec[i], (key, value));
                Some(replaced_val)
            }
            None => {
                self.vec.push((key, value));
                None
            }
        }
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q> + PartialEq<Q>,
        Q: Eq + Hash + ?Sized,
    {
        match self.vec.iter().find(|&(k, _)| *k == *key) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q> + PartialEq<Q>,
        Q: Eq + Hash + ?Sized,
    {
        match self.vec.iter().position(|(k, _)| *k == *key) {
            Some(i) => {
                let (_, removed_val) = self.vec.swap_remove(i);
                Some(removed_val)
            }
            None => None,
        }
    }
}
// type Bucket<K, V> = Vec<(K, V)>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HashMap<K, V> {
    bucket: Vec<Bucket<K, V>>,
    items: usize,
    cap: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Self {
        // let arr: Vec<Bucket<K, V>>;
        // for _ in 0..BUCKET_SIZE {
        //     arr = Bucket::<K, V>::new()
        // }
        let vec: Vec<Bucket<K, V>> = (0..BUCKET_SIZE)
            .into_iter()
            .map(|_| Bucket::<K, V>::new())
            .collect();

        HashMap {
            bucket: vec,
            items: 0,
            cap: BUCKET_SIZE,
        }
    }

    fn hash<Q>(&self, key: &Q) -> u64
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let index = hash % (self.bucket.len() as u64);
        /* hasher.finish() as usize % self.cap */
        index
    }
    // TODO: add inser()
    // TODO: add get()
    // TODO: add contains_key()
    // TODO: add remove()
    // TODO: add len()
}
// impl<K, V> HashMap<K, V>
// where
//     K: Hash + PartialEq + PartialOrd + Copy,
//     V: Ord,
// {
//     fn new() -> Self {
//         let mut bucket = Vec::with_capacity(BUCKET_SIZE);
//         for _ in 0..BUCKET_SIZE {
//             bucket.push(Vec {
//                 buf: RawVec::NEW,
//                 len: 0,
//             });
//         }
//         Self {
//             bucket: Vec::new(),
//             items: 0,
//         }
//     }

//     // fn from(bucket: Vec<(K, V)>) -> Self {
//     //     /*  HashMap { bucket:  } */
//     // }

//     fn size(&self) -> usize {
//         // self.bucket.len()
//         self.items
//     }

//     fn is_empty(&self) -> bool {
//         //self.bucket.is_empty()
//         self.items == 0
//     }

//     // hash the key
//     fn hash_key(&self, key: &K) -> u64 {
//         use std::hash::Hasher;
//         let mut hasher = DefaultHasher::new();
//         key.hash(&mut hasher);
//         let hash = hasher.finish();
//         // let index = hash % (BUCKET_SIZE as u64);
//         let index = hash % (self.bucket.len() as u64);
//         index
//     }

//     // map key to an index
//     fn key_to_bucket(&mut self, key: &K) -> Option<usize> {
//         if self.bucket.is_empty() {
//             return None;
//         }
//         let index = self.hash_key(key);
//         Some(index as usize)
//     }

//     fn clear(&mut self) {
//         self.bucket.clear();
//     }

//     // fn insert<Ord>(&mut self, key: K, value: V) -> Option<K> {
//     //     let bucket = self.key_to_bucket(&key);
//     //     for i in self.bucket.iter_mut() {
//     //         let (&mut _val, &_key): &mut Vec<(K, V)> = i;
//     //     }
//     //     std::mem::swap(key, value);
//     //     // self.bucket.push((key, value));
//     //     // !self.bucket.is_empty()
//     //     // TODO: calculate hash
//     //     // TODO: handle collision
//     //     Some(key)

//     pub fn insert(&mut self, key: K, value: V) -> Option<V> {
//         // if self.buckets.is_empty() || self.items < 3 * self.buckets.len() / 4 {
//         //     self.resize();
//         // }
//         let idx_bucket: usize = self.key_to_bucket(&key) as usize;
//         let bucket = &mut self.bucket[idx_bucket];
//         // for &mut (ref ekey, ref mut evalue) in bucket.iter_mut()
//         for item in bucket.iter_mut() {
//             let (ref ekey, ref mut evalue) = item;
//             if ekey == &key {
//                 return Some(std::mem::replace(evalue, value));
//             }
//         }
//         bucket.push((key, value));
//         self.items += 1;
//         return None;
//     }

//     fn resize(&mut self) {
//         let new_size = 2 * self.bucket.len();
//     }
// }

// // fn remove_by_key(key: K) {}
// // TODO: add resize method
// // TODO: add get method
// // TODO: add remove_by_key method
// // TODO: add find value_value_by_key

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         // let mut map = HashMap::new();
//         // map.insert(10, 14);
//         // let x: i32 = 10;
//         // let expected = map.is_empty();
//         // assert_eq!(expected, false);
//     }
//     #[test]
//     fn test_initilizating_using_from() {
//         // let n: HashMap<&str, i32> = HashMap::from(vec![("8", 2), ("9", 4), ("8", 6), ("0", 8)]);
//         // assert_eq!(n.size(), 4)
//     }
// }
