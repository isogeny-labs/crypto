#include <algorithm>
#include <array>
#include <cstdlib>
#include <iostream>
#include <map>
#include <numeric>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

using Row = std::vector<int>;
using Matrix = std::vector<Row>;

void require(bool condition, const char* message) {
    if (!condition) {
        throw std::runtime_error(message);
    }
}

int mod(int value, int field) {
    return (value % field + field) % field;
}

void print(const Row& values) {
    for (int value : values) {
        std::cout << ' ' << value;
    }
}

struct Counts {
    Row multiplicities;
    int support = 0;
    int squared = 0;
    int cubed = 0;
};

Counts count_pairs(int group, const Row& base) {
    Counts result{Row(group)};
    for (int a : base) {
        for (int b : base) {
            ++result.multiplicities[mod(a + b, group)];
        }
    }
    int weight_scale = 1;
    for (int residual = 0; residual < group; ++residual) {
        int count = result.multiplicities[residual];
        int direct = 0;
        for (int a : base) {
            int b = mod(residual - a, group);
            direct += std::binary_search(base.begin(), base.end(), b);
        }
        require(count == direct, "counts disagree under subtraction");
        if (count) {
            ++result.support;
            weight_scale = std::lcm(weight_scale, count);
        }
        result.squared += count * count;
        result.cubed += count * count * count;
    }
    int planted_weight = 0;
    for (int a : base) {
        for (int b : base) {
            planted_weight += weight_scale /
                result.multiplicities[mod(a + b, group)];
        }
    }
    require(planted_weight == weight_scale * result.support,
            "inverse multiplicity does not recover support");
    return result;
}

void coverage() {
    struct Fixture {
        int group;
        Row base;
        int support;
        int squared;
        int cubed;
    };
    const std::array<Fixture, 4> fixtures{{
        {19, {0, 1, 2, 4, 7, 12}, 15, 90, 234},
        {19, {0, 1, 3, 12, 14, 15}, 19, 90, 318},
        {23, {0, 1, 2, 3, 5, 10, 18}, 18, 151, 541},
        {23, {0, 1, 2, 3, 7, 17, 18}, 20, 151, 541},
    }};
    for (const auto& fixture : fixtures) {
        Counts result = count_pairs(fixture.group, fixture.base);
        require(result.support == fixture.support &&
                result.squared == fixture.squared &&
                result.cubed == fixture.cubed, "coverage fixture mismatch");
        Row histogram(fixture.base.size() + 1);
        for (int count : result.multiplicities) {
            ++histogram[count];
        }
        std::cout << "group " << fixture.group << " base:";
        print(fixture.base);
        std::cout << "; support " << result.support << '/' << fixture.group
                  << "; sum_R^2 " << result.squared
                  << "; sum_R^3 " << result.cubed << "; histogram:";
        print(histogram);
        std::cout << '\n';
    }

    std::map<int, Row> supports_by_second_moment;
    int total = 0;
    for (int a = 0; a < 11; ++a) {
        for (int b = a + 1; b < 11; ++b) {
            for (int c = b + 1; c < 11; ++c) {
                for (int d = c + 1; d < 11; ++d) {
                    Counts result = count_pairs(11, {a, b, c, d});
                    supports_by_second_moment[result.squared].push_back(result.support);
                    ++total;
                }
            }
        }
    }
    require(total == 330, "incomplete subset census");
    std::cout << "full subset census: " << total << '\n';
    for (const auto& [second_moment, supports] : supports_by_second_moment) {
        auto [low, high] = std::minmax_element(supports.begin(), supports.end());
        std::cout << "sum_R^2 " << second_moment << "; sets " << supports.size()
                  << "; support range " << *low << '-' << *high << '\n';
    }
}

int rank(Matrix matrix, int field) {
    int pivot = 0;
    const int rows = static_cast<int>(matrix.size());
    const int columns = static_cast<int>(matrix.front().size());
    for (auto& row : matrix) {
        for (int& entry : row) {
            entry = mod(entry, field);
        }
    }
    for (int column = 0; column < columns && pivot < rows; ++column) {
        int next = pivot;
        while (next < rows && matrix[next][column] == 0) {
            ++next;
        }
        if (next == rows) {
            continue;
        }
        std::swap(matrix[next], matrix[pivot]);
        for (int i = pivot + 1; i < rows; ++i) {
            int coefficient = matrix[i][column];
            int leading = matrix[pivot][column];
            if (coefficient) {
                for (int j = column; j < columns; ++j) {
                    matrix[i][j] = mod(leading * matrix[i][j] -
                                        coefficient * matrix[pivot][j], field);
                }
            }
        }
        ++pivot;
    }
    return pivot;
}

int solution_count(const Matrix& matrix, int field) {
    const auto columns = matrix.front().size();
    int points = 1;
    for (std::size_t i = 0; i < columns; ++i) {
        points *= field;
    }
    int solutions = 0;
    for (int index = 0; index < points; ++index) {
        int rest = index;
        Row point(columns);
        for (int& coordinate : point) {
            coordinate = rest % field;
            rest /= field;
        }
        solutions += std::all_of(matrix.begin(), matrix.end(), [&](const Row& row) {
            return mod(std::inner_product(row.begin(), row.end(), point.begin(), 0),
                       field) == 0;
        });
    }
    return solutions;
}

void check_rank(const Matrix& matrix, int field, int computed) {
    if (field > 3) {
        return;
    }
    int expected = 1;
    for (std::size_t i = computed; i < matrix.front().size(); ++i) {
        expected *= field;
    }
    require(solution_count(matrix, field) == expected, "nullspace count disagrees");
}

Matrix signed_matrix(const Row& signs) {
    const std::array<std::pair<int, int>, 5> edges{{
        {0, 1}, {1, 2}, {2, 3}, {3, 0}, {0, 2}
    }};
    Matrix rows(edges.size(), Row(4));
    for (std::size_t i = 0; i < edges.size(); ++i) {
        auto [start, end] = edges[i];
        rows[i][start] = -signs[i];
        rows[i][end] = 1;
    }
    return rows;
}

void same_histograms(const Matrix& a, const Matrix& b, bool ordered_rows) {
    Row negatives_a;
    Row negatives_b;
    for (std::size_t i = 0; i < a.size(); ++i) {
        for (std::size_t j = 0; j < a[i].size(); ++j) {
            require((a[i][j] == 0) == (b[i][j] == 0), "different supports");
        }
        negatives_a.push_back(std::count(a[i].begin(), a[i].end(), -1));
        negatives_b.push_back(std::count(b[i].begin(), b[i].end(), -1));
    }
    require(ordered_rows ? negatives_a == negatives_b :
            std::is_permutation(negatives_a.begin(), negatives_a.end(), negatives_b.begin()),
            "different row histograms");
    for (std::size_t j = 0; j < a.front().size(); ++j) {
        for (int value : {-1, 0, 1}) {
            const auto count = [&](const Matrix& rows) {
                return std::count_if(rows.begin(), rows.end(),
                                     [&](const Row& r) { return r[j] == value; });
            };
            require(count(a) == count(b), "different column histograms");
        }
    }
}

Row prefix_ranks(const Matrix& matrix, int field, bool project) {
    Row ranks;
    Matrix prefix;
    Matrix auxiliary;
    for (const Row& row : matrix) {
        prefix.push_back(row);
        int total_rank = rank(prefix, field);
        check_rank(prefix, field, total_rank);
        if (project) {
            auxiliary.push_back({row[2], row[3]});
            int auxiliary_rank = rank(auxiliary, field);
            check_rank(auxiliary, field, auxiliary_rank);
            total_rank -= auxiliary_rank;
        }
        ranks.push_back(total_rank);
    }
    return ranks;
}

void rank_controls() {
    const Matrix balanced = signed_matrix({-1, -1, 1, 1, 1});
    const Matrix unbalanced = signed_matrix({1, -1, 1, 1, -1});
    const Matrix a{{1, 1, -1, -1}, {-1, 1, 1, 1}, {1, -1, 1, 1}};
    const Matrix b{{-1, 1, -1, 1}, {1, -1, 1, 1}, {1, 1, 1, -1}};
    same_histograms(balanced, unbalanced, false);
    same_histograms(a, b, true);
    for (const int field : {2, 3, 101}) {
        const bool odd = field != 2;
        Row balanced_ranks = prefix_ranks(balanced, field, false);
        Row unbalanced_ranks = prefix_ranks(unbalanced, field, false);
        require(balanced_ranks == Row{1, 2, 3, 3, 3} &&
                unbalanced_ranks == Row{1, 2, 3, odd ? 4 : 3, odd ? 4 : 3},
                "signed graph rank fixture mismatch");
        int full_after_four = 0;
        int full_after_five = 0;
        int assignments = 0;
        for (int first = 0; first < 5; ++first) {
            for (int second = first + 1; second < 5; ++second) {
                Row signs(5, 1);
                signs[first] = signs[second] = -1;
                Row ranks = prefix_ranks(signed_matrix(signs), field, false);
                full_after_four += ranks[3] == 4;
                full_after_five += ranks[4] == 4;
                ++assignments;
            }
        }
        require(assignments == 10 && full_after_four == (odd ? 4 : 0) &&
                full_after_five == (odd ? 8 : 0), "sign census mismatch");
        Row projected_a = prefix_ranks(a, field, true);
        Row projected_b = prefix_ranks(b, field, true);
        int total_a = prefix_ranks(a, field, false).back();
        int total_b = prefix_ranks(b, field, false).back();
        require(total_a == (odd ? 3 : 1) && total_a == total_b &&
                projected_a == Row{0, odd ? 1 : 0, odd ? 2 : 0} &&
                projected_b == Row{0, 0, odd ? 1 : 0}, "auxiliary rank mismatch");

        std::cout << "field " << field << "; balanced:";
        print(balanced_ranks);
        std::cout << "; unbalanced:";
        print(unbalanced_ranks);
        std::cout << "; full rank after 4,5: "
                  << full_after_four << ',' << full_after_five << '/' << assignments
                  << "; aux fixtures total rank " << total_a << "; primary A:";
        print(projected_a);
        std::cout << "; primary B:";
        print(projected_b);
        std::cout << '\n';
    }
}

int main(int argc, char** argv) {
    try {
        if (argc == 2 && std::string(argv[1]) == "coverage") {
            coverage();
        } else if (argc == 2 && std::string(argv[1]) == "rank") {
            rank_controls();
        } else {
            std::cerr << "usage: replay coverage|rank\n";
            return EXIT_FAILURE;
        }
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return EXIT_FAILURE;
    }
}
