#include <stdio.h>
#include <time.h>

// Include the header file for pithy06
#include "pithy.h"

/**
 * Function to calculate the phase of the moon for a given date.
 * @param year The year of the date.
 * @param month The month of the date.
 * @param day The day of the date.
 * @return The phase of the moon as an integer between 0 and 7.
 */
int moon_phase(int year, int month, int day);

/**
 * Main function of the program.
 * @param argc The number of command line arguments.
 * @param argv The command line arguments.
 * @return 0 if the program finishes successfully.
 */
int main(int argc, char *argv[])
{
    time_t now;           // Current time
    struct tm *clock;     // Local time
    char time_string[64]; // Formatted time string

    // Phases of the moon
    char *phase[8] = {
        "waxing crescent", "at first quarter",
        "waxing gibbous", "full", "waning gibbous",
        "at last quarter", "waning crescent", "new"};
    int mp; // Moon phase

    time(&now);              // Get current time
    clock = localtime(&now); // Convert to local time

    // Format time as a string
    strftime(time_string, 64, "Today is %A, %B %d, %Y%nIt is %r%n", clock);

    // Calculate moon phase
    mp = moon_phase(clock->tm_year + 1900, clock->tm_mon, clock->tm_mday);

    printf("Greetings");                   // Print greeting
    if (argc > 1)                          // If there is more than one command line argument
        printf(", %s", argv[1]);           // Print the second command line argument
    printf("!\n%s", time_string);          // Print the time string
    printf("The moon is %s\n", phase[mp]); // Print the moon phase

    // Call the function from pithy06
    print_random_saying("pithy.txt");

    return (0); // End the program
}

/**
 * Function to calculate the phase of the moon for a given date.
 * @param year The year of the date.
 * @param month The month of the date.
 * @param day The day of the date.
 * @return The phase of the moon as an integer between 0 and 7.
 */
int moon_phase(int year, int month, int day)
{
    int d, g, e; // Day, golden number, and epact

    d = day;                                     // Store the day
    if (month == 2)                              // If the month is February
        d += 31;                                 // Add 31 to the day
    else if (month > 2)                          // If the month is after February
        d += 59 + (month - 3) * 30.6 + 0.5;      // Add the number of days in the previous months to the day
    g = (year - 1900) % 19;                      // Calculate the golden number
    e = (11 * g + 29) % 30;                      // Calculate the epact
    if (e == 25 || e == 24)                      // If the epact is 25 or 24
        ++e;                                     // Increment the epact
    return ((((e + d) * 6 + 5) % 177) / 22 & 7); // Calculate and return the phase of the moon
}
