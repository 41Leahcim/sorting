# 5.173683166503906e-05 for 10
# 0.0010437965393066406 for 100
# 0.1227414608001709000 for 1000
# 13.392222881317139000 for 10000
def insertion_sort(values):
    i = 1
    while i < len(values):
        if values[i] < values[i - 1]:
            values[i], values[i - 1] = values[i - 1], values[i]
            if i > 1:
                i -= 1
            else:
                i += 1
        else:
            i += 1