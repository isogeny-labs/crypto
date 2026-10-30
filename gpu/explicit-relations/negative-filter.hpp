#pragma once
#include "curve.cuh"
#include <stdexcept>

namespace explicit_relations {
// Conventional register-blocked Bloom prefilter. Only a definite absence may
// bypass exact lookup. Hash quality affects work, never witness acceptance.
// SplitMix64 finalizer: Sebastiano Vigna's public-domain splitmix.c (2015).
RF_HD U filter_mix(U x) {
  x = (x ^ (x >> 30)) * U(0xbf58476d1ce4e5b9ULL);
  x = (x ^ (x >> 27)) * U(0x94d049bb133111ebULL);
  return x ^ (x >> 31);
}
RF_HD U filter_hash(U key) {
  return filter_mix(key + U(0x9e3779b97f4a7c15ULL));
}
RF_HD U filter_pattern(U hash) {
  // Four positions from the lower 24 bits; collisions are harmless. Upper
  // bits select a word, disjoint from pattern bits for the admitted row cap.
  return (U(1) << (hash & 63)) | (U(1) << ((hash >> 6) & 63)) |
         (U(1) << ((hash >> 12) & 63)) | (U(1) << ((hash >> 18) & 63));
}
inline U filter_word_count(U rows) {
  if (rows > (U(1) << 32))
    throw std::runtime_error("LIMIT negative filter row capacity");
  if (!rows)
    return 0;
  // At least 8 bits/row, rounded to a power-of-two word count. At most4GiB.
  const U required = rows / 8 + (rows % 8 != 0);
  U words = 1;
  while (words < required)
    words *= 2;
  return words;
}
struct NegativeFilter {
  const unsigned long long *words = nullptr;
  U word_mask = 0;
  RF_HD bool maybe(U key) const {
    if (!words)
      return true; // Disabled, including empty plans: exact lookup decides.
    const U h = filter_hash(key), pattern = filter_pattern(h);
    return (words[(h >> 32) & word_mask] & pattern) == pattern;
  }
};
} // namespace explicit_relations
