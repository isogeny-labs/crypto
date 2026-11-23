// Independent certificate reader. Does not invoke or include the searcher.
#include <gmpxx.h>
#include <array>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>
using Z = mpz_class;
void check(bool ok, const char* why) { if (!ok) throw std::runtime_error(why); }
Z modpow(Z a, Z e, const Z& n) {
    Z answer = 1;
    while (e > 0) {
        if (mpz_odd_p(e.get_mpz_t())) answer = answer * a % n;
        a = a * a % n;
        e >>= 1;
    }
    return answer;
}
bool mr(const Z& n) {
    if (n < 2 || n >= (Z(1) << 64)) return false;
    // Independent first-twelve-prime-base deterministic bound exceeds 2^64.
    for (unsigned a : {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}) {
        if (n == a) return true;
        if (n % a == 0) return false;
    }
    Z d = n - 1;
    unsigned s = 0;
    while (mpz_even_p(d.get_mpz_t())) { d >>= 1; ++s; }
    for (unsigned a : {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}) {
        Z x = modpow(Z(a), d, n);
        if (x == 1 || x == n - 1) continue;
        bool pass = false;
        for (unsigned j = 1; j < s; ++j) {
            x = x * x % n;
            if (x == n - 1) { pass = true; break; }
        }
        if (!pass) return false;
    }
    return true;
}
void pocklington(const Z& n, const Z& q, unsigned exponent, const Z& a) {
    check(exponent > 0 && exponent <= 256 && a >= 2 && a <= 128, "certificate range");
    Z f = 1;
    for (unsigned i = 0; i < exponent; ++i) f *= q;
    check((n - 1) % f == 0 && f * f > n, "Pocklington known-factor size/divisibility");
    check(modpow(a, n - 1, n) == 1, "Pocklington Fermat witness");
    Z diff = modpow(a, (n - 1) / q, n) - 1, divisor;
    mpz_gcd(divisor.get_mpz_t(), diff.get_mpz_t(), n.get_mpz_t());
    check(divisor == 1, "Pocklington gcd witness");
}
int main(int argc, char** argv) {
    try {
        check(argc == 3, "usage: size-verify candidates.csv NEW_RECEIPT.json");
        check(mr(Z("18446744073709551557")) && !mr(Z("3825123056546413051")), "independent prime controls");
        std::ifstream input(argv[1]), exists(argv[2]);
        check(bool(input) && !exists, "input unavailable or receipt already exists");
        std::string line;
        std::getline(input, line);
        check(line == "kind,field_bits,abs_d,depth,p,t,ell,r,cofactor,ell_factor_exponent,ell_witness,p_witness,r_witness", "CSV schema");
        std::map<std::tuple<std::string, unsigned, unsigned>, unsigned> cells;
        std::set<std::pair<std::string, std::string>> classes;
        std::set<std::tuple<unsigned, unsigned, std::string>> gaps;
        unsigned native = 0, upper = 0;
        while (std::getline(input, line)) {
            std::vector<std::string> fields;
            std::istringstream row(line);
            std::string token;
            while (std::getline(row, token, ',')) fields.push_back(token);
            check(fields.size() == 13, "CSV column count");
            unsigned b = std::stoul(fields[1]), d = std::stoul(fields[2]), depth = std::stoul(fields[3]);
            Z p(fields[4]), t(fields[5]), ell(fields[6]), r(fields[7]), h(fields[8]);
            check(depth == 4 && mpz_sizeinbase(p.get_mpz_t(), 2) == b, "field/depth size");
            check(t * t - 4 * p == -Z(d) * (16 * ell) * (16 * ell), "Frobenius discriminant");
            check(p + 1 - t == h * r && r * r > 16 * p && ell != r, "order/cofactor/Hasse condition");
            check((t + 2) % ell == 0 && ell * ell * ell * ell > p, "kernel/gap prerequisite");
            Z common;
            mpz_gcd(common.get_mpz_t(), t.get_mpz_t(), p.get_mpz_t());
            check(common == 1, "ordinary trace condition");
            check(classes.insert({p.get_str(), t.get_str()}).second, "duplicate class");
            check(gaps.insert({b, d, ell.get_str()}).second, "duplicate gap within cell");
            if (fields[0] == "native") {
                check(b == 40 || b == 47 || b == 54 || b == 61, "native size cell");
                check(d == 11 || d == 19 || d == 43 || d == 67, "native CM cell");
                check(h >= 1 && h <= 64 && mpz_sizeinbase(r.get_mpz_t(), 2) >= b - 6, "native subgroup size");
                check(mr(p) && mr(r) && mr(ell), "independent deterministic primality");
                for (unsigned k = 9; k < 13; ++k) check(fields[k] == "0", "native certificate fields");
                ++native;
            } else {
                check(fields[0] == "upper" && (b == 112 || b == 160 || b == 256), "upper size cell");
                check((d == 43 || d == 67) && t == -2 && h == 4, "amended upper family");
                check(mpz_sizeinbase(r.get_mpz_t(), 2) == b - 2, "upper subgroup size");
                pocklington(ell, Z(2), std::stoul(fields[9]), Z(fields[10]));
                pocklington(p, ell, 2, Z(fields[11]));
                pocklington(r, ell, 2, Z(fields[12]));
                check(ell > Z("125000000302579"), "upper gap exceeds old construction");
                ++upper;
            }
            ++cells[{fields[0], b, d}];
        }
        check(native == 128 && upper == 6 && cells.size() == 22, "complete planned parameter cohort");
        for (auto& cell : cells) check(cell.second == (std::get<0>(cell.first) == "native" ? 8U : 1U), "cell count");
        std::ofstream receipt(argv[2]);
        receipt << "{\"schema\":\"conductor-gap.parameter-verification/v1\",\"native_classes\":" << native
                << ",\"upper_classes\":" << upper << ",\"complete_cells\":" << cells.size()
                << ",\"native_primality\":\"independent twelve-prime-base deterministic Miller-Rabin below 2^64\","
                << "\"upper_primality\":\"recursive Pocklington witnesses checked with independent modular exponentiation\","
                << "\"curve_models_verified\":0,\"point_maps_verified\":0,\"solver_measurements_verified\":0}\n";
        check(bool(receipt), "receipt write failed");
        std::cout << "Verified " << native << " native and " << upper
                  << " upper parameter classes; 22 complete cells; no model/map/solve claim\n";
        return 0;
    } catch (const std::exception& e) { std::cerr << e.what() << '\n'; return 2; }
}
