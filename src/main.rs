use crate::{
    central_tendency::central_tendency::{calculate_mean, calculate_median, calculate_mode},
    dispersion::measures::{calculate_range, calculate_standard_deviation, calculate_variance},
    feature::feature_engineering::experience_level_score,
    plot::plot::{draw_histogram, draw_scatter},
};

mod categorical;
mod central_tendency;
mod context;
mod data_loading;
mod dispersion;
mod feature;
mod fetch_dataset;
mod plot;
mod utils;

#[allow(dead_code)]
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

    // Measures of dispersion
    let variance_salary = calculate_variance(&salary_data, mean_salary).unwrap();
    let standard_deviation_salary = calculate_standard_deviation(variance_salary);
    let range_salary = calculate_range(&salary_data);

    println!("Variance of Salaries: {:.2}", variance_salary);
    println!(
        "Standard Deviation of salaries: {:.2}",
        standard_deviation_salary
    );
    println!("Range of salaries: {:.2}", range_salary);

    // Draw histogram
    draw_histogram(&salary_data, "salary in usd")?;

    // draw scatter
    let experience_data: Vec<String> = unique_dataset
        .iter()
        .map(|r| r.experience_level.clone())
        .collect();
    let unique_experience_level: std::collections::HashSet<_> = experience_data.iter().collect();
    println!("Unique experience level: {:?}", unique_experience_level);
    let encode_experience_data: Vec<u8> = experience_data
        .iter()
        .map(|level| experience_level_score(level))
        .collect();
    draw_scatter(&encode_experience_data, &salary_data)?;

    Ok(())
}
