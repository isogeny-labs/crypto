// Frozen round-one IDC1 helper prefix reused verbatim, without its entrypoint.
// Native issuer for the repository's identity.certificate/v1 format.
// Field, canonical statement and stream rules follow
// src/cryptanalysis/identity_certificate.rs; the Lean proof pointers are exact.
#include <openssl/sha.h>
#include <algorithm>
#include <array>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using U = std::uint64_t;
static constexpr U modulus = (U(1) << 61) - 1;

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
    for (auto b : h) s << std::hex << std::setw(2) << std::setfill('0') << unsigned(b);
    return s.str();
}
void little(std::string& out, U n) {
    for (unsigned j = 0; j < 8; ++j) out += char((n >> (8*j)) & 255);
}
U splitmix(U& s) {
    U z = (s += 0x9e3779b97f4a7c15ULL);
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
    return z ^ (z >> 31);
}
struct Expr {
    std::string op, name;
    std::int64_t value = 0;
    std::vector<Expr> args;
    std::string json() const {
        std::string a;
        if (op == "var") a = quote(name);
        else if (op == "const") a = std::to_string(value);
        else if (op == "neg") a = args.at(0).json();
        else {
            a = "[";
            for (std::size_t i = 0; i < args.size(); ++i) a += (i ? "," : "") + args[i].json();
            a += "]";
        }
        return "{\"args\":" + a + ",\"op\":" + quote(op) + "}";
    }
    unsigned degree() const {
        if (op == "var") return 1;
        if (op == "const") return 0;
        if (op == "neg") return args.at(0).degree();
        unsigned d = 0;
        for (const auto& a : args) d = op == "add" ? std::max(d, a.degree()) : d + a.degree();
        return d;
    }
    U eval(const std::map<std::string,U>& env) const {
        if (op == "var") return env.at(name);
        if (op == "const") return U(((__int128(value) % modulus) + modulus) % modulus);
        if (op == "neg") { U v = args.at(0).eval(env); return v == 0 ? 0 : modulus-v; }
        U x = op == "add" ? 0 : 1;
        for (const auto& a : args) {
            U y = a.eval(env);
            x = op == "add" ? (x+y)%modulus : U((__uint128_t(x)*y)%modulus);
        }
        return x;
    }
};
Expr var(std::string s) { return {"var", s, 0, {}}; }
Expr num(std::int64_t n) { return {"const", "", n, {}}; }
Expr add(std::initializer_list<Expr> a) { return {"add", "", 0, a}; }
Expr mul(std::initializer_list<Expr> a) { return {"mul", "", 0, a}; }
Expr neg(Expr a) { return {"neg", "", 0, {a}}; }
Expr sub(Expr a, Expr b) { return add({a,neg(b)}); }
Expr power(Expr a, unsigned n) { return {"mul", "", 0, std::vector<Expr>(n,a)}; }
struct Statement {
    std::string file, name, theorem;
    std::vector<std::string> variables;
    unsigned degree;
    Expr lhs, rhs;
    std::string json() const {
        std::string vs = "[";
        for (std::size_t i=0;i<variables.size();++i) vs += (i ? "," : "") + quote(variables[i]);
        return "{\"degree_bound\":" + std::to_string(degree) + ",\"lhs\":" + lhs.json()
            + ",\"name\":" + quote(name) + ",\"rhs\":" + rhs.json()
            + ",\"source\":" + quote(theorem)
            + ",\"variables\":" + vs + "]}";
    }
};
bool evaluate(const Statement& s, const std::string& id, U seed, std::string& values) {
    std::string input=id; little(input,seed); input += "cert";
    auto h=digest(input); U state=0;
    for(unsigned j=0;j<8;++j) state |= U(h[j]) << (8*j);
    values.clear();
    for(unsigned i=0;i<32;++i) {
        std::map<std::string,U> env;
        for(const auto& v:s.variables) env[v]=splitmix(state)%modulus;
        U lhs=s.lhs.eval(env);
        if(lhs!=s.rhs.eval(env)) return false;
        little(values,lhs);
    }
    return true;
}
