use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    count: u64,
}

struct HashMapErr {}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn get_bucket(&self, k: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        hasher.finish() % self.buckets.len() as u64
    }
    pub fn new(initial_bucket_size: u64) -> Self {
        let mut buckets: Vec<Vec<(K, V)>> = Vec::new();
        for _ in 0..initial_bucket_size {
            buckets.push(Vec::new());
        }
        HashMap {
            buckets: buckets,
            count: 0,
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let bucket = self.get_bucket(&key);
        let bucket = &mut self.buckets[bucket as usize];

        self.count += 1;
        bucket.iter_mut().for_each(|(&k, v)| {
            if k == key {
                let out = *v;
                *v = value;
                Some(out);
            }
        });
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.get_bucket(&key);
        let bucket = &self.buckets[bucket as usize];
        for (ref k, ref v) in bucket.into_iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_and_get_works() {
        let hm: HashMap<String, String> = HashMap::new(1);
    }
}
