// Separate textual equation evaluator. Does not reuse the exporter's polynomial code.
// Development verification only: checks assignments, never solves the system.
#include <cctype>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using U = std::uint64_t;

struct Evaluator {
    const std::string &s;
    const std::map<std::string, U> &values;
    U p;
    std::size_t pos = 0;
    void space() { while (pos < s.size() && std::isspace(static_cast<unsigned char>(s[pos]))) ++pos; }
    bool take(char c) { space(); if (pos < s.size() && s[pos] == c) { ++pos; return true; } return false; }
    U add(U a, U b) const { return (a + b) % p; }
    U mul(U a, U b) const { return (a * b) % p; } // p<=2^31-1, so reduced products fit U.
    U atom() {
        space();
        if (take('+')) return atom();
        if (take('-')) { U v = atom(); return v ? p - v : 0; }
        if (take('(')) { U v = sum(); if (!take(')')) throw std::runtime_error("missing close parenthesis"); return v; }
        if (pos == s.size()) throw std::runtime_error("missing operand");
        if (std::isdigit(static_cast<unsigned char>(s[pos]))) {
            U v = 0;
            while (pos < s.size() && std::isdigit(static_cast<unsigned char>(s[pos]))) v = add(mul(v, 10 % p), static_cast<U>(s[pos++] - '0') % p);
            return v;
        }
        std::size_t start = pos;
        while (pos < s.size() && (std::isalnum(static_cast<unsigned char>(s[pos])) || s[pos] == '_')) ++pos;
        if (start == pos) throw std::runtime_error("invalid expression token");
        auto it = values.find(s.substr(start, pos - start));
        if (it == values.end()) throw std::runtime_error("unknown variable");
        return it->second;
    }
    // Export contract uses expanded products; exponent tokens are rejected.
    U product() { U v = atom(); while (take('*')) v = mul(v, atom()); return v; }
    U sum() {
        U v = product();
        for (;;) {
            if (take('+')) v = add(v, product());
            else if (take('-')) { U w = product(); v = add(v, w ? p - w : 0); }
            else return v;
        }
    }
    U run() { U v = sum(); space(); if (pos != s.size()) throw std::runtime_error("trailing expression input"); return v; }
};

std::vector<std::string> split(const std::string &s, char sep) {
    std::vector<std::string> r; std::stringstream in(s); std::string part;
    while (std::getline(in, part, sep)) {
        auto a = part.find_first_not_of(" \r\n\t");
        if (a != std::string::npos) r.push_back(part.substr(a, part.find_last_not_of(" \r\n\t") - a + 1));
    }
    return r;
}

int main(int argc, char **argv) {
    try {
        if (argc != 3) throw std::runtime_error("usage: check_export system.ms assignment.tsv");
        std::ifstream system(argv[1]), assignment(argv[2]);
        if (!system || !assignment) throw std::runtime_error("cannot open input");
        std::string vars, prime, header;
        if (!std::getline(system, vars) || !std::getline(system, prime)) throw std::runtime_error("missing system header");
        std::size_t used = 0;
        U p = std::stoull(prime, &used);
        if (used != prime.size() || p < 5 || p > 2147483647) throw std::runtime_error("unsupported modulus");
        // Fixture modulus verified independently here; no composite-field arithmetic accepted.
        for (U d = 2; d * d <= p; ++d) if (p % d == 0) throw std::runtime_error("composite modulus");
        auto names = split(vars, ',');
        std::set<std::string> declared(names.begin(), names.end());
        if (names.empty() || declared.size() != names.size()) throw std::runtime_error("empty/duplicate variable declaration");
        std::map<std::string, U> values;
        if (!std::getline(assignment, header) || header != "variable\tvalue") throw std::runtime_error("invalid assignment header");
        std::string line;
        while (std::getline(assignment, line)) {
            if (line.empty()) continue;
            auto row = split(line, '\t');
            if (row.size() != 2 || !declared.count(row[0])) throw std::runtime_error("invalid assignment row");
            if (row[1].empty() || row[1].find_first_not_of("0123456789") != std::string::npos) throw std::runtime_error("nondecimal assignment");
            used = 0; U v = std::stoull(row[1], &used);
            if (used != row[1].size() || v >= p || !values.emplace(row[0], v).second) throw std::runtime_error("invalid value/duplicate assignment");
        }
        if (values.size() != declared.size()) throw std::runtime_error("incomplete assignment");
        std::string equations((std::istreambuf_iterator<char>(system)), std::istreambuf_iterator<char>());
        auto eqs = split(equations, ',');
        if (eqs.empty()) throw std::runtime_error("empty system");
        std::size_t bad = 0;
        for (const auto &eq : eqs) if (Evaluator{eq, values, p}.run() != 0) ++bad;
        std::cout << "{\"modulus\":" << p << ",\"variables\":" << values.size() << ",\"equations\":" << eqs.size() << ",\"nonzero_equations\":" << bad << "}\n";
        return bad ? 1 : 0;
    } catch (const std::exception &e) { std::cerr << e.what() << '\n'; return 2; }
}
