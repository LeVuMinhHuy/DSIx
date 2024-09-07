#!/bin/python3

import math
import os
import random
import re
import sys

#
# Complete the 'arrayManipulation' function below.
#
# The function is expected to return a LONG_INTEGER.
# The function accepts following parameters:
#  1. INTEGER n
#  2. 2D_INTEGER_ARRAY queries
#

def arrayManipulation(n, queries):
    arr = [0 for _ in range(n)]

    for i in range(len(queries)):
        a = queries[i][0]
        b = queries[i][1]
        k = queries[i][2]

        arr[a - 1] += k

        if (b < n):
            arr[b] -= k

    if n == 0:
        return 0

    max_value = arr[0] 
    for i in range(1, n):
        arr[i] += arr[i - 1] 
        
        if (arr[i] > max_value):
            max_value = arr[i]

    return max_value



if __name__ == '__main__':
    fptr = open(os.environ['OUTPUT_PATH'], 'w')

    first_multiple_input = input().rstrip().split()

    n = int(first_multiple_input[0])

    m = int(first_multiple_input[1])

    queries = []

    for _ in range(m):
        queries.append(list(map(int, input().rstrip().split())))

    result = arrayManipulation(n, queries)

    fptr.write(str(result) + '\n')

    fptr.close()

