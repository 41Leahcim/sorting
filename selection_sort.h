#pragma once

#include "list.h"

// 0.000001 for 1
// 0.000002 for 10
// 0.000005 for 100
// 0.000237 for 1000
// 0.023458 for 10000
// 2.413406 for 100000
void selection_sort(List *list){
    for(size_t i = 0;i < list->length;i++){
        size_t lowest_index = i;
        for(size_t j = i + 1;j < list->length;j++){
            if(list->values[j] < list->values[lowest_index]){
                lowest_index = j;
            }
        }
        uint64_t tmp = list->values[i];
        list->values[i] = list->values[lowest_index];
        list->values[lowest_index] = tmp;
    }
}
