#include <stdio.h>
#include "maze.c"

int main(int argc, char * argv[]){
    if (argc != 2) {
        printf("Not enough arguments!\n");
        return 1;
    }
    printf("Parsing: %s\n",argv[1]);
    loadMaze(argv[1]);
    return 0;
}