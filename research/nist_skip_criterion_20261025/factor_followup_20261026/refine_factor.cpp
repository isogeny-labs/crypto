#include "common.hpp"
#include <fstream>
#include <iostream>
#include <regex>

static std::string read_file(const char* name) {
    std::ifstream input(name);
    require(bool(input), "missing refinement input");
    std::string text((std::istreambuf_iterator<char>(input)), {});
    require(text.size() < 3000000, "refinement input too large");
    return text;
}

int main(int argc, char** argv) {
    try {
        require(argc == 4, "usage: refine_factor proposed_factors.json factor.log output.json");
        require(!std::ifstream(argv[3]), "refusing to overwrite refinement");
        pari_init(128000000, 2000000);
        const auto hint = read_file(argv[1]), log = read_file(argv[2]);
        const std::regex pair(R"rx(\[\s*"([0-9]+)"\s*,\s*([0-9]+)\s*\])rx");
        // GMP-ECM, Msieve, and YAFU default decimal output, respectively.
        // Labels such as PRP are only proposals; the separate verifier proves primes.
        const std::regex found(R"rx((?:Factor found in step [12]:|(?:prp|p|c)[0-9]+ factor:|(?:PRP|P|C|U)[0-9]+ =) ([0-9]+))rx");
        std::vector<std::pair<GEN, long>> factors;
        GEN before = gen_1;
        for (auto i = std::sregex_iterator(hint.begin(), hint.end(), pair);
             i != std::sregex_iterator(); ++i) {
            require((*i)[1].length() < 1000, "factor too large");
            GEN value = integer((*i)[1]);
            const long exponent = std::stol((*i)[2]);
            require(cmpis(value, 2) >= 0 && exponent > 0 && exponent < 1024,
                    "invalid proposed factor");
            require(factors.size() < 64, "too many factors");
            factors.emplace_back(value, exponent);
            before = mulii(before, powiu(value, exponent));
        }
        require(!factors.empty(), "empty proposed factorization");
        for (auto i = std::sregex_iterator(log.begin(), log.end(), found);
             i != std::sregex_iterator(); ++i) {
            require((*i)[1].length() < 1000, "reported factor too large");
            GEN candidate = integer((*i)[1]);
            require(cmpis(candidate, 2) >= 0, "invalid reported factor");
            std::vector<std::pair<GEN, long>> split;
            bool divides = false;
            for (const auto& entry : factors) {
                GEN divisor = gcdii(entry.first, candidate);
                if (cmpis(divisor, 1) > 0) divides = true;
                if (cmpis(divisor, 1) > 0 && cmpii(divisor, entry.first) < 0) {
                    split.emplace_back(divisor, entry.second);
                    split.emplace_back(diviiexact(entry.first, divisor), entry.second);
                } else split.push_back(entry);
            }
            require(divides, "reported factor has no divisor in exact target");
            factors = std::move(split);
        }
        GEN after = gen_1;
        for (const auto& entry : factors) after = mulii(after, powiu(entry.first, entry.second));
        require(equalii(before, after), "refinement changed the exact product");
        std::ofstream output(argv[3]);
        require(bool(output), "cannot create refinement");
        output << "{\"product_preserved\":true,\"factors\":[";
        for (size_t i = 0; i < factors.size(); ++i) {
            if (i) output << ',';
            output << "[\"" << decimal(factors[i].first) << "\"," << factors[i].second << ']';
        }
        output << "]}\n";
        require(bool(output), "failed writing refinement");
        pari_close();
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "ERROR: " << e.what() << '\n';
        return 1;
    }
}
