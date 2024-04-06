#include "maze.cc"

int main(int argc, char *argv[]) {
  if (argc == 2) {
    printf("Loading Maze\n");
    load_grid(argv[1]);
  }
  printf("Using Default Maze\n");
  print_grid();
  return 0;
}
