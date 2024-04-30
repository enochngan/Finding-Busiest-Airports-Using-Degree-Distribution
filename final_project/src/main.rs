use csv::ReaderBuilder;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Airport {
    name: String,
    latitude: f64,
    longitude: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("airports_visualization.png", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Busiest Airports", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-180f32..180f32, -90f32..90f32)?;

    chart.configure_mesh().draw()?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("Full_Merge_of_All_Unique Airports.csv")?;
    
    let airports = rdr.deserialize::<Airport>()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    chart.draw_series(
        airports.iter().map(|airport| {
            Circle::new((airport.longitude as f32, airport.latitude as f32), 5, BLUE.filled())
        })
    )?;

    root_area.present()?;
    println!("Airport locations have been saved to airports_visualization.png");

    Ok(())
}
