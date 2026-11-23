#include <algorithm>
#include <array>
#include <functional>
#include <iostream>
#include <map>
#include <optional>
#include <stdexcept>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace {

constexpr int modulus = 101;
using Point = std::array<int, 2>;  // The sum determines z from x and y.
using Points = std::vector<Point>;
using Target = std::pair<int, int>;

int residue(int value) {
    return (value % modulus + modulus) % modulus;
}

int inverse(int value) {
    for (int candidate = 1; candidate < modulus; ++candidate) {
        if (residue(candidate * value) == 1) {
            return candidate;
        }
    }
    throw std::logic_error("Expected a nonzero field element");
}

int power(int value, int exponent) {
    int result = 1;
    for (int i = 0; i < exponent; ++i) {
        result = residue(result * value);
    }
    return result;
}

int evaluation_degree(const Points& points) {
    // Incremental elimination of the evaluation columns, in total-degree order.
    std::vector<std::vector<int>> pivots(points.size());
    std::size_t rank = 0;
    for (int degree = 0; degree < static_cast<int>(points.size()); ++degree) {
        for (int x_degree = 0; x_degree <= degree; ++x_degree) {
            std::vector<int> column;
            for (const auto& [x, y] : points) {
                column.push_back(residue(power(x, x_degree) * power(y, degree - x_degree)));
            }
            for (std::size_t row = 0; row < column.size(); ++row) {
                const int coefficient = column[row];
                if (coefficient == 0) {
                    continue;
                }
                if (pivots[row].empty()) {
                    const int scale = inverse(coefficient);
                    for (int& value : column) {
                        value = residue(value * scale);
                    }
                    pivots[row] = std::move(column);
                    ++rank;
                    break;
                }
                for (std::size_t j = row; j < column.size(); ++j) {
                    column[j] = residue(column[j] - coefficient * pivots[row][j]);
                }
            }
            if (rank == points.size()) {
                return degree + 1;
            }
        }
    }
    throw std::logic_error("Evaluation columns failed to span the finite point set");
}

void emit(std::string_view cohort, std::string_view fixture, std::optional<Target> target,
          std::optional<std::pair<char, int>> pin, const Points& points) {
    std::map<int, int> counts;
    for (const auto& point : points) {
        ++counts[point[1]];
    }
    std::vector<int> sizes;
    for (const auto& entry : counts) {
        sizes.push_back(entry.second);
    }
    std::ranges::sort(sizes, std::greater{});

    const int largest = sizes.empty() ? 0 : sizes[0];
    const int second = sizes.size() < 2 ? 0 : sizes[1];
    std::optional<int> predicted;
    std::optional<int> observed;
    if (!points.empty()) {
        observed = evaluation_degree(points);
        if (sizes.size() <= 2) {
            predicted = largest + (largest == second);
            if (predicted != observed) {
                throw std::logic_error("Fiber rule and evaluation rank disagree");
            }
        }
    }

    std::cout << cohort << ',' << fixture << ',';
    if (target) {
        std::cout << target->first << ',' << target->second;
    } else {
        std::cout << ',';
    }
    std::cout << ',';
    if (pin) {
        std::cout << pin->first << ',' << pin->second;
    } else {
        std::cout << ',';
    }
    std::cout << ',' << points.size() << ',' << sizes.size() << ',' << largest << ',' << second
              << ',';
    if (predicted) {
        std::cout << *predicted;
    }
    std::cout << ',';
    if (observed) {
        std::cout << *observed;
    }
    std::cout << '\n';
}

void check_targets(std::string_view fixture, const std::array<int, 5>& base) {
    std::map<Target, Points> targets;
    for (int x : base) {
        for (int y : base) {
            for (int z : base) {
                targets[{residue(x + y + z), residue(y * (x + z))}].push_back({x, y});
            }
        }
    }
    for (const auto& [target, points] : targets) {
        emit("target", fixture, target, std::nullopt, points);
    }

    const Target shared_target{3, 2};
    const auto& points = targets.at(shared_target);
    constexpr std::string_view variables = "xyz";
    for (std::size_t source = 0; source < variables.size(); ++source) {
        for (int value : {0, 1, 2}) {
            Points pinned;
            for (const auto& [x, y] : points) {
                const std::array triple{x, y, residue(shared_target.first - x - y)};
                if (residue(triple[source]) == value) {
                    pinned.push_back({x, y});
                }
            }
            emit("pin", fixture, shared_target, std::pair{variables[source], value}, pinned);
        }
    }
}

void check_controls() {
    constexpr std::array support{0, 1, 2, 17, 43};
    constexpr unsigned subset_count = 1U << support.size();
    for (unsigned left = 0; left < subset_count; ++left) {
        for (unsigned right = 0; right < subset_count; ++right) {
            Points points;
            for (std::size_t i = 0; i < support.size(); ++i) {
                if (left & (1U << i)) {
                    points.push_back({support[i], 1});
                }
                if (right & (1U << i)) {
                    points.push_back({support[i], 2});
                }
            }
            if (!points.empty()) {
                const auto fixture = std::to_string(left) + "-" + std::to_string(right);
                emit("control", fixture, std::nullopt, std::nullopt, points);
            }
        }
    }
    emit("boundary", "collinear", std::nullopt, std::nullopt, {{0, 0}, {0, 1}, {0, 2}});
    emit("boundary", "noncollinear", std::nullopt, std::nullopt, {{0, 0}, {0, 1}, {1, 2}});
}

}  // namespace

int main() {
    std::cout << "cohort,fixture,t,s,source,pin,points,fibers,largest,second,predicted,reference\n";
    check_targets("Centered", {-2, -1, 0, 1, 2});
    check_targets("Consecutive", {0, 1, 2, 3, 4});
    check_targets("Spread", {0, 1, 2, 17, 43});
    check_controls();
    return std::cout ? 0 : 1;
}
