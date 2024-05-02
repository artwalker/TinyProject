#include <iostream>
#include <fstream>
#include <vector>
#include <cstdlib>
#include <string>

#define BSIZE 256

void print_random_saying(const std::string &filename)
{
    std::ifstream file(filename);
    if (!file)
    {
        std::cerr << "Unable to open file " << filename << std::endl;
        exit(1);
    }

    std::vector<std::string> list_base;
    std::string line;
    while (std::getline(file, line))
    {
        if (line.size() > BSIZE)
        {
            line = line.substr(0, BSIZE);
        }
        list_base.push_back(line);
    }

    file.close();

    // Randomly select a saying from the list
    srand(time(0));
    int random_index = rand() % list_base.size();
    std::cout << list_base[random_index] << std::endl;
}