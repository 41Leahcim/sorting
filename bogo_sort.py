import random

def is_sorted(values: list) -> bool:
    for i in range(1, len(values)):
        if values[i - 1] > values[i]:
            return False
    return True

# 1.7642974853515625e-05 for 1
# 3.886222839355469e-05 for 2
# 5.054473876953125e-05 for 4
# 0.0959320068359375 for 8
def bogo_sort(values: list):
    while not is_sorted(values):
        random.shuffle(values)
