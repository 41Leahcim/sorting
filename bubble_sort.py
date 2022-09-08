# 1.8358230590820312e-05 for 10
# 0.00047230720520019530 for 100
# 0.05660581588745117000 for 1000
# 6.01831507682800300000 for 10000
def bubble_sort(values):
    for i in range(0, len(values)):
        for j in range(1, len(values) - i):
            if values[j - 1] > values[j]:
                values[j - 1], values[j] = values[j], values[j - 1]