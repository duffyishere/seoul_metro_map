mod seoul_metro_map;
mod json_convert;

fn main() {
    let mut g = seoul_metro_map::Graph::new();
    let departure = String::from("노원");
    let destination = String::from("서울역");

    for i in 0..10 {
        g.add_metro_line(&format!("./resources/{}.json", i))
            .expect("파일 경로 및 내용을 확인해주세요.");
    }
    g.shortest_path(departure.clone(), destination.clone());
}