use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Define a structure to deserialize the routes data.
#[derive(Debug, Deserialize)]
struct Route {
    #[serde(rename = "Departure")]
    source_airport: String,
    #[serde(rename = "Destination")]
    destination_airport: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a reader for the CSV file.
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("Full_Merge_of_All_Unique_Routes.csv")?;

    // Print the headers first to confirm their names
    let headers = rdr.headers()?;
    println!("CSV Headers: {:?}", headers);

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    // Read the CSV records.
    for result in rdr.deserialize::<Route>() {
        let route = result?;
        // Insert the source airport if it doesn't exist.
        graph.entry(route.source_airport.clone())
             .or_insert_with(Vec::new)
             .push(route.destination_airport.clone());
        // For undirected graph, also connect destination to source.
        graph.entry(route.destination_airport)
             .or_insert_with(Vec::new)
             .push(route.source_airport);
    }

    // Write the graph to a new file.
    let mut file = File::create("airport_graph.txt")?;
    for (airport, connections) in graph {
        writeln!(file, "{}: {:?}", airport, connections)?;
    }

    Ok(())
}
