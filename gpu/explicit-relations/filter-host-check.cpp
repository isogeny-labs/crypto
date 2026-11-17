#include "negative-filter.hpp"
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace explicit_relations;
int main() {
  assert(filter_word_count(0) == 0);
  assert(filter_word_count(1) == 1);
  assert(filter_word_count(8) == 1);
  assert(filter_word_count(9) == 2);
  assert(filter_word_count(U(1) << 32) == (U(1) << 29));
  bool rejected = false;
  try { (void)filter_word_count((U(1) << 32) + 1); }
  catch (const std::runtime_error &) { rejected = true; }
  assert(rejected);
  for (U count : {U(1), U(8), U(9), U(100003)}) {
    const U words = filter_word_count(count);
    std::vector<unsigned long long> bits(words, 0);
    NegativeFilter view{bits.data(), words - 1};
    std::vector<U> keys{0, 1, UINT64_MAX, UINT64_MAX - 1};
    U state = 0x345678abcdefULL;
    for (U i = 0; i < count; ++i) {
      state ^= state << 13; state ^= state >> 7; state ^= state << 17;
      keys.push_back(state);
    }
    for (U key : keys) {
      assert(!view.maybe(key));
      assert(NegativeFilter{}.maybe(key));
    }
    for (U key : keys) {
      const U h = filter_hash(key);
      bits[(h >> 32) & (words - 1)] |= filter_pattern(h);
    }
    for (U key : keys) assert(view.maybe(key));
    // A saturated prefilter must pass everything, never manufacture a witness.
    std::fill(bits.begin(), bits.end(), UINT64_MAX);
    for (U key : keys) assert(view.maybe(~key));
    std::fill(bits.begin(), bits.end(), 0);
    for (U key : keys) assert(!view.maybe(key));
  }
  std::cout << "PASS negative filter capacity, membership, disabled, saturated and reset fixtures\n";
}
