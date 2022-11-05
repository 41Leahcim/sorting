#pragma once

#include <stdlib.h>
#include <string.h>

#include "list.h"

// 0.000002 for 1
// 0.000002 for 10
// 0.000005 for 100
// 0.000024 for 1000
// 0.000245 for 10000
// 0.004152 for 100000
// 0.034752 for 1000000
// 0.415565 for 10000000
// 4.792327 for 100000000
void merge_sort(List* a){
    if(a->length <= 2){
        if(a->length == 2 && a->values[0] > a->values[1]){
            uint64_t tmp = a->values[0];
            a->values[0] = a->values[1];
            a->values[1] = tmp;
        }
    }else{
        List b = {
            .values = (uint64_t*)malloc(a->length / 2 * sizeof(uint64_t)),
            .length = a->length / 2
        };
        if(b.values == NULL){
            perror("Failed to allocate memory for list");
            exit(EXIT_FAILURE);
        }
        List c = {
            .values = (uint64_t*)malloc((a->length - b.length) * sizeof(uint64_t)),
            .length = a->length - b.length
        };
        if(c.values == NULL){
            perror("Failed to allocate memory for list");
            exit(EXIT_FAILURE);
        }
        memcpy(b.values, a->values, b.length * sizeof(uint64_t));
        memcpy(c.values, a->values + b.length, c.length * sizeof(uint64_t));
        merge_sort(&b);
        merge_sort(&c);
        size_t i = 0, j = 0, k = 0;
        while(i < b.length && j < c.length){
            if(b.values[i] < c.values[j]){
                a->values[k++] = b.values[i++];
            }else{
                a->values[k++] = c.values[j++];
            }
        }
        while(i < b.length){
            a->values[k++] = b.values[i++];
        }
        while(j < c.length){
            a->values[k++] = c.values[j++];
        }
        free(b.values);
        free(c.values);
    }
}