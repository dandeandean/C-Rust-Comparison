#include "maze.h"

Maze *init_maze(char *file_name) {
  Maze *out_maze = (Maze *) malloc(sizeof(Maze));
  memset(out_maze->imap, 33, sizeof(int) * COLS * ROWS);
  load_maze(file_name, out_maze);
  return out_maze;
}

Node *init_node(Symbol s, int row, int col) {
  Node *out_node = (Node *) malloc(sizeof(Node));
  out_node->symbol = s;
  out_node->row = row;
  out_node->col = col;
  return out_node;
}

/*
  Loads the text file into the Maze object
*/
int load_maze(std::string file_name, Maze *maze) {
  std::ifstream file;
  file.open(file_name);
  std::string line;
  Symbol line_buf[COLS];
  int row_counter = 0;
  while(std::getline(file,line)){
    for (int i=0; i<line.length(); i++){
      // std::cout << char_to_sym(line[i]);
      line_buf[i] = char_to_sym(line[i]);
    }
    /*Actual copyinging into the buf*/
    std::copy( std::begin(line_buf), std::end(line_buf), std::begin(maze->imap[row_counter]));
    row_counter ++;
  }
  file.close();
  return 0;
}

std::vector<Point> Maze::get_walkable_neighbors(Point p){
  int xs[4] = {-1,1,0,0};
  int ys[4] = {0,0,1,-1};
  std::vector<Point> out;
  return out;
}