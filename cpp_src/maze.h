#ifndef MAZE
#define MAZE

#define BUFSIZE 1020
#define COLS 50
#define ROWS 20
typedef struct Node{
    char symbol;
    int x;
    int y;
}Node;

typedef struct Path {
    Node last_node;
} Path;

#endif // MACRO
