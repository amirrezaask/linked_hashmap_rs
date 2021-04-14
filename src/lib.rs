use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    count: u64,
    initial_bucket_size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn get_bucket(&self, k: &K, len: usize) -> u64 {
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        hasher.finish() % len as u64
    }
    pub fn new(initial_bucket_size: usize) -> Self {
        let buckets: Vec<Vec<(K, V)>> = Vec::new();
        let mut hm = HashMap {
            buckets,
            initial_bucket_size,
            count: 0,
        };
        hm.resize();
        hm
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.buckets.is_empty() || self.count > (self.buckets.len() * 3) as u64 {
            println!("need to resize");
            self.resize();
        }
        let bucket = self.get_bucket(&key, self.buckets.len());
        let bucket = &mut self.buckets[bucket as usize];

        self.count += 1;
        for (k, v) in bucket.iter_mut() {
            if k == &key {
                *v = value;
                return;
            }
        }
        bucket.push((key, value));
    }

    fn resize(&mut self) {
        let target = match self.buckets.len() {
            0 => self.initial_bucket_size,
            n => n * 2,
        };
        let mut new_buckets: Vec<Vec<(K, V)>> = Vec::with_capacity(target);
        for _ in 0..target {
            new_buckets.push(Vec::new())
        }
        for bucket in self.buckets.drain(..) {
            for (k, v) in bucket.into_iter() {
                let mut hasher = DefaultHasher::new();
                k.hash(&mut hasher);
                let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
                new_buckets[bucket].push((k, v));
            }
        }

        self.buckets = new_buckets;
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.get_bucket(&key, self.buckets.len());
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
        let mut hm: HashMap<String, String> = HashMap::new(1);
        hm.put(String::from("Name"), String::from("Amirreza"));
        println!("{:?}", hm);
        assert_eq!(
            hm.get(&String::from("Name")),
            Some(&String::from("Amirreza"))
        );
    }
    #[test]
    fn insert_and_reinsert() {
        let mut hm: HashMap<&String, &String> = HashMap::new(1);
        let key = &String::from("Name");
        let value1 = &String::from("Amirreza");
        let value2 = &String::from("Parsa");
        hm.put(&key, &value1);
        assert_eq!(hm.get(&key), Some(&value1));
        hm.put(&key, &value2);
        assert_eq!(hm.get(&key), Some(&value2));
    }
}
