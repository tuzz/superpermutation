use std::cmp::max;
use std::usize::MAX;

#[derive(Debug)]
struct Bounds {
    lower_bounds: Vec<usize>,
    upper_bounds: Vec<usize>,
    thresholds: Vec<usize>,
}

impl Bounds {
    fn new() -> Self {
        Self { lower_bounds: vec![0], upper_bounds: vec![MAX], thresholds: vec![0] }
    }

    fn update(&mut self, index: usize, bound: usize) -> bool {
        if self.lower_bounds.len() <= index {
            self.add_new_index(index, bound);
            return true;
        }

        if self.lower_bounds[index] < bound {
            self.increase_lower_bound(index, bound);
            return true
        }

        false
    }

    fn add_new_index(&mut self, index: usize, bound: usize) {
        let previous_len = self.lower_bounds.len();
        let last_bound = *self.lower_bounds.last().unwrap();

        self.lower_bounds.resize(index + 1, 0);
        self.upper_bounds.resize(index + 1, MAX);
        self.thresholds.resize(index + 1, 0);

        for i in previous_len..=index {
            self.increase_lower_bound(i, max(bound, last_bound));
        }

        for i in (previous_len - 1)..index {
            self.fix_upper_bound(i);
        }

        self.decrease_upper_bound(index);
    }

    fn increase_lower_bound(&mut self, index: usize, bound: usize) {
        self.lower_bounds[index] = bound;
        self.thresholds[index] = bound - self.lower_bounds[0];
    }

    fn fix_upper_bound(&mut self, index: usize) {
        self.upper_bounds[index] = self.lower_bounds[index];
    }

    fn decrease_upper_bound(&mut self, index: usize) {
        self.upper_bounds[index] = self.lower_bounds[index] + self.upper_bounds[0];
    }
}

#[cfg(test)]
mod test;