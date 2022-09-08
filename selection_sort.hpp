#pragma once

#include <vector>

// 1.2310e-06 for 1
// 1.4930e-06 for 10
// 4.3940e-06 for 100
// 0.00025227 for 1000
// 0.01978740 for 10000
// 2.15752000 for 100000
template<class T>
void selection_sort(std::vector<T>& values){
    for(size_t i = 0;i < values.size();i++){
        size_t lowest_index = i;
        for(size_t j = i;j < values.size();j++){
            if(values[j] < values[lowest_index]){
                lowest_index = j;
            }
        }
        std::swap(values[lowest_index], values[i]);
    }
}
