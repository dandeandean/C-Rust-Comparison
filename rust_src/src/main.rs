mod maze;

fn main() {
    let m = maze::build_maze(String::from("maze"));
    let _a = maze::build_agent(0,0);
    m.pretty_print();
    m.dbg_print();
}
