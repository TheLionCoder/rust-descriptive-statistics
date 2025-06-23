pub mod utils {
    pub fn calculate_min_value(data: &[f64]) -> f64 {
        let min_value = data
            .iter()
            .cloned()
            .fold(f64::INFINITY, |acc, val| acc.min(val));

        min_value
    }

    pub fn calculate_max_value(data: &[f64]) -> f64 {
        let max_value = data
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, |acc, val| acc.max(val));

        max_value
    }
}
