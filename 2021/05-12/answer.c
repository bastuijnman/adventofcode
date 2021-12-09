#include <stdio.h>
#include <stdlib.h>

int min (int a, int b) { return a < b ? a : b; }
int max (int a, int b) { return a > b ? a : b;}

int calculate_overlaps (int mode) {
    FILE *f = fopen("input.txt", "r");

    int maxX = 0, maxY = 0;
    int x1,y1,x2,y2;

    // Figure out max coords
    while (fscanf(f, "%d,%d -> %d,%d", &x1, &y1, &x2, &y2) == 4) {
        if (x1 > maxX) maxX = x1;
        if (x2 > maxX) maxX = x2;
        if (y1 > maxY) maxY = y1;
        if (y2 > maxY) maxY = y2;
    }

    // Make sure maxX/Y are not 0 based
    maxX++;
    maxY++;

    // Rewind for processing later
    rewind(f);

    // Create grid
    int grid[maxY][maxX];
    for (int i = 0; i < maxY * maxX; i++) {
        grid[i / maxX][i % maxX] = 0;
    }

    // Loop over paths again (re-use previous vars)
    while (fscanf(f, "%d,%d -> %d,%d", &x1, &y1, &x2, &y2) == 4) {
        
        // Skip any non horizontal/vertical lines in hor/ver only mode
        if (mode == 1 && x1 != x2 && y1 != y2) {
            continue;
        }

        // Follow lines
        for (int y = min(y1, y2); y <= max(y1, y2); y++) {
            for (int x = min(x1, x2); x <= max(x1, x2); x++) {
                grid[y][x]++;
            }
        }
        
    }

    // Calculate overlaps
    int overlaps = 0;
    for (int i = 0; i < maxY * maxX; i++) {
        if (grid[i / maxX][i % maxX] > 1) overlaps++;
    }

    return overlaps;
}

int main () {
    printf("Answer 1: %d\n", calculate_overlaps(1));

    return 0;
}