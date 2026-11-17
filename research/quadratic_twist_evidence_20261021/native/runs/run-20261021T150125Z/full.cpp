// Full native replay of the frozen finite experiment contract. No Python
// execution.
#include "json.hpp"
#include <array>
#include <cmath>
#include <memory>
#include <numeric>
#include <set>

using I = long long;
Json num(I n) {
  Json j;
  j.kind = Json::Number;
  j.text = std::to_string(n);
  return j;
}
Json str(const std::string &s) {
  Json j;
  j.kind = Json::String;
  j.text = s;
  return j;
}
Json truth(bool b = true) {
  Json j;
  j.kind = Json::Bool;
  j.text = b ? "true" : "false";
  return j;
}
Json obj(std::initializer_list<std::pair<const std::string, Json>> v = {}) {
  Json j;
  j.kind = Json::Object;
  j.fields = v;
  return j;
}
Json arr() {
  Json j;
  j.kind = Json::Array;
  return j;
}
Json nums(const std::vector<I> &v) {
  auto j = arr();
  for (auto n : v)
    j.items.push_back(num(n));
  return j;
}
std::string dump(const Json &j) {
  if (j.kind == Json::Null)
    return "null";
  if (j.kind == Json::Number || j.kind == Json::Bool)
    return j.text;
  if (j.kind == Json::String) {
    std::string s = "\"";
    for (unsigned char c : j.text) {
      if (c == '"' || c == '\\') {
        s += '\\';
        s += char(c);
      } else if (c == '\n')
        s += "\\n";
      else {
        require(c >= 32, "unserializable control");
        s += char(c);
      }
    }
    return s + '"';
  }
  std::string s = j.kind == Json::Array ? "[" : "{";
  bool first = true;
  if (j.kind == Json::Array)
    for (auto &x : j.items) {
      if (!first)
        s += ',';
      first = false;
      s += dump(x);
    }
  else
    for (auto &[k, v] : j.fields) {
      if (!first)
        s += ',';
      first = false;
      s += dump(str(k)) + ':' + dump(v);
    }
  return s + (j.kind == Json::Array ? "]" : "}");
}
I mod(I a, I p) {
  a %= p;
  return a < 0 ? a + p : a;
}
I ipow(I a, I n, I p) {
  I r = 1;
  for (; n; n /= 2, a = a * a % p)
    if (n % 2)
      r = r * a % p;
  return r;
}
std::map<I, I> factors(I n) {
  std::map<I, I> r;
  for (I d = 2; d * d <= n; ++d)
    while (n % d == 0) {
      ++r[d];
      n /= d;
    }
  if (n > 1)
    ++r[n];
  return r;
}
I nonsquare(I p) {
  for (I d = 2; d < p; ++d)
    if (ipow(d, (p - 1) / 2, p) == p - 1)
      return d;
  throw std::runtime_error("nonsquare");
}

// CPython's integer-seeded MT19937 and rejection-based _randbelow. Exact
// draw compatibility preserves the historical fixtures, not just their seeds.
struct Random {
  std::array<uint32_t, 624> mt{};
  size_t index = 624;
  explicit Random(uint64_t seed) {
    mt[0] = 19650218U;
    for (size_t i = 1; i < 624; ++i)
      mt[i] = 1812433253U * (mt[i - 1] ^ (mt[i - 1] >> 30)) + uint32_t(i);
    std::vector<uint32_t> key{uint32_t(seed)};
    if (seed >> 32)
      key.push_back(uint32_t(seed >> 32));
    size_t i = 1, j = 0;
    for (size_t k = std::max(size_t(624), key.size()); k; --k) {
      mt[i] = (mt[i] ^ ((mt[i - 1] ^ (mt[i - 1] >> 30)) * 1664525U)) + key[j] +
              uint32_t(j);
      if (++i >= 624) {
        mt[0] = mt[623];
        i = 1;
      }
      if (++j >= key.size())
        j = 0;
    }
    for (size_t k = 623; k; --k) {
      mt[i] = (mt[i] ^ ((mt[i - 1] ^ (mt[i - 1] >> 30)) * 1566083941U)) -
              uint32_t(i);
      if (++i >= 624) {
        mt[0] = mt[623];
        i = 1;
      }
    }
    mt[0] = 0x80000000U;
  }
  uint32_t word() {
    if (index >= 624) {
      for (size_t k = 0; k < 624; ++k) {
        auto y = (mt[k] & 0x80000000U) | (mt[(k + 1) % 624] & 0x7fffffffU);
        mt[k] = mt[(k + 397) % 624] ^ (y >> 1) ^ ((y & 1) ? 0x9908b0dfU : 0U);
      }
      index = 0;
    }
    auto y = mt[index++];
    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680U;
    y ^= (y << 15) & 0xefc60000U;
    y ^= y >> 18;
    return y;
  }
  I below(I n) {
    require(n > 0 && n < 0x80000000LL, "random bound");
    unsigned k = 0;
    for (I z = n; z; z >>= 1)
      ++k;
    I r;
    do {
      r = word() >> (32 - k);
    } while (r >= n);
    return r;
  }
};

struct Field {
  enum Kind { Prime, Quadratic, Polynomial, Tower } kind = Prime;
  I p, d = 0, q, size, n = 1;
  std::vector<I> modulus;
  std::shared_ptr<Field> base;
  std::vector<std::vector<I>> roots;
  explicit Field(I prime, I delta = 0)
      : kind(delta ? Quadratic : Prime), p(prime), d(delta), q(prime),
        size(delta ? prime * prime : prime), n(delta ? 2 : 1) {
    init();
  }
  Field(I prime, std::vector<I> poly)
      : kind(Polynomial), p(prime), q(prime), size(1), n(I(poly.size()) - 1),
        modulus(std::move(poly)) {
    for (I i = 0; i < n; ++i)
      size *= p;
    require(modulus.back() == 1, "monic modulus");
    init();
    // For these small degrees, checking x^(q-1)=1 for every nonzero
    // residue proves the quotient is a field (and thus irreducibility).
    for (I x = 1; x < size; ++x)
      require(pow(x, size - 1) == 1, "reducible polynomial");
  }
  Field(std::shared_ptr<Field> k, I delta)
      : kind(Tower), p(k->p), d(delta), q(k->size), size(q * q), n(2 * k->n),
        base(std::move(k)) {
    require(base->pow(d, (q - 1) / 2) == base->neg(1), "tower nonsquare");
    init();
  }
  void init() {
    roots.resize(size);
    for (I y = 0; y < size; ++y)
      roots[mul(y, y)].push_back(y);
  }
  I encode(const std::vector<I> &cs) const {
    I a = 0;
    for (auto it = cs.rbegin(); it != cs.rend(); ++it)
      a = a * p + mod(*it, p);
    return a;
  }
  std::vector<I> digits(I a) const {
    std::vector<I> r(n);
    for (auto &c : r) {
      c = a % p;
      a /= p;
    }
    return r;
  }
  I add(I a, I b) const {
    if (kind == Prime)
      return (a + b) % p;
    if (kind == Tower)
      return base->add(a % q, b % q) + q * base->add(a / q, b / q);
    if (kind == Quadratic)
      return (a % p + b % p) % p + p * ((a / p + b / p) % p);
    auto x = digits(a), y = digits(b);
    for (I i = 0; i < n; ++i)
      x[i] += y[i];
    return encode(x);
  }
  I neg(I a) const {
    if (kind == Prime)
      return mod(-a, p);
    if (kind == Tower)
      return base->neg(a % q) + q * base->neg(a / q);
    if (kind == Quadratic)
      return mod(-a, p) + p * mod(-(a / p), p);
    auto x = digits(a);
    for (auto &c : x)
      c = -c;
    return encode(x);
  }
  I sub(I a, I b) const { return add(a, neg(b)); }
  I mul(I a, I b) const {
    if (kind == Prime)
      return a * b % p;
    if (kind == Tower)
      return base->add(base->mul(a % q, b % q),
                       base->mul(d, base->mul(a / q, b / q))) +
             q * base->add(base->mul(a % q, b / q), base->mul(a / q, b % q));
    if (kind == Quadratic)
      return (a % p * (b % p) + d * (a / p) * (b / p)) % p +
             p * ((a % p * (b / p) + (a / p) * (b % p)) % p);
    auto x = digits(a), y = digits(b);
    std::vector<I> cs(2 * n - 1);
    for (I i = 0; i < n; ++i)
      for (I j = 0; j < n; ++j)
        cs[i + j] += x[i] * y[j];
    for (I i = 2 * n - 2; i >= n; --i) {
      I c = mod(cs[i], p);
      for (I j = 0; j < n; ++j)
        cs[i - n + j] -= c * modulus[j];
    }
    cs.resize(n);
    return encode(cs);
  }
  I pow(I a, I e) const {
    if (e < 0) {
      a = inv(a);
      e = -e;
    }
    I r = 1;
    for (; e; e /= 2, a = mul(a, a))
      if (e % 2)
        r = mul(r, a);
    return r;
  }
  I inv(I a) const {
    require(a != 0, "division by zero");
    if (kind == Prime)
      return ipow(a, p - 2, p);
    if (kind == Polynomial)
      return pow(a, size - 2);
    if (kind == Tower) {
      auto ni = base->inv(base->sub(base->mul(a % q, a % q),
                                    base->mul(d, base->mul(a / q, a / q))));
      return base->mul(a % q, ni) + q * base->neg(base->mul(a / q, ni));
    }
    I ni = ipow(mod((a % p) * (a % p) - d * (a / p) * (a / p), p), p - 2, p);
    return a % p * ni % p + p * mod(-(a / p) * ni, p);
  }
  I div(I a, I b) const { return mul(a, inv(b)); }
  I frob(I a) const {
    if (kind == Prime)
      return a;
    if (kind == Tower)
      return a % q + q * base->neg(a / q);
    if (kind == Quadratic)
      return a % p + p * mod(-(a / p), p);
    return pow(a, p);
  }
  std::string fmt(I a) const {
    if (kind == Prime || ((kind == Quadratic) && a < p))
      return std::to_string(a);
    if (kind == Quadratic)
      return std::to_string(a % p) + "+" + std::to_string(a / p) + "u";
    if (kind == Tower) {
      if (a < q)
        return base->fmt(a);
      return "(" + base->fmt(a % q) + ")+(" + base->fmt(a / q) + ")v";
    }
    auto x = digits(a);
    std::string s;
    for (I i = 0; i < n; ++i)
      if (x[i]) {
        if (!s.empty())
          s += '+';
        s += std::to_string(x[i]) + "w^" + std::to_string(i);
      }
    return s.empty() ? "0" : s;
  }
};
struct Point {
  I x = -1, y = -1;
  bool infinity() const { return x == -1; }
  bool operator==(Point b) const { return x == b.x && y == b.y; }
  bool operator!=(Point b) const { return !(*this == b); }
  bool operator<(Point b) const { return std::tie(x, y) < std::tie(b.x, b.y); }
};
struct Curve {
  const Field &f;
  I a, b, c;
  Curve(const Field &k, I aa, I bb, I cc = 1) : f(k), a(aa), b(bb), c(cc) {
    require(
        c && f.add(f.mul(4 % f.p, f.pow(a, 3)), f.mul(27 % f.p, f.pow(b, 2))),
        "singular model");
  }
  I rhs(I x) const { return f.add(f.add(f.pow(x, 3), f.mul(a, x)), b); }
  bool contains(Point P) const {
    return P.infinity() || f.mul(c, f.mul(P.y, P.y)) == rhs(P.x);
  }
  std::vector<Point> points(I bound = -1) const {
    std::vector<Point> r{{}};
    I ci = f.inv(c);
    for (I x = 0; x < (bound < 0 ? f.size : bound); ++x)
      for (I y : f.roots[f.mul(ci, rhs(x))])
        r.push_back({x, y});
    return r;
  }
  Point neg(Point P) const { return P.infinity() ? P : Point{P.x, f.neg(P.y)}; }
  Point frob(Point P) const {
    return P.infinity() ? P : Point{f.frob(P.x), f.frob(P.y)};
  }
  Point add(Point P, Point Q) const {
    if (P.infinity())
      return Q;
    if (Q.infinity())
      return P;
    I m;
    if (P.x == Q.x) {
      if (f.add(P.y, Q.y) == 0)
        return {};
      m = f.div(f.add(f.mul(3 % f.p, f.mul(P.x, P.x)), a),
                f.mul(2 % f.p, f.mul(c, P.y)));
    } else
      m = f.div(f.sub(Q.y, P.y), f.sub(Q.x, P.x));
    I x = f.sub(f.sub(f.mul(c, f.mul(m, m)), P.x), Q.x);
    return {x, f.sub(f.mul(m, f.sub(P.x, x)), P.y)};
  }
  Point scalar(I n, Point P) const {
    if (n < 0)
      return scalar(-n, neg(P));
    Point r;
    for (; n; n /= 2, P = add(P, P))
      if (n % 2)
        r = add(r, P);
    return r;
  }
  I order(Point P, I n) const {
    for (auto [ell, e] : factors(n)) {
      (void)e;
      while (n % ell == 0 && scalar(n / ell, P).infinity())
        n /= ell;
    }
    require(scalar(n, P).infinity(), "point order");
    return n;
  }
  std::string fmt(Point P) const {
    return P.infinity() ? "O" : "(" + f.fmt(P.x) + ", " + f.fmt(P.y) + ")";
  }
};
Point image(Point P, const Field &f) {
  return P.infinity() ? P : Point{P.x, f.mul(f.p, P.y)};
}
I count(I p, I a, I b) {
  I n = p + 1;
  for (I x = 0; x < p; ++x) {
    I rhs = mod(x * x * x + a * x + b, p);
    if (rhs)
      n += ipow(rhs, (p - 1) / 2, p) == 1 ? 1 : -1;
  }
  return n;
}
using Triple = std::array<I, 3>;
std::set<Triple> relations(const Curve &C, I p) {
  std::set<Triple> r;
  auto pts = C.points(p);
  for (auto P : pts)
    if (!P.infinity())
      for (auto Q : pts)
        if (!Q.infinity()) {
          auto R = C.add(P, Q);
          if (!R.infinity() && R.x < p)
            r.insert({P.x, Q.x, R.x});
        }
  return r;
}
I s3(I p, I a, I b, I x, I y, I z) {
  return mod((x - y) * (x - y) * z * z -
                 2 * ((x + y) * (x * y + a) + 2 * b) * z +
                 (x * y - a) * (x * y - a) - 4 * b * (x + y),
             p);
}
I s4checks(const Curve &C, I p, I a, I b) {
  std::vector<std::vector<Point>> byx(p);
  for (auto P : C.points(p))
    if (!P.infinity())
      byx[P.x].push_back(P);
  std::map<std::pair<I, I>, std::set<Point>> sums;
  std::map<std::pair<I, I>, Triple> co;
  for (I x = 0; x < p; ++x)
    for (I y = 0; y < p; ++y) {
      for (auto P : byx[x])
        for (auto Q : byx[y])
          sums[{x, y}].insert(C.add(P, Q));
      co[{x, y}] = {mod((x - y) * (x - y), p),
                    mod(-2 * ((x + y) * (x * y + a) + 2 * b), p),
                    mod((x * y - a) * (x * y - a) - 4 * b * (x + y), p)};
    }
  I n = 0;
  for (I x = 0; x < p; ++x)
    for (I y = 0; y < p; ++y)
      for (I z = 0; z < p; ++z)
        for (I w = 0; w < p; ++w) {
          auto [A, B, C0] = co[{x, y}];
          auto [D, F, G] = co[{z, w}];
          I u = A * G - C0 * D;
          I res = mod(u * u - (A * F - B * D) * (B * G - C0 * F), p);
          bool hit = false;
          for (auto P : sums[{x, y}])
            if (sums[{z, w}].count(P)) {
              hit = true;
              break;
            }
          require((res == 0) == hit, "S4 lifts");
          ++n;
        }
  return n;
}
I fieldchecks(const Field &f) {
  for (I x = 0; x < f.size; ++x) {
    require(f.add(x, f.neg(x)) == 0 && f.frob(x) == f.pow(x, f.p),
            "field/Frobenius");
    if (x)
      require(f.mul(x, f.inv(x)) == 1 && f.inv(x) == f.pow(x, f.size - 2),
              "field inversion");
  }
  Random rng(f.size);
  for (int i = 0; i < 300; ++i) {
    auto a = rng.below(f.size), b = rng.below(f.size), c = rng.below(f.size);
    require(f.mul(a, f.add(b, c)) == f.add(f.mul(a, b), f.mul(a, c)),
            "distributivity");
  }
  return f.size;
}

Json exhaustive() {
  std::map<std::string, I> counters;
  auto examples = obj(), perprime = arr(), identities = arr();
  for (I p : {5, 7, 11, 13, 17, 19}) {
    I d = nonsquare(p), models = 0;
    Field k(p), K(p, d);
    counters["field_elements_checked"] += fieldchecks(k) + fieldchecks(K);
    for (I a = 0; a < p; ++a)
      for (I b = 0; b < p; ++b) {
        if (mod(4 * a * a * a + 27 * b * b, p) == 0)
          continue;
        Curve E(k, a, b), T(k, a, b, d), L(K, a, b), LT(K, a, b, d);
        auto base = E.points(), twist = T.points(), extension = L.points();
        std::set<Point> plus(base.begin(), base.end()), anti, computedplus,
            computedminus, shared;
        for (auto P : twist)
          anti.insert(image(P, K));
        I n = I(base.size()), nt = I(twist.size());
        require(n == count(p, a, b) && n + nt == 2 * (p + 1) &&
                    I(extension.size()) == n * nt,
                "orders");
        for (auto P : extension) {
          if (L.frob(P) == P)
            computedplus.insert(P);
          if (L.frob(P) == L.neg(P))
            computedminus.insert(P);
        }
        require(computedplus == plus && computedminus == anti,
                "eigenspace sets");
        for (auto P : base)
          if (anti.count(P))
            shared.insert(P);
        std::set<Point> two;
        for (auto P : base)
          if (E.scalar(2, P).infinity())
            two.insert(P);
        require(two == shared, "intersection at two");
        for (auto P : extension) {
          require(L.contains(P) && L.frob(L.frob(P)) == P, "extension point");
          auto A = L.add(P, L.frob(P)), B = L.add(P, L.neg(L.frob(P)));
          require(plus.count(A) && anti.count(B) &&
                      L.add(A, B) == L.scalar(2, P),
                  "trace decomposition");
          ++counters["extension_points_checked"];
        }
        for (auto U : twist)
          for (auto V : twist) {
            require(image(T.add(U, V), K) == L.add(image(U, K), image(V, K)),
                    "twist group isomorphism");
            ++counters["twist_group_law_checks"];
          }
        for (auto P : twist)
          for (I s = 0; s < 6; ++s) {
            require(image(T.scalar(s, P), K) == L.scalar(s, image(P, K)),
                    "scalar transport");
            ++counters["scalar_preservation_checks"];
          }
        for (auto A : base)
          for (auto B : anti) {
            auto R = L.add(A, B);
            if (plus.count(R)) {
              require(L.scalar(2, B).infinity(), "mixed relation obstruction");
              ++counters["mixed_sums_landing_in_plus"];
              if (!A.infinity() && !B.infinity() && !R.infinity() &&
                  !examples.fields.count("two_torsion_exception"))
                examples.fields["two_torsion_exception"] =
                    obj({{"p", num(p)},
                         {"a", num(a)},
                         {"b", num(b)},
                         {"d", num(d)},
                         {"A", str(L.fmt(A))},
                         {"B", str(L.fmt(B))},
                         {"R", str(L.fmt(R))},
                         {"B_order", num(2)}});
            }
            if (!B.infinity() && L.order(B, nt) % 2) {
              require(!plus.count(R), "odd twist exclusion");
              ++counters["nonzero_odd_twist_exclusions"];
            }
          }
        auto rel = relations(L, p);
        require(rel == relations(LT, p), "x-relation twist invariance");
        for (I x = 0; x < p; ++x)
          for (I y = 0; y < p; ++y)
            for (I z = 0; z < p; ++z) {
              require((s3(p, a, b, x, y, z) == 0) == bool(rel.count({x, y, z})),
                      "S3 lifts");
              ++counters["s3_ordered_triples_checked"];
            }
        if (p <= 7)
          counters["s4_ordered_quadruples_checked"] += s4checks(L, p, a, b);
        I t = p + 1 - n;
        for (auto [ell, e] : factors(std::gcd(n, nt))) {
          (void)e;
          if (ell == 2)
            continue;
          require(t % ell == 0 && (p + 1) % ell == 0 && p % ell != 1 &&
                      p * p % ell == 1,
                  "shared prime embedding degree");
          ++counters["shared_odd_prime_cases"];
          if (t && !examples.fields.count("shared_prime_nonzero_trace"))
            examples.fields["shared_prime_nonzero_trace"] =
                obj({{"p", num(p)},
                     {"a", num(a)},
                     {"b", num(b)},
                     {"r", num(ell)},
                     {"trace", num(t)},
                     {"n", num(n)},
                     {"twist_order", num(nt)}});
        }
        if (!examples.fields.count("two_primary_product_failure")) {
          I e2 = 0;
          for (auto P : extension)
            e2 += L.scalar(2, P).infinity();
          I product = I(shared.size() * shared.size());
          if (e2 != product)
            examples.fields["two_primary_product_failure"] =
                obj({{"p", num(p)},
                     {"a", num(a)},
                     {"b", num(b)},
                     {"d", num(d)},
                     {"n", num(n)},
                     {"twist_order", num(nt)},
                     {"extension_order", num(I(extension.size()))},
                     {"extension_2_torsion", num(e2)},
                     {"product_2_torsion", num(product)}});
        }
        auto assoc = [&](Point A, Point B, Point C) {
          require(E.add(E.add(A, B), C) == E.add(A, E.add(B, C)),
                  "associativity");
          ++counters["associativity_checks"];
        };
        if (p == 5) {
          for (auto A : base)
            for (auto B : base)
              for (auto C : base)
                assoc(A, B, C);
        } else {
          Random rng(p * 10000 + a * p + b);
          for (int i = 0; i < 12; ++i) {
            auto A = base[rng.below(n)], B = base[rng.below(n)],
                 C = base[rng.below(n)];
            assoc(A, B, C);
          }
        }
        auto triples_json = arr();
        for (auto triple : rel)
          triples_json.items.push_back(nums({triple[0], triple[1], triple[2]}));
        identities.items.push_back(
            obj({{"p", num(p)},
                 {"a", num(a)},
                 {"b", num(b)},
                 {"d", num(d)},
                 {"curve_order", num(n)},
                 {"twist_order", num(nt)},
                 {"extension_order", num(I(extension.size()))},
                 {"s3_x_relation_count", num(I(rel.size()))},
                 {"s3_x_relations_sha256", str(digest(dump(triples_json)))}}));
        ++models;
        ++counters["curve_models"];
      }
    perprime.items.push_back(obj({{"p", num(p)},
                                  {"nonsquare", num(d)},
                                  {"nonsingular_models", num(models)}}));
    std::cout << "PASS census p=" << p << ": " << models << " models\n"
              << std::flush;
  }
  auto c = obj();
  for (auto &[key, value] : counters)
    c.fields[key] = num(value);
  return obj({{"counts", c},
              {"examples", examples},
              {"per_prime", perprime},
              {"census_models", identities}});
}

I bsgs(const Curve &C, Point P, Point Q, I r) {
  I w = I(std::sqrt(r)) + 1;
  std::map<Point, I> table;
  Point R;
  for (I j = 0; j < w; ++j) {
    table.emplace(R, j);
    R = C.add(R, P);
  }
  auto giant = C.neg(C.scalar(w, P));
  R = Q;
  for (I i = 0; i <= w; ++i) {
    if (table.count(R)) {
      I answer = (i * w + table[R]) % r;
      require(C.scalar(answer, P) == Q, "BSGS verification");
      return answer;
    }
    R = C.add(R, giant);
  }
  throw std::runtime_error("BSGS missing log");
}
Json no_transfer() {
  I p = 1009, a = -1, b = -1;
  bool found = false;
  for (I aa = 0; aa < 30 && !found; ++aa)
    for (I bb = 0; bb < p; ++bb)
      if (mod(4 * aa * aa * aa + 27 * bb * bb, p) && count(p, aa, bb) == 1019) {
        a = aa;
        b = bb;
        found = true;
        break;
      }
  require(found, "smooth example search");
  I d = nonsquare(p);
  Field k(p), K(p, d);
  Curve E(k, a, b), T(k, a, b, d), L(K, a, b);
  auto base = E.points(), twist = T.points();
  require(base.size() == 1019 && twist.size() == 1001, "smooth orders");
  require(factors(1019) == std::map<I, I>{{1019, 1}},
          "original group prime order");
  auto P = base[1], Q = E.scalar(617, P);
  require(bsgs(E, P, Q, 1019) == 617, "smooth known answer");
  for (auto R : twist) {
    if (!R.infinity())
      require(!T.scalar(1019, R).infinity(), "no rational r torsion");
    require(L.scalar(1001, image(R, K)).infinity(), "twist exponent");
  }
  require(!L.scalar(1001, P).infinity(), "original exponent");
  auto ff = obj();
  for (auto [ell, e] : factors(1001))
    ff.fields[std::to_string(ell)] = num(e);
  auto smooth =
      obj({{"p", num(p)},
           {"a", num(a)},
           {"b", num(b)},
           {"d", num(d)},
           {"curve_order", num(1019)},
           {"twist_order", num(1001)},
           {"twist_factorization", ff},
           {"P", str(E.fmt(P))},
           {"Q", str(E.fmt(Q))},
           {"recovered_log", num(617)},
           {"twist_r_torsion_points", num(1)},
           {"theorem", str("Every homomorphism from the order-1019 group to "
                           "the order-1001 group is zero.")}});
  p = 11;
  found = false;
  for (I aa = 0; aa < p && !found; ++aa)
    for (I bb = 0; bb < p; ++bb)
      if (mod(4 * aa * aa * aa + 27 * bb * bb, p) && count(p, aa, bb) == 13) {
        a = aa;
        b = bb;
        found = true;
        break;
      }
  require(found, "anomalous search");
  Field f(p);
  Curve A(f, a, b, nonsquare(p));
  require(A.points().size() == 11, "anomalous twist");
  return obj(
      {{"smooth_twist", smooth},
       {"anomalous_twist", obj({{"p", num(p)},
                                {"a", num(a)},
                                {"b", num(b)},
                                {"curve_order", num(13)},
                                {"twist_order", num(11)},
                                {"trace", num(-1)},
                                {"original_anomalous", truth(false)}})}});
}
Json subfield() {
  auto K = std::make_shared<Field>(7, nonsquare(7));
  I delta = 7;
  while (K->pow(delta, 24) != K->neg(1))
    ++delta;
  Field F(K, delta), k(7);
  I aa = K->pow(delta, 2), bb = K->pow(delta, 3);
  Curve E0(k, 1, 1), E0K(*K, 1, 1), E0F(F, 1, 1), T(*K, aa, bb);
  auto pts = T.points(), base = E0K.points(), full = E0F.points();
  require(E0.points().size() == 5 && base.size() == 55 && pts.size() == 45 &&
              full.size() == 55 * 45,
          "subfield orders");
  require(aa >= 7 || bb >= 7, "coefficients leave subfield");
  auto j = [](const Curve &C) {
    auto &f = C.f;
    return f.div(
        f.mul(1728 % f.p, f.mul(4 % f.p, f.pow(C.a, 3))),
        f.add(f.mul(4 % f.p, f.pow(C.a, 3)), f.mul(27 % f.p, f.pow(C.b, 2))));
  };
  require(j(T) == j(E0K) && j(T) < 7, "subfield j");
  I v = K->size;
  require(F.pow(v, 2) == delta, "splitting element");
  I ix = F.inv(delta), iy = F.inv(F.mul(delta, v));
  auto psi = [&](Point P) {
    return P.infinity() ? P : Point{F.mul(P.x, ix), F.mul(P.y, iy)};
  };
  std::set<Point> images, anti;
  for (auto P : pts)
    images.insert(psi(P));
  for (auto P : full) {
    if (E0F.frob(P) == E0F.neg(P))
      anti.insert(P);
    if (!P.infinity())
      require(F.frob(P.x) == F.pow(P.x, K->size), "relative Frobenius");
  }
  require(images == anti, "subfield anti eigenspace");
  for (auto P : pts)
    for (auto Q : pts)
      require(psi(T.add(P, Q)) == E0F.add(psi(P), psi(Q)),
              "subfield group map");
  Point P;
  for (auto U : pts)
    if (!U.infinity() && T.order(U, 45) % 3 == 0) {
      P = T.scalar(T.order(U, 45) / 3, U);
      break;
    }
  auto mapped = psi(P);
  require(!mapped.infinity() && E0F.scalar(3, mapped).infinity() &&
              std::find(base.begin(), base.end(), mapped) == base.end(),
          "target outside rational group");
  return obj({{"p", num(7)},
              {"base_quadratic_nonsquare", num(K->d)},
              {"delta", str(K->fmt(delta))},
              {"source_a", str(K->fmt(aa))},
              {"source_b", str(K->fmt(bb))},
              {"subfield_a", num(1)},
              {"subfield_b", num(1)},
              {"j", num(j(T))},
              {"source_order", num(45)},
              {"subfield_order", num(5)},
              {"subfield_model_over_K_order", num(55)},
              {"subgroup_order", num(3)},
              {"source_point", str(T.fmt(P))},
              {"mapped_point", str(E0F.fmt(mapped))},
              {"mapped_point_K_rational", truth(false)},
              {"splitting_field_size", num(F.size)},
              {"group_law_checks", num(2025)},
              {"trace_zero_points_checked", num(I(images.size()))}});
}
Json trace_zero() {
  auto rows = arr();
  for (auto [p, poly] :
       std::vector<std::pair<I, std::vector<I>>>{{5, {3, 0, 1}},
                                                 {5, {1, 1, 0, 1}},
                                                 {5, {1, 4, 0, 0, 0, 1}},
                                                 {7, {1, 1, 0, 1}}}) {
    Field f(p, poly), k(p);
    Curve C(f, 1, 1), B(k, 1, 1);
    auto pts = C.points(), bp = B.points();
    std::set<Point> base(bp.begin(), bp.end()), kernel, traces, anti;
    for (auto P : pts) {
      Point tr, U = P;
      for (I i = 0; i < f.n; ++i) {
        tr = C.add(tr, U);
        U = C.frob(U);
      }
      require(U == P && base.count(tr), "trace rationality");
      traces.insert(tr);
      if (tr.infinity())
        kernel.insert(P);
      anti.insert(C.add(C.frob(P), C.neg(P)));
    }
    require(kernel == anti && traces == base &&
                kernel.size() * base.size() == pts.size(),
            "trace kernel/image");
    I t = p + 1 - I(base.size()), s0 = 2, s1 = t;
    for (I i = 2; i <= f.n; ++i) {
      I next = t * s1 - p * s0;
      s0 = s1;
      s1 = next;
    }
    require(I(pts.size()) == f.size + 1 - s1, "Frobenius recurrence");
    I dim = f.n - 1;
    Json ic, generic;
    generic.kind = Json::Number;
    generic.text = dim == 1 ? "0.5" : std::to_string(dim / 2);
    if (dim >= 2) {
      ic.kind = Json::Number;
      ic.text = dim == 2 ? "1" : "1.5";
    }
    rows.items.push_back(
        obj({{"p", num(p)},
             {"extension_degree", num(f.n)},
             {"modulus", nums(poly)},
             {"base_order", num(I(base.size()))},
             {"extension_order", num(I(pts.size()))},
             {"trace_zero_order", num(I(kernel.size()))},
             {"dimension", num(dim)},
             {"trace_surjective", truth()},
             {"frobenius_minus_identity_image_equals_kernel", truth()},
             {"ic_q_exponent", ic},
             {"generic_q_exponent", generic},
             {"exponent_values_are_theoretical_not_runtime_measurements",
              truth()}}));
    std::cout << "PASS trace-zero p=" << p << ", n=" << f.n
              << ": kernel=" << kernel.size() << '\n'
              << std::flush;
  }
  return rows;
}

I line(const Curve &C, Point A, Point B, Point X) {
  require(C.c == 1 && !A.infinity() && !B.infinity() && !X.infinity(),
          "Miller input");
  auto &f = C.f;
  if (A.x == B.x && f.add(A.y, B.y) == 0)
    return f.sub(X.x, A.x);
  I m = A == B ? f.div(f.add(f.mul(3 % f.p, f.pow(A.x, 2)), C.a),
                       f.mul(2 % f.p, A.y))
               : f.div(f.sub(B.y, A.y), f.sub(B.x, A.x));
  I numerator = f.sub(f.sub(X.y, A.y), f.mul(m, f.sub(X.x, A.x)));
  auto AB = C.add(A, B);
  return f.div(numerator, AB.infinity() ? 1 : f.sub(X.x, AB.x));
}
I miller(const Curve &C, Point P, Point X, I n) {
  I value = 1, bit = 1;
  Point V = P;
  while (bit <= n / 2)
    bit *= 2;
  for (bit /= 2; bit; bit /= 2) {
    value = C.f.mul(C.f.mul(value, value), line(C, V, V, X));
    V = C.add(V, V);
    if (n & bit) {
      value = C.f.mul(value, line(C, V, P, X));
      V = C.add(V, P);
    }
  }
  require(V.infinity(), "Miller endpoint");
  return value;
}
I weil(const Curve &C, Point P, Point S, I r) {
  if (P.infinity() || S.infinity())
    return 1;
  I z = C.f.div(miller(C, P, S, r), miller(C, S, P, r));
  return r % 2 ? C.f.neg(z) : z;
}
struct LinearSystem {
  I p, width;
  std::map<I, std::vector<I>> rows;
  bool insert(std::vector<I> row, I rhs) {
    for (auto &v : row)
      v = mod(v, p);
    row.push_back(mod(rhs, p));
    for (auto &[i, old] : rows) {
      I c = row[i];
      for (size_t j = 0; j < row.size(); ++j)
        row[j] = mod(row[j] - c * old[j], p);
    }
    I pivot = 0;
    while (pivot < width && row[pivot] == 0)
      ++pivot;
    if (pivot == width) {
      require(row.back() == 0, "inconsistent relations");
      return false;
    }
    I inv = ipow(row[pivot], p - 2, p);
    for (auto &x : row)
      x = x * inv % p;
    for (auto &[i, old] : rows) {
      (void)i;
      I c = old[pivot];
      for (size_t j = 0; j < old.size(); ++j)
        old[j] = mod(old[j] - c * row[j], p);
    }
    rows[pivot] = row;
    return true;
  }
  std::vector<I> solution() const {
    require(I(rows.size()) == width, "rank deficient");
    std::vector<I> r;
    for (I i = 0; i < width; ++i)
      r.push_back(rows.at(i).back());
    return r;
  }
};
Json index_calculus(const Field &K, I g, I target, I r, I seed) {
  I p = K.p, h = (K.size - 1) / r;
  require(std::gcd(h, r) == 1, "projection invertibility");
  std::vector<I> cs, elements;
  for (I c = 1; c < std::min(I(7), p); ++c) {
    cs.push_back(c);
    elements.push_back(p + c);
  }
  I w = I(cs.size());
  LinearSystem sys{r, w, {}};
  Random rng(seed);
  I attempts = 0, smooths = 0;
  auto independent = arr();
  auto smooth = [&](I z) {
    I lead = z / p;
    if (!lead)
      return I(-1);
    I c = z % p * ipow(lead, p - 2, p) % p;
    auto it = std::find(cs.begin(), cs.end(), c);
    return it == cs.end() ? I(-2) : I(it - cs.begin());
  };
  auto product = [&](I z) {
    std::vector<I> exps(w);
    I terms = 1 + rng.below(w);
    const std::array<I, 6> choices{-3, -2, -1, 1, 2, 3};
    for (I j = 0; j < terms; ++j) {
      I i = rng.below(w), e = choices[rng.below(6)];
      exps[i] += e;
      z = K.mul(z, K.pow(elements[i], e));
    }
    return std::make_pair(z, exps);
  };
  while (I(sys.rows.size()) < w && attempts < 100000) {
    ++attempts;
    I e = rng.below(r);
    auto [z, exps] = product(K.pow(g, e));
    I j = smooth(z);
    if (j == -2)
      continue;
    std::vector<I> row;
    for (I x : exps)
      row.push_back(-x);
    if (j != -1)
      ++row[j];
    ++smooths;
    if (sys.insert(row, e * h))
      independent.items.push_back(
          obj({{"coefficients", nums(row)}, {"rhs", num(e * h % r)}}));
  }
  auto logs = sys.solution();
  for (I i = 0; i < w; ++i)
    require(K.pow(g, logs[i]) == K.pow(elements[i], h), "new factor base logs");
  for (I descent = 1; descent <= 100000; ++descent) {
    I e = rng.below(r);
    auto [z, exps] = product(K.mul(target, K.pow(g, e)));
    I j = smooth(z);
    if (j == -2)
      continue;
    I value = (j == -1 ? 0 : logs[j]) - e * h;
    for (I i = 0; i < w; ++i)
      value -= exps[i] * logs[i];
    I answer = mod(value * ipow(h % r, r - 2, r), r);
    require(K.pow(g, answer) == target, "new individual log verification");
    auto fb = arr();
    for (I c : cs)
      fb.items.push_back(str("u+" + std::to_string(c)));
    return obj({{"recovered_log", num(answer)},
                {"factor_base", fb},
                {"projected_factor_base_logs", nums(logs)},
                {"projection_exponent", num(h)},
                {"relation_attempts", num(attempts)},
                {"smooth_relations", num(smooths)},
                {"relation_rank", num(I(sys.rows.size()))},
                {"independent_relations", independent},
                {"individual_log_attempts", num(descent)}});
  }
  throw std::runtime_error("individual log budget exhausted");
}
Json restricted(const Field &K, I g, I r) {
  I p = K.p, h = (K.size - 1) / r;
  LinearSystem sys{r, 6, {}};
  I attempts = 0, smooths = 0;
  for (I a = 0; a < r; ++a)
    for (I i = 0; i < 6; ++i)
      for (I j = 0; j < 6; ++j)
        for (I e1 : {-1, 1})
          for (I e2 : {-1, 1}) {
            I z = K.mul(K.pow(g, a),
                        K.mul(K.pow(p + 1 + i, e1), K.pow(p + 1 + j, e2)));
            ++attempts;
            I lead = z / p, c = lead ? z % p * ipow(lead, p - 2, p) % p : -1;
            if (lead && (c < 1 || c > 6))
              continue;
            std::vector<I> row(6);
            row[i] -= e1;
            row[j] -= e2;
            if (lead)
              ++row[c - 1];
            ++smooths;
            sys.insert(row, a * h);
          }
  return obj({{"attempts", num(attempts)},
              {"smooth_relations", num(smooths)},
              {"rank", num(I(sys.rows.size()))},
              {"unknowns", num(6)},
              {"exhaustive_within_stated_family", truth()}});
}
Json pairing(I p, I r) {
  require(p % 4 == 3 && (p + 1) % r == 0, "pairing parameters");
  I d = nonsquare(p);
  Field k(p), K(p, d);
  Curve E(k, 1, 0), T(k, 1, 0, d), L(K, 1, 0);
  auto base = E.points(), twist = T.points();
  require(I(base.size()) == p + 1 && I(twist.size()) == p + 1,
          "pairing orders");
  Point P, S0;
  for (auto U : base)
    if (!(P = E.scalar((p + 1) / r, U)).infinity())
      break;
  for (auto U : twist)
    if (!(S0 = T.scalar((p + 1) / r, U)).infinity())
      break;
  auto S = image(S0, K);
  require(L.scalar(r, P).infinity() && L.scalar(r, S).infinity() &&
              L.frob(P) == P && L.frob(S) == L.neg(S),
          "pairing eigenspaces");
  I g = weil(L, P, S, r);
  require(g != 1 && K.pow(g, r) == 1 && K.pow(g, p) == K.inv(g),
          "pairing generator");
  for (I a = 0; a < r; ++a)
    for (I b = 0; b < r; ++b)
      require(weil(L, L.scalar(a, P), L.scalar(b, S), r) == K.pow(g, a * b),
              "bilinearity");
  I secret = r * 2 / 3;
  auto Q = L.scalar(secret, P);
  I target = weil(L, Q, S, r);
  auto ic = index_calculus(K, g, target, r, p * 1000 + r);
  require(ic.at("recovered_log").integer() == secret &&
              bsgs(L, P, Q, r) == secret,
          "known answer pairing recovery");
  auto all_certificates = arr();
  for (I a = 0; a < r; ++a) {
    auto recovered =
        index_calculus(K, g, weil(L, L.scalar(a, P), S, r), r, 10000 * p + a);
    require(recovered.at("recovered_log").integer() == a,
            "complete IC recovery");
    all_certificates.items.push_back(
        obj({{"scalar", num(a)},
             {"target", str(K.fmt(weil(L, L.scalar(a, P), S, r)))},
             {"index_calculus", recovered}}));
  }
  Json rest;
  if (p == 67) {
    rest = restricted(K, g, r);
    require(rest.at("rank").integer() == 5, "restricted rank");
  }
  std::cout << "PASS pairing/IC p=" << p << ": all " << r << " logs recovered\n"
            << std::flush;
  return obj({{"p", num(p)},
              {"d", num(d)},
              {"a", num(1)},
              {"b", num(0)},
              {"r", num(r)},
              {"embedding_degree", num(2)},
              {"P", str(L.fmt(P))},
              {"S0", str(T.fmt(S0))},
              {"S", str(L.fmt(S))},
              {"Q", str(L.fmt(Q))},
              {"pairing_generator", str(K.fmt(g))},
              {"pairing_target", str(K.fmt(target))},
              {"secret", num(secret)},
              {"bilinearity_checks", num(r * r)},
              {"all_logs_recovered", num(r)},
              {"index_calculus", ic},
              {"all_log_certificates", all_certificates},
              {"restricted_two_factor_family", rest}});
}

Json collection() {
  auto rows = arr();
  for (I p : {101, 211, 401, 809}) {
    Field k(p), K(p, nonsquare(p));
    I a = -1, b = -1, r = -1;
    bool found = false;
    for (I aa = 0; aa < 20 && !found; ++aa)
      for (I bb = 0; bb < p; ++bb) {
        if (!mod(4 * aa * aa * aa + 27 * bb * bb, p))
          continue;
        I n = count(p, aa, bb);
        if (n != p && n > p / 2 && factors(n) == std::map<I, I>{{n, 1}}) {
          a = aa;
          b = bb;
          r = n;
          found = true;
          break;
        }
      }
    require(found, "collection curve search");
    Curve E(K, a, b), T(K, a, b, K.d), base(k, a, b);
    auto points = base.points();
    auto target = E.scalar(r / 3, points[1]);
    I invu = K.inv(p), bound = I(std::sqrt(p));
    auto transport = [&](Point R) {
      return R.infinity() ? R : Point{R.x, K.mul(invu, R.y)};
    };
    auto normalized = [&](Point R) {
      auto S = transport(R);
      return S.infinity() ? S
                          : Point{K.mul(K.d, S.x), K.mul(K.mul(K.d, K.d), S.y)};
    };
    Curve normalized_twist(K, K.mul(a, K.pow(K.d, 2)), K.mul(b, K.pow(K.d, 3)));
    require(normalized_twist.contains(normalized(points[1])) &&
                normalized_twist.scalar(r, normalized(points[1])).infinity(),
            "normalized generator order");
    std::vector<Point> B, BT;
    for (auto R : points)
      if (!R.infinity() && R.x < bound) {
        B.push_back(R);
        BT.push_back(transport(R));
      }
    I hits = 0, thits = 0, checks = 0;
    std::map<Point, I> hist, thist, mapped;
    for (size_t i = 0; i < B.size(); ++i)
      for (size_t j = 0; j < B.size(); ++j) {
        auto A = E.add(B[i], B[j]), C = T.add(BT[i], BT[j]);
        require(transport(A) == C, "pair transport");
        ++hist[A];
        ++thist[C];
        hits += A == target;
        thits += C == transport(target);
        ++checks;
      }
    for (auto [A, n] : hist)
      mapped[transport(A)] = n;
    require(mapped == thist && hits == thits,
            "full relation histogram equality");
    rows.items.push_back(
        obj({{"p", num(p)},
             {"a", num(a)},
             {"b", num(b)},
             {"r", num(r)},
             {"x_bound", num(bound)},
             {"factor_base_size", num(I(B.size()))},
             {"ordered_pairs", num(checks)},
             {"original_target_hits", num(hits)},
             {"transported_twist_target_hits", num(thits)},
             {"generator", str(E.fmt(points[1]))},
             {"target", str(E.fmt(target))},
             {"transported_generator_normalized",
              str(E.fmt(normalized(points[1])))},
             {"transported_target_normalized", str(E.fmt(normalized(target)))},
             {"full_relation_histograms_equal", truth()}}));
  }
  return rows;
}

// Exact sparse polynomial arithmetic over Z for the historical symbolic claims.
using Monomial = std::array<int, 9>;
struct Poly {
  std::map<Monomial, I> terms;
  Poly(I c = 0) {
    if (c)
      terms[{}] = c;
  }
  static Poly var(int i) {
    Poly p;
    Monomial m{};
    m[i] = 1;
    p.terms[m] = 1;
    return p;
  }
  Poly operator+(const Poly &b) const {
    Poly r = *this;
    for (auto [m, c] : b.terms) {
      r.terms[m] += c;
      if (!r.terms[m])
        r.terms.erase(m);
    }
    return r;
  }
  Poly operator-() const {
    Poly r = *this;
    for (auto &[m, c] : r.terms) {
      (void)m;
      c = -c;
    }
    return r;
  }
  Poly operator-(const Poly &b) const { return *this + -b; }
  Poly operator*(const Poly &b) const {
    Poly r;
    for (auto [m, c] : terms)
      for (auto [n, d] : b.terms) {
        Monomial k{};
        for (int i = 0; i < 9; ++i)
          k[i] = m[i] + n[i];
        r.terms[k] += c * d;
        if (!r.terms[k])
          r.terms.erase(k);
      }
    return r;
  }
};
Poly sq(const Poly &p) { return p * p; }
Json symbolic() {
  auto x = Poly::var(0), y = Poly::var(1), z = Poly::var(2), a = Poly::var(3),
       b = Poly::var(4), d = Poly::var(5), m = Poly::var(6), n = Poly::var(7),
       u = Poly::var(8);
  auto canonical = sq(x - y) * sq(z) -
                   Poly(2) * ((x + y) * (x * y + a) + Poly(2) * b) * z +
                   sq(x * y - a) - Poly(4) * b * (x + y);
  auto symmetric =
      sq(x * y + x * z + y * z - a) - Poly(4) * (x + y + z) * (x * y * z + b);
  require((canonical - symmetric).terms.empty(),
          "integer symmetric S3 identity");
  require((sq((a - Poly(2) * d * m * n) - a) -
           Poly(4) * (d * m * m) * (d * n * n - b + b))
              .terms.empty(),
          "integer Vieta cancellation");
  auto f = x * x * x + a * x + b;
  require((sq(u) * f - d * f - (sq(u) - d) * f).terms.empty(),
          "integer isomorphism equation");
  return obj({{"canonical_equals_symmetric", truth()},
              {"twist_parameter_cancels", truth()},
              {"isomorphism_equation", truth()},
              {"symmetric_s3", str("(s2-a)^2 - 4*s1*(s3+b)")},
              {"vieta", obj({{"s1", str("d*m^2")},
                             {"s2", str("a-2*d*m*n")},
                             {"s3", str("d*n^2-b")}})}});
}

void compare(const Json &actual, const Json &expected, const std::string &path,
             I &leaves) {
  if (expected.kind == Json::Object) {
    require(actual.kind == Json::Object, "object parity at " + path);
    for (auto &[k, v] : expected.fields) {
      if (k == "elapsed_seconds" || k == "total_seconds" ||
          k == "total_verification_seconds" || k == "source_sha256" ||
          k == "sympy_version")
        continue;
      require(actual.fields.count(k), "missing parity field " + path + "/" + k);
      compare(actual.fields.at(k), v, path + "/" + k, leaves);
    }
    return;
  }
  if (expected.kind == Json::Array) {
    require(actual.kind == Json::Array &&
                actual.items.size() == expected.items.size(),
            "array parity " + path);
    for (size_t i = 0; i < expected.items.size(); ++i)
      compare(actual.items[i], expected.items[i],
              path + "/" + std::to_string(i), leaves);
    return;
  }
  bool equal = actual.kind == expected.kind &&
               (expected.kind == Json::Null || actual.text == expected.text);
  if (actual.kind == Json::Number && expected.kind == Json::Number)
    equal = std::stold(actual.text) == std::stold(expected.text);
  require(equal, "parity mismatch at " + path + ": native " + dump(actual) +
                     ", historical " + dump(expected));
  ++leaves;
}
#ifndef TWIST_FULL_NO_MAIN
int main(int argc, char **argv) {
  try {
    require(argc == 4, "usage: full HISTORICAL_RESULTS OUTPUT_RESULTS RECEIPT");
    auto result =
        obj({{"scope",
              str("quadratic twists, characteristic > 3; all points valid")},
             {"seed_policy", str("fixed integer seeds embedded in source")}});
    result.fields["symbolic"] = symbolic();
    result.fields["exhaustive"] = exhaustive();
    result.fields["no_transfer"] = no_transfer();
    result.fields["subfield_descent"] = subfield();
    result.fields["trace_zero"] = trace_zero();
    auto pairs = arr();
    for (auto [p, r] :
         std::vector<std::pair<I, I>>{{43, 11}, {67, 17}, {211, 53}})
      pairs.items.push_back(pairing(p, r));
    result.fields["pairings"] = pairs;
    result.fields["transported_relation_collection"] = collection();
    I leaves = 0;
    compare(result, parse(read(argv[1])), "", leaves);
    std::ofstream out(argv[2]);
    require(bool(out), "output");
    out << dump(result) << '\n';
    out.close();
    auto receipt =
        obj({{"schema", str("quadratic-twist-native-full-parity/v1")},
             {"status", str("passed")},
             {"deterministic_leaf_fields_compared", num(leaves)},
             {"historical_results_sha256", str(digest(read(argv[1])))},
             {"native_results_sha256", str(digest(read(argv[2])))},
             {"full_942_model_census_rerun", truth()},
             {"native_relation_collection_rerun", truth()},
             {"native_complete_ic_recoveries", num(81)},
             {"formal_general_hardness_proof", truth(false)},
             {"performance_claim", Json{}}});
    std::ofstream cert(argv[3]);
    require(bool(cert), "receipt");
    cert << dump(receipt) << '\n';
    std::cout << "PASS complete deterministic parity: " << leaves
              << " leaf fields\n";
    return 0;
  } catch (const std::exception &e) {
    std::cerr << "FAIL: " << e.what() << '\n';
    return 1;
  }
}
#endif
