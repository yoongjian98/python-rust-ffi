import time
from python_rust_ffi import reverse_file

def write_to_file(filename, size_in_mb):
    chars_per_mb = 1024 * 1024  # 1 MB is 1024 * 1024 bytes
    data = 'a' * chars_per_mb  # 1 character is 1 byte

    with open(filename, 'w') as f:
        for _ in range(size_in_mb):
            f.write(data)

def py_reverse_file(input_file):
    with open(input_file, 'r') as f:
        content = f.read()
    reversed_content = content[::-1]
    with open(input_file + "_reversed", 'w') as f:
        f.write(reversed_content)

start_time = time.time()
py_reverse_file("sample.txt")
end_time = time.time()
execution_time = end_time - start_time
print("Python Execution time:", execution_time, "seconds")

start_time = time.time()
reverse_file("sample.txt")
end_time = time.time()
execution_time = end_time - start_time
print("Rust Execution time:", execution_time, "seconds")

