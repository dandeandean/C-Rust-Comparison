#include <stdlib.h>
#include <stdio.h>
#include <sys/stat.h>

#define BUFSIZE 1000
#define COLS 50
#define ROWS 20

char* map[ROWS][COLS];


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
    int row,col;
    FILE * fd = fopen(fileName, "r");
    if (fd==NULL) {
        printf("Failed to load %s \n", fileName);
        return -1;
    }

    fread(buf, 1, fileSize(fileName), fd);

    printf(" file size = %d\n", fileSize(fileName));
    printf("%s\n",buf);
    for (int row = 0 ; row < ROWS; row ++){
        for (int col = 0; col < COLS; col ++){
            // map[row][col]  = file[]
        }
    }
    printf("Done!\n");
    return 0;
}