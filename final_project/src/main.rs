mod graph;
mod lib;

use crate::lib::{read_airports, read_routes, update_degrees, calculate_statistics, calculate_degree2};
use crate::graph::AirportGraph;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut airports = read_airports("full_airports.csv")?;
    let routes = read_routes("full_routes.csv")?;

    let airports100 = update_degrees(&mut airports, &routes);

    let degrees: HashMap<String, usize> = airports.iter()
        .map(|(id, airport)| (id.clone(), airport.degree))
        .collect();

    let (min_degree, max_degree, mean_degree, median_degree, percentiles) = calculate_statistics(&degrees);

    println!("Statistics of Airports from full_routes.csv");
    println!("Minimum Degree: {}", min_degree);
    println!("Maximum Degree: {}", max_degree);
    println!("Mean Degree: {:.2}", mean_degree);
    println!("Median Degree: {}", median_degree);

    println!(""); 

    println!("Percentiles of Airports from full_routes.csv");
    for (threshold, percentile) in percentiles {
        println!("Percent of airports with < {} degrees: {:.2}%", threshold, percentile);
    }

    println!("");

    let mut adjacency_list = HashMap::new();
    for route in &routes {
        adjacency_list.entry(route.departure_id.clone())
            .or_insert_with(Vec::new)
            .push(route.destination_id.clone());
        adjacency_list.entry(route.destination_id.clone())
            .or_insert_with(Vec::new)
            .push(route.departure_id.clone());  // Assuming bidirectional graph
    }

    // Calculate second-degree neighbors
    calculate_degree2(&mut airports, &adjacency_list);

    let degrees2: HashMap<String, usize> = airports.iter()
        .map(|(id, airport)| (id.clone(), airport.degree2))
        .collect();

    let (min_degree2, max_degree2, mean_degree2, median_degree2, percentiles2) = calculate_statistics(&degrees2);

    println!("Statistics of Airports from full_routes.csv");
    println!("Minimum Degree: {}", min_degree2);
    println!("Maximum Degree: {}", max_degree2);
    println!("Mean Degree: {:.2}", mean_degree2);
    println!("Median Degree: {}", median_degree2);

    println!(""); 

    println!("Percentiles of Airports from full_routes.csv");
    for (threshold, percentile) in percentiles2 {
        println!("Percent of airports with < {} degrees: {:.2}%", threshold, percentile);
    }

    println!(""); 

    let mut file = File::create("Busiest Airports in the World.csv")?;
    writeln!(file, "Below are Airports with Over 100 Degrees")?;
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