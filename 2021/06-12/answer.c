#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

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
    fish * first = NULL;
    fish * current = NULL;

    /*
     * Initialize fish seeds
     */
    while (token != NULL) {
        int entry = atoi(token);
        fish * new = (fish *) malloc(sizeof(fish));
        new->counter = entry;

        if (first == NULL) {
            //printf("SIZE %ld\n", sizeof(*first));
            first = new;
            current = new;
        } else {
            current->next = new;
            current = new;
        }

        token = strtok(NULL, ",");
    }
    
    fish * last = current;
    for (int d = 0; d < days; d++) {

        fish * iter = first;
        while (iter != NULL) {
            iter->counter--;

            if (iter->counter < 0) {
                iter->counter = 6;

                // Spawn new fish
                fish * new = (fish *) malloc(sizeof(fish));
                new->counter = 8;

                // To front of the list so it doesn't get processed in the current round
                new->next = first;
                first = new;

            }
            iter = iter->next;
        }
    }

    // Count number of fish in the chain
    fish * iter = first;
    int counter = 0;
    while (iter != NULL) {
        counter++;
        iter = iter->next;
    }

    return counter;
}

int main () {
    printf("Answer 1: %d\n", simulate(80));
    printf("Answer 2: %d\n", simulate(256));
    return 0;
}