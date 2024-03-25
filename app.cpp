#include <iostream>
#include <vector>
#include <chrono>
#include <functional>
#include <array>

#include "merge_sort.hpp"
#include "insertion_sort.hpp"
#include "bubble_sort.hpp"
#include "selection_sort.hpp"

namespace chrono = std::chrono;

chrono::duration<int64_t, std::nano> test_algorithm(std::function<void(std::vector<uint64_t>&)>& sort, std::vector<uint64_t>& values){
    chrono::time_point<chrono::steady_clock> start = chrono::steady_clock::now();
    sort(values);
    return chrono::steady_clock::now() - start;
}

std::vector<uint64_t> create_unsorted(size_t size){
    std::vector<uint64_t> number_list(size);

    for(size_t i = 0;i < number_list.size();i++){
        number_list[i] = number_list.size() - i;
    }

    return number_list;
}

int main(int argn, const char **argv){
    if(argn == 1){
        std::cout << "Usage: " << argv[0] << '\n' <<
            "Available algorithms:\n\n" <<
            "merge_sort\n" <<
            "insertion_sort\n" <<
            "bubble_sort\n" <<
            "selection_sort\n";
        std::vector<std::pair<std::function<void(std::vector<uint64_t>&)>, std::string>> algorithms = {
            std::pair(merge_sort<uint64_t>, "merge sort"),
            std::pair(insertion_sort<uint64_t>, "insertion sort"),
            std::pair(bubble_sort<uint64_t>, "bubble sort"),
            std::pair(selection_sort<uint64_t>, "selection sort")};
        for(std::pair<std::function<void(std::vector<uint64_t>&)>, std::string>& sort: algorithms){
            std::cout << sort.second << ":\n";
            chrono::duration<double> performance = chrono::seconds(0);
            size_t size = 1;
            while(performance < chrono::seconds(1)){
                std::vector<uint64_t> values = create_unsorted(size);
                performance = test_algorithm(sort.first, values);
                std::cout << size << ": " << performance.count() << '\n';
                size *= 10;
            }
            std::cout << std::endl;
        }
    }else{
        
        std::vector<uint64_t> number_list = create_unsorted(10);

        std::string algorithm = argv[1];

        chrono::time_point<chrono::steady_clock> start = chrono::steady_clock::now();
        if(algorithm == "merge_sort"){
            merge_sort(number_list);
        }else if(algorithm == "insertion_sort"){
            insertion_sort(number_list);
        }else if(algorithm == "bubble_sort"){
            bubble_sort(number_list);
        }else if(algorithm == "selection_sort"){
            selection_sort(number_list);
        }else{
            printf("Invalid algorithm: %s\n", argv[1]);
            return EXIT_FAILURE;
        }
        chrono::time_point<chrono::steady_clock> end = chrono::steady_clock::now();

        for(uint64_t value : number_list){
            std::cout << value << "\n";
        }

        std::cout << static_cast<chrono::duration<double>>(end - start).count() << "\n";
    }
}
