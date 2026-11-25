#pragma once
#include <gmpxx.h>
#include <algorithm>
#include <array>
#include <ostream>
#include <vector>

using SCBig = mpz_class;
using SCRow = std::vector<SCBig>;
using SCMat = std::vector<SCRow>;
const SCBig sc_p("6277101735386680763835789423207666416083908700390324961279");
const SCBig sc_n("6277101735386680763835789423176059013767194773182842284081");
SCBig sc_reduce(SCBig value, const SCBig& modulus = sc_n) {
    mpz_mod(value.get_mpz_t(), value.get_mpz_t(), modulus.get_mpz_t());
    return value;
}
size_t sc_rank(SCMat matrix) {
    if (matrix.empty()) return 0;
    const size_t width = matrix[0].size();
    for (auto& row : matrix) {
        require(row.size() == width, "rectangular rank input");
        for (auto& value : row) value = sc_reduce(value);
    }
    size_t rank = 0;
    for (size_t col = 0; col < width && rank < matrix.size(); ++col) {
        size_t pivot = rank;
        while (pivot < matrix.size() && matrix[pivot][col] == 0) ++pivot;
        if (pivot == matrix.size()) continue;
        std::swap(matrix[pivot], matrix[rank]);
        SCBig inverse;
        require(mpz_invert(inverse.get_mpz_t(), matrix[rank][col].get_mpz_t(), sc_n.get_mpz_t()) == 1, "rank inverse");
        for (auto& value : matrix[rank]) value = sc_reduce(value * inverse);
        for (size_t i = rank + 1; i < matrix.size(); ++i) {
            const SCBig coefficient = matrix[i][col];
            for (size_t j = col; j < width; ++j)
                matrix[i][j] = sc_reduce(matrix[i][j] - coefficient * matrix[rank][j]);
        }
        ++rank;
    }
    return rank;
}
struct SCClosure {
    size_t full_rank, new_rank, closed_rank, baseline_rank;
    SCMat combinations, old_rows;
};
SCClosure sc_close(const SCMat& matrix, size_t old_width, size_t baseline_rank) {
    require(!matrix.empty() && old_width <= matrix[0].size(), "closure shape");
    const size_t count = matrix.size(), width = matrix[0].size();
    require(count <= 256 && width <= 1024, "closure resource cap");
    SCMat work = matrix, provenance(count, SCRow(count));
    for (size_t i = 0; i < count; ++i) {
        require(work[i].size() == width, "closure rectangle");
        provenance[i][i] = 1;
        for (auto& value : work[i]) value = sc_reduce(value);
    }
    size_t rank = 0;
    for (size_t col = old_width; col < width && rank < count; ++col) {
        size_t pivot = rank;
        while (pivot < count && work[pivot][col] == 0) ++pivot;
        if (pivot == count) continue;
        std::swap(work[pivot], work[rank]);
        std::swap(provenance[pivot], provenance[rank]);
        SCBig inverse;
        require(mpz_invert(inverse.get_mpz_t(), work[rank][col].get_mpz_t(), sc_n.get_mpz_t()) == 1, "closure pivot inverse");
        for (auto& value : work[rank]) value = sc_reduce(value * inverse);
        for (auto& value : provenance[rank]) value = sc_reduce(value * inverse);
        for (size_t i = 0; i < count; ++i) if (i != rank) {
            const SCBig coefficient = work[i][col];
            for (size_t j = 0; j < width; ++j)
                work[i][j] = sc_reduce(work[i][j] - coefficient * work[rank][j]);
            for (size_t j = 0; j < count; ++j)
                provenance[i][j] = sc_reduce(provenance[i][j] - coefficient * provenance[rank][j]);
        }
        ++rank;
    }
    SCClosure result{sc_rank(matrix), rank, 0, baseline_rank, {}, {}};
    for (size_t i = rank; i < count; ++i) {
        for (size_t j = old_width; j < width; ++j) require(work[i][j] == 0, "all new columns eliminated");
        result.combinations.push_back(provenance[i]);
        result.old_rows.emplace_back(work[i].begin(), work[i].begin() + static_cast<std::ptrdiff_t>(old_width));
    }
    result.closed_rank = sc_rank(result.old_rows);
    require(result.full_rank == result.new_rank + result.closed_rank && result.closed_rank >= baseline_rank,
            "closure rank identity");
    return result;
}
void sc_emit_row(std::ostream& out, const SCRow& row) {
    out << '[';
    for (size_t i = 0; i < row.size(); ++i) out << (i ? "," : "") << '"' << row[i] << '"';
    out << ']';
}
void sc_emit_matrix(std::ostream& out, const SCMat& matrix) {
    out << '[';
    for (size_t i = 0; i < matrix.size(); ++i) {
        if (i) out << ',';
        sc_emit_row(out, matrix[i]);
    }
    out << ']';
}
void sc_emit_closure(std::ostream& out, const SCClosure& result) {
    out << "{\"full_rank\":" << result.full_rank << ",\"new_rank\":" << result.new_rank
        << ",\"closed_rank\":" << result.closed_rank << ",\"baseline_rank\":" << result.baseline_rank
        << ",\"gain\":" << result.closed_rank - result.baseline_rank << ",\"combinations\":";
    sc_emit_matrix(out, result.combinations);
    out << ",\"old_rows\":";
    sc_emit_matrix(out, result.old_rows);
    out << '}';
}
SCMat sc_read_matrix(const Json& values) {
    SCMat result;
    for (const auto& row : values.arr) {
        SCRow parsed;
        for (const auto& value : row.arr) parsed.emplace_back(value.scalar);
        result.push_back(parsed);
    }
    return result;
}
struct SCRoots { std::string category; std::vector<SCBig> roots; };
SCRoots sc_roots(const std::array<SCBig, 3>& coefficients) {
    const SCBig c = sc_reduce(coefficients[0], sc_p), b = sc_reduce(coefficients[1], sc_p), a = sc_reduce(coefficients[2], sc_p);
    require(a != 0, "genuine residual quadratic");
    const SCBig discriminant = sc_reduce(b * b - 4 * a * c, sc_p), exponent = (sc_p + 1) / 4;
    SCBig square;
    mpz_powm(square.get_mpz_t(), discriminant.get_mpz_t(), exponent.get_mpz_t(), sc_p.get_mpz_t());
    if (sc_reduce(square * square, sc_p) != discriminant) return {"irreducible", {}};
    SCBig inverse, twice = 2 * a;
    require(mpz_invert(inverse.get_mpz_t(), twice.get_mpz_t(), sc_p.get_mpz_t()) == 1, "quadratic denominator");
    SCBig first = sc_reduce((-b + square) * inverse, sc_p), second = sc_reduce((-b - square) * inverse, sc_p);
    std::vector<SCBig> roots{first, second};
    std::sort(roots.begin(), roots.end());
    require(sc_reduce(a * (roots[0] + roots[1]) + b, sc_p) == 0 &&
            sc_reduce(a * roots[0] * roots[1] - c, sc_p) == 0, "residual exact product");
    return {discriminant == 0 ? "repeated" : "distinct", roots};
}
