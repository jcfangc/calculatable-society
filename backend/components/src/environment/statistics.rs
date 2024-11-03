use crate::environment::resource_distribution::ResourceDistribution;
use rayon::prelude::*;

pub trait Statistics {
    type Item;
    fn min(&self) -> Self::Item;
    fn max(&self) -> Self::Item;
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
}

impl Statistics for ResourceDistribution {
    type Item = usize;

    fn min(&self) -> Self::Item {
        *self.distribution.par_iter().min().expect("分布为空")
    }

    fn max(&self) -> Self::Item {
        *self.distribution.par_iter().max().expect("分布为空")
    }

    fn mean(&self) -> f64 {
        let sum: usize = self.distribution.par_iter().sum();
        sum as f64 / self.distribution.len() as f64
    }

    fn variance(&self) -> f64 {
        let mean = self.mean();
        let sum_of_squares: f64 = self
            .distribution
            .par_iter()
            .map(|&value| {
                let diff = value as f64 - mean;
                diff * diff
            })
            .sum();

        sum_of_squares / (self.distribution.len() - 1) as f64
    }
}
