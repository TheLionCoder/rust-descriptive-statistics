pub mod descriptive {
    pub fn calculate_mean(data: &[f64]) -> f64 {
        let sum: f64 = data.iter().sum();
        sum / data.len() as f64
    }

    pub fn calculate_median(data: &mut [f64]) -> f64 {
        data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let len = data.len();
        if len % 2 == 0 {
            (data[len / 2 - 1] + data[len / 2]) / 2.0
        } else {
            data[len / 2]
        }
    }

    pub fn calculate_mode(data: &[f64]) -> Vec<f64> {
        let mut frequency_map = std::collections::HashMap::new();
        for &value in data {
            let bits = value.to_bits();
            *frequency_map.entry(bits).or_insert(0) += 1;
        }
        let max_frequency = frequency_map.values().cloned().max().unwrap_or(0);

        frequency_map
            .into_iter()
            .filter(|&(_, freq)| freq == max_frequency)
            .map(|(bits, _)| f64::from_bits(bits))
            .collect()
    }
}
