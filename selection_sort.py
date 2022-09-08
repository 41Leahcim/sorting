# 1.3589859008789062e-05 for 10
# 0.00020647048950195312 for 100
# 0.01963043212890625000 for 1000
# 1.90446424484252930000 for 10000
def selection_sort(values: list):
    for i in range(len(values)):
        lowest_index = i
        for j in range(i + 1, len(values)):
            if values[j] < values[lowest_index]:
                lowest_index = j
        values[i], values[lowest_index] = values[lowest_index], values[i]