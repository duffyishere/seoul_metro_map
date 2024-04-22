use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::json_convert::convert;

#[derive(Debug, Clone, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub line_name: String,
}

impl Data {
    pub fn new(name: String, line_name: String) -> Self {
        Data {
            name,
            line_name,
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Data {}

impl std::hash::Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
pub struct MetroGraph {
    adjacency_list: HashMap<Data, Vec<Data>>,
    edge_weights: HashMap<Data, HashMap<Data, i16>>,
}

impl MetroGraph {
    pub fn new() -> Self {
        let mut graph = MetroGraph {
            adjacency_list: HashMap::new(),
            edge_weights: HashMap::new(),
        };

        for i in 0..10 {
            let stations = convert(&format!("./resources/{}.json", i)).expect("파일 경로 및 내용을 확인해주세요.");
            for i in 0..stations.len() - 1 {
                graph.add_edge(&stations[i], &stations[i + 1].clone(), 1);
            }
        }

        graph
    }

    fn add_edge(&mut self, departure: &Data, destination: &Data, distance: i16) {
        self.adjacency_list.entry(departure.clone()).or_insert_with(Vec::new).push(destination.clone());
        self.edge_weights.entry(departure.clone()).or_insert_with(HashMap::new).insert(destination.clone(), distance);

        self.adjacency_list.entry(destination.clone()).or_insert_with(Vec::new).push(departure.clone());
        self.edge_weights.entry(destination.clone()).or_insert_with(HashMap::new).insert(departure.clone(), distance);
    }

    pub fn find_path(&self, departure: &Data, destination: &Data) -> i16 {
        let mut distance: HashMap<Data, i16> = HashMap::new();
        let mut queue: BinaryHeap<Data> = BinaryHeap::new();
        let mut parent: HashMap<Data, Option<Data>> = HashMap::new();

        distance.insert(departure.clone(), 0);
        queue.push(departure.clone());

        while let Some(current) = queue.pop() {
            if current.eq(&destination) {
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

        self.print_path(&departure, &destination, distance.clone(), parent);

        distance.get(&destination).cloned().unwrap_or(-1)
    }

    fn print_path(&self, departure: &Data, destination: &Data, distance: HashMap<Data, i16>, parent: HashMap<Data, Option<Data>>) {
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
        let s1 = Data::new(String::from("시청역"), String::from("1호선"));
        let s2 = Data::new(String::from("시청역"), String::from("2호선"));

        assert_eq!(s1, s2);
    }
}
