#include "maze.h"
#include <vector>

int char_to_int(char **p) {
  while (1) {
    char a = **p;
    *p = *p + 1;
    if (a == ' ')
      return PATH;
    else if (a == '#')
      return WALL;
    else if (a == 'S')
      return START;
    else if (a == 'F')
      return GOAL;
  }
  return -1;
}

int get_file_size(char *filename) {
  struct stat file_status;
  if (stat(filename, &file_status) < 0)
    return -1;
  else
    return file_status.st_size;
}

int load_grid(char *filename) {
  char buf[ROWS * COLS * 2], *p;
  int row, col;

  printf("Loading maze ...");
  FILE *file = fopen(filename, "r");
  if (file == NULL) {
    printf("Failed to load puzzle file %s\n", filename);
    return 1;
  }

  int fileSize = get_file_size(filename);
  // int fileSize = ((ROWS + 1) * COLS) * sizeof(char);
  fread(buf, 1, fileSize, file);

  p = &buf[0];
  for (row = 0; row < ROWS; row++) {
    for (col = 0; col < COLS; col++) {
      char c = char_to_int(&p);
      grid[row][col] = c;
    }
  }

  fclose(file);
  printf(" done loading maze!\n");
  return 0;
}
std::vector<Coord> neighbors(Coord *c) {
  std::vector<Coord> out;
  int x = c->x;
  int y = c->y;
  int xs[4] = {-1, 1, 0, 0};
  int ys[4] = {0, 0, 1, -1};
  for (int i = 0; i < 4; i++) {
    int row = c->x + xs[i];
    int col = c->y + ys[i];
    if ((col < 0 || col >= COLS) || (row < 0 || row >= ROWS)) {
      continue;
    }
    if (grid[row][col] != WALL) {
      out.push_back(Coord(row, col));
    }
  }
  return out;
}
std::vector<Coord> bfs(void) {
  std::vector<Coord> out;
  std::deque<Coord> q;
  return out;
}
