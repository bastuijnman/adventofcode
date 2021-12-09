#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int min (int a, int b) { return a < b ? a : b; }
int max (int a, int b) { return a > b ? a : b;}
int clamp (int a, int min, int max) { if (a < min) return min; if (a > max) return max; return a; }

bool is_straight(int x1, int y1, int x2, int y2) { return x1 == x2 || y1 == y2; }
bool is_45deg(int x1, int y1, int x2, int y2) {return abs(x1 - x2) == abs(y1 - y2);}

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
        if (mode == 1 && !is_straight(x1, y1, x2, y2)) {
            continue;
        }
        if (mode == 2 && !is_straight(x1, y1, x2, y2) && !is_45deg(x1, y1, x2, y2)) {
            continue;
        }

        // Set direction
        int ix = clamp(x2 - x1, -1, 1), iy = clamp(y2 - y1, -1, 1);

        // First step out of while
        grid[y1][x1]++;
        while (x1 != x2 || y1 != y2) {
            x1 += ix;
            y1 += iy;
            grid[y1][x1]++;
        }    
    }

    // Calculate overlaps
    int overlaps = 0;
    for (int i = 0; i < maxY * maxX; i++) {
        if (grid[i / maxX][i % maxX] > 1) overlaps++;
    }

    fclose(f);

    return overlaps;
}

int main () {
    printf("Answer 1: %d\n", calculate_overlaps(1));
    printf("Answer 2: %d\n", calculate_overlaps(2));

    return 0;
}