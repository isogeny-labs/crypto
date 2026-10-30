// Differential query tests: deliberately repeated targets and prepares exercise
// reset ordering, duplicate sums, chunk tails, empty plans and radix bounds.
#include "device.cuh"
#include <iostream>
using namespace explicit_relations;
static void require(bool ok) {
  if (!ok)
    throw std::runtime_error("query boundary check failed");
}
int main() {
  try {
    require(!packed_winner_fits(0, 1));
    require(packed_winner_fits(1, UINT64_MAX));
    require(packed_winner_fits(UINT64_MAX, 1));
    require(!packed_winner_fits(UINT64_MAX, 2));
    require(packed_winner_fits(U(1) << 32, (U(1) << 32) - 1));
    require(!packed_winner_fits(U(1) << 32, U(1) << 32));
    Curve c = curve(97, 0, 7);
    std::vector<Point> ps;
    for (U x = 0; x < 97; ++x) {
      Point q;
      if (c.lift(x, 0, q))
        ps.push_back(q);
    }
    for (U chunk : {U(1), U(31), U(256), U(257), U(4096)}) {
      DeviceBackend device(c, 4096, chunk);
      HostBackend host(c, 4096, chunk);
      for (unsigned size : {3, 17, 0, 3}) {
        std::vector<std::vector<Point>> bases(4);
        for (auto &base : bases)
          base.assign(ps.begin(), ps.begin() + size);
        Plan p = plan(c, bases, 4096);
        device.prepare(p);
        host.prepare(p);
        for (Point target : {ps.front(), Point{}, ps.back(), ps.front()}) {
          Result got = device.find(target, p.left + p.right);
          Result expected = host.find(target, p.left + p.right);
          require(got.found == expected.found &&
                  got.indices == expected.indices);
          U visited = p.right;
          if (got.found) {
            U right = 0;
            for (unsigned j = p.split; j < p.slots; ++j)
              right = right * size + expected.indices[j];
            visited = std::min(p.right, (right / chunk + 1) * chunk);
          }
          require(got.visited == visited);
        }
      }
    }
    std::cout
        << "PASS packed overflow bounds, deterministic witnesses, repeated "
           "queries/prepares, chunk tails, empty plans and visited counts\n";
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
