use crate::central_tendency::central_tendency::{calculate_mean, calculate_median, calculate_mode};

mod central_tendency;
mod context;
mod data_loading;
mod fetch_dataset;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";
    let mut seen = std::collections::HashSet::new();
    let mut unique_dataset = Vec::new();

    let csv_data = fetch_dataset::fetch_dataset(&url)?;
    let dataset = data_loading::load_csv_data(&csv_data)?;
    for record in dataset {
        let key = (
            record.work_year,
            record.job_title.clone(),
            record.company_location.clone(),
        );
        if seen.insert(key) {
            unique_dataset.push(record);
        }
    }
    println!("Unique Dataset size: {}", unique_dataset.len());

    // Central tendence meaures
    let mut salary_data: Vec<f64> = unique_dataset.iter().map(|r| r.salary_in_usd).collect();
    let mean_salary = calculate_mean(&salary_data);
    let median_salary = calculate_median(&mut salary_data);
    let mode_salary = calculate_mode(&salary_data);

    println!("Mean Salary: {:.2}", mean_salary);
    println!("Median Salary: {:.2}", median_salary);
    println!("Mode Salary: {:?}", mode_salary);

    Ok(())
}
