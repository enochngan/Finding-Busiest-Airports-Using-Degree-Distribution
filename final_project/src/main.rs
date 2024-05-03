mod graph;
mod lib;

use crate::lib::{read_airports, read_routes, update_degrees};
use crate::graph::AirportGraph;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut airports = read_airports("full_airports.csv")?;
    let routes = read_routes("full_routes.csv")?;

    let mut airports100 = update_degrees(&mut airports, &routes);

    let mut file = File::create("Busiest Airports in the World.csv")?;
    writeln!(file, "Below are Airports with over 100 degrees")?;
    writeln!(file, "ID, Name")?; // Header for the CSV file
    for (id, airport) in airports100 {
        writeln!(file, "{}, {}", id, airport.name)?;
    }

    println!("Please enter departure airport ID:");
    let mut departure_id = String::new();
    io::stdin().read_line(&mut departure_id)?;
    departure_id = departure_id.trim().to_string();

    println!("Please enter destination airport ID:");
    let mut destination_id = String::new();
    io::stdin().read_line(&mut destination_id)?;
    destination_id = destination_id.trim().to_string();

    let graph = AirportGraph::new(routes);
    if let Some(switches) = graph.calculate_switches(&departure_id).get(&destination_id) {
        println!("{} -> {}: {} switches", departure_id, destination_id, switches);
    } else {
        println!("Route not found.");
    }

    Ok(())
}
