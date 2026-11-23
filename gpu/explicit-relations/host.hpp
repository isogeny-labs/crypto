#pragma once
#include "curve.cuh"
#include <algorithm>
#include <limits>
#include <stdexcept>
#include <string>
#include <vector>

namespace explicit_relations {
inline U modmul(U a, U b, U p) { return (U)((unsigned __int128)a * b % p); }
inline U modpow(U a, U e, U p) {
  U y = 1;
  while (e) {
    if (e & 1)
      y = modmul(y, a, p);
    a = modmul(a, a, p);
    e >>= 1;
  }
  return y;
}
// Deterministic Miller-Rabin for all u64 inputs.
inline bool prime(U n) {
  if (n < 2)
    return false;
  for (U p : {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}) {
    if (n % p == 0)
      return n == p;
  }
  U d = n - 1;
  unsigned s = 0;
  while (!(d & 1)) {
    d >>= 1;
    ++s;
  }
  for (U a : {2ULL, 325ULL, 9375ULL, 28178ULL, 450775ULL, 9780504ULL,
              1795265022ULL}) {
    if (a % n == 0)
      continue;
    U x = modpow(a % n, d, n);
    if (x == 1 || x == n - 1)
      continue;
    bool composite = true;
    for (unsigned j = 1; j < s; ++j) {
      x = modmul(x, x, n);
      if (x == n - 1) {
        composite = false;
        break;
      }
    }
    if (composite)
      return false;
  }
  return true;
}
inline Curve curve(U p, U a, U b) {
  if (p < 5 || p >= (U(1) << 63) || !prime(p) || a >= p || b >= p)
    throw std::runtime_error("invalid prime-field curve");
  if ((modmul(4, modmul(modmul(a, a, p), a, p), p) +
       modmul(27, modmul(b, b, p), p)) %
          p ==
      0)
    throw std::runtime_error("singular curve");
  Fp64Ctx f = fp64_ctx_new(p);
  return {f, fp64_to_mont(f, a), fp64_to_mont(f, b)};
}
inline U product(U a, U b) {
  if (b && a > UINT64_MAX / b)
    throw std::runtime_error("LIMIT tuple product overflows u64");
  return a * b;
}
struct Plan {
  std::vector<Point> points;
  U offsets[MAX_SLOTS + 1]{};
  unsigned slots = 0, split = 0;
  U left = 0, right = 0;
  Bases view(const Point *p) const {
    Bases b{};
    b.points = p;
    b.slots = slots;
    std::copy(offsets, offsets + MAX_SLOTS + 1, b.offset);
    return b;
  }
  std::vector<U> indices(U l, U r) const {
    std::vector<U> out(slots);
    for (unsigned i = split; i-- > 0;) {
      U n = offsets[i + 1] - offsets[i];
      out[i] = l % n;
      l /= n;
    }
    for (unsigned i = slots; i-- > split;) {
      U n = offsets[i + 1] - offsets[i];
      out[i] = r % n;
      r /= n;
    }
    return out;
  }
};
inline Plan plan(Curve c, const std::vector<std::vector<Point>> &bases,
                 U max_rows) {
  if (bases.size() < 2 || bases.size() > MAX_SLOTS || !max_rows)
    throw std::runtime_error("invalid base slots or row cap");
  Plan p;
  p.slots = (unsigned)bases.size();
  bool empty = false;
  for (unsigned i = 0; i < p.slots; ++i) {
    const auto &b = bases[i];
    empty |= b.empty();
    if (b.size() > max_rows)
      throw std::runtime_error("LIMIT input base exceeds row cap");
    for (size_t j = 0; j < b.size(); ++j) {
      if (b[j].inf || !c.on_curve(b[j]) || (j && compare(b[j - 1], b[j]) >= 0))
        throw std::runtime_error(
            "bases require sorted distinct canonical signed points");
    }
    p.points.insert(p.points.end(), b.begin(), b.end());
    p.offsets[i + 1] = p.points.size();
  }
  if (empty) {
    p.split = 1;
    return p;
  }
  U best = UINT64_MAX;
  for (unsigned split = 1; split < p.slots; ++split) {
    U l = 1, r = 1;
    bool fits = true;
    for (unsigned i = 0; i < split; ++i) {
      if (l > UINT64_MAX / bases[i].size()) {
        fits = false;
        break;
      }
      l *= bases[i].size();
    }
    for (unsigned i = split; i < p.slots; ++i) {
      if (r > UINT64_MAX / bases[i].size()) {
        fits = false;
        break;
      }
      r *= bases[i].size();
    }
    if (!fits)
      continue;
    if (l <= max_rows && r <= UINT64_MAX - l && l + r < best) {
      best = l + r;
      p.left = l;
      p.right = r;
      p.split = split;
    }
  }
  if (!p.split)
    throw std::runtime_error("LIMIT no split fits row cap and work counter");
  return p;
}
struct Result {
  bool found = false;
  std::vector<U> indices;
  U visited = 0;
};
inline void verify(Curve c, const Plan &p, Point target, const Result &r) {
  if (!r.found)
    return;
  if (r.indices.size() != p.slots)
    throw std::runtime_error("invalid witness length");
  Jac sum;
  for (unsigned i = 0; i < p.slots; ++i) {
    if (r.indices[i] >= p.offsets[i + 1] - p.offsets[i])
      throw std::runtime_error("invalid witness index");
    sum = c.mixed(sum, p.points[p.offsets[i] + r.indices[i]]);
  }
  if (!equal(c.affine(sum), target))
    throw std::runtime_error("invalid GPU witness sum");
}
inline void hash_parameters(Curve c, U s, U alpha, U beta, U tau, U cap) {
  if (s < 2 * c.f.p || (unsigned __int128)s > 4 * (unsigned __int128)c.f.p || !prime(s) ||
      !alpha || alpha >= s || beta >= s || !tau || tau > s)
    throw std::runtime_error(
        "invalid hash parameters (u64 prime in [2p,4p] required)");
  if (tau > cap)
    throw std::runtime_error("LIMIT hash candidates exceed row cap");
}
// Host reference has the same split and deterministic witness convention as
// the device. Tests also use an independent affine addition implementation.
class HostBackend {
  Curve c;
  Plan p;
  std::vector<Row> rows;

public:
  HostBackend(Curve curve, U, U) : c(curve) {}
  void prepare(const Plan &next) {
    p = next;
    rows.resize(p.left);
    Bases b = p.view(p.points.data());
    for (U i = 0; i < p.left; ++i)
      rows[i] = {c.affine(tuple_sum(c, b, 0, p.split, i)), i};
    std::sort(rows.begin(), rows.end(), RowLess{});
  }
  Result find(Point target, U max_work) {
    if (!c.on_curve(target))
      throw std::runtime_error("invalid target");
    if (p.left + p.right > max_work)
      throw std::runtime_error("LIMIT decomposition work; not nonexistence");
    Result r;
    Bases b = p.view(p.points.data());
    for (U i = 0; i < p.right; ++i) {
      Point sum = c.affine(tuple_sum(c, b, p.split, p.slots, i));
      Point residual = c.affine(c.mixed(c.jac(target), c.neg(sum)));
      ++r.visited;
      U l = lookup(rows.data(), rows.size(), residual);
      if (l != UINT64_MAX) {
        r.found = true;
        r.indices = p.indices(l, i);
        break;
      }
    }
    verify(c, p, target, r);
    return r;
  }
  std::vector<Point> lift(const std::vector<U> &xs) {
    std::vector<Point> out;
    for (U x : xs) {
      Point q;
      if (!c.lift(x, 0, q) && !c.lift(x, 1, q))
        throw std::runtime_error("x does not lift");
      out.push_back(q);
      Point n = c.neg(q);
      if (!equal(q, n))
        out.push_back(n);
    }
    std::sort(out.begin(), out.end(),
              [](Point a, Point b) { return compare(a, b) < 0; });
    return out;
  }
  std::vector<Point> hash(U s, U alpha, U beta, U tau) {
    U inv = modpow(alpha, s - 2, s);
    std::vector<Point> out;
    for (U t = 0; t < tau; ++t) {
      U z = modmul(inv, t >= beta ? t - beta : s - (beta - t), s);
      Point q;
      if (z < 2 * c.f.p && c.lift(z / 2, z & 1, q))
        out.push_back(q);
    }
    std::sort(out.begin(), out.end(),
              [](Point a, Point b) { return compare(a, b) < 0; });
    return out;
  }
};
} // namespace explicit_relations
