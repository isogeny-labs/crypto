// Fixed exact-rational development checks for Appendix D.4.3-D.4.4.
// No solver, contraction evaluator, performance measurement or asymptotic fit.
#include <gmpxx.h>
#include <algorithm>
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <limits>
#include <map>
#include <optional>
#include <stdexcept>
#include <string>
#include <vector>

using Q = mpq_class;
using Z = mpz_class;
using Poly = std::vector<Q>;
constexpr int coordinates = 4;
using Vec = std::array<Q, coordinates>;
using FVec = std::array<Poly, coordinates>;

Q rational(const Z& n, const Z& d) {
    if (d == 0) throw std::runtime_error("zero rational denominator");
    Q q(n, d); q.canonicalize(); return q;
}
Q rational(long n, long d) { return rational(Z(n), Z(d)); }
Q power(Q base, int exponent) {
    Q result(1);
    while (exponent > 0) {
        if (exponent % 2 != 0) result *= base;
        exponent /= 2;
        if (exponent != 0) base *= base;
    }
    return result;
}
Z power_two(int exponent) {
    if (exponent < 0) throw std::runtime_error("negative power_two");
    Z result(1); mpz_mul_2exp(result.get_mpz_t(), result.get_mpz_t(),
                            static_cast<mp_bitcnt_t>(exponent)); return result;
}
std::optional<long> valuation(const Q& q) {
    if (q == 0) return std::nullopt;
    Z n = q.get_num(); if (n < 0) n = -n;
    Z d = q.get_den();
    return static_cast<long>(mpz_scan1(n.get_mpz_t(), 0))
         - static_cast<long>(mpz_scan1(d.get_mpz_t(), 0));
}
bool at_least(const Q& q, long bound) {
    const auto v = valuation(q); return !v || *v >= bound;
}
std::string vtext(const Q& q) {
    const auto v = valuation(q); return v ? std::to_string(*v) : "inf";
}
int integer_v2(int x) {
    if (x == 0) throw std::runtime_error("v2(0) has no finite value");
    if (x < 0) x = -x;
    int v = 0; while (x % 2 == 0) { ++v; x /= 2; } return v;
}
void trim(Poly& p) {
    while (p.size() > 1 && p.back() == 0) p.pop_back();
}
Poly multiply(const Poly& p, const Poly& q, std::size_t cap = 0) {
    const std::size_t full = p.size() + q.size() - 1;
    const std::size_t count = cap == 0 ? full : std::min(full, cap);
    Poly out(count, Q(0));
    for (std::size_t i = 0; i < p.size() && i < count; ++i)
        for (std::size_t j = 0; j < q.size() && i + j < count; ++j)
            out[i + j] += p[i] * q[j];
    if (cap == 0) trim(out);
    return out;
}
Poly poly_power(Poly p, int exponent) {
    Poly out{Q(1)};
    while (exponent > 0) {
        if (exponent % 2 != 0) out = multiply(out, p);
        exponent /= 2;
        if (exponent != 0) p = multiply(p, p);
    }
    return out;
}
Poly scaled(Poly p, const Q& scale) {
    for (auto& q : p) q *= scale;
    trim(p); return p;
}
Poly shift(const Poly& f, int t, int alpha) {
    Poly result{Q(0)};
    for (std::size_t i = f.size(); i-- > 0;) {
        result = multiply(result, Poly{Q(t), Q(alpha)});
        result[0] += f[i]; trim(result);
    }
    return result;
}
Q evaluate(const Poly& f, const Q& x) {
    Q result(0);
    for (std::size_t i = f.size(); i-- > 0;) result = result * x + f[i];
    return result;
}
Poly prefix(Poly p, int length) {
    p.resize(static_cast<std::size_t>(length), Q(0)); return p;
}
Poly inverse_series(const Poly& p, int length) {
    if (p.empty() || p[0] == 0) throw std::runtime_error("nonunit series");
    Poly inverse(static_cast<std::size_t>(length), Q(0));
    inverse[0] = 1 / p[0];
    for (int n = 1; n < length; ++n) {
        Q coefficient(0);
        for (int j = 1; j <= n && j < static_cast<int>(p.size()); ++j)
            coefficient += p[static_cast<std::size_t>(j)]
                         * inverse[static_cast<std::size_t>(n - j)];
        inverse[static_cast<std::size_t>(n)] = -coefficient / p[0];
    }
    return inverse;
}
Z residue(const Q& q, int precision) {
    if (!at_least(q, 0)) throw std::runtime_error("nonintegral residue request");
    const Z modulus = power_two(precision);
    Z inverse;
    if (mpz_invert(inverse.get_mpz_t(), q.get_den().get_mpz_t(),
                   modulus.get_mpz_t()) == 0)
        throw std::runtime_error("odd denominator unexpectedly not invertible");
    Z result = (q.get_num() * inverse) % modulus;
    if (result < 0) result += modulus;
    return result;
}
Poly canonical_lift(const Poly& p, int precision) {
    Poly out; out.reserve(p.size());
    for (const auto& q : p) out.emplace_back(residue(q, precision));
    return out;
}
Q perturbation(int case_id, int t, int j, int c, int precision) {
    const long sign = (case_id + t + j + c) % 2 == 0 ? 1L : -1L;
    return rational(sign * Z(2 * (j + c) + 1) * power_two(precision),
                    Z(2 * (c + 1) + 1));
}

struct Counters {
    long assertions = 0, kernel_charts = 0, kernel_coefficients = 0;
    long derivative_grid_checks = 0, nonintegral_global_kernel_coefficients = 0;
    long parameter_tuples = 0, cases = 0, hypothesis_coefficients = 0;
    long node_instances = 0, inverse_coefficients = 0;
    long node_error_rows = 0, final_error_rows = 0, negative_controls = 0;
    long finite_error_rows = 0;
    std::optional<long> minimum_error_margin;
    void require(bool condition, const std::string& message) {
        ++assertions;
        if (!condition) throw std::runtime_error(message);
    }
    void inspect_error(const Q& q, int p) {
        if (const auto v = valuation(q)) {
            ++finite_error_rows;
            const long margin = *v - p;
            if (!minimum_error_margin || margin < *minimum_error_margin)
                minimum_error_margin = margin;
        }
    }
};

struct Grid {
    int a, L, alpha;
    std::vector<int> nodes;
    Z cprime_zero;
    std::map<int, Poly> kernels;
    explicit Grid(int dimension) : a(dimension), L(1 << a), alpha(1 << (a + 1)),
                                   cprime_zero(1) {
        for (int t = -L; t < L; ++t) {
            nodes.push_back(t);
            if (t != 0) cprime_zero *= -t;
        }
        for (int delta = -(L - 1); delta < L; ++delta) {
            Poly k{Q(1)};
            for (const int q : nodes)
                if (q != delta) k = multiply(k, Poly{Q(-q), Q(1)});
            kernels[delta] = scaled(k, rational(Z(1), cprime_zero));
        }
    }
};

FVec family(const Grid& g, int M, int which, Vec& expected) {
    FVec f;
    if (which == 0) {
        const int degree = M * (2 * g.L - 1);
        for (int c = 0; c < coordinates; ++c) {
            f[static_cast<std::size_t>(c)].resize(static_cast<std::size_t>(degree + 1));
            for (int j = 0; j <= degree; ++j) {
                const long sign = (j + c) % 2 == 0 ? 1L : -1L;
                f[static_cast<std::size_t>(c)][static_cast<std::size_t>(j)] =
                    rational(sign * (j + 1) * (c + 1), 2 * (j + c) + 1);
            }
            const long sign = c % 2 == 0 ? 1L : -1L;
            expected[static_cast<std::size_t>(c)] = rational(sign * (c + 1), 2 * c + 1);
        }
    } else {
        f[0] = poly_power(g.kernels.at(0), M);
        f[1] = multiply(g.kernels.at(1), poly_power(g.kernels.at(0), M - 1));
        f[2] = scaled(f[0], rational(-3, 5));
        f[3] = scaled(poly_power(g.kernels.at(-1), M), rational(1, 3));
        f[3][0] += rational(7, 3); trim(f[3]);
        expected = Vec{Q(1), Q(0), rational(-3, 5), rational(7, 3)};
    }
    return f;
}
bool hypotheses(const Grid& g, int M, const FVec& f, Counters* counters) {
    for (const auto& coordinate : f) {
        if (coordinate.size() - 1 > static_cast<std::size_t>(M * (2 * g.L - 1)))
            return false;
        if (!at_least(coordinate[0], 0)) return false;
        for (const int t : g.nodes)
            for (const auto& q : shift(coordinate, t, g.alpha)) {
                if (counters != nullptr) ++counters->hypothesis_coefficients;
                if (!at_least(q, 0)) return false;
            }
    }
    return true;
}

void kernel_checks(const Grid& g, std::ofstream& log, Counters& counter) {
    const auto base_v = valuation(Q(g.cprime_zero));
    counter.require(base_v.has_value(), "nonzero Cprime");
    for (const int t : g.nodes) {
        Z cp(1); for (const int q : g.nodes) if (q != t) cp *= t - q;
        counter.require(valuation(Q(cp)) == base_v, "Cprime grid valuation mismatch");
        ++counter.derivative_grid_checks;
    }
    for (const auto& entry : g.kernels) {
        const int delta = entry.first;
        const Poly& k = entry.second;
        counter.require(evaluate(k, Q(0)) == (delta == 0 ? Q(1) : Q(0)),
                        "kernel value at zero mismatch");
        for (const auto& q : k)
            if (!at_least(q, 0)) ++counter.nonintegral_global_kernel_coefficients;
        for (const int t : g.nodes) {
            ++counter.kernel_charts;
            const Poly chart = shift(k, t, g.alpha);
            std::optional<long> minimum;
            for (const auto& q : chart) {
                ++counter.kernel_coefficients;
                counter.require(at_least(q, t == delta ? 0 : 1),
                                "Cauchy chart integrality/divisibility mismatch");
                const auto v = valuation(q);
                if (v && (!minimum || *v < *minimum)) minimum = v;
            }
            log << g.a << '\t' << delta << '\t' << t << '\t' << chart.size()
                << '\t' << (minimum ? std::to_string(*minimum) : "inf")
                << '\t' << (t == delta ? 0 : 1) << "\tPASS\n";
        }
    }
}

void run_case(const Grid& g, int u, int M, int p, int which,
              std::ofstream& cases, std::ofstream& weights, std::ofstream& nodes,
              std::ofstream& finals, Counters& counter) {
    const int id = static_cast<int>(++counter.cases);
    const int h = (3 * M + u - 1) / u;
    const int degree_bound = M * (2 * g.L - 1);
    std::map<int, int> multiplicity;
    int total = 0;
    for (const int t : g.nodes) if (t != 0) {
        const int q = h * std::max(0, u - integer_v2(t));
        multiplicity[t] = q; total += q;
    }
    counter.require(total > degree_bound, "insufficient Hermite multiplicity");
    counter.require(h <= 4 * M / u, "h bound mismatch");
    Vec expected;
    const FVec f = family(g, M, which, expected);
    counter.require(hypotheses(g, M, f, &counter), "source hypotheses failed");
    for (int c = 0; c < coordinates; ++c)
        counter.require(f[static_cast<std::size_t>(c)][0] == expected[static_cast<std::size_t>(c)],
                        "known F(0) mismatch");
    std::array<Vec, 4> reconstructed{};
    const std::array<std::string, 4> names =
        {"exact", "canonical_input", "perturbed_input", "perturbed_quotient"};
    int active = 0;
    for (const auto& node : multiplicity) {
        const int t = node.first, qt = node.second;
        if (qt == 0) continue;
        ++active; ++counter.node_instances;
        const int i = integer_v2(t), D = g.L / (1 << i);
        const int pt = p + 8 * h * D + 8 * M * (g.a + 1);
        Q Ezero(1);
        Poly Echart{Q(1)};
        for (const auto& other : multiplicity) {
            const int q = other.first, qq = other.second;
            if (q == t || qq == 0) continue;
            Ezero *= power(rational(-q, t - q), qq);
            const Poly factor{Q(1), rational(g.alpha, t - q)};
            for (int count = 0; count < qq; ++count)
                Echart = multiply(Echart, factor, static_cast<std::size_t>(qt));
        }
        Echart = prefix(Echart, qt);
        counter.require(Echart[0] == 1, "Hermite E chart constant not one");
        const auto ev = valuation(Ezero);
        counter.require(ev.has_value(), "zero Hermite multiplier");
        long source_valuation = 0;
        for (int j = 1; j <= g.a - i; ++j) {
            const int qhigh = h * std::max(0, u - i - j);
            source_valuation += static_cast<long>(D / (1 << j)) * j * (qhigh - qt);
        }
        counter.require(*ev == source_valuation, "D.10 exact valuation identity failed");
        counter.require(*ev >= -6L * D * h, "D.10 valuation lower bound failed");
        const Poly inverse = inverse_series(Echart, qt);
        const Poly product = prefix(multiply(Echart, inverse,
                                            static_cast<std::size_t>(qt)), qt);
        for (int j = 0; j < qt; ++j) {
            ++counter.inverse_coefficients;
            counter.require(at_least(Echart[static_cast<std::size_t>(j)], 0),
                            "nonintegral E chart");
            counter.require(at_least(inverse[static_cast<std::size_t>(j)], 0),
                            "nonintegral E inverse");
            counter.require(product[static_cast<std::size_t>(j)] == (j == 0 ? Q(1) : Q(0)),
                            "series inverse identity failed");
        }
        const Q argument = rational(-t, g.alpha);
        weights << id << '\t' << g.a << '\t' << u << '\t' << M << '\t' << p
                << '\t' << t << '\t' << i << '\t' << qt << '\t' << pt << '\t'
                << *ev << '\t' << source_valuation << '\t' << -6L * D * h << "\tPASS\n";
        for (int c = 0; c < coordinates; ++c) {
            const Poly fjet = prefix(shift(f[static_cast<std::size_t>(c)], t, g.alpha), qt);
            const Poly exact = prefix(multiply(fjet, inverse,
                                              static_cast<std::size_t>(qt)), qt);
            const Poly canon_jet = canonical_lift(fjet, pt);
            Poly perturb_jet = canon_jet;
            Poly perturb_quotient = canonical_lift(exact, pt);
            for (int j = 0; j < qt; ++j) {
                const Q delta = perturbation(id, t, j, c, pt);
                perturb_jet[static_cast<std::size_t>(j)] += delta;
                perturb_quotient[static_cast<std::size_t>(j)] += delta;
                counter.require(at_least(perturb_jet[static_cast<std::size_t>(j)]
                                         - fjet[static_cast<std::size_t>(j)], pt),
                                "input lift violates its prescribed precision");
                counter.require(at_least(perturb_quotient[static_cast<std::size_t>(j)]
                                         - exact[static_cast<std::size_t>(j)], pt),
                                "quotient lift violates its prescribed precision");
            }
            const std::array<Poly, 4> quotients = {
                exact,
                prefix(multiply(canon_jet, inverse, static_cast<std::size_t>(qt)), qt),
                prefix(multiply(perturb_jet, inverse, static_cast<std::size_t>(qt)), qt),
                perturb_quotient};
            const Q correct_contribution = Ezero * evaluate(exact, argument);
            for (int variant = 0; variant < 4; ++variant) {
                const Poly& q = quotients[static_cast<std::size_t>(variant)];
                const Q contribution = Ezero * evaluate(q, argument);
                Q direct(0), weight = Ezero;
                for (int j = 0; j < qt; ++j) {
                    direct += weight * q[static_cast<std::size_t>(j)]; weight *= argument;
                }
                counter.require(direct == contribution, "Horner/weight evaluation mismatch");
                const Q error = contribution - correct_contribution;
                counter.require(at_least(error, p), "node error exceeds precision budget");
                counter.inspect_error(error, p);
                reconstructed[static_cast<std::size_t>(variant)][static_cast<std::size_t>(c)]
                    += contribution;
                ++counter.node_error_rows;
                nodes << id << '\t' << t << '\t' << c << '\t'
                      << names[static_cast<std::size_t>(variant)] << '\t' << pt << '\t'
                      << error.get_num() << '\t' << error.get_den() << '\t'
                      << vtext(error) << '\t' << p << "\tPASS\n";
            }
        }
    }
    cases << id << '\t' << g.a << '\t' << u << '\t' << M << '\t' << p << '\t'
          << which << '\t' << degree_bound << '\t' << h << '\t' << total << '\t'
          << active << "\tPASS\n";
    for (int variant = 0; variant < 4; ++variant) for (int c = 0; c < coordinates; ++c) {
        const Q value = reconstructed[static_cast<std::size_t>(variant)][static_cast<std::size_t>(c)];
        const Q error = value - expected[static_cast<std::size_t>(c)];
        if (variant == 0) counter.require(error == 0, "exact D.9 reconstruction failed");
        counter.require(at_least(error, p), "final error exceeds precision budget");
        counter.require(at_least(value, 0), "final reconstructed coordinate not integral");
        const Z actual = residue(value, p), wanted = residue(expected[static_cast<std::size_t>(c)], p);
        counter.require(actual == wanted, "final residue differs from known F(0)");
        counter.inspect_error(error, p);
        ++counter.final_error_rows;
        finals << id << '\t' << c << '\t' << names[static_cast<std::size_t>(variant)]
               << '\t' << error.get_num() << '\t' << error.get_den() << '\t'
               << vtext(error) << '\t' << p << '\t' << value.get_den() << '\t'
               << actual << '\t' << wanted << "\tPASS\n";
    }
}

void negative_controls(std::ofstream& log, Counters& counter) {
    const Grid g(1);
    FVec bad{Poly{rational(1, 2)}, Poly{Q(0)}, Poly{Q(0)}, Poly{Q(0)}};
    counter.require(!hypotheses(g, 1, bad, nullptr), "nonintegral constant accepted");
    ++counter.negative_controls; log << "nonintegral_constant\trejected\tPASS\n";
    bad[0] = Poly{Q(0), rational(1, 2)};
    counter.require(!hypotheses(g, 1, bad, nullptr), "nonintegral odd chart accepted");
    ++counter.negative_controls; log << "nonintegral_odd_chart\trejected\tPASS\n";
    bad[0] = Poly(5, Q(0)); bad[0][4] = 1;
    counter.require(!hypotheses(g, 1, bad, nullptr), "excess degree accepted");
    ++counter.negative_controls; log << "excess_degree\trejected\tPASS\n";
    // Inject into an actual D.9 quotient, then repeat the reconstruction.
    // The closed-form 1/128 below is an independent expected answer.
    Q exact_sum(0), corrupted_sum(0);
    for (const int t : {-1, 1}) {
        const int other = -t;
        const Q Ezero = power(rational(-other, t - other), 3);
        const Poly Echart = prefix(poly_power(
            Poly{Q(1), rational(g.alpha, t - other)}, 3), 3);
        const Poly inverse = inverse_series(Echart, 3);
        const Poly jet = prefix(shift(Poly{Q(1)}, t, g.alpha), 3);
        const Poly quotient = prefix(multiply(jet, inverse, 3), 3);
        Poly corrupted = quotient;
        if (t == -1) corrupted[2] += 1;
        const Q argument = rational(-t, g.alpha);
        exact_sum += Ezero * evaluate(quotient, argument);
        corrupted_sum += Ezero * evaluate(corrupted, argument);
    }
    counter.require(exact_sum == 1, "negative-control exact D.9 reference failed");
    const Q error = corrupted_sum - exact_sum;
    counter.require(error == rational(1, 128) && valuation(error) == std::optional<long>(-7),
                    "low-precision control arithmetic mismatch");
    counter.require(!at_least(error, 1) && !at_least(error, 0),
                    "low-precision control was not rejected");
    ++counter.negative_controls;
    log << "invalid_unit_quotient_perturbation\tactual_D9_injection;1/128;v2=-7;rejected\tPASS\n";
}

int main(int argc, char** argv) {
    try {
        if (argc != 2) throw std::runtime_error("usage: hermite_unit OUTPUT_DIRECTORY");
        const std::filesystem::path out(argv[1]); std::filesystem::create_directories(out);
        std::ofstream cases(out / "cases.tsv"), kernels(out / "kernels.tsv"),
            weights(out / "weights.tsv"), nodes(out / "node-errors.tsv"),
            finals(out / "final-errors.tsv"), negative(out / "negative.tsv");
        if (!cases || !kernels || !weights || !nodes || !finals || !negative)
            throw std::runtime_error("cannot open output file");
        cases << "case\ta\tu\tM\tp\tfamily\tdegree_bound\th\ttotal_multiplicity\tactive_nodes\tstatus\n";
        kernels << "a\tdelta\tt\tcoefficients\tmin_v2\trequired_v2\tstatus\n";
        weights << "case\ta\tu\tM\tp\tt\ti\tq_t\tp_t\tv2_E0\tD10_value\tD10_lower_bound\tstatus\n";
        nodes << "case\tt\tcoordinate\tvariant\tp_t\terror_num\terror_den\terror_v2\trequired_p\tstatus\n";
        finals << "case\tcoordinate\tvariant\terror_num\terror_den\terror_v2\trequired_p\tvalue_den\tactual_residue\texpected_residue\tstatus\n";
        negative << "control\tobservation\tstatus\n";
        Counters counter;
        for (int a = 1; a <= 3; ++a) {
            const Grid g(a); kernel_checks(g, kernels, counter);
            for (int u = 1; u <= a; ++u) for (const int M : {u, u + 1}) {
                ++counter.parameter_tuples;
                for (const int p : {1, 3, 5}) for (int which = 0; which < 2; ++which)
                    run_case(g, u, M, p, which, cases, weights, nodes, finals, counter);
            }
        }
        negative_controls(negative, counter);
        counter.require(counter.cases == 72 && counter.final_error_rows == 1152,
                        "fixed panel count mismatch");
        std::ofstream summary(out / "summary.json");
        summary << "{\n  \"schema\": \"exact-hermite-lemma-development/v1\",\n"
                << "  \"parameter_tuples\": " << counter.parameter_tuples
                << ",\n  \"polynomial_cases\": " << counter.cases
                << ",\n  \"variants\": 4,\n  \"coordinates\": 4,\n  \"kernel_charts\": " << counter.kernel_charts
                << ",\n  \"kernel_coefficients\": " << counter.kernel_coefficients
                << ",\n  \"derivative_grid_checks\": " << counter.derivative_grid_checks
                << ",\n  \"nonintegral_global_kernel_coefficients\": " << counter.nonintegral_global_kernel_coefficients
                << ",\n  \"hypothesis_coefficients\": " << counter.hypothesis_coefficients
                << ",\n  \"node_instances\": " << counter.node_instances
                << ",\n  \"inverse_coefficients\": " << counter.inverse_coefficients
                << ",\n  \"node_error_rows\": " << counter.node_error_rows
                << ",\n  \"final_error_rows\": " << counter.final_error_rows
                << ",\n  \"finite_error_rows\": " << counter.finite_error_rows
                << ",\n  \"minimum_error_margin_above_p\": "
                << (counter.minimum_error_margin ? std::to_string(*counter.minimum_error_margin) : "null")
                << ",\n  \"negative_controls\": " << counter.negative_controls
                << ",\n  \"assertions\": " << counter.assertions
                << ",\n  \"scientific_timing\": false,\n  \"status\": \"PASS\"\n}\n";
        std::cout << "PASS fixed tapered-Hermite cases=" << counter.cases
                  << " variants=4 coordinates=4 kernel_charts=" << counter.kernel_charts
                  << " node_errors=" << counter.node_error_rows
                  << " final_errors=" << counter.final_error_rows
                  << " negative_controls=" << counter.negative_controls << '\n';
        std::cout << "PASS exact D.9 identity; all perturbed coordinate errors v2>=p; no timing or theorem claim\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "FAIL " << error.what() << '\n'; return 1;
    }
}
