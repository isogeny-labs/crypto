#include <algorithm>
#include <array>
#include <bitset>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <stdexcept>

namespace {

constexpr int group_size = 17;
constexpr int base_size = 5;
constexpr int tuple_count = base_size * base_size;
constexpr int gate_count = base_size * (base_size - 1) / 2;
constexpr int uniform_draws = 5;
constexpr int planted_draws = 2;
constexpr int probe_budget = 10;
constexpr int inverse_scale = 60;  // Divisible by each possible positive multiplicity.
constexpr int score_scale = group_size * inverse_scale;
constexpr int expected_bases = 6188;
static_assert(2 * uniform_draws == probe_budget);
static_assert(base_size * planted_draws == probe_budget);

using Integer = std::int64_t;
using Base = std::array<int, base_size>;
using Hits = std::array<bool, base_size>;
using Residues = std::bitset<group_size>;
using RowSupports = std::array<Residues, base_size>;

struct Fraction {
    Integer numerator;
    Integer denominator;
};

void require(bool condition, const char* message) {
    if (!condition) {
        throw std::runtime_error(message);
    }
}

Fraction reduced(Integer numerator, Integer denominator) {
    const Integer divisor = std::gcd(numerator, denominator);
    return {numerator / divisor, denominator / divisor};
}

std::ostream& operator<<(std::ostream& out, Fraction fraction) {
    return out << fraction.numerator << '/' << fraction.denominator;
}

struct GateSums {
    int accepted = 0;
    int inverse_sum = 0;

    void add(int hits, int multiplicity) {
        accepted += hits;
        if (hits > 0) {
            require(multiplicity > 0 && inverse_scale % multiplicity == 0,
                    "invalid multiplicity for an accepted residual");
            inverse_sum += hits * (inverse_scale / multiplicity);
        }
    }

    Integer difference_numerator(int choices) const {
        // Multiply the exact block variance difference by
        // draws_uniform * draws_planted * 60 * (group_size * choices)^2.
        const Integer scaled_space = group_size * choices;
        const Integer planted_second = tuple_count * choices * inverse_sum;
        const Integer uniform_second = inverse_scale * scaled_space * accepted;
        const Integer mean_squared = inverse_scale * accepted * accepted;
        return uniform_draws * (planted_second - mean_squared) -
               planted_draws * (uniform_second - mean_squared);
    }
};

Integer difference_denominator(int choices) {
    return uniform_draws * planted_draws * inverse_scale *
           group_size * group_size * choices * choices;
}

struct OutcomeMoments {
    Integer outcomes = 0;
    Integer sum = 0;
    Integer square_sum = 0;

    void add(int scaled_score) {
        ++outcomes;
        sum += scaled_score;
        square_sum += scaled_score * scaled_score;
    }

    Integer centered_sum() const { return outcomes * square_sum - sum * sum; }
};

void check_direct(const Base& base, const RowSupports& rows,
                  const std::array<int, group_size>& tally, int choices,
                  Integer expected_difference) {
    OutcomeMoments uniform;
    OutcomeMoments planted;
    const auto add_gate = [&](int first, int second) {
        const Residues gate = rows[first] | rows[second];
        for (int target = 0; target < group_size; ++target) {
            uniform.add(gate.test(target) ? score_scale : 0);
        }
        for (int left : base) {
            for (int right : base) {
                const int target = (left + right) % group_size;
                const int score = tuple_count * inverse_scale / tally[target];
                planted.add(gate.test(target) ? score : 0);
            }
        }
    };

    if (choices == 1) {
        add_gate(0, 1);
    } else {
        for (int first = 0; first < base_size; ++first) {
            for (int second = first + 1; second < base_size; ++second) {
                add_gate(first, second);
            }
        }
    }
    require(uniform.outcomes == group_size * choices &&
            planted.outcomes == tuple_count * choices, "outcome count mismatch");
    require(uniform.sum * planted.outcomes == planted.sum * uniform.outcomes,
            "weighted and uniform means disagree");

    const Integer uniform_denominator = uniform.outcomes * uniform.outcomes * uniform_draws;
    const Integer planted_denominator = planted.outcomes * planted.outcomes * planted_draws;
    const auto direct = reduced(planted.centered_sum() * uniform_denominator -
                                    uniform.centered_sum() * planted_denominator,
                                planted_denominator * uniform_denominator *
                                    score_scale * score_scale);
    const auto aggregate = reduced(expected_difference, difference_denominator(choices));
    require(direct.numerator == aggregate.numerator && direct.denominator == aggregate.denominator,
            "direct and aggregate variance differences disagree");
}

struct Comparison {
    int support;
    Integer fixed;
    Integer all_pairs;
};

Comparison compare(const Base& base) {
    Residues members;
    RowSupports rows;
    std::array<int, group_size> tally{};
    for (int first = 0; first < base_size; ++first) {
        members.set(base[first]);
        for (int second : base) {
            const int target = (base[first] + second) % group_size;
            rows[first].set(target);
            ++tally[target];
        }
    }

    GateSums fixed;
    GateSums all_pairs;
    int support = 0;
    for (int target = 0; target < group_size; ++target) {
        Hits hits{};
        int multiplicity = 0;
        for (int position = 0; position < base_size; ++position) {
            const int complement = (target + group_size - base[position]) % group_size;
            hits[position] = members.test(complement);
            multiplicity += hits[position];
        }
        require(multiplicity == tally[target], "pair tally and complement count disagree");
        support += multiplicity > 0;
        fixed.add(hits[0] || hits[1], multiplicity);

        const int misses = base_size - multiplicity;
        const int accepted_pairs = gate_count - misses * (misses - 1) / 2;
        all_pairs.add(accepted_pairs, multiplicity);
    }

    const Integer fixed_difference = fixed.difference_numerator(1);
    const Integer all_pairs_difference = all_pairs.difference_numerator(gate_count);
    check_direct(base, rows, tally, 1, fixed_difference);
    check_direct(base, rows, tally, gate_count, all_pairs_difference);
    return {support, fixed_difference, all_pairs_difference};
}

bool advance(Base& base) {
    for (int position = base_size - 1; position >= 0; --position) {
        if (base[position] < group_size - base_size + position) {
            ++base[position];
            for (int suffix = position + 1; suffix < base_size; ++suffix) {
                base[suffix] = base[suffix - 1] + 1;
            }
            return true;
        }
    }
    return false;
}

// The difference is planted minus uniform, so its sign orders these three cases.
constexpr std::array<const char*, 3> preference_names{"planted", "tie", "uniform"};
using Counts = std::array<int, preference_names.size()>;
using JointCounts = std::array<Counts, preference_names.size()>;

std::size_t preferred(Integer difference) {
    return static_cast<std::size_t>((difference > 0) - (difference < 0) + 1);
}

struct Summary {
    Counts counts{};
    Integer minimum = 0;
    Integer maximum = 0;

    void add(Integer difference) {
        if (std::accumulate(counts.begin(), counts.end(), 0) == 0) {
            minimum = maximum = difference;
        }
        ++counts[preferred(difference)];
        minimum = std::min(minimum, difference);
        maximum = std::max(maximum, difference);
    }

    void print(const char* name, int choices) const {
        std::cout << name << " planted,tie,uniform="
                  << counts[0] << ',' << counts[1] << ',' << counts[2]
                  << " delta_range=" << reduced(minimum, difference_denominator(choices))
                  << ',' << reduced(maximum, difference_denominator(choices)) << '\n';
    }
};

void census() {
    Summary fixed;
    Summary all_pairs;
    JointCounts joint{};
    std::array<JointCounts, group_size + 1> by_support{};
    int bases = 0;
    Base base{0, 1, 2, 3, 4};
    do {
        const auto result = compare(base);
        fixed.add(result.fixed);
        all_pairs.add(result.all_pairs);
        const auto fixed_preference = preferred(result.fixed);
        const auto all_pairs_preference = preferred(result.all_pairs);
        ++joint[fixed_preference][all_pairs_preference];
        ++by_support[result.support][fixed_preference][all_pairs_preference];
        ++bases;
    } while (advance(base));
    require(bases == expected_bases, "subset census is incomplete");

    std::cout << "bases=" << bases << " group=Z/17Z base_size=5 ordered_pairs=25"
              << " ascending_residue_order probes=" << probe_budget << '\n'
              << "uniform_draws=" << uniform_draws << " probes_per_draw=2 planted_draws="
              << planted_draws << " probes_per_draw=5\n"
              << "prior_scope=two Z/19Z six-subsets with three-position gates at six probes\n"
              << "delta=variance(planted mean)-variance(uniform mean)\n";
    fixed.print("first_two", 1);
    all_pairs.print("all_pairs_fresh_each_draw", gate_count);
    for (std::size_t i = 0; i < joint.size(); ++i) {
        std::cout << "fixed=" << preference_names[i] << " all_pairs(planted,tie,uniform)="
                  << joint[i][0] << ',' << joint[i][1] << ',' << joint[i][2] << '\n';
    }
    std::cout << "strict_reversals=" << joint[0][2] + joint[2][0] << '\n';

    std::cout << "support bases fixed(planted,tie,uniform) all_pairs(planted,tie,uniform) strict_reversals\n";
    for (std::size_t support = 0; support < by_support.size(); ++support) {
        const auto& support_joint = by_support[support];
        Counts fixed_counts{};
        Counts all_pairs_counts{};
        for (std::size_t i = 0; i < support_joint.size(); ++i) {
            for (std::size_t j = 0; j < support_joint[i].size(); ++j) {
                fixed_counts[i] += support_joint[i][j];
                all_pairs_counts[j] += support_joint[i][j];
            }
        }
        const int count = std::accumulate(fixed_counts.begin(), fixed_counts.end(), 0);
        if (count > 0) {
            std::cout << support << ' ' << count << ' '
                      << fixed_counts[0] << ',' << fixed_counts[1] << ',' << fixed_counts[2] << ' '
                      << all_pairs_counts[0] << ',' << all_pairs_counts[1] << ',' << all_pairs_counts[2]
                      << ' ' << support_joint[0][2] + support_joint[2][0] << '\n';
        }
    }
    std::cout << "cross_checks=pair-tally,weighted-mean,direct-outcome-variance on every base and gate\n";
}

}  // namespace

int main() {
    census();
}
