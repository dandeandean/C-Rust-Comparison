mod maze;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 1{
        print!("NEED MORE ARGUMENTS!");
    }
    let mut m = maze::build_maze(String::from(args.last().unwrap()));
    m.pretty_print();
    println!();
    let points = m.bfs().unwrap();
    m.draw_back(points);
}
