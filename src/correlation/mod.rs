pub mod coefficients {
    use crate::central_tendency::descriptive::calculate_mean;

    pub fn calculate_pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
        let mean_x = calculate_mean(x);
        let mean_y = calculate_mean(y);
        let numerator: f64 = x
            .iter()
            .zip(y.iter())
            .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
            .sum();
        let denominator_x: f64 = x.iter().map(|&x| (x - mean_x).powi(2)).sum();
        let denominator_y: f64 = y.iter().map(|&y| (y - mean_y).powi(2)).sum();

        numerator / (denominator_x.sqrt() * denominator_y.sqrt())
    }
}
