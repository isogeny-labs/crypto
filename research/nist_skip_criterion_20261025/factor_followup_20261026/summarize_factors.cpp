#include "common.hpp"
#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <regex>

struct Row { std::vector<std::string> fields; };
struct Record { bool complete = false; long largest_bits = 0, total = 0; };

static std::string hexadecimal(GEN value) {
    require(signe(value) >= 0, "negative factor");
    const char* digits = "0123456789ABCDEF";
    std::string out;
    do { out += digits[umodiu(value, 16)]; value = shifti(value, -4); }
    while (signe(value));
    std::reverse(out.begin(), out.end());
    return "0x" + out;
}

int main(int argc, char** argv) {
  try {
    require(argc == 4, "usage: summarize_factors historical.tsv complete_record_dir fresh_output_dir");
    pari_init(512000000, 2000000);
    std::ifstream input(argv[1]);
    require(bool(input), "missing exact input table");
    std::vector<Row> rows;
    std::string line;
    while (std::getline(input, line)) {
        if (line.empty() || line[0] == '#') continue;
        std::istringstream in(line);
        Row row;
        while (std::getline(in, line, '\t')) row.fields.push_back(line);
        require(row.fields.size() == 9, "input schema mismatch");
        rows.push_back(row);
    }
    require(rows.size() == 21, "must retain all 21 controls");
    const std::string prefix = std::string(argv[3]) + '/';
    for (const auto* name : {"factor-summary.json", "cohort-factor-bits.tsv", "completed-targets-hex.md"})
        require(!std::ifstream(prefix + name), "refusing to overwrite summary");
    std::ofstream table(prefix + "cohort-factor-bits.tsv"),
                  summary(prefix + "factor-summary.json"),
                  detail(prefix + "completed-targets-hex.md");
    require(bool(table) && bool(summary) && bool(detail), "cannot create outputs");
    table << "label\trole\tfield_bits\ttarget\tcomplete\tlargest_prime_bits\tlargest_per_field_permille\n";
    detail << "# Newly completed factorizations\n\nIntegers below are hexadecimal. Exponents and bit lengths are decimal.\n";
    std::map<std::string, long> counts;
    long complete_total = 0;
    const std::regex pair(R"rx(\[\s*"([0-9]+)"\s*,\s*([0-9]+)\s*\])rx");
    for (size_t i = 0; i < rows.size(); ++i) {
        const auto& fields = rows[i].fields;
        GEN p = integer(fields[5]), n = integer(fields[7]);
        const long bits = std::stol(fields[1]);
        for (const std::string type : {"minus", "plus", "twist", "disc"}) {
            GEN target = nullptr;
            if (type == "minus") target = subis(n, 1);
            else if (type == "plus") target = addis(n, 1);
            else if (type == "twist") target = subii(shifti(addis(p, 1), 1), n);
            else target = absi(subii(sqri(subii(addis(p, 1), n)), shifti(p, 2)));
            const auto name = std::string(argv[2]) + "/curve-" + std::to_string(i) + '-' + type + ".json";
            std::ifstream record(name);
            Record value;
            std::vector<std::pair<GEN, long>> factors;
            if (record) {
                const std::string text((std::istreambuf_iterator<char>(record)), {});
                require(text.size() < 1000000, "factor record too large");
                require(text.find("\"complete_and_proven\":true") != std::string::npos, "noncomplete record in complete set");
                require(text.find("\"label\":\"" + fields[0] + "\"") != std::string::npos, "record label mismatch");
                require(text.find("\"target\":\"" + type + "\"") != std::string::npos, "record target mismatch");
                GEN product = gen_1;
                for (auto it = std::sregex_iterator(text.begin(), text.end(), pair);
                     it != std::sregex_iterator(); ++it) {
                    require((*it)[1].length() < 1000, "factor too large");
                    GEN prime = integer((*it)[1]);
                    const long exponent = std::stol((*it)[2]);
                    require(exponent > 0 && exponent < 1024 && isprime(prime), "unproved prime factor");
                    require(factors.size() < 64, "too many factors");
                    factors.emplace_back(prime, exponent);
                    product = mulii(product, powiu(prime, exponent));
                    value.total += exponent;
                    value.largest_bits = std::max(value.largest_bits, bit_length(prime));
                }
                require(!factors.empty() && equalii(product, target), "record differs from recomputed target");
                value.complete = true;
                ++counts[type];
                ++complete_total;
            } else require(i == 20 && type == "plus", "unexpected missing factor record");
            table << fields[0] << '\t' << fields[2] << '\t' << bits << '\t' << type << '\t'
                  << (value.complete ? "true" : "false") << '\t';
            if (value.complete) table << value.largest_bits << '\t' << 1000 * value.largest_bits / bits;
            else table << "NA\tNA";
            table << '\n';
            if ((i == 13 && type == "plus") || (i == 19 && type == "disc") ||
                (i == 20 && type == "minus") || (i == 20 && type == "plus" && value.complete)) {
                require(value.complete, "newly completed target missing");
                detail << "\n## " << fields[0] << ' ' << type << "\n\n";
                detail << "Target: `" << hexadecimal(target) << "`.\n\nPrime factors:\n\n";
                for (const auto& factor : factors)
                    detail << "- `" << hexadecimal(factor.first) << "`"
                           << (factor.second == 1 ? "" : " ^ " + std::to_string(factor.second)) << '\n';
                detail << "\nLargest prime: " << value.largest_bits << " bits. Exact target product and native primality proofs passed.\n";
            }
        }
    }
    summary << "{\"kind\":\"native-factor-coverage/v1\",\"rows\":21,\"target_records\":84,\"complete_and_proven\":"
            << complete_total << ",\"complete_by_family\":{";
    bool first = true;
    for (const auto& [type, count] : counts) {
        if (!first) summary << ',';
        first = false;
        summary << '"' << type << "\":" << count;
    }
    summary << "},\"unknown_values_encoded_as\":\"NA\",\"full_factor_plus_search_eligible\":"
            << (counts["plus"] == 21 ? "true" : "false") << "}\n";
    require(bool(table) && bool(summary) && bool(detail), "summary write failed");
    pari_close();
    return 0;
  } catch (const std::exception& e) {
    std::cerr << "ERROR: " << e.what() << '\n';
    return 1;
  }
}
