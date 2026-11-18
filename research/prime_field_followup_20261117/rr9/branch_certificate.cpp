#include <algorithm>
#include <array>
#include <cassert>
#include <chrono>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <vector>

using F = std::uint32_t;
constexpr F p = 2147483647, a = 847514846, b = 2087444724;
constexpr std::array<F, 8> xs{0,1,2,3,5,6,8,9};
using Row = std::array<F, 16>;
using Forms = std::array<std::array<Row, 2>, 8>;

F add(F x, F y) {
    F sum = x + y;
    return sum >= p ? sum - p : sum;
}

F sub(F x, F y) {
    return x >= y ? x - y : x + p - y;
}

F mul(F x, F y) {
    std::uint64_t z = std::uint64_t(x) * y;
    z = (z & p) + (z >> 31);
    return z >= p ? z - p : z;
}

F pow(F x, F n) {
    F out = 1;
    for (; n; n >>= 1, x = mul(x, x)) if (n & 1) out = mul(out, x);
    return out;
}

F rhs(F x) {
    return add(mul(x, add(mul(x, x), a)), b);
}

Forms lift(F r, F s) {
    assert(r < p && s < p && mul(s, s) == rhs(r));
    Forms forms{};
    for (std::size_t i = 0; i < xs.size(); ++i) {
        F x = xs[i];
        F delta = sub(x, r);
        F y = pow(rhs(x), (p + 1u) / 4);
        assert(delta != 0 && y != 0 && mul(y, y) == rhs(x));
        y = std::min(y, p - y);
        for (int sign = 0; sign < 2; ++sign) {
            F c = sign ? sub(s, y) : add(s, y);
            Row& f = forms[i][sign];
            F xp = 1;
            for (int j = 0; j < 4; ++j, xp = mul(xp, x)) {
                f[j] = mul(delta, xp);
                if (j < 3) f[4 + j] = mul(c, xp);
                else f[7] = mul(c, xp);
            }
            f[8 + i] = 1;
        }
    }
    return forms;
}

struct Leaf {
    std::uint8_t depth, mask;
    std::array<F, 8> combination;
};

struct Result {
    std::vector<Leaf> leaves;
    std::array<F, 7> values{};
    std::uint64_t inserted = 0, early = 0, unresolved = 0;
    unsigned mask = 0;
    bool positive = false;
};

struct Ladder {
    std::array<Row, 7> pivots{};
    unsigned rank = 0;

    // Returns the normalized affine contradiction when the new equation is inconsistent.
    bool insert(Row& row) {
        for (unsigned col = 0; col < 7; ++col) {
            F entry = row[col];
            if (entry == 0) continue;
            const Row& pivot = pivots[col];
            if (pivot[col] == 0) {
                F inv = pow(entry, p - 2);
                for (auto& coefficient : row) coefficient = mul(coefficient, inv);
                pivots[col] = row;
                ++rank;
                return false;
            }
            for (unsigned k = col; k < row.size(); ++k)
                row[k] = sub(row[k], mul(entry, pivot[k]));
        }
        if (row[7] == 0) return false;
        F inv = pow(row[7], p - 2);
        for (auto& coefficient : row) coefficient = mul(coefficient, inv);
        return true;
    }

    std::array<F, 7> point() const {
        assert(rank == 7);
        std::array<F, 7> solution{};
        for (int col = 6; col >= 0; --col) {
            F value = sub(0, pivots[col][7]);
            for (unsigned k = col + 1; k < solution.size(); ++k)
                value = sub(value, mul(pivots[col][k], solution[k]));
            solution[col] = value;
        }
        return solution;
    }
};

void search(const Forms& forms, const Ladder& prefix, unsigned depth, unsigned mask, Result& result) {
    if (depth == 8) {
        if (prefix.rank == 7) {
            result.values = prefix.point();
            result.mask = mask;
            result.positive = true;
        } else ++result.unresolved;
        return;
    }
    for (unsigned bit = 0; bit < 2 && !result.positive; ++bit) {
        Ladder child = prefix;
        Row row = forms[depth][bit];
        ++result.inserted;
        unsigned extended = mask | (bit << depth);
        if (child.insert(row)) {
            Leaf leaf{static_cast<std::uint8_t>(depth + 1), static_cast<std::uint8_t>(extended), {}};
            std::copy(row.begin() + 8, row.end(), leaf.combination.begin());
            result.leaves.push_back(leaf);
            result.early += (depth + 1 < 8);
        } else search(forms, child, depth + 1, extended, result);
    }
}

void word(std::ostream& out, F value) {
    for (int i = 0; i < 4; ++i, value >>= 8) out.put(static_cast<char>(value & 255));
}

int main(int argc, char** argv) {
    assert(argc == 5);
    auto r = std::stoul(argv[1]), s = std::stoul(argv[2]);
    assert(r < p && s < p);
    unsigned runs = std::stoul(argv[3]);
    assert(runs > 0);
    std::vector<double> generating, cold;
    Result output;
    for (unsigned n = 0; n < runs; ++n) {
        using Clock = std::chrono::steady_clock;
        auto start = Clock::now();
        auto forms = lift(r, s);
        auto lifted = Clock::now();
        Result current;
        current.leaves.reserve(256);
        search(forms, Ladder{}, 0, 0, current);
        auto done = Clock::now();
        generating.push_back(std::chrono::duration<double, std::micro>(done - lifted).count());
        cold.push_back(std::chrono::duration<double, std::micro>(done - start).count());
        output = std::move(current);
    }
    assert(output.unresolved == 0 || output.positive);
    std::sort(cold.begin(), cold.end());
    std::sort(generating.begin(), generating.end());
    std::ofstream out(argv[4], std::ios::binary);
    out.write("RRB1", 4);
    out.put(static_cast<char>(output.positive));
    if (output.positive) {
        out.put(static_cast<char>(output.mask));
        for (F value : output.values) word(out, value);
    } else {
        word(out, output.leaves.size());
        for (const auto& leaf : output.leaves) {
            out.put(static_cast<char>(leaf.depth));
            out.put(static_cast<char>(leaf.mask));
            for (F coefficient : leaf.combination) word(out, coefficient);
        }
    }
    auto bytes = out.tellp();
    out.close();
    if (!out) return 1;
    std::cout << "positive=" << output.positive << " bytes=" << bytes
              << " inserted=" << output.inserted << " refutations=" << output.leaves.size()
              << " early=" << output.early << " unresolved=" << output.unresolved
              << " lift_search_median_us=" << cold[runs/2]
              << " search_median_us=" << generating[runs/2] << '\n';
}
