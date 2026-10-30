// Public-input arithmetic for explicit signed relation finding. Reuses the
// repository's Montgomery Fp64 implementation; 5 <= p < 2^63, p prime.
#pragma once
#include "../rootfind/fp64.cuh"

namespace explicit_relations {
using U = uint64_t;
constexpr unsigned MAX_SLOTS = 12;
// Canonical residues, 0 < modulus < 2^63. Every addition is <2*modulus,
// hence fits u64. Used only for wider hash moduli; curve Montgomery stays
// on its checked p<2^63 path; this helper retains its narrower bound.
RF_HD U hash_mul63(U a, U b, U modulus) {
  U result = 0;
  while (b) {
    if (b & 1) {
      result += a;
      if (result >= modulus)
        result -= modulus;
    }
    a += a;
    if (a >= modulus)
      a -= modulus;
    b >>= 1;
  }
  return result;
}
// Overflow-safe canonical multiplication for any nonzero u64 modulus.
RF_HD U hash_mul64(U a, U b, U modulus) {
  U result = 0;
  while (b) {
    if (b & 1)
      result = result >= modulus - a ? result - (modulus - a) : result + a;
    a = a >= modulus - a ? a - (modulus - a) : a + a;
    b >>= 1;
  }
  return result;
}
struct Point {
  U x = 0, y = 0;
  bool inf = true;
};
// Exact affine cache: x=UINT64_MAX is disjoint from every supported field.
struct alignas(16) CachedPoint {
  U x = UINT64_MAX, y = 0;
  RF_HD static CachedPoint from(Point p) {
    return p.inf ? CachedPoint{} : CachedPoint{p.x, p.y};
  }
  RF_HD Point point() const {
    return x == UINT64_MAX ? Point{} : Point{x, y, false};
  }
};
static_assert(sizeof(CachedPoint) == 16 && alignof(CachedPoint) == 16,
              "exact aligned affine cache layout");
// Admission arithmetic avoids overflowing count*sizeof(CachedPoint).
RF_HD bool right_cache_fits(U count, U retained, U free_bytes, U byte_limit) {
  const U budget = free_bytes - free_bytes / 4;
  return count && retained <= budget &&
         count <= (budget - retained) / sizeof(CachedPoint) &&
         count <= byte_limit / sizeof(CachedPoint);
}
struct Jac {
  U x = 0, y = 0, z = 0;
};
struct Curve {
  Fp64Ctx f;
  U a, b; // Montgomery
  RF_HD U add(U x, U y) const { return fp64_add(f, x, y); }
  RF_HD U sub(U x, U y) const { return fp64_sub(f, x, y); }
  RF_HD U mul(U x, U y) const { return fp64_mul(f, x, y); }
  RF_HD U sq(U x) const { return mul(x, x); }
  RF_HD U twice(U x) const { return add(x, x); }
  RF_HD U pow(U x, U e) const {
    U y = f.one;
    while (e) {
      if (e & 1)
        y = mul(y, x);
      x = sq(x);
      e >>= 1;
    }
    return y;
  }
  RF_HD U rhs(U x) const { return add(mul(add(sq(x), a), x), b); }
  RF_HD bool on_curve(Point p) const {
    return p.inf || (p.x < f.p && p.y < f.p &&
                     sq(fp64_to_mont(f, p.y)) == rhs(fp64_to_mont(f, p.x)));
  }
  // Tonelli-Shanks, with verified output and all odd-prime congruence classes.
  RF_HD bool sqrt(U v, U &out) const {
    if (!v) {
      out = 0;
      return true;
    }
    if (pow(v, (f.p - 1) / 2) != f.one)
      return false;
    if ((f.p & 3) == 3) {
      out = pow(v, (f.p + 1) / 4);
      return sq(out) == v;
    }
    U q = f.p - 1;
    unsigned s = 0;
    while (!(q & 1)) {
      q >>= 1;
      ++s;
    }
    U z = 2;
    while (pow(fp64_to_mont(f, z), (f.p - 1) / 2) == f.one)
      ++z;
    U c = pow(fp64_to_mont(f, z), q), t = pow(v, q), x = pow(v, (q + 1) / 2);
    while (t != f.one) {
      U u = t;
      unsigned i = 0;
      do {
        u = sq(u);
        ++i;
      } while (u != f.one && i < s);
      if (i == s)
        return false;
      U d = c;
      for (unsigned j = 0; j < s - i - 1; ++j)
        d = sq(d);
      x = mul(x, d);
      c = sq(d);
      t = mul(t, c);
      s = i;
    }
    out = x;
    return sq(out) == v;
  }
  RF_HD bool lift(U x, unsigned parity, Point &out) const {
    if (x >= f.p)
      return false;
    U y;
    if (!sqrt(rhs(fp64_to_mont(f, x)), y))
      return false;
    y = fp64_from_mont(f, y);
    if ((y & 1) != parity)
      y = y ? f.p - y : 0;
    if ((y & 1) != parity)
      return false;
    out = {x, y, false};
    return true;
  }
  RF_HD Point neg(Point p) const {
    if (!p.inf && p.y)
      p.y = f.p - p.y;
    return p;
  }
  RF_HD Jac jac(Point p) const {
    return p.inf ? Jac{}
                 : Jac{fp64_to_mont(f, p.x), fp64_to_mont(f, p.y), f.one};
  }
  RF_HD Jac dbl(Jac p) const {
    if (!p.z || !p.y)
      return {};
    U xx = sq(p.x), yy = sq(p.y), yyyy = sq(yy), zz = sq(p.z);
    U s = twice(sub(sub(sq(add(p.x, yy)), xx), yyyy));
    U m = add(add(twice(xx), xx), mul(a, sq(zz)));
    U x = sub(sq(m), twice(s));
    U y = sub(mul(m, sub(s, x)), twice(twice(twice(yyyy))));
    return {x, y, twice(mul(p.y, p.z))};
  }
  RF_HD Jac mixed(Jac p, Point q) const {
    if (q.inf)
      return p;
    if (!p.z)
      return jac(q);
    U zz = sq(p.z), u = mul(fp64_to_mont(f, q.x), zz);
    U s = mul(fp64_to_mont(f, q.y), mul(p.z, zz));
    U h = sub(u, p.x), d = sub(s, p.y);
    if (!h)
      return !d ? dbl(p) : Jac{};
    U hh = sq(h), i = twice(twice(hh)), j = mul(h, i), r = twice(d),
      v = mul(p.x, i);
    U x = sub(sub(sq(r), j), twice(v));
    U y = sub(mul(r, sub(v, x)), twice(mul(p.y, j)));
    return {x, y, sub(sub(sq(add(p.z, h)), zz), hh)};
  }
  RF_HD Point affine(Jac p, U iz) const {
    if (!p.z)
      return {};
    U zz = sq(iz);
    return {fp64_from_mont(f, mul(p.x, zz)),
            fp64_from_mont(f, mul(p.y, mul(zz, iz))), false};
  }
  RF_HD Point affine(Jac p) const { return affine(p, fp64_inv(f, p.z)); }
};
RF_HD bool equal(Point a, Point b) {
  return a.inf == b.inf && (a.inf || (a.x == b.x && a.y == b.y));
}
// Infinity is a distinct full key. Both coordinates are compared: no hash or
// x-only collision can produce a relation, and opposite signs remain distinct.
RF_HD int compare(Point a, Point b) {
  if (a.inf != b.inf)
    return a.inf ? -1 : 1;
  if (a.inf)
    return 0;
  if (a.x != b.x)
    return a.x < b.x ? -1 : 1;
  return a.y == b.y ? 0 : (a.y < b.y ? -1 : 1);
}
struct Row {
  Point point;
  U id;
};
struct RowLess {
  RF_HD bool operator()(const Row &a, const Row &b) const {
    int c = compare(a.point, b.point);
    return c < 0 || (c == 0 && a.id < b.id);
  }
};
// Exact point compression on one validated odd-prime curve (SEC 1 section
// 2.3.3): the two roots y and p-y have opposite parity. Infinity is key zero;
// finite keys are 2*(x+1)+(y&1). The service requires p<2^63, so all finite keys fit u64.
// The terminal prefix offset is written as table size, without encoding 2^63. This is not a fingerprint and must never
// be used to compare arbitrary off-curve coordinates or different curves.
RF_HD U point_key(Point p) {
  return p.inf ? 0 : ((p.x + 1) << 1) | (p.y & 1);
}
struct CompactRow {
  U key, id;
};
static_assert(sizeof(CompactRow) == 16, "compact rows must remain 16 bytes");
struct CompactRowLess {
  RF_HD bool operator()(const CompactRow &a, const CompactRow &b) const {
    return a.key < b.key || (a.key == b.key && a.id < b.id);
  }
};
RF_HD U lookup(const CompactRow *rows, U size, Point point) {
  U key = point_key(point), lo = 0, hi = size;
  while (lo < hi) {
    U mid = lo + (hi - lo) / 2;
    if (rows[mid].key < key)
      lo = mid + 1;
    else
      hi = mid;
  }
  return lo < size && rows[lo].key == key ? rows[lo].id : UINT64_MAX;
}
// Three naturally aligned 32-bit words: no unaligned 64-bit loads. The
// exact 64-bit point key is split into halves; tuple IDs are bounded by
// left_rows<=2^32 before construction. UINT32_MAX is a valid tuple ID,
// distinct from the widened UINT64_MAX missing-result sentinel.
struct DenseRow {
  uint32_t key_lo = 0, key_hi = 0, id = 0;
  RF_HD DenseRow() = default;
  RF_HD DenseRow(U key, U tuple_id)
      : key_lo(uint32_t(key)), key_hi(uint32_t(key >> 32)),
        id(uint32_t(tuple_id)) {}
  RF_HD U key() const { return U(key_lo) | (U(key_hi) << 32); }
};
static_assert(sizeof(DenseRow) == 12 && alignof(DenseRow) == 4,
              "dense rows require three naturally aligned u32 words");
struct DenseRowLess {
  RF_HD bool operator()(const DenseRow &a, const DenseRow &b) const {
    return a.key() < b.key() || (a.key() == b.key() && a.id < b.id);
  }
};
RF_HD U lookup(const DenseRow *rows, U size, Point point) {
  U key = point_key(point), lo = 0, hi = size;
  while (lo < hi) {
    U mid = lo + (hi - lo) / 2;
    if (rows[mid].key() < key)
      lo = mid + 1;
    else
      hi = mid;
  }
  return lo < size && rows[lo].key() == key ? U(rows[lo].id) : UINT64_MAX;
}
struct Bases {
  const Point *points;
  U offset[MAX_SLOTS + 1];
  unsigned slots;
};
RF_HD Jac tuple_sum(Curve c, Bases bases, unsigned first, unsigned end, U id) {
  Jac sum;
  for (unsigned slot = end; slot-- > first;) {
    U n = bases.offset[slot + 1] - bases.offset[slot];
    sum = c.mixed(sum, bases.points[bases.offset[slot] + id % n]);
    id /= n;
  }
  return sum;
}
RF_HD U lookup(const Row *rows, U size, Point key) {
  U lo = 0, hi = size;
  while (lo < hi) {
    U mid = lo + (hi - lo) / 2;
    if (compare(rows[mid].point, key) < 0)
      lo = mid + 1;
    else
      hi = mid;
  }
  return lo < size && equal(rows[lo].point, key) ? rows[lo].id : UINT64_MAX;
}
} // namespace explicit_relations
