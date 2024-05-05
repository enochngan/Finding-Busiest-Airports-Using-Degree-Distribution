mod graph;

use final_project::{Airport, read_airports, read_routes, update_degrees, calculate_statistics, calculate_degree2};
use crate::graph::AirportGraph;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // reads airport data from full_airports.csv file
    let mut airports = read_airports("full_airports.csv")?;
    // reads routes data from full_routes.csv CSV file
    let routes = read_routes("full_routes.csv")?;

    // calculates the degree of connectivity using update_degrees for each airport based on routes
    update_degrees(&mut airports, &routes);

    // creates and writes to a file, listing airports with more than 100 direct connections
    let mut file = File::create("Ranked Busiest Airports in the World by Degrees.csv")?;

    // writes the header of the file
    writeln!(file, "ID, Name, Degree")?;

    // converts the HashMap into a vector for sorting
    let mut airports_vec: Vec<(&String, &Airport)> = airports.iter().collect();

    // sorts the vector by degree in descending order
    airports_vec.sort_by(|a, b| b.1.degree.cmp(&a.1.degree));

    // iterates over the sorted vector and write each entry to the file, ensuring names are properly quoted
    for (id, airport) in airports_vec {
        writeln!(file, "\"{}\", {}, {}", airport.name, id, airport.degree)?;
    }

    // collects the degrees of all airports into a HashMap 
    let degrees: HashMap<String, usize> = airports.iter()
        .map(|(id, airport)| (id.clone(), airport.degree))
        .collect();

    // calculates and stores statistics for airport degrees
    let (min_degree, max_degree, mean_degree, median_degree, percentiles) = calculate_statistics(&degrees);

    // prints out the calculated statistics for first-degree neighbors
    println!("Statistics of Airports for neighbors of distance 1 from full_routes.csv");
    println!("Minimum Degree: {}", min_degree);
    println!("Maximum Degree: {}", max_degree);
    println!("Mean Degree: {:.2}", mean_degree);
    println!("Median Degree: {}", median_degree);

    println!(""); 

    // prints out percentile information for first-degree neighbors
    println!("Percentiles of Airports for neighbors of distance 1 from full_routes.csv");
    for (threshold, percentile) in percentiles {
        println!("Percent of airports with < {} degrees: {:.2}%", threshold, percentile);
    }

    println!("");

    // builds an adjacency list for the airports based on routes
    let mut adjacency_list = HashMap::new();
    for route in &routes {
        adjacency_list.entry(route.departure_id.clone())
            .or_insert_with(Vec::new)
            .push(route.destination_id.clone());
        adjacency_list.entry(route.destination_id.clone())
            .or_insert_with(Vec::new)
            .push(route.departure_id.clone());  // assumes a bidirectional graph
    }

    // calculates second-degree neighbor stats using the adjacency list
    calculate_degree2(&mut airports, &adjacency_list);

    // collects the degrees of second-level neighbors into a HashMap
    let degrees2: HashMap<String, usize> = airports.iter()
        .map(|(id, airport)| (id.clone(), airport.degree2))
        .collect();

    // calculates and store statistics for second-degree neighbors
    let (min_degree2, max_degree2, mean_degree2, median_degree2, percentiles2) = calculate_statistics(&degrees2);

    // prints out the calculated statistics for second-degree neighbors
    println!("Statistics of Airports for neighbors of distance 2 from full_routes.csv");
    println!("Minimum Degree: {}", min_degree2);
    println!("Maximum Degree: {}", max_degree2);
    println!("Mean Degree: {:.2}", mean_degree2);
    println!("Median Degree: {}", median_degree2);

    println!(""); 

    // prints out percentile information for second-level neighbors
    println!("Percentiles of Airports for neighbors of distance 2 from full_routes.csv");
    for (threshold, percentile) in percentiles2 {
        println!("Percent of airports with < {} degrees: {:.2}%", threshold, percentile);
    }

    println!(""); 

    println!("Calculate the shortest path from input departure airport to destination airport");    
    // user input for departure airport ID
    println!("Please enter departure airport ID:");
    let mut departure_id = String::new();
    io::stdin().read_line(&mut departure_id)?;
    departure_id = departure_id.trim().to_string();

    // user input for destination airport ID
    println!("Please enter destination airport ID:");
    let mut destination_id = String::new();
    io::stdin().read_line(&mut destination_id)?;
    destination_id = destination_id.trim().to_string();

    // creates an airport graph and calculates the number of switches needed for a direct path
    let graph = AirportGraph::new(routes);
    if let Some(switches) = graph.calculate_switches(&departure_id).get(&destination_id) {
        println!("{} -> {}: {} switches", departure_id, destination_id, switches);
    } else {
        println!("Route not found.");
    }

    Ok(())
}