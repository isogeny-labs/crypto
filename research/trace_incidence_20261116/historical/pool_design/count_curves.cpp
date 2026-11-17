#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <numeric>
#include <random>
#include <vector>

// Bounded exact curve search. Inputs are a prime p, requested number of
// curves and a seed. Counts are exact by quadratic-residue lookup; no
// probabilistic point-counting assertion is made.
static bool prime(uint64_t n) {
    if (n < 2) return false;
    if (!(n & 1)) return n == 2;
    for (uint64_t d = 3; d*d <= n; d += 2)
        if (n % d == 0) return false;
    return true;
}
int main(int argc, char** argv) {
    if (argc != 4) return 2;
    uint64_t p = std::strtoull(argv[1], nullptr, 10);
    unsigned wanted = std::strtoul(argv[2], nullptr, 10);
    uint64_t seed = std::strtoull(argv[3], nullptr, 10);
    if (!prime(p) || p <= 3 || p > 20000000) return 3;
    std::vector<unsigned char> square(p, 0);
    for (uint64_t y = 1; y < p; ++y) square[y*y % p] = 1;
    std::mt19937_64 rng(seed);
    unsigned found = 0;
    uint64_t tries = 0;
    auto started = std::chrono::steady_clock::now();
    std::cout << "[";
    while (found < wanted && tries < 10000) {
        ++tries;
        uint64_t a = 1 + rng() % (p-1), b = 1 + rng() % (p-1);
        uint64_t disc = (4 * (a*a % p) % p * a + 27 * (b*b % p)) % p;
        if (!disc) continue;
        int64_t character_sum = 0;
        uint64_t f = b, first = (1+a) % p, second = 6 % p;
        for (uint64_t x = 0; x < p; ++x) {
            if (f) character_sum += square[f] ? 1 : -1;
            f += first; if (f >= p) f -= p;
            first += second; if (first >= p) first -= p;
            second += 6; if (second >= p) second -= p;
        }
        uint64_t order = p + 1 + character_sum;
        if (order == p || !prime(order)) continue;
        if (found++) std::cout << ",";
        std::cout << "{\"p\":" << p << ",\"a\":" << a
                  << ",\"b\":" << b << ",\"r\":" << order
                  << ",\"curves_tested_cumulative\":" << tries << "}";
    }
    std::cout << "]\n";
    return found == wanted ? 0 : 4;
}
