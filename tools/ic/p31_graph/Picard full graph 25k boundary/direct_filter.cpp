// Direct-index PP graph filter for the audited compact p31 edge stream.
// Each x coordinate has one signed 32-bit slot: zero=unseen,
// negative=root component size, positive=parent x plus one.
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <sys/mman.h>
#include <vector>

struct Edge { uint32_t ordinal, x0, x1; };
static_assert(sizeof(Edge) == 12);
constexpr uint32_t cutoff = 1717986917;

int main(int argc, char** argv) {
  if (argc != 3) throw std::runtime_error("usage: direct_filter edges.bin max_edges");
  uint64_t max_edges = std::stoull(argv[2]);
  if (!max_edges || max_edges > 350000000) throw std::runtime_error("max_edges outside bounded pilot");
  size_t bytes = size_t(cutoff) * sizeof(int32_t);
  auto* state = static_cast<int32_t*>(mmap(nullptr, bytes, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0));
  if (state == MAP_FAILED) throw std::runtime_error("anonymous mmap failed");
  std::vector<uint64_t> cyclic((size_t(cutoff) + 63) / 64, 0);
  auto has_cycle = [&](uint32_t x) { return (cyclic[x >> 6] >> (x & 63)) & 1ULL; };
  auto set_cycle = [&](uint32_t x) { cyclic[x >> 6] |= 1ULL << (x & 63); };
  std::ifstream in(argv[1], std::ios::binary);
  if (!in) throw std::runtime_error("edge file unavailable");
  auto start = std::chrono::steady_clock::now();
  uint64_t edges = 0, vertices = 0, components = 0, cycles = 0, cyclic_components = 0;
  uint32_t maximum = 0, last = 0;
  auto root = [&](uint32_t x) {
    while (state[x] > 0) {
      uint32_t y = uint32_t(state[x] - 1);
      if (state[y] > 0) state[x] = state[y];
      x = y;
    }
    return x;
  };
  Edge edge{};
  while (edges < max_edges && in.read(reinterpret_cast<char*>(&edge), sizeof(edge))) {
    if ((edges && edge.ordinal <= last) || edge.ordinal >= 937462500 || edge.x0 >= cutoff || edge.x1 >= cutoff)
      throw std::runtime_error("edge order or bound invalid");
    last = edge.ordinal;
    if (state[edge.x0] == 0) { state[edge.x0] = -1; ++vertices; ++components; maximum = std::max(maximum, 1u); }
    if (state[edge.x1] == 0) { state[edge.x1] = -1; ++vertices; ++components; maximum = std::max(maximum, 1u); }
    uint32_t u = root(edge.x0), v = root(edge.x1);
    if (u == v) {
      ++cycles;
      if (!has_cycle(u)) { set_cycle(u); ++cyclic_components; }
    }
    else {
      if (state[u] > state[v]) std::swap(u, v);  // More negative means larger.
      bool cu = has_cycle(u), cv = has_cycle(v);
      if (cu && cv) --cyclic_components;
      if (cv) set_cycle(u);
      state[u] += state[v];
      state[v] = int32_t(u + 1);
      --components;
      maximum = std::max(maximum, uint32_t(-state[u]));
    }
    ++edges;
    if (edges % 20000000 == 0) {
      double elapsed = std::chrono::duration<double>(std::chrono::steady_clock::now() - start).count();
      std::cerr << "edges " << edges << " vertices " << vertices << " cycles " << cycles << " seconds " << elapsed << '\n';
    }
  }
  if (edges != max_edges || edges - vertices + components != cycles || cyclic_components > cycles)
    throw std::runtime_error("short file or cycle identity mismatch");
  double seconds = std::chrono::duration<double>(std::chrono::steady_clock::now() - start).count();
  std::cout << "{\"edge_records\":" << edges << ",\"last_ordinal\":" << last
            << ",\"distinct_vertices\":" << vertices << ",\"components\":" << components
            << ",\"cycle_edges\":" << cycles << ",\"cyclic_components\":" << cyclic_components
            << ",\"weighted_lp_free_rows_lower_bound\":" << (cycles - cyclic_components)
            << ",\"largest_component_vertices\":" << maximum
            << ",\"elapsed_internal_seconds\":" << seconds << "}\n";
  munmap(state, bytes);
}
