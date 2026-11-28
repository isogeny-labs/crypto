#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <map>
#include <optional>
#include <set>
#include <string_view>
#include <utility>
#include <vector>

namespace {

constexpr int modulus = 101;
constexpr int target_sum = 3;
constexpr int target_mixed = 2;
constexpr std::string_view variables = "xyz";
constexpr std::array stages{"membership", "sum", "mixed"};

using Triple = std::array<int, 3>;
using StageTuples = std::array<std::vector<Triple>, stages.size()>;

struct Base {
    std::string_view name;
    std::array<int, 5> values;
};

constexpr std::array bases{
    Base{"Centered", {-2, -1, 0, 1, 2}},
    Base{"Consecutive", {0, 1, 2, 3, 4}},
    Base{"Spread", {0, 1, 2, 17, 43}},
};

struct Support {
    std::size_t triples = 0;
    std::array<std::set<int>, 2> values;
    std::map<std::pair<int, int>, std::size_t> pairs;
};

int residue(int value) {
    return (value % modulus + modulus) % modulus;
}

StageTuples run_filters(const Base& base) {
    StageTuples survivors;
    for (int x : base.values) {
        for (int y : base.values) {
            for (int z : base.values) {
                const Triple triple{x, y, z};
                survivors[0].push_back(triple);
                if (residue(x + y + z) == target_sum) {
                    survivors[1].push_back(triple);
                    if (residue(y * (x + z)) == target_mixed) {
                        survivors[2].push_back(triple);
                    }
                }
            }
        }
    }
    return survivors;
}

std::vector<int> shared_pins() {
    std::vector<int> common;
    for (int value : bases.front().values) {
        if (std::ranges::all_of(bases, [value](const Base& base) {
                return std::ranges::find(base.values, value) != base.values.end();
            })) {
            common.push_back(value);
        }
    }
    return common;
}

Support project(const std::vector<Triple>& triples, std::size_t source,
                std::array<std::size_t, 2> receivers, std::optional<int> pin) {
    Support result;
    for (const auto& triple : triples) {
        if (pin && triple[source] != *pin) {
            continue;
        }
        const int first = triple[receivers[0]];
        const int second = triple[receivers[1]];
        ++result.triples;
        result.values[0].insert(first);
        result.values[1].insert(second);
        ++result.pairs[{first, second}];
    }
    return result;
}

void write_support(std::ostream& output, const Base& base, std::string_view stage,
                   std::size_t source, std::array<std::size_t, 2> receivers,
                   std::optional<int> pin, const Support& support) {
    for (const auto& [pair, count] : support.pairs) {
        output << base.name << ',' << stage << ',' << variables[source] << ',';
        if (pin) {
            output << *pin;
        }
        output << ',' << variables[receivers[0]] << ',' << pair.first << ','
               << variables[receivers[1]] << ',' << pair.second << ',' << count << '\n';
    }
}

void measure(std::ostream& summary, std::ostream& detail, const Base& base,
             const std::vector<int>& pins) {
    const auto survivors = run_filters(base);
    for (std::size_t stage = 0; stage < stages.size(); ++stage) {
        for (std::size_t source = 0; source < variables.size(); ++source) {
            // Preserve the variable order after removing the pinned coordinate.
            const std::array<std::size_t, 2> receivers{
                source == 0 ? 1U : 0U,
                source == 2 ? 1U : 2U,
            };
            const auto baseline = project(survivors[stage], source, receivers, std::nullopt);
            write_support(detail, base, stages[stage], source, receivers, std::nullopt, baseline);
            for (int pin : pins) {
                const auto pinned = project(survivors[stage], source, receivers, pin);
                summary << base.name << ',' << stages[stage] << ',' << variables[source] << ','
                        << pin << ',' << baseline.triples << ',' << pinned.triples;
                for (std::size_t i = 0; i < receivers.size(); ++i) {
                    summary << ',' << variables[receivers[i]] << ',' << baseline.values[i].size()
                            << ',' << pinned.values[i].size();
                }
                summary << ',' << baseline.pairs.size() << ',' << pinned.pairs.size() << '\n';
                write_support(detail, base, stages[stage], source, receivers, pin, pinned);
            }
        }
    }
}

}  // namespace

int main(int argc, char* argv[]) {
    if (argc != 3 || std::string_view(argv[1]) == argv[2]) {
        std::cerr << "Usage: measure_survivors SUMMARY.csv SUPPORT.csv\n";
        return 1;
    }
    std::ofstream summary(argv[1]);
    std::ofstream detail(argv[2]);
    if (!summary || !detail) {
        std::cerr << "Could not open the survivor output files\n";
        return 1;
    }
    const auto pins = shared_pins();
    summary << "base,stage,source,pin,triples_before,triples_after,"
               "receiver_a,values_a_before,values_a_after,receiver_b,values_b_before,"
               "values_b_after,pairs_before,pairs_after\n";
    detail << "base,stage,source,pin,receiver_a,value_a,receiver_b,value_b,multiplicity\n";
    for (const auto& base : bases) {
        measure(summary, detail, base, pins);
    }
    summary.close();
    detail.close();
    if (!summary || !detail) {
        std::cerr << "Could not finish writing the survivor output files\n";
        return 1;
    }
}
