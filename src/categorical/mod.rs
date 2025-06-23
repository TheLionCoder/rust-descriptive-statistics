use crate::context::salary_model::SalaryRecord;

pub mod categorical {
    use super::*;

    pub fn job_title_frequency(data: &[SalaryRecord]) -> std::collections::HashMap<String, usize> {
        let mut frequency = std::collections::HashMap::new();
        for record in data {
            *frequency.entry(record.job_title.clone()).or_insert(0) += 1;
        }
        frequency
    }
}
