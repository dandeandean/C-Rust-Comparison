use std::fs;
// use std::{cell::RefCell, rc::Rc};

// #[derive(Debug, Clone)]
// pub struct Node {
//     // This design could definitely use some work
//     up: Option<Rc<RefCell<Node>>>,
//     down: Option<Rc<RefCell<Node>>>,
//     left: Option<Rc<RefCell<Node>>>,
//     right: Option<Rc<RefCell<Node>>>,
//     kind: bool,
//     coors: (i32,i32)
// }

pub struct Maze {
    // 50x20 maze hard-coded
    pub map: [[i32; 50]; 20],
    pub start: (i32,i32),
    pub fin: (i32,i32)
}

pub struct Agent{
    pub coors: (i32,i32),
}

pub fn build_agent(x:i32, y:i32) -> Agent{
    Agent {
        coors: (x,y),
    }
}

pub fn build_maze(file_name: String) -> Maze {
    let file =  fs::read_to_string(file_name).unwrap();
    let lines = file.lines();
    let mut mealhouse: [[i32; 50]; 20] = [[0;50]; 20];
    let mut start: (i32,i32) = (0,0);
    let mut fin: (i32,i32) = (0,0);
    for (i,line) in lines.enumerate() {
        for (j, symbol) in line.chars().enumerate() {
            match symbol {
                ' ' => { mealhouse [i][j] = 0}
                'S' => { 
                    mealhouse [i][j] = -100;
                    start = (i.try_into().unwrap(),j.try_into().unwrap());
                }
                'F' => { 
                    mealhouse [i][j] = 100;
                    fin = (i.try_into().unwrap(),j.try_into().unwrap());
                }
                _   => { mealhouse [i][j] = 1}
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
}