#pragma once
#include "native_rr.hpp"

namespace cubic_native {
// Prime-field coefficients are intentional: the diagonal certificate and the
// final inversion equations must be over Fp, not the ambient splitting field.
inline uint64_t fp_add(uint64_t a, uint64_t b) { return (a + b) % FP; }
inline uint64_t fp_sub(uint64_t a, uint64_t b) { return (a + FP - b) % FP; }
inline uint64_t fp_mul(uint64_t a, uint64_t b) { return a * b % FP; }
inline uint64_t fp_pow(uint64_t a, uint64_t b) {
  uint64_t r = 1;
  while (b) {
    if (b & 1)
      r = fp_mul(r, a);
    a = fp_mul(a, a);
    b >>= 1;
  }
  return r;
}
template <size_t N>
using PrimePolynomial = std::map<std::array<int, N>, uint64_t>;
template <size_t N>
inline void prime_add(PrimePolynomial<N> &p, const std::array<int, N> &e,
                      uint64_t a) {
  if (!a) {
    return;
  }
  auto it = p.find(e);
  if (it == p.end()) {
    p.emplace(e, a);
  } else {
    it->second = fp_add(it->second, a);
    if (!it->second) {
      p.erase(it);
    }
  }
}
using PrimePlane = PrimePolynomial<2>;
using SixPolynomial = PrimePolynomial<6>;
inline PrimePlane prime_plane(const Polynomial &p) {
  PrimePlane out;
  for (auto &[e, a] : p.terms) {
    require(a.frob() == a, "non-prime section coefficient");
    prime_add(out, std::array<int, 2>{e[0], e[1]}, a.constant());
  }
  return out;
}
inline PrimePlane plane_mul(const PrimePlane &a, const PrimePlane &b) {
  PrimePlane out;
  for (auto &[e, c] : a) {
    for (auto &[f, d] : b) {
      prime_add(out, std::array<int, 2>{e[0] + f[0], e[1] + f[1]},
                fp_mul(c, d));
    }
  }
  return out;
}
inline PrimePlane reduce_plane(PrimePlane p, const PrimePlane &g) {
  auto lead = g.find({0, 4});
  require(lead != g.end() && lead->second, "quartic is not monicizable in y");
  uint64_t unit = fp_pow(lead->second, FP - 2);
  for (;;) {
    auto it = std::find_if(p.begin(), p.end(),
                           [](const auto &term) { return term.first[1] >= 4; });
    if (it == p.end()) {
      break;
    }
    auto e = it->first;
    uint64_t a = fp_mul(it->second, unit);
    p.erase(it);
    for (auto &[f, b] : g) {
      if (f == std::array<int, 2>{0, 4}) {
        continue;
      }
      require(f[1] < 4, "quartic has a second y-leading term");
      prime_add(p, std::array<int, 2>{e[0] + f[0], e[1] - 4 + f[1]},
                fp_sub(0, fp_mul(a, b)));
    }
  }
  return p;
}
struct PrimeKernel {
  size_t rank;
  std::vector<std::vector<uint64_t>> basis;
};
inline PrimeKernel prime_kernel(std::vector<std::vector<uint64_t>> a,
                                size_t columns) {
  size_t rank = 0;
  std::vector<size_t> pivots;
  for (size_t col = 0; col < columns; col++) {
    size_t row = rank;
    while (row < a.size() && !a[row][col]) {
      row++;
    }
    if (row == a.size()) {
      continue;
    }
    std::swap(a[row], a[rank]);
    uint64_t unit = fp_pow(a[rank][col], FP - 2);
    for (auto &x : a[rank]) {
      x = fp_mul(x, unit);
    }
    for (size_t i = 0; i < a.size(); i++) {
      if (i == rank || !a[i][col]) {
        continue;
      }
      uint64_t k = a[i][col];
      for (size_t j = col; j < columns; j++) {
        a[i][j] = fp_sub(a[i][j], fp_mul(k, a[rank][j]));
      }
    }
    pivots.push_back(col);
    rank++;
  }
  std::vector<std::vector<uint64_t>> basis;
  for (size_t col = 0; col < columns; col++) {
    if (std::find(pivots.begin(), pivots.end(), col) != pivots.end()) {
      continue;
    }
    std::vector<uint64_t> v(columns);
    v[col] = 1;
    for (size_t i = 0; i < rank; i++) {
      v[pivots[i]] = fp_sub(0, a[i][col]);
    }
    basis.push_back(v);
  }
  return {rank, basis};
}
struct SymmetricSections {
  std::array<Polynomial, 8> functions;
  std::vector<std::array<int, 3>> triples;
  std::vector<std::vector<uint64_t>> basis;
  std::vector<SixPolynomial> polynomials;
  size_t constraint_rows = 0, rank = 0;
};
inline SymmetricSections symmetric_sections(const Polynomial &g) {
  SymmetricSections s;
  auto X = Polynomial::variable(0), Y = Polynomial::variable(1);
  Polynomial q;
  for (auto &[e, c] : g.terms) {
    if (e[2]) {
      continue;
    }
    require(e[1] >= 2, "tangent infinity section not divisible by y squared");
    q.add({e[0], e[1] - 2, 0}, c);
  }
  require(bool(q.coefficient({2, 0, 0})),
          "ordinary tangent quadratic degenerated");
  s.functions = {Polynomial::constant(F(1)),
                 X,
                 Y,
                 X.pow(2),
                 X * Y,
                 Y.pow(2),
                 X * q,
                 Y * q};
  std::array<PrimePlane, 8> b;
  auto equation = prime_plane(g);
  for (int i = 0; i < 8; i++) {
    b[i] = prime_plane(s.functions[i]);
  }
  for (int i = 0; i < 8; i++) {
    for (int j = i; j < 8; j++) {
      for (int k = j; k < 8; k++) {
        s.triples.push_back({i, j, k});
      }
    }
  }
  require(s.triples.size() == 120, "symmetric cube dimension");
  std::map<std::array<int, 4>, std::vector<uint64_t>> constraints;
  for (size_t column = 0; column < s.triples.size(); column++) {
    auto t = s.triples[column];
    for (int slot = 0; slot < 3; slot++) {
      int i = t[slot], j = t[(slot + 1) % 3], k = t[(slot + 2) % 3];
      auto product = reduce_plane(plane_mul(b[j], b[k]), equation);
      for (auto &[e, a] : product) {
        for (auto &[f, c] : b[i]) {
          auto &row = constraints[{e[0], e[1], f[0], f[1]}];
          if (row.empty()) {
            row.resize(120);
          }
          row[column] = fp_add(row[column], fp_mul(a, c));
        }
      }
    }
  }
  std::vector<std::vector<uint64_t>> rows;
  for (auto &[e, row] : constraints) {
    (void)e;
    rows.push_back(row);
  }
  s.constraint_rows = rows.size();
  auto kernel = prime_kernel(rows, 120);
  s.rank = kernel.rank;
  s.basis = kernel.basis;
  require(s.rank == 112 && s.basis.size() == 8,
          "exact diagonal section kernel must have dimension eight");
  for (auto &v : s.basis) {
    for (auto &row : rows) {
      uint64_t r = 0;
      for (size_t j = 0; j < 120; j++) {
        r = fp_add(r, fp_mul(row[j], v[j]));
      }
      require(r == 0, "section failed exact diagonal certificate");
    }
  }
  s.polynomials.resize(8);
  for (size_t column = 0; column < 120; column++) {
    auto t = s.triples[column];
    std::array<int, 3> permutation{0, 1, 2};
    do {
      for (auto &[e, a] : b[t[0]]) {
        for (auto &[f, c] : b[t[1]]) {
          for (auto &[h, d] : b[t[2]]) {
            std::array<int, 6> powers{};
            powers[2 * permutation[0]] = e[0];
            powers[2 * permutation[0] + 1] = e[1];
            powers[2 * permutation[1]] = f[0];
            powers[2 * permutation[1] + 1] = f[1];
            powers[2 * permutation[2]] = h[0];
            powers[2 * permutation[2] + 1] = h[1];
            uint64_t value = fp_mul(fp_mul(a, c), d);
            for (int i = 0; i < 8; i++) {
              prime_add(s.polynomials[i], powers,
                        fp_mul(value, s.basis[i][column]));
            }
          }
        }
      }
    } while (std::next_permutation(permutation.begin(), permutation.end()));
  }
  return s;
}
inline std::array<F, 8> section_values(const SymmetricSections &s,
                                       const std::array<V3, 3> &points) {
  std::array<std::array<F, 8>, 3> b;
  for (int j = 0; j < 3; j++) {
    for (int i = 0; i < 8; i++) {
      b[j][i] = s.functions[i].eval(points[j]);
    }
  }
  std::array<F, 8> out{};
  for (size_t column = 0; column < 120; column++) {
    auto t = s.triples[column];
    F value;
    std::array<int, 3> permutation{0, 1, 2};
    do {
      value += b[permutation[0]][t[0]] * b[permutation[1]][t[1]] *
               b[permutation[2]][t[2]];
    } while (std::next_permutation(permutation.begin(), permutation.end()));
    for (int i = 0; i < 8; i++) {
      out[i] += F(long(s.basis[i][column])) * value;
    }
  }
  return out;
}
using Matrix8 = std::array<std::array<F, 8>, 8>;
inline Matrix8 invert8(const Matrix8 &input) {
  std::array<std::array<F, 16>, 8> a{};
  for (int i = 0; i < 8; i++) {
    for (int j = 0; j < 8; j++) {
      a[i][j] = input[i][j];
    }
    a[i][8 + i] = 1;
  }
  for (int col = 0; col < 8; col++) {
    int pivot = col;
    while (pivot < 8 && !a[pivot][col]) {
      pivot++;
    }
    require(pivot < 8, "singular section calibration");
    std::swap(a[pivot], a[col]);
    F unit = a[col][col];
    for (auto &x : a[col]) {
      x = x / unit;
    }
    for (int i = 0; i < 8; i++) {
      if (i == col) {
        continue;
      }
      F k = a[i][col];
      for (int j = 0; j < 16; j++) {
        a[i][j] = a[i][j] - k * a[col][j];
      }
    }
  }
  Matrix8 out{};
  for (int i = 0; i < 8; i++) {
    for (int j = 0; j < 8; j++) {
      out[i][j] = a[i][8 + j];
    }
  }
  return out;
}
inline std::array<F, 8> row8(const std::array<F, 8> &v, const Matrix8 &m) {
  std::array<F, 8> out{};
  for (int j = 0; j < 8; j++) {
    for (int i = 0; i < 8; i++) {
      out[j] += v[i] * m[i][j];
    }
  }
  return out;
}
inline std::array<F, 8> divisor_values(const Construction &c,
                                       const RiemannData &rr,
                                       const std::array<V3, 3> &points) {
  M3 coordinates{};
  for (int j = 0; j < 3; j++) {
    require(points[j][2] == F(1), "calibration point outside affine chart");
    coordinates[j] = {F(1), points[j][0], points[j][1]};
  }
  std::array<F, 8> values{};
  values[0] = det(coordinates).pow(2);
  require(bool(values[0]), "calibration determinant pole");
  for (int k = 0; k < 7; k++) {
    auto &fn = rr.functions[k];
    M3 evaluation{};
    F denominator(1);
    for (int j = 0; j < 3; j++) {
      for (int i = 0; i < 3; i++) {
        evaluation[j][i] = fn.numerators[i].eval(points[j]);
      }
      denominator = denominator * dot(c.lines.at(fn.a), points[j]) *
                    dot(c.lines.at(fn.b), points[j]);
    }
    require(bool(denominator), "calibration Weil denominator pole");
    values[k + 1] = det(evaluation).pow(2) / denominator;
  }
  return values;
}
struct Calibration {
  Matrix8 transform;
  int attempts = 0, holdouts = 0;
};
inline Calibration calibrate_sections(const Construction &c,
                                      const RiemannData &rr,
                                      const SymmetricSections &s) {
  std::vector<V3> pool;
  for (int attempt = 0; attempt < 9600 && pool.size() < 96; attempt++) {
    F x;
    for (int j = 0; j < 3; j++) {
      x += c.source.z.pow(j) * F(long(n_randint(FR, FP)));
    }
    Univariate f(5);
    for (auto &[e, a] : c.quartic.terms) {
      f[e[1]] += a * x.pow(e[0]);
    }
    auto ys = roots(f);
    std::vector<F> eligible;
    for (auto &y : ys) {
      if (y.frob(3) == y) {
        eligible.push_back(y);
      }
    }
    if (eligible.empty()) {
      continue;
    }
    pool.push_back({x, eligible[n_randint(FR, eligible.size())], F(1)});
  }
  require(pool.size() == 96, "calibration sample budget exhausted");
  std::vector<std::array<F, 8>> left, right;
  Calibration out;
  for (; out.attempts < 10000 && out.holdouts < 12; out.attempts++) {
    std::array<V3, 3> points;
    for (int j = 0; j < 3; j++) {
      points[j] = pool[n_randint(FR, pool.size())];
    }
    std::array<F, 8> values;
    try {
      values = divisor_values(c, rr, points);
    } catch (const std::exception &) {
      continue;
    }
    auto zz = section_values(s, points);
    if (left.size() < 8) {
      std::vector<std::vector<F>> rows;
      for (auto &v : left) {
        rows.emplace_back(v.begin(), v.end());
      }
      rows.emplace_back(values.begin(), values.end());
      if (8 - nullspace(rows, 8).size() != left.size() + 1) {
        continue;
      }
      left.push_back(values);
      right.push_back(zz);
      if (left.size() == 8) {
        Matrix8 L{}, R{};
        for (int i = 0; i < 8; i++) {
          L[i] = left[i];
          R[i] = right[i];
        }
        auto inverse = invert8(L);
        for (int i = 0; i < 8; i++) {
          out.transform[i] = row8(inverse[i], R);
        }
      }
    } else {
      require(row8(values, out.transform) == zz,
              "independent section calibration holdout failed");
      out.holdouts++;
    }
  }
  require(out.holdouts == 12, "section calibration holdout budget");
  return out;
}
inline std::array<uint64_t, 8> image_coordinates(const Construction &c,
                                                 const RiemannData &rr,
                                                 const Calibration &cal,
                                                 const Point &P) {
  auto mu = source_signature(c, P);
  std::array<F, 8> values{};
  values[0] = 1;
  for (int i = 0; i < 7; i++) {
    values[i + 1] = rr.functions[i].at_zero * mu[i];
  }
  auto image = row8(values, cal.transform);
  std::array<uint64_t, 8> out{};
  bool nonzero = false;
  for (int i = 0; i < 8; i++) {
    require(image[i].frob() == image[i],
            "quotient coordinates failed individual prime-field descent");
    out[i] = image[i].constant();
    nonzero |= out[i] != 0;
  }
  require(nonzero, "zero quotient coordinates");
  return out;
}
} // namespace cubic_native
