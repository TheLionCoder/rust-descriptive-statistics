use crate::utils::extrema::{calculate_max_value, calculate_min_value};

pub mod measures {
    use super::*;

    pub fn calculate_range(data: &[f64]) -> f64 {
        let min_value = calculate_min_value(data);
        let max_value = calculate_max_value(data);

        max_value - min_value
    }

    pub fn calculate_variance(data: &[f64], mean: f64) -> Option<f64> {
        let n = data.len();
        if n < 2 {
            return None;
        }

        let sum_of_squared_diff = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>();
        let divisor = (n - 1) as f64;
        Some(sum_of_squared_diff / divisor)
    }

    pub fn calculate_standard_deviation(variance: f64) -> f64 {
        variance.sqrt()
    }
}
