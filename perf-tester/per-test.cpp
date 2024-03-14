/*
Perf-Test is a perfomance tester application that can run binary files and measure their performance.

Basic usage:
    perftest <lang> <binary> <args> <options> > <output_file.txt>

<lang> : The language of the binary file
<args> : Arguments to pass to the binary Ex: "node_count=1000"
*/

#include <iostream>
#include <string>
#include <chrono>

void print_usage() {
    std::cout << "Usage: perftest <lang> <binary> <args> <options> > <output_file.txt>" << std::endl;
}

int main(int argc, char *argv[]) {
    if (argc < 4) {
        print_usage();
        return 1;
    }

    std::string lang = argv[1];
    std::string binary = argv[2];
    std::string args = argv[3];
    std::string options = argv[4];

    // start timer
    auto start_time = std::chrono::high_resolution_clock::now();

    // run binary
    


    // stop timer
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);

    // print results with lang, binary, args, options
    std::cout << "lang: " << lang << std::endl;
    std::cout << "binary: " << binary << std::endl;
    std::cout << "args: " << args << std::endl;
    std::cout << "options: " << options << std::endl;

