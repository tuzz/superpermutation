use super::candidate::Candidate;

use ::bucket_queue::*;
use std::collections::VecDeque;

struct Frontier {
    priority_queue: BucketQueue<BucketQueue<VecDeque<Candidate>>>,
}

impl Frontier {
    fn new() -> Self {
        let queue = BucketQueue::<BucketQueue<VecDeque<Candidate>>>::new();

        Frontier { priority_queue: queue }
    }

    fn add(&mut self, candidate: Candidate, n: usize) {
        let permutations = candidate.permutations_seen.len();

        self.priority_queue
            .bucket_for_adding(candidate.total_waste(n))
            .enqueue(candidate, permutations);
    }

    fn next(&mut self) -> Option<Candidate> {
        let waste = self.min_waste()?;
        let bucket = self.priority_queue.bucket_for_removing(waste)?;

        bucket.dequeue_max()
    }

    fn len(&self) -> usize {
        self.priority_queue.len()
    }

    fn len_for_waste(&self, wasted_symbols: usize) -> usize {
        let bucket = self.priority_queue.bucket_for_peeking(wasted_symbols);
        bucket.map_or(0, |b| b.len())
    }

    fn min_waste(&self) -> Option<usize> {
        self.priority_queue.min_priority()
    }

    fn max_waste(&self) -> Option<usize> {
        self.priority_queue.max_priority()
    }
}

#[cfg(test)]
mod test;