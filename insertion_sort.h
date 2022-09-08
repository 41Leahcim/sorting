#pragma once

#include "list.h"

//  0.000001 for 1
//  0.000002 for 10
//  0.000027 for 100
//  0.002328 for 1000
//  0.238688 for 10000
// 23.814988 for 100000
void insertion_sort(List *list){
    size_t i = 1;
    while(i < list->length){
        if(list->values[i - 1] > list->values[i]){
            uint64_t tmp = list->values[i];
            list->values[i] = list->values[i - 1];
            list->values[i - 1] = tmp;
            if(i > 1){
                i--;
            }else{
                i++;
            }
        }else{
            i++;
        }
    }
}