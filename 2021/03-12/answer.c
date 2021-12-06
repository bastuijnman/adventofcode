#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#include <string.h>

// Definition of the number of bits per line present in the diagnostics report
const int NUMBER_OF_BITS = 12;

/**
 * @brief Compare two integer arrays
 * 
 * @param a - First array
 * @param b - Second array
 * @param size - Size of arrays to compare
 * @return bool 
 */
bool compare(int a[], int b[], int size) {
    for (int i = 0; i < size; i++) {
        if (a[i] != b[i]) return false;
    }

    return true;
}

/**
 * @brief Turn an array of bits into its decimal value
 * 
 * @param bits 
 * @return int 
 */
int bits_to_decimal(int bits[]) {
    int result = 0;
    for (int i = 0; i < NUMBER_OF_BITS; i++) {
        result += bits[i] * pow(2, (NUMBER_OF_BITS - 1) - i);
    }
    return result;
}

/**
 * @brief Find a particular value in the input buffer
 * 
 * @param buffer - Buffer containing all bits from the input
 * @param lines - Number of lines contained by the input
 * @param mode - Look for Oxygen (0), or Co2 (1)
 * @param values - Values array to pass into
 */
void find_values(int *buffer, int lines, int mode, int* values) {
    for (int i = 0; i < NUMBER_OF_BITS; i++) {
        int counter = 0;
        int line_counter = 0;
        int last_known_line = 0;
        for (int j = 0; j < lines; j++) {

            if (i > 0) {
                int current_prefix[NUMBER_OF_BITS];
                for (int k = 0; k < i; k++) {
                    current_prefix[k] = buffer[(j * NUMBER_OF_BITS) + k] == 49;
                }

                if (!compare(values, current_prefix, i)) {
                    continue;
                }
            }

            last_known_line = j;
            line_counter++;
            counter += buffer[(j * NUMBER_OF_BITS) + i] == 49;
        }

        // In case we've got one line left we can exit out of the loop with the known report value
        if (line_counter == 1) {
            for (int j = 0; j < NUMBER_OF_BITS; j++) {
                values[j] = buffer[(last_known_line * NUMBER_OF_BITS) + j] == 49;
            }
            break;
        }

        if (mode == 0) {
            values[i] = counter >= (line_counter - counter) ? 1 : 0;
        } else {
            values[i] = counter < (line_counter - counter) ? 1 : 0;
        }
    }
}

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

    // Only count the higher number of bits over the whole set
    for (int i = 0; i < NUMBER_OF_BITS; i++) {
        gamma_counter[i] = gamma_counter[i] >= (lines / 2) ? 1 : 0;
    }

    // Calculate gamma
    int gamma = bits_to_decimal(gamma_counter);

    // epsilon = XOR with mask consisting of number of bits per line (flips all bits)
    int epsilon = gamma ^ ((1 << NUMBER_OF_BITS) - 1);
    printf("Answer 1: %d\n", gamma * epsilon);

    // Find the oxygen value in the list
    int oxygen_values[NUMBER_OF_BITS];
    find_values(buffer, lines, 0, oxygen_values);
    int oxygen = bits_to_decimal(oxygen_values);

    // Find the co2 value in the list
    int co2_values[NUMBER_OF_BITS];
    find_values(buffer, lines, 1, co2_values);
    int co2 = bits_to_decimal(co2_values);

    printf("Answer 2: %d\n", oxygen * co2);
    

    fclose(f);
    return 0;
}