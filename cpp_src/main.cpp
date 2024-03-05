#include "maze.cpp"
#include <stdio.h>

int main(int argc, char *argv[]) {
  if (argc != 2) {
    printf("Usage: ./run <maze-file>\n");
    return 1;
  }
  std::cout << "Parsing:" << argv[1] << "\n";
  Maze *maze = init_maze(argv[1]);
  print_map(maze);
  return 0;
}
