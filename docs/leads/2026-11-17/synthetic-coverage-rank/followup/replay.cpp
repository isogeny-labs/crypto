#include <algorithm>
#include <array>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <numeric>
#include <set>
#include <stdexcept>
#include <string_view>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        throw std::runtime_error(message);
    }
}

struct Fraction {
    std::int64_t n;
    std::int64_t d;

    Fraction(std::int64_t numerator = 0, std::int64_t denominator = 1) {
        require(denominator > 0, "invalid denominator");
        auto divisor = std::gcd(numerator, denominator);
        n = numerator / divisor;
        d = denominator / divisor;
    }

    bool operator==(Fraction rhs) const { return n == rhs.n && d == rhs.d; }
};

Fraction operator+(Fraction a, Fraction b) {
    return {a.n * b.d + b.n * a.d, a.d * b.d};
}

Fraction operator-(Fraction a, Fraction b) {
    return a + Fraction{-b.n, b.d};
}

Fraction operator*(Fraction a, Fraction b) {
    return {a.n * b.n, a.d * b.d};
}

Fraction operator/(Fraction a, Fraction b) {
    return {a.n * b.d, a.d * b.n};
}

std::ostream& operator<<(std::ostream& stream, Fraction x) {
    return stream << x.n << '/' << x.d;
}

struct Moments {
    Fraction mean;
    Fraction second;

    void add(Fraction x, Fraction probability) {
        mean = mean + probability * x;
        second = second + probability * x * x;
    }

    Fraction variance() const { return second - mean * mean; }
};

constexpr int group_size = 19;
constexpr int base_size = 6;
constexpr int tuple_count = base_size * base_size;
using Base = std::array<int, base_size>;
using Hits = std::array<bool, base_size>;

enum class Gate { raw, first_three, all_three_subsets };

int residue(int x, int field) {
    return (x % field + field) % field;
}

void add_gate(Moments& uniform, Moments& plant, bool accepted, int multiplicity,
              Fraction gate_probability) {
    uniform.add(accepted, gate_probability / group_size);
    if (multiplicity) {
        const Fraction score = accepted ? Fraction{tuple_count, group_size * multiplicity} : 0;
        plant.add(score, gate_probability * Fraction{multiplicity, tuple_count});
    }
}

void check_coverage(const Base& base, char label, Gate gate) {
    std::array<int, group_size> tally{};
    for (int a : base) {
        for (int b : base) {
            ++tally[residue(a + b, group_size)];
        }
    }

    Moments uniform;
    Moments plant;
    int uniform_probes = 0;
    for (int target = 0; target < group_size; ++target) {
        Hits hits{};
        int count = 0;
        int first = base_size;
        for (int i = 0; i < base_size; ++i) {
            int complement = residue(target - base[i], group_size);
            for (int b : base) {
                hits[i] = hits[i] || b == complement;
            }
            if (hits[i]) {
                ++count;
                first = std::min(first, i + 1);
            }
        }
        require(count == tally[target], "pair tally and complement count disagree");
        switch (gate) {
        case Gate::raw:
            uniform_probes += first;
            add_gate(uniform, plant, count > 0, count, 1);
            break;
        case Gate::first_three:
            add_gate(uniform, plant, hits[0] || hits[1] || hits[2], count, 1);
            break;
        case Gate::all_three_subsets:
            for (int i = 0; i < base_size; ++i) {
                for (int j = i + 1; j < base_size; ++j) {
                    for (int k = j + 1; k < base_size; ++k) {
                        add_gate(uniform, plant, hits[i] || hits[j] || hits[k],
                                 count, {1, 20});
                    }
                }
            }
            break;
        }
    }
    require(uniform.mean == plant.mean, "inverse weighting changed the mean");
    std::cout << label << ' ';
    switch (gate) {
    case Gate::raw: {
        const Fraction cost{uniform_probes, group_size};
        require(uniform.mean == (label == 'A' ? Fraction{15, 19} : Fraction{1}),
                "raw coverage mismatch");
        std::cout << "raw mean=" << uniform.mean << " probes_per_draw=" << cost
                  << ",6/1 var_times_expected_probes="
                  << uniform.variance() * cost << ',' << plant.variance() * 6 << '\n';
        break;
    }
    case Gate::first_three:
        require(uniform.mean == Fraction{13, 19}, "ascending gate mean mismatch");
        std::cout << "first_three";
        break;
    case Gate::all_three_subsets:
        require(uniform.mean == (label == 'A' ? Fraction{129, 190} : Fraction{68, 95}),
                "all subsets gate mean mismatch");
        std::cout << "all_three_subsets";
        break;
    }
    if (gate != Gate::raw) {
        const Fraction uniform_block = uniform.variance() / 2;
        const Fraction plant_block = plant.variance();
        std::cout << " mean=" << uniform.mean << " var_at_6_probes="
                  << uniform_block << ',' << plant_block
                  << " plant_over_uniform=" << plant_block / uniform_block << '\n';
    }
}

void coverage() {
    const Base a{0, 1, 2, 4, 7, 12};
    const Base b{0, 1, 3, 12, 14, 15};
    for (auto gate : {Gate::raw, Gate::first_three, Gate::all_three_subsets}) {
        check_coverage(a, 'A', gate);
        check_coverage(b, 'B', gate);
    }
}

using Row = std::array<int, 4>;
using Span = std::set<Row>;
using Fixture = std::array<Row, 4>;

void add_row(Span& span, const Row& row, int field) {
    Span next;
    for (const auto& point : span) {
        for (int coefficient = 0; coefficient < field; ++coefficient) {
            Row result{};
            for (std::size_t j = 0; j < result.size(); ++j) {
                result[j] = residue(point[j] + coefficient * row[j], field);
            }
            next.insert(result);
        }
    }
    span.swap(next);
}

int dimension(std::size_t count, unsigned field) {
    int result = 0;
    while (count > 1 && count % field == 0) {
        count /= field;
        ++result;
    }
    require(count == 1, "span size is not a power of field order");
    return result;
}

int primary_rank(const Span& span, unsigned field) {
    std::size_t primary = 0;
    for (auto row : span) {
        primary += row[2] == 0 && row[3] == 0;
    }
    return dimension(primary, field);
}

void rank_stream(int field, const Fixture& a, const Fixture& b) {
    constexpr std::array<int, 8> costs{2, 3, 1, 4, 2, 3, 1, 5};
    constexpr std::array<int, 8> events{-1, 0, 0, 1, -1, 2, 2, 3};
    constexpr std::array<int, 8> full_rank{0, 1, 1, 2, 2, 3, 3, 4};
    constexpr std::array<int, 8> expected_a{0, 0, 0, 1, 1, 2, 2, 2};
    constexpr std::array<int, 8> expected_b{0, 0, 0, 0, 0, 1, 1, 2};
    Span span_a{Row{}};
    Span span_b{Row{}};
    std::array<bool, 4> coverage_a{};
    std::array<bool, 4> coverage_b{};
    int work = 0;
    int accepted = 0;
    int threshold_a = 0;
    int threshold_b = 0;
    for (std::size_t i = 0; i < events.size(); ++i) {
        work += costs[i];
        const int event = events[i];
        if (event >= 0) {
            ++accepted;
            add_row(span_a, a[event], field);
            add_row(span_b, b[event], field);
            for (std::size_t j = 0; j < coverage_a.size(); ++j) {
                coverage_a[j] = coverage_a[j] || residue(a[event][j], field) != 0;
                coverage_b[j] = coverage_b[j] || residue(b[event][j], field) != 0;
            }
        }
        require(coverage_a == coverage_b, "column coverage differs");
        const int columns = std::accumulate(coverage_a.begin(), coverage_a.end(), 0);
        const int rank_a = dimension(span_a.size(), field);
        const int rank_b = dimension(span_b.size(), field);
        const int out_a = primary_rank(span_a, field);
        const int out_b = primary_rank(span_b, field);
        require(rank_a == rank_b && rank_a == (field == 2 ? (i != 0) : full_rank[i]),
                "total rank differs");
        require(out_a == (field == 2 ? 0 : expected_a[i]) &&
                out_b == (field == 2 ? 0 : expected_b[i]), "primary rank differs");
        if (!threshold_a && out_a == 2) {
            threshold_a = work;
        }
        if (!threshold_b && out_b == 2) {
            threshold_b = work;
        }
        std::cout << "field=" << field << " attempt=" << i + 1 << " work=" << work
                  << " accepted=" << accepted << " columns=" << columns
                  << " total_rank=" << rank_a << " primary_rank=" << out_a
                  << ',' << out_b << '\n';
    }
    require(field == 2 ? threshold_a == 0 && threshold_b == 0
                       : threshold_a == 15 && threshold_b == 21, "wrong threshold");
    std::cout << "field=" << field << " threshold_work=";
    if (field == 2) {
        std::cout << "unreached\n";
    } else {
        std::cout << threshold_a << ',' << threshold_b << '\n';
    }
}

void rank() {
    const Fixture a{{{1, 1, -1, -1}, {-1, 1, 1, 1}, {1, -1, 1, 1}, {1, 1, -1, 1}}};
    const Fixture b{{{-1, 1, -1, 1}, {1, -1, 1, 1}, {1, 1, 1, -1}, {1, 1, -1, 1}}};
    for (int field : {2, 3, 5}) {
        rank_stream(field, a, b);
    }
}

}  // namespace

int main(int argc, char** argv) {
    try {
        if (argc == 2 && std::string_view(argv[1]) == "coverage") {
            coverage();
        } else if (argc == 2 && std::string_view(argv[1]) == "rank") {
            rank();
        } else {
            std::cerr << "usage: replay coverage|rank\n";
            return EXIT_FAILURE;
        }
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return EXIT_FAILURE;
    }
}
