// Container development control on six already published, hash-bound curves.
// No nonce, seed search, random sample, or timing claim.
#define main retained_journal_check_main
#include "../journal_qualification_20261104/journal_check.cpp"
#undef main
#include <set>

int main(int argc, char** argv) {
    try {
        audit::need(argc == 2, "known-orders COHORT");
        const audit::fs::path cohort = argv[1];
        audit::need(audit::bind(cohort).sha256 == audit::cohort_hash,
                    "frozen cohort SHA-256");
        pari_init(256000000, 1000000);
        audit::Rows rows(cohort, "# label\tbits\trole\trun\tindex\tp\tb\tn\tseed", false);
        std::set<unsigned> checked;
        while (auto line = rows.next()) {
            auto row = audit::columns(*line, 9);
            unsigned bits = static_cast<unsigned>(audit::counter(row[1]));
            if (row[2] != "published" || checked.count(bits)) continue;
            audit::Stack frame;
            auto field = audit::fixed_field(cohort, bits);
            audit::Int b, expected;
            audit::need(b.set_str(row[6], 0) == 0 && expected.set_str(row[7], 0) == 0,
                        "known coefficient/order");
            std::cout << "{\"event\":\"start-known-order\",\"bits\":" << bits
                      << ",\"label\":\"" << row[0] << "\"}" << std::endl;
            GEN curve = ellinit(mkvec2(stoi(-3), audit::pari_int(b)),
                                audit::pari_int(field.p), DEFAULTPREC);
            auto actual = audit::from_pari(ellcard(curve, nullptr));
            audit::need(actual == expected, "published full order mismatch");
            checked.insert(bits);
            std::cout << "{\"event\":\"matched-known-order\",\"bits\":" << bits
                      << ",\"label\":\"" << row[0] << "\",\"order\":\"" << actual
                      << "\",\"signed_verification\":false}" << std::endl;
        }
        audit::need(checked == std::set<unsigned>({192, 224, 239, 256, 384, 521}),
                    "all six known field controls required");
        audit::need(audit::bind(cohort).sha256 == audit::cohort_hash, "cohort changed");
        std::cout << "{\"schema\":\"nist-known-order-control/v1\",\"known_curves\":6,"
                     "\"random_curves\":0,\"production_qualified\":false,"
                     "\"signed_verification\":false}" << std::endl;
        pari_close();
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "REFUSED: " << error.what() << '\n';
        return 1;
    }
}
