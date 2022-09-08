# 2.0503997802734375e-05 for 10
# 0.00011467933654785156 for 100
# 0.00135970115661621100 for 1000
# 0.01644515991210937500 for 10000
# 0.21151900291442870000 for 100000
# 2.61152029037475600000 for 1000000
def merge_sort(values):
    if len(values) == 2 and values[0] > values[1]:
        values[0], values[1] = values[1], values[0]
    elif len(values) > 2:
        b, c = values[:len(values) // 2], values[len(values) // 2:]
        merge_sort(b)
        merge_sort(c)
        i, j, k = (0, 0, 0)
        while i < len(b) and j < len(c):
            if b[i] < c[j]:
                values[k] = b[i]
                i += 1
            else:
                values[k] = c[j]
                j += 1
            k += 1
        while i < len(b):
            values[k] = b[i]
            i += 1
            k += 1
        while j < len(c):
            values[k] = c[j]
            j += 1
            k += 1