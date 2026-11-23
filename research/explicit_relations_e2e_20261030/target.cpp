// Public target selected by coordinate lifting. No target scalar is generated
// or supplied to the solver. Native field implementation differs from Rust
// replay.
#include "../../gpu/explicit-relations/host.hpp"
#include <iostream>
using namespace explicit_relations;
int main(int argc, char **argv) {
  if (argc != 5)
    return 2;
  Curve c =
      curve(std::stoull(argv[1]), std::stoull(argv[2]), std::stoull(argv[3]));
  U seed = std::stoull(argv[4]);
  for (U i = 0; i < 1000; ++i) {
    Point q;
    if (c.lift((seed + i) % c.f.p, 0, q)) {
      std::cout << q.x << ' ' << q.y << '\n';
      return 0;
    }
  }
  return 3;
}
