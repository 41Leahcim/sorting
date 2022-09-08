#pragma once

#include <vector>

// 1.20200e-06 for 1
// 2.61500e-06 for 10
// 5.68900e-06 for 100
// 2.93730e-05 for 1000
// 0.000324169 for 10000
// 0.003749460 for 100000
// 0.040714700 for 1000000
// 0.480725000 for 10000000
// 5.363470000 for 100000000
template<class T>
void merge_sort(std::vector<T>& a){
    if(a.size() <= 2){
        if(a.size() == 2 && a[0] > a[1]){
            std::swap(a[0], a[1]);
        }
    }else{
        std::vector<T> b;
        std::vector<T> c;
        b.assign(a.begin(), a.begin() + a.size() / 2);
        c.assign(a.begin() + a.size() / 2, a.end());
        a.clear();
        merge_sort(b);
        merge_sort(c);
        a.resize(b.size() + c.size());
        size_t i = 0, j = 0, k = 0;
        while(i < b.size() && j < c.size()){
            if(b[i] < c[j]){
                a[k++] = b[i++];
            }else{
                a[k++] = c[j++];
            }
        }
        while(i < b.size()){
            a[k++] = b[i++];
        }
        while(j < c.size()){
            a[k++] = c[j++];
        }
    }
}
