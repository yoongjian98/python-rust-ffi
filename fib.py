import time
import random

from python_rust_ffi import fibonacci, get_coordinate, bubble_sort;
from file import reverse_file

def fibonacci_py(nth):

    if nth <= 2:
        return 1
    else:
        return fibonacci(nth - 1) + fibonacci(nth - 2)

print(fibonacci(2))

params = {'x': 1, 'y': 2, 'z': 3}
print(get_coordinate(params))

unsorted = [random.randint(1, 1000) for _ in range(150)]

num = 55

def py_bubble_sort(nums):
    nums_copy = nums.copy()
    n = len(nums_copy)
    for i in range(n):
        for j in range(n - i - 1):
            if nums_copy[j] > nums_copy[j + 1]:
                nums_copy[j], nums_copy[j + 1] = nums_copy[j + 1], nums_copy[j]
    return nums_copy

# start_time = time.time()
# # fibonacci_py(num)
# py_bubble_sort(unsorted)
# end_time = time.time()

# execution_time = end_time - start_time
# print("Python Execution time:", execution_time, "seconds")


# start_time = time.time()
# # fibonacci(num)
# bubble_sort(unsorted)
# end_time = time.time()

# execution_time = end_time - start_time
# print("Rust Execution time:", execution_time, "seconds")


start_time = time.time()
reverse_file("sample.txt")
end_time = time.time()
execution_time = end_time - start_time
print("Python Execution time:", execution_time, "seconds")
