// SPDX-License-Identifier: LGPL-3.0-or-later
#include "backend.hpp"
#include "polytope.hpp"
#include <iostream>
using namespace jsr;
int main(int argc, char **argv) {
  try {
    require(argc == 4,
            "usage structural Singular cdd-helper fresh-output-directory");
    fs::path output = argv[3];
    require(fs::create_directory(output),
            "fresh structural output directory required");
    configure_cdd(argv[2], output / "cdd");
    Order o{{1000, 50, 50, 50, 50, 100, 100}};
    size_t n = 7;
    auto p = Poly::variable(n, 0), x = Poly::variable(n, 1),
         y = Poly::variable(n, 2), h = Poly::variable(n, 5);
    auto c = [&](const Q &q) { return Poly::constant(n, q); };
    require(exact_memory_limit("8589934592") &&
                exact_memory_limit(" 8589934592\n") &&
                !exact_memory_limit("85899345920") &&
                !exact_memory_limit("8589934592x") &&
                !exact_memory_limit("max") && !exact_memory_limit("") &&
                !exact_memory_limit("+8589934592") &&
                !exact_memory_limit("8589934592 1"),
            "exact memory.max codec fixture");
    require(rational("-6/-8") == Q(3, 4) &&
                rational("1/3") + rational("1/6") == Q(1, 2),
            "rational fixture");
    require(exponent("0,1,0,0,0,0,0") == E({0, 1, 0, 0, 0, 0, 0}),
            "tuple codec");
    bool rejected = false;
    try {
      exponent("0,-1");
    } catch (...) {
      rejected = true;
    }
    require(rejected, "negative exponent accepted");
    require(compare(p, power(x, 19), o) > 0 &&
                compare(p, power(x, 20), o) > 0 &&
                compare(power(x, 2), h, o) > 0 && compare(h, x, o) > 0,
            "weighted lex fixture");
    require(compare(scale(p, -1), c(0), o) < 0 &&
                compare(scale(p, -1), c(-1), o) < 0 &&
                compare(p - c(1), p + c(1), o) < 0 &&
                compare(scale(p, -1) + c(2), scale(p, -1) + c(3), o) < 0,
            "signed comparator fixture");
    auto lead = lt_no_p(scale(power(p, 2) * x, 2) + scale(p * x, 3) + y, o);
    require(lead.first == scale(power(p, 2), 2) &&
                lead.second == E({0, 1, 0, 0, 0, 0, 0}),
            "first stripped coefficient fixture");
    auto tri = hull({E{0, 0}, E{2, 0}, E{0, 2}});
    require(tri.points ==
                    std::vector<E>(
                        {{0, 0}, {0, 1}, {0, 2}, {1, 0}, {1, 1}, {2, 0}}) &&
                tri.dimension == 2 && !contains(tri.facets, {1, 2}),
            "triangle membership");
    auto line = hull({E{0, 0}, E{2, 0}});
    require(line.points == std::vector<E>({{0, 0}, {1, 0}, {2, 0}}) &&
                line.dimension == 1,
            "affine equality membership");
    std::vector<std::string> trace;
    Matrix a{{p, c(1)}, {c(1), c(0)}};
    constant_pivot(a, trace);
    require(a == Matrix({{c(1), c(0)}, {c(0), c(1)}}), "constant pivot swap");
    Matrix b{{p, power(p, 2)}, {scale(p, 2), scale(power(p, 2), 2)}},
        before = b;
    constant_pivot(b, trace);
    require(b == before, "nonconstant pivot incorrectly divided");
    Matrix r{{c(1), c(1), p}, {c(0), p, c(1)}, {c(0), p + c(1), c(1)}};
    constant_pivot(r, trace);
    require(r == Matrix({{c(1), c(1) - power(p, 2), c(0)},
                         {c(0), p, c(1)},
                         {c(0), c(1), c(0)}}),
            "pivot refresh/skipped-column fixture");
    Backend backend{argv[1], output / "singular",
                    o,       {"p", "z1", "z2", "z3", "z4", "h1", "h2"},
                    0,       {}};
    auto v = backend.call(
        "emit(lead(p+z1^20)); emit(lead(z1^2+h1)); emit(2/3*p*z1-5/7);",
        "structural-order-codec-only");
    require(v.size() == 3 && v[0] == p && v[1] == power(x, 2) &&
                v[2] == scale(p * x, Q(2, 3)) - c(Q(5, 7)),
            "Singular Wp and exact codec fixture");
    require(parse_codec(v[2].codec(o), n) == v[2], "native codec roundtrip");
    std::string tr;
    for (auto &s : trace)
      tr += s + "\n";
    write(output / "constant-pivot-trace", tr);
    write(output / "passed", "8 sealed fixture groups plus exact memory.max "
                             "codec passed; no scientific call\n");
    std::cout << "8 sealed structural fixture groups plus exact memory.max "
                 "codec passed; no ideal, bound, "
                 "interpolation, control or root computation invoked\n";
    return 0;
  } catch (const std::exception &e) {
    std::cerr << e.what() << "\n";
    return 1;
  }
}
