#pragma once

#include <stdint.h>
#include <stdio.h>

typedef struct List{
    uint64_t *values;
    size_t length;
} List;
