#include "order_common.hpp"
#include <iostream>

// This checker never reduces an order or calls znorder. Direct powers certify minimality.
static bool exact_degree(GEN p, GEN n, GEN k, const FactorList& factors) {
    if (signe(k) <= 0 || !isprime(n) || !equali1(gcdii(p, n))) return false;
    if (signe(modii(subis(n, 1), k)) || !equali1(Fp_pow(p, k, n))) return false;
    for (const auto& [q, ignored] : factors) {
        (void)ignored;
        if (!signe(modii(k, q)) && equali1(Fp_pow(p, diviiexact(k, q), n))) return false;
    }
    return true;
}

static void self_test() {
    const FactorList six = {{stoi(2), 1}, {stoi(3), 1}}, sixteen = {{stoi(2), 4}};
    require(exact_degree(stoi(3), stoi(7), stoi(6), six), "maximal certificate rejected");
    require(exact_degree(stoi(2), stoi(7), stoi(3), six), "nonmaximal certificate rejected");
    require(exact_degree(stoi(4), stoi(17), stoi(4), sixteen), "power-of-two certificate rejected");
    require(!exact_degree(stoi(4), stoi(17), stoi(8), sixteen), "nonminimal degree accepted");
    require(!exact_degree(stoi(4), stoi(17), stoi(2), sixteen), "incorrect exponent accepted");
    require(!exact_degree(stoi(1), stoi(17), stoi(16), sixteen), "wrong trivial-base order accepted");
    std::cout << "Three positive and three rejecting certificate controls passed.\n";
}

int main(int argc, char** argv) {
  try {
    pari_init(256000000, 2000000);
    if (argc == 2 && std::string(argv[1]) == "--self-test") {
        self_test(); pari_close(); return 0;
    }
    require(argc == 5, "usage: verify historical.tsv minus_record_dir certificate_dir output.json");
    const auto rows = read_rows(argv[1]);
    for (size_t i = 0; i < rows.size(); ++i) {
        const pari_sp saved = avma;
        const auto& row = rows[i];
        GEN p = integer(row.fields[5]), n = integer(row.fields[7]);
        require(isprime(p) && isprime(n) && !equalii(p, n), "invalid original prime inputs");
        const auto factors = read_minus(std::string(argv[2]) + "/curve-" + std::to_string(i) + "-minus.json", row, n);
        const std::string certificate = read_text(std::string(argv[3]) + "/curve-" + std::to_string(i) + ".json");
        require(string_field(certificate, "label") == row.fields[0] &&
                string_field(certificate, "role") == row.fields[2], "certificate identity mismatch");
        require(equalii(integer(string_field(certificate, "p")), p) &&
                equalii(integer(string_field(certificate, "n")), n), "certificate original input mismatch");
        GEN k = integer(string_field(certificate, "degree"));
        require(exact_degree(p, n, k, factors), "degree certificate failed: " + row.fields[0]);
        require(equalii(integer(string_field(certificate, "index")), diviiexact(subis(n, 1), k)), "index mismatch");
        avma = saved;
    }
    std::ofstream out(argv[4]); require(bool(out), "cannot write verification receipt");
    out << "{\"kind\":\"native-direct-power-degree-verification/v1\",\"rows\":21,\"passed\":21,"
           "\"all_original_primes_and_minus_products_rechecked\":true,"
           "\"all_minimality_certificates_valid\":true,\"znorder_called_by_checker\":false}\n";
    std::cout << "All 21 exact minimal degrees verified by direct modular-power certificates.\n";
    pari_close(); return 0;
  } catch (const std::exception& error) {
    std::cerr << "ERROR: " << error.what() << '\n'; return 1;
  }
}
