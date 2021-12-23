#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef struct node {
    char counter;
    struct node * next;
} fish;

int simulate (int days) {

    FILE *f = fopen("input.txt", "r");

    // Get line with the seeder for the fish. We don't know the size, but 1024 should be fine.
    char seed[1024];
    fgets(seed, 1024, f);

    char *token = strtok(seed, ",");
    char *fish = NULL;
    fish = malloc(1000 * sizeof(*fish));

    /*
     * Initialize fish seeds
     */
    int index = 0;
    while (token != NULL) {
        char entry = atoi(token);
        fish[index] = entry;
        token = strtok(NULL, ",");
        index++;
    }

    unsigned long number_of_fish = index;
    for (int d = 0; d < days; d++) {

        int fish_to_spawn = 0;
        for (unsigned long i = 0; i < number_of_fish; i++) {
            fish[i]--;

            if (fish[i] < 0) {
                fish[i] = 6;
                fish_to_spawn++;
            }
        }

        // Make sure we can accomodate all fish
        fish = realloc(fish, (number_of_fish + fish_to_spawn) * sizeof(char));

        for (unsigned int i = 0; i < fish_to_spawn; i++) {
            fish[number_of_fish + i] = 8;
        }
        number_of_fish += fish_to_spawn;

        printf("Day %d, Fish: %ld\n", d, number_of_fish);

    }

    return number_of_fish;
}

int main () {
    printf("Answer 1: %d\n", simulate(80));
    printf("Answer 2: %d\n", simulate(256));

    return 0;
}