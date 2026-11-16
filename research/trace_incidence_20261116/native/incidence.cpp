#include "incidence.hpp"
#include <flint/nmod_poly.h>
#include <flint/ulong_extras.h>
#include <algorithm>
#include <set>
#include <stdexcept>
#include <unordered_map>

namespace paper_ec {
namespace {
struct Poly {
    nmod_poly_t n;
    explicit Poly(Word p) { nmod_poly_init(n, p); }
    Poly(Word p, const Coefficients& c) : Poly(p) {
        for (std::size_t i = 0; i < c.size(); ++i) nmod_poly_set_coeff_ui(n, i, c[i]);
    }
    Poly(const Poly& a) : Poly(a.n->mod.n) { nmod_poly_set(n, a.n); }
    Poly(Poly&& a) noexcept : Poly(a.n->mod.n) { nmod_poly_swap(n, a.n); }
    Poly& operator=(const Poly& a) { if (this != &a) nmod_poly_set(n, a.n); return *this; }
    Poly& operator=(Poly&& a) noexcept { nmod_poly_swap(n, a.n); return *this; }
    ~Poly() { nmod_poly_clear(n); }
    Word p() const { return n->mod.n; }
    long degree() const { return nmod_poly_degree(n); }
    Coefficients coefficients() const {
        Coefficients c(nmod_poly_length(n));
        for (std::size_t i = 0; i < c.size(); ++i) c[i] = nmod_poly_get_coeff_ui(n, i);
        return c;
    }
};
void count(Stats* s, const std::string& key, Word n = 1) { if (s) s->add(key, n); }
Word neg(Word a, Word p) { return a ? p - a : 0; }
Word mul(Word a, Word b, Word p) { return static_cast<Word>((__uint128_t(a) * b) % p); }
Word power(Word a, Word n, Word p) {
    Word out = 1;
    while (n) { if (n & 1) out = mul(out, a, p); a = mul(a, a, p); n >>= 1; }
    return out;
}
Poly constant(Word p, Word a) { return Poly(p, {a % p}); }
Poly plus(const Poly& a, const Poly& b) { Poly c(a.p()); nmod_poly_add(c.n, a.n, b.n); return c; }
Poly minus(const Poly& a, const Poly& b) { Poly c(a.p()); nmod_poly_sub(c.n, a.n, b.n); return c; }
Poly scale(const Poly& a, Word b) { Poly c(a.p()); nmod_poly_scalar_mul_nmod(c.n, a.n, b); return c; }
Poly product(const Poly& a, const Poly& b) { Poly c(a.p()); nmod_poly_mul(c.n, a.n, b.n); return c; }
Poly roots_polynomial(const std::vector<LabelledPoint>& pool, Word p,
                      std::size_t first, std::size_t last) {
    if (first == last) return constant(p, 1);
    if (last - first == 1) return Poly(p, {neg(pool[first].point.x, p), 1});
    const auto middle = first + (last - first) / 2;
    return product(roots_polynomial(pool, p, first, middle), roots_polynomial(pool, p, middle, last));
}
struct Ring {
    const Poly& modulus; Stats* stats;
    Poly reduce(const Poly& a) const { Poly c(a.p()); nmod_poly_rem(c.n, a.n, modulus.n); return c; }
    Poly times(const Poly& a, const Poly& b) const {
        Poly c(a.p()); nmod_poly_mulmod(c.n, a.n, b.n, modulus.n);
        count(stats, "incidence.quotient_products"); return c;
    }
    Poly inverse(const Poly& a) const {
        Poly c(a.p());
        if (!nmod_poly_invmod(c.n, a.n, modulus.n)) throw std::invalid_argument("nonunit incidence denominator");
        count(stats, "incidence.quotient_inversions"); return c;
    }
};
Poly incidence_gcd(const LabelledPools& input, Stats* stats) {
    const auto& c = input.target;
    const auto f2 = roots_polynomial(input.pools[1], c.p, 0, input.pools[1].size());
    const auto f3 = roots_polynomial(input.pools[2], c.p, 0, input.pools[2].size());
    const Ring ring{f3, stats};
    const Poly z(c.p, {0, 1}); Poly accumulated = constant(c.p, 1);
    for (const auto& first : input.pools[0]) {
        const Word x = first.point.x;
        const auto zx = minus(z, constant(c.p, x));
        const auto denominator = ring.inverse(ring.reduce(product(zx, zx)));
        const auto B = scale(plus(product(plus(z, constant(c.p, x)),
            plus(scale(z, x), constant(c.p, c.a))), constant(c.p, mul(2, c.b, c.p))), neg(2 % c.p, c.p));
        const auto za = minus(scale(z, x), constant(c.p, c.a));
        const auto C = minus(product(za, za), scale(plus(z, constant(c.p, x)), mul(4, c.b, c.p)));
        const auto beta = ring.times(ring.reduce(B), denominator);
        const auto gamma = ring.times(ring.reduce(C), denominator);
        Poly r0(c.p), r1(c.p);
        for (long i = f2.degree(); i >= 0; --i) {
            auto next0 = minus(constant(c.p, nmod_poly_get_coeff_ui(f2.n, i)), ring.times(gamma, r1));
            auto next1 = minus(r0, ring.times(beta, r1));
            r0 = std::move(next0); r1 = std::move(next1);
        }
        const auto cross = ring.times(r0, r1);
        const auto norm = plus(minus(ring.times(r0, r0), ring.times(beta, cross)),
            ring.times(gamma, ring.times(r1, r1)));
        accumulated = ring.times(accumulated, norm);
        count(stats, "incidence.first_pool_iterations");
    }
    Poly result(c.p); nmod_poly_gcd(result.n, f3.n, accumulated.n);
    count(stats, "incidence.final_gcds");
    count(stats, "incidence.gcd_degree", std::max<long>(0, result.degree()));
    return result;
}
} // namespace

IncidenceAnswer solve_incidence(const LabelledPools& input, RootBackend backend,
                               const PreparedDatum* auxiliary, Stats* stats) {
    const auto& c = input.target; const Word r = input.subgroup_order;
    count(stats, "incidence.calls");
    if (c.p <= 3 || c.p >= (Word(1) << 63) || r <= 3 || r >= (Word(1) << 63) ||
        !n_is_prime(c.p) || !n_is_prime(r) || input.generator.infinity || input.public_target.infinity ||
        !on_curve(c, input.generator) || !on_curve(c, input.public_target) ||
        !point_mul(c, r, input.generator, stats).infinity || !point_mul(c, r, input.public_target, stats).infinity)
        throw std::invalid_argument("invalid target field, prime subgroup or public points");
    if (auxiliary && auxiliary->get().curve.p != c.p) throw std::invalid_argument("auxiliary field mismatch");
    std::set<Word> seen;
    std::unordered_map<Word, std::size_t> second_index, third_index;
    for (std::size_t j = 0; j < 3; ++j) {
        if (input.pools[j].empty()) throw std::invalid_argument("empty labelled pool");
        for (std::size_t i = 0; i < input.pools[j].size(); ++i) {
            const auto& entry = input.pools[j][i];
            if (entry.point.infinity || !on_curve(c, entry.point) || entry.alpha >= r || entry.beta >= r || !seen.insert(entry.point.x).second)
                throw std::invalid_argument("invalid or repeated labelled coordinate");
            if (j == 1) second_index[entry.point.x] = i;
            if (j == 2) third_index[entry.point.x] = i;
            count(stats, "incidence.indexed_input_points");
        }
    }
    const auto g = incidence_gcd(input, stats);
    IncidenceAnswer answer; answer.incidence_gcd = g.coefficients();
    answer.coordinate_relation = g.degree() > 0;
    if (!answer.coordinate_relation) return answer;
    std::vector<Word> roots;
    // Shared constant/linear shortcuts avoid giving any backend artificial work.
    if (g.degree() == 1) {
        roots.push_back(neg(nmod_poly_get_coeff_ui(g.n, 0), c.p));
        count(stats, "incidence.linear_root_shortcuts");
    } else if (backend == RootBackend::KnownCoordinates) {
        for (const auto& entry : input.pools[2]) {
            count(stats, "incidence.known_coordinate_evaluations");
            if (nmod_poly_evaluate_nmod(g.n, entry.point.x) == 0) roots.push_back(entry.point.x);
        }
    } else {
        if (!auxiliary) throw std::invalid_argument("nonlinear root backend requires an auxiliary datum");
        count(stats, "incidence.nonlinear_root_backend_calls");
        roots = backend == RootBackend::Pullback
            ? root_find_pullback(*auxiliary, g.coefficients(), stats)
            : root_find(*auxiliary, g.coefficients(),
                backend == RootBackend::PaperTrace ? TranslationMethod::Trace : TranslationMethod::Dense, stats);
    }
    std::sort(roots.begin(), roots.end());
    for (const auto x : roots) {
        const auto found_third = third_index.find(x);
        if (found_third == third_index.end()) throw std::logic_error("incidence root outside the supplied pool");
        const auto k = found_third->second;
        const auto& third = input.pools[2][k];
        for (std::size_t i = 0; i < input.pools[0].size(); ++i) {
            for (const int sign : {1, -1}) {
                const auto& first = input.pools[0][i];
                const auto signed_first = sign == 1 ? first.point : point_neg(c, first.point);
                const auto needed = point_add(c, point_neg(c, third.point), point_neg(c, signed_first), stats);
                count(stats, "incidence.witness_group_queries");
                if (needed.infinity) continue;
                const auto found_second = second_index.find(needed.x);
                if (found_second == second_index.end()) continue;
                const auto j = found_second->second; const auto& second = input.pools[1][j];
                int second_sign = 0;
                if (second.point.y == needed.y) second_sign = 1;
                else if (neg(second.point.y, c.p) == needed.y) second_sign = -1;
                if (!second_sign) continue;
                const auto signed_second = second_sign == 1 ? second.point : point_neg(c, second.point);
                if (!point_add(c, point_add(c, signed_first, signed_second, stats), third.point, stats).infinity)
                    throw std::logic_error("geometric witness failed original-curve check");
                Word A = (static_cast<__uint128_t>(sign == 1 ? first.alpha : neg(first.alpha, r)) +
                    (second_sign == 1 ? second.alpha : neg(second.alpha, r)) + third.alpha) % r;
                Word B = (static_cast<__uint128_t>(sign == 1 ? first.beta : neg(first.beta, r)) +
                    (second_sign == 1 ? second.beta : neg(second.beta, r)) + third.beta) % r;
                count(stats, "incidence.candidate_witnesses");
                if (!B) {
                    if (A) throw std::invalid_argument("geometric witness has inconsistent mixed labels");
                    count(stats, "incidence.uninformative_witnesses"); continue;
                }
                const Word recovered = mul(neg(A, r), power(B, r - 2, r), r);
                if (point_mul(c, recovered, input.generator, stats) != input.public_target)
                    throw std::invalid_argument("recovered scalar fails original-curve verification");
                answer.informative_relation = true;
                answer.positions = {i, j, k}; answer.signs = {sign, second_sign, 1};
                answer.recovered_scalar = recovered;
                return answer;
            }
        }
    }
    return answer; // all coordinate relations can be uninformative; never a solve.
}
} // namespace paper_ec
