#include <stdio.h>

int main()
{
	int ch;

	while ((ch = getchar()) != EOF)
		putchar(ch);
	return (0);
}
// Ctrl+D = EOF - Linux
