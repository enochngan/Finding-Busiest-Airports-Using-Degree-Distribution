use csv::Reader;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use serde::Deserialize;

// defines data structure for airports with serialization details
#[derive(Debug, Deserialize, Clone)]
pub struct Airport {
    #[serde(rename = "Label")]
    pub name: String,
    #[serde(rename = "ID")] 
    pub id: String,
    #[serde(skip)]
    pub degree: usize, 
    #[serde(skip)]
    pub degree2: usize, 
}

// defines data structure for routes with serialization details
#[derive(Debug, Deserialize)]
pub struct Route {
    #[serde(rename = "Departure")] 
    pub departure_id: String,
    #[serde(rename = "Destination")] 
    pub destination_id: String,
}

// reads and parses airports data from a CSV file
pub fn read_airports(path: &str) -> Result<HashMap<String, Airport>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?; // creates a CSV reader from a file path
    let mut airports = HashMap::new();
    for result in rdr.deserialize() {
        let airport: Airport = result?; // deserializes each record into an Airport object
        airports.insert(airport.id.clone(), airport); // adds to the hashmap with the airport ID as the key
    }
    Ok(airports)
}

// reads and parses routes data from a CSV file
pub fn read_routes(path: &str) -> Result<Vec<Route>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?; // creates a CSV reader from a file path
    let mut routes = Vec::new();
    for result in rdr.deserialize() {
        let route: Route = result?; // deserializes each record into a Route object
        routes.push(route); // adds to the vector
    }
    Ok(routes)
}

// updates the degree of connectivity for each airport based on the routes data
pub fn update_degrees(airports: &mut HashMap<String, Airport>, routes: &[Route]) -> HashMap<String, Airport> {
    let mut airports100 = HashMap::new();
    for route in routes {
        // increments the degree for departure and destination airports
        if let Some(dep) = airports.get_mut(&route.departure_id) {
            dep.degree += 1;
            if dep.degree >= 100 { // checkes if the degree is 100 or more and store separately
                airports100.entry(dep.id.clone()).or_insert_with(|| dep.clone());
            }
        }
        if let Some(dest) = airports.get_mut(&route.destination_id) {
            dest.degree += 1;
            if dest.degree >= 100 {
                airports100.entry(dest.id.clone()).or_insert_with(|| dest.clone());
            }
        }
    }
    airports100 // returns airports with degrees of 100 or more
}

// calculates second-degree connections for each airport
pub fn calculate_degree2(airports: &mut HashMap<String, Airport>, adjacency_list: &HashMap<String, Vec<String>>) {
    for (airport_id, airport) in airports.iter_mut() {
        let mut neighbors2 = HashSet::new();
        if let Some(neighbors) = adjacency_list.get(airport_id) {
            for neighbor in neighbors {
                if let Some(second_neighbors) = adjacency_list.get(neighbor) {
                    for second_neighbor in second_neighbors {
                        if second_neighbor != airport_id && !neighbors.contains(second_neighbor) {
                            neighbors2.insert(second_neighbor); // collects unique second-degree neighbors
                        }
                    }
                }
            }
        }
        airport.degree2 = neighbors2.len(); // updates the count of second-degree neighbors
    }
}

// calculates statistical metrics from a set of degree values
pub fn calculate_statistics(degrees: &HashMap<String, usize>) -> (usize, usize, f64, usize, Vec<(usize, f64)>) {
    let mut degree_values: Vec<usize> = degrees.values().cloned().collect();
    degree_values.sort_unstable(); 
    let min = *degree_values.first().unwrap();
    let max = *degree_values.last().unwrap(); 
    let sum: usize = degree_values.iter().sum(); 
    let count = degree_values.len(); 
    let mean = sum as f64 / count as f64; 

    // calculates median degree
    let mid = count / 2;
    let median = if count % 2 == 0 {
        (degree_values[mid - 1] + degree_values[mid]) / 2
    } else {
        degree_values[mid]
    };

    // calculates percentiles
    let thresholds = [100, 250, 500, 750, 1000, 1250, 1500, 1750, 2000];
    let mut percentiles = Vec::new();
    let mut last_count: f64 = 0.0;
    for &threshold in &thresholds {
        let count_up_to_threshold = degree_values.iter().filter(|&&d| d <= threshold).count();
        let percentile = (count_up_to_threshold as f64 / count as f64) * 100.0;
        percentiles.push((threshold, percentile - last_count));
        last_count = percentile;
    }

    (min, max, mean, median, percentiles) 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // tests the calculate_statistics function 
    #[test]
    fn test_calculate_statistics() {
        // creates a sample HashMap 
        let degrees: HashMap<String, usize> = [
            ("A1".to_string(), 1),
            ("A2".to_string(), 2),
            ("A3".to_string(), 3),
            ("A4".to_string(), 4),
            ("A5".to_string(), 5)
        ].iter().cloned().collect();

        // calculates statistics from the degrees HashMap
        let (min_degree, max_degree, mean_degree, median_degree, percentiles) = calculate_statistics(&degrees);

        assert_eq!(min_degree, 1); 
        assert_eq!(max_degree, 5); 
        assert_eq!(mean_degree, 3.0); 
        assert_eq!(median_degree, 3); 
    }

    // tests the calculate_degree2 function
    #[test]
    fn test_calculate_degree2() {
        // initializes a HashMap of airports, setting each with an initial degree and degree2 of 0
        let mut airports: HashMap<String, Airport> = HashMap::from([
            ("A1".to_string(), Airport { name: "Airport 1".to_string(), id: "A1".to_string(), degree: 0, degree2: 0 }),
            ("A2".to_string(), Airport { name: "Airport 2".to_string(), id: "A2".to_string(), degree: 0, degree2: 0 }),
            ("A3".to_string(), Airport { name: "Airport 3".to_string(), id: "A3".to_string(), degree: 0, degree2: 0 }),
            ("A4".to_string(), Airport { name: "Airport 4".to_string(), id: "A4".to_string(), degree: 0, degree2: 0 }),
        ]);

        // defines the adjacency list to simulate direct connections between airports
        let adjacency_list: HashMap<String, Vec<String>> = HashMap::from([
            ("A1".to_string(), vec!["A2".to_string(), "A3".to_string()]),
            ("A2".to_string(), vec!["A1".to_string(), "A3".to_string(), "A4".to_string()]),
            ("A3".to_string(), vec!["A1".to_string(), "A2".to_string()]),
            ("A4".to_string(), vec!["A2".to_string()]),
        ]);        

        calculate_degree2(&mut airports, &adjacency_list);

        assert_eq!(airports.get("A1").unwrap().degree2, 1); 
        assert_eq!(airports.get("A2").unwrap().degree2, 0); 
        assert_eq!(airports.get("A3").unwrap().degree2, 1); 
        assert_eq!(airports.get("A4").unwrap().degree2, 2); 
    }
}