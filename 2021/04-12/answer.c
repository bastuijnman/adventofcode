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
    bool solved;
} board;

/**
 * @brief Check if the haystack array contains the needles array.
 * 
 * @param needles - Values to check for
 * @param needles_size - Size of the values to check for
 * @param haystack - Values to check in
 * @param haystack_size - Size of the values to check in
 * @return bool 
 */
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

    // Grab all drawn bingo numbers, read first line of input and advance pointer
    char draws[1024];
    fgets(draws, 1024, f);

    // Grab board lines
    int index = 0;
    int a,b,c,d,e;

    // Create first board
    board * first = (board *) malloc(sizeof(board));
    int number_of_boards = 1;

    // Loop over board input lines & store
    board * current = first;

    while (fscanf(f, "%d %d %d %d %d", &a, &b, &c, &d, &e) == 5) {

        /*
         * As a means of not having to write out individual assignments
         * for each of the parsed numbers we store it in a new array and
         * copy over the memory contents into the board struct.
         */
        int current_numbers[5] = { a, b, c, d, e};
        memcpy(current->numbers[index % 5], current_numbers, sizeof(current_numbers));
        
        if ((index + 1) % 5 == 0) {

            // Make sure all boards are set to not be solved
            current->solved = false;
            
            // Do not create new boards for first index or end of file
            if (index != 0 && !feof(f)) {
                // Memory leak waiting to happen -.-
                current->next = (board *) malloc(sizeof(board));
                current = current->next;
                number_of_boards++;
            }
        }
        index++;
    }

    // Parse the drawn numbers as tokens
    char *token = strtok(draws, ",");

    // Maintain draw index & the actual numbers, we don't know the length but 1024 should be more than enough (inefficient)
    char draw_index = 0;
    int draw_array[1024];

    // Keep track of the number of solved boards
    int solved_count = 0;

    // Continue looping over drawn numbers while we receive them
    while (token != NULL) {

        // Cast to int and store in array
        draw_array[draw_index] = atoi(token);

        // Loop through all boards
        board * iter = first;
        while (iter != NULL) {

            // If a board is solved we can exit out, draw index needs to be increased to be non 0 based
            if( !iter->solved && is_solved(iter, draw_array, draw_index + 1)) {
                int sum = get_inverse_sum(iter, draw_array, draw_index + 1);

                // Increase solve count and mark board so it doesn't get processed anymore
                solved_count++;
                iter->solved = true;

                if (solved_count == 1)
                    printf("Answer 1: %d\n", sum * draw_array[draw_index]);

                if (solved_count == number_of_boards) {
                    printf("Answer 2: %d\n", sum * draw_array[draw_index]);
                    
                    // Once we've reached the second answer we can exit out
                    return 0;
                }

            }
            iter = iter->next;
        }

        // Increase draw index and grab new token if available
        draw_index++;
        token = strtok(NULL, ",");
    }

    return 0;
}