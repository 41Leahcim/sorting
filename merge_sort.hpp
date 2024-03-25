#pragma once

#include <vector>

//           1: 1.18e-07
//          10: 7.03e-07
//         100: 4.403e-06
//       1'000: 6.3253e-05
//      10'000: 0.000330678
//     100'000: 0.004516
//   1'000'000: 0.0425676
//  10'000'000: 0.512723
// 100'000'000: 6.22701
/*template<class T>
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
}*/

template<typename T>
void merge(std::vector<T>& values, std::vector<T>& buffer, const size_t start, const size_t length){
    const size_t middle = start + length / 2;
    const size_t end = start + length;
    size_t i = start;
    size_t j = middle;
    buffer.clear();

    while(i < middle && j < end){
        if(values[i] < values[j]){
            buffer.push_back(values[i++]);
        }else{
            buffer.push_back(values[j++]);
        }
    }

    while(i < middle){
        buffer.push_back(values[i++]);
    }
    while(j < end){
        buffer.push_back(values[j++]);
    }

    for(i = 0;i < length;i++){
        values[start + i] = buffer[i];
    }
}

//           1: 1.598e-06
//          10: 1.619e-06
//         100: 2.249e-06
//       1'000: 2.142e-05
//      10'000: 0.000251302
//     100'000: 0.0030927
//   1'000'000: 0.0367311
//  10'000'000: 0.466735
// 100'000'000: 5.33765
template<typename T>
void merge_sort(std::vector<T>& values){
    std::vector<std::pair<size_t, size_t>> ranges;
    ranges.reserve(257);
    std::vector<T> buffer(values.size());
    ranges.emplace_back(0, values.size());
    size_t previous_length = values.size();
    while(!ranges.empty()){
        std::pair<size_t, size_t> range = ranges[ranges.size() - 1];
        ranges.pop_back();
        size_t length = range.second - range.first;
        switch(length){
            case 0: case 1: break;
            case 2:
                if(values[range.first] > values[range.second] - 1){
                    T& tmp = values[range.first];
                    values[range.first] = values[range.second - 1];
                    values[range.second - 1] = tmp;
                }
                break;
            default:
                if(previous_length < length){
                    merge(values, buffer, range.first, length);
                    break;
                }
                size_t middle = range.first + length / 2;
                ranges.push_back(range);
                ranges.emplace_back(range.first, middle);
                ranges.emplace_back(middle, range.second);
        }
        previous_length = length;
    }
}
