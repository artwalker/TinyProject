CXX = clang++
CXXFLAGS = -Wall -g
TARGET = greetings
FILE = pithy
DESTDIR = ~/bin

all: $(TARGET) link

$(TARGET): greet.o pithy.o
	$(CXX) $(CXXFLAGS) -o $(TARGET) greet.o pithy.o

greet.o: greet.cpp pithy.h
	$(CXX) $(CXXFLAGS) -c greet.cpp

pithy.o: pithy.cpp pithy.h
	$(CXX) $(CXXFLAGS) -c pithy.cpp

link:
	ln -s $(PWD)/$(TARGET) $(DESTDIR)
	ln -s $(PWD)/$(FILE).txt $(DESTDIR)

clean:
	rm -f $(TARGET) *.o
	rm -f $(DESTDIR)/$(TARGET)
	rm -f $(DESTDIR)/$(FILE).txt