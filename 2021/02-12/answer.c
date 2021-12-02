#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/**
 * @brief Get the result for the given input file.
 * 
 * @param int method - 1 for the method that directly calculates depth, 2 for the aim method
 * @return int 
 */
int get_answer (int method) {
    FILE *f = fopen("input.txt", "r");

    int distance = 0, depth = 0, depth_or_aim = 0;

    // To be passed values from input
    char direction[8] = {0};
    int count = 0;

    // Scan for "DIRECTION COUNT" in input file, loop while we get 2 values
    while (fscanf(f, "%s %d", direction, &count) == 2)
    {
        if (strcmp(direction, "forward") == 0) { 

            // Always increase distance
            distance += count;

            // In the case we're calculating depth with forward motion take the aim value into consideration
            depth += (depth_or_aim * count);
        }

        // Increase/Decrease aim or depth when neccessary 
        if (strcmp(direction, "up") == 0) depth_or_aim -= count;
        if (strcmp(direction, "down") == 0) depth_or_aim += count;

        // Increase file pointer
        fgetc(f);
    }
    fclose(f);

    return method == 1 ? distance * depth_or_aim : distance * depth;
}

int main () {
    printf("Answer 1: %d\n", get_answer(1));
    printf("Answer 2: %d\n", get_answer(2));
}