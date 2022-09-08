#include "list.h"

//  0.000001 for 1
//  0.000001 for 10
//  0.000021 for 100
//  0.002038 for 1000
//  0.203936 for 10000
// 21.233743 for 100000
void bubble_sort(List *list){
    for(size_t i = 0;i < list->length;i++){
        for(size_t j = 1;j < list->length - i;j++){
            if(list->values[j - 1] > list->values[j]){
                uint64_t tmp = list->values[j];
                list->values[j] = list->values[j - 1];
                list->values[j - 1] = tmp;
            }
        }
    }
}
