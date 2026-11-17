#pragma once
#include <functional>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

namespace tests {
using Test = std::pair<const char*, std::function<void()>>;
inline std::vector<Test>& all() { static std::vector<Test> value; return value; }
struct Add { Add(const char* name, std::function<void()> test) { all().push_back({name, test}); } };
inline int run() {
  int failures = 0;
  for (const auto& test : all()) {
    try { test.second(); std::cout << "PASS " << test.first << '\n'; }
    catch (const std::exception& error) { ++failures; std::cerr << "FAIL " << test.first << ": " << error.what() << '\n'; }
  }
  std::cout << all().size() - failures << '/' << all().size() << " tests passed\n";
  return failures ? 1 : 0;
}
}
#define CHECK(expression) do { if (!(expression)) throw std::runtime_error(std::string(__FILE__) + ":" + std::to_string(__LINE__) + " " #expression); } while (false)
#define TEST(name) static void name(); static tests::Add add_##name(#name, name); static void name()
