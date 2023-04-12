from time import perf_counter
from merge_sort import merge_sort
from insertion_sort import insertion_sort
from bubble_sort import bubble_sort
from selection_sort import selection_sort
from bogo_sort import bogo_sort

def main():
    algorithm = bogo_sort
    start = perf_counter()
    values = [i for i in range(16, 0, -1)]
    algorithm(values)
    print(values)
    print(perf_counter() - start)

if __name__ == "__main__":
    main()
