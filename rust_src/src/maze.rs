use std::collections::VecDeque;
// use std::rc::Rc;
use std::{fs, usize};
const ROWS: usize = 20;
const COLS: usize = 50;

const WALKABLE: i32 = 1;
const WALL: i32 = 0;
const STEPS: i32 = 2;
const GOAL: i32 = 5;
const START: i32 = 4;
pub struct Maze {
    pub map: [[i32; COLS]; ROWS],
    pub start: (usize, usize),
    pub fin: (usize, usize),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VisitedCell {
    pub coord: (usize, usize),
    /* memory implications */
    pub ancestors: VecDeque<(usize, usize)>,
}
pub fn build_maze(file_name: String) -> Maze {
    let file = fs::read_to_string(file_name).unwrap();
    let lines = file.lines();
    let mut mealhouse: [[i32; COLS]; ROWS] = [[0; COLS]; ROWS];
    let mut start: (usize, usize) = (0, 0);
    let mut fin: (usize, usize) = (0, 0);
    for (i, line) in lines.enumerate() {
        // TODO: handle if the maze is not up to 50!
        for (j, symbol) in line.chars().enumerate() {
            // TODO: use types instead of strings
            // if i >= ROWS || j >= COLS {
            //     panic!("The maze is not the right shape!");
            // }
            match symbol {
                ' ' => {
                    // See: https://doc.rust-lang.org/std/vec/struct.Vec.html
                    // There has GOT to be a better way of doing this
                    if let Some(row) = mealhouse.get_mut(i) {
                        if let Some(ele) = row.get_mut(j) {
                            *ele = WALKABLE;
                        }
                    }
                    // mealhouse[i][j] = WALKABLE;
                }
                'S' => {
                    if let Some(row) = mealhouse.get_mut(i) {
                        if let Some(ele) = row.get_mut(j) {
                            *ele = START;
                            start = (i, j);
                        }
                    }
                    // mealhouse[i][j] = START;
                }
                'F' => {
                    if let Some(row) = mealhouse.get_mut(i) {
                        if let Some(ele) = row.get_mut(j) {
                            *ele = GOAL;
                            fin = (i, j);
                        }
                    }
                    // mealhouse[i][j] = GOAL;
                }
                _ => {
                    if let Some(row) = mealhouse.get_mut(i) {
                        if let Some(ele) = row.get_mut(j) {
                            *ele = WALL;
                        }
                    }
                    // mealhouse[i][j] = WALL;
                }
            }
        }
    }
    Maze {
        map: mealhouse,
        start,
        fin,
    }
}
fn build_vc_from_parent(coord: (usize, usize), parent: &VisitedCell) -> VisitedCell {
    /* HEAVY memory implications */
    let mut ancestors = parent.ancestors.clone();
    ancestors.push_front(parent.coord);
    VisitedCell { coord, ancestors }
}
#[allow(dead_code)]
impl Maze {
    pub fn dbg_print(&self) {
        for i in self.map {
            for num in i {
                print!("{}, ", num);
            }
            print!("\n");
        }
    }
    pub fn pretty_print(&self) {
        println!("+--------------------------------------------------+");
        for line in self.map {
            print!("|");
            for num in line {
                match num {
                    START => {
                        print!("S");
                    }
                    WALL => {
                        print!("#");
                    }
                    GOAL => {
                        print!("F");
                    }
                    STEPS => {
                        print!(".");
                    }
                    WALKABLE => {
                        print!(" ");
                    }
                    _ => {
                        print!("?")
                    }
                }
            }
            print!("|\n");
        }
        println!("+--------------------------------------------------+");
    }
    fn get_walkable_neighbors(&self, point: VisitedCell) -> Vec<VisitedCell> {
        let xs = [-1, 1, 0, 0];
        let ys = [0, 0, 1, -1];
        let x: i32 = point.coord.0.try_into().unwrap();
        let y: i32 = point.coord.1.try_into().unwrap();
        let mut out: Vec<VisitedCell> = Vec::new();
        for i in 0..xs.len() {
            let nx = x + xs[i];
            let ny = y + ys[i];
            // dbg!("{} {}",&nx,&ny);
            if nx < 0 || nx >= ROWS.try_into().unwrap() {
                continue;
            }
            if ny < 0 || ny >= COLS.try_into().unwrap() {
                continue;
            }
            let ix: usize = nx.try_into().unwrap();
            let iy: usize = ny.try_into().unwrap();
            // println!(" {} ({} {}) -> {}",self.map[ix][iy],ix,iy,i);
            if self.map[ix][iy] != WALL {
                let new = build_vc_from_parent((ix, iy), &point);
                out.push(new);
            }
        }
        // dbg!(&out);
        out
    }
    pub fn draw_back(&mut self, vc: VisitedCell) {
        let mut drawable = vc.ancestors.clone();
        drawable.pop_back();
        for cell in drawable {
            self.map[cell.0][cell.1] = STEPS;
        }
        self.pretty_print();
    }
    pub fn bfs(&mut self) -> Option<VisitedCell> {
        let mut queue: VecDeque<VisitedCell> = VecDeque::new();
        let mut been_to: Vec<(usize, usize)> = Vec::new();
        let agent: VisitedCell = VisitedCell {
            coord: self.start,
            ancestors: VecDeque::from([]),
        };
        queue.push_back(agent);
        while !queue.is_empty() {
            let parent = queue.pop_front().unwrap();
            let neighbors: Vec<VisitedCell> = self.get_walkable_neighbors(parent);
            for child in neighbors {
                /* FIXME */
                let child_clone = child.clone();
                if !been_to.contains(&child.coord) {
                    been_to.push(child.coord);
                    queue.push_back(child_clone);
                    // agent.goto(child);
                    if child.coord == self.fin {
                        return Some(child);
                    }
                }
            }
        }
        None
    }
}
