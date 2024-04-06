#include "maze.c"

/* ENTRY POINT*/
int main(int argc, char *argv[]) {
  if (argc == 2) {
    printf("Usage: ./run <maze-file>\n");
    load_grid(argv[1]);
  }
  print_grid();
  return 0;
}
