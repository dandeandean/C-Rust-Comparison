mod maze;

fn main() {
    let m = maze::build_maze(String::from("maze"));
    m.pretty_print();
    m.dbg_print();
}
