#include "../../../measurement_hook.hpp"
#include <cstring>
#include <fstream>
#include <iostream>
#include <unistd.h>
int main(int argc, char **argv) {
  try {
    CubicMeasurement observe;
    if (argc > 2 && std::strcmp(argv[1], "mutate-input") == 0)
      std::ofstream(argv[2]) << "changed";
    if (argc > 1 && std::strcmp(argv[1], "background") == 0) {
      if (fork() == 0) { sleep(30); _exit(0); }
    }
    if (argc > 1 && std::strcmp(argv[1], "sleep") == 0) sleep(30);
    if (argc > 1 && std::strcmp(argv[1], "fail") == 0) return 7;
    std::cout << "{\"fixture\":\"cpp\",\"value\":42}\n";
    return 0;
  } catch (const std::exception &e) { std::cerr << e.what() << '\n'; return 8; }
}
