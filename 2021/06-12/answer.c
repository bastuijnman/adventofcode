#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>

typedef struct node {
    int counter;
    bool skip;
    struct node * next;
    struct node * previous;
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
        new->skip = false;
        new->counter = entry;

        if (first == NULL) {
            first = new;
            current = new;
        } else {
            new->previous = current;
            current->next = new;
            current = new;
        }

        token = strtok(NULL, ",");
    }
    
    fish * last = current;
    for (int d = 0; d < days; d++) {

        clock_t before = clock();

        fish * iter = first;
        while (iter != NULL) {

            if (iter->skip) {
                iter->skip = false;
            } else {

                iter->counter--;

                if (iter->counter < 0) {
                    iter->counter = 6;

                    fish * new = (fish *) malloc(sizeof(fish));
                    new->counter = 8;
                    new->skip = true;

                    last->next = new;
                    last = new;

                }
            }
            iter = iter->next;
        }

        printf("Day %d took %ld miliseconds\n", d+1, (clock() - before) * 1000 / CLOCKS_PER_SEC);
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