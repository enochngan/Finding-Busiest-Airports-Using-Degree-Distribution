use crate::lib::{Route};
use std::collections::{HashMap, VecDeque};

// defines a struct to represent an airport graph
pub struct AirportGraph {
    // stores the adjacency list of the graph where each key is an airport ID and the value is a list of adjacent airports
    pub adjacency_list: HashMap<String, Vec<String>>,
}

// implementation for the AirportGraph
impl AirportGraph {
    // constructor method to create a new AirportGraph from a list of routess
    pub fn new(routes: Vec<Route>) -> Self {
        let mut adjacency_list = HashMap::new();
        for route in routes {
            // inserts the destination under the departure key for each route
            adjacency_list.entry(route.departure_id.clone())
                          .or_insert_with(Vec::new)
                          .push(route.destination_id.clone());
        }
        AirportGraph { adjacency_list }
    }

    // calculates the shortest path distances from a source airport to all other airports in the graph using BFS
    pub fn calculate_switches(&self, source_id: &str) -> HashMap<String, usize> {

        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        // sets the distance to the source itself as 0 and adds the source to the BFS queue
        distances.insert(source_id.to_string(), 0);
        queue.push_back(source_id.to_string());

        // processes the queue until empty
        while let Some(current_id) = queue.pop_front() {
            // retrieves neighbors of the current airport
            if let Some(neighbors) = self.adjacency_list.get(&current_id) {
                for neighbor in neighbors {
                    // if the neighbor hasn't been visited (i.e., not in distances), calculate its distance.
                    if !distances.contains_key(neighbor) {
                        distances.insert(neighbor.clone(), distances[&current_id] + 1); // sets distance as 1 or more than current airport
                        queue.push_back(neighbor.clone()); // adds neighbor to queue to continue BFS
                    }
                }
            }
        }
        distances
    }
}