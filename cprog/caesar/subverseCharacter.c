#include <stdio.h>
#include <ctype.h>

int main(){
    int ch;

    while ((ch = getchar()) != EOF) {
        if (ch >= 'A' && ch <= 'Z') {
            ch = tolower(ch);
        } else if (ch >= 'a' && ch <= 'z') {
            ch = toupper(ch);
        }
        putchar(ch);
    }
    return 0;
}