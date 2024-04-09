#include "maze.cc"
#include "maze.h"
#include <iostream>

int main(int argc, char *argv[]) {
  Coord *grid_start;
  Coord *grid_finish;
  if (argc == 2) {
    load_grid(argv[1]);
  } else {
    std::cout << "Using Default Maze\n";
  }
  for (int row = 0; row < ROWS; row++) {
    for (int col = 0; col < COLS; col++) {
      if (grid[row][col] == START) {
        grid_start = new Coord(row, col);
      }
      if (grid[row][col] == GOAL) {
        grid_finish = new Coord(row, col);
      }
    }
  }
  std::cout << "Start: " << grid_start->x << ", " << grid_start->y << "\n";
  std::cout << "Finish: " << grid_finish->x << ", " << grid_finish->y << "\n";
  print_grid();
  Coord *fin = bfs(grid_start, grid_finish);
  if (*fin == *grid_finish) {
    draw_back(fin);
  } else {
    std::cout << "No path found :(\n";
  }

  return 0;
}
