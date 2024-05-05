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
