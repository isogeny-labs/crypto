// Independent exhaustive check of the distinct-image chart over F_127.
#include <algorithm>
#include <array>
#include <cstdint>
#include <iostream>
#include <map>
#include <optional>
#include <set>
#include <stdexcept>
#include <tuple>
#include <utility>
#include <vector>

namespace {
constexpr int p = 127;
constexpr int a = 1;
constexpr int b = 7;

int field(int x) {
    const int reduced = x % p;
    return reduced < 0 ? reduced + p : reduced;
}

void require(bool condition, const char* message) {
    if (!condition) {
        throw std::logic_error(message);
    }
}

int inverse(int x) {
    require(field(x) != 0, "inverse of zero");
    int result = 1;
    int power = field(x);
    for (int exponent = p - 2; exponent > 0; exponent >>= 1) {
        if (exponent & 1) {
            result = field(result * power);
        }
        power = field(power * power);
    }
    return result;
}

using Point = std::optional<std::pair<int, int>>;

Point add(const Point& left, const Point& right) {
    if (!left) {
        return right;
    }
    if (!right) {
        return left;
    }
    const auto [x, y] = *left;
    const auto [u, v] = *right;
    if (x == u && field(y + v) == 0) {
        return std::nullopt;
    }
    const int slope = x == u
        ? field((3*x*x + a) * inverse(2*y))
        : field((v-y) * inverse(u-x));
    const int z = field(slope*slope - x - u);
    return std::pair{z, field(slope*(x-z)-y)};
}

std::pair<int, int> torus_mul(std::pair<int, int> left, std::pair<int, int> right) {
    const auto [x, y] = left;
    const auto [u, v] = right;
    return {field(x*u-y*v), field(x*v+y*u)};
}

std::pair<int, int> torus_pow(std::pair<int, int> base, int exponent) {
    std::pair<int, int> result{1, 0};
    while (exponent > 0) {
        if (exponent & 1) {
            result = torus_mul(result, base);
        }
        base = torus_mul(base, base);
        exponent >>= 1;
    }
    return result;
}

int selector(int x, int steps) {
    for (int level = 0; level < steps; ++level) {
        x = field(x*x-2);
    }
    return x;
}

struct Jet {
    int value;
    std::array<int, 3> gradient;

    Jet(int value = 0) : value(field(value)), gradient{} {}

    static Jet variable(int value, int index) {
        Jet result(value);
        result.gradient[index] = 1;
        return result;
    }
};

Jet operator+(const Jet& left, const Jet& right) {
    Jet result(left.value+right.value);
    for (int i = 0; i < 3; ++i) {
        result.gradient[i] = field(left.gradient[i]+right.gradient[i]);
    }
    return result;
}

Jet operator-(const Jet& left, const Jet& right) {
    Jet result(left.value-right.value);
    for (int i = 0; i < 3; ++i) {
        result.gradient[i] = field(left.gradient[i]-right.gradient[i]);
    }
    return result;
}

Jet operator*(const Jet& left, const Jet& right) {
    Jet result(left.value*right.value);
    for (int i = 0; i < 3; ++i) {
        result.gradient[i] = field(left.gradient[i]*right.value+left.value*right.gradient[i]);
    }
    return result;
}

using Quartic = std::array<Jet, 4>;

Quartic pushforward(const Quartic& coefficients) {
    const auto [c0, c1, c2, c3] = coefficients;
    const auto d0 = c0*c0;
    const auto d1 = 2*c0*c2-c1*c1;
    const auto d2 = c2*c2+2*c0-2*c1*c3;
    const auto d3 = 2*c2-c3*c3;
    return {d0+2*d1+4*d2+8*d3+16, d1+4*d2+12*d3+32, d2+6*d3+24, d3+8};
}

std::array<int, 4> polynomial(const std::array<int, 4>& roots) {
    std::array<int, 5> coefficients{1, 0, 0, 0, 0};
    int degree = 0;
    for (const int root : roots) {
        ++degree;
        for (int i = degree; i >= 1; --i) {
            coefficients[i] = field(coefficients[i-1] - root*coefficients[i]);
        }
        coefficients[0] = field(-root*coefficients[0]);
    }
    require(coefficients[4] == 1, "quartic leading coefficient");
    return {coefficients[0], coefficients[1], coefficients[2], coefficients[3]};
}

using Matrix = std::array<std::array<int, 4>, 4>;

// Row reduce the first three columns, retaining the fourth as the RHS.
int reduce(Matrix& matrix) {
    int rank = 0;
    for (int column = 0; column < 3; ++column) {
        int pivot = rank;
        while (pivot < 4 && matrix[pivot][column] == 0) {
            ++pivot;
        }
        if (pivot == 4) {
            continue;
        }
        std::swap(matrix[rank], matrix[pivot]);
        const int scale = inverse(matrix[rank][column]);
        for (int& coefficient : matrix[rank]) {
            coefficient = field(coefficient*scale);
        }
        for (int row = 0; row < 4; ++row) {
            if (row == rank) {
                continue;
            }
            const int factor = matrix[row][column];
            for (int j = column; j < 4; ++j) {
                matrix[row][j] = field(matrix[row][j]-factor*matrix[rank][j]);
            }
        }
        ++rank;
    }
    return rank;
}

int differential_rank(const Quartic& quartic) {
    Matrix matrix{};
    for (int row = 0; row < 4; ++row) {
        for (int column = 0; column < 3; ++column) {
            matrix[row][column] = quartic[row].gradient[column];
        }
    }
    return reduce(matrix);
}

Quartic divisor_quartic(const std::pair<int, int>& negative_target,
                         const std::array<std::pair<int, int>, 4>& leaves) {
    const auto [u, v] = negative_target;
    Matrix matrix{};
    for (int row = 0; row < 4; ++row) {
        const auto [x, y] = leaves[row];
        matrix[row] = {field(x-u), field(x*(x-u)), field(y-v), field(-x*(y-v))};
    }
    require(reduce(matrix) == 3 && matrix[3][3] == 0, "RR5 interpolation");
    const Jet alpha = Jet::variable(matrix[0][3], 0);
    const Jet beta = Jet::variable(matrix[1][3], 1);
    const Jet t = Jet::variable(matrix[2][3], 2);
    const Jet z1 = v-alpha+beta*u;
    const Jet z0 = v*t+alpha*u;
    const std::array<Jet, 5> norm = {
        2*z1*z0-a*t*t-2*b*t,
        z1*z1-2*beta*z0-2*a*t-b,
        0-2*beta*z1-t*t-a,
        beta*beta-2*t,
        -1
    };
    Quartic result{};
    Jet carry = norm.back();
    for (int i = 3; i >= 0; --i) {
        carry = norm[i]+u*carry;
        result[i] = 0-carry;
    }
    return result;
}

void check_quartic(const Quartic& quartic, const std::array<int, 4>& roots) {
    const auto expected = polynomial(roots);
    for (int i = 0; i < 4; ++i) {
        require(quartic[i].value == expected[i], "coefficient and point images disagree");
    }
}

void run(int depth) {
    const int cardinality = 1 << depth;
    std::pair<int, int> alpha{};
    for (int x = 2; x < p; ++x) {
        const int denominator = inverse(x*x+1);
        alpha = {field((x*x-1)*denominator), field(2*x*denominator)};
        if (torus_pow(alpha, (p+1)/2) != std::pair{1, 0}) {
            break;
        }
    }
    require(torus_pow(alpha, p+1) == std::pair{1, 0}, "invalid torus element");
    const int terminal = field(2*torus_pow(alpha, cardinality).first);
    std::vector<std::pair<int, int>> base;
    std::map<int, int> pools;
    int trace_count = 0;
    for (int x = 0; x < p; ++x) {
        if (selector(x, depth) != terminal) {
            continue;
        }
        ++trace_count;
        const int rhs = field(x*x*x+a*x+b);
        for (int y = 1; y < p/2+1; ++y) {
            if (field(y*y) == rhs) {
                base.emplace_back(x, y);
                ++pools[selector(x, depth-2)];
                break;
            }
        }
    }
    require(trace_count == cardinality, "unexpected selector cardinality");
    int checked = 0;
    int separated = 0;
    int collapsed_rank_one = 0;
    int separated_rank_three = 0;
    std::map<std::pair<int, int>, std::pair<int, int>> target_counts;
    const auto size = base.size();
    for (std::size_t i = 0; i < size; ++i) {
        for (std::size_t j = i+1; j < size; ++j) {
            for (std::size_t k = j+1; k < size; ++k) {
                for (std::size_t l = k+1; l < size; ++l) {
                    const std::array<std::pair<int, int>, 4> support{base[i], base[j], base[k], base[l]};
                    std::array<int, 4> roots{base[i].first, base[j].first, base[k].first, base[l].first};
                    std::set<int> images;
                    for (const int x : roots) {
                        images.insert(selector(x, depth-2));
                    }
                    const bool distinct = images.size() == 4;
                    for (int signs = 0; signs < 16; ++signs) {
                        auto leaves = support;
                        Point target;
                        for (int leaf = 0; leaf < 4; ++leaf) {
                            if (signs & (1 << leaf)) {
                                leaves[leaf].second = field(-leaves[leaf].second);
                            }
                            target = add(target, leaves[leaf]);
                        }
                        if (!target || std::find(roots.begin(), roots.end(), target->first) != roots.end()) {
                            continue;
                        }
                        const auto [u, y] = *target;
                        auto quartic = divisor_quartic({u, field(-y)}, leaves);
                        auto current = roots;
                        check_quartic(quartic, current);
                        for (int level = 0; level < depth; ++level) {
                            if (distinct && level == depth-2) {
                                require(differential_rank(quartic) == 3, "separated differential lost rank");
                                ++separated_rank_three;
                            }
                            quartic = pushforward(quartic);
                            for (int& root : current) {
                                root = selector(root, 1);
                            }
                            check_quartic(quartic, current);
                        }
                        const int rank = differential_rank(quartic);
                        require(rank <= 1, "collapsed differential exceeded rank one");
                        collapsed_rank_one += rank == 1;
                        ++checked;
                        separated += distinct;
                        auto& counts = target_counts[*target];
                        ++counts.first;
                        counts.second += distinct;
                    }
                }
            }
        }
    }
    require(separated_rank_three == separated, "separated coverage mismatch");
    std::cout << "q=" << cardinality << " base=" << size << " pools=";
    for (const auto& [image, count] : pools) {
        std::cout << image << ":" << count << " ";
    }
    std::cout << "signed=" << checked << " separated=" << separated
              << " collapsed_rank_one=" << collapsed_rank_one
              << " separated_rank_three=" << separated_rank_three << '\n';
    for (const auto& target : {std::pair{20, 91}, {116, 69}, {41, 95}, {41, 32}}) {
        if (!target_counts.contains(target)) {
            continue;
        }
        const auto [total, selected] = target_counts.at(target);
        std::cout << "  target=(" << target.first << "," << target.second
                  << ") signed=" << total << " separated=" << selected << '\n';
    }
}
} // namespace

int main() {
    int order = 1;
    for (int x = 0; x < p; ++x) {
        for (int y = 0; y < p; ++y) {
            order += field(y*y) == field(x*x*x+a*x+b);
        }
    }
    require(field(4*a*a*a+27*b*b) != 0 && order == 109, "curve fixture");
    run(3);
    run(4);
}
