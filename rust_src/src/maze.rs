use std::fs;
// use std::alloc::System;
use std::collections::VecDeque;

pub struct Maze {
    // 50x20 maze hard-coded
    pub map: [[i32; 50]; 20],
    pub nodes: [[Node; 50];20],
    pub start: Node,
    pub fin: Node
}

pub struct Agent{
    pub node: Node,
    pub been_to: Vec<Node>
}

struct Node{
    pub coors: (usize,usize),
    pub north: (usize,usize),
    pub south: (usize,usize),
    pub east:  (usize,usize),
    pub west:  (usize,usize),
}

fn build_node(x:usize,y:usize)-> Node{
    // FIXME
    let mut north = (x,y);
    let mut south = (x,y);
    let mut east = (x,y);
    let mut west = (x,y);
    if x+1 < 20 {
        north = (x+1,y);
    }
    if x-1 >= 0{
        south = (x-1,y);
    }
    if y+1 < 50{
        east = (x,y+1);
    }
    if y-1 >= 0{
        west = (x,y-1);
    }
    Node {
        coors: (x,y),
        north: north,
        south: south,
        east: east,
        west: west
    }
}

pub fn build_agent(point: Node) -> Agent{
    Agent {
        node: point,
        been_to: vec!{point}
    }
}

pub fn build_maze(file_name: String) -> Maze {
    let file =  fs::read_to_string(file_name).unwrap();
    let lines = file.lines();
    let mut mealhouse: [[i32; 50]; 20] = [[0;50]; 20];
    let mut nodes: [[Node; 50]; 20];
    let mut start: Node = build_node(0, 0);
    let mut fin: Node = build_node(0, 0);
    for (i,line) in lines.enumerate() {
        for (j, symbol) in line.chars().enumerate() {
            match symbol {
                ' ' => { 
                    mealhouse [i][j] = 0;
                    nodes [i][j] = build_node(i,j);
                }
                'S' => { 
                    mealhouse [i][j] = 2;
                    nodes [i][j] = build_node(i,j);
                    start = build_node(i,j);
                }
                'F' => { 
                    mealhouse [i][j] = 3;
                    nodes [i][j] = build_node(i,j);
                    fin = build_node(i,j);
                }
                _   => { 
                    mealhouse [i][j] = 1;
                    nodes [i][j] = build_node(i,j);
                }
            }


        }
    }
    Maze { 
        map : mealhouse,
        nodes: nodes,
        start : start,
        fin : fin
    }
}

impl Maze{
    pub fn dbg_print(&self){
        for i in self.map {
            for num in i {
                print!("{}, ",num);
            }
            print!("\n");
        }
    }

    pub fn pretty_print(&self){
        for i in self.map {
            for num in i {
                match num {
                    -100 => {
                        print!("S");
                    }
                    1 => {
                        print!("#");
                    }
                    100 => {
                        print!("F");
                    }
                    _ => {
                        print!(" ")
                    }

                }
            }
            print!("\n");
        }
    }

    pub fn solve_maze(&self){
        //BFS
        let mut queue: VecDeque<Node> = VecDeque::new();
        let agent : Agent = build_agent(self.start);
        let mut neighbors : Vec<&Node>;
        let mut cur: Node;
        queue.push_back(agent.node);
        while !queue.is_empty() {
           cur = queue.pop_front().unwrap();
           neighbors = cur.get_walkable_neighbors(self);
           for n in neighbors {
            if 
           }
        }
    }
}

impl Node {
    fn get_walkable_neighbors<'a, 'b>(&'a self, maze: &'b Maze) -> Vec<&'b Node>{
        let mut out: Vec<&Node> = Vec::new();  
        if maze.map[self.north.0][self.north.1] == 1{
            out.push(
                &maze.nodes[self.north.0][self.north.1]
            )
        }
        if maze.map[self.south.0][self.south.1] == 1{
            out.push(
                &maze.nodes[self.south.0][self.south.1]
            )
        }
        out
    }
}