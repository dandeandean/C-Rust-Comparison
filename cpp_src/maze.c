#include <stdlib.h>
#include <stdio.h>
#include <sys/stat.h>
#include "maze.h"
#include <string.h>

int map[ROWS][COLS];
typedef enum Symbol{
    PATH,
    WALL,
    WALK,
    DUNNO 
} Symbol;

int fileSize(char *filename) {
    /*From Lab1 code*/
    struct stat file_status;
    if (stat(filename, &file_status) < 0)
        return -1;
    else
        return file_status.st_size;
}

int loadMaze(char * fileName){
    char buf[BUFSIZE];

    FILE * fd = fopen(fileName, "r");
    if (fd==NULL) {
        printf("Failed to load %s \n", fileName);
        return -1;
    }

    fread(buf, sizeof(char), fileSize(fileName), fd);
    printf(" file size = %d\n", fileSize(fileName));
    printf("%s\n",buf);
    // memcpy(map,(const void *)buf,fileSize(fileName));
    // for (int i=0; i < ROWS; i++){
    //     for (int j=0; j < COLS; j++){
    //         printf("%c",map[i][j]);
    //     }
    // }
    printf("Done!\n");
    return 0;
}

void print_map(void){
    for (int i=0; i < ROWS; i++){
        for (int j=0; j < COLS; j++){
            printf("%d",map[i][j]);
        }
    }
}
