use std::convert::Infallible;
use log::debug;
use rand::{Rng, thread_rng};
use crate::scheduled_stream::Fetcher;

/// Implementation of trait [Fetcher] for generation of random-length batches of strings wrapping
/// random integers. In real-world scenarios, the [Fetcher] would read from a database, collect
/// data from external APIs, or similar.
pub struct RandomFetcher {
    max_batch: u16,
    max_number: u16
}

impl RandomFetcher {
    pub fn new(max_batch: u16, max_number: u16) -> Self {
        Self { max_batch, max_number }
    }
}

// In this implementation, [Infallible] is used because no error can occur.
impl Fetcher<String, Infallible> for RandomFetcher {
    fn fetch(&mut self) -> Result<Vec<String>, Infallible> {
        let mut rng = thread_rng();
        let mut batch = Vec::new();
        let limit = rng.gen_range(0..self.max_batch);
        debug!("Fetch {limit} items");
        for _ in 0..limit {
            batch.push(rng.gen_range(0..self.max_number).to_string());
        }
        Ok(batch)
    }
}
