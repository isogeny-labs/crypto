// Multiply two sparse GF(2^n) polynomials modulo a binary subspace polynomial.
#include <algorithm>
#include <array>
#include <cstdint>
#include <iostream>
#if defined(__PCLMUL__)
#include <immintrin.h>
#endif
#include <unordered_map>
#include <vector>

using Word = std::uint64_t;
using Terms = std::unordered_map<Word, Word>;

class Field {
public:
    Field(unsigned degree, Word modulus)
        : degree(degree), leading(Word{1} << degree), modulus(modulus) {
#if defined(__PCLMUL__)
        Word power = modulus ^ leading;
        for (auto& slice : reduction) {
            for (unsigned bit = 0; bit < 8; ++bit) {
                for (unsigned j = 0; j < (1U << bit); ++j) {
                    slice[j | (1U << bit)] = slice[j] ^ power;
                }
                power <<= 1;
                if (power & leading) power ^= modulus;
            }
        }
#endif
    }

    Word mul(Word a, Word b) const {
#if defined(__PCLMUL__)
        const auto raw = _mm_clmulepi64_si128(_mm_cvtsi64_si128(a), _mm_cvtsi64_si128(b), 0);
        Word low = static_cast<Word>(_mm_cvtsi128_si64(raw));
        const Word upper = static_cast<Word>(_mm_cvtsi128_si64(_mm_srli_si128(raw, 8)));
        Word high = (low >> degree) | (upper << (64 - degree));
        low &= leading - 1;
        for (unsigned i = 0; i < (degree + 7) / 8; ++i) {
            low ^= reduction[i][high & 255];
            high >>= 8;
        }
        return low;
#else
        Word result = 0;
        while (b) {
            if (b & 1) result ^= a;
            b >>= 1;
            a <<= 1;
            if (a & leading) a ^= modulus;
        }
        return result;
#endif
    }

private:
    unsigned degree;
    Word leading, modulus;
#if defined(__PCLMUL__)
    std::array<std::array<Word, 256>, 8> reduction{};
#endif
};

Word evaluate(const Field& field, const Terms& terms, Word at, unsigned width) {
    std::vector<Word> powers{at};
    for (unsigned i = 1; i < width; ++i) {
        powers.push_back(field.mul(powers.back(), powers.back()));
    }
    Word total = 0;
    for (const auto& [exponent, value] : terms) {
        Word coefficient = value;
        for (Word m = exponent, i = 0; m; m >>= 1, ++i) {
            if (m & 1) coefficient = field.mul(coefficient, powers[i]);
        }
        total ^= coefficient;
    }
    return total;
}

int main() {
    unsigned degree, width;
    Word field_modulus, selector_modulus, check;
    if (!(std::cin >> degree >> field_modulus >> width >> selector_modulus >> check) ||
        degree < 2 || degree > 63 || width < 1 || width > 62) return 1;
    Field field(degree, field_modulus);
    const Word bound = Word{1} << width;
    std::vector<Word> low_powers;
    for (unsigned i = 0; i < width; ++i) {
        if (selector_modulus >> i & 1) low_powers.push_back(Word{1} << i);
    }
    auto read = [] {
        unsigned count;
        std::cin >> count;
        Terms terms;
        terms.reserve(count);
        for (unsigned i = 0; i < count; ++i) {
            Word exponent, coefficient;
            std::cin >> exponent >> coefficient;
            terms.emplace(exponent, coefficient);
        }
        return terms;
    };
    auto a = read();
    auto b = read();
    Terms convolution;
    for (const auto& [i, x] : a) {
        for (const auto& [j, y] : b) {
            convolution[i + j] ^= field.mul(x, y);
        }
    }
    Terms result;
    for (const auto& [e, value] : convolution) {
        if (!value) continue;
        if (e < bound) {
            result[e] ^= value;
            continue;
        }
        for (Word lower : low_powers) {
            Word reduced = e - bound + lower;
            if (reduced < bound) {
                result[reduced] ^= value;
            } else {
                // Each reduction lowers the exponent by at least bound / 2.
                for (Word next : low_powers) {
                    result[reduced - bound + next] ^= value;
                }
            }
        }
    }
    if (evaluate(field, result, check, width) !=
        field.mul(evaluate(field, a, check, width), evaluate(field, b, check, width))) return 2;
    for (const auto& [e, value] : result) {
        if (value) std::cout << e << ' ' << value << '\n';
    }
}
