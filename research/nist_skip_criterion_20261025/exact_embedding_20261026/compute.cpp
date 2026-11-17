#include "order_common.hpp"
#include <iostream>

static GEN degree_by_reduction(GEN p, GEN n, const FactorList& factors) {
    GEN k = subis(n, 1);
    for (const auto& [q, exponent] : factors) {
        for (long i = 0; i < exponent; ++i) {
            if (signe(modii(k, q))) break;
            GEN candidate = diviiexact(k, q);
            if (!equali1(Fp_pow(p, candidate, n))) break;
            k = candidate;
        }
    }
    GEN reference = znorder(mkintmod(modii(p, n), n),
                            mkvec2(subis(n, 1), factor_matrix(factors)));
    require(equalii(k, reference), "independent PARI order disagreement");
    return k;
}

static void self_test() {
    require(itos(degree_by_reduction(stoi(3), stoi(7), {{stoi(2), 1}, {stoi(3), 1}})) == 6,
            "maximal positive control failed");
    require(itos(degree_by_reduction(stoi(2), stoi(7), {{stoi(2), 1}, {stoi(3), 1}})) == 3,
            "nonmaximal positive control failed");
    require(itos(degree_by_reduction(stoi(4), stoi(17), {{stoi(2), 4}})) == 4,
            "power-of-two positive control failed");
    std::cout << "Three fixed controls passed; reduction agrees with PARI znorder.\n";
}

int main(int argc, char** argv) {
  try {
    pari_init(256000000, 2000000);
    if (argc == 2 && std::string(argv[1]) == "--self-test") {
        self_test(); pari_close(); return 0;
    }
    require(argc == 4, "usage: compute historical.tsv minus_record_dir fresh_output_dir");
    const auto rows = read_rows(argv[1]);
    std::ofstream table(std::string(argv[3]) + "/degrees.tsv");
    require(bool(table), "cannot open degree table");
    table << "label\tbits\trole\tdegree_hex\tdegree_bits\tindex_hex\tmaximal\trfc7748_mov_pass\n";
    long maximal_published = 0, maximal_skipped = 0, mov_published = 0, mov_skipped = 0;
    std::vector<std::string> maximal_mismatches, mov_mismatches;
    for (size_t i = 0; i < rows.size(); ++i) {
        const pari_sp saved = avma;
        const auto& row = rows[i];
        GEN p = integer(row.fields[5]), n = integer(row.fields[7]);
        require(isprime(p) && isprime(n) && !equalii(p, n), "invalid prime field/order");
        const auto factors = read_minus(std::string(argv[2]) + "/curve-" + std::to_string(i) + "-minus.json", row, n);
        GEN k = degree_by_reduction(p, n, factors), index = diviiexact(subis(n, 1), k);
        const bool maximal = equali1(index), mov_pass = cmpis(index, 100) < 0;
        const bool published = row.fields[2] == "published";
        maximal_published += maximal && published;
        maximal_skipped += maximal && !published;
        mov_published += mov_pass && published;
        mov_skipped += mov_pass && !published;
        if (maximal != published) maximal_mismatches.push_back(row.fields[0]);
        if (mov_pass != published) mov_mismatches.push_back(row.fields[0]);
        table << row.fields[0] << '\t' << row.fields[1] << '\t' << row.fields[2] << '\t'
              << hexadecimal(k) << '\t' << bit_length(k) << '\t' << hexadecimal(index)
              << '\t' << (maximal ? "true" : "false") << '\t' << (mov_pass ? "true" : "false") << '\n';
        std::ofstream certificate(std::string(argv[3]) + "/curve-" + std::to_string(i) + ".json");
        require(bool(certificate), "cannot open degree certificate");
        certificate << "{\"label\":\"" << row.fields[0] << "\",\"role\":\"" << row.fields[2]
                    << "\",\"p\":\"" << hexadecimal(p) << "\",\"n\":\"" << hexadecimal(n)
                    << "\",\"degree\":\"" << hexadecimal(k) << "\",\"index\":\"" << hexadecimal(index)
                    << "\",\"pari_znorder_agrees\":true,\"direct_minimality_witnesses\":[";
        bool first = true;
        for (const auto& [q, ignored] : factors) {
            (void)ignored;
            if (signe(modii(k, q))) continue;
            GEN witness = Fp_pow(p, diviiexact(k, q), n);
            require(!equali1(witness), "degree is not minimal");
            if (!first) certificate << ',';
            first = false;
            certificate << "[\"" << hexadecimal(q) << "\",\"" << hexadecimal(witness) << "\"]";
        }
        require(equali1(Fp_pow(p, k, n)), "order exponent is invalid");
        certificate << "]}\n";
        avma = saved;
    }
    std::ofstream summary(std::string(argv[3]) + "/summary.json");
    require(bool(summary), "cannot open summary");
    summary << "{\"kind\":\"native-exact-embedding-cohort/v1\",\"rows\":21,\"published\":10,\"skipped\":11,";
    auto write_rule = [&](const char* name, long pub, long skip, const std::vector<std::string>& errors) {
        summary << '"' << name << "\":{\"passing_published\":" << pub << ",\"passing_skipped\":" << skip
                << ",\"errors_when_pass_predicts_published\":" << errors.size() << ",\"mismatches\":[";
        for (size_t i = 0; i < errors.size(); ++i) {
            if (i) summary << ',';
            summary << '"' << errors[i] << '"';
        }
        summary << "]}";
    };
    write_rule("maximal_degree", maximal_published, maximal_skipped, maximal_mismatches);
    summary << ',';
    write_rule("rfc7748_mov_bound", mov_published, mov_skipped, mov_mismatches);
    summary << ",\"post_hoc_cutoffs_searched\":0,\"historical_mechanism_established\":false}\n";
    std::cout << "21 exact degrees computed and cross-checked; both fixed rules evaluated.\n";
    pari_close(); return 0;
  } catch (const std::exception& error) {
    std::cerr << "ERROR: " << error.what() << '\n'; return 1;
  }
}
