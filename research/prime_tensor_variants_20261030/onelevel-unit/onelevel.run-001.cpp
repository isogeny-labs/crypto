#include <gmpxx.h>

#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

// Fixed finite identity checker. No timing code and no cryptographic instance.
using Q = mpq_class;
using Z = mpz_class;
using QPoly = std::vector<Q>;

static void require(bool condition, const std::string& message) {
  if (!condition) throw std::runtime_error(message);
}

static int bit(int mask, int index) { return (mask >> index) & 1; }

static Q quotient(long numerator, long denominator) {
  require(denominator != 0, "zero rational denominator");
  Q result{Z(numerator), Z(denominator)};
  result.canonicalize();
  return result;
}

static QPoly qadd(QPoly a, const QPoly& b) {
  if (a.size() < b.size()) a.resize(b.size());
  for (std::size_t i = 0; i < b.size(); ++i) a[i] += b[i];
  return a;
}

static QPoly qscale(QPoly a, const Q& s) {
  for (Q& x : a) x *= s;
  return a;
}

static QPoly qmul(const QPoly& a, const QPoly& b, std::size_t cap = 1000) {
  QPoly out(std::min(cap, a.size() + b.size() - 1));
  for (std::size_t i = 0; i < a.size(); ++i)
    for (std::size_t j = 0; j < b.size() && i + j < out.size(); ++j)
      out[i + j] += a[i] * b[j];
  return out;
}

static QPoly qchart(const QPoly& f, int t, int alpha) {
  QPoly out{Q(0)};
  for (std::size_t i = f.size(); i-- > 0;) {
    out = qmul(out, QPoly{Q(t), Q(alpha)});
    out[0] += f[i];
  }
  return out;
}

struct Ring {
  std::array<Z, 3> c{};
};

static bool operator==(const Ring& a, const Ring& b) { return a.c == b.c; }

class Arithmetic {
 public:
  explicit Arithmetic(unsigned precision) : precision_(precision) {
    mpz_ui_pow_ui(modulus_.get_mpz_t(), 3, precision_);
  }

  unsigned precision() const { return precision_; }
  const Z& modulus() const { return modulus_; }

  Z reduce(const Z& x) const {
    Z result;
    mpz_mod(result.get_mpz_t(), x.get_mpz_t(), modulus_.get_mpz_t());
    return result;
  }

  Ring normalize(Ring a) const {
    for (Z& x : a.c) x = reduce(x);
    return a;
  }

  Ring scalar(long x) const {
    Ring r;
    r.c[0] = reduce(Z(x));
    return r;
  }

  Ring lift(unsigned index) const {
    require(index < 27, "F27 lift index out of range");
    Ring r;
    for (int i = 0; i < 3; ++i) {
      r.c[static_cast<std::size_t>(i)] = index % 3;
      index /= 3;
    }
    return r;
  }

  Ring add(const Ring& a, const Ring& b) const {
    Ring r;
    for (int i = 0; i < 3; ++i) r.c[i] = reduce(a.c[i] + b.c[i]);
    return r;
  }

  Ring sub(const Ring& a, const Ring& b) const {
    Ring r;
    for (int i = 0; i < 3; ++i) r.c[i] = reduce(a.c[i] - b.c[i]);
    return r;
  }

  Ring neg(const Ring& a) const { return sub(scalar(0), a); }

  Ring mul(const Ring& a, const Ring& b) const {
    std::array<Z, 5> d{};
    for (int i = 0; i < 3; ++i)
      for (int j = 0; j < 3; ++j) d[i + j] += a.c[i] * b.c[j];
    // U^3=U+1 and U^4=U^2+U.
    Ring r;
    r.c[0] = reduce(d[0] + d[3]);
    r.c[1] = reduce(d[1] + d[3] + d[4]);
    r.c[2] = reduce(d[2] + d[4]);
    return r;
  }

  bool nonzero_residue(const Ring& a) const {
    for (const Z& x : a.c)
      if (mpz_fdiv_ui(x.get_mpz_t(), 3) != 0) return true;
    return false;
  }

  Ring inverse(const Ring& a) const {
    if (!nonzero_residue(a)) throw std::domain_error("nonunit inverse rejected");
    Arithmetic residue(1);
    Ring ar = residue.normalize(a);
    Ring y;
    bool found = false;
    for (unsigned k = 1; k < 27; ++k) {
      Ring candidate = residue.lift(k);
      if (residue.mul(ar, candidate) == residue.scalar(1)) {
        y = candidate;
        found = true;
        break;
      }
    }
    require(found, "nonzero F27 element has no inverse");
    for (unsigned known = 1; known < precision_; known *= 2)
      y = mul(y, sub(scalar(2), mul(a, y)));
    require(mul(a, y) == scalar(1), "Newton inverse failed full precision");
    return y;
  }

  Ring rational(const Q& q) const {
    // mpq values are canonical: their common factors, including powers of 3,
    // have already been cancelled. Keep an explicit cancellation/rejection
    // path, rather than inverting the nonunit C'(0).
    Z numerator = q.get_num();
    Z denominator = q.get_den();
    while (mpz_divisible_ui_p(numerator.get_mpz_t(), 3) &&
           mpz_divisible_ui_p(denominator.get_mpz_t(), 3)) {
      mpz_divexact_ui(numerator.get_mpz_t(), numerator.get_mpz_t(), 3);
      mpz_divexact_ui(denominator.get_mpz_t(), denominator.get_mpz_t(), 3);
    }
    if (mpz_divisible_ui_p(denominator.get_mpz_t(), 3))
      throw std::domain_error("non-three-integral rational rejected");
    Z denominator_inverse;
    require(mpz_invert(denominator_inverse.get_mpz_t(), denominator.get_mpz_t(),
                       modulus_.get_mpz_t()) != 0,
            "unit rational denominator inversion failed");
    Ring result;
    result.c[0] = reduce(numerator * denominator_inverse);
    return result;
  }

 private:
  unsigned precision_;
  Z modulus_;
};

using RPoly = std::vector<Ring>;
using Matrix = std::array<std::array<Ring, 3>, 3>;
using QKernels = std::array<QPoly, 5>;  // indexed by delta+2

static Ring evaluate(const Arithmetic& a, const RPoly& f, const Ring& z) {
  Ring result = a.scalar(0);
  for (std::size_t i = f.size(); i-- > 0;)
    result = a.add(a.mul(result, z), f[i]);
  return result;
}

static RPoly convert(const Arithmetic& a, const QPoly& f, std::size_t size) {
  require(f.size() <= size || f.back() == 0, "polynomial degree exceeds cap");
  RPoly result(size, a.scalar(0));
  for (std::size_t i = 0; i < f.size(); ++i) {
    if (i < size)
      result[i] = a.rational(f[i]);
    else
      require(f[i] == 0, "discarding nonzero high coefficient");
  }
  return result;
}

static QKernels kernels() {
  QKernels out;
  for (int delta = -2; delta <= 2; ++delta) {
    QPoly f{Q(1)};
    for (int s = -3; s <= 5; ++s)
      if (s != delta) f = qmul(f, QPoly{Q(-s), Q(1)});
    out[delta + 2] = qscale(f, Q(-1, 720));
    require(out[delta + 2].size() == 9, "wrong kernel degree");
  }
  return out;
}

static QPoly exact_global(const QKernels& w, int pin_x, int pin_y) {
  QPoly f{Q(1)};
  for (int x = 0; x < 3; ++x) {
    QPoly row{Q(0)};
    for (int y = 0; y < 3; ++y)
      if (bit(pin_x, y)) row = qadd(row, w[y - x + 2]);
    if (!bit(pin_y, x)) {
      row = qscale(row, Q(-1));
      row[0] += 1;
    }
    f = qmul(f, row);
  }
  require(f.size() <= 25, "exact global degree exceeds 24");
  return f;
}

static int original_direct(int pin_x, int pin_y) {
  int result = 0;
  for (int xm = 0; xm < 8; ++xm)
    for (int ym = 0; ym < 8; ++ym) {
      int weight = 1;
      for (int k = 0; k < 3; ++k) {
        weight *= bit(xm, k) == bit(pin_x, k);
        weight *= bit(ym, k) == bit(pin_y, k);
        weight *= bit(xm, k) == bit(ym, k);
      }
      result += weight;
    }
  return result;
}

struct Counters {
  std::uint64_t evaluations = 0;
  std::uint64_t mode_patterns = 0;
  std::uint64_t child_contractions = 0;
  std::uint64_t child_assignments = 0;
  std::uint64_t empty_group_factors = 0;
  std::uint64_t direct_assignments = 0;
  std::uint64_t coefficient_comparisons = 0;
};

static Ring direct_deformed(const Arithmetic& a, const Matrix& w, int pin_x,
                            int pin_y, Counters* counters) {
  Ring result = a.scalar(0);
  for (int xm = 0; xm < 8; ++xm)
    for (int ym = 0; ym < 8; ++ym) {
      int unary_weight = 1;
      for (int k = 0; k < 3; ++k) {
        unary_weight *= bit(xm, k) == bit(pin_x, k);
        unary_weight *= bit(ym, k) == bit(pin_y, k);
      }
      Ring packet = a.scalar(1);
      for (int x = 0; x < 3; ++x) {
        Ring row = a.scalar(0);
        for (int y = 0; y < 3; ++y)
          row = a.add(row, a.mul(w[x][y], a.scalar(bit(xm, y))));
        if (!bit(ym, x)) row = a.sub(a.scalar(1), row);
        packet = a.mul(packet, row);
      }
      result = a.add(result, a.mul(a.scalar(unary_weight), packet));
      if (counters) ++counters->direct_assignments;
    }
  return result;
}

struct Groups {
  std::array<int, 3> row;
  std::array<int, 3> col;
};

static Groups groups_for(int t) {
  int c = ((t % 3) + 3) % 3;
  require(c == 1 || c == 2, "inactive node passed to grouping");
  if (c == 1) return Groups{{0, 4, 1}, {1, 0, 4}};
  return Groups{{0, 1, 4}, {1, 4, 0}};
}

static int index_in(const std::array<int, 3>& list, int group) {
  for (int k = 0; k < 3; ++k)
    if (list[k] == group) return k;
  return -1;
}

static Ring mode_children(const Arithmetic& a, const Matrix& w, const Groups& g,
                          int pin_x, int pin_y, Counters* counters) {
  std::array<std::array<int, 2>, 3> crossing{};
  for (int x = 0; x < 3; ++x) {
    int pos = 0;
    for (int y = 0; y < 3; ++y)
      if (g.row[x] != g.col[y]) crossing[x][pos++] = y;
    require(pos == 2, "target row must have two crossing modes");
  }
  Ring sum = a.scalar(0);
  for (int pattern = 0; pattern < 27; ++pattern) {
    int digits = pattern;
    std::array<int, 3> selected{{-1, -1, -1}};
    std::array<std::array<int, 3>, 3> beta{};
    for (int x = 0; x < 3; ++x) {
      int choice = digits % 3;
      digits /= 3;
      if (choice != 0) {
        selected[x] = crossing[x][choice - 1];
        beta[x][selected[x]] = 1;  // target x AND source column
      }
    }
    Ring product = a.scalar(1);
    for (int group = 0; group < 5; ++group) {
      int x = index_in(g.row, group);
      int y = index_in(g.col, group);
      if (x == -1 || y == -1) {
        require(x == -1 && y == -1, "unexpected one-sided group");
        if (counters) ++counters->empty_group_factors;
        continue;
      }
      require(g.row[x] == g.col[y], "child local edge labels disagree");
      Ring child = a.scalar(0);
      // Actual two-edge child, with retained unary pin tensors. The packet
      // entry is evaluated for every one of its four Boolean assignments.
      for (int ybit = 0; ybit < 2; ++ybit)
        for (int xbit = 0; xbit < 2; ++xbit) {
          Ring packet;
          if (selected[x] == -1) {
            packet = a.mul(w[x][y], a.scalar(xbit));
            if (!ybit) packet = a.sub(a.scalar(1), packet);
          } else {
            packet = w[x][selected[x]];
            if (!ybit) packet = a.neg(packet);
          }
          for (int target = 0; target < 3; ++target)
            if (beta[target][y]) packet = a.mul(packet, a.scalar(xbit));
          int old_unary = (ybit == bit(pin_y, x)) * (xbit == bit(pin_x, y));
          child = a.add(child, a.mul(a.scalar(old_unary), packet));
          if (counters) ++counters->child_assignments;
        }
      product = a.mul(product, child);
      if (counters) ++counters->child_contractions;
    }
    sum = a.add(sum, product);
    if (counters) ++counters->mode_patterns;
  }
  if (counters) ++counters->evaluations;
  return sum;
}

// Deliberately wrong D.16 factorization: one source variable's exponent is
// imposed separately at both target groups. Keep this regression in the unit.
static Ring faulty_unsplit(const Arithmetic& a, const Matrix& w,
                            const Groups& g, int pin_x, int pin_y) {
  Ring total = a.scalar(0);
  for (int beta = 0; beta < 8; ++beta) {
    Ring product = a.scalar(1);
    for (int group : {0, 1, 4}) {
      int row = index_in(g.row, group);
      int col = index_in(g.col, group);
      int count = 0;
      int source = -1;
      for (int j = 0; j < 3; ++j)
        if (g.col[j] != group && bit(beta, j)) {
          ++count;
          source = j;
        }
      Ring child = a.scalar(0);
      for (int ybit = 0; ybit < 2; ++ybit)
        for (int xbit = 0; xbit < 2; ++xbit) {
          Ring packet = a.scalar(0);
          if (count == 0) {
            packet = a.mul(w[row][col], a.scalar(xbit));
            if (!ybit) packet = a.sub(a.scalar(1), packet);
          } else if (count == 1) {
            packet = w[row][source];
            if (!ybit) packet = a.neg(packet);
          }
          if (bit(beta, col)) packet = a.mul(packet, a.scalar(xbit));
          int unary = (ybit == bit(pin_y, row)) * (xbit == bit(pin_x, col));
          child = a.add(child, a.mul(a.scalar(unary), packet));
        }
      product = a.mul(product, child);
    }
    total = a.add(total, product);
  }
  return total;
}

class Interpolator {
 public:
  explicit Interpolator(const Arithmetic& arithmetic) : a(arithmetic) {
    for (unsigned k = 0; k < 25; ++k) points.push_back(a.lift(k));
    for (int order = 1; order < 25; ++order)
      for (int j = order; j < 25; ++j)
        inv_diff[order][j] = a.inverse(a.sub(points[j], points[j - order]));
  }

  RPoly interpolate(RPoly divided) const {
    require(divided.size() == 25, "wrong interpolation input size");
    for (int order = 1; order < 25; ++order)
      for (int j = 24; j >= order; --j)
        divided[j] = a.mul(a.sub(divided[j], divided[j - 1]), inv_diff[order][j]);
    RPoly answer(25, a.scalar(0));
    RPoly basis{a.scalar(1)};
    for (int k = 0; k < 25; ++k) {
      for (std::size_t j = 0; j < basis.size(); ++j)
        answer[j] = a.add(answer[j], a.mul(divided[k], basis[j]));
      RPoly next(basis.size() + 1, a.scalar(0));
      for (std::size_t j = 0; j < basis.size(); ++j) {
        next[j] = a.sub(next[j], a.mul(points[k], basis[j]));
        next[j + 1] = a.add(next[j + 1], basis[j]);
      }
      basis = std::move(next);
    }
    return answer;
  }

  const Arithmetic& a;
  std::vector<Ring> points;
  std::array<std::array<Ring, 25>, 25> inv_diff{};
};

constexpr std::array<int, 6> active{{-2, -1, 1, 2, 4, 5}};

struct HermiteWeights {
  std::array<std::array<Q, 6>, 6> weight{};

  HermiteWeights() {
    for (int k = 0; k < 6; ++k) {
      int t = active[k];
      Q e0(1);
      QPoly local{Q(1)};
      for (int other : active) {
        if (other == t) continue;
        for (int repetition = 0; repetition < 6; ++repetition) {
          e0 *= quotient(-other, t - other);
          local = qmul(local, QPoly{Q(1), quotient(9, t - other)}, 6);
        }
      }
      QPoly reciprocal(6);
      reciprocal[0] = 1;
      for (int j = 1; j < 6; ++j)
        for (int i = 1; i <= j; ++i) reciprocal[j] -= local[i] * reciprocal[j - i];
      std::array<Q, 6> powers{};
      powers[0] = 1;
      for (int j = 1; j < 6; ++j) powers[j] = powers[j - 1] * Q(-t, 9);
      // Linear D.9 functional on the first six local coefficients.
      for (int coefficient = 0; coefficient < 6; ++coefficient)
        for (int j = coefficient; j < 6; ++j)
          weight[k][coefficient] += e0 * reciprocal[j - coefficient] * powers[j];
    }
  }

  std::array<Q, 3> recover(const std::array<RPoly, 6>& charts) const {
    std::array<Q, 3> answer{};
    for (int t = 0; t < 6; ++t)
      for (int j = 0; j < 6; ++j)
        for (int coordinate = 0; coordinate < 3; ++coordinate)
          answer[coordinate] += weight[t][j] * Q(charts[t][j].c[coordinate]);
    return answer;
  }

  Q recover_exact(const std::array<QPoly, 6>& charts) const {
    Q answer(0);
    for (int t = 0; t < 6; ++t)
      for (int j = 0; j < 6; ++j)
        if (static_cast<std::size_t>(j) < charts[t].size())
          answer += weight[t][j] * charts[t][j];
    return answer;
  }
};

static std::pair<int, int> s_action(std::pair<int, int> p, bool inverse) {
  int neg_sum = ((-p.first - p.second) % 5 + 5) % 5;
  if (inverse) return {p.second, neg_sum};
  return {neg_sum, p.first};
}

static void controls(const QKernels& w, std::ostream& out) {
  Arithmetic residue(1);
  for (int value = 0; value < 3; ++value)
    require(((value * value * value - value - 1) % 3 + 3) % 3 != 0,
            "extension polynomial has F3 root");
  for (unsigned i = 0; i < 27; ++i)
    for (unsigned j = i + 1; j < 27; ++j)
      require(residue.nonzero_residue(residue.sub(residue.lift(i), residue.lift(j))),
              "duplicate canonical residue lift");
  out << "extension_no_F3_root\t3\tPASS\n";
  out << "all_F27_lift_pairs_distinct\t351\tPASS\n";

  unsigned inversions = 0;
  unsigned pair_differences = 0;
  unsigned rejected = 0;
  for (unsigned precision : {121U, 122U}) {
    Arithmetic a(precision);
    for (unsigned index = 1; index < 27; ++index) {
      Ring element = a.lift(index);
      require(a.mul(element, a.inverse(element)) == a.scalar(1), "unit inverse control");
      ++inversions;
    }
    for (unsigned i = 0; i < 25; ++i)
      for (unsigned j = i + 1; j < 25; ++j) {
        Ring difference = a.sub(a.lift(j), a.lift(i));
        require(a.mul(difference, a.inverse(difference)) == a.scalar(1),
                "interpolation difference not invertible");
        ++pair_differences;
      }
    for (long nonunit : {0L, 3L}) {
      bool failed = false;
      try {
        static_cast<void>(a.inverse(a.scalar(nonunit)));
      } catch (const std::domain_error&) {
        failed = true;
      }
      require(failed, "nonunit inversion was accepted");
      ++rejected;
    }
    bool failed = false;
    try {
      static_cast<void>(a.rational(Q(1, 3)));
    } catch (const std::domain_error&) {
      failed = true;
    }
    require(failed, "nonintegral rational was accepted");
    ++rejected;
  }
  out << "F27_Newton_unit_inversions\t" << inversions << "\tPASS\n";
  out << "interpolation_unit_difference_inversions\t" << pair_differences << "\tPASS\n";
  out << "nonunit_and_nonintegral_rejections\t" << rejected << "\tPASS\n";

  std::array<std::pair<int, int>, 3> c1{};
  std::array<std::pair<int, int>, 3> c2{};
  c1[0] = c2[0] = {0, 1};
  for (int x = 1; x < 3; ++x) {
    c1[x] = s_action(c1[x - 1], false);
    c2[x] = s_action(c2[x - 1], true);
  }
  for (int x = 0; x < 3; ++x) {
    require(c1[x].first == groups_for(1).row[x] &&
                c1[x].second == groups_for(1).col[x], "component 1 action mismatch");
    require(c2[x].first == groups_for(2).row[x] &&
                c2[x].second == groups_for(2).col[x], "component 2 action mismatch");
  }
  require(s_action(c1[2], false) == c1[0] && s_action(c2[2], true) == c2[0],
          "slot action does not have order three");
  require(c1[2].second != c1[0].first,
          "wrong component-2 S direction was not detected at t=2,x=0,y=2");
  out << "slot_action_and_wrong_direction_control\t7\tPASS\n";

  unsigned identities = 0;
  unsigned integral_coefficients = 0;
  unsigned localities = 0;
  for (int x = 0; x < 3; ++x)
    for (int y = 0; y < 3; ++y) {
      require(w[y - x + 2][0] == Q(x == y), "W(0) is not identity");
      ++identities;
    }
  Arithmetic a(3);
  for (int t : active) {
    Groups g = groups_for(t);
    for (int x = 0; x < 3; ++x)
      for (int y = 0; y < 3; ++y) {
        QPoly f = qchart(w[y - x + 2], t, 9);
        for (const Q& coefficient : f) {
          static_cast<void>(a.rational(coefficient));
          ++integral_coefficients;
        }
        if (g.row[x] != g.col[y]) {
          require(f[0] == 0, "W(t) crossing coefficient is nonzero");
          ++localities;
        }
      }
  }
  out << "W_at_zero_identity\t" << identities << "\tPASS\n";
  out << "kernel_chart_integral_coefficients\t" << integral_coefficients << "\tPASS\n";
  out << "crossing_locality_at_active_nodes\t" << localities << "\tPASS\n";
  Matrix regression{};
  Ring zeta = a.scalar(1);
  for (int x = 0; x < 3; ++x)
    for (int y = 0; y < 3; ++y)
      regression[x][y] = evaluate(a, convert(a, qchart(w[y - x + 2], 1, 9), 9), zeta);
  Ring correct = mode_children(a, regression, groups_for(1), 1, 1, nullptr);
  Ring wrong = faulty_unsplit(a, regression, groups_for(1), 1, 1);
  Ring direct = direct_deformed(a, regression, 1, 1, nullptr);
  require(correct == a.scalar(9) && correct == direct && wrong == a.scalar(0),
          "target-specific/unsplit regression does not distinguish formulas");
  out << "target_split_regression_mod27_correct9_faulty0\t1\tPASS\n";
}

int main(int argc, char** argv) {
  try {
    require(argc == 2, "usage: onelevel OUTPUT_DIRECTORY");
    const std::string output = argv[1];
    std::ofstream controls_out(output + "/controls.tsv");
    std::ofstream cases_out(output + "/cases.tsv");
    std::ofstream summary_out(output + "/summary.tsv");
    require(controls_out.good() && cases_out.good() && summary_out.good(),
            "cannot open output files");
    QKernels w = kernels();
    controls(w, controls_out);
    HermiteWeights hermite;
    cases_out << "root_precision\tchild_precision\tpin_X\tpin_Y\texpected\trecovered\t"
                 "mode_evaluations\tchart_coefficients\texact_Hermite\tstatus\n";
    Counters counters;
    unsigned case_count = 0;
    unsigned ones = 0;
    unsigned hermite_checks = 0;
    for (unsigned p : {1U, 2U}) {
      Arithmetic a(p + 120);
      Arithmetic root(p);
      Interpolator interpolation(a);
      std::array<std::array<Matrix, 25>, 6> values{};
      for (int t_index = 0; t_index < 6; ++t_index)
        for (int x = 0; x < 3; ++x)
          for (int y = 0; y < 3; ++y) {
            RPoly local_kernel = convert(a, qchart(w[y - x + 2], active[t_index], 9), 9);
            for (int k = 0; k < 25; ++k)
              values[t_index][k][x][y] = evaluate(a, local_kernel, interpolation.points[k]);
          }
      for (int pin_x = 0; pin_x < 8; ++pin_x)
        for (int pin_y = 0; pin_y < 8; ++pin_y) {
          int expected = original_direct(pin_x, pin_y);
          require(expected == (pin_x == pin_y), "original direct contraction wrong");
          QPoly global = exact_global(w, pin_x, pin_y);
          require(global[0] == Q(expected), "exact global constant wrong");
          std::array<QPoly, 6> exact_charts{};
          std::array<RPoly, 6> recovered_charts{};
          for (int t_index = 0; t_index < 6; ++t_index) {
            int t = active[t_index];
            exact_charts[t_index] = qchart(global, t, 9);
            RPoly exact_mod = convert(a, exact_charts[t_index], 25);
            RPoly sampled;
            for (int k = 0; k < 25; ++k) {
              Ring from_children = mode_children(a, values[t_index][k], groups_for(t),
                                                 pin_x, pin_y, &counters);
              Ring from_direct = direct_deformed(a, values[t_index][k], pin_x, pin_y, &counters);
              require(from_children == from_direct,
                      "mode/child vs direct mismatch at p=" + std::to_string(p) +
                          " X=" + std::to_string(pin_x) + " Y=" + std::to_string(pin_y) +
                          " t=" + std::to_string(t) + " point=" + std::to_string(k));
              require(from_children == evaluate(a, exact_mod, interpolation.points[k]),
                      "child result differs from exact rational chart");
              sampled.push_back(from_children);
            }
            recovered_charts[t_index] = interpolation.interpolate(sampled);
            for (int coefficient = 0; coefficient < 25; ++coefficient) {
              require(recovered_charts[t_index][coefficient] == exact_mod[coefficient],
                      "interpolation coefficient differs from exact chart");
              ++counters.coefficient_comparisons;
            }
          }
          require(hermite.recover_exact(exact_charts) == Q(expected),
                  "exact rational Hermite functional failed");
          std::array<Q, 3> lifted_sum = hermite.recover(recovered_charts);
          Ring recovered;
          for (int coordinate = 0; coordinate < 3; ++coordinate)
            recovered.c[coordinate] = root.rational(lifted_sum[coordinate]).c[0];
          require(recovered == root.scalar(expected), "canonical lifted Hermite recovery failed");
          ++hermite_checks;
          ++case_count;
          ones += expected;
          cases_out << p << '\t' << p + 120 << '\t' << pin_x << '\t' << pin_y << '\t'
                    << expected << '\t' << recovered.c[0] << "\t150\t150\tPASS\tPASS\n";
          cases_out.flush();
        }
    }
    require(case_count == 128 && ones == 16, "panel size or exact ones count wrong");
    require(counters.evaluations == 19200 && counters.mode_patterns == 518400 &&
                counters.child_contractions == 1555200 && counters.child_assignments == 6220800 &&
                counters.empty_group_factors == 1036800 && counters.direct_assignments == 1228800 &&
                counters.coefficient_comparisons == 19200,
            "exact structural coverage counters wrong");
    summary_out << "metric\tvalue\nroot_cases\t" << case_count << "\nroot_ones\t" << ones
                << "\nroot_zeros\t" << case_count - ones << "\nactive_nodes_per_case\t6\n"
                   "points_per_chart\t25\nmode_evaluations\t" << counters.evaluations
                << "\nrow_choice_patterns\t" << counters.mode_patterns
                << "\nactual_two_edge_child_contractions\t" << counters.child_contractions
                << "\nactual_child_assignments\t" << counters.child_assignments
                << "\nempty_group_unit_factors\t" << counters.empty_group_factors
                << "\ndirect_six_bit_assignments\t" << counters.direct_assignments
                << "\nfull_chart_coefficient_comparisons\t" << counters.coefficient_comparisons
                << "\nexact_and_lifted_Hermite_cases\t" << hermite_checks
                << "\nstatus\tPASS\n";
    std::cout << "PASS: 128 finite root cases; 19200 mode/direct/exact-chart comparisons; "
                 "all coefficient and exact Hermite controls.\n";
    return 0;
  } catch (const std::exception& error) {
    std::cerr << "FAIL: " << error.what() << '\n';
    return 1;
  }
}
