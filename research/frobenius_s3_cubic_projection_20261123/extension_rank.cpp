// Rank of a sparse-input matrix over F_2[z]/g(z).
#include <cstdint>
#if defined(__PCLMUL__)
#include <array>
#include <immintrin.h>
#endif
#include <iostream>
#include <string_view>
#include <vector>

using Word = std::uint64_t;

class Field {
public:
    Field(unsigned degree, Word modulus, bool fast_arithmetic)
        : leading(Word{1} << degree), modulus(modulus), order(leading - 1) {
#if defined(__PCLMUL__)
        if (fast_arithmetic && degree <= 31) {
            carryless_degree = degree;
            Word reduction = modulus ^ leading;
            for (auto& slice : slices) {
                for (unsigned bit = 0; bit < 8; ++bit) {
                    for (unsigned i = 0; i < (1U << bit); ++i) {
                        slice[i | (1U << bit)] = slice[i] ^ reduction;
                    }
                    reduction <<= 1;
                    if (reduction & leading) {
                        reduction ^= modulus;
                    }
                }
            }
            return;
        }
#endif
        if (!fast_arithmetic || degree > 23) {
            return;
        }
        std::vector<Word> factors;
        Word remaining = order;
        for (Word p = 2; p * p <= remaining; ++p) {
            if (remaining % p == 0) {
                factors.push_back(p);
                do {
                    remaining /= p;
                } while (remaining % p == 0);
            }
        }
        if (remaining > 1) {
            factors.push_back(remaining);
        }
        Word primitive = 1;
        while (primitive < leading) {
            bool good = true;
            for (Word p : factors) {
                good &= pow(primitive, order / p) != 1;
            }
            if (good) {
                break;
            }
            ++primitive;
        }
        if (primitive == leading) {
            return;
        }
        log.resize(leading);
        exp.resize(order);
        Word value = 1;
        for (Word i = 0; i < order; ++i) {
            exp[i] = value;
            log[value] = i;
            value = polynomial_mul(value, primitive);
        }
    }

    Word mul(Word a, Word b) const {
        if (!a || !b) {
            return 0;
        }
        if (a == 1 || b == 1) {
            return a == 1 ? b : a;
        }
#if defined(__PCLMUL__)
        if (carryless_degree) {
            auto left = _mm_cvtsi64_si128(a);
            auto right = _mm_cvtsi64_si128(b);
            Word raw = _mm_cvtsi128_si64(_mm_clmulepi64_si128(left, right, 0));
            Word high = raw >> carryless_degree;
            return (raw & (leading - 1)) ^ slices[0][high & 255] ^ slices[1][(high >> 8) & 255]
                   ^ slices[2][(high >> 16) & 255] ^ slices[3][high >> 24];
        }
#endif
        if (log.empty()) {
            return polynomial_mul(a, b);
        }
        Word exponent = log[a] + log[b];
        return exp[exponent >= order ? exponent - order : exponent];
    }

    Word inverse(Word a) const {
        if (a == 1) {
            return 1;
        }
        return log.empty() ? pow(a, order - 1) : exp[order - log[a]];
    }

private:
    Word polynomial_mul(Word a, Word b) const {
        Word result = 0;
        while (b) {
            if (b & 1) {
                result ^= a;
            }
            b >>= 1;
            a <<= 1;
            if (a & leading) {
                a ^= modulus;
            }
        }
        return result;
    }

    Word pow(Word a, Word n) const {
        Word result = 1;
        while (n) {
            if (n & 1) {
                result = polynomial_mul(result, a);
            }
            n >>= 1;
            a = polynomial_mul(a, a);
        }
        return result;
    }

    Word leading;
    Word modulus;
    Word order;
#if defined(__PCLMUL__)
    unsigned carryless_degree = 0;
    std::array<std::array<Word, 256>, 4> slices{};
#endif
    std::vector<std::uint32_t> log;
    std::vector<std::uint32_t> exp;
};

int main(int argc, char** argv) {
    if (argc > 2 || (argc == 2 && std::string_view(argv[1]) != "--polynomial")) {
        return 1;
    }
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    unsigned degree, columns, rows;
    Word modulus;
    if (!(std::cin >> degree >> modulus >> columns >> rows) || degree < 2 || degree > 62) {
        return 1;
    }
    if ((modulus >> degree) != 1 || !(modulus & 1)) {
        return 1;
    }
    Field field(degree, modulus, argc == 1);
    std::vector<std::vector<Word>> pivots(columns);
    unsigned rank = 0;
    for (unsigned i = 0; i < rows; ++i) {
        std::vector<Word> row(columns);
        unsigned terms;
        if (!(std::cin >> terms)) {
            return 1;
        }
        for (unsigned j = 0; j < terms; ++j) {
            unsigned col;
            Word value;
            if (!(std::cin >> col >> value) || col >= columns || value >> degree) {
                return 1;
            }
            row[col] ^= value;
        }
        for (unsigned col = 0; col < columns; ++col) {
            const Word value = row[col];
            if (!value) {
                continue;
            }
            auto& pivot = pivots[col];
            if (pivot.empty()) {
                const Word inverse = field.inverse(value);
                for (unsigned w = col; w < columns; ++w) {
                    row[w] = field.mul(row[w], inverse);
                }
                pivot.swap(row);
                ++rank;
                break;
            }
            if (value == 1) {
                for (unsigned w = col; w < columns; ++w) {
                    row[w] ^= pivot[w];
                }
            } else {
                for (unsigned w = col; w < columns; ++w) {
                    if (pivot[w]) {
                        row[w] ^= field.mul(value, pivot[w]);
                    }
                }
            }
        }
    }
    std::cout << rank << '\n';
}
