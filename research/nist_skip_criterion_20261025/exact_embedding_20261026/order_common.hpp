#pragma once
#include "common.hpp"
#include <algorithm>
#include <fstream>
#include <regex>
#include <set>

struct CurveRow { std::vector<std::string> fields; };
using FactorList = std::vector<std::pair<GEN, long>>;

inline std::string read_text(const std::string& path) {
    std::ifstream in(path);
    require(bool(in), "cannot read " + path);
    return std::string((std::istreambuf_iterator<char>(in)), {});
}

inline std::string string_field(const std::string& text, const std::string& key) {
    std::regex pattern("\"" + key + "\"\\s*:\\s*\"([^\"]+)\"");
    auto it = std::sregex_iterator(text.begin(), text.end(), pattern);
    require(it != std::sregex_iterator(), "missing string field " + key);
    std::string value = (*it)[1];
    require(++it == std::sregex_iterator(), "duplicate string field " + key);
    return value;
}

inline std::vector<CurveRow> read_rows(const std::string& path) {
    std::ifstream in(path);
    require(bool(in), "missing historical input");
    std::vector<CurveRow> rows;
    std::string line;
    std::set<std::string> labels;
    long published = 0, skipped = 0;
    while (std::getline(in, line)) {
        if (line.empty() || line[0] == '#') continue;
        CurveRow row;
        std::istringstream split(line);
        std::string field;
        while (std::getline(split, field, '\t')) row.fields.push_back(field);
        require(row.fields.size() == 9, "historical schema mismatch");
        require(labels.insert(row.fields[0]).second, "duplicate historical identity");
        require(row.fields[2] == "published" || row.fields[2] == "skipped", "unknown role");
        published += row.fields[2] == "published";
        skipped += row.fields[2] == "skipped";
        rows.push_back(row);
    }
    require(rows.size() == 21 && published == 10 && skipped == 11, "cohort identity/count mismatch");
    return rows;
}

inline FactorList read_minus(const std::string& path, const CurveRow& row, GEN n) {
    const std::string text = read_text(path);
    require(string_field(text, "label") == row.fields[0], "factor identity mismatch");
    require(string_field(text, "target") == "minus", "wrong factor target");
    require(equalii(integer(string_field(text, "integer")), subis(n, 1)), "wrong original target");
    const std::regex pattern(R"rx(\[\s*"([0-9]+)"\s*,\s*([0-9]+)\s*\])rx");
    FactorList result;
    GEN product = gen_1;
    std::set<std::string> seen;
    for (auto it = std::sregex_iterator(text.begin(), text.end(), pattern);
         it != std::sregex_iterator(); ++it) {
        GEN prime = integer((*it)[1]);
        const long exponent = std::stol((*it)[2]);
        require(exponent > 0 && exponent < 1024 && isprime(prime), "invalid proved factor");
        require(seen.insert(decimal(prime)).second, "duplicate factor");
        result.emplace_back(prime, exponent);
        product = mulii(product, powiu(prime, exponent));
    }
    require(!result.empty() && equalii(product, subis(n, 1)), "incomplete n-1 product");
    return result;
}

inline GEN factor_matrix(const FactorList& factors) {
    GEN primes = cgetg(factors.size() + 1, t_COL);
    GEN exponents = cgetg(factors.size() + 1, t_COL);
    for (size_t i = 0; i < factors.size(); ++i) {
        gel(primes, i + 1) = factors[i].first;
        gel(exponents, i + 1) = stoi(factors[i].second);
    }
    return mkmat2(primes, exponents);
}

inline std::string hexadecimal(GEN value) {
    require(signe(value) >= 0, "negative hexadecimal value");
    std::string out;
    do { out += "0123456789ABCDEF"[umodiu(value, 16)]; value = shifti(value, -4); }
    while (signe(value));
    std::reverse(out.begin(), out.end());
    return "0x" + out;
}
