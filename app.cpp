#include <iostream>
#include <vector>
#include <chrono>

#include "merge_sort.hpp"
#include "insertion_sort.hpp"
#include "bubble_sort.hpp"
#include "selection_sort.hpp"

using std::chrono::time_point;
using std::chrono::system_clock;
using std::chrono::duration;

int main(int argn, const char **argv){
    if(argn == 1){
        printf("Usage: %s algorithm\n", argv[0]);
        puts("Available algorithms:");
        puts("merge_sort");
        puts("insertion_sort");
        puts("bubble_sort");
        puts("selection_sort");
    }else{
        time_point<system_clock> start, end;
        
        std::vector<uint64_t> number_list(10);

        std::string algorithm = argv[1];

        for(size_t i = 0;i < number_list.size();i++){
            number_list[i] = number_list.size() - i;
        }

        start = system_clock::now();
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
        end = system_clock::now();

        for(uint64_t value : number_list){
            std::cout << value << "\n";
        }

        std::cout << static_cast<duration<double>>(end - start).count() << "\n";
    }
}
