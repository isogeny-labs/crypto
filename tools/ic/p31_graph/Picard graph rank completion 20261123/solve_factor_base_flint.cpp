// Solve 24,999 p31 graph relations with the first factor-base log fixed to 1.
// The input is row-major 24,999 x 25,000 canonical uint64 residues.
#include <flint/flint.h>
#include <flint/nmod_mat.h>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <vector>

int main(int argc, char **argv) {
    if (argc != 3) throw std::runtime_error("usage: solve_factor_base_flint input.bin output.json");
    constexpr uint32_t n = 24999;
    constexpr uint64_t ell = 11832483282198303559ULL;
    std::ifstream in(argv[1], std::ios::binary | std::ios::ate);
    if (!in || in.tellg() != std::streamoff(uint64_t(n) * (n + 1) * 8))
        throw std::runtime_error("input size mismatch");
    in.seekg(0);
    flint_set_num_threads(8);
    nmod_mat_t A, B, X;
    nmod_mat_init(A, n, n, ell);
    nmod_mat_init(B, n, 1, ell);
    nmod_mat_init(X, n, 1, ell);
    for (uint32_t i = 0; i < n; ++i) {
        uint64_t value;
        if (!in.read(reinterpret_cast<char *>(&value), 8) || value >= ell)
            throw std::runtime_error("bad right-hand-side entry");
        nmod_mat_entry(B, i, 0) = value ? ell - value : 0;
        for (uint32_t j = 0; j < n; ++j) {
            if (!in.read(reinterpret_cast<char *>(&value), 8) || value >= ell)
                throw std::runtime_error("bad matrix entry");
            nmod_mat_entry(A, i, j) = value;
        }
        if ((i + 1) % 1000 == 0) std::cerr << "loaded " << i + 1 << " rows\n";
    }
    std::cerr << "starting finite-field solve\n";
    int invertible = nmod_mat_solve(X, A, B);
    if (!invertible) {
        std::cout << "{\"invertible_after_fixing_column_zero\":false}\n";
        return 2;
    }
    std::vector<uint64_t> vector(n + 1);
    vector[0] = 1;
    for (uint32_t j = 0; j < n; ++j) vector[j + 1] = nmod_mat_entry(X, j, 0);
    in.clear(); in.seekg(0);
    for (uint32_t i = 0; i < n; ++i) {
        uint64_t sum = 0, value;
        for (uint32_t j = 0; j <= n; ++j) {
            if (!in.read(reinterpret_cast<char *>(&value), 8)) throw std::runtime_error("verify read failed");
            if (value) sum = uint64_t((__uint128_t(sum) + (__uint128_t(value) * vector[j]) % ell) % ell);
        }
        if (sum) throw std::runtime_error("nonzero selected-row residual");
        if ((i + 1) % 1000 == 0) std::cerr << "verified " << i + 1 << " rows\n";
    }
    std::ofstream out(argv[2]);
    out << "{\"schema\":\"p31-graph-factor-base-kernel-v1\",\"ell\":" << ell
        << ",\"rows_verified\":" << n << ",\"columns\":" << n+1
        << ",\"fixed_column\":0,\"kernel\":[";
    for (uint32_t j = 0; j <= n; ++j) { if (j) out << ','; out << vector[j]; }
    out << "]}\n";
    out.flush();
    if (!out) throw std::runtime_error("output write failed");
    nmod_mat_clear(X); nmod_mat_clear(B); nmod_mat_clear(A);
    std::cout << "{\"invertible_after_fixing_column_zero\":true,\"rows_verified\":" << n << "}\n";
}
