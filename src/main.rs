use crate::metro_graph::Data;

mod metro_graph;
mod json_convert;

fn main() {
    let metro_graph = metro_graph::MetroGraph::new();
    let departure = Data::new(String::from("노원"), String::from("4호선"));
    let destination = Data::new(String::from("서울역"), String::from("4호선"));

    metro_graph.find_path(&departure, &destination);
}