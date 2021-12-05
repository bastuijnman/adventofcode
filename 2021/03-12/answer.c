#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

// Definition of the number of bits per line present in the diagnostics report
const int NUMBER_OF_BITS = 12;

int main () {

    FILE *f = fopen("input.txt", "r");

    int lines = 1;
    while (EOF != (fscanf(f, "%*[^\n]"), fscanf(f,"%*c")))
        lines++;

    /*
     * Ineffecient AF :(
     */
    int index = 0;
    int buffer[NUMBER_OF_BITS * lines];

    rewind(f);
    while (!feof(f)) {
        int c = fgetc(f);

        // Don't include newlines and EOF chars
        if (c != 10 && c != EOF) {
            buffer[index] = c;
            index++;
        }
    }

    int gamma_counter[NUMBER_OF_BITS] = {0};
    for (int i = 0; i < lines; i++) {
        for (int j = 0; j < NUMBER_OF_BITS; j++) {
            gamma_counter[((i * NUMBER_OF_BITS) + j) % NUMBER_OF_BITS] += buffer[(i * NUMBER_OF_BITS) + j] == 49;
        }
    }

    // Calculate gamma
    int gamma = 0;
    for (int i = 0; i < NUMBER_OF_BITS; i++) {
        gamma += (gamma_counter[i] >= (lines / 2) ? 1 : 0) * pow(2, (NUMBER_OF_BITS - 1) - i);
    }

    // epsilon = XOR with mask consisting of number of bits per line (flips all bits)
    int epsilon = gamma ^ ((1 << NUMBER_OF_BITS) - 1);
    printf("Answer 1: %d\n", gamma * epsilon);

    // TODO: answer 2

    fclose(f);
    return 0;
}