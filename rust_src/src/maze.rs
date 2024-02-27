use std::{fs, usize};
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


#[derive(Debug,Clone)]
pub struct Agent{
    pub location: (usize,usize),
    pub been_to: Vec<(usize,usize)>,
    // pub path: Path
}

#[allow(dead_code)]
#[derive(Debug,Clone, Copy)]
pub struct Path<'p>{
    pub current: (usize,usize),
    pub prev: Option<&'p Path<'p>>
}

#[allow(dead_code)]
fn build_path<'p>(point: (usize,usize), prev: Option<&'p Path<'p>> ) -> Path{
    Path {
        current: point,
        prev: prev
    }
}

fn build_agent(point: (usize,usize)) -> Agent{
    Agent {
        location: point,
        been_to: vec!{point},
        // path : build_path(point, point)
    }
}

pub fn build_maze(file_name: String) -> Maze {
    let file =  fs::read_to_string(file_name).unwrap();
    let lines = file.lines();
    let mut mealhouse: [[i32; COLS]; ROWS] = [[0;COLS]; ROWS];
    let mut start:(usize,usize) = (0,0);
    let mut fin:(usize,usize) = (0,0);
    for (i,line) in lines.enumerate() {
        // TODO: handle if the maze is not up to 50!
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

impl Maze {

    #[allow(dead_code)]
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

    pub fn bfs(&mut self) -> Option<Vec<(usize,usize)>>{
        //BFS
        let mut queue: VecDeque<(usize,usize)> = VecDeque::new();
        let mut agent : Agent = build_agent(self.start);
        queue.push_back(agent.location);
        while !queue.is_empty() {
            let parent = queue.pop_front().unwrap();
            let neighbors: Vec<(usize, usize)> = self.get_walkable_neighbors(parent);
            let mut parent_agent:Agent = build_agent(parent);
            //FIXME: below is the bug!
            parent_agent.been_to = agent.been_to.clone(); 
            for child in neighbors {
                let mut baby_agent: Agent = build_agent(child);
                // baby_agent.been_to = parent_agent.been_to.clone(); 
                baby_agent.been_to.push(parent);
                if ! agent.been_to.contains(&child){
                    queue.push_back(child);
                    agent.goto(child);
                    if child == self.fin {
                        let mut cur = baby_agent.clone();
                        while cur.location != self.start {
                            cur.goback();
                        }
                        return Some(baby_agent.been_to);
                    }
                }
            }
        }
        None
    }

    pub fn draw_back(&mut self, points : Vec<(usize,usize)>) {
        // let mut cur_p: &Path = &p;
        for point in points{
            self.map[point.0][point.1] = PATH;
        }
        self.map[self.start.0][self.start.1] = START;
        self.map[self.fin.0][self.fin.1] = GOAL;
        self.pretty_print();
    }
}

impl Agent {
    fn goto(&mut self, new_location: (usize,usize)) {
        self.been_to.push(self.location);
        self.location = new_location;
    }
    fn goback(&mut self) {
        let last = self.been_to.pop().unwrap();
        self.location = last;
    }
}