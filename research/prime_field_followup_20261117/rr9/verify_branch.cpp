#define main prior_cli
#include "prior_verify.cpp"
#undef main
#include <bitset>

struct Invalid : std::runtime_error { using std::runtime_error::runtime_error; };

void check(bool ok, const char* detail) {
    if (!ok) throw Invalid(detail);
}

unsigned byte(std::istream& in) {
    int value = in.get();
    check(value != EOF, "truncated certificate");
    return static_cast<unsigned>(value);
}

std::uint32_t word(std::istream& in) {
    std::uint32_t value = 0;
    for (unsigned shift = 0; shift < 32; shift += 8)
        value |= std::uint32_t(byte(in)) << shift;
    return value;
}

Int coeff(std::istream& in) {
    auto value = word(in);
    check(value < p, "noncanonical field coefficient");
    return value;
}

Int value_at(const Poly& f, const std::array<Int, 7>& point) {
    Int result = 0;
    for (auto [e, c] : f) {
        for (int v = 0; v < 7; ++v) c = mod(c * power(point[v], (e >> (4 * v)) & 15));
        result = mod(result + c);
    }
    return result;
}

using Forms = std::array<std::array<Poly, 2>, 8>;

Forms factor(Point target, const std::array<Point, 8>& base, const std::array<Poly, 8>& eq) {
    Forms result;
    for (std::size_t i = 0; i < base.size(); ++i) {
        auto [x, y] = base[i];
        auto d = mod(x - target[0]);
        check(d != 0, "unsupported coinciding abscissa");
        for (int bit = 0; bit < 2; ++bit) {
            Int c = mod(target[1] + (bit ? -y : y));
            Poly& f = result[i][bit];
            for (int j = 0; j < 4; ++j) {
                scaled_add(f, variable(j), mod(d * power(x, j)));
                scaled_add(f, j < 3 ? variable(4 + j) : constant(1), mod(c * power(x, j)));
            }
        }
        auto product = multiply(result[i][0], result[i][1]);
        scaled_add(product, eq[i], d);
        check(product.empty(), "root evaluation quadric did not match factored affine equation");
    }
    return result;
}

int run(int argc, char** argv) {
    check(argc == 5, "usage: verify cert x y repeats");
    std::ifstream in(argv[1], std::ios::binary);
    check(bool(in), "could not open certificate");
    char magic[4];
    in.read(magic, 4);
    check(in.gcount() == 4 && std::string(magic, 4) == "RRB1", "invalid certificate header");
    unsigned kind = byte(in);
    check(kind <= 1, "invalid certificate kind");
    Point target{std::stoll(argv[2]), std::stoll(argv[3])};
    auto [r,s] = target;
    check(r >= 0 && s >= 0 && r < p && s < p, "noncanonical residual");
    check(mod(s*s) == mod(mod(r*r)*r + mod(curve_a*r) + curve_b), "residual is off curve");
    auto base = base_points();
    auto eq = equations(target, 1);
    auto forms = factor(target, base, eq);
    if (kind == 1) {
        unsigned mask = byte(in);
        std::array<Int, 7> solution;
        for (auto& entry : solution) entry = coeff(in);
        for (const auto& f : eq) check(value_at(f, solution) == 0, "positive chart residual");
        Point sum = infinity;
        for (std::size_t i = 0; i < base.size(); ++i) {
            unsigned bit = (mask >> i) & 1;
            check(value_at(forms[i][bit], solution) == 0, "positive branch residual");
            sum = add(sum, bit ? negate(base[i]) : base[i]);
        }
        check(sum == target, "positive fails group law replay");
        std::cout << "chart";
        for (Int v : solution) std::cout << ' ' << v;
        std::cout << '\n';
    } else {
        auto leaves = word(in);
        check(leaves <= 256, "too many proof leaves");
        std::bitset<256> covered;
        for (unsigned i = 0; i < leaves; ++i) {
            unsigned depth = byte(in), prefix = byte(in);
            check(depth >= 1 && depth <= 8 && prefix < (1u << depth), "invalid branch prefix");
            Poly replay;
            for (unsigned j = 0; j < 8; ++j) {
                Int c = coeff(in);
                check(j < depth || c == 0, "certificate uses a factor outside the prefix");
                if (c) scaled_add(replay, forms[j][(prefix >> j) & 1], c);
            }
            check(replay == constant(1), "affine combination is not 1");
            for (unsigned mask = prefix; mask < covered.size(); mask += 1u << depth) {
                check(!covered.test(mask), "overlapping refutation prefixes");
                covered.set(mask);
            }
        }
        check(covered.all(), "branch cover incomplete");
        std::cout << "verified branches=" << covered.count() << " leaves=" << leaves << '\n';
    }
    check(in.peek() == EOF, "trailing certificate data");

    unsigned runs = std::stoul(argv[4]);
    check(runs > 0, "repeat count must be positive");
    std::vector<double> hot, cold;
    for (unsigned i = 0; i < runs; ++i) {
        using Clock = std::chrono::steady_clock;
        auto start = Clock::now();
        auto lifted = base_points();
        auto midpoint = Clock::now();
        bool generic = mitm(target, lifted);
        auto done = Clock::now();
        check(generic == bool(kind), "group-law checker disagrees");
        hot.push_back(std::chrono::duration<double, std::micro>(done-midpoint).count());
        cold.push_back(std::chrono::duration<double, std::micro>(done-start).count());
    }
    std::sort(hot.begin(), hot.end());
    std::sort(cold.begin(), cold.end());
    std::cout << "generic_lift_search_median_us=" << cold[runs/2]
              << " generic_search_median_us=" << hot[runs/2] << '\n';
    return 0;
}

int main(int argc, char** argv) {
    try { return run(argc, argv); }
    catch (const Invalid& e) { std::cerr << e.what() << '\n'; return 1; }
}
