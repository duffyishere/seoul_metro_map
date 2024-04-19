use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::json_convert::convert;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub name: String,
    pub code: String,
    pub line_name: String,
    pub order: i8,
}

impl Station {
    pub fn with_name(name: String) -> Self {
        Station {
            name,
            code: String::from(""),
            line_name: String::from(""),
            order: 0,
        }
    }
}

impl PartialEq for Station {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Station {}

impl std::hash::Hash for Station {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Ord for Station {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Station {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<Station, Vec<Station>>,
    edge_weights: HashMap<Station, HashMap<Station, i16>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            edge_weights: HashMap::new(),
        }
    }

    pub fn add_metro_line(&mut self, file_path: &String) -> Result<(), Box<dyn Error>> {
        let stations = convert(&file_path)?;
        for i in 0..stations.len() - 1 {
            self.add_edge(stations[i].clone(), stations[i + 1].clone(), 1);
        }
        Ok(())
    }

    fn add_edge(&mut self, departure: Station, destination: Station, distance: i16) {
        self.adjacency_list.entry(departure.clone()).or_insert_with(Vec::new).push(destination.clone());
        self.edge_weights.entry(departure.clone()).or_insert_with(HashMap::new).insert(destination.clone(), distance);

        self.adjacency_list.entry(destination.clone()).or_insert_with(Vec::new).push(departure.clone());
        self.edge_weights.entry(destination).or_insert_with(HashMap::new).insert(departure, distance);
    }

    pub fn shortest_path(&self, departure_name: String, destination_name: String) -> i16 {
        let mut distance: HashMap<Station, i16> = HashMap::new();
        let mut queue: BinaryHeap<Station> = BinaryHeap::new();
        let mut parent: HashMap<Station, Option<Station>> = HashMap::new();

        let departure = Station::with_name(departure_name.clone());
        let destination = Station::with_name(destination_name.clone());

        distance.insert(departure.clone(), 0);
        queue.push(departure.clone());

        while let Some(current) = queue.pop() {
            if current == destination {
                break;
            }
            let current_distance = distance[&current];

            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for neighbor in neighbors {
                    let neighbor_distance = current_distance + self.edge_weights[&current][&neighbor];
                    if !distance.contains_key(&neighbor) || neighbor_distance < distance[&neighbor] {
                        distance.insert(neighbor.clone(), neighbor_distance);
                        queue.push(neighbor.clone());
                        parent.insert(neighbor.clone(), Some(current.clone()));
                    }
                }
            }
        }

        self.print_shortest_path(&departure, &destination, distance.clone(), parent);

        distance.get(&destination).cloned().unwrap_or(-1)
    }

    pub fn print_shortest_path(&self, departure: &Station, destination: &Station, distance: HashMap<Station, i16>, parent: HashMap<Station, Option<Station>>) {
        let mut path = vec![destination.clone()];
        let mut current = destination.clone();
        while let Some(prev) = parent.get(&current).unwrap() {
            path.push(prev.clone());
            current = prev.clone();
            if current == *departure {
                break;
            }
        }
        path.reverse();

        for station in &path {
            print!("{} ({}) -> ", station.name, station.line_name);
        }
        println!("\n========================================================");
        println!("{}에서 {}까지 {}정거장 소요됩니다.", departure.name, destination.name, distance[destination]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        let s1 = Station {
            name: String::from("1"),
            code: String::from("2"),
            line_name: String::from("1"),
            order: 0,
        };

        let s2 = Station {
            name: String::from("1"),
            code: String::from("1"),
            line_name: String::from("1"),
            order: 0,
        };

        assert_eq!(s1, s2);
    }
}
