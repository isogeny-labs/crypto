// spmv-oracle-check.cpp -- host oracle for the CUDA worker's arithmetic.
//
// spmv.cu computes products with spmv_mul_mod (128-bit Barrett) on
// host-reduced operands; the CPU worker (spmv_solve.cpp) and the previous
// CUDA kernel used the add-and-double loop on the raw x.  This compiles the
// same spmv_math.cuh the kernel uses and checks, bit for bit:
//   1. spmv_mul_mod(a mod m, b mod m) == add-and-double(a mod m, b) for
//      edge-case and random moduli and operands (b unreduced, as before);
//   2. spmv_task over random CSR problems == the CPU worker's loop.
// No GPU needed.  Exit status 0 means every comparison matched.
#include "spmv_math.cuh"

#include <cstdio>
#include <vector>

static std::uint64_t ref_add_mod(std::uint64_t a, std::uint64_t b, std::uint64_t m) {
    const auto sum = a + b;
    return sum >= m ? sum - m : sum;
}
static std::uint64_t ref_multiply_mod(std::uint64_t a, std::uint64_t b, std::uint64_t m) {
    std::uint64_t result = 0;
    while (b != 0) {
        if (b & 1) result = ref_add_mod(result, a, m);
        b >>= 1;
        if (b != 0) a = ref_add_mod(a, a, m);
    }
    return result;
}

static std::uint64_t rng_state = 0x243F6A8885A308D3ull;
static std::uint64_t next() {
    std::uint64_t z = (rng_state += 0x9E3779B97F4A7C15ull);
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ull;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EBull;
    return z ^ (z >> 31);
}

int main() {
    long checks = 0, bad = 0;
    std::vector<std::uint64_t> moduli = {2, 3, 4, 5, 7, 255, 256, 257, 65521,
                                         (1ull << 31) - 1, 1ull << 32, (1ull << 32) + 15,
                                         (1ull << 61) - 1, 1ull << 62, (1ull << 62) + 1,
                                         (1ull << 63) - 25, (1ull << 63) - 1, (1ull << 63) - 2};
    for (int i = 0; i < 200; ++i) {
        const int bits = 2 + (int)(next() % 62);
        std::uint64_t m = next() >> (64 - bits);
        if (m < 2) m = 2;
        if (m >= (1ull << 63)) m = (1ull << 63) - 1;
        moduli.push_back(m);
    }
    for (const std::uint64_t m : moduli) {
        const SpmvModulus mod = spmv_modulus(m);
        std::vector<std::uint64_t> ops = {0, 1, 2, m - 1, m - 2, m / 2, m, m + 1,
                                          ~0ull, ~0ull - 1, 1ull << 63, (1ull << 63) - 1};
        for (int k = 0; k < 64; ++k) ops.push_back(next());
        for (int k = 0; k < 16; ++k) ops.push_back(next() % m);
        for (std::uint64_t a0 : ops)
            for (std::uint64_t b : ops) {
                const std::uint64_t a = a0 % m; // both paths reduce the coefficient
                const std::uint64_t want = ref_multiply_mod(a, b, m);
                const std::uint64_t got = spmv_mul_mod(a, b % m, mod);
                ++checks;
                if (want != got) {
                    if (bad++ < 10)
                        std::printf("MISMATCH m=%llu a=%llu b=%llu want=%llu got=%llu\n",
                                    (unsigned long long)m, (unsigned long long)a,
                                    (unsigned long long)b, (unsigned long long)want,
                                    (unsigned long long)got);
                }
            }
    }
    // Random CSR problems: the CPU worker's loop vs the kernel's task body.
    long tasks = 0;
    for (int trial = 0; trial < 300; ++trial) {
        const std::uint64_t m = moduli[next() % moduli.size()];
        const std::uint64_t rows = 1 + next() % 40, columns = 1 + next() % 40,
                            lanes = 1 + next() % 9;
        std::vector<std::uint64_t> row_ptr(rows + 1, 0), col, coef,
            x(columns * lanes);
        for (std::uint64_t r = 0; r < rows; ++r) {
            const std::uint64_t nnz = next() % 12;
            for (std::uint64_t k = 0; k < nnz; ++k) {
                col.push_back(next() % columns);
                coef.push_back(next() % 3 ? next() : m - 1);
            }
            row_ptr[r + 1] = col.size();
        }
        for (auto &v : x) v = next() % 4 ? next() : m - 1 - next() % 2;
        std::vector<std::uint64_t> want(rows * lanes, 0);
        for (std::uint64_t r = 0; r < rows; ++r)
            for (std::uint64_t at = row_ptr[r]; at < row_ptr[r + 1]; ++at) {
                const auto c = coef[at] % m;
                for (std::uint64_t l = 0; l < lanes; ++l)
                    want[r * lanes + l] = ref_add_mod(
                        want[r * lanes + l], ref_multiply_mod(c, x[col[at] * lanes + l], m), m);
            }
        auto rc = coef, rx = x;
        for (auto &v : rc) v %= m;
        for (auto &v : rx) v %= m;
        const SpmvModulus mod = spmv_modulus(m);
        for (std::uint64_t t = 0; t < rows * lanes; ++t) {
            const std::uint64_t got = spmv_task(row_ptr.data(), col.data(), rc.data(),
                                                rx.data(), t / lanes, t % lanes, lanes, mod);
            ++tasks;
            if (got != want[t] && bad++ < 20)
                std::printf("TASK MISMATCH trial=%d task=%llu\n", trial, (unsigned long long)t);
        }
    }
    std::printf("%s: %ld product checks, %ld task checks, %ld mismatches\n",
                bad ? "FAILED" : "spmv oracle ok", checks, tasks, bad);
    return bad ? 1 : 0;
}
