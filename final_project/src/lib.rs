use csv::Reader;
use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize, Clone)]
pub struct Airport {
    #[serde(rename = "Label")]
    pub name: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(skip)]
    pub degree: usize,
    #[serde(skip)]
    pub degree2: usize,  // New field to count second-degree neighbors
}

#[derive(Debug, Deserialize)]
pub struct Route {
    #[serde(rename = "Departure")]
    pub departure_id: String,
    #[serde(rename = "Destination")]
    pub destination_id: String,
}

pub fn read_airports(path: &str) -> Result<HashMap<String, Airport>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut airports = HashMap::new();
    for result in rdr.deserialize() {
        let mut airport: Airport = result?;
        airport.degree = 0;
        airports.insert(airport.id.clone(), airport);
    }
    Ok(airports)
}

pub fn read_routes(path: &str) -> Result<Vec<Route>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut routes = Vec::new();
    for result in rdr.deserialize() {
        let route: Route = result?;
        routes.push(route);
    }
    Ok(routes)
}

pub fn update_degrees(airports: &mut HashMap<String, Airport>, routes: &[Route]) -> HashMap<String, Airport> {
    let mut airports100 = HashMap::new();
    
    for route in routes {
        if let Some(dep) = airports.get_mut(&route.departure_id) {
            dep.degree += 1;
            if dep.degree >= 100 {
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

    airports100
}

pub fn calculate_degree2(airports: &mut HashMap<String, Airport>, adjacency_list: &HashMap<String, Vec<String>>) {
    for (airport_id, airport) in airports.iter_mut() {
        let mut neighbors2 = HashSet::new();  // To keep track of unique second-degree neighbors

        if let Some(neighbors) = adjacency_list.get(airport_id) {
            for neighbor in neighbors {
                if let Some(second_neighbors) = adjacency_list.get(neighbor) {
                    for second_neighbor in second_neighbors {
                        if second_neighbor != airport_id && !neighbors.contains(second_neighbor) {
                            neighbors2.insert(second_neighbor);
                        }
                    }
                }
            }
        }

        airport.degree2 = neighbors2.len();  // Set the count of unique second-degree neighbors
    }
}

pub fn calculate_statistics(degrees: &HashMap<String, usize>) -> (usize, usize, f64, usize, Vec<(usize, f64)>) {
    let mut degree_values: Vec<usize> = degrees.values().cloned().collect();
    if degree_values.is_empty() {
        return (0, 0, 0.0, 0, vec![]);
    }

    degree_values.sort_unstable();
    let min = *degree_values.first().unwrap();
    let max = *degree_values.last().unwrap();
    let sum: usize = degree_values.iter().sum();
    let count = degree_values.len();
    let mean = sum as f64 / count as f64;

    // Calculate median
    let mid = count / 2;
    let median = if count % 2 == 0 {
        (degree_values[mid - 1] + degree_values[mid]) / 2
    } else {
        degree_values[mid]
    };

    // Calculate cumulative percentiles
    let thresholds = [100, 250, 500, 750, 1000, 1250, 1500, 1750, 2000];
    let mut percentiles = Vec::new();
    let mut last_count: f64 = 0.0;  // Change type to f64
    for &threshold in &thresholds {
        let count_up_to_threshold = degree_values.iter().filter(|&&d| d <= threshold).count();
        let percentile = (count_up_to_threshold as f64 / count as f64) * 100.0;
        percentiles.push((threshold, percentile - last_count));
        last_count = percentile;  // Ensure last_count is also f64
    }

    (min, max, mean, median, percentiles)
}
