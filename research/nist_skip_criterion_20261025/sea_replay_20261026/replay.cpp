#include "common.hpp"
#include <fstream>
#include <iostream>
#include <map>
#include <utility>

struct Coverage {
    int last = 0;
    std::vector<int> included;
};

static Coverage coverage(GEN p, GEN t, bool include_ramified = true) {
    require(cmpis(p, 2) > 0, "invalid field input");
    GEN discriminant = subii(sqri(t), shifti(p, 2));
    GEN threshold = shifti(sqrtint(shifti(p, 2)), 1);
    GEN product = gen_1;
    Coverage result;
    for (int ell : primes_through(1000)) {
        if (ell == 2) continue;
        const long symbol = kronecker(discriminant, utoipos(ell));
        if (symbol == 1 || (include_ramified && symbol == 0)) {
            product = mulsi(ell, product);
            result.included.push_back(ell);
        }
        if (cmpii(product, threshold) > 0) {
            result.last = ell;
            return result;
        }
    }
    throw std::runtime_error("coverage not reached through 1000");
}

static void controls() {
    const auto first = coverage(stoi(103), gen_0);
    require(first.last == 13 && first.included == std::vector<int>({7, 13}),
            "p103 t0 control failed");
    const auto second = coverage(stoi(101), stoi(2));
    require(second.last == 13 && second.included == std::vector<int>({5, 13}),
            "ramified-prime control failed");
    const auto strict = coverage(stoi(101), stoi(2), false);
    require(strict.last == 17 && strict.included == std::vector<int>({13, 17}),
            "strict-Elkies distinction control failed");
}

int main(int argc, char** argv) {
    try {
        pari_init(64000000, 1000000);
        if (argc == 2 && std::string(argv[1]) == "--self-test") {
            controls();
            std::cout << "{\"fixed_arithmetic_controls_passed\":3}\n";
            pari_close();
            return 0;
        }
        require(argc == 4, "usage: replay historical.tsv recorded_feature_dir fresh_output_dir");
        controls();
        std::ifstream input(argv[1]);
        require(bool(input), "missing exact input table");
        const std::string out = std::string(argv[3]) + '/';
        require(!std::ifstream(out + "coverage.tsv") && !std::ifstream(out + "summary.json"),
                "refusing to overwrite replay evidence");
        std::ofstream table(out + "coverage.tsv"), summary(out + "summary.json");
        require(bool(table) && bool(summary), "cannot create replay outputs");
        table << "label\trole\tfield_bits\trecorded_lmax\treplayed_lmax\tmatched\n";
        size_t row_index = 0, published = 0, skipped = 0;
        std::string line;
        while (std::getline(input, line)) {
            if (line.empty() || line[0] == '#') continue;
            require(row_index < 21, "too many input rows");
            std::vector<std::string> fields;
            std::istringstream stream(line);
            std::string field;
            while (std::getline(stream, field, '\t')) fields.push_back(field);
            require(fields.size() == 9, "input schema mismatch");
            require(fields[2] == "published" || fields[2] == "skipped", "unknown role");
            published += fields[2] == "published";
            skipped += fields[2] == "skipped";
            std::ifstream features(std::string(argv[2]) + "/curve-" +
                                   std::to_string(row_index) + ".features.tsv");
            require(bool(features), "missing recorded feature table");
            std::map<std::string, long> recorded;
            std::string name;
            long value = 0;
            while (features >> name >> value)
                require(recorded.emplace(name, value).second, "duplicate feature key");
            require(features.eof(), "malformed recorded feature table");
            require(recorded.count("sea_elkies_only_lmax") == 1, "missing coverage feature");
            pari_sp saved = avma;
            GEN p = integer(fields[5]), n = integer(fields[7]);
            const auto result = coverage(p, subii(addis(p, 1), n));
            const long expected = recorded.at("sea_elkies_only_lmax");
            table << fields[0] << '\t' << fields[2] << '\t' << fields[1] << '\t'
                  << expected << '\t' << result.last << '\t'
                  << (expected == result.last ? "true" : "false") << '\n';
            require(expected == result.last, "coverage mismatch at " + fields[0]);
            avma = saved;
            ++row_index;
        }
        require(row_index == 21 && published == 10 && skipped == 11,
                "must preserve the complete 21-row cohort and roles");
        summary << "{\"kind\":\"native-recorded-sea-coverage-replay/v1\","
                << "\"rows\":21,\"published_rows\":10,\"skipped_rows\":11,"
                << "\"matched_recorded_values\":21,\"fixed_arithmetic_controls_passed\":3,"
                << "\"ramified_primes_included\":true,\"prime_two_included\":false,"
                << "\"point_count_cost_measured\":false,\"new_statistical_claim\":false,"
                << "\"historical_selector_established\":false}\n";
        require(bool(table) && bool(summary), "failed writing replay evidence");
        pari_close();
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "ERROR: " << error.what() << '\n';
        return 1;
    }
}
