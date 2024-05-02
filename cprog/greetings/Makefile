CC = clang
CFLAGS = -Wall -g
TARGET = greetings
FILE = pithy
DESTDIR = ~/bin

all: $(TARGET) link

$(TARGET): greet.o pithy.o
	$(CC) $(CFLAGS) -o $(TARGET) greet.o pithy.o

greet.o: greet.c pithy.h
	$(CC) $(CFLAGS) -c greet.c

pithy.o: pithy.c pithy.h
	$(CC) $(CFLAGS) -c pithy.c

link:
	ln -s $(PWD)/$(TARGET) $(DESTDIR)
	ln -s $(PWD)/$(FILE).txt $(DESTDIR)

clean:
	rm -f $(TARGET) *.o
	rm -f $(DESTDIR)/$(TARGET)
	rm -f $(DESTDIR)/$(FILE).txt
