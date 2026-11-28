#include "isogeny.hpp"
#include <flint/ulong_extras.h>
#include <algorithm>
#include <stdexcept>
#include <utility>

namespace paper_ec {
namespace {
void count(Stats* s, const std::string& key, Word n = 1) { if (s) s->add(key, n); }
Word mul(Word a, Word b, Word p, Stats* s) {
    count(s, "auxiliary.scalar_multiplications");
    return static_cast<Word>((__uint128_t(a) * b) % p);
}
Word add(Word a, Word b, Word p) { return static_cast<Word>((__uint128_t(a) + b) % p); }
Word sub(Word a, Word b, Word p) { return a >= b ? a - b : p - (b - a); }
Word power(Word a, Word exponent, Word p, Stats* s) {
    Word out = 1;
    while (exponent) {
        if (exponent & 1) out = mul(out, a, p, s);
        a = mul(a, a, p, s); exponent >>= 1;
    }
    return out;
}
Word inverse(Word a, Word p, Stats* s) {
    if (!a) throw std::invalid_argument("zero auxiliary scalar inverse");
    count(s, "auxiliary.scalar_inversions");
    return power(a, p - 2, p, s);
}
void valid_curve(const Curve& c, Stats* s) {
    count(s, "auxiliary.primality_tests");
    if (c.p <= 3 || c.p >= (Word(1) << 63) || !n_is_prime(c.p) || c.a >= c.p || c.b >= c.p)
        throw std::invalid_argument("unsupported auxiliary curve field");
    const Word a3 = mul(c.a, mul(c.a, c.a, c.p, s), c.p, s);
    const Word b2 = mul(c.b, c.b, c.p, s);
    if (!add(mul(4 % c.p, a3, c.p, s), mul(27 % c.p, b2, c.p, s), c.p))
        throw std::invalid_argument("singular auxiliary curve");
}
std::vector<PrimePower> factor_integer(Word n, Stats* s) {
    std::vector<PrimePower> out;
    for (Word ell = 2; ell <= n / ell; ++ell) {
        count(s, "auxiliary.factor_trial_divisions");
        if (n % ell) continue;
        unsigned exponent = 0;
        do { n /= ell; ++exponent; count(s, "auxiliary.factor_exact_divisions"); } while (n % ell == 0);
        out.push_back({ell, exponent});
    }
    if (n > 1) out.push_back({n, 1});
    return out;
}
std::vector<Point> enumerate(const Curve& c, Stats* s) {
    std::vector<Point> points(1);
    for (Word x = 0; x < c.p; ++x) {
        const Word h = add(add(mul(mul(x, x, c.p, s), x, c.p, s), mul(c.a, x, c.p, s), c.p), c.b, c.p);
        for (Word y = 0; y < c.p; ++y) {
            count(s, "auxiliary.exhaustive_coordinate_pairs");
            if (mul(y, y, c.p, s) == h) points.push_back({x, y, false});
        }
    }
    count(s, "auxiliary.points_materialized", points.size());
    return points;
}
Word order_of(const Curve& c, Point point, Word group_order,
              const std::vector<PrimePower>& factors, Stats* s) {
    Word order = group_order;
    count(s, "auxiliary.point_order_tests");
    for (const auto& factor : factors) while (order % factor.prime == 0) {
        const auto trial = point_mul(c, order / factor.prime, point, s);
        if (!trial.infinity) break;
        order /= factor.prime;
    }
    return order;
}
GroupBasis complete_basis(const Curve& c, const std::vector<Point>& points,
                          const std::vector<PrimePower>& factors, Stats* s) {
    const Word order = points.size();
    std::vector<Word> orders;
    Point first; Word largest = 1;
    for (const auto point : points) {
        const Word current = order_of(c, point, order, factors, s);
        orders.push_back(current);
        if (current > largest) { first = point; largest = current; }
    }
    const Word other = order / largest;
    for (std::size_t i = 0; i < points.size(); ++i) {
        if (orders[i] != other) continue;
        const auto second = points[i]; bool independent = true;
        for (const auto& factor : factors) {
            if (largest % factor.prime || other % factor.prime) continue;
            const auto a = point_mul(c, largest / factor.prime, first, s);
            const auto b = point_mul(c, other / factor.prime, second, s);
            Point multiple;
            for (Word j = 0; j < factor.prime; ++j) {
                count(s, "auxiliary.basis_independence_tests");
                if (multiple == b) { independent = false; break; }
                multiple = point_add(c, multiple, a, s);
            }
            if (!independent) break;
        }
        if (independent) return {first, second, largest, other};
    }
    throw std::logic_error("exhaustive auxiliary basis completion failed");
}
} // namespace

TwoIsogeny two_isogeny(const Curve& c, Point kernel, Stats* stats) {
    valid_curve(c, stats);
    if (kernel.infinity || kernel.y != 0 || !on_curve(c, kernel))
        throw std::invalid_argument("isogeny kernel must be a rational point of exact order two");
    const Word coefficient = add(mul(3 % c.p, mul(kernel.x, kernel.x, c.p, stats), c.p, stats), c.a, c.p);
    Curve target{c.p,
        sub(c.a, mul(5 % c.p, coefficient, c.p, stats), c.p),
        sub(c.b, mul(7 % c.p, mul(kernel.x, coefficient, c.p, stats), c.p, stats), c.p)};
    valid_curve(target, stats);
    count(stats, "auxiliary.two_isogenies_constructed");
    return {c, target, kernel, coefficient};
}
Point isogeny_image(const TwoIsogeny& map, Point point, Stats* stats) {
    if (!on_curve(map.domain, point)) throw std::invalid_argument("isogeny input is off curve");
    count(stats, "auxiliary.isogeny_point_evaluations");
    if (point.infinity || point == map.kernel) return {};
    const Word p = map.domain.p;
    const Word denominator = sub(point.x, map.kernel.x, p);
    const Word inv = inverse(denominator, p, stats);
    const Word x = add(point.x, mul(map.kernel_coefficient, inv, p, stats), p);
    const Word y = mul(point.y, sub(1, mul(map.kernel_coefficient, mul(inv, inv, p, stats), p, stats), p), p, stats);
    const Point out{x, y, false};
    if (!on_curve(map.codomain, out)) throw std::logic_error("isogeny image fails codomain equation");
    return out;
}
Datum construct_toy_datum(const Curve& c, Word beta, Stats* stats) {
    if (c.p > 257) throw std::invalid_argument("exhaustive auxiliary fixture construction limited to p<=257");
    valid_curve(c, stats);
    if (beta >= c.p || !beta || power(beta, (c.p - 1) / 2, c.p, stats) != c.p - 1)
        throw std::invalid_argument("auxiliary datum requires a nonsquare");
    const Word inv = inverse(beta, c.p, stats);
    const Word inv2 = mul(inv, inv, c.p, stats);
    const Curve twist{c.p, mul(c.a, inv2, c.p, stats), mul(c.b, mul(inv, inv2, c.p, stats), c.p, stats)};
    const auto points = enumerate(c, stats), twisted_points = enumerate(twist, stats);
    const auto factors = factor_integer(points.size(), stats);
    const auto twist_factors = factor_integer(twisted_points.size(), stats);
    const auto basis = complete_basis(c, points, factors, stats);
    const auto twist_basis = complete_basis(twist, twisted_points, twist_factors, stats);
    count(stats, "auxiliary.exhaustive_data_constructed");
    return {beta, c, twist, static_cast<Word>(points.size()), static_cast<Word>(twisted_points.size()),
            factors, twist_factors, basis, twist_basis};
}
} // namespace paper_ec
