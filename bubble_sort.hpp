#pragma once

#include <vector>

// 1.2430e-06 for 1
// 1.5850e-06 for 10
// 2.4492e-05 for 100
// 0.00225331 for 1000
// 0.23372600 for 10000
// 22.4350000 for 100000
template<class T>
void bubble_sort(std::vector<T>& values){
    for(size_t i = 0;i < values.size();i++){
        for(size_t j = 1;j < values.size();j++){
            if(values[j] < values[j - 1]){
                std::swap(values[j], values[j - 1]);
            }
        }
    }
}
