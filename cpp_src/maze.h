#ifndef MAZE_H
#define MAZE_H

#define COLS 50
#define ROWS 20
#define BUFSIZE ROWS *COLS + 20 // plus 20

#include <cstring>
#include <deque>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdio.h>
#include <stdlib.h>
#include <string>
#include <sys/stat.h>
#include <vector>

typedef enum Symbol { PATH, WALL, WALK, START, GOAL, UNK } Symbol;

typedef struct Point {
  int row;
  int col;
  bool operator == (const Point &p){ 
    return (row == p.row && col == p.col) ;
  } 
} Point;

typedef struct Node {
  Symbol symbol;
  int row;
  int col;
  bool operator == (const Node &p){ 
    return (row == p.row && col == p.col && symbol == p.symbol) ;
  } 
} Node;

typedef struct Path {
  Node *last_point;
} Path;


typedef struct Maze {
  Symbol imap[ROWS][COLS];
  Node *nodes[ROWS][COLS];
  Node *start;
  Node *end;
  std::vector<Node*> get_walkable_neighbors(int row, int col);
  Path *bfs(void);  
  void print_map(void);
} Maze;

Symbol char_to_sym(char c) {
  switch (c) {
  case ' ':
    return PATH;
  case '#':
    return WALL;
  case 'S':
    return START;
  case 'F':
    return GOAL;
  case '.':
    return WALK;
  default:
    return UNK;
  }
}

char sym_to_char(Symbol s) {
  switch (s) {
  case PATH:
    return ' ';
  case WALL:
    return '#';
  case START:
    return 'S';
  case GOAL:
    return 'F';
  case WALK:
    return '.';
  default:
    return '?';
  }
}

int load_maze(std::string file_name, Maze *maze);

void Maze::print_map(void) {
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      // printf("%c", sym_to_char(m->imap[i][j]));
      std::cout << sym_to_char(this->nodes[i][j]->symbol) ;
    }
    std::cout << std::endl;
  }
}
/*From Lab1 code*/
int fileSize(char *filename) {
  struct stat file_status;
  if (stat(filename, &file_status) < 0)
    return -1;
  else
    return file_status.st_size;
}

#endif // MACRO
