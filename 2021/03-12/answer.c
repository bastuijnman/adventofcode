#include <stdio.h>
#include <stdlib.h>
#include <math.h>

// Definition of the number of bits per line present in the diagnostics report
const int NUMBER_OF_BITS = 12;

int main () {

    FILE *f = fopen("input.txt", "r");

    int gamma_counter[NUMBER_OF_BITS] = {0};
    int char_counter = 0; // Chars start at 0 for 0-based index
    int line_counter = 1; // We can start lines at 1 assuming we're reading from lines at all

    /*
     * I too like to live dangerously 
     */
    while (1) {

        // Break the loop once the end of the file is reached
        if (feof(f)) {
            break;
        }

        int c = fgetc(f);

        // Increase line counter when newline is reached
        if (c == 10) { 
            line_counter++;

            // No need for processing for newlines
            continue; 
        }

        // Increase count in case the bit is set to 1
        if (c == 49) {
            gamma_counter[char_counter % NUMBER_OF_BITS] += 1;
        }

        // Increase actual number of chars processed
        char_counter++;
    }

    // Calculate gamma
    int gamma = 0;
    for (int i = 0; i < NUMBER_OF_BITS; i++) {
        gamma += (gamma_counter[i] >= (line_counter / 2) ? 1 : 0) * pow(2, (NUMBER_OF_BITS - 1) - i);
    }

    // epsilon = XOR with mask consisting of number of bits per line (flips all bits)
    int epsilon = gamma ^ ((1 << NUMBER_OF_BITS) - 1);

    printf("Answer 1: %d\n", gamma * epsilon);

    return 0;
}