// SPDX-License-Identifier: LGPL-3.0-or-later
// Software-only child: never opens or interprets a Singular/cdd input file.
#include <csignal>
#include <filesystem>
#include <iostream>
#include <string>
int main(int argc, char **argv) {
  const std::string mode = std::filesystem::path(argv[0]).filename().string();
  const bool backend = argc == 3 && std::string(argv[1]) == "-q";
  if (!(backend || argc == 2))
    return 64;
  std::cerr << "native fake child only; mode=" << mode << "\n";
  if (mode == "fake-child-malformed") {
    std::cout << "MALFORMED_FAKE_OUTPUT\n";
    return 0;
  }
  if (mode != "fake-child-exit0" && mode != "fake-child-exit17" &&
      mode != "fake-child-sigterm")
    return 64;
  std::cout << (backend ? "ENDPOLY\nSUCCESS-END\n" : "1 1\n0 0/1 1/1\n");
  std::cout.flush();
  if (mode == "fake-child-exit17")
    return 17;
  if (mode == "fake-child-sigterm") {
    std::raise(SIGTERM);
    return 65;
  }
  return 0;
}
