#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <time.h>

#include "merge_sort.h"
#include "insertion_sort.h"
#include "bubble_sort.h"
#include "selection_sort.h"

int main(int argn, const char **argv){
    if(argn == 1){
        printf("Usage: %s algorithm\n", argv[0]);
        puts("Available algorithms:");
        puts("merge_sort");
        puts("insertion_sort");
        puts("bubble_sort");
        puts("selection_sort");
        return 0;
    }else{
        clock_t start, took;
        const uint64_t ARRAY_LENGTH = 100000;
        List number_list = {
            .values=(uint64_t*)malloc(ARRAY_LENGTH * sizeof(uint64_t)),
            .length=ARRAY_LENGTH
        };

        for(size_t i = 0;i < number_list.length;i++){
            number_list.values[i] = number_list.length - i;
        }

        start = clock();
        if(strcmp(argv[1], "merge_sort") == 0){
            merge_sort(&number_list);
        }else if(strcmp(argv[1], "insertion_sort") == 0){
            insertion_sort(&number_list);
        }else if(strcmp(argv[1], "bubble_sort") == 0){
            bubble_sort(&number_list);
        }else if(strcmp(argv[1], "selection_sort") == 0){
            selection_sort(&number_list);
        }else{
            printf("Invalid algorithm: %s\n", argv[1]);
            return EXIT_FAILURE;
        }
        took = clock() - start;

        for(size_t i = 0;i < number_list.length;i++){
            printf("%lu\n", number_list.values[i]);
        }

        free(number_list.values);
        printf("%lf\n", (double)took / CLOCKS_PER_SEC);
    }
    return EXIT_FAILURE;
}
