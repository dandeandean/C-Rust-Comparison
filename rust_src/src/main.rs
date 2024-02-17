mod maze;

fn main() {
    let m = maze::build_maze(String::from("maze"));
    dbg!("{}",m.map);
}
