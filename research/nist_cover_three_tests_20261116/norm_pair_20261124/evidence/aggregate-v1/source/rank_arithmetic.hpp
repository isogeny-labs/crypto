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
