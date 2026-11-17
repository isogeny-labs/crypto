#include "common.hpp"
#include <fstream>
#include <iostream>
#include <regex>

int main(int argc, char** argv) {
    try {
        require(argc == 6, "usage: prepare_factor inputs.tsv row target hint.json output_prefix");
        pari_init(128000000, 2000000);
        const int wanted = std::stoi(argv[2]);
        require(wanted >= 0 && wanted < 21, "invalid row");
        std::ifstream input(argv[1]);
        require(bool(input), "missing exact input table");
        std::string line;
        int at = 0;
        while (std::getline(input, line)) {
            if (line.empty() || line[0] == '#') continue;
            if (at++ != wanted) continue;
            std::vector<std::string> fields;
            std::istringstream row(line);
            std::string field;
            while (std::getline(row, field, '\t')) fields.push_back(field);
            require(fields.size() == 9, "input schema mismatch");
            GEN p = integer(fields[5]), n = integer(fields[7]), target = nullptr;
            const std::string kind = argv[3], prefix = argv[5];
            if (kind == "minus") target = subis(n, 1);
            else if (kind == "plus") target = addis(n, 1);
            else if (kind == "disc")
                target = absi(subii(sqri(subii(addis(p, 1), n)), shifti(p, 2)));
            else throw std::runtime_error("unsupported target");
            require(!std::ifstream(prefix + ".json"), "refusing to overwrite plan record");
            require(!std::ifstream(prefix + ".residuals.txt"), "refusing to overwrite residuals");
            std::ifstream hint(argv[4]);
            require(bool(hint), "missing proposed factors");
            std::string text((std::istreambuf_iterator<char>(hint)), {});
            require(text.size() < 1000000, "hint too large");
            std::regex pair(R"rx(\[\s*"([0-9]+)"\s*,\s*([0-9]+)\s*\])rx");
            std::vector<std::pair<GEN, long>> entries;
            GEN product = gen_1;
            for (auto i = std::sregex_iterator(text.begin(), text.end(), pair);
                 i != std::sregex_iterator(); ++i) {
                require((*i)[1].length() < 1000, "factor too large");
                GEN factor = integer((*i)[1]);
                long exponent = std::stol((*i)[2]);
                require(cmpis(factor, 2) >= 0 && exponent > 0 && exponent < 1024,
                        "invalid proposed factor");
                require(entries.size() < 64, "too many factors");
                entries.emplace_back(factor, exponent);
                product = mulii(product, powiu(factor, exponent));
            }
            require(!entries.empty() && equalii(product, target), "proposed factor product mismatch");
            std::ostringstream factors, residuals;
            factors << '[';
            size_t unresolved = 0;
            for (size_t i = 0; i < entries.size(); ++i) {
                const bool proven_prime = isprime(entries[i].first);
                if (i) factors << ',';
                factors << "{\"value\":\"" << decimal(entries[i].first)
                        << "\",\"exponent\":" << entries[i].second
                        << ",\"proven_prime\":" << (proven_prime ? "true" : "false") << '}';
                if (!proven_prime) {
                    require(entries[i].second == 1, "unsupported repeated composite residual");
                    ++unresolved;
                    residuals << decimal(entries[i].first) << '\n';
                }
            }
            factors << ']';
            require(unresolved <= 1, "multiple residuals require a separate plan");
            std::ofstream json(prefix + ".json"), residual_file(prefix + ".residuals.txt");
            require(bool(json) && bool(residual_file), "cannot create plan records");
            json << "{\"label\":\"" << fields[0] << "\",\"target\":\"" << kind
                 << "\",\"integer\":\"" << decimal(target) << "\",\"factors\":" << factors.str()
                 << ",\"product_verified\":true,\"complete_and_proven\":"
                 << (unresolved == 0 ? "true" : "false") << "}\n";
            residual_file << residuals.str();
            require(bool(json) && bool(residual_file), "failed writing plan records");
            std::cout << fields[0] << ' ' << kind << ": " << unresolved
                      << " unresolved composite residual(s)\n";
            pari_close();
            return 0;
        }
        throw std::runtime_error("row not found");
    } catch (const std::exception& e) {
        std::cerr << "ERROR: " << e.what() << '\n';
        return 1;
    }
}
