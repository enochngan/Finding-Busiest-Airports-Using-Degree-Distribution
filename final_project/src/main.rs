use csv::ReaderBuilder;
use plotters::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Airport {
    #[serde(rename = "Label")]
    #[allow(dead_code)]
    name: String,
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Latitude")]
    latitude: f64,
    #[serde(rename = "Longitude")]
    longitude: f64,
    #[serde(skip)]
    degree: usize, 
}

#[derive(Debug, Deserialize)]
struct Route {
    #[serde(rename = "Departure")]
    departure_id: String,
    #[serde(rename = "Destination")]
    destination_id: String,
}

fn read_airports(path: &str) -> Result<HashMap<String, Airport>, Box<dyn Error>> {
    let mut rdr = match csv::Reader::from_path(path) {
        Ok(reader) => reader,
        Err(err) => return Err(Box::new(err)),
    };
    let mut airports = HashMap::new();
    for result in rdr.deserialize() {
        let mut airport: Airport = result?;
        airport.degree = 0; // Initialize degree to zero
        airports.insert(airport.id.clone(), airport);
    }
    Ok(airports)
}

fn update_degrees(airports: &mut HashMap<String, Airport>, routes_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = match csv::Reader::from_path(routes_path) {
        Ok(reader) => reader,
        Err(err) => return Err(Box::new(err)),
    };
    for result in rdr.deserialize() {
        let route: Route = result?;
        if let Some(dep) = airports.get_mut(&route.departure_id) {
            dep.degree += 1;
        }
        if let Some(dest) = airports.get_mut(&route.destination_id) {
            dest.degree += 1;
        }
    }
    Ok(())
}

fn plot_airports(airports: &HashMap<String, Airport>, root: &DrawingArea<BitMapBackend, plotters::coord::Shift>) -> Result<(), Box<dyn Error>> {
    let mut chart = ChartBuilder::on(root)
        .caption("Global Airports", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-180f32..180f32, -90f32..90f32)?;

    chart.configure_mesh().draw()?;

    for airport in airports.values() {
        chart.draw_series(std::iter::once(Circle::new(
            (airport.longitude as f32, airport.latitude as f32), 1, &RED.mix(0.8),)))?;
    }
    Ok(())
}

fn write_degrees(airports: &HashMap<String, Airport>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(output_path)?;
    wtr.write_record(&["ID", "Name", "Latitude", "Longitude", "Degree"])?;  // Write CSV headers

    for airport in airports.values() {
        wtr.write_record(&[
            airport.id.clone(),
            airport.name.clone(),
            airport.latitude.to_string(),
            airport.longitude.to_string(),
            airport.degree.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut airports = read_airports("full_airports.csv")?;
    update_degrees(&mut airports, "full_routes.csv")?;

    let root_area = BitMapBackend::new("airports_visualization.png", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;
    plot_airports(&airports, &root_area)?;

    write_degrees(&airports, "airports_degrees.csv")?;

    println!("Airport visualization created and airport degrees written to 'airports_degrees.csv'.");
    Ok(())
}
