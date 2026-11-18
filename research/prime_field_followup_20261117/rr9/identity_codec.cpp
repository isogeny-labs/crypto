#define main prior_cli
#include "prior_verify.cpp"
#undef main

constexpr unsigned alphabet_size = 3432;
constexpr unsigned support_bytes = alphabet_size / 8;

void demand(bool condition, const char* message) {
    if (!condition) throw std::runtime_error(message);
}

void monomial_level(std::vector<Exp>& out, unsigned variable, unsigned free_degree, Exp prefix) {
    if (variable == 7) {
        out.push_back(prefix);
        return;
    }
    for (unsigned degree = 0; degree <= free_degree; ++degree)
        monomial_level(out, variable + 1, free_degree - degree, prefix | (degree << (4 * variable)));
}

std::vector<Exp> monomials() {
    std::vector<Exp> out;
    monomial_level(out, 0, 7, 0);
    demand(out.size() == alphabet_size, "incorrect monomial count");
    return out;
}

void put_word(std::ostream& out, std::uint32_t v) {
    for (int shift = 0; shift < 32; shift += 8)
        out.put(static_cast<char>((v >> shift) & 255));
}

std::uint8_t get_byte(std::istream& in) {
    int v = in.get();
    demand(v != EOF, "truncated identity");
    return static_cast<std::uint8_t>(v);
}

std::uint32_t get_word(std::istream& in) {
    std::uint32_t v = 0;
    for (int shift = 0; shift < 32; shift += 8)
        v |= std::uint32_t(get_byte(in)) << shift;
    return v;
}

std::array<Poly, 8> parse(std::istream& in, const std::array<Poly, 8>& expected) {
    std::array<Poly, 8> multipliers;
    std::array<bool, 8> equations_seen{}, multipliers_seen{};
    for (std::string line; std::getline(in, line);) {
        if (line.size() < 4 || line[2] != ' ' || (line[0] != 'Q' && line[0] != 'W')) continue;
        unsigned k = line[1] - '1';
        demand(k < 8, "polynomial index out of range");
        auto poly = read_poly(line.substr(3));
        if (line[0] == 'Q') {
            demand(!equations_seen[k] && poly == expected[k], "equation differs");
            equations_seen[k] = true;
        } else {
            demand(!multipliers_seen[k], "duplicate multiplier");
            multipliers_seen[k] = true;
            multipliers[k] = std::move(poly);
        }
    }
    for (unsigned k = 0; k < 8; ++k)
        demand(equations_seen[k] && multipliers_seen[k], "missing equation or multiplier");
    return multipliers;
}

void encode(std::istream& source, std::ostream& target, const std::array<Poly, 8>& equations) {
    auto multipliers = parse(source, equations);
    auto basis = monomials();
    target.write("RRI1", 4);
    for (const auto& poly : multipliers) {
        std::array<unsigned char, support_bytes> support{};
        std::vector<Int> values;
        for (unsigned i = 0; i < basis.size(); ++i) {
            if (auto it = poly.find(basis[i]); it != poly.end()) {
                support[i / 8] |= (1u << (i % 8));
                values.push_back(it->second);
            }
        }
        demand(values.size() == poly.size(), "multiplier exceeds degree seven");
        target.write(reinterpret_cast<const char*>(support.data()), support_bytes);
        for (Int c : values) put_word(target, c);
    }
    target.flush();
    demand(bool(target), "could not write encoded identity");
}

void verify(std::istream& in, const std::array<Poly, 8>& equations) {
    char magic[4];
    in.read(magic, 4);
    demand(in.gcount() == 4 && std::string(magic, 4) == "RRI1", "wrong identity header");
    auto basis = monomials();
    Poly equality;
    std::uint64_t term_count = 0;
    for (const auto& eq : equations) {
        std::array<std::uint8_t, support_bytes> support;
        for (auto& octet : support) octet = get_byte(in);
        Poly multiplier;
        for (unsigned i = 0; i < basis.size(); ++i) {
            if (!(support[i / 8] & (1u << (i % 8)))) continue;
            auto value = get_word(in);
            demand(value > 0 && value < p, "noncanonical multiplier coefficient");
            multiplier.emplace(basis[i], value);
        }
        term_count += multiplier.size();
        scaled_add(equality, multiply(multiplier, eq), 1);
    }
    demand(in.peek() == EOF, "trailing bytes");
    demand(equality == constant(1), "expanded identity did not replay to 1");
    std::cout << "verified multiplier terms=" << term_count << '\n';
}

int main(int argc, char** argv) {
    try {
        demand(argc == 6 || argc == 7, "usage: identity_codec encode|verify x y presentation input [output]");
        std::string mode = argv[1];
        Point target{std::stoll(argv[2]), std::stoll(argv[3])};
        auto [x, y] = target;
        demand(x >= 0 && x < p && y >= 0 && y < p, "noncanonical point");
        demand(mod(y*y) == mod(mod(x*x)*x + mod(curve_a*x) + curve_b), "off-curve point");
        int presentation = std::stoi(argv[4]);
        demand(presentation == 0 || presentation == 1, "unknown presentation");
        auto eq = equations(target, presentation == 1);
        std::ifstream in(argv[5], std::ios::binary);
        demand(bool(in), "missing input");
        if (mode == "encode") {
            demand(argc == 7, "missing output");
            std::ofstream out(argv[6], std::ios::binary);
            encode(in, out, eq);
        } else {
            demand(argc == 6 && mode == "verify", "unknown mode");
            verify(in, eq);
        }
        return 0;
    } catch (const std::exception& e) {
        std::cerr << e.what() << '\n';
        return 1;
    }
}
