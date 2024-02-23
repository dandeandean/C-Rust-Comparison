use std::fs;
// use std::alloc::System;
use std::collections::VecDeque;

const ROWS:usize = 20;
const COLS:usize = 50;

const PATH:i32 = 9;
const WALK:i32 = 1;
const WALL:i32 = 0;
const GOAL:i32 = 3;
const START:i32 = 2;
const UNK:i32 = -1;
pub struct Maze {
    pub map: [[i32; COLS]; ROWS],
    pub start: (usize,usize),
    pub fin: (usize,usize),
}

pub struct Agent{
    pub location: (usize,usize),
    pub been_to: Vec<(usize,usize)>,
    // pub path: Vec<(usize,usize)>
}


fn build_agent(point: (usize,usize)) -> Agent{
    Agent {
        location: point,
        been_to: vec!{point}
    }
}

pub fn build_maze(file_name: String) -> Maze {
    let file =  fs::read_to_string(file_name).unwrap();
    let lines = file.lines();
    let mut mealhouse: [[i32; COLS]; ROWS] = [[0;COLS]; ROWS];
    let mut start:(usize,usize) = (0,0);
    let mut fin:(usize,usize) = (0,0);
    for (i,line) in lines.enumerate() {
        for (j, symbol) in line.chars().enumerate() {
            match symbol {
                ' ' => { 
                    mealhouse [i][j] = WALK;
                }
                'S' => { 
                    mealhouse [i][j] = START;
                    start = (i,j);
                }
                'F' => { 
                    mealhouse [i][j] = GOAL;
                    fin = (i,j);
                }
                '#' =>{
                    mealhouse [i][j] = WALL;
                }
                _   => { 
                    mealhouse [i][j] = UNK;
                }
            }
        }
    }
    Maze { 
        map : mealhouse,
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
                    START => {
                        print!("S");
                    }
                    WALL=> {
                        print!("#");
                    }
                    GOAL => {
                        print!("F");
                    }
                    PATH => {
                        print!(".");
                    }
                    WALK => {
                        print!(" ");
                    }
                    _ => {
                        print!("/")
                    }

                }
            }
            print!("\n");
        }
    }

    fn get_walkable_neighbors(&self, point: (usize,usize)) -> Vec<(usize,usize)>{
        let xs = [-1,1,0,0];
        let ys = [0,0,1,-1];
        let x:i32 = point.0.try_into().unwrap();
        let y:i32 = point.1.try_into().unwrap();
        let mut out : Vec<(usize,usize)> = Vec::new();
        for i in 0..xs.len() {
            let nx = x + xs[i];
            let ny = y + ys[i];
            // dbg!("{} {}",&nx,&ny);
            if nx < 0 || nx >= ROWS.try_into().unwrap() {continue;}
            if ny < 0 || ny >= COLS.try_into().unwrap() {continue;}
            let ix: usize = nx.try_into().unwrap();
            let iy: usize = ny.try_into().unwrap();
            // println!(" {} ({} {}) -> {}",self.map[ix][iy],ix,iy,i);
            if self.map[ix][iy] != WALL {
                out.push((ix,iy));
            }
        }
        // dbg!(&out);
        out
    }

    pub fn bfs(&mut self) -> Agent{
        //BFS
        let mut queue: VecDeque<(usize,usize)> = VecDeque::new();
        let mut agent : Agent = build_agent(self.start);
        queue.push_back(agent.location);
        while !queue.is_empty() && agent.location != self.fin{
            let cur = queue.pop_front().unwrap();
            let neighbors = self.get_walkable_neighbors(cur);
            for coord in neighbors{
                if ! agent.been_to.contains(&coord){
                    queue.push_back(coord);
                    agent.been_to.push(coord);
                    self.map[coord.0][coord.1] = PATH;
                }
            }
        }
        // FIXME
        self.map[self.fin.0][self.fin.1] = GOAL;
        self.pretty_print();
        agent
    }
}
