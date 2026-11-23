// Native port of the preserved trace/dense translation kernel.
// Sources: supplied arXiv:2610.32508v1, Algorithm 3, Remarks 5.1/5.2 and
// section 8.1; historical/fast_trace/{kernel.py,recurrence.cpp} (frozen).
// This is a development implementation, with no timing/evidence admission.
// The trace recurrence is quadratic Berlekamp--Massey, not a half-GCD claim.
// Preconditions: 3 < p < 2^63 is prime; each u is split; components have
// disjoint actual points (opposite ordinates at a shared x are permitted).
// Monicity, separability, curve membership and nonsingularity are checked.
// nonsquare=0 requests a charged least-nonsquare search when collision lifting
// needs it. Such search has no unconditional polylog deterministic bound.

#include "../native/api.hpp"
#include <flint/flint.h>
#include <flint/nmod_mat.h>
#include <flint/nmod_poly.h>
#include <algorithm>
#include <cmath>
#include <limits>
#include <stdexcept>
#include <utility>

namespace paper_ec {
namespace {
using Wide = __uint128_t;

void count(Stats* stats, const std::string& key, Word n = 1) {
    if (stats) stats->add(key, n);
}
Word add(Word x, Word y, Word p) { return (x + y) % p; }
Word sub(Word x, Word y, Word p) { return (x + p - y) % p; }
Word mul(Word x, Word y, Word p) { return static_cast<Word>(Wide(x) * y % p); }
Word power(Word x, Word e, Word p) {
    Word result = 1;
    while (e) {
        if (e & 1) result = mul(result, x, p);
        x = mul(x, x, p);
        e >>= 1;
    }
    return result;
}
Word inverse(Word x, Word p) {
    if (!x) throw std::invalid_argument("nonunit field denominator");
    return power(x, p - 2, p);
}
void check_field(const Curve& curve) {
    if (curve.p <= 3 || curve.p >= (Word(1) << 63))
        throw std::invalid_argument("prime must satisfy 3 < p < 2^63");
}
void check_curve(const Curve& curve) {
    check_field(curve);
    Word p = curve.p, a = curve.a % p, b = curve.b % p;
    if (!add(mul(4, mul(a, mul(a, a, p), p), p),
             mul(27 % p, mul(b, b, p), p), p))
        throw std::invalid_argument("singular curve");
}
Point canonical(const Curve& curve, Point point) {
    if (point.infinity) return {};
    point.x %= curve.p;
    point.y %= curve.p;
    return point;
}

// RAII owners keep the native execution path independent of Python/wrappers.
struct Poly {
    nmod_poly_t value;
    explicit Poly(Word p) { nmod_poly_init(value, p); }
    Poly(Word p, const Coefficients& coefficients) : Poly(p) {
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            nmod_poly_set_coeff_ui(value, static_cast<slong>(i), coefficients[i] % p);
    }
    Poly(const Poly& other) : Poly(other.modulus()) { nmod_poly_set(value, other.value); }
    Poly(Poly&& other) noexcept : Poly(other.modulus()) { nmod_poly_swap(value, other.value); }
    Poly& operator=(const Poly& other) {
        if (this != &other) {
            if (modulus() != other.modulus()) throw std::logic_error("polynomial modulus mismatch");
            nmod_poly_set(value, other.value);
        }
        return *this;
    }
    Poly& operator=(Poly&& other) noexcept {
        nmod_poly_swap(value, other.value);
        return *this;
    }
    ~Poly() { nmod_poly_clear(value); }
    Word modulus() const { return value->mod.n; }
    slong degree() const { return nmod_poly_degree(value); }
    Word coefficient(slong i) const { return nmod_poly_get_coeff_ui(value, i); }
    Coefficients coefficients() const {
        Coefficients result(static_cast<std::size_t>(nmod_poly_length(value)));
        for (std::size_t i = 0; i < result.size(); ++i) result[i] = coefficient(static_cast<slong>(i));
        return result;
    }
};
struct Matrix {
    nmod_mat_t value;
    Matrix(slong rows, slong columns, Word p) { nmod_mat_init(value, rows, columns, p); }
    Matrix(const Matrix&) = delete;
    Matrix& operator=(const Matrix&) = delete;
    ~Matrix() { nmod_mat_clear(value); }
};
Poly constant(Word p, Word x) { return Poly(p, {x % p}); }
Poly add_poly(const Poly& x, const Poly& y) {
    Poly result(x.modulus()); nmod_poly_add(result.value, x.value, y.value); return result;
}
Poly sub_poly(const Poly& x, const Poly& y) {
    Poly result(x.modulus()); nmod_poly_sub(result.value, x.value, y.value); return result;
}
Poly mul_poly(const Poly& x, const Poly& y) {
    Poly result(x.modulus()); nmod_poly_mul(result.value, x.value, y.value); return result;
}
Poly scalar(const Poly& x, Word c) {
    Poly result(x.modulus()); nmod_poly_scalar_mul_nmod(result.value, x.value, c % x.modulus()); return result;
}
Poly remainder(const Poly& x, const Poly& modulus) {
    Poly result(x.modulus());
    if (modulus.degree() == 0) return result;
    nmod_poly_rem(result.value, x.value, modulus.value); return result;
}
Poly product_mod(const Poly& x, const Poly& y, const Poly& modulus) {
    return remainder(mul_poly(x, y), modulus);
}
Poly exact_quotient(const Poly& x, const Poly& y) {
    Poly result(x.modulus()), rest(x.modulus());
    nmod_poly_divrem(result.value, rest.value, x.value, y.value);
    if (rest.degree() >= 0) throw std::logic_error("polynomial division was not exact");
    return result;
}
Poly gcd(const Poly& x, const Poly& y) {
    Poly result(x.modulus()); nmod_poly_gcd(result.value, x.value, y.value);
    if (result.degree() >= 0) nmod_poly_make_monic(result.value, result.value);
    return result;
}
Poly derivative(const Poly& x) {
    Poly result(x.modulus()); nmod_poly_derivative(result.value, x.value); return result;
}
Poly invert_mod(const Poly& x, const Poly& modulus) {
    Poly result(x.modulus());
    if (!nmod_poly_invmod(result.value, x.value, modulus.value))
        throw std::invalid_argument("nonunit polynomial denominator");
    return result;
}
Poly pow_mod(const Poly& x, Word e, const Poly& modulus, Stats* stats = nullptr) {
    count(stats, "polynomial_modular_powers");
    Poly result(x.modulus());
    Poly reduced = remainder(x, modulus);
    nmod_poly_powmod_ui_binexp(result.value, reduced.value, e, modulus.value);
    return result;
}
bool equal(const Poly& x, const Poly& y) { return nmod_poly_equal(x.value, y.value) != 0; }

Poly trace_series(const Poly& u, slong length) {
    Word p = u.modulus(); slong d = u.degree();
    Poly reverse(p), inv(p), tail(p), result(p);
    nmod_poly_reverse(reverse.value, u.value, d + 1);
    nmod_poly_inv_series(inv.value, reverse.value, length);
    tail = mul_poly(derivative(reverse), inv);
    nmod_poly_truncate(tail.value, length - 1);
    nmod_poly_shift_left(result.value, tail.value, 1);
    result = sub_poly(constant(p, static_cast<Word>(d) % p), result);
    nmod_poly_truncate(result.value, length);
    return result;
}
std::vector<Word> dual(const Poly& w, const Poly& series, slong d) {
    Poly reversed(w.modulus()); nmod_poly_reverse(reversed.value, w.value, d);
    Poly convolution = mul_poly(series, reversed);
    std::vector<Word> result(static_cast<std::size_t>(d));
    for (slong k = 0; k < d; ++k) result[static_cast<std::size_t>(k)] = convolution.coefficient(d - 1 + k);
    return result;
}
std::pair<std::vector<Word>, std::vector<Word>>
power_project(const Poly& u, const Poly& phi, const Poly& g, Stats* stats) {
    Word p = u.modulus(); slong d = u.degree();
    if (d <= 0 || d > std::numeric_limits<slong>::max() / 2)
        throw std::invalid_argument("unsupported power projection degree");
    slong length = 2 * d;
    slong m = static_cast<slong>(std::sqrt(static_cast<long double>(length)));
    while (m * m < length) ++m;
    slong blocks = (length + m - 1) / m;
    Poly series = trace_series(u, 2 * d - 1);
    Poly reverse_u(p), inverse_u(p);
    nmod_poly_reverse(reverse_u.value, u.value, d + 1);
    nmod_poly_inv_series(inverse_u.value, reverse_u.value, d + 1);
    auto preproduct = [&](const Poly& a, const Poly& b) {
        Poly out(p);
        nmod_poly_mulmod_preinv(out.value, a.value, b.value, u.value, inverse_u.value);
        return out;
    };
    std::vector<Poly> babies; babies.reserve(static_cast<std::size_t>(m));
    babies.push_back(constant(p, 1));
    for (slong j = 1; j < m; ++j) babies.push_back(preproduct(babies.back(), phi));
    Poly giant = preproduct(babies.back(), phi);
    Matrix A(2 * blocks, d, p), BT(d, m, p), result(2 * blocks, m, p);
    Poly w = constant(p, 1), wg = remainder(g, u);
    for (slong i = 0; i < blocks; ++i) {
        auto first = dual(w, series, d), second = dual(wg, series, d);
        for (slong k = 0; k < d; ++k) {
            nmod_mat_entry(A.value, 2 * i, k) = first[static_cast<std::size_t>(k)];
            nmod_mat_entry(A.value, 2 * i + 1, k) = second[static_cast<std::size_t>(k)];
        }
        w = preproduct(w, giant); wg = preproduct(wg, giant);
    }
    for (slong k = 0; k < d; ++k)
        for (slong j = 0; j < m; ++j) nmod_mat_entry(BT.value, k, j) = babies[static_cast<std::size_t>(j)].coefficient(k);
    nmod_mat_mul(result.value, A.value, BT.value);
    std::vector<Word> moments(static_cast<std::size_t>(length)), weighted(static_cast<std::size_t>(length));
    for (slong k = 0; k < length; ++k) {
        slong i = k / m, j = k % m;
        moments[static_cast<std::size_t>(k)] = nmod_mat_entry(result.value, 2 * i, j);
        weighted[static_cast<std::size_t>(k)] = nmod_mat_entry(result.value, 2 * i + 1, j);
    }
    count(stats, "trace_projection_calls");
    count(stats, "trace_projection_baby_powers", static_cast<Word>(m));
    count(stats, "trace_projection_giant_blocks", static_cast<Word>(blocks));
    count(stats, "trace_projection_hankel_convolutions", static_cast<Word>(2 * blocks));
    count(stats, "trace_projection_matrix_entries", static_cast<Word>(2 * blocks * d + d * m + 2 * blocks * m));
    return {std::move(moments), std::move(weighted)};
}
Poly recurrence_quadratic(const std::vector<Word>& sequence, Word p, Stats* stats) {
    std::vector<Word> C(1, 1), B(1, 1);
    std::size_t degree = 0, shift = 1; Word last = 1;
    for (std::size_t n = 0; n < sequence.size(); ++n) {
        Word delta = sequence[n];
        for (std::size_t i = 1; i <= degree; ++i) delta = add(delta, mul(C[i], sequence[n - i], p), p);
        count(stats, "trace_bm_steps"); count(stats, "trace_bm_discrepancy_terms", static_cast<Word>(degree));
        if (!delta) { ++shift; continue; }
        auto old = C; Word factor = mul(delta, inverse(last, p), p);
        count(stats, "trace_bm_field_inversions");
        if (C.size() < B.size() + shift) C.resize(B.size() + shift, 0);
        for (std::size_t j = 0; j < B.size(); ++j) C[j + shift] = sub(C[j + shift], mul(factor, B[j], p), p);
        count(stats, "trace_bm_update_terms", static_cast<Word>(B.size()));
        if (2 * degree <= n) { degree = n + 1 - degree; B = std::move(old); last = delta; shift = 1; }
        else ++shift;
    }
    // Keep the nominal degree even when the image contains the eigenvalue 0.
    C.resize(degree + 1, 0); std::reverse(C.begin(), C.end());
    return Poly(p, C);
}
// FLINT's batch recurrence uses Euclidean / half-GCD reduction. V1 is
// already in characteristic-polynomial orientation, including a zero root.
#ifndef ROOT_RECURRENCE_MODE
#define ROOT_RECURRENCE_MODE 1
#endif
Poly recurrence(const std::vector<Word>& sequence, Word p, Stats* stats) {
    if (ROOT_RECURRENCE_MODE == 2 && sequence.size() <= 64)
        return recurrence_quadratic(sequence, p, stats);
    nmod_berlekamp_massey_t state;
    nmod_berlekamp_massey_init(state, p);
    std::vector<mp_limb_t> points(sequence.begin(), sequence.end());
    nmod_berlekamp_massey_add_points(state, points.data(), points.size());
    nmod_berlekamp_massey_reduce(state);
    Poly result(p);
    nmod_poly_make_monic(result.value, nmod_berlekamp_massey_V_poly(state));
    nmod_berlekamp_massey_clear(state);
    count(stats, "trace_flint_recurrence_calls");
    return result;
}
Poly trace_numerator(const Poly& q, const std::vector<Word>& moments) {
    slong n = q.degree(); Coefficients reverse(static_cast<std::size_t>(n));
    for (slong i = 0; i < n; ++i) reverse[static_cast<std::size_t>(i)] = moments[static_cast<std::size_t>(n - 1 - i)];
    Poly product = mul_poly(q, Poly(q.modulus(), reverse)), result(q.modulus());
    nmod_poly_shift_right(result.value, product.value, n); return result;
}
struct Reconstructed {
    Poly single, doubled, ordinate;
    Reconstructed(Poly s, Poly d, Poly v) : single(std::move(s)), doubled(std::move(d)), ordinate(std::move(v)) {}
};
Reconstructed reconstruct_trace(const Poly& u, const Poly& phi, const Poly& g, Stats* stats) {
    Word p = u.modulus(); auto projections = power_project(u, phi, g, stats);
    Poly q = recurrence(projections.first, p, stats);
    Poly inv = invert_mod(derivative(q), q);
    Poly multiplicity = product_mod(trace_numerator(q, projections.first), inv, q);
    Poly single = gcd(q, sub_poly(multiplicity, constant(p, 1)));
    Poly doubled = gcd(q, sub_poly(multiplicity, constant(p, 2)));
    if (!equal(mul_poly(single, doubled), q)) throw std::invalid_argument("image multiplicity exceeds two");
    Poly ordinate = remainder(product_mod(trace_numerator(q, projections.second), inv, q), single);
    count(stats, "trace_reconstruction_calls");
    return {std::move(single), std::move(doubled), std::move(ordinate)};
}
Reconstructed reconstruct_dense(const Poly& u, const Poly& phi, const Poly& g, Stats* stats) {
    Word p = u.modulus(); slong d = u.degree();
    Matrix matrix(d, d, p); Poly x(p, {0, 1}), current = remainder(phi, u), chi(p);
    for (slong j = 0; j < d; ++j) {
        for (slong i = 0; i < d; ++i) nmod_mat_entry(matrix.value, i, j) = current.coefficient(i);
        current = product_mod(current, x, u);
    }
    nmod_mat_charpoly(chi.value, matrix.value);
    Poly doubled = gcd(chi, derivative(chi));
    Poly single = exact_quotient(chi, mul_poly(doubled, doubled));
    count(stats, "dense_reconstruction_calls"); count(stats, "dense_charpoly_dimension", static_cast<Word>(d));
    count(stats, "dense_matrix_entries", static_cast<Word>(d * d));
    if (single.degree() <= 0) return {std::move(single), std::move(doubled), Poly(p)};
    Poly composed(p); nmod_poly_compose_mod(composed.value, single.value, phi.value, u.value);
    Poly preimage = gcd(u, composed); slong degree = preimage.degree();
    if (degree != single.degree()) throw std::logic_error("singleton preimage dimension mismatch");
    Matrix A(degree, degree, p), rhs(degree, 1, p), result(degree, 1, p);
    current = constant(p, 1);
    for (slong j = 0; j < degree; ++j) {
        for (slong i = 0; i < degree; ++i) nmod_mat_entry(A.value, i, j) = current.coefficient(i);
        current = product_mod(current, phi, preimage);
    }
    Poly reduced = remainder(g, preimage);
    for (slong i = 0; i < degree; ++i) nmod_mat_entry(rhs.value, i, 0) = reduced.coefficient(i);
    if (!nmod_mat_solve(result.value, A.value, rhs.value)) throw std::logic_error("inverse composition matrix singular");
    Poly ordinate(p);
    for (slong i = 0; i < degree; ++i) nmod_poly_set_coeff_ui(ordinate.value, i, nmod_mat_entry(result.value, i, 0));
    count(stats, "dense_interpolation_dimension", static_cast<Word>(degree));
    count(stats, "dense_matrix_entries", static_cast<Word>(degree * degree));
    return {std::move(single), std::move(doubled), std::move(ordinate)};
}

Poly sqrt_split(const Poly& h, const Poly& u, Word beta, Stats* stats) {
    Word p = u.modulus();
    if (u.degree() <= 0) return Poly(p);
    count(stats, "collision_lift_calls"); count(stats, "collision_lift_degree", static_cast<Word>(u.degree()));
    if (p % 4 == 3) {
        Poly result = pow_mod(h, (p + 1) / 4, u, stats);
        if (remainder(sub_poly(mul_poly(result, result), h), u).degree() >= 0)
            throw std::invalid_argument("collision polynomial does not lift");
        return result;
    }
    if (!beta) {
        beta = 2;
        for (;;) {
            count(stats, "nonsquare_search_trials"); count(stats, "nonsquare_legendre_tests");
            if (power(beta, (p - 1) / 2, p) == p - 1) break;
            if (++beta == p) throw std::logic_error("nonsquare search exhausted field");
        }
    } else {
        beta %= p; count(stats, "nonsquare_supplied_checks"); count(stats, "nonsquare_legendre_tests");
        if (power(beta, (p - 1) / 2, p) != p - 1) throw std::invalid_argument("supplied beta is not a nonsquare");
    }
    Word odd = p - 1, s = 0;
    while (!(odd & 1)) { ++s; odd >>= 1; }
    Word inv_beta = inverse(beta, p);
    std::vector<std::pair<Poly, Word>> parts; parts.emplace_back(u, 0);
    for (Word k = 0; k < s; ++k) {
        std::vector<std::pair<Poly, Word>> next;
        for (const auto& entry : parts) {
            const Poly& part = entry.first; Word j = entry.second;
            Poly value = remainder(scalar(h, power(inv_beta, j, p)), part);
            Poly positive = gcd(part, sub_poly(pow_mod(value, (p - 1) >> (k + 1), part, stats), constant(p, 1)));
            Poly negative = exact_quotient(part, positive); count(stats, "collision_class_gcds");
            if (positive.degree() > 0) next.emplace_back(std::move(positive), j);
            if (negative.degree() > 0) next.emplace_back(std::move(negative), j + (Word(1) << k));
        }
        parts = std::move(next);
    }
    std::vector<std::pair<Poly, Poly>> factors;
    for (const auto& entry : parts) {
        const Poly& part = entry.first; Word j = entry.second;
        if (j & 1) throw std::invalid_argument("collision abscissa has no rational ordinate");
        Poly value = remainder(scalar(h, power(inv_beta, j, p)), part);
        Poly root = remainder(scalar(pow_mod(value, odd == 1 ? 0 : (odd + 1) / 2, part, stats), power(beta, j / 2, p)), part);
        factors.emplace_back(part, std::move(root));
    }
    while (factors.size() > 1) {
        std::vector<std::pair<Poly, Poly>> next;
        for (std::size_t i = 0; i < factors.size(); i += 2) {
            if (i + 1 == factors.size()) { next.push_back(std::move(factors[i])); continue; }
            const Poly& f = factors[i].first; const Poly& v = factors[i].second;
            const Poly& q = factors[i + 1].first; const Poly& w = factors[i + 1].second;
            Poly correction = product_mod(invert_mod(remainder(f, q), q), sub_poly(w, v), q);
            Poly joined = add_poly(v, mul_poly(f, correction));
            next.emplace_back(mul_poly(f, q), std::move(joined)); count(stats, "collision_crt_merges");
        }
        factors = std::move(next);
    }
    if (factors.empty()) throw std::logic_error("empty collision partition");
    Poly result = remainder(factors.front().second, u);
    if (remainder(sub_poly(mul_poly(result, result), h), u).degree() >= 0)
        throw std::logic_error("split square root reconstruction failed");
    return result;
}
void append_point(Divisor& divisor, Point point, Word p) {
    if (point.infinity) divisor.infinity = true;
    else divisor.components.push_back({{sub(0, point.x, p), 1}, {point.y}});
}
} // namespace

bool on_curve(const Curve& curve, Point point) {
    if (curve.p <= 3 || curve.p >= (Word(1) << 63)) return false;
    if (point.infinity) return true;
    Word p = curve.p, x = point.x % p, y = point.y % p;
    return mul(y, y, p) == add(add(mul(x, mul(x, x, p), p), mul(curve.a % p, x, p), p), curve.b % p, p);
}
Point point_neg(const Curve& curve, Point point) {
    check_field(curve); point = canonical(curve, point);
    if (!point.infinity) point.y = sub(0, point.y, curve.p);
    return point;
}
Point point_add(const Curve& curve, Point first, Point second, Stats* stats) {
    check_field(curve); count(stats, "point_add_calls");
    Word p = curve.p; first = canonical(curve, first); second = canonical(curve, second);
    if (first.infinity) return second;
    if (second.infinity) return first;
    Word slope;
    if (first.x == second.x) {
        if (!add(first.y, second.y, p)) return {};
        slope = mul(add(mul(3, mul(first.x, first.x, p), p), curve.a % p, p), inverse(mul(2, first.y, p), p), p);
        count(stats, "point_doublings");
    } else slope = mul(sub(second.y, first.y, p), inverse(sub(second.x, first.x, p), p), p);
    count(stats, "point_add_field_inversions");
    Word x = sub(sub(mul(slope, slope, p), first.x, p), second.x, p);
    return {x, sub(mul(slope, sub(first.x, x, p), p), first.y, p), false};
}
Point point_mul(const Curve& curve, Word scalar_value, Point point, Stats* stats) {
    check_field(curve); count(stats, "point_mul_calls"); Point result;
    while (scalar_value) {
        if (scalar_value & 1) result = point_add(curve, result, point, stats);
        scalar_value >>= 1;
        if (scalar_value) point = point_add(curve, point, point, stats);
    }
    return result;
}
Divisor translate(const Curve& curve, const Divisor& input, Point Q,
                  TranslationMethod method, Word nonsquare, Stats* stats) {
    check_curve(curve); flint_set_num_threads(1);
    if (!on_curve(curve, Q)) throw std::invalid_argument("Q is not on curve");
    if (method != TranslationMethod::Trace && method != TranslationMethod::Dense)
        throw std::invalid_argument("unknown translation method");
    Word p = curve.p; Q = canonical(curve, Q);
    Poly h(p, {curve.b % p, curve.a % p, 0, 1}); Divisor output;
    count(stats, "translation_calls"); count(stats, "translation_input_components", input.components.size());
    if (input.infinity) { append_point(output, Q, p); count(stats, "translation_input_infinity"); }
    for (const auto& component : input.components) {
        Poly u(p, component.u), v(p, component.v);
        if (u.degree() < 0 || u.coefficient(u.degree()) != 1) throw std::invalid_argument("u must be monic");
        if (gcd(u, derivative(u)).degree() != 0) throw std::invalid_argument("u must be squarefree");
        if (remainder(sub_poly(mul_poly(v, v), h), u).degree() >= 0) throw std::invalid_argument("input divisor is not on curve");
        count(stats, "translation_input_degree", static_cast<Word>(u.degree()));
        if (Q.infinity) {
            if (u.degree() > 0) output.components.push_back({u.coefficients(), remainder(v, u).coefficients()});
            continue;
        }
        if (u.degree() > 0 && !nmod_poly_evaluate_nmod(u.value, Q.x)) {
            Point P{Q.x, nmod_poly_evaluate_nmod(v.value, Q.x), false};
            append_point(output, point_add(curve, P, Q, stats), p);
            u = exact_quotient(u, Poly(p, {sub(0, Q.x, p), 1})); v = remainder(v, u);
            count(stats, "translation_exceptional_points");
        }
        if (u.degree() <= 0) continue;
        Poly denominator(p, {sub(0, Q.x, p), 1});
        Poly slope = product_mod(sub_poly(v, constant(p, Q.y)), invert_mod(denominator, u), u);
        Poly phi = remainder(sub_poly(mul_poly(slope, slope), Poly(p, {Q.x, 1})), u);
        Poly ordinate = remainder(sub_poly(mul_poly(slope, sub_poly(constant(p, Q.x), phi)), constant(p, Q.y)), u);
        Reconstructed result = method == TranslationMethod::Trace ? reconstruct_trace(u, phi, ordinate, stats) : reconstruct_dense(u, phi, ordinate, stats);
        if (result.single.degree() > 0) output.components.push_back({result.single.coefficients(), result.ordinate.coefficients()});
        if (result.doubled.degree() > 0) {
            Poly root = sqrt_split(h, result.doubled, nonsquare, stats);
            output.components.push_back({result.doubled.coefficients(), root.coefficients()});
            output.components.push_back({result.doubled.coefficients(), scalar(root, p - 1).coefficients()});
        }
        count(stats, "translation_singleton_degree", static_cast<Word>(result.single.degree()));
        count(stats, "translation_collision_degree", static_cast<Word>(result.doubled.degree()));
    }
    return output;
}
} // namespace paper_ec
