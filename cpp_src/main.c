#include <stdio.h>
#include "maze.c"

int main(int argc, char * argv[]){
    if (argc != 2) {
        printf("Not enough arguments!\n");
        return 1;
    }
    printf("Parsing: %s\n",argv[1]);
    // load_maze(argv[1]);
    Maze * maze = init_maze(argv[1]);

    printf("\n");
    print_map(maze);
    return 0;
}