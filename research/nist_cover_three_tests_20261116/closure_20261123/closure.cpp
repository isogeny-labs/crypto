#include "identity_arithmetic.hpp"
#include <functional>

using Row = std::vector<mpz_class>;
using Matrix = std::vector<Row>;
const mpz_class subgroup_order("FFFFFFFFFFFFFFFFFFFFFFFF99DEF836146BC9B1B4D22831", 16);
mpz_class reduced(mpz_class x) {
    x %= subgroup_order;
    if (x < 0) x += subgroup_order;
    return x;
}
Matrix normalized(Matrix matrix) {
    const size_t width = matrix.empty() ? 0 : matrix.front().size();
    for (auto& row : matrix) {
        require(row.size() == width, "nonrectangular matrix");
        for (auto& value : row) value = reduced(value);
    }
    return matrix;
}
size_t matrix_rank(Matrix matrix) {
    matrix = normalized(std::move(matrix));
    if (matrix.empty()) return 0;
    size_t pivot = 0;
    for (size_t col = 0; col < matrix.front().size() && pivot < matrix.size(); ++col) {
        size_t selected = pivot;
        while (selected < matrix.size() && matrix[selected][col] == 0) ++selected;
        if (selected == matrix.size()) continue;
        std::swap(matrix[pivot], matrix[selected]);
        mpz_class inverse;
        require(mpz_invert(inverse.get_mpz_t(), matrix[pivot][col].get_mpz_t(),
                           subgroup_order.get_mpz_t()) != 0, "noninvertible pivot");
        for (auto& value : matrix[pivot]) value = reduced(value * inverse);
        for (size_t row = pivot + 1; row < matrix.size(); ++row) {
            const mpz_class factor = matrix[row][col];
            for (size_t j = col; j < matrix[row].size(); ++j)
                matrix[row][j] = reduced(matrix[row][j] - factor * matrix[pivot][j]);
        }
        ++pivot;
    }
    return pivot;
}
Matrix new_columns(const Matrix& matrix, size_t old) {
    Matrix result;
    for (const auto& row : matrix) {
        require(old <= row.size(), "original-column boundary");
        result.emplace_back(row.begin() + static_cast<std::ptrdiff_t>(old), row.end());
    }
    return result;
}
using Witness = std::pair<size_t, size_t>;
void check_peeling(const Matrix& input, size_t old, const std::vector<Witness>& witnesses) {
    const auto matrix = normalized(input);
    require(!matrix.empty(), "empty certificate matrix");
    std::vector<bool> active(matrix.size(), true);
    for (auto [owner, col] : witnesses) {
        require(owner < matrix.size() && active[owner], "inactive owner");
        require(col >= old && col < matrix.front().size(), "not a new column");
        require(matrix[owner][col] != 0, "zero owner coefficient");
        for (size_t row = 0; row < matrix.size(); ++row)
            require(row == owner || !active[row] || matrix[row][col] == 0,
                    "column is not singleton");
        active[owner] = false;
    }
}
std::vector<Witness> peel(const Matrix& input, size_t old) {
    const auto matrix = normalized(input);
    if (matrix.empty()) return {};
    std::vector<bool> active(matrix.size(), true);
    std::vector<Witness> result;
    bool changed = true;
    while (changed) {
        changed = false;
        for (size_t col = old; col < matrix.front().size(); ++col) {
            size_t count = 0, owner = 0;
            for (size_t row = 0; row < matrix.size(); ++row)
                if (active[row] && matrix[row][col] != 0) { ++count; owner = row; }
            if (count == 1) {
                result.emplace_back(owner, col);
                active[owner] = false;
                changed = true;
            }
        }
    }
    check_peeling(matrix, old, result);
    return result;
}
void rejects(const std::function<void()>& call) {
    bool rejected = false;
    try { call(); } catch (const std::runtime_error&) { rejected = true; }
    require(rejected, "corrupt singleton witness accepted");
}
int main(int argc, char** argv) {
    try {
        require(argc == 3, "usage: closure FROZEN-relation.json NEW-output-directory");
        const auto baseline = read_json(argv[1]);
        const size_t columns = static_cast<size_t>(baseline.at("projected_base_size").integer());
        require(columns == 225, "frozen base columns");
        Matrix original;
        for (const auto& sparse : baseline.at("rows").arr) {
            Row row(columns);
            for (const auto& item : sparse.arr) {
                require(item.arr.size() == 2, "sparse pair");
                const int index = item.arr[0].integer();
                require(index >= 0 && static_cast<size_t>(index) < columns, "sparse index");
                row[static_cast<size_t>(index)] += mpz_class(item.arr[1].scalar);
            }
            original.push_back(std::move(row));
        }
        const size_t original_rank = matrix_rank(original);
        require(original_rank == 6 && baseline.at("rank_mod_r").integer() == 6,
                "frozen baseline rank mismatch");
        const Row base{1,-1,0,0,0,0};
        struct Case { const char* name; Matrix rows; size_t gain; };
        const Matrix private_rows{base, {0,0,1,1,0,0}, {1,0,0,0,1,0}};
        const Matrix shared_rows{base, {0,0,1,1,0,0}, {1,0,0,1,0,0}};
        const std::vector<Case> cases{
            {"private_columns", private_rows, 0},
            {"shared_alias", shared_rows, 1},
            {"opposite_signed_alias", {base, {0,0,1,1,0,0}, {1,0,0,-1,0,0}}, 1},
            {"unsigned_three_cycle", {base, {0,0,1,1,1,0}, {0,0,0,0,1,1}, {0,0,0,1,0,1}}, 0},
            {"signed_three_cycle", {base, {0,0,1,1,-1,0}, {0,0,0,0,1,-1}, {0,0,0,-1,0,1}}, 1},
            {"canonicalized_zero_new_part", {base, {0,0,1,0,0,0}}, 1}
        };
        std::cout << "{\"scope\":\"synthetic-pencil-closure-and-frozen-baseline-only\","
                     "\"standing\":\"producer-unverified\",\"subgroup_order\":\"" << subgroup_order
                  << "\",\"actual_baseline_columns\":" << columns
                  << ",\"actual_baseline_rank\":" << original_rank << ",\"cases\":[";
        bool first = true;
        for (const auto& item : cases) {
            const size_t total = matrix_rank(item.rows), projected = matrix_rank(new_columns(item.rows, 3));
            require(total >= projected + 1, "closed rank contains original row");
            const size_t gain = total - projected - 1;
            require(gain == item.gain, "synthetic expected gain mismatch");
            const auto witnesses = peel(item.rows, 3);
            if (!first) std::cout << ',';
            first = false;
            std::cout << "{\"name\":\"" << item.name << "\",\"total_rank\":" << total
                      << ",\"new_part_rank\":" << projected << ",\"closed_rank_gain\":" << gain
                      << ",\"peeled_rows\":" << witnesses.size() << ",\"matrix\":[";
            for (size_t row = 0; row < item.rows.size(); ++row) {
                if (row) std::cout << ',';
                std::cout << '[';
                for (size_t col = 0; col < item.rows[row].size(); ++col) {
                    if (col) std::cout << ',';
                    std::cout << '"' << item.rows[row][col] << '"';
                }
                std::cout << ']';
            }
            std::cout << "],\"singleton_witnesses\":[";
            for (size_t j = 0; j < witnesses.size(); ++j) {
                if (j) std::cout << ',';
                std::cout << '[' << witnesses[j].first << ',' << witnesses[j].second << ']';
            }
            std::cout << "]}";
        }
        rejects([&] { check_peeling(private_rows, 3, {{0,3}}); });
        rejects([&] { check_peeling(shared_rows, 3, {{1,3}}); });
        rejects([&] { check_peeling(private_rows, 3, {{1,2}}); });
        rejects([&] { check_peeling(private_rows, 3, {{1,3},{1,3}}); });
        std::cout << "],\"corruptions_rejected\":4,\"new_curve_relations_generated\":0,"
                     "\"scalar_recovery_executed\":false,\"performance_measurement\":false}\n";
        const auto lambda = var("lambda"), w = var("w"), v = var("v");
        issue(std::string(argv[2]) + "/singleton-step.identity.json", "Cleared singleton-column elimination",
              {"lambda","w","v"}, mul({lambda, sub(mul({w,v}), num(1))}),
              sub(mul({lambda,w,v}), lambda), 3);
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "REJECT " << error.what() << '\n';
        return 1;
    }
}
