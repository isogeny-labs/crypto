#include <m4ri/m4ri.h>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>

static uint32_t u32(std::istream& in) {
  unsigned char b[4];
  if (!in.read(reinterpret_cast<char*>(b), 4)) throw std::runtime_error("header");
  return uint32_t(b[0]) | (uint32_t(b[1]) << 8) |
         (uint32_t(b[2]) << 16) | (uint32_t(b[3]) << 24);
}
static uint16_t u16(std::istream& in) {
  unsigned char b[2];
  if (!in.read(reinterpret_cast<char*>(b), 2)) throw std::runtime_error("row");
  return uint16_t(b[0]) | (uint16_t(b[1]) << 8);
}
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  std::ifstream in(argv[1], std::ios::binary);
  char magic[8];
  if (!in.read(magic, 8) || std::string(magic, 8) != std::string("G2M4R1\0\0", 8)) return 3;
  auto rows = u32(in), cols = u32(in), vars = u32(in), degree = u32(in);
  if (rows != 16399 || cols < 15856 || cols > 15867 || vars != 32 || degree != 4) return 4;
  mzd_t* mat = mzd_init(rows, cols);
  for (uint32_t i = 0; i < rows; ++i) {
    auto count = u16(in);
    int prev = -1;
    for (int j = 0; j < count; ++j) {
      auto col = u16(in);
      if (col >= cols || col <= prev) return 5;
      mzd_write_bit(mat, i, col, 1);
      prev = col;
    }
  }
  if (in.get() != std::char_traits<char>::eof()) return 6;
  std::cout << mzd_echelonize_m4ri(mat, 0, 5) << '\n';
  mzd_free(mat);
}
