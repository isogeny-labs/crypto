// Native issuer for the report's identity.certificate/v1 record.
// The canonical JSON, point stream, field arithmetic and digest rules mirror
// src/cryptanalysis/identity_certificate.rs. The certificate deliberately
// covers only the polynomial core (c-y)+y=c, not division by tau-z.
#include <openssl/sha.h>

#include <algorithm>
#include <array>
#include <cstdint>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using U = std::uint64_t;
static constexpr U modulus = (U(1) << 61) - 1;
static constexpr U issue_seed = 0x202611174597ULL;
static constexpr U extra_seed = 0x202611174598ULL;

std::string quote(const std::string& s) {
    std::string out = "\"";
    for (char c : s) {
        if (c == '"' || c == '\\') out += '\\';
        if (c == '\n') out += "\\n"; else out += c;
    }
    return out + '"';
}
std::array<unsigned char, 32> digest(const std::string& s) {
    std::array<unsigned char, 32> h{};
    SHA256(reinterpret_cast<const unsigned char*>(s.data()), s.size(), h.data());
    return h;
}

std::string hex(const std::array<unsigned char, 32>& h) {
    std::ostringstream s;
    for (auto b : h)
        s << std::hex << std::setw(2) << std::setfill('0') << unsigned(b);
    return s.str();
}

void little(std::string& out, U n) {
    for (unsigned j = 0; j < 8; ++j) out += char((n >> (8 * j)) & 255);
}

U splitmix(U& s) {
    U z = (s += 0x9e3779b97f4a7c15ULL);
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
    return z ^ (z >> 31);
}

struct Expr {
    std::string op;
    std::string name;
    std::int64_t value = 0;
    std::vector<Expr> args;

    std::string json() const {
        std::string encoded;
        if (op == "var") encoded = quote(name);
        else if (op == "const") encoded = std::to_string(value);
        else if (op == "neg") encoded = args.at(0).json();
        else {
            encoded = "[";
            for (std::size_t i = 0; i < args.size(); ++i)
                encoded += (i ? "," : "") + args[i].json();
            encoded += "]";
        }
        return "{\"args\":" + encoded + ",\"op\":" + quote(op) + "}";
    }

    U eval(const std::map<std::string, U>& env) const {
        if (op == "var") return env.at(name);
        if (op == "const")
            return U(((__int128(value) % modulus) + modulus) % modulus);
        if (op == "neg") {
            U v = args.at(0).eval(env);
            return v == 0 ? 0 : modulus - v;
        }
        U x = op == "add" ? 0 : 1;
        for (const auto& arg : args) {
            U y = arg.eval(env);
            x = op == "add" ? (x + y) % modulus
                            : U((__uint128_t(x) * y) % modulus);
        }
        return x;
    }
};

Expr var(std::string s) { return {"var", std::move(s), 0, {}}; }
Expr num(std::int64_t n) { return {"const", "", n, {}}; }
Expr add(std::initializer_list<Expr> xs) { return {"add", "", 0, xs}; }
Expr neg(Expr x) { return {"neg", "", 0, {std::move(x)}}; }

struct Statement {
    std::string name;
    std::string source;
    std::vector<std::string> variables;
    unsigned degree;
    Expr lhs;
    Expr rhs;

    std::string json() const {
        std::string vars = "[";
        for (std::size_t i = 0; i < variables.size(); ++i)
            vars += (i ? "," : "") + quote(variables[i]);
        vars += "]";
        return "{\"degree_bound\":" + std::to_string(degree)
            + ",\"lhs\":" + lhs.json()
            + ",\"name\":" + quote(name)
            + ",\"rhs\":" + rhs.json()
            + ",\"source\":" + quote(source)
            + ",\"variables\":" + vars + "}";
    }
};

Statement statement() {
    auto c = var("c");
    auto y = var("y");
    return {
        "KZG trapdoor-opening additive simplification (polynomial core only)",
        "research/ecdlp_weakness_lean_20261104/reports/attack-corpus-five-20261117/REPORT.md#4-kzg-toxic-waste",
        {"c", "y"},
        1,
        add({c, neg(y), y}),
        c,
    };
}

bool evaluate(const Statement& st, const std::string& id, U seed,
              const std::string& domain, unsigned count, std::string* values) {
    std::string input = id;
    little(input, seed);
    input += domain;
    auto h = digest(input);
    U state = 0;
    for (unsigned j = 0; j < 8; ++j) state |= U(h[j]) << (8 * j);
    if (values) values->clear();
    for (unsigned i = 0; i < count; ++i) {
        std::map<std::string, U> env;
        for (const auto& v : st.variables) env[v] = splitmix(state) % modulus;
        U lhs = st.lhs.eval(env);
        if (lhs != st.rhs.eval(env)) return false;
        if (values) little(*values, lhs);
    }
    return true;
}

int main() try {
    auto st = statement();
    const std::string id = "IDC1h" + hex(digest(st.json())).substr(0, 16);
    std::string values;
    if (!evaluate(st, id, issue_seed, "cert", 32, &values))
        throw std::runtime_error("identity refused during issuance");
    if (!evaluate(st, id, extra_seed, "extra", 64, nullptr))
        throw std::runtime_error("identity refused during extra replay");

    auto altered = st;
    altered.rhs = add({altered.rhs, num(1)});
    const std::string altered_id =
        "IDC1h" + hex(digest(altered.json())).substr(0, 16);
    if (evaluate(altered, altered_id, issue_seed, "cert", 32, nullptr))
        throw std::runtime_error("negative control accepted");

    std::cout
        << "{\"schema\":\"identity.certificate/v1\",\"id\":" << quote(id)
        << ",\"statement\":" << st.json()
        << ",\"modulus\":" << modulus
        << ",\"points\":32,\"seed\":" << issue_seed
        << ",\"value_digest\":" << quote(hex(digest(values)))
        << ",\"independence\":"
        << quote("Native C++ issuer; replayed with 64 additional domain-separated points and a changed-RHS negative control. Rust replay was unavailable in this environment. The record covers only (c-y)+y=c over F_(2^61-1); tau-z != 0 and the KZG interpretation require the separate Lean theorem and source review.")
        << ",\"proof\":null}\n";
    std::cerr << "accepted " << id
              << " at 32 issuance + 64 extra points; negative control refused\n";
} catch (const std::exception& e) {
    std::cerr << e.what() << '\n';
    return 1;
}
