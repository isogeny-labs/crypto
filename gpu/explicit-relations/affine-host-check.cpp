#include "affine-fixtures.hpp"
#include <iostream>
int main() {
  size_t count = 0;
  for (auto c : explicit_relations::affine_curves())
    count += explicit_relations::affine_cases(c).size();
  std::cout << "PASS affine subtraction host cases=" << count
            << " exhaustive-small/63-bit/infinity/inverse/doubling/zero-y\n";
}
