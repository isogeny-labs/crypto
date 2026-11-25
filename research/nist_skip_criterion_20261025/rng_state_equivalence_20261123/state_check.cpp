// Exact source-model arithmetic checks. No primality test or curve generation.
// Independent implementation of the documented FreeLIP state recurrence.
#include <array>
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string>
using U = unsigned __int128;
constexpr U M = (U(1) << 107) - 1;
constexpr U radix = U(1) << 26;
// Inputs stay below M; each addition/doubling stays below 2^108 < 2^128.
U mul(U a, U b) {
    U c = 0;
    while (b) {
        if (b & 1) c = (c + a) % M;
        a = (a + a) % M;
        b >>= 1;
    }
    return c;
}
U pow_mod(U a, unsigned n) {
    U v = 1;
    while (n) {
        if (n & 1) v = mul(v, a);
        a = mul(a, a);
        n >>= 1;
    }
    return v;
}
std::string decimal(U a) {
    if (!a) return "0";
    std::string s;
    while (a) { s.insert(s.begin(), char('0' + a % 10)); a /= 10; }
    return s;
}
void require(bool v) { if (!v) throw std::runtime_error("arithmetic control failed"); }
U rejection_bound(U b) {
    require(b > 0 && b <= radix);
    return M - M % b;
}
bool accepts(U state, U b) { return state < rejection_bound(b); }
int main() {
    try {
        // Source-defined boundary controls, including exact equality rejection.
        require(rejection_bound(1) == M);
        require(rejection_bound(radix) == M - radix + 1);
        const U boundary = rejection_bound(radix);
        require(accepts(boundary - 1, radix));
        require(!accepts(boundary, radix));
        require(mul(M-1, M-1) == 1);
        const U g = pow_mod(3, 121);
        // Published recovered seeds, not fitted or searched here.
        const std::array<std::uint32_t,6> seeds{20000,20420,20507,20509,20512,20517};
        std::cout << "seed\tstep\tstate\tall_26bit_bounds_first_advance\treference_equal\n";
        unsigned checked = 0;
        for (auto seed : seeds) {
            U state = seed;
            U reference = seed;
            for (unsigned k=1;k<=7;++k) {
                state = mul(state, g);
                // Distinct arithmetic route: 121 modular triplings per step.
                for (unsigned j=0;j<121;++j) reference=(reference*3)%M;
                require(state == reference);
                // For every 1<=b<=2^26, M%b<=b-1<=2^26-1.
                // This sufficient inequality proves no rejection for ANY bound.
                require(state < M - radix + 1);
                std::cout << seed << '\t' << k << '\t' << decimal(state)
                          << "\ttrue\ttrue\n";
                ++checked;
            }
        }
        require(checked == 42);
        std::cerr << "42 exact source-model states agree; all accept first advance for every positive bound <=2^26.\n";
        return 0;
    } catch (const std::exception& e) { std::cerr << e.what() << '\n'; return 1; }
}
