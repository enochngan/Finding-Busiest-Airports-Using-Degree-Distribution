use crate::lib::{Route};
use std::collections::{HashMap, VecDeque};

pub struct AirportGraph {
    pub adjacency_list: HashMap<String, Vec<String>>,
}

impl AirportGraph {
    pub fn new(routes: Vec<Route>) -> Self {
        let mut adjacency_list = HashMap::new();
        for route in routes {
            adjacency_list.entry(route.departure_id.clone()).or_insert_with(Vec::new).push(route.destination_id.clone());
        }
        AirportGraph { adjacency_list }
    }

    pub fn calculate_switches(&self, source_id: &str) -> HashMap<String, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        distances.insert(source_id.to_string(), 0);
        queue.push_back(source_id.to_string());

        while let Some(current_id) = queue.pop_front() {
            if let Some(neighbors) = self.adjacency_list.get(&current_id) {
                for neighbor in neighbors {
                    if !distances.contains_key(neighbor) {
                        distances.insert(neighbor.clone(), distances[&current_id] + 1);
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        distances
    }
}