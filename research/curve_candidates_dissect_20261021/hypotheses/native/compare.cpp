// Fixed-input arithmetic diagnostic using the native PARI C library and OpenSSL.
// No timing comparison or discrete-log solver is implemented here.
#include <pari/pari.h>
#include <openssl/sha.h>
#include <array>
#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

static void need(bool ok, const char* message) {
    if (!ok) throw std::runtime_error(message);
}
static std::string dec(GEN n) {
    char* s = GENtostr(n); std::string result(s); pari_free(s); return result;
}
static GEN integer(const std::string& s) {
    need(!s.empty() && s.find_first_not_of("0123456789abcdefABCDEFxX") == std::string::npos,
         "invalid integer input");
    return gp_read_str(s.c_str());
}
static GEN from_bytes(const unsigned char* b, size_t n) {
    std::ostringstream s; s << "0x" << std::hex << std::setfill('0');
    for (size_t i = 0; i < n; ++i) s << std::setw(2) << unsigned(b[i]);
    return integer(s.str());
}
static std::array<unsigned char, 20> bytes20(GEN n) {
    std::array<unsigned char, 20> out{};
    for (int i = 19; i >= 0; --i) { out[i] = umodiu(n, 256); n = shifti(n, -8); }
    need(signe(n) == 0, "integer exceeds 20 bytes"); return out;
}
static GEN x962(GEN seed, long bits) {
    const long s = (bits - 1) / 160, v = bits - 160 * s;
    auto block = bytes20(seed); unsigned char h[20];
    SHA1(block.data(), block.size(), h);
    GEN r = modii(from_bytes(h, 20), int2n(v - 1));
    for (long i = 1; i <= s; ++i) {
        block = bytes20(modii(addis(seed, i), int2n(160)));
        SHA1(block.data(), block.size(), h);
        r = addii(shifti(r, 160), from_bytes(h, 20));
    }
    return r;
}
static GEN stream_seed(uint32_t run, uint32_t index) {
    need(index > 0, "candidate indices are one based");
    uint64_t state = (uint64_t(run) << 16) | 0x330e;
    std::array<unsigned char, 20> block{};
    for (uint32_t i = 1; i <= index; ++i) for (auto& b : block) {
        state = (state * UINT64_C(0x5deece66d) + 0xb) & ((UINT64_C(1) << 48) - 1);
        b = state >> 40;
    }
    return from_bytes(block.data(), block.size());
}
struct Factors {
    GEN square_root, largest;
    std::string json;
};
static Factors proven_factors(GEN n) {
    need(signe(n) > 0, "positive factorization target required");
    GEN f = factorint(n, 0), product = gen_1, square = gen_1, largest = gen_1;
    std::ostringstream out; out << '[';
    for (long i = 1; i < lg(gel(f, 1)); ++i) {
        GEN prime = gcoeff(f, i, 1); long e = itos(gcoeff(f, i, 2));
        need(isprime(prime), "factor is not a proven prime");
        product = mulii(product, powiu(prime, e));
        square = mulii(square, powiu(prime, e / 2));
        if (cmpii(prime, largest) > 0) largest = prime;
        if (i > 1) out << ',';
        out << "[\"" << dec(prime) << "\"," << e << ']';
    }
    need(equalii(product, n), "factor product mismatch"); out << ']';
    return {square, largest, out.str()};
}
int main(int argc, char** argv) {
    try {
        need(argc == 3, "usage: compare inputs.tsv zero_based_row");
        pari_init(256000000, 2000000);
        setrand(gen_1);
        std::ifstream file(argv[1]); need(bool(file), "cannot open input table");
        std::string line; int wanted = std::stoi(argv[2]), current = 0;
        need(wanted >= 0, "row must be nonnegative");
        while (std::getline(file, line)) {
            if (line.empty() || line[0] == '#') continue;
            if (current++ != wanted) continue;
            std::vector<std::string> row; std::istringstream fields(line); std::string field;
            while (std::getline(fields, field, '\t')) row.push_back(field);
            need(row.size() == 12, "expected twelve input columns");
            long bits = std::stol(row[1]); uint32_t run = std::stoul(row[3]), index = std::stoul(row[4]);
            GEN p = integer(row[5]), b = integer(row[6]), gx = integer(row[7]), gy = integer(row[8]);
            GEN n = integer(row[9]), seed = integer(row[10]), a = subis(p, 3);
            std::cerr << row[0] << ": prime and seed checks\n";
            need(isprime(p) && isprime(n), "field or subgroup order is not proven prime");
            need(equalii(seed, stream_seed(run, index)), "seed does not match claimed run/index");
            need(!equalii(seed, stream_seed(run, index + 1)), "adjacent-block negative control failed");
            need(!equalii(seed, stream_seed(run + 1, index)), "adjacent-run negative control failed");
            GEN r = x962(seed, bits);
            need(equalii(modii(mulii(r, sqri(b)), p), modii(powiu(a, 3), p)), "X9.62 seed relation failed");
            need(signe(modii(addii(mulsi(4, powiu(a, 3)), mulsi(27, sqri(b))), p)) != 0,
                 "singular curve");
            std::cerr << row[0] << ": generator and order checks\n";
            GEN e = ellinit(mkvec2(a, b), p, DEFAULTPREC), point = mkvec2(gx, gy);
            need(!gequal0(ellisoncurve(e, point)), "supplied generator is off curve");
            GEN killed = ellmul(e, point, n);
            need(lg(killed) == 2 && gequal0(gel(killed, 1)), "nG is not infinity");
            GEN bound = sqrtint(shifti(p, 2)), center = addis(p, 1);
            GEN low = addis(divii(subis(subii(center, bound), 1), n), 1);
            GEN high = divii(addii(center, bound), n);
            need(equali1(low) && equali1(high), "Hasse interval does not uniquely certify order n");
            GEN trace = subii(center, n), disc = subii(sqri(trace), shifti(p, 2));
            need(!equalii(n, p), "anomalous order");
            GEN power = gen_1;
            for (long k = 1; k < 1024; ++k) {
                power = modii(mulii(power, p), n);
                need(!equali1(power), "embedding degree below 1024");
            }
            std::cerr << row[0] << ": discriminant factorization\n";
            const Factors d = proven_factors(absi(disc));
            GEN field_disc = diviiexact(disc, sqri(d.square_root));
            need(umodiu(field_disc, 4) == 1, "unsupported even fundamental discriminant");
            std::cerr << row[0] << ": twist factorization\n";
            const Factors twist = proven_factors(subii(shifti(center, 1), n));
            std::cerr << row[0] << ": 3-division polynomial\n";
            GEN psi = liftall(elldivpol(e, 3, 0)), poly_factors = FpX_factor(psi, p);
            long linear = 0, total_degree = 0;
            std::ostringstream degrees, characters; degrees << '['; characters << '[';
            for (long i = 1; i < lg(gel(poly_factors, 1)); ++i) {
                long degree = degpol(gcoeff(poly_factors, i, 1));
                GEN exponents = gel(poly_factors, 2);
                long multiplicity = typ(exponents) == t_VECSMALL ? exponents[i] : itos(gel(exponents, i));
                need(multiplicity == 1, "unexpected repeated division-polynomial factor");
                total_degree += degree;
                if (degree == 1) {
                    GEN polynomial = gcoeff(poly_factors, i, 1);
                    GEN x = Fp_div(negi(gel(polynomial, 2)), gel(polynomial, 3), p);
                    GEN y2 = modii(addii(subii(powiu(x, 3), mulsi(3, x)), b), p);
                    if (linear++) characters << ',';
                    characters << kronecker(y2, p);
                }
                if (i > 1) degrees << ','; degrees << degree;
            }
            need(total_degree == 4, "3-division polynomial degree mismatch");
            degrees << ']'; characters << ']';
            std::cerr << row[0] << ": adjacent-order factorizations\n";
            const Factors minus = proven_factors(subis(n, 1)), plus = proven_factors(addis(n, 1));
            std::cout << std::setprecision(15)
              << "{\"label\":\"" << row[0] << "\",\"role\":\"" << row[2]
              << "\",\"field_bits\":" << bits << ",\"run\":" << run << ",\"candidate\":" << index
              << ",\"icv1\":\"" << row[11] << "\",\"field_and_order_prime_proven\":true"
              << ",\"stream_seed_matches\":true,\"adjacent_seed_controls_pass\":true"
              << ",\"seed_relation_matches\":true,\"full_order_certified_by_generator_and_Hasse\":true"
              << ",\"embedding_degree_at_least\":1024,\"trace\":\"" << dec(trace)
              << "\",\"frobenius_discriminant\":\"" << dec(disc)
              << "\",\"frobenius_discriminant_factors\":" << d.json
              << ",\"frobenius_order_conductor\":\"" << dec(d.square_root)
              << "\",\"fundamental_field_discriminant\":\"" << dec(field_disc)
              << "\",\"twist_order\":\"" << dec(subii(shifti(center, 1), n))
              << "\",\"twist_factors\":" << twist.json
              << ",\"largest_twist_prime_bits\":" << (expi(twist.largest) + 1)
              << ",\"generic_twist_work_log2_approx\":" << std::log2(gtodouble(twist.largest)) / 2
              << ",\"psi3_factor_degrees\":" << degrees.str()
              << ",\"psi3_rational_x_count\":" << linear
              << ",\"psi3_rational_x_y_square_characters\":" << characters.str()
              << ",\"n_minus_1_factors\":" << minus.json
              << ",\"n_minus_1_largest_prime_bits\":" << (expi(minus.largest) + 1)
              << ",\"n_plus_1_factors\":" << plus.json
              << ",\"n_plus_1_largest_prime_bits\":" << (expi(plus.largest) + 1) << "}\n";
            pari_close(); return 0;
        }
        throw std::runtime_error("input row not found");
    } catch (const std::exception& e) {
        std::cerr << "ERROR: " << e.what() << '\n'; return 1;
    }
}
