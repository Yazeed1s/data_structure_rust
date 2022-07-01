#![allow(dead_code)]
#![allow(unused_variables)]
// struct Bucket<K, V> {
//     vec: Vec<(K, V)>,
// }

#[derive(Debug, PartialEq)]
struct HashMap<K, V> {
    bucket: Vec<(K, V)>,
}

impl<K, V: Ord> HashMap<K, V> {
    fn new() -> Self {
        HashMap { bucket: Vec::new() }
    }

    fn from(bucket: Vec<(K, V)>) -> Self {
        HashMap { bucket }
    }

    fn size(&self) -> usize {
        self.bucket.len()
    }

    fn clear(&mut self) {
        self.bucket.clear();
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
