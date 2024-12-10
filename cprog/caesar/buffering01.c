#include <stdio.h>

int main()
{
	char buffer[BUFSIZ];
	int a,b;

	setbuf(stdout,buffer);
	// This means that the output is buffered and 
	// not immediately flushed to the screen.
	// so the string 'Type a letter' also is stored in buffer.

	printf("Type a letter:");
	a = getchar();
	printf("Type a letter:");
	b = getchar();

	printf("a='%c', b='%c'\n",a,b);


	return(0);
}

