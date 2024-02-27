mod maze;

fn main() {
    let mut m = maze::build_maze(String::from("maze"));
    // m.pretty_print();
    m.bfs();
}
