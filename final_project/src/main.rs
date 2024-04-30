use csv::ReaderBuilder;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Airport {
    #[serde(rename = "Label")]
    name: Option<String>,
    #[serde(rename = "ID")]
    id: Option<String>,
    #[serde(rename = "Latitude")]
    latitude: Option<f64>,
    #[serde(rename = "Longitude")]
    longitude: Option<f64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // creates the drawing backend
    let root_area = BitMapBackend::new("airports_visualization.png", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Busiest Global Airport ", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-180f32..180f32, -90f32..90f32)?;

    chart.configure_mesh().draw()?;

    // reads and parses the CSV file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("Full_Merge_of_All_Unique Airports.csv")?;

    let airports = rdr.deserialize::<Airport>()
        .filter_map(Result::ok) // converts results to Option, filtering out Err values
        .filter(|airport|       // furthers filter to drop any record with missing fields
            airport.id.is_some() && 
            airport.name.is_some() &&
            airport.latitude.is_some() &&
            airport.longitude.is_some()
        )
        .collect::<Vec<_>>();   // Collect all valid records into a vector

    // draws each airport on the map with reasonably sized circles
    let drawing_result = chart.draw_series(
        airports.iter().map(|airport| {
            Circle::new(
                (airport.longitude.unwrap() as f32, airport.latitude.unwrap() as f32), 1, RED.filled()
            )
        })
    );

    if let Err(e) = drawing_result {
        eprintln!("Failed to draw series: {}", e);
    }

    // finishes drawing and saves to file
    root_area.present()?;
    println!("created 'airports_visualization.png'.");

    Ok(())
}

