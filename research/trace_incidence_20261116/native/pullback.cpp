#include "pullback.hpp"
#include <flint/nmod_poly.h>
#include <algorithm>
#include <functional>
#include <set>
#include <stdexcept>
#include <utility>

namespace paper_ec {
namespace {
struct Polynomial {
    nmod_poly_t n;
    explicit Polynomial(Word p) { nmod_poly_init(n, p); }
    Polynomial(Word p, const Coefficients& c) : Polynomial(p) {
        for (std::size_t i = 0; i < c.size(); ++i) nmod_poly_set_coeff_ui(n, i, c[i]);
    }
    Polynomial(const Polynomial& x) : Polynomial(x.n->mod.n) { nmod_poly_set(n, x.n); }
    Polynomial(Polynomial&& x) noexcept : Polynomial(x.n->mod.n) { nmod_poly_swap(n, x.n); }
    Polynomial& operator=(const Polynomial& x) {
        if (this != &x) {
            if (n->mod.n != x.n->mod.n) { nmod_poly_clear(n); nmod_poly_init(n, x.n->mod.n); }
            nmod_poly_set(n, x.n);
        }
        return *this;
    }
    Polynomial& operator=(Polynomial&& x) noexcept { nmod_poly_swap(n, x.n); return *this; }
    ~Polynomial() { nmod_poly_clear(n); }
    Word p() const { return n->mod.n; }
    long degree() const { return nmod_poly_degree(n); }
    Coefficients coefficients() const {
        Coefficients out(static_cast<std::size_t>(std::max<long>(0, nmod_poly_length(n))));
        for (std::size_t i = 0; i < out.size(); ++i) out[i] = nmod_poly_get_coeff_ui(n, i);
        return out;
    }
};
Word multiply(Word a, Word b, Word p) { return static_cast<Word>((__uint128_t(a) * b) % p); }
Word power(Word a, Word e, Word p) {
    Word out = 1;
    while (e) { if (e & 1) out = multiply(out, a, p); a = multiply(a, a, p); e >>= 1; }
    return out;
}
Word negate(Word a, Word p) { return a ? p - a : 0; }
void count(Stats* s, const std::string& key, Word n = 1) { if (s) s->add(key, n); }
Polynomial remainder(const Polynomial& a, const Polynomial& u) {
    Polynomial out(u.p()); nmod_poly_rem(out.n, a.n, u.n); return out;
}
Polynomial times(const Polynomial& a, const Polynomial& b, const Polynomial& u) {
    Polynomial out(u.p()); nmod_poly_mulmod(out.n, a.n, b.n, u.n); return out;
}
Polynomial plus(const Polynomial& a, const Polynomial& b) {
    Polynomial out(a.p()); nmod_poly_add(out.n, a.n, b.n); return out;
}
Polynomial minus(const Polynomial& a, const Polynomial& b) {
    Polynomial out(a.p()); nmod_poly_sub(out.n, a.n, b.n); return out;
}
Polynomial constant(Word p, Word c) { return Polynomial(p, {c}); }
Polynomial gcd(const Polynomial& a, const Polynomial& b) {
    Polynomial out(a.p()); nmod_poly_gcd(out.n, a.n, b.n); return out;
}
Polynomial divide_exact(const Polynomial& a, const Polynomial& b) {
    Polynomial out(a.p()), residue(a.p());
    nmod_poly_divrem(out.n, residue.n, a.n, b.n);
    if (!nmod_poly_is_zero(residue.n)) throw std::logic_error("nonexact source-factor division");
    return out;
}
Polynomial product_of_roots(Word p, const std::vector<Word>& roots,
                            std::size_t first, std::size_t last) {
    if (first == last) return constant(p, 1);
    if (last - first == 1) return Polynomial(p, {negate(roots[first], p), 1});
    const auto middle = first + (last - first) / 2;
    auto left = product_of_roots(p, roots, first, middle);
    auto right = product_of_roots(p, roots, middle, last);
    Polynomial out(p); nmod_poly_mul(out.n, left.n, right.n); return out;
}
Word scalar_evaluate(const Polynomial& a, Word x, Stats* stats) {
    count(stats, "pullback_scalar_evaluations");
    count(stats, "pullback_scalar_evaluation_coefficients", nmod_poly_length(a.n));
    return nmod_poly_evaluate_nmod(a.n, x);
}
Word checked_integer_power(Word base, unsigned exponent) {
    __uint128_t out = 1;
    for (unsigned i = 0; i < exponent; ++i) {
        out *= base;
        if (out > UINT64_MAX) throw std::invalid_argument("integer prime power overflow");
    }
    return static_cast<Word>(out);
}
unsigned valuation(Word value, Word prime) {
    unsigned out = 0;
    while (value % prime == 0) { value /= prime; ++out; }
    return out;
}
struct Factor { Polynomial u, v; };

// Extract all known finite source points in one pass and one product/division.
// The graph y=v(t) contains at most one actual point over each original t.
void extract_known(Factor& factor, const std::set<Point>& candidates,
                   std::vector<Word>& roots, Stats* stats) {
    std::vector<Word> removed;
    for (const auto& point : candidates) {
        if (point.infinity) continue;
        count(stats, "pullback_exception_candidates");
        if (scalar_evaluate(factor.u, point.x, stats) == 0 &&
            scalar_evaluate(factor.v, point.x, stats) == point.y) removed.push_back(point.x);
    }
    std::sort(removed.begin(), removed.end());
    removed.erase(std::unique(removed.begin(), removed.end()), removed.end());
    if (removed.empty()) return;
    auto product = product_of_roots(factor.u.p(), removed, 0, removed.size());
    factor.u = divide_exact(factor.u, product);
    factor.v = remainder(factor.v, factor.u);
    roots.insert(roots.end(), removed.begin(), removed.end());
    count(stats, "pullback_exception_linear_roots", removed.size());
}

struct Sylow {
    const Curve& curve;
    Word ell, cofactor;
    Point first, second;
    unsigned nu1, nu2;
    Stats* stats;
    std::map<unsigned, std::vector<Point>> cached_representatives;

    const std::vector<Point>& representatives(unsigned m) {
        auto found = cached_representatives.find(m);
        if (found != cached_representatives.end()) return found->second;
        std::vector<Point> reps(1);
        for (const auto& item : {std::make_pair(first, nu1), std::make_pair(second, nu2)}) {
            if (m > item.second) continue;
            const auto generator = point_mul(curve, checked_integer_power(ell, item.second - m), item.first, stats);
            const auto prior = reps;
            reps.clear();
            Point multiple;
            for (Word k = 0; k < ell; ++k) {
                for (const auto& r : prior) reps.push_back(point_add(curve, r, multiple, stats));
                multiple = point_add(curve, multiple, generator, stats);
            }
        }
        count(stats, "pullback_representatives_constructed", reps.size());
        return cached_representatives.emplace(m, std::move(reps)).first->second;
    }

    void split(Factor factor, Point offset, unsigned m,
               std::vector<Factor>& remaining, std::vector<Word>& roots) {
        count(stats, "pullback_recursion_calls");
        count(stats, "pullback_recursion_degree_mass", std::max<long>(0, factor.u.degree()));
        if (factor.u.degree() <= 0) return;
        if (factor.u.degree() == 1) {
            roots.push_back(negate(nmod_poly_get_coeff_ui(factor.u.n, 0), curve.p));
            count(stats, "pullback_linear_shortcuts");
            return;
        }
        if (m == 0) { remaining.push_back(std::move(factor)); return; }
        const auto& reps = representatives(m);
        std::vector<Point> offsets;
        std::set<Point> known;
        offsets.reserve(reps.size());
        for (const auto& representative : reps) {
            const auto t = point_add(curve, offset, representative, stats);
            offsets.push_back(t);
            if (!t.infinity) { known.insert(t); known.insert(point_neg(curve, t)); }
        }
        extract_known(factor, known, roots, stats);
        if (factor.u.degree() <= 0) return;
        if (factor.u.degree() == 1) {
            roots.push_back(negate(nmod_poly_get_coeff_ui(factor.u.n, 0), curve.p));
            count(stats, "pullback_linear_shortcuts");
            return;
        }
        const Word kernel_index = static_cast<Word>(__uint128_t(cofactor) * checked_integer_power(ell, m - 1));
        const auto x = remainder(Polynomial(curve.p, {0, 1}), factor.u);
        long selected_degree = 0;
        for (const auto& t : offsets) {
            Polynomial mapped_x(x), mapped_y(factor.v);
            if (!t.infinity) {
                auto denominator = minus(x, constant(curve.p, t.x));
                Polynomial inverse(curve.p);
                if (!nmod_poly_invmod(inverse.n, denominator.n, factor.u.n))
                    throw std::logic_error("unremoved source exception in pullback denominator");
                count(stats, "pullback_denominator_inversions");
                auto slope = times(plus(factor.v, constant(curve.p, t.y)), inverse, factor.u);
                mapped_x = minus(minus(times(slope, slope, factor.u), x), constant(curve.p, t.x));
                // R-T: -y(-T)+s*(x(T)-x_new) = +y(T)+s*(x(T)-x_new).
                mapped_y = plus(constant(curve.p, t.y), times(slope, minus(constant(curve.p, t.x), mapped_x), factor.u));
            }
            const auto test_coefficients = division_polynomial_mod(curve, kernel_index,
                mapped_x.coefficients(), mapped_y.coefficients(), factor.u.coefficients(), stats);
            const auto selected = gcd(factor.u, Polynomial(curve.p, test_coefficients));
            count(stats, "pullback_membership_tests");
            count(stats, "pullback_membership_degree_mass", factor.u.degree());
            if (selected.degree() <= 0) continue;
            selected_degree += selected.degree();
            Factor child{selected, remainder(factor.v, selected)};
            count(stats, "pullback_parent_to_child_reductions");
            split(std::move(child), t, m - 1, remaining, roots);
        }
        if (selected_degree != factor.u.degree())
            throw std::logic_error("Sylow source factors do not partition their parent");
    }
};

std::vector<Word> separate(const Curve& curve, Word order,
                           const std::vector<PrimePower>& factorization,
                           const GroupBasis& basis, Factor input, Stats* stats) {
    std::vector<Factor> factors;
    if (input.u.degree() > 0) factors.push_back(std::move(input));
    std::vector<Word> roots;
    for (const auto& pp : factorization) {
        if (factors.empty()) break;
        const auto cofactor = order / checked_integer_power(pp.prime, pp.exponent);
        const auto g1 = point_mul(curve, cofactor, basis.first, stats);
        const auto g2 = point_mul(curve, cofactor, basis.second, stats);
        const auto nu1 = valuation(basis.first_order, pp.prime);
        const auto nu2 = valuation(basis.second_order, pp.prime);
        Sylow recursion{curve, pp.prime, cofactor, g1, g2, nu1, nu2, stats, {}};
        std::vector<Factor> next;
        for (auto& factor : factors) recursion.split(std::move(factor), Point{}, std::max(nu1, nu2), next, roots);
        factors = std::move(next);
    }
    for (const auto& factor : factors) {
        if (factor.u.degree() != 1) throw std::logic_error("joint Sylow projection left a non-linear factor");
        roots.push_back(negate(nmod_poly_get_coeff_ui(factor.u.n, 0), curve.p));
    }
    return roots;
}

Polynomial scale_argument(const Polynomial& f, Word scale, Word multiplier) {
    Polynomial out(f.p()); Word current = multiplier;
    for (long i = 0; i < nmod_poly_length(f.n); ++i) {
        nmod_poly_set_coeff_ui(out.n, i, multiply(nmod_poly_get_coeff_ui(f.n, i), current, f.p()));
        current = multiply(current, scale, f.p());
    }
    return out;
}
} // namespace

std::vector<Word> root_find_pullback(const PreparedDatum& prepared,
                                   const Coefficients& input, Stats* stats) {
    const auto& datum = prepared.get(); const auto p = datum.curve.p;
    count(stats, "pullback_calls");
    count(stats, "pullback_full_translation_calls", 0);
    count(stats, "pullback_image_characteristic_polynomials", 0);
    count(stats, "pullback_inverse_compositions", 0);
    if (input.empty() || input.back() != 1 ||
        std::any_of(input.begin(), input.end(), [p](Word c) { return c >= p; }))
        throw std::invalid_argument("input must be canonical and monic");
    Polynomial original(p, input), u(original);
    if (u.degree() == 0) return {};
    Polynomial derivative(p); nmod_poly_derivative(derivative.n, u.n);
    if (gcd(u, derivative).degree() != 0) throw std::invalid_argument("input is not squarefree");
    const Polynomial x(p, {0, 1}); const auto reduced_x = remainder(x, u);
    Polynomial frobenius(p); nmod_poly_powmod_ui_binexp(frobenius.n, reduced_x.n, p, u.n);
    if (!nmod_poly_equal(frobenius.n, reduced_x.n)) throw std::invalid_argument("input is not split over the field");
    std::set<Point> two_torsion;
    for (const auto& g : {std::make_pair(datum.basis.first, datum.basis.first_order),
                          std::make_pair(datum.basis.second, datum.basis.second_order)}) {
        if ((g.second & 1) == 0) two_torsion.insert(point_mul(datum.curve, g.second / 2, g.first, stats));
    }
    const auto initial_torsion = two_torsion;
    for (const auto& a : initial_torsion) for (const auto& b : initial_torsion)
        two_torsion.insert(point_add(datum.curve, a, b, stats));
    std::vector<Word> roots;
    std::vector<Word> removed;
    for (const auto& point : two_torsion) if (!point.infinity && scalar_evaluate(u, point.x, stats) == 0)
        removed.push_back(point.x);
    std::sort(removed.begin(), removed.end()); removed.erase(std::unique(removed.begin(), removed.end()), removed.end());
    if (!removed.empty()) { u = divide_exact(u, product_of_roots(p, removed, 0, removed.size())); roots = removed; }
    if (u.degree() > 0) {
        const auto embedding = finite_field_embedding(datum.curve, u.coefficients(), datum.beta, stats);
        auto direct = separate(datum.curve, datum.order, datum.factors, datum.basis,
            Factor{Polynomial(p, embedding.u0), Polynomial(p, embedding.v0)}, stats);
        roots.insert(roots.end(), direct.begin(), direct.end());
        Polynomial twist_u(p, embedding.u1), twist_v(p, embedding.v1);
        const auto beta_inverse = power(datum.beta, p - 2, p);
        const auto monic_scale = power(beta_inverse, std::max<long>(0, twist_u.degree()), p);
        twist_u = scale_argument(twist_u, datum.beta, monic_scale);
        twist_v = scale_argument(twist_v, datum.beta, beta_inverse);
        auto twisted = separate(datum.twist, datum.twist_order, datum.twist_factors, datum.twist_basis,
            Factor{std::move(twist_u), std::move(twist_v)}, stats);
        for (const auto root : twisted) roots.push_back(multiply(datum.beta, root, p));
    }
    std::sort(roots.begin(), roots.end());
    if (roots.size() != static_cast<std::size_t>(original.degree()) ||
        std::adjacent_find(roots.begin(), roots.end()) != roots.end())
        throw std::logic_error("wrong root count or duplicate output");
    const auto reconstruction = product_of_roots(p, roots, 0, roots.size());
    count(stats, "pullback_output_product_validations");
    if (!nmod_poly_equal(original.n, reconstruction.n))
        throw std::logic_error("output product does not reconstruct input");
    return roots;
}
} // namespace paper_ec
