use crate::central_tendency::descriptive::{calculate_max_value, calculate_min_value};
use plotters::prelude::*;

pub mod visualization {
    use super::*;

    pub fn draw_histogram(data: &[f64], caption: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from("data-viz")
            .join(format!("{}.jpg", caption.to_lowercase().replace(" ", "_")));
        let drawing_area = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
        drawing_area.fill(&WHITE)?;
        let Some(min_data_value) = calculate_min_value(data) else {
            return Ok(());
        };
        let Some(max_data_value) = calculate_max_value(data) else {
            return Ok(());
        };

        let mut chart_builder = ChartBuilder::on(&drawing_area);
        chart_builder
            .caption(caption, ("sans-seriff", 40).into_font())
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
            BitMapBackend::new("data-viz/salary_vs_experience.png", (640, 480)).into_drawing_area();
        drawing_area.fill(&WHITE)?;
        let Some(max_experience) = calculate_max_value(experience) else {
            return Ok(());
        };
        let Some(min_experience) = calculate_max_value(experience) else {
            return Ok(());
        };
        let Some(max_salary) = calculate_max_value(salary) else {
            return Ok(());
        };
        let Some(min_salary) = calculate_min_value(salary) else {
            return Ok(());
        };

        let mut chart_builder = ChartBuilder::on(&drawing_area);
        chart_builder
            .caption("Salary vs Experience", ("sans-seriff", 40).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40);
        let mut chart_context = chart_builder.build_cartesian_2d(
            min_experience as f64..max_experience as f64,
            min_salary..max_salary,
        )?;
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

    pub fn draw_bar_chart(
        frequency: std::collections::HashMap<String, usize>,
        top: Option<usize>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let drawing_area =
            BitMapBackend::new("data-viz/salary_freq.png", (1280, 720)).into_drawing_area();
        drawing_area.fill(&WHITE)?;
        let mut data: Vec<(&String, &usize)> = frequency.iter().collect();
        data.sort_by(|a, b| b.1.cmp(a.1));

        if let Some(top) = top {
            data.truncate(top);
        };
        let n_to_take = top.unwrap_or(20);

        let labels: Vec<&String> = data.iter().map(|&(title, _)| title).collect();
        let values: Vec<&usize> = data.iter().map(|&(_, count)| count).collect();

        let max_value = *values.iter().max().unwrap();
        let mut chart_builder = ChartBuilder::on(&drawing_area);
        chart_builder
            .caption("Job Title Distribution", ("sans-serif", 40).into_font())
            .margin(10)
            .x_label_area_size(180)
            .y_label_area_size(60);
        let mut chart_context = chart_builder.build_cartesian_2d(0..labels.len(), 0..*max_value)?;
        chart_context
            .configure_mesh()
            .x_labels(n_to_take)
            .x_label_formatter(&|x| {
                if *x < labels.len() {
                    labels[*x].to_string()
                } else {
                    "".to_string()
                }
            })
            .x_label_style(("sans-serif", 10).into_font())
            .y_desc("freq")
            .x_desc("job_title")
            .axis_desc_style(("sans-serif", 20))
            .draw()?;
        chart_context.draw_series(values.iter().enumerate().map(|(i, &v)| {
            let x0 = i;
            let x1 = i + 1;
            let y0 = 0;
            let y1 = *v;
            Rectangle::new([(x0, y0), (x1, y1)], BLUE.filled())
        }))?;
        Ok(())
    }
}
