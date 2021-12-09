#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

/**
 * @brief Holds bingo board data.
 * 
 */
typedef struct item {
    int numbers[5][5];
    struct item * next;
    int index;
} board;

bool contains(int* needles, size_t needles_size, int* haystack, size_t haystack_size) {
    for (int i = 0; i < needles_size; i++) {

        bool found = false;
        for (int j = 0; j < haystack_size; j++) {
            if (haystack[j] == needles[i]) {
                found = true;
            }
        }

        if (!found) {
            return false;
        }
    }
    return true;
}

/**
 * @brief Check whether a board is solved for bingo
 * 
 * @param b - The board to check
 * @param numbers - The drawn numbers so far
 * @param numbers_size - The amount of numbers drawn so far
 * @return bool 
 */
bool is_solved(board * b, int* numbers, size_t numbers_size) {
    for (int i = 0; i < 5; i++) {
        int row[5] = { b->numbers[i][0], b->numbers[i][1], b->numbers[i][2], b->numbers[i][3], b->numbers[i][4] };
        int col[5] = { b->numbers[0][i], b->numbers[1][i], b->numbers[2][i], b->numbers[3][i], b->numbers[4][i] };

        if (contains(row, 5, numbers, numbers_size) || contains(col, 5, numbers, numbers_size)) {
            return true;
        }
    }
    return false;
}

/**
 * @brief Get the inverse sum of a board. Disregards drawn numbers and sums all remaining.
 * 
 * @param b - The board to check
 * @param numbers - The drawn numbers so far
 * @param numbers_size - The amount of numbers drawn so far
 * @return int 
 */
int get_inverse_sum(board * b, int* numbers, size_t numbers_size) {
    int sum = 0;
    for (int i = 0; i < 5 * 5; i++) {
        int needle[1] = { b->numbers[i / 5][i % 5] };
        if (!contains(needle, 1, numbers, numbers_size)) {
            sum += needle[0];
        }
    }
    return sum;
}

int main () {

    FILE *f = fopen("input.txt", "r");

    // Grab all drawn bingo numbers
    char draws[1024];
    fgets(draws, 1024, f);

    // Grab board lines
    int index = 0;
    int a,b,c,d,e;

    // Create first board
    board * first = (board *) malloc(sizeof(board));

    // Loop over board input lines & store
    board * current = first;
    while (fscanf(f, "%d %d %d %d %d", &a, &b, &c, &d, &e) == 5) {

        // TODO: There has to be a better way to do this -.-
        current->numbers[index % 5][0] = a;
        current->numbers[index % 5][1] = b;
        current->numbers[index % 5][2] = c;
        current->numbers[index % 5][3] = d;
        current->numbers[index % 5][4] = e;
        
        if ((index + 1) % 5 == 0 && index != 0 && !feof(f)) {

            // Memory leak waiting to happen
            current->next = (board *) malloc(sizeof(board));
            current = current->next;
        }
        index++;
    }

    char *token = strtok(draws, ",");
    char draw_index = 0;
    int draw_array[1024];
    while (token != NULL) {
        draw_array[draw_index] = atoi(token);

        board * tmp = first;
        while (tmp != NULL) {
            if( is_solved(tmp, draw_array, draw_index + 1) ) {
                int sum = get_inverse_sum(tmp, draw_array, draw_index + 1);
                
                printf("Answer 1: %d\n", sum * draw_array[draw_index]);
                return 0;
            }
            tmp = tmp->next;
        }

        draw_index++;
        token = strtok(NULL, ",");
    }

    return 0;
}