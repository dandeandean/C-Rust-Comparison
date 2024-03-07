#include "maze.cpp"
#include <stdio.h>

int main(int argc, char *argv[]) {
  if (argc != 2) {
    printf("Usage: ./run <maze-file>\n");
    return 1;
  }
  std::cout << "Parsing:" << argv[1] << "\n";
  Maze *maze = init_maze(argv[1]);
  maze->bfs();
  maze->print_map();
  std::cout<<maze->start->symbol<< "\n";
  std::cout<<maze->nodes[19][48]->symbol<< "\n";
  std::cout<<maze->nodes[19][49]->symbol<< "\n";
  return 0;
}
