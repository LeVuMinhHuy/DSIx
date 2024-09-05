#!/bin/python3

import math
import os
import random
import re
import sys

#
# Complete the 'rotateLeft' function below.
#
# The function is expected to return an INTEGER_ARRAY.
# The function accepts following parameters:
#  1. INTEGER d
#  2. INTEGER_ARRAY arr
#

def rotateLeft(d, arr):
    n = len(arr)
    d = d % n

    reverseArray(arr, 0, n - 1)
    reverseArray(arr, 0, n - 1 - d) 
    reverseArray(arr, n - d, n - 1)
    return arr

def reverseArray(arr, a, b):
    size = b - a

    for i in range(size // 2 + 1):
        temp = arr[a + i]
        arr[a + i] = arr[b - i]
        arr[b - i] = temp

    return arr


if __name__ == '__main__':
    fptr = open(os.environ['OUTPUT_PATH'], 'w')

    first_multiple_input = input().rstrip().split()

    n = int(first_multiple_input[0])

    d = int(first_multiple_input[1])

    arr = list(map(int, input().rstrip().split()))

    result = rotateLeft(d, arr)

    fptr.write(' '.join(map(str, result)))
    fptr.write('\n')

    fptr.close()

