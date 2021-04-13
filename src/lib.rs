struct HashMap<K,V>{
    buckets: Vec<Vec<(K,V)>>,
}

impl<K,V> HashMap<K,V> where K:std::hash::Hasher + Eq {
    pub fn new(initial_bucket_size: u64) -> Self {
        let buckets: Vec<Vec<(K,V)>> = Vec::new();
        for i in 0..initial_bucket_size {
            buckets.push(Vec::new());
        }
        HashMap{
            buckets,
        }
    }

    pub fn put(&mut self) -> Result<(), core::result::Result::Err>{
        Err("not implemented")
    }

    pub fn get(self) -> Option<V> {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
