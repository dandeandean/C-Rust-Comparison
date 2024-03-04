#include "maze.h"

Maze *init_maze(char *file_name) {
  // I think this may be a problem
  // Node ** map = malloc(rows*sizeof(Node*));
  // for (int i=0; i<cols; i++){
  //     map[i] = malloc(cols*sizeof(Node));
  // }
  // Node map [ROWS][COLS];
  Maze *out_maze = malloc(sizeof(Maze));
  memset(out_maze->imap, 33, sizeof(int) * COLS * ROWS);
  load_maze(file_name, out_maze);
  return out_maze;
}

Node *init_node(Symbol s, int row, int col) {
  Node *out_node = malloc(sizeof(Node));
  out_node->symbol = s;
  out_node->row = row;
  out_node->col = col;
  return out_node;
}

int load_maze(char *file_name, Maze *maze) {
  int file_size = fileSize(file_name);
  /*Should inject bug here*/
  char buf[file_size];
  FILE *fd = fopen(file_name, "r");
  if (fd < 0) {
    return -1;
  }
  fread(buf, sizeof(char), file_size, fd);

  /*Convert buf into nodes in maze*/
  /*Maybe go though and just memcpy into char array,
      Then go back and conver*/
  char row_buf[COLS];
  int row_conter = 0, col_counter = 0, offset = 0;
  for (int i = 0; i < file_size; i++) {
    col_counter = i % COLS;
    if (buf[i] == '\n') {
      printf(" row = %d , offset = %d ", row_conter, offset);
      row_conter++;
      offset++;
      printf("\n");
      printf("%s ", row_buf);
      memset(row_buf, '.', sizeof(char) * COLS);
      row_buf[COLS - 1] = '\n';
    } else {
      maze->imap[row_conter][col_counter] = char_to_sym(buf[i]);
      row_buf[col_counter] = buf[i];
      printf("%c", buf[i]);
      // printf("|%x|",maze->imap[row_conter][col_counter-offset]);
      col_counter++;
    }
  }

  fclose(fd);
  return 0;
}
