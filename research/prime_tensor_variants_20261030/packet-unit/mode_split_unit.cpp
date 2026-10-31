// Fixed local coefficient-enumeration regression, not a contraction algorithm.
#include <array>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <limits>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

using I = std::int64_t;
using Matrix = std::array<std::array<I, 3>, 3>;
using Bits = std::array<int, 3>;

I add(I a, I b) {
    if ((b > 0 && a > std::numeric_limits<I>::max() - b)
        || (b < 0 && a < std::numeric_limits<I>::min() - b))
        throw std::runtime_error("integer addition overflow");
    return a + b;
}
I multiply(I a, I b) {
    if (a > 0) {
        if ((b > 0 && a > std::numeric_limits<I>::max() / b)
            || (b < 0 && b < std::numeric_limits<I>::min() / a))
            throw std::runtime_error("integer multiplication overflow");
    } else if (a < 0) {
        if ((b > 0 && a < std::numeric_limits<I>::min() / b)
            || (b < 0 && a < std::numeric_limits<I>::max() / b))
            throw std::runtime_error("integer multiplication overflow");
    }
    return a * b;
}
I residue(I value, I modulus) {
    const I r = value % modulus; return r < 0 ? r + modulus : r;
}
int valuation3(I value) {
    if (value == 0) throw std::runtime_error("finite valuation requested at zero");
    if (value < 0) value = -value;
    int result = 0; while (value % 3 == 0) { ++result; value /= 3; } return result;
}
void require(bool value, const std::string& message) {
    if (!value) throw std::runtime_error(message);
}
Bits bits(int mask) {
    return Bits{mask & 1, (mask >> 1) & 1, (mask >> 2) & 1};
}
struct Row {
    I constant;
    std::vector<std::pair<int, I>> crossing;
};
std::array<Row, 3> rows(const Matrix& W, const Bits& X, const Bits& Y,
                       const Bits& row_group, const Bits& column_group) {
    std::array<Row, 3> result;
    for (int x = 0; x < 3; ++x) {
        I local = 0;
        for (int y = 0; y < 3; ++y)
            if (row_group[static_cast<std::size_t>(x)] == column_group[static_cast<std::size_t>(y)])
                local = add(local, multiply(W[static_cast<std::size_t>(x)][static_cast<std::size_t>(y)],
                                            X[static_cast<std::size_t>(y)]));
        const I sign = Y[static_cast<std::size_t>(x)] == 1 ? 1 : -1;
        result[static_cast<std::size_t>(x)].constant =
            Y[static_cast<std::size_t>(x)] == 1 ? local : add(1, -local);
        for (int y = 0; y < 3; ++y)
            if (row_group[static_cast<std::size_t>(x)] != column_group[static_cast<std::size_t>(y)])
                result[static_cast<std::size_t>(x)].crossing.push_back(
                    {y, multiply(sign, W[static_cast<std::size_t>(x)][static_cast<std::size_t>(y)])});
        require(result[static_cast<std::size_t>(x)].crossing.size() == 2,
                "fixed packet does not have two crossing sources per row");
    }
    return result;
}
I direct(const Matrix& W, const Bits& X, const Bits& Y) {
    I value = 1;
    for (int x = 0; x < 3; ++x) {
        I row = 0;
        for (int y = 0; y < 3; ++y)
            row = add(row, multiply(W[static_cast<std::size_t>(x)][static_cast<std::size_t>(y)],
                                    X[static_cast<std::size_t>(y)]));
        const I factor = Y[static_cast<std::size_t>(x)] == 1 ? row : add(1, -row);
        value = multiply(value, factor);
    }
    return value;
}
I split(const std::array<Row, 3>& row, const Bits& X, int precision) {
    // A mode is identified by (source column, target row group). Each selected
    // target group has one row, so one affine term choice per row enumerates
    // ordinary distinct-mode coefficients exactly.
    I sum = 0;
    for (int a = 0; a < 3; ++a) for (int b = 0; b < 3; ++b) for (int c = 0; c < 3; ++c) {
        const std::array<int, 3> choices{a, b, c};
        int weight = 0; I coefficient = 1, source_value = 1;
        for (int x = 0; x < 3; ++x) {
            const int choice = choices[static_cast<std::size_t>(x)];
            const Row& r = row[static_cast<std::size_t>(x)];
            if (choice == 0) coefficient = multiply(coefficient, r.constant);
            else {
                ++weight;
                const auto term = r.crossing[static_cast<std::size_t>(choice - 1)];
                coefficient = multiply(coefficient, term.second);
                source_value = multiply(source_value, X[static_cast<std::size_t>(term.first)]);
            }
        }
        if (weight < precision) sum = add(sum, multiply(coefficient, source_value));
    }
    return sum;
}
I wrongly_unsplit(const std::array<Row, 3>& row, const Bits& X, int precision) {
    // Intentionally wrong: a single source-column exponent is imposed at BOTH
    // crossed target-group coefficients, while the source linear form is
    // evaluated only at that one exponent. This violates the one-target premise.
    I sum = 0;
    for (int a = 0; a < precision; ++a) for (int b = 0; b < precision; ++b)
        for (int c = 0; c < precision; ++c) {
            const Bits exponent{a, b, c};
            if (a + b + c >= precision) continue;
            I coefficient = 1, source = 1;
            for (int x = 0; x < 3; ++x) {
                const Row& r = row[static_cast<std::size_t>(x)];
                int targeted_degree = 0; I linear = 0;
                for (const auto& term : r.crossing) {
                    const int power = exponent[static_cast<std::size_t>(term.first)];
                    targeted_degree += power;
                    if (power == 1) linear = term.second;
                }
                const I row_coefficient = targeted_degree == 0 ? r.constant
                                        : targeted_degree == 1 ? linear : 0;
                coefficient = multiply(coefficient, row_coefficient);
            }
            for (int y = 0; y < 3; ++y)
                if (exponent[static_cast<std::size_t>(y)] > 0)
                    source = multiply(source, X[static_cast<std::size_t>(y)]);
            sum = add(sum, multiply(coefficient, source));
        }
    return sum;
}

int main(int argc, char** argv) {
    try {
        if (argc != 2) throw std::runtime_error("usage: mode_split_unit OUTPUT_DIRECTORY");
        const std::filesystem::path path(argv[1]); std::filesystem::create_directories(path);
        std::ofstream matrix(path / "matrix.tsv"), cases(path / "cases.tsv"),
                      witness(path / "witness.tsv"), summary(path / "summary.json");
        require(matrix && cases && witness && summary, "cannot open mode records");
        matrix << "x\ty\trow_group\tcolumn_group\tW_xy\tcrossing\tv3_W\tstatus\n";
        cases << "X_mask\tY_mask\tp\tdirect_integer\tsplit_integer\twrong_unsplit_integer\tdirect_residue\tsplit_residue\twrong_unsplit_residue\tsplit_equal\twrong_unsplit_equal\tstatus\n";
        witness << "case\tobservation\tstatus\n";
        Bits row_group{}, column_group{};
        int first = 0, second = 1;
        for (int x = 0; x < 3; ++x) {
            row_group[static_cast<std::size_t>(x)] = first;
            column_group[static_cast<std::size_t>(x)] = second;
            const int next = static_cast<int>(residue(-first - second, 5));
            second = first; first = next;
        }
        require(row_group == Bits{0, 4, 1} && column_group == Bits{1, 0, 4},
                "generated packet groups differ from frozen witness");
        I C = 1, derivative_zero = 1;
        for (int q = -3; q <= 5; ++q) {
            C = multiply(C, 10 - q);
            if (q != 0) derivative_zero = multiply(derivative_zero, -q);
        }
        require(derivative_zero == -720, "Cprime(0) mismatch");
        Matrix W{}; int crossed = 0;
        for (int x = 0; x < 3; ++x) for (int y = 0; y < 3; ++y) {
            const I denominator = multiply(derivative_zero, 10 - (y - x));
            require(C % denominator == 0, "Cauchy entry not exact integer");
            const I value = C / denominator;
            W[static_cast<std::size_t>(x)][static_cast<std::size_t>(y)] = value;
            const bool crossing = row_group[static_cast<std::size_t>(x)]
                               != column_group[static_cast<std::size_t>(y)];
            if (crossing) { require(valuation3(value) >= 1, "cross coefficient not 3-divisible"); ++crossed; }
            matrix << x << '\t' << y << '\t' << row_group[static_cast<std::size_t>(x)] << '\t'
                   << column_group[static_cast<std::size_t>(y)] << '\t' << value << '\t'
                   << (crossing ? 1 : 0) << '\t' << valuation3(value) << "\tPASS\n";
        }
        require(W[0][0] == -36036 && W[1][0] == -32760 && W[2][0] == -30030 && crossed == 6,
                "fixed Cauchy column or crossing count mismatch");
        int number = 0; std::array<int, 3> mismatches{};
        for (int Xmask = 0; Xmask < 8; ++Xmask) for (int Ymask = 0; Ymask < 8; ++Ymask) {
            const Bits X = bits(Xmask), Y = bits(Ymask);
            const auto row = rows(W, X, Y, row_group, column_group);
            const I answer = direct(W, X, Y);
            I modulus = 1;
            for (int p = 1; p <= 3; ++p) {
                modulus *= 3; ++number;
                const I split_answer = split(row, X, p);
                const I bad_answer = wrongly_unsplit(row, X, p);
                const I a = residue(answer, modulus), b = residue(split_answer, modulus),
                        bad = residue(bad_answer, modulus);
                require(a == b, "correct target-split coefficient identity failed");
                if (a != bad) ++mismatches[static_cast<std::size_t>(p - 1)];
                cases << Xmask << '\t' << Ymask << '\t' << p << '\t' << answer << '\t'
                      << split_answer << '\t' << bad_answer << '\t' << a << '\t' << b << '\t'
                      << bad << "\ttrue\t" << (a == bad ? "true" : "false") << "\tPASS\n";
                if (Xmask == 1 && Ymask == 1 && p == 3) {
                    require(a == 9 && b == 9 && bad == 0, "frozen9-versus0 regression not reproduced");
                    const I surviving_term = multiply(W[0][0], add(1, -W[2][0]));
                    require(residue(surviving_term, 27) == 9, "split degree-one witness term mismatch");
                    witness << "X100,Y100,p3\tdirect=" << a << ",split=" << b
                            << ",wrong_unsplit=" << bad << ";surviving_split_term=9\tPASS\n";
                }
            }
        }
        require(number == 192, "fixed pin count mismatch");
        summary << "{\n  \"schema\": \"local-ternary-mode-split-development/v1\",\n"
                << "  \"pin_assignments\": 64,\n  \"precisions\": [1,2,3],\n  \"case_rows\": " << number
                << ",\n  \"correct_split_equal\": " << number
                << ",\n  \"wrong_unsplit_mismatches_by_precision\": ["
                << mismatches[0] << ',' << mismatches[1] << ',' << mismatches[2]
                << "],\n  \"frozen_witness_direct_mod27\": 9,\n"
                << "  \"frozen_witness_split_mod27\": 9,\n"
                << "  \"frozen_witness_wrong_unsplit_mod27\": 0,\n"
                << "  \"full_contraction_or_precision_theorem\": false,\n  \"status\": \"PASS\"\n}\n";
        for (auto* stream : {&matrix, &cases, &witness, &summary}) {
            stream->flush(); require(static_cast<bool>(*stream), "mode record write failed");
            stream->close(); require(!stream->fail(), "mode record close failed");
        }
        std::cout << "PASS local mode-split cases=192 correct_equal=192 wrong_mismatches="
                  << mismatches[0] << ',' << mismatches[1] << ',' << mismatches[2] << '\n';
        std::cout << "PASS fixed pin witness direct=9 split=9 wrong_unsplit=0 mod27; local identity only\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "FAIL " << error.what() << '\n'; return 1;
    }
}
