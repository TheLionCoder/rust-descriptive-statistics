pub mod extrema {
    pub fn calculate_min_value(data: &[f64]) -> f64 {
        data.iter()
            .cloned()
            .fold(f64::INFINITY, |acc, val| acc.min(val))
    }

    pub fn calculate_max_value(data: &[f64]) -> f64 {
        data.iter()
            .cloned()
            .fold(f64::NEG_INFINITY, |acc, val| acc.max(val))
    }
}
