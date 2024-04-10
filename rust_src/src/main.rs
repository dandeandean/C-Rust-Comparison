mod maze;
use std::env;
fn main() {
    let args: Vec<_> = env::args().collect();
    let mut file_name = "default-maze";
    if args.len() < 2 {
        println!("No file provided, using default maze.");
    } else {
        file_name = args.last().unwrap();
    }
    println!("Reading... {}", file_name);
    let mut m = maze::build_maze(String::from(file_name));
    m.pretty_print();
    let path = m.bfs().unwrap();
    m.draw_back(path);
}
