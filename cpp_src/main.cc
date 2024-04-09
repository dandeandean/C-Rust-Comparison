#include "maze.cc"
#include "maze.h"
#include <iostream>

int main(int argc, char *argv[]) {
  if (argc == 2) {
    printf("Loading Maze\n");
    load_grid(argv[1]);
  }
  printf("Using Default Maze\n");
  print_grid();
  Coord *fin = bfs();
  // std::cout << fin->x << ", " << fin->y << "\n";
  if (*fin == *grid_finish) {
    draw_back(fin);
  } else {
    std::cout << "No path found :(\n";
  }

  return 0;
}
