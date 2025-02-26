#!/usr/bin/env python3

import re
import csv
from collections import defaultdict

# Initialize two dictionaries
# The 'error' dictionary will be used to store the count of different error messages.
# The key is the error message, and the value is the number of occurrences of that error message.
error = defaultdict(int)
# The 'per_user' dictionary will be used to store the number of INFO and ERROR messages for each user.
# The key is the username, and the value is another dictionary with keys 'INFO' and 'ERROR'
# representing the counts of corresponding message types for that user.
per_user = defaultdict(lambda: {'INFO': 0, 'ERROR': 0})

# Open and parse the syslog.log file
with open('syslog.log', 'r') as file:
    for line in file:
        # Use regular expressions to match INFO messages.
        # The pattern is designed to capture the INFO message content and the username.
        # The INFO message content may contain alphanumeric characters, spaces, '#', '[', and ']'.
        info_pattern = r"ticky: INFO ([\w\s\[#\]]+) \(([\w.]+)\)"
        info_match = re.search(info_pattern, line)
        # Use regular expressions to match ERROR messages.
        # The pattern is designed to capture the ERROR message content and the username.
        # The ERROR message content may contain alphanumeric characters, spaces, and single quotes.
        error_pattern = r"ticky: ERROR ([\w\s']+) \(([\w.]+)\)"
        error_match = re.search(error_pattern, line)

        if info_match:
            # Extract the INFO message content and the username from the matched line.
            message, user = info_match.groups()
            # Increment the count of INFO messages for the corresponding user.
            per_user[user]['INFO'] += 1
        elif error_match:
            # Extract the ERROR message content and the username from the matched line.
            message, user = error_match.groups()
            # Increment the count of this specific error message.
            error[message] += 1
            # Increment the count of ERROR messages for the corresponding user.
            per_user[user]['ERROR'] += 1

# Sort the 'error' dictionary by the number of errors in descending order.
# The sorting key is the value (number of occurrences) of each item in the dictionary.
sorted_error = sorted(error.items(), key=lambda item: item[1], reverse=True)
# Insert column names at the beginning of the sorted error list.
# These column names will be used as headers in the CSV file for error statistics.
sorted_error.insert(0, ("Error", "Count"))

# Sort the 'per_user' dictionary by username in ascending order.
# The sorting key is the username (the key of each item in the dictionary).
sorted_per_user = sorted(per_user.items(), key=lambda item: item[0])
# Convert the 'per_user' dictionary to a format suitable for CSV and insert column names.
# The resulting list contains tuples with username, INFO count, and ERROR count,
# along with the column names at the beginning.
sorted_per_user = [("Username", "INFO", "ERROR")] + [(user, stats['INFO'], stats['ERROR']) for user, stats in sorted_per_user]

# Save the sorted dictionaries to CSV files
# Save the error statistics to the 'error_message.csv' file.
# The 'newline=""' parameter is used to handle newlines correctly in the CSV file.
with open('error_message.csv', 'w', newline='') as error_file:
    writer = csv.writer(error_file)
    writer.writerows(sorted_error)

# Save the user statistics to the 'user_statistics.csv' file.
# The 'newline=""' parameter is used to handle newlines correctly in the CSV file.
with open('user_statistics.csv', 'w', newline='') as user_file:
    writer = csv.writer(user_file)
    writer.writerows(sorted_per_user)

print("Reports generated successfully!")