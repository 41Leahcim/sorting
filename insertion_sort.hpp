#pragma once

#include <vector>

// 1.3740e-06 for 1
// 1.9880e-06 for 10
// 3.5544e-05 for 100
// 0.00306872 for 1000
// 0.29882400 for 10000
// 26.4415000 for 100000
template<class T>
void insertion_sort(std::vector<T>& values){
    size_t i = 1;
    while(i < values.size()){
        if(values[i - 1] > values[i]){
            std::swap(values[i - 1], values[i]);
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