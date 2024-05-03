use csv::Reader;
use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Airport {
    #[serde(rename = "Label")]
    pub name: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    pub longitude: f64,
    #[serde(skip)]
    pub degree: usize,
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

