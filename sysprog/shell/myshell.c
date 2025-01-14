// 总结一下使用的库函数功能并记录
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

#define MAX_INPUT_SIZE 1024
#define MAX_ARGS 64

// Parse the input command line into arguments
void parse_input(char *input, char **args) {
    int i = 0;
    args[i] = strtok(input, " \t\n"); // Split input by spaces, tabs, or newlines
    while (args[i] != NULL) {
        i++;
        args[i] = strtok(NULL, " \t\n");
    }
}

// Execute the command
void execute_command(char **args) {
    pid_t pid = fork(); // Create a child process

    if (pid < 0) {
        // Fork failed
        perror("fork");
    } else if (pid == 0) {
        // Child process
        if (execvp(args[0], args) == -1) { // Execute the command
            perror("execvp");
        }
        exit(EXIT_FAILURE); // Exit if execvp fails
    } else {
        // Parent process
        wait(NULL); // Wait for the child process to finish
    }
}

int main() {
    char input[MAX_INPUT_SIZE];
    char *args[MAX_ARGS];

    while (1) {
        // Print the shell prompt
        printf("mysh> ");
        fflush(stdout);

        // Read user input
        if (fgets(input, MAX_INPUT_SIZE, stdin) == NULL) {
            break; // Exit on EOF (Ctrl+D)
        }

        // Parse the input into arguments
        parse_input(input, args);

        // Check for empty input
        if (args[0] == NULL) {
            continue;
        }

        // Check for the "exit" command
        if (strcmp(args[0], "exit") == 0) {
            break;
        }

        // Execute the command
        execute_command(args);
    }

    printf("Exiting shell.\n");
    return 0;
}
