import sys
import time
from datetime import datetime
import math
from pithy import print_random_saying

def moon_phase(year, month, day):
    d = day
    if month == 2:
        d += 31
    elif month > 2:
        d += 59 + (month - 3) * 30.6 + 0.5
    g = (year - 1900) % 19
    e = (11 * g + 29) % 30
    if e == 25 or e == 24:
        e += 1
    return int((((e + d) * 6 + 5) % 177) / 22) & 7

def greet():
    phase = ["waxing crescent", "at first quarter", "waxing gibbous", "full", 
             "waning gibbous", "at last quarter", "waning crescent", "new"]
    now = datetime.now()
    time_string = now.strftime("Today is %A, %B %d, %Y\nIt is %I:%M:%S %p")
    mp = moon_phase(now.year, now.month, now.day)

    print("Greetings", end="")
    if len(sys.argv) > 1:
        print(", " + sys.argv[1], end="")
    print("!")
    print(time_string)
    print("The moon is", phase[mp])

    # Call the function from pithy.py
    print_random_saying("pithy.txt")

if __name__ == "__main__":
    greet()