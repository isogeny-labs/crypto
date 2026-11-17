#include "common.hpp"
#include <fstream>
#include <iostream>
#include <map>
#include <regex>

static GEN signed_integer(const std::string& value) {
    if (!value.empty() && value[0] == '-') return negi(integer(value.substr(1)));
    return integer(value);
}

int main(int argc, char** argv) {
  try {
    require(argc == 5, "usage: check_polynomials residual.txt thread.p new_master.p new_check.json");
    require(!std::ifstream(argv[3]) && !std::ifstream(argv[4]), "refusing to overwrite polynomial admission");
    pari_init(256000000, 2000000);
    std::ifstream input(argv[1]), candidates(argv[2]);
    require(bool(input) && bool(candidates), "missing polynomial input");
    std::string number, extra;
    input >> number;
    require(!number.empty() && number.find_first_not_of("0123456789") == std::string::npos,
            "invalid residual integer");
    require(!(input >> extra), "multiple residuals");
    GEN n = integer(number);
    require(cmpis(n, 2) > 0 && !isprime(n), "expected a composite residual");
    std::ostringstream accepted;
    accepted << "n: " << decimal(n) << '\n';
    std::string line, block;
    std::map<std::string, GEN> coefficients;
    bool skew = false;
    long valid = 0, partial = 0;
    const std::regex coefficient(R"rx(^(c[0-5]|Y[01]):\s*(-?[0-9]+)\s*$)rx");
    const std::regex skew_line(R"rx(^skew:\s*[0-9]+(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?\s*$)rx");
    auto flush = [&](bool at_end) {
        if (block.empty()) return;
        if (coefficients.size() != 8 || !skew) {
            require(at_end, "incomplete polynomial before end of source");
            ++partial;
        } else {
            GEN y1 = coefficients.at("Y1"), y0 = coefficients.at("Y0");
            GEN divisor = gcdii(y1, n);
            if (!equali1(divisor) && cmpii(divisor, n) < 0)
                std::cerr << "Factor found in step 1: " << decimal(divisor) << '\n';
            require(equali1(divisor), "polynomial rational leading coefficient is not invertible");
            GEN root = Fp_div(negi(y0), y1, n), value = gen_0;
            for (int i = 5; i >= 0; --i)
                value = modii(addii(mulii(value, root), coefficients.at("c" + std::to_string(i))), n);
            require(signe(value) == 0, "complete polynomial has wrong modular root");
            accepted << block;
            ++valid;
        }
        block.clear(); coefficients.clear(); skew = false;
    };
    while (std::getline(candidates, line)) {
        require(line.size() < 10000, "polynomial line too long");
        if (!line.empty() && line.back() == '\r') line.pop_back();
        if (line.empty()) continue;
        if (line[0] == '#') {
            flush(false);
            require(line.rfind("# norm ", 0) == 0, "unexpected polynomial header");
            block = line + '\n';
            continue;
        }
        require(!block.empty(), "coefficient without polynomial header");
        std::smatch match;
        if (std::regex_match(line, match, coefficient)) {
            require(!coefficients.count(match[1]), "duplicate polynomial coefficient");
            require(match[2].length() < 1000, "polynomial coefficient too large");
            coefficients.emplace(match[1], signed_integer(match[2]));
        } else if (std::regex_match(line, skew_line)) {
            require(!skew, "duplicate polynomial skew");
            skew = true;
        } else throw std::runtime_error("unrecognized polynomial line");
        block += line + '\n';
        require(block.size() < 100000, "polynomial block too large");
    }
    flush(true);
    require(valid > 0, "no complete valid polynomial candidates");
    std::ofstream master(argv[3]), report(argv[4]);
    require(bool(master) && bool(report), "cannot write polynomial admission");
    master << accepted.str();
    report << "{\"kind\":\"native-polynomial-input-check/v1\",\"target_decimal\":\"" << decimal(n)
           << "\",\"target_bits\":" << bit_length(n) << ",\"target_is_prime\":false,\"complete_validated_blocks\":"
           << valid << ",\"discarded_incomplete_trailing_blocks\":" << partial
           << ",\"all_complete_modular_roots_checked\":true}\n";
    require(bool(master) && bool(report), "polynomial admission write failed");
    pari_close();
    return 0;
  } catch (const std::exception& e) {
    std::cerr << "ERROR: " << e.what() << '\n';
    return 1;
  }
}
