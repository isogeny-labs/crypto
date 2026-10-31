// SPDX-License-Identifier: LGPL-3.0-or-later
// Native semantic port of keeganryan/cuso fb2747d, with Sage 10.9
// comparator/echelon semantics.
#pragma once
#include <algorithm>
#include <deque>
#include <functional>
#include <gmpxx.h>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace jsr {
inline bool exact_memory_limit(std::string s) {
  const auto first = s.find_first_not_of(" \t\r\n");
  if (first == std::string::npos)
    return false;
  const auto last = s.find_last_not_of(" \t\r\n");
  return s.substr(first, last - first + 1) == "8589934592";
}
using Q = mpq_class;
using Z = mpz_class;
using E = std::vector<int>;
inline void require(bool b, const std::string &s) {
  if (!b)
    throw std::runtime_error(s);
}
inline Q rational(const std::string &s) {
  Q q;
  require(q.set_str(s, 10) == 0, "bad rational");
  require(q.get_den() != 0, "zero denominator");
  q.canonicalize();
  return q;
}
inline std::string qstr(const Q &q) {
  return q.get_num().get_str() + "/" + q.get_den().get_str();
}
inline std::string estr(const E &e) {
  std::string s;
  for (int x : e) {
    if (!s.empty())
      s += ",";
    s += std::to_string(x);
  }
  return s;
}
inline E exponent(const std::string &s) {
  std::istringstream in(s);
  std::string x;
  E e;
  while (std::getline(in, x, ',')) {
    size_t n = 0;
    long v = std::stol(x, &n);
    require(n == x.size() && v >= 0 && v <= 2147483647L, "bad exponent");
    e.push_back(static_cast<int>(v));
  }
  require(!e.empty(), "empty exponent");
  return e;
}
struct Order {
  E w;
  int cmp(const E &a, const E &b) const {
    require(a.size() == w.size() && b.size() == w.size(), "order arity");
    Z x = 0, y = 0;
    for (size_t i = 0; i < w.size(); ++i) {
      x += Z(w[i]) * a[i];
      y += Z(w[i]) * b[i];
    }
    if (x != y)
      return x < y ? -1 : 1;
    return a == b ? 0 : (a < b ? -1 : 1);
  }
  bool operator()(const E &a, const E &b) const { return cmp(a, b) < 0; }
};
struct Poly {
  size_t n;
  std::map<E, Q> t;
  explicit Poly(size_t n_ = 7) : n(n_) {}
  static Poly constant(size_t n, const Q &q) {
    Poly p(n);
    p.add(E(n, 0), q);
    return p;
  }
  static Poly mono(const E &e, const Q &q = 1) {
    Poly p(e.size());
    p.add(e, q);
    return p;
  }
  static Poly variable(size_t n, size_t i) {
    E e(n, 0);
    e.at(i) = 1;
    return mono(e);
  }
  void add(const E &e, Q q) {
    require(e.size() == n, "polynomial arity");
    for (int v : e)
      require(v >= 0, "negative exponent");
    q.canonicalize();
    if (q == 0)
      return;
    auto j = t.find(e);
    if (j == t.end())
      t.emplace(e, q);
    else {
      j->second += q;
      if (j->second == 0)
        t.erase(j);
    }
  }
  bool zero() const { return t.empty(); }
  bool constant() const {
    return zero() || (t.size() == 1 && t.begin()->first == E(n, 0));
  }
  Q scalar() const {
    require(constant(), "nonconstant scalar");
    return zero() ? Q(0) : t.begin()->second;
  }
  std::vector<std::pair<E, Q>> terms(const Order &o) const {
    std::vector<std::pair<E, Q>> v(t.begin(), t.end());
    std::stable_sort(v.begin(), v.end(), [&](auto &a, auto &b) {
      return o.cmp(a.first, b.first) > 0;
    });
    return v;
  }
  Poly lt(const Order &o) const {
    require(!zero(), "zero leading term");
    auto v = terms(o);
    return mono(v.front().first, v.front().second);
  }
  std::string codec(const Order &o) const {
    std::string s;
    for (auto &x : terms(o))
      s += qstr(x.second) + ":" + estr(x.first) + "\n";
    return s;
  }
  std::string singular(const std::vector<std::string> &names,
                       const Order &o) const {
    require(names.size() == n, "names arity");
    if (zero())
      return "0";
    std::string s;
    for (auto &x : terms(o)) {
      if (!s.empty())
        s += "+";
      s += "(" + qstr(x.second) + ")";
      for (size_t i = 0; i < n; ++i)
        if (x.first[i])
          s += "*" + names[i] + "^" + std::to_string(x.first[i]);
    }
    return s;
  }
  bool operator==(const Poly &b) const { return n == b.n && t == b.t; }
};
inline Poly operator+(Poly a, const Poly &b) {
  require(a.n == b.n, "sum arity");
  for (auto &x : b.t)
    a.add(x.first, x.second);
  return a;
}
inline Poly operator-(Poly a, const Poly &b) {
  require(a.n == b.n, "difference arity");
  for (auto &x : b.t)
    a.add(x.first, -x.second);
  return a;
}
inline Poly operator*(const Poly &a, const Poly &b) {
  require(a.n == b.n, "product arity");
  Poly p(a.n);
  for (auto &x : a.t)
    for (auto &y : b.t) {
      E e(a.n);
      for (size_t i = 0; i < a.n; ++i) {
        require(x.first[i] <= 2147483647 - y.first[i], "exponent overflow");
        e[i] = x.first[i] + y.first[i];
      }
      p.add(e, x.second * y.second);
    }
  return p;
}
inline Poly scale(Poly a, const Q &q) {
  for (auto &x : a.t)
    x.second *= q;
  if (q == 0)
    a.t.clear();
  return a;
}
inline Poly power(Poly a, unsigned n) {
  Poly p = Poly::constant(a.n, 1);
  while (n) {
    if (n & 1)
      p = p * a;
    a = a * a;
    n >>= 1;
  }
  return p;
}
// Literal Sage singular_polynomial_cmp: missing monomials compare by
// coefficient sign.
inline int compare(const Poly &a, const Poly &b, const Order &o) {
  auto x = a.terms(o), y = b.terms(o);
  size_t i = 0, j = 0;
  while (i < x.size() || j < y.size()) {
    if (i == x.size())
      return y[j].second > 0 ? -1 : 1;
    if (j == y.size())
      return x[i].second > 0 ? 1 : -1;
    int c = o.cmp(x[i].first, y[j].first);
    if (c < 0)
      return y[j].second > 0 ? -1 : 1;
    if (c > 0)
      return x[i].second > 0 ? 1 : -1;
    if (x[i].second != y[j].second)
      return x[i].second < y[j].second ? -1 : 1;
    ++i;
    ++j;
  }
  return 0;
}
inline E strip(E e) {
  e.at(0) = 0;
  return e;
}
inline std::pair<Poly, E> lt_no_p(const Poly &f, const Order &o) {
  E best(f.n, 0);
  Poly c(f.n);
  bool have = false;
  for (auto &x : f.terms(o)) {
    E m = strip(x.first);
    if (!have || o.cmp(m, best) > 0) {
      best = m;
      E p(f.n, 0);
      p[0] = x.first[0];
      c = Poly::mono(p, x.second);
      have = true;
    }
  }
  require(have, "lt_no_p zero polynomial");
  return {c, best};
}
inline Poly coeff_no_p(const Poly &f, const E &m) {
  Poly p(f.n);
  for (auto &x : f.t)
    if (strip(x.first) == m) {
      E e = x.first;
      for (size_t i = 1; i < f.n; ++i)
        e[i] -= m[i];
      p.add(e, x.second);
    }
  return p;
}
inline bool divides(const E &a, const E &b) {
  require(a.size() == b.size(), "division arity");
  for (size_t i = 0; i < a.size(); ++i)
    if (a[i] > b[i])
      return false;
  return true;
}
inline E difference(E a, const E &b) {
  require(divides(b, a), "nondividing monomial");
  for (size_t i = 0; i < a.size(); ++i)
    a[i] -= b[i];
  return a;
}
inline Poly parse_codec(const std::string &s, size_t n) {
  Poly p(n);
  std::istringstream in(s);
  std::string line;
  while (std::getline(in, line)) {
    if (line.empty())
      continue;
    auto k = line.find(':');
    require(k != std::string::npos, "term codec");
    p.add(exponent(line.substr(k + 1)), rational(line.substr(0, k)));
  }
  return p;
}
using Matrix = std::vector<std::vector<Poly>>;
// Sage rescales pivot starting at c, then swaps, eliminates from all rows
// starting at0.
inline void constant_pivot(Matrix &B, std::vector<std::string> &trace) {
  if (B.empty())
    return;
  size_t nr = B.size(), nc = B[0].size(), start = 0;
  for (auto &r : B)
    require(r.size() == nc, "ragged matrix");
  for (size_t c = 0; c < nc; ++c) {
    size_t r = start;
    while (r < nr && (!B[r][c].constant() || B[r][c].zero()))
      ++r;
    if (r == nr) {
      trace.push_back("skip column=" + std::to_string(c));
      continue;
    }
    Q inv = 1 / B[r][c].scalar();
    trace.push_back("pivot column=" + std::to_string(c) +
                    " row=" + std::to_string(r) +
                    " start=" + std::to_string(start) + " scalar=" + qstr(inv));
    for (size_t j = c; j < nc; ++j)
      B[r][j] = scale(B[r][j], inv);
    if (r != start) {
      std::swap(B[r], B[start]);
      trace.push_back("swap=" + std::to_string(r) + "," +
                      std::to_string(start));
    }
    for (size_t i = 0; i < nr; ++i)
      if (i != start) {
        Poly factor = scale(B[i][c], -1);
        trace.push_back("eliminate row=" + std::to_string(i) +
                        " pivot=" + std::to_string(start) +
                        " factor=" + factor.codec(Order{E(factor.n, 1)}));
        for (size_t j = 0; j < nc; ++j)
          B[i][j] = B[i][j] + factor * B[start][j];
      }
    ++start;
  }
}
struct Closure {
  std::vector<E> M, Mbar;
  std::vector<Poly> Sbar, S;
  Matrix before, after;
  std::vector<std::string> trace;
};
inline Closure optimal(std::vector<E> M, const std::vector<Poly> &G,
                       const Order &o,
                       const std::function<Poly(const Poly &)> &nf,
                       const std::function<void(const Closure &)> &checkpoint) {
  require(!G.empty(), "empty basis");
  (void)lt_no_p(G.back(), o);
  std::stable_sort(M.begin(), M.end(), o);
  Closure c;
  c.M = M;
  c.Mbar = M;
  std::deque<E> q(M.begin(), M.end());
  while (!q.empty()) {
    E m = q.front();
    q.pop_front();
    const Poly *chosen = nullptr;
    E lm;
    for (auto &g : G) {
      auto x = lt_no_p(g, o);
      if (divides(x.second, m) &&
          (!chosen || compare(x.first, lt_no_p(*chosen, o).first, o) < 0)) {
        chosen = &g;
        lm = x.second;
      }
    }
    if (chosen) {
      Poly h = Poly::mono(difference(m, lm)) * (*chosen);
      Poly lt = h.lt(o);
      Poly hp = lt + nf(h - lt);
      c.Sbar.push_back(hp);
      for (auto &x : hp.terms(o)) {
        E e = strip(x.first);
        if (std::find(c.Mbar.begin(), c.Mbar.end(), e) == c.Mbar.end()) {
          c.Mbar.push_back(e);
          q.push_back(e);
        }
      }
    }
    checkpoint(c);
  }
  if (c.M.size() == c.Mbar.size()) {
    c.S = c.Sbar;
    return c;
  }
  for (auto &f : c.Sbar) {
    std::vector<Poly> row;
    for (auto &m : c.Mbar)
      row.push_back(coeff_no_p(f, m));
    c.before.push_back(row);
  }
  c.after = c.before;
  std::reverse(c.after.begin(), c.after.end());
  for (auto &r : c.after)
    std::reverse(r.begin(), r.end());
  constant_pivot(c.after, c.trace);
  std::reverse(c.after.begin(), c.after.end());
  for (auto &r : c.after)
    std::reverse(r.begin(), r.end());
  for (auto &r : c.after) {
    bool ok = true;
    for (size_t j = M.size(); j < r.size(); ++j)
      if (!r[j].zero())
        ok = false;
    if (ok) {
      Poly f(G.front().n);
      for (size_t j = 0; j < M.size(); ++j)
        f = f + r[j] * Poly::mono(M[j]);
      c.S.push_back(f);
    }
  }
  checkpoint(c);
  return c;
}
// Univariate rational polynomials represented low-degree first.
using U = std::vector<Q>;
inline void trim(U &v) {
  while (!v.empty() && v.back() == 0)
    v.pop_back();
}
inline U uadd(U a, const U &b) {
  a.resize(std::max(a.size(), b.size()));
  for (size_t i = 0; i < b.size(); ++i)
    a[i] += b[i];
  trim(a);
  return a;
}
inline U uscale(U a, const Q &c) {
  for (auto &x : a)
    x *= c;
  trim(a);
  return a;
}
inline U umul(const U &a, const U &b) {
  if (a.empty() || b.empty())
    return {};
  U r(a.size() + b.size() - 1);
  for (size_t i = 0; i < a.size(); ++i)
    for (size_t j = 0; j < b.size(); ++j)
      r[i + j] += a[i] * b[j];
  trim(r);
  return r;
}
inline std::pair<U, U> udiv(U a, const U &b) {
  require(!b.empty(), "zero polynomial divisor");
  U q(a.size() >= b.size() ? a.size() - b.size() + 1 : 0);
  while (a.size() >= b.size() && !a.empty()) {
    size_t k = a.size() - b.size();
    Q c = a.back() / b.back();
    q[k] = c;
    for (size_t i = 0; i < b.size(); ++i)
      a[i + k] -= c * b[i];
    trim(a);
  }
  trim(q);
  return {q, a};
}
inline U ugcd(U a, U b) {
  while (!b.empty()) {
    U r = udiv(a, b).second;
    a = b;
    b = r;
  }
  return a.empty() ? a : uscale(a, 1 / a.back());
}
inline U interpolate(const std::vector<Q> &ys) {
  U r;
  for (size_t i = 0; i < ys.size(); ++i) {
    U p{1};
    Q den = 1;
    for (size_t j = 0; j < ys.size(); ++j)
      if (i != j) {
        p = umul(p, {Q(-static_cast<long>(j + 1)), 1});
        den *= Q(static_cast<long>(i) - static_cast<long>(j));
      }
    r = uadd(r, uscale(p, ys[i] / den));
  }
  return r;
}
struct Limit {
  U raw_num, raw_den, gcd, num, den;
  Q delta;
};
inline Limit limit(
    const U &dim, const std::vector<U> &xs, const U &C, const E &weights,
    int mu,
    const std::function<void(const Limit &)> &save = [](const Limit &) {}) {
  require(xs.size() == weights.size() && mu > 0, "limit arity");
  Limit r;
  r.raw_num = uadd(umul({0, 1}, dim), uscale(C, Q(-1, mu)));
  for (size_t i = 0; i < xs.size(); ++i)
    r.raw_den = uadd(r.raw_den, uscale(xs[i], Q(weights[i], mu)));
  save(r);
  require(!r.raw_den.empty(), "zero limit denominator");
  r.gcd = ugcd(r.raw_num, r.raw_den);
  r.num = udiv(r.raw_num, r.gcd).first;
  r.den = udiv(r.raw_den, r.gcd).first;
  save(r);
  require(!r.num.empty() && r.num.size() == r.den.size(),
          "unequal limit degrees or zero numerator");
  r.delta = r.num.back() / r.den.back();
  return r;
}
} // namespace jsr
