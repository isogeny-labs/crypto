// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "cdd_ipc.hpp"
namespace jsr {
struct Facet {
  std::vector<Q> a;
  bool equality;
};
struct Domain {
  std::vector<E> vertices, points;
  std::vector<Facet> facets;
  int dimension;
};
inline int affine_dimension(const std::vector<E> &v) {
  require(!v.empty(), "empty affine hull");
  std::vector<std::vector<Q>> a;
  for (size_t i = 1; i < v.size(); ++i) {
    std::vector<Q> r;
    for (size_t j = 0; j < v[0].size(); ++j)
      r.emplace_back(v[i][j] - v[0][j]);
    a.push_back(r);
  }
  size_t k = 0;
  for (size_t j = 0; j < v[0].size() && k < a.size(); ++j) {
    size_t i = k;
    while (i < a.size() && a[i][j] == 0)
      ++i;
    if (i == a.size())
      continue;
    std::swap(a[i], a[k]);
    Q c = a[k][j];
    for (auto &x : a[k])
      x /= c;
    for (i = k + 1; i < a.size(); ++i) {
      c = a[i][j];
      for (size_t h = j; h < v[0].size(); ++h)
        a[i][h] -= c * a[k][h];
    }
    ++k;
  }
  return static_cast<int>(k);
}
inline bool contains(const std::vector<Facet> &facets, const E &p) {
  for (auto &f : facets) {
    require(f.a.size() == p.size() + 1, "facet arity");
    Q v = f.a[0];
    for (size_t j = 0; j < p.size(); ++j)
      v += f.a[j + 1] * p[j];
    if (f.equality ? v != 0 : v < 0)
      return false;
  }
  return true;
}
inline Domain hull(
    const std::vector<E> &vertices,
    const std::function<void(const Domain &)> &checkpoint = [](const Domain &) {
    }) {
  require(!vertices.empty(), "empty polytope");
  Domain d;
  d.vertices = vertices;
  d.dimension = affine_dimension(vertices);
  size_t n = vertices.front().size();
  for (auto &v : vertices)
    require(v.size() == n, "vertex arity");
  std::istringstream hs(cdd_halfspaces(vertices));
  long rows = 0, dimension = 0;
  hs >> rows >> dimension;
  require(bool(hs) && rows >= 0 && dimension == static_cast<long>(n),
          "cdd IPC header");
  for (long i = 0; i < rows; ++i) {
    int eq = 0;
    hs >> eq;
    require(bool(hs) && (eq == 0 || eq == 1), "cdd equality codec");
    Facet f;
    f.equality = eq;
    for (size_t j = 0; j <= n; ++j) {
      std::string q;
      hs >> q;
      require(bool(hs), "cdd rational codec");
      f.a.push_back(rational(q));
    }
    d.facets.push_back(f);
  }
  std::string trailing;
  require(!(hs >> trailing), "trailing cdd IPC output");
  checkpoint(d);
  E lo(n, 2147483647), hi(n, 0), point(n, 0);
  for (auto &v : vertices)
    for (size_t j = 0; j < n; ++j) {
      lo[j] = std::min(lo[j], v[j]);
      hi[j] = std::max(hi[j], v[j]);
    }
  std::function<void(size_t)> visit = [&](size_t j) {
    if (j == n) {
      if (contains(d.facets, point))
        d.points.push_back(point);
      return;
    }
    for (int x = lo[j];;) {
      point[j] = x;
      visit(j + 1);
      if (x == hi[j])
        break;
      ++x;
    }
  };
  visit(0);
  checkpoint(d);
  return d;
}
struct Aug {
  std::vector<E> points;
  std::map<E, int> values;
  int dim;
};
inline Aug augment(const std::vector<Poly> &S, const Order &o,
                   const std::function<void(const Domain &)> &save) {
  require(!S.empty(), "empty shift set");
  std::vector<E> verts;
  std::map<E, int> lc;
  for (auto &f : S) {
    for (auto &x : f.terms(o)) {
      E e(x.first.begin() + 1, x.first.end());
      verts.push_back(e);
    }
    require(!f.zero(), "zero shift leading monomial failure");
    auto l = f.terms(o).front().first;
    E e(l.begin() + 1, l.end());
    lc[e] = l[0];
  }
  Domain d = hull(verts, save);
  std::vector<E> withzero = d.points;
  withzero.emplace_back(S.front().n - 1, 0);
  Domain all = hull(withzero, save);
  Aug a{d.points, {}, d.dimension};
  for (auto &p : all.points) {
    bool found = false;
    int v = 0;
    for (auto &kv : lc)
      if (divides(kv.first, p)) {
        if (!found || kv.second < v)
          v = kv.second;
        found = true;
      }
    require(found, "None valuation in AugPolyhedron.from_polytope");
    a.values[p] = v;
  }
  return a;
}
inline Aug convolution(const Aug &a, const Aug &b) {
  Aug c;
  c.dim = a.dim;
  std::set<E> points;
  for (auto &x : a.points)
    for (auto &y : b.points) {
      E z = x;
      for (size_t i = 0; i < z.size(); ++i) {
        require(z[i] <= 2147483647 - y[i], "sum exponent overflow");
        z[i] += y[i];
      }
      points.insert(z);
    }
  c.points.assign(points.begin(), points.end());
  for (auto &x : a.values)
    for (auto &y : b.values) {
      E z = x.first;
      for (size_t i = 0; i < z.size(); ++i) {
        require(z[i] <= 2147483647 - y.first[i], "valuation exponent overflow");
        z[i] += y.first[i];
      }
      require(x.second <= 2147483647 - y.second, "valuation overflow");
      int v = x.second + y.second;
      auto j = c.values.find(z);
      if (j == c.values.end() || v < j->second)
        c.values[z] = v;
    }
  return c;
}
struct Statistic {
  Q count;
  std::vector<Q> xs;
  Q C;
};
inline Statistic statistics(Aug &a) {
  require(!a.points.empty(), "empty augmented points");
  Statistic s{Q(static_cast<unsigned long>(a.points.size())),
              std::vector<Q>(a.points[0].size()), 0};
  for (auto &p : a.points) {
    auto j = a.values.find(p);
    if (j == a.values.end()) {
      bool found = false;
      int v = 0;
      for (auto &kv : a.values)
        if (divides(kv.first, p)) {
          if (!found || kv.second < v)
            v = kv.second;
          found = true;
        }
      require(found, "None valuation in AugPolyhedron.value");
      j = a.values.emplace(p, v).first;
    }
    s.C += j->second;
    for (size_t i = 0; i < p.size(); ++i)
      s.xs[i] += p[i];
  }
  return s;
}
} // namespace jsr
