#include <gmpxx.h>

#include <array>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

using Q = mpq_class;
using Mat = std::vector<std::vector<Q>>;
struct Slot { int x; int y; };
std::ofstream checks;
unsigned assertions = 0;

void require(bool condition, const std::string& name, const std::string& detail) {
    checks << name << '\t' << (condition ? "PASS" : "FAIL") << '\t' << detail << '\n';
    checks.flush();
    ++assertions;
    if (!condition) throw std::runtime_error(name + ": " + detail);
}

int syndrome(int mask) {
    constexpr std::array<int, 3> columns{1, 2, 3};
    int result = 0;
    for (int j = 0; j < 3; ++j) if ((mask & (1 << j)) != 0) result ^= columns[j];
    return result;
}

int order2(const Q& value) {
    if (value == 0) return 1000000;
    mpz_class num = value.get_num();
    if (num < 0) num = -num;
    const auto vn = mpz_scan1(num.get_mpz_t(), 0);
    const auto vd = mpz_scan1(value.get_den().get_mpz_t(), 0);
    return static_cast<int>(vn) - static_cast<int>(vd);
}

Q cardinal(int q, int t) {
    Q value(1);
    for (int s = -8; s < 8; ++s) {
        if (s == q) continue;
        Q factor(t - s, q - s);
        factor.canonicalize();
        value *= factor;
    }
    value.canonicalize();
    return value;
}

std::array<Q, 4> selectors(int t, std::ofstream& table) {
    std::array<Q, 4> f{Q(0), Q(0), Q(0), Q(0)};
    for (int q = -8; q < 8; ++q) {
        const Q value = cardinal(q, t);
        int label = 0;
        if (q == -8) label = 1;
        else if (q != 0) {
            int absolute = q < 0 ? -q : q;
            int j = 0;
            while ((absolute & 1) == 0) { ++j; absolute /= 2; }
            label = j + 1;
        }
        f[static_cast<std::size_t>(label)] += value;
        table << "cardinal\t" << t << '\t' << q << '\t' << label << '\t'
              << value.get_num() << '\t' << value.get_den() << '\t' << order2(value) << '\n';
    }
    Q sum(0);
    for (std::size_t j = 0; j < f.size(); ++j) {
        sum += f[j];
        table << "selector\t" << t << '\t' << j << '\t' << j << '\t'
              << f[j].get_num() << '\t' << f[j].get_den() << '\t' << order2(f[j]) << '\n';
    }
    require(sum == 1, "selector_sum_" + std::to_string(t), sum.get_str());
    return f;
}

Mat single_operator(const std::array<Q, 4>& f) {
    Mat a(8, std::vector<Q>(8, Q(0)));
    for (int x = 0; x < 8; ++x) {
        a[static_cast<std::size_t>(x)][static_cast<std::size_t>(x)] += f[0];
        for (int j = 0; j < 3; ++j) {
            a[static_cast<std::size_t>(x ^ (1 << j))][static_cast<std::size_t>(x)] +=
                f[static_cast<std::size_t>(j + 1)];
        }
    }
    return a;
}

Mat projected(const Mat& a, const std::vector<Slot>& slots) {
    Mat result(slots.size(), std::vector<Q>(slots.size(), Q(0)));
    for (std::size_t row = 0; row < slots.size(); ++row) {
        for (std::size_t col = 0; col < slots.size(); ++col) {
            result[row][col] = a[static_cast<std::size_t>(slots[row].x)][static_cast<std::size_t>(slots[col].x)] *
                               a[static_cast<std::size_t>(slots[row].y)][static_cast<std::size_t>(slots[col].y)];
        }
    }
    return result;
}

Mat diagonal_formula(const std::array<Q, 4>& f, const std::vector<Slot>& slots) {
    Mat result(slots.size(), std::vector<Q>(slots.size(), Q(0)));
    for (std::size_t row = 0; row < slots.size(); ++row) {
        for (std::size_t col = 0; col < slots.size(); ++col) {
            if (row == col) result[row][col] += f[0] * f[0];
            for (int j = 0; j < 3; ++j) {
                const int flip = 1 << j;
                if (slots[row].x == (slots[col].x ^ flip) && slots[row].y == (slots[col].y ^ flip)) {
                    const auto label = static_cast<std::size_t>(j + 1);
                    result[row][col] += f[label] * f[label];
                }
            }
        }
    }
    return result;
}

int residue2(const Q& value) {
    if (mpz_even_p(value.get_den().get_mpz_t()) != 0)
        throw std::runtime_error("non-integral2-adic entry: " + value.get_str());
    return mpz_odd_p(value.get_num().get_mpz_t()) != 0 ? 1 : 0;
}

int binary_rank(std::vector<std::vector<int>> matrix) {
    const auto n = matrix.size();
    std::size_t pivot_row = 0;
    for (std::size_t col = 0; col < n && pivot_row < n; ++col) {
        std::size_t chosen = pivot_row;
        while (chosen < n && matrix[chosen][col] == 0) ++chosen;
        if (chosen == n) continue;
        std::swap(matrix[chosen], matrix[pivot_row]);
        for (std::size_t row = 0; row < n; ++row) {
            if (row == pivot_row || matrix[row][col] == 0) continue;
            for (std::size_t j = col; j < n; ++j) matrix[row][j] ^= matrix[pivot_row][j];
        }
        ++pivot_row;
    }
    return static_cast<int>(pivot_row);
}

Q determinant(Mat matrix) {
    Q det(1);
    const auto n = matrix.size();
    for (std::size_t col = 0; col < n; ++col) {
        std::size_t pivot = col;
        while (pivot < n && matrix[pivot][col] == 0) ++pivot;
        if (pivot == n) return Q(0);
        if (pivot != col) { std::swap(matrix[pivot], matrix[col]); det = -det; }
        const Q scalar = matrix[col][col];
        det *= scalar;
        for (std::size_t row = col + 1; row < n; ++row) {
            const Q ratio = matrix[row][col] / scalar;
            for (std::size_t j = col + 1; j < n; ++j) matrix[row][j] -= ratio * matrix[col][j];
        }
    }
    det.canonicalize();
    return det;
}

int main() {
    try {
        checks.open("checks.tsv");
        checks << "check\tstatus\tdetail\n";
        std::ofstream slot_table("slots.tsv");
        slot_table << "index\tx\ty\tsyndrome\tside_bit0\n";
        std::vector<Slot> slots;
        std::array<int, 8> first_counts{};
        for (int x = 0; x < 8; ++x) for (int y = 0; y < 8; ++y) {
            if (syndrome(x ^ y) != 0) continue;
            slot_table << slots.size() << '\t' << x << '\t' << y << '\t' << syndrome(x ^ y) << '\t' << (x & 1) << '\n';
            slots.push_back(Slot{x, y});
            ++first_counts[static_cast<std::size_t>(x)];
        }
        require(slots.size() == 16, "code_size", std::to_string(slots.size()));
        for (std::size_t x = 0; x < first_counts.size(); ++x)
            require(first_counts[x] == 2, "first_orientation_" + std::to_string(x), std::to_string(first_counts[x]));

        std::ofstream selector_table("selectors.tsv");
        selector_table << "kind\tT\tq_or_label\tlabel\tnumerator\tdenominator\tv2_or_zero_sentinel\n";
        const auto at1 = selectors(1, selector_table);
        const auto at17 = selectors(17, selector_table);
        require(at1[0] == 0 && at1[1] == 1 && at1[2] == 0 && at1[3] == 0,
                "active_node_selector", "0,1,0,0");
        require(order2(at17[0]) == 4, "fstar_order", std::to_string(order2(at17[0])));
        require(order2(at17[2]) >= 5, "f1_order", std::to_string(order2(at17[2])));
        require(order2(at17[3]) >= 5, "f2_order", std::to_string(order2(at17[3])));

        const std::array<std::array<Q, 4>, 4> fixtures{{
            {Q(1), Q(2), Q(3), Q(4)},
            {Q(1, 3), Q(-2, 5), Q(3, 7), Q(-4, 9)},
            {Q(0), Q(-1, 2), Q(2, 3), Q(-3, 4)},
            at17
        }};
        std::ofstream matrix_table("projection-matrices.tsv");
        matrix_table << "fixture\trow\tcol\tdirect\tformula\n";
        for (std::size_t fixture = 0; fixture < fixtures.size(); ++fixture) {
            const Mat direct = projected(single_operator(fixtures[fixture]), slots);
            const Mat formula = diagonal_formula(fixtures[fixture], slots);
            unsigned mismatches = 0;
            for (std::size_t row = 0; row < slots.size(); ++row) for (std::size_t col = 0; col < slots.size(); ++col) {
                matrix_table << fixture << '\t' << row << '\t' << col << '\t' << direct[row][col] << '\t' << formula[row][col] << '\n';
                if (direct[row][col] != formula[row][col]) ++mismatches;
            }
            require(mismatches == 0, "projected_tensor_fixture_" + std::to_string(fixture),
                    "256entries,mismatches=" + std::to_string(mismatches));
        }

        const Mat node_matrix = projected(single_operator(at1), slots);
        unsigned node_mismatches = 0;
        unsigned wrong_node_mismatches = 0;
        for (std::size_t row = 0; row < slots.size(); ++row) for (std::size_t col = 0; col < slots.size(); ++col) {
            const Q expected(slots[row].x == (slots[col].x ^ 1) && slots[row].y == (slots[col].y ^ 1) ? 1 : 0);
            if (node_matrix[row][col] != expected) ++node_mismatches;
            if (node_matrix[row][col] != Q(row == col ? 1 : 0)) ++wrong_node_mismatches;
        }
        require(node_mismatches == 0, "active_node_U0", "256entries,mismatches=" + std::to_string(node_mismatches));
        require(wrong_node_mismatches != 0, "negative_replace_U0_with_I", std::to_string(wrong_node_mismatches));

        const Mat full = projected(single_operator(at17), slots);
        std::ofstream cross_table("crossing-matrices.tsv");
        cross_table << "side\trow\tcol\tB\tB_div_256\tmod2\n";
        for (int side = 0; side < 2; ++side) {
            std::vector<std::size_t> indices;
            for (std::size_t i = 0; i < slots.size(); ++i) if ((slots[i].x & 1) == side) indices.push_back(i);
            require(indices.size() == 8, "cross_side_size_" + std::to_string(side), std::to_string(indices.size()));
            Mat b(8, std::vector<Q>(8, Q(0)));
            std::vector<std::vector<int>> mod2(8, std::vector<int>(8, 0));
            std::vector<std::vector<int>> without_identity(8, std::vector<int>(8, 0));
            unsigned mismatches = 0;
            int minimum_order = 1000000;
            for (std::size_t row = 0; row < 8; ++row) for (std::size_t col = 0; col < 8; ++col) {
                b[row][col] = full[indices[row]][indices[col]];
                const Q divided = b[row][col] / 256;
                const int bit = residue2(divided);
                mod2[row][col] = bit;
                if (bit != (row == col ? 1 : 0)) ++mismatches;
                const int entry_order = order2(b[row][col]);
                if (entry_order < minimum_order) minimum_order = entry_order;
                Q omitted = b[row][col];
                if (row == col) omitted -= at17[0] * at17[0];
                without_identity[row][col] = residue2(omitted / 256);
                cross_table << side << '\t' << row << '\t' << col << '\t' << b[row][col] << '\t' << divided << '\t' << bit << '\n';
            }
            const Q det = determinant(b);
            const std::string suffix = "_side_" + std::to_string(side);
            require(mismatches == 0, "scaled_cross_mod2_I" + suffix, "64entries,mismatches=" + std::to_string(mismatches));
            require(binary_rank(mod2) == 8, "scaled_cross_rank" + suffix, std::to_string(binary_rank(mod2)));
            require(minimum_order == 8, "cross_minimum_order" + suffix, std::to_string(minimum_order));
            require(order2(det) == 64, "cross_determinant_order" + suffix,
                    std::to_string(order2(det)) + ";det=" + det.get_str());
            require(binary_rank(without_identity) == 0, "negative_omit_identity" + suffix,
                    "normalizedrank=" + std::to_string(binary_rank(without_identity)));
        }
        std::cout << "PASS assertions=" << assertions << " code_slots=16 first_orientation_multiplicity=2 fixtures=4"
                  << " projection_entries=1024 active_node_entries=256 crossing_entries=128"
                  << " normalized_cross_ranks=8,8 determinant_orders=64,64 smith_orders_each_side=8x8\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "FAIL " << error.what() << '\n';
        return 1;
    }
}
