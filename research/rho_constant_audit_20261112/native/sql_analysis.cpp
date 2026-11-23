#include <sqlite3.h>
#include <fstream>
#include <iostream>
#include <iterator>
#include <stdexcept>
#include <string>

static std::string read_file(const char* path) {
  std::ifstream in(path, std::ios::binary);
  if (!in) throw std::runtime_error(std::string("cannot read ") + path);
  return {std::istreambuf_iterator<char>(in), std::istreambuf_iterator<char>()};
}
static int output(void*, int n, char** values, char**) {
  if (n != 1 || values[0] == nullptr) return 1;
  std::cout << values[0] << '\n';
  return std::cout ? 0 : 1;
}
int main(int argc, char** argv) {
  if (argc == 2 && std::string(argv[1]) == "--version") {
    std::cout << sqlite3_libversion() << '\n';
    return 0;
  }
  if (argc != 3) {
    std::cerr << "usage: sql_analysis native-index.sql queries.sql\n";
    return 2;
  }
  sqlite3* db = nullptr;
  try {
    if (sqlite3_open(":memory:", &db) != SQLITE_OK)
      throw std::runtime_error("sqlite open failed");
    for (int i = 1; i != 3; ++i) {
      char* error = nullptr;
      const std::string sql = read_file(argv[i]);
      if (sqlite3_exec(db, sql.c_str(), i == 2 ? output : nullptr, nullptr, &error) != SQLITE_OK) {
        const std::string message = error ? error : "sqlite execution failed";
        sqlite3_free(error);
        throw std::runtime_error(message);
      }
    }
    if (sqlite3_close(db) != SQLITE_OK) throw std::runtime_error("sqlite close failed");
    return 0;
  } catch (const std::exception& e) {
    std::cerr << e.what() << '\n';
    if (db != nullptr) sqlite3_close(db);
    return 1;
  }
}
