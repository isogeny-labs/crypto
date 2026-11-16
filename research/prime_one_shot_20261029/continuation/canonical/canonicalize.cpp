// Bounded format adapter, not a polynomial-system solver.
// Expands arithmetic expressions and emits one collected occurrence per monomial.
#include <algorithm>
#include <cctype>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace canonical {
using U = std::uint64_t;
using Monomial = std::vector<unsigned>;
using Terms = std::map<Monomial, U>;
constexpr unsigned max_degree = 32;
constexpr std::size_t max_terms = 100000;
constexpr std::size_t max_variables = 1024;
constexpr std::size_t max_input_bytes = 1024 * 1024;

std::string trim(const std::string &s) {
    const auto first = s.find_first_not_of(" \t\r\n");
    if (first == std::string::npos) return {};
    return s.substr(first, s.find_last_not_of(" \t\r\n") - first + 1);
}

U unsigned_decimal(const std::string &s) {
    if (s.empty() || s.find_first_not_of("0123456789") != std::string::npos)
        throw std::invalid_argument("invalid unsigned decimal");
    U value = 0;
    for (char c : s) {
        const U digit = static_cast<U>(c - '0');
        if (value > (UINT64_MAX - digit) / 10)
            throw std::invalid_argument("unsigned decimal overflow");
        value = value * 10 + digit;
    }
    return value;
}

bool prime(U p) {
    if (p < 2) return false;
    for (U d = 2; d * d <= p; ++d) if (p % d == 0) return false;
    return true;
}

unsigned degree(const Monomial &m) {
    return std::accumulate(m.begin(), m.end(), 0U);
}

struct Polynomial {
    std::size_t n;
    U p;
    Terms terms;

    Polynomial(std::size_t variable_count, U modulus) : n(variable_count), p(modulus) {}
    static Polynomial constant(std::size_t n, U p, U c) {
        Polynomial r(n, p);
        if (c % p) r.terms.emplace(Monomial(n, 0), c % p);
        return r;
    }
    static Polynomial variable(std::size_t n, U p, std::size_t index) {
        Polynomial r(n, p);
        Monomial m(n, 0);
        m.at(index) = 1;
        r.terms.emplace(std::move(m), 1);
        return r;
    }
    void insert(const Monomial &m, U c) {
        c %= p;
        if (!c) return;
        auto found = terms.find(m);
        if (found == terms.end()) {
            if (terms.size() == max_terms) throw std::invalid_argument("term expansion cap exceeded");
            terms.emplace(m, c);
        } else {
            found->second = (found->second + c) % p;
            if (!found->second) terms.erase(found);
        }
    }
    Polynomial plus(const Polynomial &other, bool subtract = false) const {
        Polynomial r = *this;
        for (const auto &entry : other.terms)
            r.insert(entry.first, subtract ? p - entry.second : entry.second);
        return r;
    }
    Polynomial times(const Polynomial &other) const {
        Polynomial r(n, p);
        for (const auto &a : terms) for (const auto &b : other.terms) {
            Monomial m(n, 0);
            for (std::size_t i = 0; i < n; ++i) m[i] = a.first[i] + b.first[i];
            if (degree(m) > max_degree) throw std::invalid_argument("polynomial degree cap exceeded");
            // p<2^31, so reduced coefficient products fit uint64.
            r.insert(m, (a.second * b.second) % p);
        }
        return r;
    }
    Polynomial power(unsigned e) const {
        Polynomial value = *this;
        Polynomial r = constant(n, p, 1);
        while (e) {
            if (e & 1U) r = r.times(value);
            e >>= 1;
            if (e) value = value.times(value);
        }
        return r;
    }
    U evaluate(const std::vector<U> &assignment) const {
        if (assignment.size() != n) throw std::invalid_argument("wrong assignment size");
        U answer = 0;
        for (const auto &entry : terms) {
            U value = entry.second;
            for (std::size_t i = 0; i < n; ++i)
                for (unsigned k = 0; k < entry.first[i]; ++k)
                    value = (value * assignment[i]) % p;
            answer = (answer + value) % p;
        }
        return answer;
    }
    unsigned largest_degree() const {
        unsigned answer = 0;
        for (const auto &entry : terms) answer = std::max(answer, degree(entry.first));
        return answer;
    }
    std::string serialize(const std::vector<std::string> &names, bool products) const {
        if (terms.empty()) return "0";
        std::ostringstream out;
        bool first = true;
        for (auto it = terms.rbegin(); it != terms.rend(); ++it) {
            if (!first) out << '+';
            first = false;
            out << it->second;
            for (std::size_t i = 0; i < n; ++i) {
                const unsigned exponent = it->first[i];
                if (!exponent) continue;
                if (products) {
                    for (unsigned j = 0; j < exponent; ++j) out << '*' << names[i];
                } else {
                    out << '*' << names[i];
                    if (exponent != 1) out << '^' << exponent;
                }
            }
        }
        return out.str();
    }
};

struct Parser {
    const std::string &input;
    const std::map<std::string, std::size_t> &variables;
    U p;
    std::size_t n, position = 0;
    unsigned nesting = 0;

    void spaces() {
        while (position < input.size() && std::isspace(static_cast<unsigned char>(input[position]))) ++position;
    }
    bool take(char c) {
        spaces();
        if (position < input.size() && input[position] == c) { ++position; return true; }
        return false;
    }
    Polynomial atom() {
        spaces();
        if (take('(')) {
            if (++nesting > 1024) throw std::invalid_argument("nesting cap exceeded");
            Polynomial r = sum();
            if (!take(')')) throw std::invalid_argument("missing closing parenthesis");
            --nesting;
            return r;
        }
        if (position == input.size()) throw std::invalid_argument("missing operand");
        if (std::isdigit(static_cast<unsigned char>(input[position]))) {
            U c = 0;
            while (position < input.size() && std::isdigit(static_cast<unsigned char>(input[position])))
                c = (10 * c + static_cast<U>(input[position++] - '0')) % p;
            return Polynomial::constant(n, p, c);
        }
        const auto start = position;
        if (!(std::isalpha(static_cast<unsigned char>(input[position])) || input[position] == '_'))
            throw std::invalid_argument("unsupported expression token");
        while (position < input.size() && (std::isalnum(static_cast<unsigned char>(input[position])) || input[position] == '_')) ++position;
        auto found = variables.find(input.substr(start, position - start));
        if (found == variables.end()) throw std::invalid_argument("undeclared variable");
        return Polynomial::variable(n, p, found->second);
    }
    Polynomial power() {
        Polynomial r = atom();
        if (take('^')) {
            spaces();
            const auto start = position;
            while (position < input.size() && std::isdigit(static_cast<unsigned char>(input[position]))) ++position;
            U e = unsigned_decimal(input.substr(start, position - start));
            if (e > max_degree) throw std::invalid_argument("exponent cap exceeded");
            r = r.power(static_cast<unsigned>(e));
        }
        return r;
    }
    Polynomial unary() {
        if (take('+')) return unary();
        if (take('-')) return Polynomial::constant(n, p, 0).plus(unary(), true);
        return power();
    }
    Polynomial product() {
        Polynomial r = unary();
        while (take('*')) r = r.times(unary());
        return r;
    }
    Polynomial sum() {
        Polynomial r = product();
        for (;;) {
            if (take('+')) r = r.plus(product());
            else if (take('-')) r = r.plus(product(), true);
            else return r;
        }
    }
    Polynomial parse() {
        Polynomial r = sum();
        spaces();
        if (position != input.size()) throw std::invalid_argument("trailing expression input");
        return r;
    }
};

std::vector<std::string> split_nonempty(const std::string &text, char delimiter) {
    std::vector<std::string> out;
    std::size_t start = 0;
    for (;;) {
        const auto end = text.find(delimiter, start);
        auto part = trim(text.substr(start, end == std::string::npos ? end : end - start));
        if (part.empty()) throw std::invalid_argument("empty delimited field");
        out.push_back(std::move(part));
        if (end == std::string::npos) return out;
        start = end + 1;
    }
}

struct System {
    std::vector<std::string> names;
    std::map<std::string, std::size_t> variables;
    U p = 0;
    std::vector<Polynomial> equations;
};

System read_system(const std::filesystem::path &file) {
    if (std::filesystem::file_size(file) > max_input_bytes) throw std::invalid_argument("input byte cap exceeded");
    std::ifstream in(file);
    if (!in) throw std::invalid_argument("cannot open system input");
    System s;
    std::string header, characteristic;
    if (!std::getline(in, header) || !std::getline(in, characteristic)) throw std::invalid_argument("missing system headers");
    s.names = split_nonempty(header, ',');
    if (s.names.size() > max_variables) throw std::invalid_argument("variable cap exceeded");
    for (std::size_t i = 0; i < s.names.size(); ++i) {
        const std::string &name = s.names[i];
        if (!(std::isalpha(static_cast<unsigned char>(name[0])) || name[0] == '_')) throw std::invalid_argument("invalid variable name");
        for (char c : name) if (!(std::isalnum(static_cast<unsigned char>(c)) || c == '_')) throw std::invalid_argument("invalid variable name");
        if (!s.variables.emplace(name, i).second) throw std::invalid_argument("duplicate variable declaration");
    }
    s.p = unsigned_decimal(trim(characteristic));
    if (s.p >= (U{1} << 31) || !prime(s.p)) throw std::invalid_argument("unsupported or nonprime modulus");
    const std::string body((std::istreambuf_iterator<char>(in)), std::istreambuf_iterator<char>());
    for (const auto &expression : split_nonempty(body, ','))
        s.equations.push_back(Parser{expression, s.variables, s.p, s.names.size()}.parse());
    return s;
}

std::vector<U> read_assignment(const std::filesystem::path &file, const System &s) {
    std::ifstream in(file);
    std::string line;
    if (!in || !std::getline(in, line) || trim(line) != "variable\tvalue") throw std::invalid_argument("invalid assignment header");
    std::vector<U> result(s.names.size());
    std::set<std::size_t> seen;
    while (std::getline(in, line)) {
        auto fields = split_nonempty(line, '\t');
        if (fields.size() != 2) throw std::invalid_argument("invalid assignment row");
        auto found = s.variables.find(fields[0]);
        if (found == s.variables.end()) throw std::invalid_argument("unknown assignment variable");
        U value = unsigned_decimal(fields[1]);
        if (value >= s.p || !seen.insert(found->second).second) throw std::invalid_argument("invalid or duplicate assignment");
        result[found->second] = value;
    }
    if (seen.size() != s.names.size()) throw std::invalid_argument("incomplete assignment");
    return result;
}

void write_system(const std::filesystem::path &file, const System &s, bool products) {
    std::ofstream out(file);
    if (!out) throw std::invalid_argument("cannot open system output");
    for (std::size_t i = 0; i < s.names.size(); ++i) out << (i ? "," : "") << s.names[i];
    out << '\n' << s.p << '\n';
    for (std::size_t i = 0; i < s.equations.size(); ++i)
        out << s.equations[i].serialize(s.names, products) << (i + 1 == s.equations.size() ? "\n" : ",\n");
    out.close();
    if (!out) throw std::invalid_argument("output write failed");
}

unsigned selftest() {
    const std::map<std::string, std::size_t> names{{"x",0},{"y",1}};
    const std::vector<std::string> ordered{"x","y"};
    unsigned checks = 0;
    auto parse = [&](const std::string &s) { return Parser{s,names,17,2}.parse(); };
    auto expect = [&](bool value) { if (!value) throw std::runtime_error("selftest failed"); ++checks; };
    expect(parse("(x+y)^2-x*x-2*x*y-y*y").terms.empty());
    expect(parse("x+x+17*y").serialize(ordered,false)=="2*x");
    expect(parse("(x+y)*(x-y)").serialize(ordered,false)=="1*x^2+16*y^2");
    expect(parse("-x^2").serialize(ordered,false)=="16*x^2");
    expect(parse("(-x)^2").serialize(ordered,false)=="1*x^2");
    expect(parse("2*x*y+15*y*x").terms.empty());
    expect(parse("x^0+16").terms.empty());
    expect(parse("(x+y)^3").evaluate({2,3})==6);
    for (const std::string &bad : {std::string(""), std::string("x/2"), std::string("x^^2"), std::string("x^33"), std::string("z"), std::string("2x"), std::string("(x"), std::string("x;"), std::string("x^-1")}) {
        bool rejected = false;
        try { (void)parse(bad); } catch (const std::invalid_argument &) { rejected = true; }
        expect(rejected);
    }
    expect(prime(2) && prime(17) && !prime(1) && !prime(15));
    for (const std::string &bad : {std::string("x,"), std::string(",x"), std::string("x,,y")}) {
        bool rejected = false;
        try { (void)split_nonempty(bad, ','); } catch (const std::invalid_argument &) { rejected = true; }
        expect(rejected);
    }
    return checks;
}
} // namespace canonical

int main(int argc, char **argv) {
    try {
        using namespace canonical;
        if (argc == 2 && std::string(argv[1]) == "--selftest") {
            std::cout << "PASS canonical adapter selftests=" << selftest() << "; no solving or timings\n";
            return 0;
        }
        if (argc < 3) throw std::invalid_argument("usage: canonicalize INPUT OUTPUT [--assignment FILE] [--products-output FILE]");
        std::filesystem::path assignment, products;
        for (int i = 3; i < argc; i += 2) {
            if (i + 1 == argc) throw std::invalid_argument("missing option value");
            const std::string key = argv[i];
            if (key == "--assignment" && assignment.empty()) assignment = argv[i + 1];
            else if (key == "--products-output" && products.empty()) products = argv[i + 1];
            else throw std::invalid_argument("unknown or duplicate option");
        }
        if (std::filesystem::weakly_canonical(argv[1]) == std::filesystem::weakly_canonical(argv[2])) throw std::invalid_argument("refusing to overwrite input");
        if (!products.empty() && (std::filesystem::weakly_canonical(products) == std::filesystem::weakly_canonical(argv[1]) || std::filesystem::weakly_canonical(products) == std::filesystem::weakly_canonical(argv[2]))) throw std::invalid_argument("overlapping input/output paths");
        const System s = read_system(argv[1]);
        std::size_t nonzero = 0, term_count = 0;
        unsigned largest_degree = 0;
        if (!assignment.empty()) {
            const auto values = read_assignment(assignment, s);
            for (const auto &eq : s.equations) if (eq.evaluate(values)) ++nonzero;
            if (nonzero) {
                std::cerr << "assignment fails " << nonzero << " equations; no output written\n";
                return 1;
            }
        }
        for (const auto &eq : s.equations) { term_count += eq.terms.size(); largest_degree = std::max(largest_degree,eq.largest_degree()); }
        write_system(argv[2],s,false);
        if (!products.empty()) write_system(products,s,true);
        const System roundtrip = read_system(argv[2]);
        if (roundtrip.equations.size()!=s.equations.size()) throw std::runtime_error("roundtrip equation count mismatch");
        for (std::size_t i=0;i<s.equations.size();++i)
            if (roundtrip.equations[i].terms != s.equations[i].terms) throw std::runtime_error("roundtrip polynomial mismatch");
        std::cout << "{\"modulus\":" << s.p << ",\"variables\":" << s.names.size() << ",\"equations\":" << s.equations.size() << ",\"collected_terms\":" << term_count << ",\"max_degree\":" << largest_degree << ",\"assignment_checked\":" << (assignment.empty()?"false":"true") << ",\"nonzero_equations\":" << nonzero << ",\"roundtrip_identical\":true}\n";
        return 0;
    } catch (const std::exception &e) {
        std::cerr << e.what() << '\n';
        return 2;
    }
}
