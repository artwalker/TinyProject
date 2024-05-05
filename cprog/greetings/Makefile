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
	