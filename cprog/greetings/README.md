# Using and Understanding the Greetings C Program

![](https://raw.githubusercontent.com/artwalker/PicGo/main/20240505235739.png)

This project is based on the source code from the book "Tiny C Projects". The code for this project can be found on my GitHub at [Greetings](https://github.com/artwalker/TinyProject/tree/main/cprog/greetings).

> Site: https://github.com/artwalker/TinyProject/tree/main/cprog/greetings

There are also implementations of this project in other programming languages.

## Project Analysis

This project is a C language program, the main function of which is to randomly select and print a quote from a text file. The `print_random_saying` function in the project is responsible for this function. This function is declared in the `pithy.h` header file and defined in the `pithy.c` source file.

The project uses a Makefile to automate the compilation and linking process. The Makefile defines several rules for compiling source files (`greet.c` and `pithy.c`), linking the generated object files (`greet.o` and `pithy.o`), and creating an executable file (`greetings`). The Makefile also defines a rule for creating symbolic links to the `pithy.txt` file and the `greetings` executable file.

## Usage Guide

1. **Compile the project**: In the terminal, navigate to the directory containing the Makefile, then enter `make` or `make all`. This will compile the source files, link the generated object files to create an executable file.

2. **Run the program**: In the terminal, navigate to the `~/bin` directory, then enter `./greetings`. This will run the program, and the program will randomly select and print a quote from the `pithy.txt` file.

3. **Clean up the project**: If you want to delete the generated executable file and object files, as well as the links created in the `~/bin` directory, you can enter `make clean` in the terminal.

4. **Configure .zshrc**: If you want to automatically run the `greetings` program when starting a new shell session, you can add the following line to the `.zshrc` file:

```bash
cd ~/bin && ./greeting Ethan && cd
```

This will first switch to the `~/bin` directory, then run the `greetings` program.

>Note: This project assumes that you have installed a C compiler (such as clang) on your system. If you do not have a C compiler installed on your system, you need to install a C compiler to compile and run this project.

## Code Anylysis
### tree
```bash
./cprog
└── greetings
    ├── Makefile
    ├── README.md
    ├── greet.c
    ├── pithy.c
    ├── pithy.h
    └── pithy.txt
```
### greet.c
```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define BSIZE 256

/**
 * Function to print a random saying from a file.
 * @param filename The name of the file containing the sayings.
 */
void print_random_saying(const char *filename)
{
	FILE *fp;			// File pointer
	char buffer[BSIZE]; // Buffer to store lines from the file
	char *r, *entry;	// Pointers to store the result of fgets and the allocated memory for each line
	int items, saying;	// Number of items read from the file and the index of the selected saying
	char **list_base;	// Base of the list of sayings

	// Open the file
	fp = fopen(filename, "r");
	if (fp == NULL)
	{
		fprintf(stderr, "Unable to open file %s\n", filename);
		exit(1);
	}

	// Allocate initial memory for the list of sayings
	list_base = malloc(sizeof(char *) * 100);
	if (list_base == NULL)
	{
		fprintf(stderr, "Unable to allocate memory\n");
		exit(1);
	}

	items = 0;
	// Read lines from the file until EOF
	while (!feof(fp))
	{
		r = fgets(buffer, BSIZE, fp);
		if (r == NULL)
			break;
		// Allocate memory for the current line and copy it from the buffer
		entry = malloc(sizeof(char) * strlen(buffer) + 1);
		if (entry == NULL)
		{
			fprintf(stderr, "Unable to allocate memory\n");
			exit(1);
		}
		strcpy(entry, buffer);
		// Store the pointer to the current line in the list
		*(list_base + items) = entry;
		items++;
		// If the list is full, reallocate more memory
		if (items % 100 == 0)
		{
			list_base = realloc(list_base, sizeof(char *) * (items + 100));
			if (list_base == NULL)
			{
				fprintf(stderr, "Unable to reallocate memory\n");
				exit(1);
			}
		}
	}

	// Close the file
	fclose(fp);

	// Seed the random number generator and select a random saying
	srand((unsigned)time(NULL));
	saying = rand() % (items - 1);
	// Print the selected saying
	printf("%s", *(list_base + saying));

	// Free the allocated memory
	for (int x = 0; x < items; x++)
		free(*(list_base + x));
	free(list_base);
}

// int main()
// {
// 	print_random_saying("pithy.txt");
// 	return (0);
// }
```
### pithy.h
```c
#ifndef PITHY_H
#define PITHY_H

/**
 * Function to print a random saying from a file.
 * @param filename The name of the file containing the sayings.
 */
void print_random_saying(const char *filename);

#endif // PITHY_H
```
### pithy.c
```c
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
```
### Makefile
```Makefile
# Set the compiler to clang
CC = clang
# Set the compiler flags to enable all warnings and debugging information
CFLAGS = -Wall -g
# Set the target executable name
TARGET = greetings
# Set the name of the file containing the sayings
FILE = pithy
# Set the destination directory for the symbolic links
DESTDIR = ~/bin

# Default target
all: $(TARGET) link

# Link the object files to create the target executable
$(TARGET): greet.o pithy.o
    $(CC) $(CFLAGS) -o $(TARGET) greet.o pithy.o

# Compile the greet.c source file into the greet.o object file
greet.o: greet.c pithy.h
    $(CC) $(CFLAGS) -c greet.c

# Compile the pithy.c source file into the pithy.o object file
pithy.o: pithy.c pithy.h
    $(CC) $(CFLAGS) -c pithy.c

# Create symbolic links to the target executable and the sayings file in the destination directory
link:
    ln -s $(PWD)/$(TARGET) $(DESTDIR)
    ln -s $(PWD)/$(FILE).txt $(DESTDIR)

# Remove the target executable, the object files, and the symbolic links
clean:
    rm -f $(TARGET) *.o
    rm -f $(DESTDIR)/$(TARGET)
    rm -f $(DESTDIR)/$(FILE).txt
```
### pithy.txt
```txt
Politics exists so that uncoordinated people can play sports.
Water alone doesn't get you clean. You must use soap. That's because dirt and crud loves soap and sticks to it really well. The water then washes away the soap, along with the dirt, and the result is that you are clean.
You buy popcorn, soda, and candy so that you have something to eat before the movie starts.
Just wait until Starbucks figures out that you can snort coffee.
Nothing instills more doubt in the curious than the sign "Wet Paint."
You might dislike texting, but it certainly does get annoying people to shut up.
You know you have an eating problem when you finish a meal and think, "Boy! When can I do that again?"
The middle of nowhere is equidistant from everywhere else.
Marketing wizards are looking for the human equivalent of what a dog feels at the sound of a can opener.
Having a pet ensures that you don't freak out over every noise in the house. Loud bang? It's the cat. So what if the cat is in the room with me. It's the cat.
The true experience at an amusement park is waiting in lines.
There is no logic in the computer industry.
The car's manual calls it the "check engine" lamp, but I call it the "This is going to cost at least $200" light.
You drive on a parkway and park on a driveway.
Do I take a break from work to play a video game, or take a break from a video game to get work done?
```
