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
    pub map: [[i32; 50]; 20] 
}

pub fn build_maze(file_name: String) -> Maze {
    let file =  fs::read_to_string(file_name).unwrap();
    let lines = file.lines();
    let mut mealhouse: [[i32; 50]; 20] = [[0;50]; 20];
    for (i,line) in lines.enumerate() {
        for (j, symbol) in line.chars().enumerate() {
            match symbol {
                ' ' => { mealhouse [i][j] = 0}
                'S' => { mealhouse [i][j] = -100}
                'F' => { mealhouse [i][j] = 100}
                _   => { mealhouse [i][j] = 1}
            }


        }
    }
    Maze { map : mealhouse}
}
