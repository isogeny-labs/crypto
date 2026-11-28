// Exact finite arithmetic for the *proposed* D_1024 seven-sum coverage lemma.
//
// This program evaluates formulas adapted from Kaarelson v2, Section 5. It
// neither proves that adaptation nor constructs a target-decomposition oracle.
// The P-61 identity is icv1-fp61-tm813574577-a3b71255. B and D are inputs
// from an independently replayed factor-base and pair-spectrum census.
//
// Build: g++ -std=c++20 -O2 -Wall -Wextra -Werror p61_bounds.cpp
//        -lgmpxx -lgmp -o p61_bounds
// Run:   ./p61_bounds 486 236196

#include <gmpxx.h>

#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <numbers>
#include <stdexcept>
#include <string>

namespace {

mpz_class power(unsigned long base, unsigned long exponent) {
    mpz_class result;
    mpz_ui_pow_ui(result.get_mpz_t(), base, exponent);
    return result;
}

mpz_class odd_double_factorial(unsigned long n) {
    mpz_class result = 1;
    for (unsigned long j = 1; j <= n; j += 2) result *= j;
    return result;
}

mpz_class ceil_positive(const mpq_class& fraction) {
    if (fraction < 0) throw std::logic_error("expected a positive fraction");
    mpz_class result;
    mpz_cdiv_q(result.get_mpz_t(), fraction.get_num().get_mpz_t(),
               fraction.get_den().get_mpz_t());
    return result;
}

void emit_fraction(const std::string& key, const mpq_class& value) {
    mpq_class normalized = value;
    normalized.canonicalize();
    std::cout << "  \"" << key << "\": {\"numerator\": \""
              << normalized.get_num() << "\", \"denominator\": \""
              << normalized.get_den() << "\", \"approx\": "
              << std::setprecision(17) << normalized.get_d() << "},\n";
}

unsigned long parse_positive(const char* text, const char* name) {
    const std::string source(text);
    if (source.empty() || source.find_first_not_of("0123456789") != std::string::npos)
        throw std::invalid_argument(std::string("invalid ") + name);
    const unsigned long value = std::stoul(source);
    if (value == 0) throw std::invalid_argument(std::string("zero ") + name);
    return value;
}

}  // namespace

int main(int argc, char** argv) {
    try {
        if (argc != 3) {
            std::cerr << "usage: p61_bounds <lifted-abscissa-count B> <pair-x-count D>\n";
            return 2;
        }
        const unsigned long B = parse_positive(argv[1], "B");
        const unsigned long D = parse_positive(argv[2], "D");
        if (B != 486 || D != B * B)
            throw std::invalid_argument("this frozen P-61 run requires B=486 and D=B^2");

        const mpz_class p("2305843009213693951");
        const mpz_class r("2305843010027268529");
        constexpr unsigned long q = 1024;
        constexpr unsigned long m = 7;
        constexpr unsigned long kappa = 4;
        mpz_class sqrt_floor;
        mpz_sqrt(sqrt_floor.get_mpz_t(), p.get_mpz_t());
        const mpz_class sqrt_upper = sqrt_floor + 1;
        if (mpz_class(q) > sqrt_floor || q < 12 * m || (p + 1) % (4 * q) != 0)
            throw std::logic_error("selector parameter preconditions failed");

        const mpz_class theta = odd_double_factorial(2 * m - 1) * power(4, m);
        const mpz_class degree = (2 * m - 1) * power(2, 2 * m - 2);
        const mpq_class mu = mpq_class(theta * power(q, m))
            + mpq_class(2 * power(4, m) * degree * power(q, 2 * m), p - 1);
        const mpq_class variance = mpq_class(q)
            + mpq_class(mpz_class(q) * (q - 1) * (11 * sqrt_upper + 5), p - 1);
        const mpq_class small_base = 4 * variance / power(q, 2);
        const mpq_class good_model = 1 - mpq_class(1, kappa) - small_base;
        const mpq_class coverage = mpq_class(power(q / 2, 2 * m))
            / (4 * kappa * mu * mpq_class(r));
        if (good_model <= 0 || coverage <= 0)
            throw std::logic_error("coverage preconditions failed");

        const mpz_class covered_targets = ceil_positive(coverage * r);
        const unsigned long signed_base = 2 * B;
        const unsigned long two_sum = 2 * D + 1;
        const mpz_class additive_lower = 3 * (mpz_class(two_sum) - 1) + signed_base;
        const mpz_class fixed_seven_lower = additive_lower < r ? additive_lower : r;
        const mpq_class fixed_coverage = mpq_class(fixed_seven_lower, r);
        mpz_class multisets;
        mpz_bin_uiui(multisets.get_mpz_t(), signed_base + m - 1, m);
        const mpq_class representations = mpq_class(multisets, r - 1);
        const mpq_class calls = mpq_class(B + 1) / representations;
        const mpq_class pair_inspections = calls * D;
        mpq_class proof_round_cap = mpq_class(2 * (q + 1)) / coverage;
        proof_round_cap.canonicalize();
        const long double cube_root_reference = std::cbrt(p.get_d());
        const long double q_four_thirds_reference = std::pow(q, 4.0L / 3.0L);
        const long double ideal_negation_rho_reference =
            std::sqrt(std::numbers::pi_v<long double> * r.get_d() / 4.0L);

        std::cout << "{\n"
                  << "  \"standing\": \"exact arithmetic for a proposed unrefereed coverage adaptation\",\n"
                  << "  \"curve_id\": \"icv1-fp61-tm813574577-a3b71255\",\n"
                  << "  \"p\": \"" << p << "\",\n"
                  << "  \"group_order\": \"" << r << "\",\n"
                  << "  \"selector_size_q\": " << q << ",\n"
                  << "  \"arity_m\": " << m << ",\n"
                  << "  \"markov_multiplier_kappa\": " << kappa << ",\n"
                  << "  \"theta_m\": " << theta << ",\n"
                  << "  \"semaev_degree_bound_D_prime\": " << degree << ",\n";
        emit_fraction("average_m_fold_energy_upper_bound", mu);
        emit_fraction("factor_base_size_variance_upper_bound", variance);
        emit_fraction("small_base_probability_upper_bound", small_base);
        emit_fraction("good_model_probability_lower_bound", good_model);
        emit_fraction("generic_7_sum_coverage_probability_lower_bound_on_good_model", coverage);
        std::cout << "  \"generic_covered_targets_at_least\": " << covered_targets << ",\n"
                  << "  \"fixed_model_lifted_abscissae_B\": " << B << ",\n"
                  << "  \"fixed_model_pair_abscissae_D\": " << D << ",\n"
                  << "  \"fixed_model_signed_base_size\": " << signed_base << ",\n"
                  << "  \"fixed_model_two_sum_size_exact\": " << two_sum << ",\n"
                  << "  \"fixed_model_unrestricted_seven_sum_size_lower\": "
                  << fixed_seven_lower << ",\n";
        emit_fraction("fixed_model_uniform_target_unrestricted_coverage_lower_bound",
                      fixed_coverage);
        emit_fraction("fixed_model_expected_representations_per_uniform_nonidentity_target_upper",
                      representations);
        emit_fraction("fixed_model_expected_uniform_queries_lower_for_B_plus_1_outputs",
                      calls);
        emit_fraction("fixed_model_full_pair_scan_entries_lower_for_B_plus_1_outputs",
                      pair_inspections);
        // No trailing comma on the final JSON field.
        std::cout << "  \"random_good_model_round_query_cap_for_at_most_q_columns\": {"
                  << "\"numerator\": \"" << proof_round_cap.get_num()
                  << "\", \"denominator\": \"" << proof_round_cap.get_den()
                  << "\", \"approx\": " << std::setprecision(17)
                  << proof_round_cap.get_d() << "},\n"
                  << "  \"p_to_one_third_operation_count_reference\": "
                  << cube_root_reference << ",\n"
                  << "  \"q_to_four_thirds_operation_count_reference\": "
                  << q_four_thirds_reference << ",\n"
                  << "  \"ideal_negation_rho_random_walk_group_step_reference\": "
                  << ideal_negation_rho_reference << ",\n"
                  << "  \"rho_step_equivalent_budget_per_query_at_query_lower_bound\": "
                  << ideal_negation_rho_reference / calls.get_d() << "\n} \n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "p61_bounds: " << error.what() << '\n';
        return 1;
    }
}
