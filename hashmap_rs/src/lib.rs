#![allow(dead_code)]
#![allow(unused_variables)]
// struct Bucket<K, V> {
//     vec: Vec<(K, V)>,
// }

#[derive(Debug)]
#[derive(PartialEq)]
struct HashMap<K, V> {
    bucket: Vec<(K, V)>,
}

impl <K, V: Ord> HashMap <K, V> where {
    fn new() -> Self {
        HashMap {
            bucket: Vec::new()
        }
    }
    fn insert(&mut self, key: K, value: V) -> bool {
        self.bucket.push((key, value));
        !self.bucket.is_empty()
        // TODO: calculate hash 
        // TODO: handle collision 
    }
    fn is_empty(&self) -> bool {
        self.bucket.is_empty()
    }
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
       let expected = map.is_empty();
       assert_eq!(expected, false);
    }
}