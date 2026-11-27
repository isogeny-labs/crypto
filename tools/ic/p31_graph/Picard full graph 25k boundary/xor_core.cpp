// Exact multigraph 2-core by degree/XOR leaf peeling. The input format is
// (pair ordinal, first x, second x), three little-endian u32 words per edge.
// A degree-one vertex has exactly one live neighbor, held in neighbor_xor.
#include <chrono>
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <vector>
#include <bit>

struct Edge { uint32_t ordinal, x0, x1; };
static_assert(sizeof(Edge) == 12);
constexpr uint32_t cutoff = 1717986917;

int main(int argc, char** argv) {
  if (argc != 4) throw std::runtime_error("usage: xor_core edges.bin core_edges.bin max_edges");
  const uint64_t limit = std::stoull(argv[3]);
  if (!limit || limit > 299945714) throw std::runtime_error("invalid edge limit");
  const auto start = std::chrono::steady_clock::now();
  std::ifstream input(argv[1], std::ios::binary);
  if (!input) throw std::runtime_error("edge file unavailable");
  std::vector<uint64_t> occupied((size_t(cutoff) + 63) / 64);
  uint32_t last = 0;
  Edge edge{};
  for (uint64_t i = 0; i < limit; ++i) {
    if (!input.read(reinterpret_cast<char*>(&edge), sizeof(edge))) throw std::runtime_error("short edge file");
    if ((i && edge.ordinal <= last) || edge.ordinal >= 937462500 ||
        edge.x0 >= cutoff || edge.x1 >= cutoff) throw std::runtime_error("invalid edge");
    last = edge.ordinal;
    occupied[edge.x0 >> 6] |= 1ULL << (edge.x0 & 63);
    occupied[edge.x1 >> 6] |= 1ULL << (edge.x1 & 63);
  }
  std::vector<uint32_t> prefix(occupied.size() + 1);
  for (size_t i = 0; i < occupied.size(); ++i)
    prefix[i+1] = prefix[i] + std::popcount(occupied[i]);
  const uint32_t vertices = prefix.back();
  auto index = [&](uint32_t x) -> uint32_t {
    const uint32_t word = x >> 6;
    const uint64_t lower = occupied[word] & ((1ULL << (x & 63)) - 1);
    return prefix[word] + std::popcount(lower);
  };
  std::vector<uint8_t> degree(vertices);
  std::vector<uint32_t> neighbor_xor(vertices);
  input.clear(); input.seekg(0);
  for (uint64_t i = 0; i < limit; ++i) {
    if (!input.read(reinterpret_cast<char*>(&edge), sizeof(edge))) throw std::runtime_error("second pass short");
    uint32_t a = index(edge.x0), b = index(edge.x1);
    if (degree[a] == 255 || degree[b] == 255 || (a == b && degree[a] >= 254))
      throw std::runtime_error("degree overflow");
    ++degree[a]; ++degree[b];
    neighbor_xor[a] ^= edge.x1;
    neighbor_xor[b] ^= edge.x0;
  }
  std::vector<uint32_t> queue;
  queue.reserve(vertices);
  for (uint32_t word = 0; word < occupied.size(); ++word) {
    uint64_t bits = occupied[word];
    uint32_t i = prefix[word];
    while (bits) {
      uint32_t x = word * 64 + std::countr_zero(bits);
      if (degree[i] == 1) queue.push_back(x);
      bits &= bits - 1;
      ++i;
    }
  }
  uint64_t peeled = 0;
  for (size_t pos = 0; pos < queue.size(); ++pos) {
    uint32_t x = queue[pos];
    uint32_t a = index(x);
    if (degree[a] != 1) continue;
    uint32_t y = neighbor_xor[a];
    if (y >= cutoff || !(occupied[y >> 6] & (1ULL << (y & 63))))
      throw std::runtime_error("peel neighbor absent");
    uint32_t b = index(y);
    if (degree[b] == 0) throw std::runtime_error("peel invariant failed");
    degree[a] = 0;
    neighbor_xor[a] = 0;
    --degree[b];
    neighbor_xor[b] ^= x;
    if (degree[b] == 1) queue.push_back(y);
    ++peeled;
  }
  uint64_t core_vertices = 0, degree_sum = 0;
  for (uint32_t i = 0; i < vertices; ++i) if (degree[i]) {
    if (degree[i] < 2) throw std::runtime_error("residual leaf");
    ++core_vertices; degree_sum += degree[i];
  }
  input.clear(); input.seekg(0);
  std::ofstream output(argv[2], std::ios::binary | std::ios::trunc);
  if (!output) throw std::runtime_error("core output unavailable");
  uint64_t core_edges = 0;
  for (uint64_t i = 0; i < limit; ++i) {
    if (!input.read(reinterpret_cast<char*>(&edge), sizeof(edge))) throw std::runtime_error("second pass short");
    if (degree[index(edge.x0)] && degree[index(edge.x1)]) {
      output.write(reinterpret_cast<const char*>(&edge), sizeof(edge));
      ++core_edges;
    }
  }
  if (!output || core_edges * 2 != degree_sum) throw std::runtime_error("core degree identity failed");
  const double seconds = std::chrono::duration<double>(std::chrono::steady_clock::now() - start).count();
  std::cout << "{\"input_edges\":" << limit << ",\"input_vertices\":" << vertices
            << ",\"peeled_vertices\":" << peeled << ",\"core_vertices\":" << core_vertices
            << ",\"core_edges\":" << core_edges << ",\"core_bytes\":" << core_edges * sizeof(Edge)
            << ",\"elapsed_internal_seconds\":" << seconds << "}\n";
}
