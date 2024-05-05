use final_project::Route;
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

#[cfg(test)]
mod tests {
    use super::*;
    use final_project::Route;

    #[test]
    fn test_calculate_switches() {
        // sets up the test data for the routes
        let routes = vec![
            Route {
                departure_id: "A1".to_string(),
                destination_id: "A2".to_string(),
            },
            Route {
                departure_id: "A2".to_string(),
                destination_id: "A3".to_string(),
            },
            Route {
                departure_id: "A3".to_string(),
                destination_id: "A4".to_string(),
            },
            Route {
                departure_id: "A1".to_string(),
                destination_id: "A4".to_string(),
            }
        ];

        // creates an instance of the AirportGraph using the routes defined above
        let graph = AirportGraph::new(routes);

        // calculates switches from airport A1
        let distances = graph.calculate_switches("A1");

        // assertions to ensure the distances are as expected
        assert_eq!(distances.get("A1").cloned().unwrap_or(usize::MAX), 0, "Distance from A1 to itself should be 0");
        assert_eq!(distances.get("A2").cloned().unwrap_or(usize::MAX), 1, "Distance from A1 to A2 should be 1");
        assert_eq!(distances.get("A3").cloned().unwrap_or(usize::MAX), 2, "Distance from A1 to A3 should be 2");
        assert_eq!(distances.get("A4").cloned().unwrap_or(usize::MAX), 1, "Distance from A1 to A4 should be 1 through the direct route");
    }
}