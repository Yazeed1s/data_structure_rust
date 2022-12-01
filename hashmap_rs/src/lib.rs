#![allow(dead_code)]
#![allow(unused_variables)]

use core::hash::Hash;
use std::{borrow::Borrow, collections::hash_map::DefaultHasher, hash::Hasher, ops::Index};

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
    // TODO: add insert()
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index: u64 = self.hash(&key);
        let insert = self.bucket[index as usize].insert(key, value);
        if insert.is_none() {
            self.items += 1;
        }
        insert
    }
    // TODO: add getKey()
    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index: u64 = self.hash(key);
        let key = self.bucket[index as usize].get(key);
        key
    }
    // TODO: add contains_key()
    fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.get(key).is_some() {
            return true;
        }
        false
    }
    // TODO: add remove()
    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = self.hash(key);
        let removed_result = self.bucket[index as usize].remove(key);
        if removed_result.is_some() {
            self.items -= 1;
        }
        removed_result
    }
    // TODO: add len()
    fn len(&self) -> usize {
        self.items
    }
}

impl<K, Q, V> Index<&Q> for HashMap<K, V>
where
    K: Eq + Hash + Borrow<Q> + PartialEq<Q>,
    Q: Eq + Hash + ?Sized,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index)
            .expect("HashMap has no value associated to this key")
    }
}

// TODO: implement Iterator + IntoIterator for HashMap
// TODO: add tests
