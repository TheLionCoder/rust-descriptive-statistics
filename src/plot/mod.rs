use crate::utils::utils::{calculate_max_value, calculate_min_value};
use plotters::prelude::*;

pub mod plot {
    use super::*;

    pub fn draw_histogram(data: &[f64], caption: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from("data-viz")
            .join(format!("{}.jpg", caption.to_lowercase().replace(" ", "_")));
        let drawing_area = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
        drawing_area.fill(&WHITE)?;
        let min_data_value = calculate_min_value(&data);
        let max_data_value = calculate_max_value(&data);

        let mut chart_builder = ChartBuilder::on(&drawing_area);
        chart_builder
            .caption(&caption, ("sans-seriff", 40).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30);

        let mut chart_context = chart_builder.build_cartesian_2d(
            min_data_value as i64..max_data_value as i64,
            0..data.len() as i64 / 10,
        )?;

        chart_context.configure_mesh().draw()?;
        chart_context.draw_series(
            Histogram::vertical(&chart_context)
                .style(BLUE.filled())
                .data(data.iter().map(|&x| (x as i64, 2))),
        )?;

        Ok(())
    }

    pub fn draw_scatter(
        experience: &[u8],
        salary: &[f64],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let drawing_area =
            BitMapBackend::new("data-viz/salary_vs_expericen.png", (640, 480)).into_drawing_area();
        drawing_area.fill(&WHITE)?;
        let max_experience = *experience.iter().max().unwrap() as f64;
        let min_experience = *experience.iter().min().unwrap() as f64;

        let max_salary = calculate_max_value(&salary);
        let min_salary = calculate_min_value(&salary);

        let mut chart_builder = ChartBuilder::on(&drawing_area);
        chart_builder
            .caption("Salary vs Experience", ("sans-seriff", 40).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40);
        let mut chart_context = chart_builder
            .build_cartesian_2d(min_experience..max_experience, min_salary..max_salary)?;
        chart_context
            .configure_mesh()
            .x_desc("Experiencie Level")
            .y_desc("Salary in USD")
            .draw()?;
        chart_context.draw_series(
            salary
                .iter()
                .zip(experience.iter())
                .map(|(&s, &e)| Circle::new((e as f64, s), 3, RED.filled())),
        )?;

        Ok(())
    }
}
