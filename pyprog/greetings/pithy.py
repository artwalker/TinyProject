# pithy.py

import random

def print_random_saying(filename):
    try:
        with open(filename, 'r') as file:
            lines = file.readlines()
            print(random.choice(lines).strip())
    except FileNotFoundError:
        print(f"Unable to open file {filename}")
        exit(1)
    except MemoryError:
        print("Unable to allocate memory")
        exit(1)