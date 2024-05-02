# Project Analysis and Usage Guide

## Project Analysis

This project is a C language program, the main function of which is to randomly select and print a quote from a text file. The `print_random_saying` function in the project is responsible for this function. This function is declared in the `pithy.h` header file and defined in the `pithy.c` source file.

The project uses a Makefile to automate the compilation and linking process. The Makefile defines several rules for compiling source files (`greet.c` and `pithy.c`), linking the generated object files (`greet.o` and `pithy.o`), and creating an executable file (`greetings`). The Makefile also defines a rule for creating symbolic links to the `pithy.txt` file and the `greetings` executable file.

## Usage Guide

1. **Compile the project**: In the terminal, navigate to the directory containing the Makefile, then enter `make` or `make all`. This will compile the source files, link the generated object files to create an executable file.

2. **Run the program**: In the terminal, navigate to the `~/bin` directory, then enter `./greetings`. This will run the program, and the program will randomly select and print a quote from the `pithy.txt` file.

3. **Clean up the project**: If you want to delete the generated executable file and object files, as well as the links created in the `~/bin` directory, you can enter `make clean` in the terminal.

4. **Configure .zshrc**: If you want to automatically run the `greetings` program when starting a new shell session, you can add the following line to the `.zshrc` file:

```bash
cd ~/bin && ./greetings
```

This will first switch to the `~/bin` directory, then run the `greetings` program.

Note: This project assumes that you have installed a C compiler (such as clang) on your system. If you do not have a C compiler installed on your system, you need to install a C compiler to compile and run this project.