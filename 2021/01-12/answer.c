#include <stdio.h>
#include <stdlib.h>

/*
 * Can't be arsed with dynamic allocation, so the number of depth 
 * measurements is defined here. Make sure this is correct to 
 * the amount of entries in the input, otherwise it'll segfault.
 */
int ENTRIES_COUNT = 2000;

int main () {
    FILE *f = fopen("input.txt", "r");
    int entries[ENTRIES_COUNT];

    char line[8] = {0};
    int i = 0;
    while (fgets(line, 8, f)) {
        entries[i] = strtol(line, NULL, 10);
        i++;
    }

    /*
     * Part 1
     */
    int answerPartOne = 0;
    for (int j = 1; j < ENTRIES_COUNT; j++) {
        if (entries[j] > entries[j - 1]) {
            answerPartOne++;
        }
    }
    printf("Answer part 1: %d\n", answerPartOne);

    /*
     * Part 2
     */
    int answerPartTwo = 0;
    for (int h = 1; h < (ENTRIES_COUNT - 2); h++) {
        if (
            (entries[h] + entries[h + 1] + entries[h + 2]) >
            (entries[h - 1] + entries[h] + entries[h + 1])
        ) {
            answerPartTwo++;
        }
    }  
    printf("Answer part 2: %d\n", answerPartTwo);

    fclose(f);

    return 0;

}
