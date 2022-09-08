from time import time
from merge_sort import merge_sort
from insertion_sort import insertion_sort
from bubble_sort import bubble_sort
from selection_sort import selection_sort

def main():
    algorithm = selection_sort
    start = time()
    values = [i for i in range(10000, 0, -1)]
    algorithm(values)
    print(values)
    print(time() - start)

if __name__ == "__main__":
    main()
