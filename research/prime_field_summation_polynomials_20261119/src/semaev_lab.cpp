#include <algorithm>
#include <array>
#include <bit>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <limits>
#include <map>
#include <numeric>
#include <optional>
#include <random>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

using u64 = std::uint64_t;
__extension__ typedef unsigned __int128 u128;
using Poly = std::vector<u64>; // coefficients, low degree first

struct Field {
    enum class Reduction { Auto, Generic };
    enum class Reducer { Generic, Barrett32, Mersenne31 };

    u64 p;
    Reducer reducer;
    u64 barrett_mu;

    explicit Field(u64 modulus, Reduction policy = Reduction::Auto)
        : p(modulus), reducer(Reducer::Generic), barrett_mu(0) {
        if (policy == Reduction::Generic) return;
        if (p == 2147483647ULL) {
            reducer = Reducer::Mersenne31;
        } else if (p < (1ULL << 32)) {
            reducer = Reducer::Barrett32;
            barrett_mu = static_cast<u64>((static_cast<u128>(1) << 64) / p);
        }
    }

    u64 add(u64 a, u64 b) const {
        const u64 gap = p - b;
        return a >= gap ? a - gap : a + b;
    }
    u64 sub(u64 a, u64 b) const { return a >= b ? a - b : p - (b - a); }
    u64 neg(u64 a) const { return a == 0 ? 0 : p - a; }
    u64 mul(u64 a, u64 b) const {
        if (reducer == Reducer::Mersenne31) {
            const u64 x = a * b; // a,b < 2^31-1, so x < 2^62
            u64 r = (x & 2147483647ULL) + (x >> 31);
            if (r >= 2147483647ULL) r -= 2147483647ULL;
            return r;
        }
        if (reducer == Reducer::Barrett32) {
            const u64 x = a * b; // p < 2^32, so x < 2^64
            const u64 q = static_cast<u64>((static_cast<u128>(x) * barrett_mu) >> 64);
            u64 r = x - q * p;
            if (r >= p) r -= p;
            return r;
        }
        return static_cast<u64>((static_cast<u128>(a) * b) % p);
    }
    u64 sqr(u64 a) const { return mul(a, a); }
    u64 pow(u64 a, u64 e) const {
        u64 r = 1;
        while (e != 0) {
            if ((e & 1U) != 0) r = mul(r, a);
            e >>= 1U;
            if (e != 0) a = sqr(a);
        }
        return r;
    }
    u64 inv(u64 a) const {
        if (a == 0) throw std::runtime_error("division by zero");
        return pow(a, p - 2);
    }
    int legendre(u64 a) const {
        if (a == 0) return 0;
        const u64 z = pow(a, (p - 1) / 2);
        return z == 1 ? 1 : -1;
    }
    std::optional<u64> sqrt(u64 n) const {
        if (n == 0) return 0;
        if (legendre(n) != 1) return std::nullopt;
        if ((p & 3U) == 3U) return pow(n, (p + 1) / 4);

        u64 q = p - 1;
        unsigned s = 0;
        while ((q & 1U) == 0) { q >>= 1U; ++s; }
        u64 z = 2;
        while (legendre(z) != -1) ++z;
        u64 c = pow(z, q);
        u64 x = pow(n, (q + 1) / 2);
        u64 t = pow(n, q);
        unsigned m = s;
        while (t != 1) {
            unsigned i = 1;
            u64 tt = sqr(t);
            while (i < m && tt != 1) { tt = sqr(tt); ++i; }
            if (i == m) throw std::runtime_error("Tonelli-Shanks invariant failure");
            u64 b = c;
            for (unsigned j = 0; j < m - i - 1; ++j) b = sqr(b);
            x = mul(x, b);
            const u64 b2 = sqr(b);
            t = mul(t, b2);
            c = b2;
            m = i;
        }
        return x;
    }
};

u64 mul_mod_raw(u64 a, u64 b, u64 m) {
    return static_cast<u64>((static_cast<u128>(a) * b) % m);
}

u64 pow_mod_raw(u64 a, u64 e, u64 m) {
    u64 r = 1 % m;
    while (e != 0) {
        if ((e & 1U) != 0) r = mul_mod_raw(r, a, m);
        e >>= 1U;
        if (e != 0) a = mul_mod_raw(a, a, m);
    }
    return r;
}

bool is_prime(u64 n) {
    if (n < 2) return false;
    for (u64 q : {2ULL, 3ULL, 5ULL, 7ULL, 11ULL, 13ULL, 17ULL, 19ULL, 23ULL, 29ULL, 31ULL, 37ULL}) {
        if (n % q == 0) return n == q;
    }
    u64 d = n - 1;
    unsigned s = 0;
    while ((d & 1U) == 0) { d >>= 1U; ++s; }
    for (u64 a : {2ULL, 325ULL, 9375ULL, 28178ULL, 450775ULL, 9780504ULL, 1795265022ULL}) {
        if (a % n == 0) continue;
        u64 x = pow_mod_raw(a % n, d, n);
        if (x == 1 || x == n - 1) continue;
        bool witness = true;
        for (unsigned r = 1; r < s; ++r) {
            x = mul_mod_raw(x, x, n);
            if (x == n - 1) { witness = false; break; }
        }
        if (witness) return false;
    }
    return true;
}

struct Point {
    u64 x = 0;
    u64 y = 0;
    bool inf = true;
    bool operator==(const Point&) const = default;
};

struct Curve {
    Field f;
    u64 a;
    u64 b;

    bool nonsingular() const {
        const u64 a3 = f.mul(f.sqr(a), a);
        const u64 b2 = f.sqr(b);
        return f.add(f.mul(4, a3), f.mul(27, b2)) != 0;
    }
    u64 rhs(u64 x) const { return f.add(f.add(f.mul(f.sqr(x), x), f.mul(a, x)), b); }
    bool on_curve(const Point& p) const { return p.inf || f.sqr(p.y) == rhs(p.x); }
    Point neg(Point p) const {
        if (!p.inf) p.y = f.neg(p.y);
        return p;
    }
    Point add(const Point& p, const Point& q) const {
        if (p.inf) return q;
        if (q.inf) return p;
        if (p.x == q.x) {
            if (f.add(p.y, q.y) == 0) return {};
            if (p.y == 0) return {};
            const u64 num = f.add(f.mul(3, f.sqr(p.x)), a);
            const u64 den = f.mul(2, p.y);
            const u64 lam = f.mul(num, f.inv(den));
            const u64 xr = f.sub(f.sqr(lam), f.add(p.x, q.x));
            const u64 yr = f.sub(f.mul(lam, f.sub(p.x, xr)), p.y);
            return {xr, yr, false};
        }
        const u64 lam = f.mul(f.sub(q.y, p.y), f.inv(f.sub(q.x, p.x)));
        const u64 xr = f.sub(f.sub(f.sqr(lam), p.x), q.x);
        const u64 yr = f.sub(f.mul(lam, f.sub(p.x, xr)), p.y);
        return {xr, yr, false};
    }
    Point dbl(const Point& p) const { return add(p, p); }
    Point random_point(std::mt19937_64& rng) const {
        std::uniform_int_distribution<u64> dist(0, f.p - 1);
        for (;;) {
            const u64 x = dist(rng);
            auto y = f.sqrt(rhs(x));
            if (y) {
                u64 yy = *y;
                if ((rng() & 1U) != 0) yy = f.neg(yy);
                return {x, yy, false};
            }
        }
    }
};

void trim(Poly& a) {
    while (a.size() > 1 && a.back() == 0) a.pop_back();
    if (a.empty()) a.push_back(0);
}

bool is_zero(const Poly& a) { return a.size() == 1 && a[0] == 0; }
std::size_t degree(const Poly& a) { return is_zero(a) ? 0 : a.size() - 1; }

Poly poly_add(const Poly& a, const Poly& b, const Field& f) {
    Poly r(std::max(a.size(), b.size()), 0);
    for (std::size_t i = 0; i < r.size(); ++i) {
        const u64 x = i < a.size() ? a[i] : 0;
        const u64 y = i < b.size() ? b[i] : 0;
        r[i] = f.add(x, y);
    }
    trim(r); return r;
}

Poly poly_sub(const Poly& a, const Poly& b, const Field& f) {
    Poly r(std::max(a.size(), b.size()), 0);
    for (std::size_t i = 0; i < r.size(); ++i) {
        const u64 x = i < a.size() ? a[i] : 0;
        const u64 y = i < b.size() ? b[i] : 0;
        r[i] = f.sub(x, y);
    }
    trim(r); return r;
}

Poly poly_mul(const Poly& a, const Poly& b, const Field& f) {
    if (is_zero(a) || is_zero(b)) return {0};
    Poly r(a.size() + b.size() - 1, 0);
    for (std::size_t i = 0; i < a.size(); ++i)
        for (std::size_t j = 0; j < b.size(); ++j)
            r[i + j] = f.add(r[i + j], f.mul(a[i], b[j]));
    trim(r); return r;
}

constexpr std::size_t KARATSUBA_THRESHOLD = 32;

Poly poly_slice(const Poly& a, std::size_t begin, std::size_t end) {
    if (begin >= a.size() || begin >= end) return {0};
    end = std::min(end, a.size());
    Poly r(a.begin() + static_cast<std::ptrdiff_t>(begin),
           a.begin() + static_cast<std::ptrdiff_t>(end));
    trim(r);
    return r;
}

void poly_accumulate_shifted(Poly& out, const Poly& in, std::size_t shift,
                             const Field& f, bool subtract = false) {
    if (is_zero(in)) return;
    if (out.size() < in.size() + shift) out.resize(in.size() + shift, 0);
    for (std::size_t i = 0; i < in.size(); ++i) {
        out[i + shift] = subtract ? f.sub(out[i + shift], in[i])
                                  : f.add(out[i + shift], in[i]);
    }
}

Poly poly_mul_fast(const Poly& a, const Poly& b, const Field& f) {
    if (is_zero(a) || is_zero(b)) return {0};
    const std::size_t small = std::min(a.size(), b.size());
    const std::size_t large = std::max(a.size(), b.size());
    if (small <= KARATSUBA_THRESHOLD || large > 2 * small) return poly_mul(a, b, f);

    const std::size_t split = large / 2;
    const Poly a0 = poly_slice(a, 0, split);
    const Poly a1 = poly_slice(a, split, a.size());
    const Poly b0 = poly_slice(b, 0, split);
    const Poly b1 = poly_slice(b, split, b.size());
    if (is_zero(a1) || is_zero(b1)) return poly_mul(a, b, f);

    const Poly z0 = poly_mul_fast(a0, b0, f);
    const Poly z2 = poly_mul_fast(a1, b1, f);
    Poly z1 = poly_mul_fast(poly_add(a0, a1, f), poly_add(b0, b1, f), f);
    z1 = poly_sub(poly_sub(z1, z0, f), z2, f);

    Poly out(a.size() + b.size() - 1, 0);
    poly_accumulate_shifted(out, z0, 0, f);
    poly_accumulate_shifted(out, z1, split, f);
    poly_accumulate_shifted(out, z2, 2 * split, f);
    trim(out);
    return out;
}

std::pair<Poly, Poly> poly_divmod(Poly a, const Poly& b, const Field& f) {
    if (is_zero(b)) throw std::runtime_error("polynomial division by zero");
    trim(a);
    if (degree(a) < degree(b)) return {{0}, a};
    Poly q(degree(a) - degree(b) + 1, 0);
    const u64 inv_lead = f.inv(b.back());
    while (!is_zero(a) && degree(a) >= degree(b)) {
        const std::size_t shift = degree(a) - degree(b);
        const u64 c = f.mul(a.back(), inv_lead);
        q[shift] = c;
        for (std::size_t i = 0; i < b.size(); ++i)
            a[i + shift] = f.sub(a[i + shift], f.mul(c, b[i]));
        trim(a);
    }
    trim(q); return {q, a};
}

Poly poly_monic(Poly a, const Field& f) {
    trim(a);
    if (is_zero(a)) return a;
    const u64 z = f.inv(a.back());
    for (u64& c : a) c = f.mul(c, z);
    return a;
}

Poly poly_gcd(Poly a, Poly b, const Field& f) {
    while (!is_zero(b)) {
        Poly r = poly_divmod(a, b, f).second;
        a = std::move(b);
        b = std::move(r);
    }
    return poly_monic(a, f);
}

Poly poly_derivative(const Poly& a, const Field& f) {
    if (a.size() <= 1) return {0};
    Poly r(a.size() - 1, 0);
    for (std::size_t i = 1; i < a.size(); ++i) r[i - 1] = f.mul(a[i], static_cast<u64>(i) % f.p);
    trim(r); return r;
}

Poly poly_mod(const Poly& a, const Poly& m, const Field& f) { return poly_divmod(a, m, f).second; }

Poly poly_mul_mod(const Poly& a, const Poly& b, const Poly& m, const Field& f) {
    return poly_mod(poly_mul(a, b, f), m, f);
}

Poly poly_pow_mod(Poly a, u64 e, const Poly& m, const Field& f) {
    Poly r{1};
    a = poly_mod(a, m, f);
    while (e != 0) {
        if ((e & 1U) != 0) r = poly_mul_mod(r, a, m, f);
        e >>= 1U;
        if (e != 0) a = poly_mul_mod(a, a, m, f);
    }
    return r;
}

Poly poly_from_roots(const std::vector<u64>& roots, const Field& f) {
    Poly r{1};
    for (u64 z : roots) {
        r.push_back(0);
        for (std::size_t i = r.size() - 1; i > 0; --i)
            r[i] = f.sub(r[i - 1], f.mul(z, r[i]));
        r[0] = f.neg(f.mul(z, r[0]));
    }
    return r;
}

Poly poly_from_roots_product_tree(const std::vector<u64>& roots, const Field& f) {
    if (roots.empty()) return {1};
    std::vector<Poly> level;
    level.reserve(roots.size());
    for (u64 root : roots) level.push_back({f.neg(root), 1});
    while (level.size() > 1) {
        std::vector<Poly> next;
        next.reserve((level.size() + 1) / 2);
        std::size_t i = 0;
        for (; i + 1 < level.size(); i += 2)
            next.push_back(poly_mul_fast(level[i], level[i + 1], f));
        if (i < level.size()) next.push_back(std::move(level[i]));
        level = std::move(next);
    }
    return std::move(level[0]);
}

constexpr std::size_t PRODUCT_TREE_AUTO_THRESHOLD = 256;

std::string resolved_coefficient_method(std::size_t root_count, const std::string& method) {
    if (method == "auto") return root_count < PRODUCT_TREE_AUTO_THRESHOLD ? "quadratic" : "product";
    return method;
}

Poly poly_from_roots_selected(const std::vector<u64>& roots, const Field& f,
                              const std::string& method) {
    const std::string selected = resolved_coefficient_method(roots.size(), method);
    if (selected == "quadratic") return poly_from_roots(roots, f);
    if (selected == "product") return poly_from_roots_product_tree(roots, f);
    throw std::runtime_error("coefficient method must be auto, quadratic, or product");
}

u64 poly_eval(const Poly& a, u64 x, const Field& f) {
    u64 r = 0;
    for (auto it = a.rbegin(); it != a.rend(); ++it) r = f.add(f.mul(r, x), *it);
    return r;
}

struct RootResult {
    std::vector<u64> finite;
    std::size_t infinity = 0;
};

RootResult roots_naive(const Curve& e, const std::vector<Point>& points) {
    if (points.empty()) throw std::runtime_error("need at least one point");
    const std::size_t k = points.size() - 1;
    if (k >= 63) throw std::runtime_error("arity too large");
    const u64 count = 1ULL << k;
    RootResult out;
    out.finite.reserve(static_cast<std::size_t>(count));
    for (u64 mask = 0; mask < count; ++mask) {
        Point s = points[0];
        for (std::size_t j = 0; j < k; ++j)
            s = e.add(s, ((mask >> j) & 1U) != 0 ? points[j + 1] : e.neg(points[j + 1]));
        if (s.inf) ++out.infinity; else out.finite.push_back(s.x);
    }
    return out;
}

std::vector<Point> enumerate_gray(const Curve& e, Point base, const std::vector<Point>& vars) {
    if (vars.size() >= 63) throw std::runtime_error("too many variables");
    const u64 count = 1ULL << vars.size();
    std::vector<Point> out;
    out.reserve(static_cast<std::size_t>(count));
    Point s = base;
    std::vector<Point> twice;
    twice.reserve(vars.size());
    for (const Point& p : vars) {
        s = e.add(s, e.neg(p));
        twice.push_back(e.dbl(p));
    }
    out.push_back(s);
    u64 old_gray = 0;
    for (u64 i = 1; i < count; ++i) {
        const u64 gray = i ^ (i >> 1U);
        const u64 changed = gray ^ old_gray;
        const unsigned bit = static_cast<unsigned>(std::countr_zero(changed));
        const bool became_one = ((gray >> bit) & 1U) != 0;
        s = e.add(s, became_one ? twice[bit] : e.neg(twice[bit]));
        out.push_back(s);
        old_gray = gray;
    }
    return out;
}

void enumerate_gray_range_into(const Curve& e, Point base, const std::vector<Point>& points,
                               std::size_t begin, std::size_t end,
                               std::vector<Point>& out, std::vector<Point>& twice) {
    if (end < begin || end > points.size()) throw std::runtime_error("invalid Gray range");
    const std::size_t count_bits = end - begin;
    if (count_bits >= 63) throw std::runtime_error("too many variables");
    const std::size_t count = 1ULL << count_bits;
    out.clear();
    out.reserve(count);
    twice.resize(count_bits);
    Point s = base;
    for (std::size_t j = 0; j < count_bits; ++j) {
        const Point& p = points[begin + j];
        s = e.add(s, e.neg(p));
        twice[j] = e.dbl(p);
    }
    out.push_back(s);
    u64 old_gray = 0;
    for (u64 i = 1; i < count; ++i) {
        const u64 gray = i ^ (i >> 1U);
        const u64 changed = gray ^ old_gray;
        const unsigned bit = static_cast<unsigned>(std::countr_zero(changed));
        const bool became_one = ((gray >> bit) & 1U) != 0;
        s = e.add(s, became_one ? twice[bit] : e.neg(twice[bit]));
        out.push_back(s);
        old_gray = gray;
    }
}

RootResult roots_gray(const Curve& e, const std::vector<Point>& points) {
    std::vector<Point> vars(points.begin() + 1, points.end());
    const auto sums = enumerate_gray(e, points[0], vars);
    RootResult out;
    out.finite.reserve(sums.size());
    for (const Point& s : sums) {
        if (s.inf) ++out.infinity; else out.finite.push_back(s.x);
    }
    return out;
}

RootResult roots_mitm_batch(const Curve& e, const std::vector<Point>& points) {
    if (points.empty()) throw std::runtime_error("need at least one point");
    const std::size_t k = points.size() - 1;
    const std::size_t cut = k / 2;
    std::vector<Point> lv(points.begin() + 1, points.begin() + 1 + static_cast<std::ptrdiff_t>(cut));
    std::vector<Point> rv(points.begin() + 1 + static_cast<std::ptrdiff_t>(cut), points.end());
    const auto left = enumerate_gray(e, points[0], lv);
    const auto right = enumerate_gray(e, Point{}, rv);

    const std::size_t total = left.size() * right.size();
    RootResult out;
    out.finite.resize(total);
    std::vector<u64> den(total, 1), num(total, 0), prefix(total, 1);
    std::vector<unsigned char> mode(total, 0); // 0=batch, 1=known finite, 2=infinity
    std::size_t idx = 0;
    for (const Point& l : left) for (const Point& r : right) {
        if (l.inf && r.inf) { mode[idx] = 2; ++idx; continue; }
        if (l.inf) { mode[idx] = 1; out.finite[idx] = r.x; ++idx; continue; }
        if (r.inf) { mode[idx] = 1; out.finite[idx] = l.x; ++idx; continue; }
        if (l.x == r.x && e.f.add(l.y, r.y) == 0) { mode[idx] = 2; ++idx; continue; }
        if (l.x == r.x) {
            if (l.y == 0) { mode[idx] = 2; ++idx; continue; }
            den[idx] = e.f.mul(2, l.y);
            num[idx] = e.f.add(e.f.mul(3, e.f.sqr(l.x)), e.a);
        } else {
            den[idx] = e.f.sub(r.x, l.x);
            num[idx] = e.f.sub(r.y, l.y);
        }
        ++idx;
    }
    u64 product = 1;
    for (std::size_t i = 0; i < total; ++i) {
        prefix[i] = product;
        if (mode[i] == 0) product = e.f.mul(product, den[i]);
    }
    u64 inv_product = e.f.inv(product);
    for (std::size_t i = total; i-- > 0;) {
        if (mode[i] != 0) continue;
        const u64 inv_den = e.f.mul(inv_product, prefix[i]);
        inv_product = e.f.mul(inv_product, den[i]);
        const std::size_t li = i / right.size();
        const std::size_t ri = i % right.size();
        const u64 lam = e.f.mul(num[i], inv_den);
        out.finite[i] = e.f.sub(e.f.sub(e.f.sqr(lam), left[li].x), right[ri].x);
    }
    std::vector<u64> compact;
    compact.reserve(total);
    for (std::size_t i = 0; i < total; ++i) {
        if (mode[i] == 2) ++out.infinity;
        else compact.push_back(out.finite[i]);
    }
    out.finite = std::move(compact);
    return out;
}

std::vector<Point> expand_affine_level_batch(const Curve& e, const std::vector<Point>& current, const Point& q) {
    enum class Mode : unsigned char { regular, double_plus, double_minus, known };
    const std::size_t n = current.size();
    std::vector<Point> next(2 * n);
    std::vector<u64> den(n, 1), prefix(n, 1);
    std::vector<Mode> mode(n, Mode::known);
    u64 product = 1;
    for (std::size_t i = 0; i < n; ++i) {
        const Point& s = current[i];
        if (s.inf) {
            next[2 * i] = q;
            next[2 * i + 1] = e.neg(q);
            continue;
        }
        if (s.x != q.x) {
            den[i] = e.f.sub(q.x, s.x);
            mode[i] = Mode::regular;
        } else if (s.y == q.y) {
            if (s.y == 0) continue;
            den[i] = e.f.mul(2, s.y);
            mode[i] = Mode::double_plus;
        } else {
            if (s.y == 0) continue;
            den[i] = e.f.mul(2, s.y);
            mode[i] = Mode::double_minus;
        }
        prefix[i] = product;
        product = e.f.mul(product, den[i]);
    }
    u64 inv_product = e.f.inv(product);
    for (std::size_t i = n; i-- > 0;) {
        if (mode[i] == Mode::known) continue;
        const u64 inv_den = e.f.mul(inv_product, prefix[i]);
        inv_product = e.f.mul(inv_product, den[i]);
        const Point& s = current[i];
        auto with_slope = [&](u64 lambda) {
            const u64 xr = e.f.sub(e.f.sub(e.f.sqr(lambda), s.x), q.x);
            const u64 yr = e.f.sub(e.f.mul(lambda, e.f.sub(s.x, xr)), s.y);
            return Point{xr, yr, false};
        };
        if (mode[i] == Mode::regular) {
            const u64 plus_lambda = e.f.mul(e.f.sub(q.y, s.y), inv_den);
            const u64 minus_lambda = e.f.mul(e.f.sub(e.f.neg(q.y), s.y), inv_den);
            next[2 * i] = with_slope(plus_lambda);
            next[2 * i + 1] = with_slope(minus_lambda);
        } else {
            const u64 numerator = e.f.add(e.f.mul(3, e.f.sqr(s.x)), e.a);
            const Point doubled = with_slope(e.f.mul(numerator, inv_den));
            if (mode[i] == Mode::double_plus) next[2 * i] = doubled;
            else next[2 * i + 1] = doubled;
        }
    }
    return next;
}

RootResult roots_tree_batch(const Curve& e, const std::vector<Point>& points) {
    if (points.empty()) throw std::runtime_error("need at least one point");
    std::vector<Point> current{points[0]};
    for (std::size_t i = 1; i < points.size(); ++i)
        current = expand_affine_level_batch(e, current, points[i]);
    RootResult out;
    out.finite.reserve(current.size());
    for (const Point& s : current) {
        if (s.inf) ++out.infinity;
        else out.finite.push_back(s.x);
    }
    return out;
}

struct JacobianPoint {
    u64 x = 0;
    u64 y = 1;
    u64 z = 0;
};

JacobianPoint jacobian_double(const Curve& e, const JacobianPoint& p) {
    if (p.z == 0 || p.y == 0) return {};
    const Field& f = e.f;
    const u64 xx = f.sqr(p.x);
    const u64 yy = f.sqr(p.y);
    const u64 yyyy = f.sqr(yy);
    const u64 zz = f.sqr(p.z);
    const u64 s = f.mul(2, f.sub(f.sub(f.sqr(f.add(p.x, yy)), xx), yyyy));
    const u64 m = f.add(f.mul(3, xx), f.mul(e.a, f.sqr(zz)));
    const u64 x3 = f.sub(f.sqr(m), f.mul(2, s));
    const u64 y3 = f.sub(f.mul(m, f.sub(s, x3)), f.mul(8, yyyy));
    const u64 z3 = f.sub(f.sub(f.sqr(f.add(p.y, p.z)), yy), zz);
    return {x3, y3, z3};
}

std::pair<JacobianPoint, JacobianPoint> jacobian_add_pair(const Curve& e, const JacobianPoint& p, const Point& q) {
    if (p.z == 0) return {{q.x, q.y, 1}, {q.x, e.f.neg(q.y), 1}};
    const Field& f = e.f;
    const u64 z2 = f.sqr(p.z);
    const u64 u = f.mul(q.x, z2);
    const u64 s = f.mul(q.y, f.mul(z2, p.z));
    const u64 h = f.sub(u, p.x);
    if (h == 0) {
        const JacobianPoint plus = s == p.y ? jacobian_double(e, p) : JacobianPoint{};
        const JacobianPoint minus = f.neg(s) == p.y ? jacobian_double(e, p) : JacobianPoint{};
        return {plus, minus};
    }
    const u64 hh = f.sqr(h);
    const u64 i = f.mul(4, hh);
    const u64 j = f.mul(h, i);
    const u64 v = f.mul(p.x, i);
    const u64 z3 = f.mul(2, f.mul(p.z, h));
    const u64 twice_yj = f.mul(2, f.mul(p.y, j));
    auto finish = [&](u64 signed_s) {
        const u64 r = f.mul(2, f.sub(signed_s, p.y));
        const u64 x3 = f.sub(f.sub(f.sqr(r), j), f.mul(2, v));
        const u64 y3 = f.sub(f.mul(r, f.sub(v, x3)), twice_yj);
        return JacobianPoint{x3, y3, z3};
    };
    return {finish(s), finish(f.neg(s))};
}

RootResult roots_tree_projective(const Curve& e, const std::vector<Point>& points) {
    if (points.empty()) throw std::runtime_error("need at least one point");
    std::vector<JacobianPoint> current{{points[0].x, points[0].y, 1}};
    for (std::size_t level = 1; level < points.size(); ++level) {
        std::vector<JacobianPoint> next(2 * current.size());
        for (std::size_t i = 0; i < current.size(); ++i) {
            auto [plus, minus] = jacobian_add_pair(e, current[i], points[level]);
            next[2 * i] = plus;
            next[2 * i + 1] = minus;
        }
        current = std::move(next);
    }
    std::vector<u64> prefix(current.size(), 1);
    u64 product = 1;
    for (std::size_t i = 0; i < current.size(); ++i) {
        prefix[i] = product;
        if (current[i].z != 0) product = e.f.mul(product, current[i].z);
    }
    u64 inv_product = e.f.inv(product);
    RootResult out;
    out.finite.resize(current.size());
    std::vector<unsigned char> finite(current.size(), 0);
    for (std::size_t i = current.size(); i-- > 0;) {
        if (current[i].z == 0) { ++out.infinity; continue; }
        const u64 inv_z = e.f.mul(inv_product, prefix[i]);
        inv_product = e.f.mul(inv_product, current[i].z);
        out.finite[i] = e.f.mul(current[i].x, e.f.sqr(inv_z));
        finite[i] = 1;
    }
    std::vector<u64> compact;
    compact.reserve(current.size() - out.infinity);
    for (std::size_t i = 0; i < current.size(); ++i) if (finite[i] != 0) compact.push_back(out.finite[i]);
    out.finite = std::move(compact);
    return out;
}

struct MitmWorkspace {
    std::vector<Point> left;
    std::vector<Point> right;
    std::vector<Point> left_twice;
    std::vector<Point> right_twice;
    std::vector<u64> den;
    std::vector<u64> num;
    std::vector<u64> prefix;
    std::vector<unsigned char> mode;
    std::size_t common_path_calls = 0;
    std::size_t fallback_calls = 0;
};

void roots_mitm_workspace_into(const Curve& e, const std::vector<Point>& points,
                               MitmWorkspace& ws, RootResult& out) {
    if (points.empty()) throw std::runtime_error("need at least one point");
    const std::size_t k = points.size() - 1;
    const std::size_t cut = k / 2;
    enumerate_gray_range_into(e, points[0], points, 1, 1 + cut, ws.left, ws.left_twice);
    enumerate_gray_range_into(e, Point{}, points, 1 + cut, points.size(), ws.right, ws.right_twice);

    const std::size_t right_size = ws.right.size();
    const std::size_t total = ws.left.size() * right_size;
    ws.den.resize(total);
    ws.num.resize(total);
    ws.prefix.resize(total);
    out.finite.resize(total);
    out.infinity = 0;

    bool common = std::none_of(ws.left.begin(), ws.left.end(), [](const Point& p) { return p.inf; })
               && std::none_of(ws.right.begin(), ws.right.end(), [](const Point& p) { return p.inf; });

    if (common) {
        std::size_t idx = 0;
        for (const Point& l : ws.left) for (const Point& r : ws.right) {
            ws.den[idx] = e.f.sub(r.x, l.x);
            ws.num[idx] = e.f.sub(r.y, l.y);
            common &= ws.den[idx] != 0;
            ++idx;
        }
    }
    if (common) {
        ++ws.common_path_calls;
        u64 product = 1;
        for (std::size_t i = 0; i < total; ++i) {
            ws.prefix[i] = product;
            product = e.f.mul(product, ws.den[i]);
        }
        u64 inv_product = e.f.inv(product);
        for (std::size_t li = ws.left.size(); li-- > 0;) {
            for (std::size_t ri = right_size; ri-- > 0;) {
                const std::size_t i = li * right_size + ri;
                const u64 inv_den = e.f.mul(inv_product, ws.prefix[i]);
                inv_product = e.f.mul(inv_product, ws.den[i]);
                const u64 lam = e.f.mul(ws.num[i], inv_den);
                out.finite[i] = e.f.sub(e.f.sub(e.f.sqr(lam), ws.left[li].x), ws.right[ri].x);
            }
        }
        return;
    }

    ++ws.fallback_calls;
    ws.mode.resize(total);
    std::size_t idx = 0;
    for (const Point& l : ws.left) for (const Point& r : ws.right) {
        ws.mode[idx] = 0;
        ws.den[idx] = 1;
        if (l.inf && r.inf) { ws.mode[idx] = 2; ++idx; continue; }
        if (l.inf) { ws.mode[idx] = 1; out.finite[idx] = r.x; ++idx; continue; }
        if (r.inf) { ws.mode[idx] = 1; out.finite[idx] = l.x; ++idx; continue; }
        if (l.x == r.x && e.f.add(l.y, r.y) == 0) { ws.mode[idx] = 2; ++idx; continue; }
        if (l.x == r.x) {
            if (l.y == 0) { ws.mode[idx] = 2; ++idx; continue; }
            ws.den[idx] = e.f.mul(2, l.y);
            ws.num[idx] = e.f.add(e.f.mul(3, e.f.sqr(l.x)), e.a);
        } else {
            ws.den[idx] = e.f.sub(r.x, l.x);
            ws.num[idx] = e.f.sub(r.y, l.y);
        }
        ++idx;
    }
    u64 product = 1;
    for (std::size_t i = 0; i < total; ++i) {
        ws.prefix[i] = product;
        product = e.f.mul(product, ws.den[i]);
    }
    u64 inv_product = e.f.inv(product);
    for (std::size_t li = ws.left.size(); li-- > 0;) {
        for (std::size_t ri = right_size; ri-- > 0;) {
            const std::size_t i = li * right_size + ri;
            if (ws.mode[i] != 0) continue;
            const u64 inv_den = e.f.mul(inv_product, ws.prefix[i]);
            inv_product = e.f.mul(inv_product, ws.den[i]);
            const u64 lam = e.f.mul(ws.num[i], inv_den);
            out.finite[i] = e.f.sub(e.f.sub(e.f.sqr(lam), ws.left[li].x), ws.right[ri].x);
        }
    }
    std::size_t write = 0;
    for (std::size_t i = 0; i < total; ++i) {
        if (ws.mode[i] == 2) ++out.infinity;
        else out.finite[write++] = out.finite[i];
    }
    out.finite.resize(write);
}

RootResult roots_mitm_workspace(const Curve& e, const std::vector<Point>& points) {
    MitmWorkspace ws;
    RootResult out;
    roots_mitm_workspace_into(e, points, ws, out);
    return out;
}

// Bits describe P_2,...,P_(m-1): 1 selects +P_i and 0 selects -P_i.
// P_1 is fixed positive, matching the summation-polynomial root convention.
Point one_signed_sum(const Curve& e, const std::vector<Point>& points, u64 sign_mask) {
    if (points.empty()) throw std::runtime_error("need at least one point");
    Point sum = points[0];
    for (std::size_t j = 1; j < points.size(); ++j)
        sum = e.add(sum, ((sign_mask >> (j - 1)) & 1U) != 0 ? points[j] : e.neg(points[j]));
    return sum;
}

struct PointKey {
    u64 x;
    u64 y;
    bool inf;
    bool operator==(const PointKey&) const = default;
};

struct PointKeyHash {
    std::size_t operator()(const PointKey& p) const noexcept {
        if (p.inf) return 0x9e3779b97f4a7c15ULL;
        u64 h = p.x + 0x9e3779b97f4a7c15ULL;
        h ^= p.y + 0x9e3779b97f4a7c15ULL + (h << 6U) + (h >> 2U);
        return static_cast<std::size_t>(h);
    }
};

PointKey point_key(const Point& p) { return p.inf ? PointKey{0, 0, true} : PointKey{p.x, p.y, false}; }

struct TargetSearchResult {
    bool found = false;
    u64 target_x = 0;
    u64 sign_mask = 0;
    Point sum;
    std::size_t left_count = 0;
    std::size_t right_count = 0;
};

std::size_t target_split_bits(std::size_t sign_bits, std::size_t target_count) {
    std::size_t best = 0;
    long double best_cost = std::numeric_limits<long double>::infinity();
    const long double targets = static_cast<long double>(std::max<std::size_t>(target_count, 1));
    for (std::size_t left_bits = 0; left_bits <= sign_bits; ++left_bits) {
        const std::size_t right_bits = sign_bits - left_bits;
        const long double left = std::ldexp(1.0L, static_cast<int>(left_bits));
        const long double right = std::ldexp(1.0L, static_cast<int>(right_bits));
        // Build/hash the right table; probe it twice per target (Q and -Q) for each left sum.
        const long double cost = right + 2.0L * targets * left;
        if (cost < best_cost) { best_cost = cost; best = left_bits; }
    }
    return best;
}

std::vector<Point> batch_target_differences(const Curve& e, const Point& target,
                                            const std::vector<Point>& left) {
    const std::size_t count = left.size();
    std::vector<Point> out(count);
    std::vector<u64> den(count, 1), num(count, 0), prefix(count, 1);
    std::vector<unsigned char> mode(count, 0); // 0=batch, 1=known result
    u64 product = 1;
    for (std::size_t i = 0; i < count; ++i) {
        const Point rhs = e.neg(left[i]);
        if (target.inf) { out[i] = rhs; mode[i] = 1; continue; }
        if (rhs.inf) { out[i] = target; mode[i] = 1; continue; }
        if (target.x == rhs.x && e.f.add(target.y, rhs.y) == 0) {
            out[i] = {}; mode[i] = 1; continue;
        }
        if (target.x == rhs.x) {
            if (target.y == 0) { out[i] = {}; mode[i] = 1; continue; }
            den[i] = e.f.mul(2, target.y);
            num[i] = e.f.add(e.f.mul(3, e.f.sqr(target.x)), e.a);
        } else {
            den[i] = e.f.sub(rhs.x, target.x);
            num[i] = e.f.sub(rhs.y, target.y);
        }
        prefix[i] = product;
        product = e.f.mul(product, den[i]);
    }
    u64 inv_product = e.f.inv(product);
    for (std::size_t i = count; i-- > 0;) {
        if (mode[i] != 0) continue;
        const Point rhs = e.neg(left[i]);
        const u64 inv_den = e.f.mul(inv_product, prefix[i]);
        inv_product = e.f.mul(inv_product, den[i]);
        const u64 lam = e.f.mul(num[i], inv_den);
        const u64 xr = e.f.sub(e.f.sub(e.f.sqr(lam), target.x), rhs.x);
        const u64 yr = e.f.sub(e.f.mul(lam, e.f.sub(target.x, xr)), target.y);
        out[i] = {xr, yr, false};
    }
    return out;
}

TargetSearchResult find_target_root(const Curve& e, const std::vector<Point>& points,
                                    const std::vector<Point>& target_points) {
    if (points.empty()) throw std::runtime_error("need at least one input point");
    if (target_points.empty()) return {};
    const std::size_t sign_bits = points.size() - 1;
    if (sign_bits >= 63) throw std::runtime_error("too many sign variables");
    const std::size_t left_bits = target_split_bits(sign_bits, target_points.size());
    std::vector<Point> left_vars(points.begin() + 1,
                                 points.begin() + 1 + static_cast<std::ptrdiff_t>(left_bits));
    std::vector<Point> right_vars(points.begin() + 1 + static_cast<std::ptrdiff_t>(left_bits), points.end());
    const auto left = enumerate_gray(e, points[0], left_vars);
    const auto right = enumerate_gray(e, Point{}, right_vars);

    std::unordered_map<PointKey, u64, PointKeyHash> right_index;
    right_index.reserve(right.size() * 2);
    for (u64 i = 0; i < right.size(); ++i) right_index.try_emplace(point_key(right[static_cast<std::size_t>(i)]), i);

    TargetSearchResult out;
    out.left_count = left.size();
    out.right_count = right.size();
    for (const Point& target : target_points) {
        for (unsigned target_sign = 0; target_sign < 2; ++target_sign) {
            const Point q = target_sign == 0 ? target : e.neg(target);
            const auto needed_points = batch_target_differences(e, q, left);
            for (u64 li = 0; li < needed_points.size(); ++li) {
                auto it = right_index.find(point_key(needed_points[static_cast<std::size_t>(li)]));
                if (it == right_index.end()) continue;
                const u64 left_gray = li ^ (li >> 1U);
                const u64 ri = it->second;
                const u64 right_gray = ri ^ (ri >> 1U);
                out.found = true;
                out.target_x = target.x;
                out.sign_mask = left_gray | (right_gray << left_bits);
                out.sum = one_signed_sum(e, points, out.sign_mask);
                return out;
            }
        }
    }
    return out;
}

std::vector<u64> sorted(std::vector<u64> x);

enum class AutomorphismKind { none, j0, j1728 };

struct Automorphism {
    AutomorphismKind kind = AutomorphismKind::none;
    u64 x_multiplier = 1;
    u64 y_multiplier = 1;
    unsigned point_order = 1;
    unsigned x_order = 1;

    bool available() const { return kind != AutomorphismKind::none; }
};

std::string automorphism_name(AutomorphismKind kind) {
    if (kind == AutomorphismKind::j0) return "j0_order3";
    if (kind == AutomorphismKind::j1728) return "j1728_order4";
    return "none";
}

Automorphism curve_automorphism(const Curve& e) {
    if (e.a == 0 && e.b != 0 && e.f.p % 3 == 1) {
        for (u64 z = 2; z < e.f.p; ++z) {
            const u64 beta = e.f.pow(z, (e.f.p - 1) / 3);
            if (beta != 1) {
                if (e.f.mul(e.f.sqr(beta), beta) != 1) throw std::runtime_error("cube-root construction failure");
                return {AutomorphismKind::j0, beta, 1, 3, 3};
            }
        }
    }
    if (e.b == 0 && e.a != 0 && e.f.p % 4 == 1) {
        auto i = e.f.sqrt(e.f.neg(1));
        if (!i || e.f.sqr(*i) != e.f.neg(1)) throw std::runtime_error("sqrt(-1) construction failure");
        return {AutomorphismKind::j1728, e.f.neg(1), *i, 4, 2};
    }
    return {};
}

Point apply_automorphism(const Curve& e, const Automorphism& aut, const Point& p) {
    if (p.inf || !aut.available()) return p;
    Point q{e.f.mul(aut.x_multiplier, p.x), e.f.mul(aut.y_multiplier, p.y), false};
    if (!e.on_curve(q)) throw std::runtime_error("automorphism did not preserve curve");
    return q;
}

u64 apply_automorphism_x(const Curve& e, const Automorphism& aut, u64 x) {
    return e.f.mul(aut.x_multiplier, x);
}

struct SignedPermutation {
    std::vector<std::size_t> destination;
    std::vector<int> sign;
};

std::optional<SignedPermutation> signed_input_action(const Curve& e, const Automorphism& aut,
                                                     const std::vector<Point>& points) {
    if (!aut.available()) return std::nullopt;
    SignedPermutation action;
    action.destination.resize(points.size());
    action.sign.resize(points.size());
    std::vector<bool> used(points.size(), false);
    for (std::size_t i = 0; i < points.size(); ++i) {
        const Point q = apply_automorphism(e, aut, points[i]);
        std::optional<std::size_t> match;
        int match_sign = 0;
        for (int wanted_sign : {1, -1}) {
            for (std::size_t j = 0; j < points.size(); ++j) {
                if (used[j] || q.x != points[j].x) continue;
                const u64 wanted_y = wanted_sign == 1 ? points[j].y : e.f.neg(points[j].y);
                if (q.y == wanted_y) { match = j; match_sign = wanted_sign; break; }
            }
            if (match) break;
        }
        if (!match) return std::nullopt;
        used[*match] = true;
        action.destination[i] = *match;
        action.sign[i] = match_sign;
    }
    return action;
}

u64 transform_sign_class(u64 mask, const SignedPermutation& action) {
    const std::size_t n = action.destination.size();
    if (n == 0 || n >= 63) throw std::runtime_error("invalid sign-class dimension");
    u64 transformed = 0;
    for (std::size_t i = 0; i < n; ++i) {
        const int input_sign = ((mask >> i) & 1U) != 0 ? 1 : -1;
        if (input_sign * action.sign[i] == 1) transformed |= 1ULL << action.destination[i];
    }
    const u64 all = (1ULL << n) - 1;
    if ((transformed & 1U) == 0) transformed ^= all;
    return transformed;
}

struct OrbitRootResult {
    RootResult roots;
    std::size_t sign_classes = 0;
    std::size_t sign_orbits = 0;
};

std::optional<OrbitRootResult> roots_automorphism_orbits(const Curve& e, const std::vector<Point>& points) {
    const Automorphism aut = curve_automorphism(e);
    auto action = signed_input_action(e, aut, points);
    if (!action || points.empty() || points.size() >= 63) return std::nullopt;
    const std::size_t classes = 1ULL << (points.size() - 1);
    std::vector<bool> visited(classes, false);
    OrbitRootResult out;
    out.sign_classes = classes;
    out.roots.finite.reserve(classes);
    for (u64 index = 0; index < classes; ++index) {
        if (visited[index]) continue;
        const u64 representative = (index << 1U) | 1U;
        std::vector<u64> orbit;
        u64 current = representative;
        do {
            const std::size_t current_index = static_cast<std::size_t>(current >> 1U);
            if (current_index >= classes) throw std::runtime_error("sign-class index failure");
            if (visited[current_index]) {
                if (current != representative) throw std::runtime_error("sign action entered another orbit");
                break;
            }
            visited[current_index] = true;
            orbit.push_back(current);
            current = transform_sign_class(current, *action);
            if (orbit.size() > aut.point_order) throw std::runtime_error("sign orbit exceeded automorphism order");
        } while (current != representative);
        ++out.sign_orbits;

        Point sum{};
        for (std::size_t i = 0; i < points.size(); ++i)
            sum = e.add(sum, ((representative >> i) & 1U) != 0 ? points[i] : e.neg(points[i]));
        if (sum.inf) {
            out.roots.infinity += orbit.size();
        } else {
            u64 x = sum.x;
            for (std::size_t k = 0; k < orbit.size(); ++k) {
                out.roots.finite.push_back(x);
                x = apply_automorphism_x(e, aut, x);
            }
        }
    }
    return out;
}

bool automorphism_covariance(const Curve& e, const Automorphism& aut, const std::vector<Point>& points) {
    RootResult original = roots_mitm_batch(e, points);
    std::vector<Point> transformed_points;
    transformed_points.reserve(points.size());
    for (const Point& p : points) transformed_points.push_back(apply_automorphism(e, aut, p));
    RootResult transformed = roots_mitm_batch(e, transformed_points);
    for (u64& x : original.finite) x = apply_automorphism_x(e, aut, x);
    return original.infinity == transformed.infinity && sorted(original.finite) == sorted(transformed.finite);
}

bool automorphism_coefficient_support(const Poly& polynomial, unsigned x_order) {
    const std::size_t deg = degree(polynomial);
    for (std::size_t k = 0; k < polynomial.size(); ++k)
        if (polynomial[k] != 0 && (deg - k) % x_order != 0) return false;
    return true;
}

std::vector<Point> invariant_automorphism_points(const Curve& e, const Automorphism& aut,
                                                 unsigned blocks, std::mt19937_64& rng) {
    std::vector<Point> points;
    std::set<u64> used_x;
    const unsigned orbit_x_points = aut.kind == AutomorphismKind::j0 ? 3U : 2U;
    for (unsigned block = 0; block < blocks; ++block) {
        for (;;) {
            Point p = e.random_point(rng);
            std::vector<Point> orbit;
            Point q = p;
            bool valid = true;
            for (unsigned i = 0; i < orbit_x_points; ++i) {
                if (q.inf || q.y == 0 || used_x.count(q.x) != 0 ||
                    std::any_of(orbit.begin(), orbit.end(), [&](const Point& z) { return z.x == q.x; })) {
                    valid = false; break;
                }
                orbit.push_back(q);
                q = apply_automorphism(e, aut, q);
            }
            if (!valid) continue;
            for (const Point& z : orbit) { points.push_back(z); used_x.insert(z.x); }
            break;
        }
    }
    return points;
}

Poly s3_direct(const Curve& e, u64 x1, u64 x2) {
    const Field& f = e.f;
    const u64 sum = f.add(x1, x2);
    const u64 prod = f.mul(x1, x2);
    const u64 diff = f.sub(x1, x2);
    const u64 c2 = f.sqr(diff);
    const u64 middle = f.add(f.mul(sum, f.add(prod, e.a)), f.mul(2, e.b));
    const u64 c1 = f.neg(f.mul(2, middle));
    const u64 c0 = f.sub(f.sqr(f.sub(prod, e.a)), f.mul(f.mul(4, e.b), sum));
    return {c0, c1, c2};
}

int permutation_sign(const std::array<int, 4>& p) {
    int inv = 0;
    for (int i = 0; i < 4; ++i) for (int j = i + 1; j < 4; ++j) if (p[i] > p[j]) ++inv;
    return (inv & 1) == 0 ? 1 : -1;
}

Poly s4_resultant(const Curve& e, u64 x1, u64 x2, u64 x3) {
    const Field& f = e.f;
    const Poly q1 = s3_direct(e, x1, x2);
    const Poly a2{f.sqr(x3), f.neg(f.mul(2, x3)), 1};
    const Poly b2{
        f.neg(f.mul(2, f.add(f.mul(e.a, x3), f.mul(2, e.b)))),
        f.neg(f.mul(2, f.add(f.sqr(x3), e.a))),
        f.neg(f.mul(2, x3))};
    const Poly c2{
        f.sub(f.sqr(e.a), f.mul(f.mul(4, e.b), x3)),
        f.neg(f.add(f.mul(f.mul(2, e.a), x3), f.mul(4, e.b))),
        f.sqr(x3)};
    const Poly zero{0};
    const std::array<std::array<Poly, 4>, 4> m{{
        {Poly{q1[2]}, Poly{q1[1]}, Poly{q1[0]}, zero},
        {zero, Poly{q1[2]}, Poly{q1[1]}, Poly{q1[0]}},
        {a2, b2, c2, zero},
        {zero, a2, b2, c2}
    }};
    Poly det{0};
    std::array<int, 4> perm{0, 1, 2, 3};
    do {
        Poly term{1};
        for (int i = 0; i < 4; ++i) term = poly_mul(term, m[i][perm[i]], f);
        det = permutation_sign(perm) > 0 ? poly_add(det, term, f) : poly_sub(det, term, f);
    } while (std::next_permutation(perm.begin(), perm.end()));
    trim(det);
    return det;
}

void cz_split(const Poly& fpoly, const Field& f, std::mt19937_64& rng, std::vector<u64>& roots) {
    const std::size_t n = degree(fpoly);
    if (n == 0) return;
    if (n == 1) {
        roots.push_back(f.mul(f.neg(fpoly[0]), f.inv(fpoly[1])));
        return;
    }
    std::uniform_int_distribution<u64> dist(0, f.p - 1);
    for (unsigned attempt = 0; attempt < 256; ++attempt) {
        Poly a(n, 0);
        for (u64& c : a) c = dist(rng);
        trim(a);
        Poly h = poly_pow_mod(a, (f.p - 1) / 2, fpoly, f);
        h[0] = f.sub(h[0], 1);
        trim(h);
        Poly d = poly_gcd(fpoly, h, f);
        if (!is_zero(d) && degree(d) > 0 && degree(d) < n) {
            Poly q = poly_divmod(fpoly, d, f).first;
            cz_split(poly_monic(d, f), f, rng, roots);
            cz_split(poly_monic(q, f), f, rng, roots);
            return;
        }
    }
    throw std::runtime_error("Cantor-Zassenhaus failed to split after 256 attempts");
}

std::vector<u64> roots_cantor_zassenhaus(const Poly& input, const Field& f, std::mt19937_64& rng) {
    Poly a = poly_monic(input, f);
    const Poly d = poly_gcd(a, poly_derivative(a, f), f);
    if (degree(d) > 0) a = poly_divmod(a, d, f).first;
    std::vector<u64> roots;
    cz_split(poly_monic(a, f), f, rng, roots);
    std::sort(roots.begin(), roots.end());
    roots.erase(std::unique(roots.begin(), roots.end()), roots.end());
    return roots;
}

std::vector<u64> sorted(std::vector<u64> x) { std::sort(x.begin(), x.end()); return x; }
std::vector<u64> unique_sorted(std::vector<u64> x) {
    std::sort(x.begin(), x.end()); x.erase(std::unique(x.begin(), x.end()), x.end()); return x;
}

void require(bool ok, const std::string& msg) { if (!ok) throw std::runtime_error("self-test: " + msg); }

void self_test() {
    const std::array<std::tuple<u64, u64, u64>, 9> fixtures{{
        {1009, 2, 3}, {1009, 0, 7}, {1009, 5, 11},
        {1000003, 2, 3}, {1000003, 0, 7}, {1000003, 5, 11},
        {2147483647, 2, 3}, {2147483647, 0, 7}, {2147483647, 5, 11}
    }};
    std::mt19937_64 rng(0x53454d414556ULL);
    std::size_t s3_checks = 0, s4_checks = 0, root_checks = 0, targeted_checks = 0;
    std::size_t disc_checks = 0, coefficient_checks = 0;
    std::size_t reducer_pairs = 0, reducer_inverses = 0;
    std::size_t one_root_checks = 0, target_hit_checks = 0, target_miss_checks = 0, target_edge_checks = 0;
    std::size_t automorphism_checks = 0;
    for (u64 p : {1009ULL, 1000003ULL, 2147483647ULL, 4294967291ULL, 2305843009213693951ULL}) {
        require(is_prime(p), "reducer-test modulus is not prime");
        const Field fast(p);
        const std::array<u64, 9> edges{{0, 1, 2, 3, p / 2, p / 2 + 1, p - 3, p - 2, p - 1}};
        for (u64 a : edges) for (u64 b : edges) {
            const u64 expected = static_cast<u64>((static_cast<u128>(a) * b) % p);
            require(fast.mul(a, b) == expected, "reducer edge pair");
            ++reducer_pairs;
        }
        if (p == 1009) {
            for (u64 a = 0; a < p; ++a) for (u64 b = 0; b < p; ++b) {
                const u64 expected = static_cast<u64>((static_cast<u128>(a) * b) % p);
                require(fast.mul(a, b) == expected, "specialized reducer exhaustive pair");
                ++reducer_pairs;
            }
        }
        std::uniform_int_distribution<u64> field_dist(0, p - 1);
        for (std::size_t i = 0; i < 250000; ++i) {
            const u64 a = field_dist(rng), b = field_dist(rng);
            const u64 expected = static_cast<u64>((static_cast<u128>(a) * b) % p);
            require(fast.mul(a, b) == expected, "reducer random pair");
            ++reducer_pairs;
            if (a != 0 && (i & 1023U) == 0) {
                require(fast.mul(a, fast.inv(a)) == 1, "specialized reducer inverse");
                ++reducer_inverses;
            }
        }
    }
    for (auto [p, a, b] : fixtures) {
        Curve e{Field(p), a, b};
        require(is_prime(p), "fixture modulus is not prime");
        require(e.nonsingular(), "singular curve fixture");
        Point edge_p;
        do { edge_p = e.random_point(rng); } while (e.dbl(edge_p).inf);
        const Point edge_q = e.random_point(rng);
        const Point edge_double = e.dbl(edge_p);
        const std::vector<Point> infinity_case{edge_p, edge_p};
        require(one_signed_sum(e, infinity_case, 0).inf, "explicit one-root infinity case");
        require(roots_mitm_batch(e, infinity_case).infinity == 1, "explicit infinity multiplicity");
        const auto infinity_target = find_target_root(e, infinity_case, {edge_double});
        require(infinity_target.found && infinity_target.sign_mask == 1 && infinity_target.sum == edge_double,
                "finite target beside infinity root");
        const std::vector<Point> multiplicity_case{edge_p, edge_q, edge_q};
        const auto multiplicity_roots = roots_mitm_batch(e, multiplicity_case);
        require(static_cast<std::size_t>(std::count(multiplicity_roots.finite.begin(), multiplicity_roots.finite.end(), edge_p.x)) >= 2,
                "explicit repeated-root case");
        const auto repeated_target = find_target_root(e, multiplicity_case, {edge_p});
        require(repeated_target.found && repeated_target.sum.x == edge_p.x, "target repeated-root case");
        ++target_edge_checks;
        std::uniform_int_distribution<u64> dist(0, p - 1);
        for (std::size_t count : {0U, 1U, 2U, 3U, 4U, 7U, 8U, 16U, 31U, 32U, 64U, 127U, 128U, 256U, 512U}) {
            std::vector<u64> roots(count);
            for (u64& root : roots) root = dist(rng);
            if (!roots.empty()) roots[0] = 0;
            if (roots.size() >= 2) roots[1] = roots[0];
            if (roots.size() >= 3) roots[2] = p - 1;
            require(poly_from_roots(roots, e.f) == poly_from_roots_product_tree(roots, e.f),
                    "quadratic and product-tree coefficients differ");
            require(poly_from_roots(roots, e.f) == poly_from_roots_selected(roots, e.f, "auto"),
                    "automatic coefficient method differs");
            ++coefficient_checks;
        }
        for (unsigned trial = 0; trial < 100; ++trial) {
            const u64 x1 = dist(rng), x2 = dist(rng);
            const Poly q = s3_direct(e, x1, x2);
            const u64 lhs = e.f.sub(e.f.sqr(q[1]), e.f.mul(4, e.f.mul(q[2], q[0])));
            const u64 rhs = e.f.mul(16, e.f.mul(e.rhs(x1), e.rhs(x2)));
            require(lhs == rhs, "S3 discriminant identity");
            ++disc_checks;

            std::vector<Point> pts{e.random_point(rng), e.random_point(rng), e.random_point(rng), e.random_point(rng)};
            auto rn = roots_naive(e, pts);
            auto rg = roots_gray(e, pts);
            auto rm = roots_mitm_batch(e, pts);
            auto rw = roots_mitm_workspace(e, pts);
            auto rt = roots_tree_batch(e, pts);
            auto rp = roots_tree_projective(e, pts);
            require(rn.infinity == rg.infinity && rg.infinity == rm.infinity && rm.infinity == rw.infinity &&
                    rw.infinity == rt.infinity && rt.infinity == rp.infinity,
                    "root infinity counts differ");
            require(sorted(rn.finite) == sorted(rg.finite), "naive and Gray roots differ");
            require(sorted(rn.finite) == sorted(rm.finite), "naive and MITM roots differ");
            require(sorted(rn.finite) == sorted(rw.finite), "naive and workspace MITM roots differ");
            require(sorted(rn.finite) == sorted(rt.finite), "naive and affine-tree roots differ");
            require(sorted(rn.finite) == sorted(rp.finite), "naive and projective-tree roots differ");
            std::vector<Point> forced_degenerate{pts[0], pts[0]};
            const auto rd_naive = roots_naive(e, forced_degenerate);
            const auto rd_workspace = roots_mitm_workspace(e, forced_degenerate);
            require(rd_naive.infinity == rd_workspace.infinity, "forced-degenerate infinity counts differ");
            require(sorted(rd_naive.finite) == sorted(rd_workspace.finite), "forced-degenerate roots differ");
            ++root_checks;

            const u64 chosen_mask = rng() & 7U;
            const Point one = one_signed_sum(e, pts, chosen_mask);
            if (one.inf) require(rm.infinity != 0, "one-root infinity absent from root multiset");
            else require(std::find(rm.finite.begin(), rm.finite.end(), one.x) != rm.finite.end(), "one-root x absent from root multiset");
            ++one_root_checks;

            if (!rm.finite.empty()) {
                const u64 target_x = rm.finite.front();
                const auto target_y = e.f.sqrt(e.rhs(target_x));
                require(target_y.has_value(), "finite target does not lift");
                const auto hit = find_target_root(e, pts, {{target_x, *target_y, false}});
                require(hit.found && !hit.sum.inf && hit.sum.x == target_x, "target hit not recovered");
                require(one_signed_sum(e, pts, hit.sign_mask) == hit.sum, "target sign-mask certificate invalid");
                ++target_hit_checks;
                const auto root_xs = unique_sorted(rm.finite);
                Point miss;
                do { miss = e.random_point(rng); }
                while (std::binary_search(root_xs.begin(), root_xs.end(), miss.x));
                require(!find_target_root(e, pts, {miss}).found, "target miss returned a root");
                ++target_miss_checks;
            }

            std::vector<Point> p3{pts[0], pts[1]};
            auto r3 = roots_gray(e, p3);
            if (r3.infinity == 0 && s3_direct(e, p3[0].x, p3[1].x).back() != 0) {
                require(poly_monic(s3_direct(e, p3[0].x, p3[1].x), e.f) == poly_from_roots(r3.finite, e.f), "S3 roots vs direct formula");
                ++s3_checks;
            }

            std::vector<Point> p4{pts[0], pts[1], pts[2]};
            auto r4 = roots_gray(e, p4);
            Poly res = s4_resultant(e, p4[0].x, p4[1].x, p4[2].x);
            if (!is_zero(res) && r4.infinity == 0 && degree(res) == r4.finite.size()) {
                require(poly_monic(res, e.f) == poly_from_roots(r4.finite, e.f), "S4 roots vs Sylvester resultant");
                ++s4_checks;
            }

            std::vector<Point> changed = pts;
            changed[1] = e.neg(changed[1]);
            require(sorted(roots_gray(e, changed).finite) == sorted(rg.finite), "orientation invariance");
            std::swap(changed[0], changed[2]);
            require(sorted(roots_gray(e, changed).finite) == sorted(rg.finite), "permutation invariance");

            Poly split = poly_from_roots(rg.finite, e.f);
            auto generic = roots_cantor_zassenhaus(split, e.f, rng);
            require(generic == unique_sorted(rg.finite), "generic factor roots differ");
        }
        const Point p1 = e.random_point(rng);
        const Point p2 = e.random_point(rng);
        std::vector<std::vector<Point>> targeted{{p1, p1}, {p1, e.neg(p1)}, {p1, p1, p2}};
        if (p <= 1009) {
            for (u64 x = 0; x < p; ++x) if (e.rhs(x) == 0) {
                const Point torsion{x, 0, false};
                targeted.push_back({torsion, torsion});
                targeted.push_back({torsion, torsion, p2});
                break;
            }
        }
        for (const auto& special : targeted) {
            const RootResult expected = roots_naive(e, special);
            const RootResult affine_tree = roots_tree_batch(e, special);
            const RootResult projective_tree = roots_tree_projective(e, special);
            require(expected.infinity == affine_tree.infinity && expected.infinity == projective_tree.infinity,
                    "targeted infinity counts differ");
            require(sorted(expected.finite) == sorted(affine_tree.finite), "targeted affine-tree roots differ");
            require(sorted(expected.finite) == sorted(projective_tree.finite), "targeted projective-tree roots differ");
            ++targeted_checks;
        }
    }
    for (u64 p : {1009ULL, 1000003ULL, 2147483647ULL}) {
        Curve e{Field(p), 0, 7};
        const Automorphism aut = curve_automorphism(e);
        require(aut.kind == AutomorphismKind::j0, "j=0 automorphism unavailable");
        for (unsigned trial = 0; trial < 20; ++trial) {
            auto invariant = invariant_automorphism_points(e, aut, trial % 2 + 1, rng);
            auto orbit_roots = roots_automorphism_orbits(e, invariant);
            auto baseline = roots_mitm_batch(e, invariant);
            require(orbit_roots.has_value(), "j=0 invariant action not detected");
            require(orbit_roots->roots.infinity == baseline.infinity, "j=0 infinity mismatch");
            require(sorted(orbit_roots->roots.finite) == sorted(baseline.finite), "j=0 orbit roots mismatch");
            require(automorphism_covariance(e, aut, invariant), "j=0 covariance failure");
            std::vector<Point> negative{e.random_point(rng), e.random_point(rng), e.random_point(rng)};
            require(automorphism_covariance(e, aut, negative), "j=0 negative-control covariance failure");
            ++automorphism_checks;
        }
    }
    {
        Curve e{Field(1009), 5, 0};
        const Automorphism aut = curve_automorphism(e);
        require(aut.kind == AutomorphismKind::j1728, "j=1728 automorphism unavailable");
        for (unsigned trial = 0; trial < 20; ++trial) {
            auto invariant = invariant_automorphism_points(e, aut, trial % 3 + 1, rng);
            auto orbit_roots = roots_automorphism_orbits(e, invariant);
            auto baseline = roots_mitm_batch(e, invariant);
            require(orbit_roots.has_value(), "j=1728 invariant action not detected");
            require(orbit_roots->roots.infinity == baseline.infinity, "j=1728 infinity mismatch");
            require(sorted(orbit_roots->roots.finite) == sorted(baseline.finite), "j=1728 orbit roots mismatch");
            require(automorphism_covariance(e, aut, invariant), "j=1728 covariance failure");
            ++automorphism_checks;
        }
    }
    {
        Curve e{Field(43), 0, 7};
        const Automorphism aut = curve_automorphism(e);
        Point p{2, 12, false};
        std::vector<Point> positive{p, apply_automorphism(e, aut, p), apply_automorphism(e, aut, apply_automorphism(e, aut, p))};
        auto positive_orbits = roots_automorphism_orbits(e, positive);
        require(positive_orbits.has_value(), "p=43 positive automorphism fixture not invariant");
        require(automorphism_coefficient_support(poly_from_roots(positive_orbits->roots.finite, e.f), aut.x_order),
                "p=43 invariant coefficient support failure");
        std::vector<Point> negative{positive[0], positive[1]};
        require(!signed_input_action(e, aut, negative), "p=43 negative fixture unexpectedly invariant");
        require(automorphism_covariance(e, aut, negative), "p=43 negative fixture covariance failure");
        require(curve_automorphism(Curve{Field(1019), 0, 7}).kind == AutomorphismKind::none,
                "j=0 automorphism should not be rational for p=2 mod 3");
        require(curve_automorphism(Curve{Field(1000003), 5, 0}).kind == AutomorphismKind::none,
                "j=1728 automorphism should not be rational for p=3 mod 4");
        automorphism_checks += 4;
    }
    std::cout << "PASS self-test"
              << " s3=" << s3_checks
              << " s4=" << s4_checks
              << " root_methods=" << root_checks
              << " one_root=" << one_root_checks
              << " target_hits=" << target_hit_checks
              << " target_misses=" << target_miss_checks
              << " target_edges=" << target_edge_checks
              << " targeted_degeneracies=" << targeted_checks
              << " discriminants=" << disc_checks
              << " coefficient_methods=" << coefficient_checks
              << " reducer_pairs=" << reducer_pairs
              << " reducer_inverses=" << reducer_inverses
              << " automorphisms=" << automorphism_checks << '\n';
}

struct Options {
    std::string command;
    u64 prime = 1000003;
    u64 a = 2;
    u64 b = 3;
    u64 seed = 0x53454d4145562026ULL;
    unsigned min_arity = 3;
    unsigned max_arity = 8;
    std::size_t samples = 1000;
    std::size_t bench_cases = 40;
    unsigned bench_repeats = 2;
    std::filesystem::path output;
    std::vector<u64> xs;
    std::vector<u64> target_xs;
    std::string method = "auto";
    std::optional<u64> sign_mask;
    std::string coefficient_method = "auto";
    bool emit_coefficients = false;
    std::size_t min_degree = 2;
    std::size_t max_degree = 4096;
    Field::Reduction reduction = Field::Reduction::Auto;
};

u64 parse_u64(const std::string& s) {
    std::size_t pos = 0;
    const u64 v = std::stoull(s, &pos, 0);
    if (pos != s.size()) throw std::runtime_error("invalid integer: " + s);
    return v;
}

Options parse_options(int argc, char** argv) {
    if (argc < 2) throw std::runtime_error("usage: semaev_lab self-test|generate|benchmark|coeff-benchmark|roots|one-root|target-root|benchmark-target|automorphism-audit|automorphism-benchmark [options]");
    Options o;
    o.command = argv[1];
    for (int i = 2; i < argc; ++i) {
        const std::string k = argv[i];
        auto value = [&]() -> std::string {
            if (++i >= argc) throw std::runtime_error("missing value for " + k);
            return argv[i];
        };
        if (k == "--prime") o.prime = parse_u64(value());
        else if (k == "--a") o.a = parse_u64(value());
        else if (k == "--b") o.b = parse_u64(value());
        else if (k == "--seed") o.seed = parse_u64(value());
        else if (k == "--min-arity") o.min_arity = static_cast<unsigned>(parse_u64(value()));
        else if (k == "--max-arity") o.max_arity = static_cast<unsigned>(parse_u64(value()));
        else if (k == "--samples") o.samples = static_cast<std::size_t>(parse_u64(value()));
        else if (k == "--bench-cases") o.bench_cases = static_cast<std::size_t>(parse_u64(value()));
        else if (k == "--bench-repeats") o.bench_repeats = static_cast<unsigned>(parse_u64(value()));
        else if (k == "--output") o.output = value();
        else if (k == "--x") o.xs.push_back(parse_u64(value()));
        else if (k == "--target-x") o.target_xs.push_back(parse_u64(value()));
        else if (k == "--sign-mask") o.sign_mask = parse_u64(value());
        else if (k == "--method") o.method = value();
        else if (k == "--coefficient-method") o.coefficient_method = value();
        else if (k == "--min-degree") o.min_degree = static_cast<std::size_t>(parse_u64(value()));
        else if (k == "--max-degree") o.max_degree = static_cast<std::size_t>(parse_u64(value()));
        else if (k == "--emit-coefficients") o.emit_coefficients = true;
        else if (k == "--reduction") {
            const std::string mode = value();
            if (mode == "auto") o.reduction = Field::Reduction::Auto;
            else if (mode == "generic") o.reduction = Field::Reduction::Generic;
            else throw std::runtime_error("--reduction must be auto or generic");
        }
        else throw std::runtime_error("unknown option: " + k);
    }
    if (!is_prime(o.prime) || o.prime <= 3 || o.prime >= (1ULL << 63)) throw std::runtime_error("prime must be an odd prime in (3,2^63)");
    o.a %= o.prime; o.b %= o.prime;
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    if (!e.nonsingular()) throw std::runtime_error("curve is singular");
    if (o.min_arity < 3 || o.max_arity < o.min_arity || o.max_arity > 20) throw std::runtime_error("arity range must satisfy 3 <= min <= max <= 20");
    if ((o.command == "generate" || o.command == "benchmark" || o.command == "benchmark-target" || o.command == "automorphism-benchmark") && o.output.empty())
        throw std::runtime_error("--output is required");
    if (o.command == "coeff-benchmark" && o.output.empty()) throw std::runtime_error("--output is required");
    if (o.coefficient_method != "auto" && o.coefficient_method != "quadratic" && o.coefficient_method != "product")
        throw std::runtime_error("--coefficient-method must be auto, quadratic, or product");
    if (o.min_degree == 0 || o.max_degree < o.min_degree)
        throw std::runtime_error("degree range must satisfy 1 <= min <= max");
    if (o.bench_cases == 0 || o.bench_repeats == 0)
        throw std::runtime_error("benchmark cases and repeats must be positive");
    if (o.command == "roots" || o.command == "one-root" || o.command == "target-root" || o.command == "automorphism-audit") {
        if (o.xs.size() < 2 || o.xs.size() > 19) throw std::runtime_error("root commands require 2 to 19 repeated --x values");
        for (u64& x : o.xs) x %= o.prime;
    }
    if (o.command == "roots") {
        if (o.method != "auto" && o.method != "naive" && o.method != "gray" && o.method != "mitm" && o.method != "workspace" && o.method != "orbit" &&
            o.method != "tree" && o.method != "projective")
            throw std::runtime_error("--method must be auto, naive, gray, mitm, workspace, tree, projective, or orbit");
    }
    if (o.command == "one-root" && o.sign_mask.has_value()) {
        const std::size_t bits = o.xs.size() - 1;
        if (bits < 64 && (*o.sign_mask >> bits) != 0) throw std::runtime_error("--sign-mask has bits outside the supplied inputs");
    }
    if (o.command == "target-root") {
        if (o.target_xs.empty()) throw std::runtime_error("target-root requires at least one --target-x");
        for (u64& x : o.target_xs) x %= o.prime;
    }
    return o;
}

struct Stats {
    std::size_t samples = 0;
    std::size_t nominal_roots = 0;
    std::size_t finite_roots = 0;
    std::size_t infinity_roots = 0;
    std::size_t distinct_roots = 0;
    std::size_t repeated_samples = 0;
    std::size_t degree_drop_samples = 0;
    std::size_t zero_coefficients = 0;
    std::size_t coefficients = 0;
    std::size_t even_only_samples = 0;
    std::size_t palindromic_samples = 0;
};

void write_array(std::ostream& os, const std::vector<u64>& a) {
    os << '[';
    for (std::size_t i = 0; i < a.size(); ++i) { if (i != 0) os << ','; os << a[i]; }
    os << ']';
}

void generate(const Options& o) {
    std::filesystem::create_directories(o.output.parent_path());
    std::ofstream corpus(o.output);
    if (!corpus) throw std::runtime_error("cannot open output: " + o.output.string());
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    std::mt19937_64 rng(o.seed);
    std::map<unsigned, Stats> stats;
    const auto started = std::chrono::steady_clock::now();
    for (unsigned arity = o.min_arity; arity <= o.max_arity; ++arity) {
        Stats& s = stats[arity];
        const std::size_t point_count = arity - 1;
        const std::size_t nominal = 1ULL << (arity - 2);
        for (std::size_t sample = 0; sample < o.samples; ++sample) {
            std::vector<Point> points;
            points.reserve(point_count);
            for (std::size_t i = 0; i < point_count; ++i) points.push_back(e.random_point(rng));
            RootResult roots = roots_mitm_batch(e, points);
            Poly poly = poly_from_roots_selected(roots.finite, e.f, o.coefficient_method);
            std::vector<u64> xs;
            xs.reserve(points.size());
            for (const Point& p : points) xs.push_back(p.x);
            const std::size_t distinct = unique_sorted(roots.finite).size();
            bool even_only = true;
            for (std::size_t i = 1; i < poly.size(); i += 2) if (poly[i] != 0) { even_only = false; break; }
            bool pal = true;
            for (std::size_t i = 0; i < poly.size(); ++i) if (poly[i] != poly[poly.size() - 1 - i]) { pal = false; break; }

            ++s.samples;
            s.nominal_roots += nominal;
            s.finite_roots += roots.finite.size();
            s.infinity_roots += roots.infinity;
            s.distinct_roots += distinct;
            if (distinct < roots.finite.size()) ++s.repeated_samples;
            if (roots.infinity != 0) ++s.degree_drop_samples;
            s.coefficients += poly.size();
            s.zero_coefficients += static_cast<std::size_t>(std::count(poly.begin(), poly.end(), 0));
            if (even_only) ++s.even_only_samples;
            if (pal) ++s.palindromic_samples;

            corpus << "{\"p\":" << o.prime << ",\"a\":" << o.a << ",\"b\":" << o.b
                   << ",\"seed\":" << o.seed << ",\"arity\":" << arity << ",\"sample\":" << sample
                   << ",\"nominal_degree\":" << nominal << ",\"infinity_roots\":" << roots.infinity
                   << ",\"distinct_roots\":" << distinct << ",\"x\":";
            write_array(corpus, xs);
            corpus << ",\"coeff_low_to_high\":";
            write_array(corpus, poly);
            corpus << "}\n";
        }
    }
    corpus.close();
    const double seconds = std::chrono::duration<double>(std::chrono::steady_clock::now() - started).count();
    const auto summary_path = o.output.string() + ".summary.csv";
    std::ofstream summary(summary_path);
    summary << "p,a,b,seed,arity,samples,nominal_degree,generated_coefficients,finite_roots,infinity_roots,distinct_roots,repeated_samples,degree_drop_samples,zero_coefficients,total_coefficients,even_only_samples,palindromic_samples\n";
    for (const auto& [arity, s] : stats) {
        summary << o.prime << ',' << o.a << ',' << o.b << ',' << o.seed << ',' << arity << ',' << s.samples << ','
                << (1ULL << (arity - 2)) << ',' << s.coefficients << ',' << s.finite_roots << ',' << s.infinity_roots << ','
                << s.distinct_roots << ',' << s.repeated_samples << ',' << s.degree_drop_samples << ',' << s.zero_coefficients << ','
                << s.coefficients << ',' << s.even_only_samples << ',' << s.palindromic_samples << '\n';
    }
    std::cout << std::fixed << std::setprecision(6)
              << "generated=" << (o.samples * (o.max_arity - o.min_arity + 1ULL))
              << " seconds=" << seconds << " coefficient_method=" << o.coefficient_method
              << " corpus=" << o.output << " summary=" << summary_path << '\n';
}

template<class Fn>
double time_per_case_ns(Fn&& fn, std::size_t cases, unsigned repeats, volatile u64& sink) {
    const auto t0 = std::chrono::steady_clock::now();
    for (unsigned r = 0; r < repeats; ++r) sink ^= fn();
    const auto t1 = std::chrono::steady_clock::now();
    return std::chrono::duration<double, std::nano>(t1 - t0).count() / static_cast<double>(cases * repeats);
}

bool all_roots_contain_target(const Curve& e, const std::vector<Point>& points,
                              const std::vector<Point>& targets) {
    const auto roots = unique_sorted(roots_mitm_batch(e, points).finite);
    for (const Point& target : targets)
        if (std::binary_search(roots.begin(), roots.end(), target.x)) return true;
    return false;
}

struct TargetBenchCase {
    std::vector<Point> points;
    std::vector<Point> targets;
};

TargetBenchCase make_target_bench_case(const Curve& e, std::size_t point_count,
                                       std::size_t target_count, bool should_hit,
                                       std::mt19937_64& rng) {
    for (;;) {
        TargetBenchCase out;
        out.points.reserve(point_count);
        for (std::size_t i = 0; i < point_count; ++i) out.points.push_back(e.random_point(rng));
        const auto roots = unique_sorted(roots_mitm_batch(e, out.points).finite);
        if (should_hit && roots.empty()) continue;
        std::unordered_set<u64> root_set(roots.begin(), roots.end());
        std::unordered_set<u64> used;
        if (should_hit) {
            const u64 x = roots[static_cast<std::size_t>(rng() % roots.size())];
            const auto y = e.f.sqrt(e.rhs(x));
            if (!y) throw std::runtime_error("benchmark target root failed to lift");
            out.targets.push_back({x, *y, false});
            used.insert(x);
        }
        std::size_t attempts = 0;
        const std::size_t max_attempts = std::max<std::size_t>(target_count * 4096, 65536);
        while (out.targets.size() < target_count && attempts++ < max_attempts) {
            Point candidate = e.random_point(rng);
            if (root_set.contains(candidate.x) || !used.insert(candidate.x).second) continue;
            auto y = e.f.sqrt(e.rhs(candidate.x));
            out.targets.push_back({candidate.x, *y, false});
        }
        if (out.targets.size() != target_count) throw std::runtime_error("unable to construct disjoint target set");
        std::shuffle(out.targets.begin(), out.targets.end(), rng);
        return out;
    }
}

void benchmark_target(const Options& o) {
    std::filesystem::create_directories(o.output.parent_path());
    std::ofstream out(o.output);
    if (!out) throw std::runtime_error("cannot open output: " + o.output.string());
    out << "p,a,b,seed,arity,degree,target_count,hit,cases,validated_cases,repeats,left_count,right_count,one_root_ns,all_roots_ns,target_search_ns,all_roots_vs_one,all_roots_vs_target\n";
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    std::mt19937_64 rng(o.seed ^ 0x544152474554ULL);
    volatile u64 sink = 0;
    constexpr std::array<std::size_t, 4> target_counts{{1, 4, 16, 64}};
    for (unsigned arity = o.min_arity; arity <= o.max_arity; ++arity) {
        const std::size_t point_count = arity - 1;
        const std::size_t degree_value = 1ULL << (arity - 2);
        for (std::size_t target_count : target_counts) {
            for (bool hit : {false, true}) {
                std::vector<TargetBenchCase> cases;
                cases.reserve(o.bench_cases);
                for (std::size_t i = 0; i < o.bench_cases; ++i)
                    cases.push_back(make_target_bench_case(e, point_count, target_count, hit, rng));

                for (const auto& c : cases) {
                    const bool expected = all_roots_contain_target(e, c.points, c.targets);
                    const auto observed = find_target_root(e, c.points, c.targets);
                    if (expected != observed.found || expected != hit)
                        throw std::runtime_error("target benchmark hit/miss mismatch");
                    if (observed.found) {
                        const Point certified = one_signed_sum(e, c.points, observed.sign_mask);
                        if (certified.inf || certified != observed.sum || certified.x != observed.target_x)
                            throw std::runtime_error("target benchmark sign-mask certificate mismatch");
                    }
                    const u64 plus_mask = (1ULL << (c.points.size() - 1)) - 1;
                    const Point one = one_signed_sum(e, c.points, plus_mask);
                    const RootResult all = roots_mitm_batch(e, c.points);
                    if (one.inf ? all.infinity == 0 : std::find(all.finite.begin(), all.finite.end(), one.x) == all.finite.end())
                        throw std::runtime_error("one-root benchmark certificate mismatch");
                }

                auto run_one = [&]() {
                    u64 value = 0;
                    for (const auto& c : cases) {
                        const u64 mask = (1ULL << (c.points.size() - 1)) - 1;
                        const Point z = one_signed_sum(e, c.points, mask);
                        value ^= z.inf ? 0 : z.x;
                    }
                    return value;
                };
                auto run_all = [&]() {
                    u64 value = 0;
                    for (const auto& c : cases) value ^= all_roots_contain_target(e, c.points, c.targets) ? 1 : 0;
                    return value;
                };
                auto run_target = [&]() {
                    u64 value = 0;
                    for (const auto& c : cases) {
                        const auto z = find_target_root(e, c.points, c.targets);
                        value ^= z.found ? z.target_x : 0;
                    }
                    return value;
                };
                const double one_ns = time_per_case_ns(run_one, cases.size(), o.bench_repeats, sink);
                const double all_ns = time_per_case_ns(run_all, cases.size(), o.bench_repeats, sink);
                const double target_ns = time_per_case_ns(run_target, cases.size(), o.bench_repeats, sink);
                const auto shape = find_target_root(e, cases.front().points, cases.front().targets);
                out << o.prime << ',' << o.a << ',' << o.b << ',' << o.seed << ',' << arity << ',' << degree_value << ','
                    << target_count << ',' << (hit ? 1 : 0) << ',' << cases.size() << ',' << cases.size() << ','
                    << o.bench_repeats << ',' << shape.left_count << ',' << shape.right_count << ','
                    << std::fixed << std::setprecision(3) << one_ns << ',' << all_ns << ',' << target_ns << ','
                    << all_ns / one_ns << ',' << all_ns / target_ns << '\n';
                std::cout << "target bench arity=" << arity << " degree=" << degree_value
                          << " targets=" << target_count << " hit=" << hit
                          << " all_ns=" << all_ns << " target_ns=" << target_ns << '\n';
            }
        }
    }
    std::cerr << "target benchmark checksum=" << sink << '\n';
}

void benchmark(const Options& o) {
    std::filesystem::create_directories(o.output.parent_path());
    std::ofstream out(o.output);
    if (!out) throw std::runtime_error("cannot open output: " + o.output.string());
    out << "reduction,p,a,b,seed,arity,degree,cases,validated_cases,common_path_validated,structure_repeats,generic_ns,naive_ns,gray_ns,mitm_batch_ns,workspace_generate_ns,workspace_distinct_ns,sort_dedup_ns,tree_batch_ns,projective_tree_ns,generic_vs_tree,mitm_vs_tree,workspace_vs_tree,projective_vs_tree\n";
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    std::mt19937_64 rng(o.seed);
    volatile u64 sink = 0;
    for (unsigned arity = o.min_arity; arity <= o.max_arity; ++arity) {
        std::vector<std::vector<Point>> cases;
        std::vector<Poly> polys;
        cases.reserve(o.bench_cases); polys.reserve(o.bench_cases);
        for (std::size_t c = 0; c < o.bench_cases; ++c) {
            std::vector<Point> points;
            for (unsigned i = 0; i < arity - 1; ++i) points.push_back(e.random_point(rng));
            RootResult roots = roots_mitm_batch(e, points);
            cases.push_back(points);
            polys.push_back(poly_from_roots_selected(roots.finite, e.f, o.coefficient_method));
        }
        std::mt19937_64 validation_rng(o.seed ^ (static_cast<u64>(arity) << 40));
        MitmWorkspace validation_ws;
        RootResult validation_out;
        for (std::size_t i = 0; i < cases.size(); ++i) {
            const auto expected = unique_sorted(roots_mitm_batch(e, cases[i]).finite);
            roots_mitm_workspace_into(e, cases[i], validation_ws, validation_out);
            if (expected != unique_sorted(validation_out.finite))
                throw std::runtime_error("benchmark workspace root mismatch");
            if (expected != unique_sorted(roots_tree_batch(e, cases[i]).finite))
                throw std::runtime_error("benchmark affine-tree root mismatch");
            if (expected != unique_sorted(roots_tree_projective(e, cases[i]).finite))
                throw std::runtime_error("benchmark projective-tree root mismatch");
            const auto observed = roots_cantor_zassenhaus(polys[i], e.f, validation_rng);
            if (expected != observed) throw std::runtime_error("benchmark validation root mismatch");
        }
        const std::size_t common_path_validated = validation_ws.common_path_calls;
        auto run_naive = [&]() {
            u64 v = 0; for (const auto& c : cases) { auto z = unique_sorted(roots_naive(e, c).finite); v ^= z.empty() ? 0 : z[0]; } return v;
        };
        auto run_gray = [&]() {
            u64 v = 0; for (const auto& c : cases) { auto z = unique_sorted(roots_gray(e, c).finite); v ^= z.empty() ? 0 : z[0]; } return v;
        };
        auto run_mitm = [&]() {
            u64 v = 0; for (const auto& c : cases) { auto z = unique_sorted(roots_mitm_batch(e, c).finite); v ^= z.empty() ? 0 : z[0]; } return v;
        };
        MitmWorkspace generation_ws;
        RootResult generation_out;
        auto run_workspace_generate = [&]() {
            u64 v = 0;
            for (const auto& c : cases) {
                roots_mitm_workspace_into(e, c, generation_ws, generation_out);
                v ^= generation_out.finite.empty() ? generation_out.infinity : generation_out.finite[0];
            }
            return v;
        };
        MitmWorkspace distinct_ws;
        RootResult distinct_out;
        auto run_workspace_distinct = [&]() {
            u64 v = 0;
            for (const auto& c : cases) {
                roots_mitm_workspace_into(e, c, distinct_ws, distinct_out);
                std::sort(distinct_out.finite.begin(), distinct_out.finite.end());
                distinct_out.finite.erase(std::unique(distinct_out.finite.begin(), distinct_out.finite.end()), distinct_out.finite.end());
                v ^= distinct_out.finite.empty() ? 0 : distinct_out.finite[0];
            }
            return v;
        };
        std::vector<std::vector<u64>> cached_roots;
        cached_roots.reserve(cases.size());
        for (const auto& c : cases) cached_roots.push_back(roots_mitm_batch(e, c).finite);
        std::vector<u64> sort_buffer;
        auto run_sort_dedup = [&]() {
            u64 v = 0;
            for (const auto& roots : cached_roots) {
                sort_buffer.assign(roots.begin(), roots.end());
                std::sort(sort_buffer.begin(), sort_buffer.end());
                sort_buffer.erase(std::unique(sort_buffer.begin(), sort_buffer.end()), sort_buffer.end());
                v ^= sort_buffer.empty() ? 0 : sort_buffer[0];
            }
            return v;
        };
        auto run_tree = [&]() {
            u64 v = 0; for (const auto& c : cases) { auto z = unique_sorted(roots_tree_batch(e, c).finite); v ^= z.empty() ? 0 : z[0]; } return v;
        };
        auto run_projective = [&]() {
            u64 v = 0; for (const auto& c : cases) { auto z = unique_sorted(roots_tree_projective(e, c).finite); v ^= z.empty() ? 0 : z[0]; } return v;
        };
        auto run_generic = [&]() {
            u64 v = 0;
            std::mt19937_64 local(o.seed ^ (static_cast<u64>(arity) << 32));
            for (const auto& p : polys) { auto z = roots_cantor_zassenhaus(p, e.f, local); v ^= z.empty() ? 0 : z[0]; }
            return v;
        };
        const double naive_ns = time_per_case_ns(run_naive, cases.size(), o.bench_repeats, sink);
        const double gray_ns = time_per_case_ns(run_gray, cases.size(), o.bench_repeats, sink);
        const double mitm_ns = time_per_case_ns(run_mitm, cases.size(), o.bench_repeats, sink);
        const double workspace_generate_ns = time_per_case_ns(run_workspace_generate, cases.size(), o.bench_repeats, sink);
        const double workspace_distinct_ns = time_per_case_ns(run_workspace_distinct, cases.size(), o.bench_repeats, sink);
        const double sort_dedup_ns = time_per_case_ns(run_sort_dedup, cases.size(), o.bench_repeats, sink);
        const double tree_ns = time_per_case_ns(run_tree, cases.size(), o.bench_repeats, sink);
        const double projective_ns = time_per_case_ns(run_projective, cases.size(), o.bench_repeats, sink);
        const double generic_ns = time_per_case_ns(run_generic, cases.size(), 1, sink);
        const std::size_t deg = 1ULL << (arity - 2);
        out << (o.reduction == Field::Reduction::Auto ? "auto" : "generic") << ','
            << o.prime << ',' << o.a << ',' << o.b << ',' << o.seed << ',' << arity << ',' << deg << ',' << cases.size() << ',' << cases.size() << ',' << common_path_validated << ',' << o.bench_repeats << ','
            << std::fixed << std::setprecision(3) << generic_ns << ',' << naive_ns << ',' << gray_ns << ',' << mitm_ns << ','
            << workspace_generate_ns << ',' << workspace_distinct_ns << ',' << sort_dedup_ns << ','
            << tree_ns << ',' << projective_ns << ',' << generic_ns / tree_ns << ',' << mitm_ns / tree_ns << ',' << workspace_distinct_ns / tree_ns << ','
            << projective_ns / tree_ns << '\n';
        std::cout << "bench arity=" << arity << " degree=" << deg << " generic_ns=" << generic_ns
                  << " mitm_ns=" << mitm_ns << " tree_ns=" << tree_ns << " projective_ns=" << projective_ns << '\n';
    }
    std::cerr << "benchmark checksum=" << sink << '\n';
}

void coefficient_benchmark(const Options& o) {
    std::filesystem::create_directories(o.output.parent_path());
    std::ofstream out(o.output);
    if (!out) throw std::runtime_error("cannot open output: " + o.output.string());
    out << "p,a,b,seed,degree,cases,validated_cases,repeats,quadratic_ns,product_tree_ns,quadratic_over_product,auto_method\n";
    const Field f{o.prime, o.reduction};
    std::mt19937_64 rng(o.seed);
    std::uniform_int_distribution<u64> dist(0, f.p - 1);
    volatile u64 sink = 0;
    for (std::size_t count = o.min_degree; count <= o.max_degree;) {
        std::vector<std::vector<u64>> cases(o.bench_cases, std::vector<u64>(count));
        for (auto& roots : cases) {
            for (u64& root : roots) root = dist(rng);
            if (roots.size() >= 2) roots[1] = roots[0]; // retain a multiplicity control
            if (roots.size() >= 3) roots[2] = 0;
        }
        for (const auto& roots : cases) {
            if (poly_from_roots(roots, f) != poly_from_roots_product_tree(roots, f))
                throw std::runtime_error("coefficient benchmark validation mismatch");
        }
        auto run_quadratic = [&]() {
            u64 value = 0;
            for (const auto& roots : cases) {
                const Poly poly = poly_from_roots(roots, f);
                value ^= poly.front() ^ poly[poly.size() / 2] ^ poly.back();
            }
            return value;
        };
        auto run_product = [&]() {
            u64 value = 0;
            for (const auto& roots : cases) {
                const Poly poly = poly_from_roots_product_tree(roots, f);
                value ^= poly.front() ^ poly[poly.size() / 2] ^ poly.back();
            }
            return value;
        };
        const double quadratic_ns = time_per_case_ns(run_quadratic, cases.size(), o.bench_repeats, sink);
        const double product_ns = time_per_case_ns(run_product, cases.size(), o.bench_repeats, sink);
        out << o.prime << ',' << o.a << ',' << o.b << ',' << o.seed << ',' << count << ','
            << cases.size() << ',' << cases.size() << ',' << o.bench_repeats << ','
            << std::fixed << std::setprecision(3) << quadratic_ns << ',' << product_ns << ','
            << quadratic_ns / product_ns << ','
            << (count < PRODUCT_TREE_AUTO_THRESHOLD ? "quadratic" : "product") << '\n';
        std::cout << "coeff degree=" << count << " quadratic_ns=" << quadratic_ns
                  << " product_ns=" << product_ns << '\n';
        if (count > o.max_degree / 2) break;
        count *= 2;
    }
    std::cerr << "coefficient benchmark checksum=" << sink << '\n';
}

void automorphism_benchmark(const Options& o) {
    std::filesystem::create_directories(o.output.parent_path());
    std::ofstream out(o.output);
    if (!out) throw std::runtime_error("cannot open output: " + o.output.string());
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    const Automorphism aut = curve_automorphism(e);
    if (!aut.available()) throw std::runtime_error("curve has no supported rational j=0 or j=1728 automorphism");
    out << "p,a,b,seed,kind,point_count,arity,degree,cases,validated_cases,sign_orbits,class_reduction,"
           "negative_invariant_cases,repeats,mitm_distinct_ns,orbit_distinct_ns,mitm_over_orbit\n";
    std::mt19937_64 rng(o.seed);
    volatile u64 sink = 0;
    const unsigned points_per_block = aut.kind == AutomorphismKind::j0 ? 3U : 2U;
    const unsigned max_points = std::min<unsigned>(o.max_arity - 1, 11);
    for (unsigned blocks = 1; blocks * points_per_block <= max_points; ++blocks) {
        const unsigned point_count = blocks * points_per_block;
        std::vector<std::vector<Point>> cases;
        cases.reserve(o.bench_cases);
        std::size_t negative_invariant = 0;
        for (std::size_t c = 0; c < o.bench_cases; ++c) {
            cases.push_back(invariant_automorphism_points(e, aut, blocks, rng));
            std::vector<Point> negative;
            for (unsigned i = 0; i < point_count; ++i) negative.push_back(e.random_point(rng));
            if (signed_input_action(e, aut, negative)) ++negative_invariant;
        }
        std::size_t sign_orbits = 0;
        for (const auto& points : cases) {
            const auto orbit = roots_automorphism_orbits(e, points);
            if (!orbit) throw std::runtime_error("constructed invariant input was not detected");
            const RootResult baseline = roots_mitm_batch(e, points);
            if (orbit->roots.infinity != baseline.infinity || sorted(orbit->roots.finite) != sorted(baseline.finite))
                throw std::runtime_error("automorphism benchmark validation mismatch");
            if (!automorphism_covariance(e, aut, points)) throw std::runtime_error("automorphism benchmark covariance failure");
            if (sign_orbits == 0) sign_orbits = orbit->sign_orbits;
            else if (sign_orbits != orbit->sign_orbits) throw std::runtime_error("sign-orbit count varied unexpectedly");
        }
        auto run_mitm = [&]() {
            u64 value = 0;
            for (const auto& points : cases) {
                auto roots = unique_sorted(roots_mitm_batch(e, points).finite);
                value ^= roots.empty() ? 0 : roots.front();
            }
            return value;
        };
        auto run_orbit = [&]() {
            u64 value = 0;
            for (const auto& points : cases) {
                auto result = roots_automorphism_orbits(e, points);
                auto roots = unique_sorted(result->roots.finite);
                value ^= roots.empty() ? 0 : roots.front();
            }
            return value;
        };
        const double mitm_ns = time_per_case_ns(run_mitm, cases.size(), o.bench_repeats, sink);
        const double orbit_ns = time_per_case_ns(run_orbit, cases.size(), o.bench_repeats, sink);
        const std::size_t degree_value = 1ULL << (point_count - 1);
        out << o.prime << ',' << o.a << ',' << o.b << ',' << o.seed << ',' << automorphism_name(aut.kind) << ','
            << point_count << ',' << (point_count + 1) << ',' << degree_value << ',' << cases.size() << ',' << cases.size() << ','
            << sign_orbits << ',' << std::fixed << std::setprecision(6)
            << static_cast<double>(degree_value) / static_cast<double>(sign_orbits) << ','
            << negative_invariant << ',' << o.bench_repeats << ',' << std::setprecision(3) << mitm_ns << ',' << orbit_ns << ','
            << mitm_ns / orbit_ns << '\n';
        std::cout << "automorphism bench kind=" << automorphism_name(aut.kind) << " degree=" << degree_value
                  << " sign_orbits=" << sign_orbits << " mitm_ns=" << mitm_ns << " orbit_ns=" << orbit_ns << '\n';
    }
    std::cerr << "automorphism benchmark checksum=" << sink << '\n';
}

std::string tuned_root_method(u64 p, std::size_t degree_value) {
    if (degree_value <= 8) return "tree";
    if (p == 1009) {
        if (degree_value == 16) return "projective";
        if (degree_value == 32) return "workspace";
        return "mitm";
    }
    if (p == 1000003 && degree_value >= 128) return "workspace";
    return "tree";
}

void find_roots(const Options& o) {
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    std::vector<Point> points;
    points.reserve(o.xs.size());
    for (u64 x : o.xs) {
        auto y = e.f.sqrt(e.rhs(x));
        if (!y) throw std::runtime_error("x=" + std::to_string(x) + " is not an F_p point x-coordinate on this curve");
        points.push_back({x, *y, false});
    }
    const std::size_t nominal_degree = 1ULL << (points.size() - 1);
    const std::string selected = o.method == "auto" ? tuned_root_method(o.prime, nominal_degree) : o.method;
    RootResult r;
    if (selected == "naive") r = roots_naive(e, points);
    else if (selected == "gray") r = roots_gray(e, points);
    else if (selected == "workspace") r = roots_mitm_workspace(e, points);
    else if (selected == "mitm") r = roots_mitm_batch(e, points);
    else if (selected == "projective") r = roots_tree_projective(e, points);
    else if (selected == "tree") r = roots_tree_batch(e, points);
    else {
        auto orbit = roots_automorphism_orbits(e, points);
        if (!orbit) throw std::runtime_error("orbit method requires an input multiset invariant under a supported rational automorphism");
        r = std::move(orbit->roots);
    }
    std::sort(r.finite.begin(), r.finite.end());
    std::size_t distinct = r.finite.empty() ? 0 : 1;
    for (std::size_t i = 1; i < r.finite.size(); ++i) distinct += r.finite[i] != r.finite[i - 1];
    std::cout << "{\"p\":" << o.prime << ",\"a\":" << o.a << ",\"b\":" << o.b
              << ",\"arity\":" << (points.size() + 1) << ",\"method\":\"" << selected
              << "\",\"nominal_degree\":" << nominal_degree << ",\"infinity_roots\":" << r.infinity
              << ",\"distinct_roots\":" << distinct << ",\"roots_with_multiplicity\":";
    write_array(std::cout, r.finite);
    if (o.emit_coefficients) {
        std::cout << ",\"coefficient_method\":\""
                  << resolved_coefficient_method(r.finite.size(), o.coefficient_method)
                  << "\",\"coeff_low_to_high\":";
        write_array(std::cout, poly_from_roots_selected(r.finite, e.f, o.coefficient_method));
    }
    std::cout << "}\n";
}

std::vector<Point> lift_inputs(const Curve& e, const std::vector<u64>& xs) {
    std::vector<Point> points;
    points.reserve(xs.size());
    for (u64 x : xs) {
        auto y = e.f.sqrt(e.rhs(x));
        if (!y) throw std::runtime_error("x=" + std::to_string(x) + " is not an F_p point x-coordinate on this curve");
        points.push_back({x, *y, false});
    }
    return points;
}

void find_one_root(const Options& o) {
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    const auto points = lift_inputs(e, o.xs);
    const std::size_t sign_bits = points.size() - 1;
    const u64 mask = o.sign_mask.value_or((1ULL << sign_bits) - 1);
    const Point root = one_signed_sum(e, points, mask);
    std::cout << "{\"p\":" << o.prime << ",\"a\":" << o.a << ",\"b\":" << o.b
              << ",\"arity\":" << (points.size() + 1) << ",\"method\":\"one-signed-sum\""
              << ",\"sign_mask\":" << mask << ",\"infinity\":" << (root.inf ? "true" : "false");
    if (!root.inf) std::cout << ",\"x\":" << root.x << ",\"y\":" << root.y;
    std::cout << "}\n";
}

void find_target(const Options& o) {
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    const auto points = lift_inputs(e, o.xs);
    std::vector<Point> targets;
    targets.reserve(o.target_xs.size());
    std::size_t unliftable = 0;
    for (u64 x : o.target_xs) {
        auto y = e.f.sqrt(e.rhs(x));
        if (!y) { ++unliftable; continue; }
        targets.push_back({x, *y, false});
    }
    const auto result = find_target_root(e, points, targets);
    std::cout << "{\"p\":" << o.prime << ",\"a\":" << o.a << ",\"b\":" << o.b
              << ",\"arity\":" << (points.size() + 1) << ",\"method\":\"target-mitm-hash\""
              << ",\"targets\":" << o.target_xs.size() << ",\"liftable_targets\":" << targets.size()
              << ",\"unliftable_targets\":" << unliftable << ",\"left_count\":" << result.left_count
              << ",\"right_count\":" << result.right_count << ",\"found\":" << (result.found ? "true" : "false");
    if (result.found)
        std::cout << ",\"target_x\":" << result.target_x << ",\"sign_mask\":" << result.sign_mask
                  << ",\"sum_y\":" << result.sum.y;
    std::cout << "}\n";
}

void automorphism_audit(const Options& o) {
    Curve e{Field(o.prime, o.reduction), o.a, o.b};
    const auto points = lift_inputs(e, o.xs);
    const Automorphism aut = curve_automorphism(e);
    std::cout << "{\"p\":" << o.prime << ",\"a\":" << o.a << ",\"b\":" << o.b
              << ",\"available\":" << (aut.available() ? "true" : "false")
              << ",\"kind\":\"" << automorphism_name(aut.kind) << "\"";
    if (!aut.available()) { std::cout << "}\n"; return; }
    const bool covariance = automorphism_covariance(e, aut, points);
    auto action = signed_input_action(e, aut, points);
    auto orbit = roots_automorphism_orbits(e, points);
    RootResult baseline = roots_mitm_batch(e, points);
    std::vector<u64> transformed_roots = baseline.finite;
    for (u64& x : transformed_roots) x = apply_automorphism_x(e, aut, x);
    const bool root_invariant = sorted(transformed_roots) == sorted(baseline.finite);
    const bool coefficient_sparse = root_invariant &&
        automorphism_coefficient_support(poly_from_roots(baseline.finite, e.f), aut.x_order);
    std::cout << ",\"x_multiplier\":" << aut.x_multiplier
              << ",\"y_multiplier\":" << aut.y_multiplier
              << ",\"point_order\":" << aut.point_order
              << ",\"x_order\":" << aut.x_order
              << ",\"covariance_verified\":" << (covariance ? "true" : "false")
              << ",\"input_invariant\":" << (action ? "true" : "false")
              << ",\"fixed_specialization_root_invariant\":" << (root_invariant ? "true" : "false")
              << ",\"coefficient_orbit_support_verified\":" << (coefficient_sparse ? "true" : "false");
    if (orbit) {
        const bool roots_match = orbit->roots.infinity == baseline.infinity && sorted(orbit->roots.finite) == sorted(baseline.finite);
        std::cout << ",\"sign_classes\":" << orbit->sign_classes
                  << ",\"sign_orbits\":" << orbit->sign_orbits
                  << ",\"class_reduction\":" << std::fixed << std::setprecision(6)
                  << static_cast<double>(orbit->sign_classes) / static_cast<double>(orbit->sign_orbits)
                  << ",\"orbit_roots_verified\":" << (roots_match ? "true" : "false");
    }
    std::cout << "}\n";
}

int main(int argc, char** argv) {
    try {
        Options o = parse_options(argc, argv);
        if (o.command == "self-test") self_test();
        else if (o.command == "generate") generate(o);
        else if (o.command == "benchmark") benchmark(o);
        else if (o.command == "coeff-benchmark") coefficient_benchmark(o);
        else if (o.command == "benchmark-target") benchmark_target(o);
        else if (o.command == "automorphism-benchmark") automorphism_benchmark(o);
        else if (o.command == "roots") find_roots(o);
        else if (o.command == "one-root") find_one_root(o);
        else if (o.command == "target-root") find_target(o);
        else if (o.command == "automorphism-audit") automorphism_audit(o);
        else throw std::runtime_error("unknown command: " + o.command);
        return 0;
    } catch (const std::exception& ex) {
        std::cerr << "error: " << ex.what() << '\n';
        return 1;
    }
}
