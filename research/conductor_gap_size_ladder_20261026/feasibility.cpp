// Parameter certificates and planning arithmetic only. No ECDLP solver.
#include <gmpxx.h>
#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>

using U = uint64_t;
using Wide = __uint128_t;
using Z = mpz_class;
using Clock = std::chrono::steady_clock;
void require(bool ok, const std::string& message) {
    if (!ok) throw std::runtime_error(message);
}
U multiply(U a, U b, U n) { return Wide(a) * b % n; }
U power(U a, U e, U n) {
    U value = 1;
    for (; e; e >>= 1, a = multiply(a, a, n))
        if (e & 1) value = multiply(value, a, n);
    return value;
}
bool prime64(U n) {
    if (n < 2) return false;
    for (U p : {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37})
        if (n % p == 0) return n == p;
    U odd = n - 1;
    unsigned shifts = 0;
    while (!(odd & 1)) { odd >>= 1; ++shifts; }
    // Deterministic for all unsigned 64-bit integers.
    for (U a : {2ULL, 325ULL, 9375ULL, 28178ULL, 450775ULL,
                9780504ULL, 1795265022ULL}) {
        if (a % n == 0) continue;
        U x = power(a % n, odd, n);
        if (x == 1 || x == n - 1) continue;
        bool passed = false;
        for (unsigned i = 1; i < shifts; ++i) {
            x = multiply(x, x, n);
            if (x == n - 1) { passed = true; break; }
        }
        if (!passed) return false;
    }
    return true;
}
U root(Wide value) {
    U low = 0, high = UINT64_MAX;
    while (low < high) {
        U mid = low + (high - low) / 2 + 1;
        if (Wide(mid) * mid <= value) low = mid;
        else high = mid - 1;
    }
    return low;
}
unsigned bits(U n) { return n ? 64 - __builtin_clzll(n) : 0; }
Z zp(const Z& a, const Z& e, const Z& n) {
    Z result;
    mpz_powm(result.get_mpz_t(), a.get_mpz_t(), e.get_mpz_t(), n.get_mpz_t());
    return result;
}
Z gcd(const Z& a, const Z& b) {
    Z result;
    mpz_gcd(result.get_mpz_t(), a.get_mpz_t(), b.get_mpz_t());
    return result;
}
// q is already proved prime; F=q^exponent is the known factor of n-1.
unsigned witness(const Z& n, const Z& q, unsigned exponent) {
    Z factor;
    mpz_pow_ui(factor.get_mpz_t(), q.get_mpz_t(), exponent);
    if (n < 3 || (n - 1) % factor != 0 || factor * factor <= n) return 0;
    for (unsigned a = 2; a <= 128; ++a)
        if (zp(Z(a), n - 1, n) == 1 && gcd(zp(Z(a), (n - 1) / q, n) - 1, n) == 1)
            return a;
    return 0;
}
double phi(double z) { return std::erfc(-z / std::sqrt(2.0)) / 2; }
double normal_quantile(double probability) {
    double low = -12, high = 12;
    for (int i = 0; i < 100; ++i) {
        double mid = (low + high) / 2;
        if (phi(mid) < probability) low = mid; else high = mid;
    }
    return (low + high) / 2;
}
double equivalence_power(double ratio, double se, double critical) {
    double low = std::log(.95) + critical * se;
    double high = std::log(1.05) - critical * se;
    if (low >= high) return 0;
    return std::max(0.0, phi((high - std::log(ratio)) / se) -
                          phi((low - std::log(ratio)) / se));
}
double number(const std::string& input, const std::string& key) {
    std::regex pattern("\"" + key + "\"\\s*:\\s*([-+0-9.eE]+)");
    std::sregex_iterator begin(input.begin(), input.end(), pattern), end;
    require(begin != end, "missing model field " + key);
    double value = std::stod((*begin)[1]);
    require(++begin == end && std::isfinite(value) && value >= 0, "ambiguous model field " + key);
    return value;
}
struct Candidate {
    std::string kind;
    unsigned field_bits, d, depth, h, ell_exponent, ell_witness, p_witness, r_witness;
    Z p, t, ell, r;
};
void emit(std::ofstream& csv, const Candidate& c) {
    csv << c.kind << ',' << c.field_bits << ',' << c.d << ',' << c.depth
        << ',' << c.p << ',' << c.t << ',' << c.ell << ',' << c.r << ',' << c.h
        << ',' << c.ell_exponent << ',' << c.ell_witness << ',' << c.p_witness
        << ',' << c.r_witness << '\n';
    csv.flush();
    require(bool(csv), "candidate write failed");
}
void controls() {
    for (U n : {2ULL, 3ULL, 97ULL, 2147483647ULL, 2305843009213693951ULL,
                18446744073709551557ULL}) require(prime64(n), "prime control");
    for (U n : {0ULL, 1ULL, 4ULL, 561ULL, 1105ULL, 3215031751ULL,
                341550071728321ULL, 3825123056546413051ULL, 18446744073709551615ULL})
        require(!prime64(n), "pseudoprime control");
    require(root(Wide(UINT64_MAX) * UINT64_MAX) == UINT64_MAX &&
            root(Wide(1) << 120) == (U(1) << 60) && root(80) == 8, "integer-root control");
    require(witness(Z(97), Z(2), 5) != 0, "Pocklington positive control");
    require(witness(Z(91), Z(3), 2) == 0 && witness(Z(97), Z(2), 2) == 0,
            "Pocklington composite/insufficient-factor controls");
    require(witness(Z(15), Z(7), 1) == 0, "composite with sufficiently large known factor");
    for (unsigned residue = 1; residue < 5; ++residue)
        require((1216 * residue * residue + 1) % 5 == 0 ||
                (304 * residue * residue + 1) % 5 == 0, "D19 upper-family obstruction");
    double z = normal_quantile(1 - .01 / 4 / 2);
    require(std::abs(z - 3.023341439739154) < 1e-10, "multiplicity critical value");
    require(equivalence_power(1, .005, z) > .999 &&
            equivalence_power(1.1, .005, z) < 1e-9, "power controls");
    require(equivalence_power(1, .03, z) == 0, "empty acceptance interval");
    std::cout << "Native primality, Pocklington, integer bounds and planning controls passed\n";
}
int main(int argc, char** argv) {
    try {
        if (argc == 2 && std::string(argv[1]) == "--self-test") { controls(); return 0; }
        bool completion = argc == 5 && std::string(argv[1]) == "--complete-v2";
        require(argc == 3 || completion, "usage: size-plan MODEL.json NEW_RESULTS_DIR; or --complete-v2 MODEL.json OLD_CSV NEW_DIR");
        std::string model_path = argv[completion ? 2 : 1];
        std::ifstream model(model_path);
        require(bool(model), "planning model unavailable");
        std::ostringstream buffer;
        buffer << model.rdbuf();
        double tau = number(buffer.str(), "truncated_between_class_variance");
        double seed = number(buffer.str(), "average_seed_component");
        double target = number(buffer.str(), "average_target_component_truncated");
        std::filesystem::path out(argv[completion ? 4 : 2]);
        require(std::filesystem::create_directory(out), "results path must not exist");
        std::ofstream csv(out / "candidates.csv");
        csv << "kind,field_bits,abs_d,depth,p,t,ell,r,cofactor,ell_factor_exponent,ell_witness,p_witness,r_witness\n";
        auto started = Clock::now();
        std::vector<Candidate> candidates;
        struct Cell { std::string kind; unsigned bits, d, accepted; U examined; bool complete; std::string stop; };
        std::vector<Cell> cells;
        auto expired = [&] { return Clock::now() - started > std::chrono::seconds(900); };
        if (completion) {
            std::ifstream retained(argv[3]);
            require(bool(retained), "v1 CSV unavailable");
            std::string line;
            std::getline(retained, line);
            require(line == "kind,field_bits,abs_d,depth,p,t,ell,r,cofactor,ell_factor_exponent,ell_witness,p_witness,r_witness", "v1 CSV schema");
            while (std::getline(retained, line)) {
                std::istringstream row(line);
                std::vector<std::string> fields;
                std::string token;
                while (std::getline(row, token, ',')) fields.push_back(token);
                require(fields.size() == 13, "v1 CSV columns");
                Candidate c{fields[0], unsigned(std::stoul(fields[1])), unsigned(std::stoul(fields[2])),
                    unsigned(std::stoul(fields[3])), unsigned(std::stoul(fields[8])),
                    unsigned(std::stoul(fields[9])), unsigned(std::stoul(fields[10])),
                    unsigned(std::stoul(fields[11])), unsigned(std::stoul(fields[12])),
                    Z(fields[4]), Z(fields[5]), Z(fields[6]), Z(fields[7])};
                require(c.kind == "native" || (c.kind == "upper" && c.d == 43), "unexpected v1 family");
                candidates.push_back(c); emit(csv, c);
            }
            require(candidates.size() == 131, "v1 retained count");
            for (unsigned b : {40, 47, 54, 61}) for (unsigned d : {11, 19, 43, 67}) {
                unsigned n = std::count_if(candidates.begin(), candidates.end(), [&](const Candidate& c) {
                    return c.kind == "native" && c.field_bits == b && c.d == d;
                });
                require(n == 8, "v1 native cell count");
                cells.push_back({"native", b, d, n, 0, true, "reused-v1-certificate"});
            }
            for (unsigned b : {112, 160, 256}) {
                require(std::count_if(candidates.begin(), candidates.end(), [&](const Candidate& c) {
                    return c.kind == "upper" && c.field_bits == b && c.d == 43;
                }) == 1, "v1 upper cell count");
                cells.push_back({"upper", b, 43, 1, 0, true, "reused-v1-certificate"});
            }
        }
        if (!completion) for (unsigned b : {40, 47, 54, 61}) for (unsigned d : {11, 19, 43, 67}) {
            Wide low = Wide(1) << (b - 1), high = Wide(1) << b, c = d * 64;
            U ell = root((high - 1) / c), examined = 0;
            unsigned accepted = 0;
            for (; ell > 3 && examined < 500000 && accepted < 8 && !expired(); --ell) {
                ++examined;
                if (!prime64(ell) || ell == d) continue;
                Wide base = c * ell * ell;
                int64_t bound = root(high - 1 - base), mmax = bound / ell + 2;
                std::vector<int64_t> traces;
                for (int64_t m = -mmax; m <= mmax; ++m) {
                    int64_t tp = m * int64_t(ell) - 1;
                    if (std::abs(tp) <= bound && tp % 2 != 0) traces.push_back(tp);
                }
                std::sort(traces.begin(), traces.end(), [](int64_t a, int64_t z) {
                    return std::make_pair(std::abs(a), a) < std::make_pair(std::abs(z), z);
                });
                bool found = false;
                for (int64_t tp : traces) {
                    Wide pwide = Wide(std::abs(tp)) * std::abs(tp) + base;
                    if (pwide < low || !prime64(U(pwide))) continue;
                    U p = pwide, order = U(__int128_t(p) + 1 - 2 * tp);
                    for (unsigned h = 1; h <= 64; ++h) {
                        if (order % h) continue;
                        U r = order / h;
                        if (!prime64(r) || bits(r) < b - 6 || r == ell ||
                            Wide(r) * r <= 16 * Wide(p) || Wide(ell) * ell * ell * ell <= p ||
                            std::gcd(U(std::abs(2 * tp)), p) != 1) continue;
                        Candidate row{"native", b, d, 4, h, 0, 0, 0, 0,
                                      Z(p), Z(2 * tp), Z(ell), Z(r)};
                        emit(csv, row); candidates.push_back(row); ++accepted; found = true; break;
                    }
                    if (found) break;
                }
            }
            cells.push_back({"native", b, d, accepted, examined, accepted == 8,
                accepted == 8 ? "complete" : expired() ? "total-time-guard" :
                examined >= 500000 ? "candidate-guard" : "ell-range-exhausted"});
            std::cerr << "native bits=" << b << " d=" << d << " accepted=" << accepted
                      << " candidates=" << examined << '\n';
        }
        std::vector<unsigned> upper_d = completion ? std::vector<unsigned>{67} : std::vector<unsigned>{19, 43};
        for (unsigned b : {112, 160, 256}) for (unsigned d : upper_d) {
            Z high = Z(1) << b, low = high / 2, quotient = (high - 2) / (d * 64);
            Z ell_max;
            mpz_sqrt(ell_max.get_mpz_t(), quotient.get_mpz_t());
            unsigned exponent = (mpz_sizeinbase(ell_max.get_mpz_t(), 2) + 1) / 2;
            Z step = Z(1) << exponent, k = (ell_max - 1) / step;
            U examined = 0;
            unsigned accepted = 0;
            auto cell_started = Clock::now();
            for (; k > 0 && examined < 3000000 && !expired() &&
                   Clock::now() - cell_started < std::chrono::seconds(120); --k) {
                ++examined;
                Z ell = k * step + 1;
                if (mpz_probab_prime_p(ell.get_mpz_t(), 30) == 0) continue;
                Z p = d * 64 * ell * ell + 1, r = d * 16 * ell * ell + 1;
                if (p < low) break;
                if (mpz_probab_prime_p(p.get_mpz_t(), 30) == 0 ||
                    mpz_probab_prime_p(r.get_mpz_t(), 30) == 0) continue;
                unsigned ae = witness(ell, Z(2), exponent), ap = witness(p, ell, 2), ar = witness(r, ell, 2);
                if (!ae || !ap || !ar) continue;
                require(mpz_sizeinbase(r.get_mpz_t(), 2) == b - 2, "upper subgroup size");
                Candidate row{"upper", b, d, 4, 4, exponent, ae, ap, ar, p, Z(-2), ell, r};
                emit(csv, row); candidates.push_back(row); accepted = 1; break;
            }
            cells.push_back({"upper", b, d, accepted, examined, accepted == 1,
                accepted == 1 ? "complete" : expired() ? "total-time-guard" :
                examined >= 3000000 ? "candidate-guard" :
                Clock::now() - cell_started >= std::chrono::seconds(120) ? "cell-time-guard" : "field-range-exhausted"});
            std::cerr << "upper bits=" << b << " d=" << d << " accepted=" << accepted
                      << " candidates=" << examined << '\n';
        }
        std::ofstream json(out / "design.json");
        json << std::setprecision(17)
             << "{\n\"schema\":\"conductor-gap.size-ladder/v1\",\"solver_measurements_added\":0,"
             << "\"curve_models_constructed\":0,\"source_model\":\"" << model_path << "\",\n\"cells\":[";
        for (size_t i = 0; i < cells.size(); ++i) {
            auto& c = cells[i]; if (i) json << ',';
            json << "{\"kind\":\"" << c.kind << "\",\"field_bits\":" << c.bits
                 << ",\"abs_d\":" << c.d << ",\"accepted\":" << c.accepted
                 << ",\"candidates_examined\":" << c.examined << ",\"complete\":"
                 << (c.complete ? "true" : "false") << ",\"stop_reason\":\"" << c.stop << "\"}";
        }
        json << "],\n\"candidates\":[";
        for (size_t i = 0; i < candidates.size(); ++i) {
            auto& c = candidates[i]; if (i) json << ',';
            json << "{\"kind\":\"" << c.kind << "\",\"field_bits\":" << c.field_bits
                 << ",\"abs_d\":" << c.d << ",\"depth\":4,\"p\":\"" << c.p
                 << "\",\"t\":\"" << c.t << "\",\"ell\":\"" << c.ell << "\",\"r\":\"" << c.r
                 << "\",\"subgroup_bits\":" << mpz_sizeinbase(c.r.get_mpz_t(), 2)
                 << ",\"cofactor\":" << c.h << ",\"generic_rho_exponent\":"
                 << std::log2(c.r.get_d()) / 2 << '}';
        }
        json << "],\n\"size_ranges\":[";
        bool first_range = true;
        double predicted_primary = 0;
        for (unsigned b : {40, 47, 54, 61, 112, 160, 256}) {
            Z min_gap = 0, max_gap = 0;
            unsigned min_r = 1000, max_r = 0, count = 0;
            for (const auto& c : candidates) if (c.field_bits == b) {
                if (!count || c.ell < min_gap) min_gap = c.ell;
                if (!count || c.ell > max_gap) max_gap = c.ell;
                unsigned rbits = mpz_sizeinbase(c.r.get_mpz_t(), 2);
                min_r = std::min(min_r, rbits); max_r = std::max(max_r, rbits); ++count;
                if (c.kind == "native") predicted_primary += 8000 * std::sqrt(std::acos(-1.) * c.r.get_d() / 4);
            }
            if (!count) continue;
            if (!first_range) json << ',';
            first_range = false;
            json << "{\"field_bits\":" << b << ",\"parameter_classes\":" << count
                 << ",\"min_gap\":\"" << min_gap << "\",\"max_gap\":\"" << max_gap
                 << "\",\"min_subgroup_bits\":" << min_r << ",\"max_subgroup_bits\":" << max_r << '}';
        }
        double critical = normal_quantile(1 - .01 / 4 / 2);
        json << "],\n\"execution_projection\":{\"primary_calls\":1024000,\"separate_full_replay_calls\":1024000,"
             << "\"predicted_primary_rho_group_additions\":" << predicted_primary
             << ",\"predicted_with_full_replay_group_additions\":" << 2 * predicted_primary
             << ",\"status\":\"Generic-negation sqrt(pi*r/4) model, not measured runtime or complete cost; excludes controls and setup. No campaign launched.\"},"
             << "\n\"planning\":{\"nominal_family_confidence\":0.99,\"rung_interval_confidence\":0.9975,"
             << "\"normal_critical\":" << critical << ",\"tau\":" << tau
             << ",\"seed_component\":" << seed << ",\"target_component\":" << target
             << ",\"warning\":\"Old small-group variance model, unvalidated at new sizes; approximate normal/delta power only.\"},\n\"power_scenarios\":[";
        bool first = true;
        for (unsigned classes : {16, 32, 64, 128}) for (unsigned targets : {100, 1000})
            for (unsigned inflation : {1, 2, 4}) for (double ratio : {.99, 1., 1.01, 1.0111251967610384, 1.03}) {
                double se = std::sqrt(inflation * (tau / classes + (target + seed / 2) / (classes * targets)));
                double prob = equivalence_power(ratio, se, critical);
                if (!first) json << ',';
                first = false;
                json << "{\"classes_per_rung\":" << classes << ",\"targets_per_crater\":" << targets
                     << ",\"seeds_per_target\":2,\"variance_inflation\":" << inflation
                     << ",\"true_ratio_assumption\":" << ratio << ",\"se_log_ratio\":" << se
                     << ",\"per_rung_power\":" << prob << ",\"joint_power_union_lower_bound\":"
                     << std::max(0., 1 - 4 * (1 - prob)) << ",\"total_primary_solves\":"
                     << U(classes) * 4 * targets * 2 * 4 << '}';
            }
        json << "]\n}\n";
        require(bool(json), "JSON write failed");
        bool complete = std::all_of(cells.begin(), cells.end(), [](const Cell& c) { return c.complete; });
        std::cout << "Parameter candidates retained: " << candidates.size()
                  << "; complete cells: " << std::count_if(cells.begin(), cells.end(), [](const Cell& c) { return c.complete; })
                  << '/' << cells.size() << "; ECDLP solves: 0; explicit models: 0\n";
        return complete ? 0 : 3;
    } catch (const std::exception& e) { std::cerr << e.what() << '\n'; return 2; }
}
