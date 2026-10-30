#pragma once
#include "native_sections.hpp"

namespace cubic_native {
using SevenPolynomial = PrimePolynomial<7>;
inline std::string six_string(const SixPolynomial &p) {
  std::ostringstream s;
  bool first = true;
  for (auto &[e, a] : p) {
    if (!first) {
      s << "+";
    }
    first = false;
    s << a;
    for (int j = 0; j < 6; j++) {
      if (e[j]) {
        s << "*" << (j % 2 ? "y" : "x") << (j / 2 + 1) << "^" << e[j];
      }
    }
  }
  return first ? "0" : s.str();
}
inline F seven_value(const SevenPolynomial &p, const std::array<F, 7> &v) {
  F out;
  for (auto &[e, a] : p) {
    F term{long(a)};
    for (int j = 0; j < 7; j++) {
      term = term * v[j].pow(e[j]);
    }
    out += term;
  }
  return out;
}
struct Inversion {
  std::vector<std::array<V3, 3>> divisors;
  size_t groebner_length = 0, lex_length = 0, ordered_solutions = 0;
};
inline void triangular_solutions(const std::vector<SevenPolynomial> &gb,
                                 int variable, std::array<F, 7> &v,
                                 std::vector<std::array<F, 7>> &out,
                                 size_t &nodes) {
  require(++nodes <= 4096, "triangular inversion node budget");
  if (variable < 0) {
    for (auto &p : gb) {
      require(!seven_value(p, v), "lex solution violates its certificate");
    }
    out.push_back(v);
    require(out.size() <= 64, "inversion solution budget");
    return;
  }
  std::vector<Univariate> constraints;
  for (auto &p : gb) {
    bool eligible = true;
    Univariate polynomial;
    for (auto &[e, a] : p) {
      for (int j = 0; j < variable; j++) {
        if (e[j]) {
          eligible = false;
          break;
        }
      }
      if (!eligible) {
        break;
      }
      F value{long(a)};
      for (int j = variable + 1; j < 7; j++) {
        value = value * v[j].pow(e[j]);
      }
      if (polynomial.size() <= size_t(e[variable])) {
        polynomial.resize(e[variable] + 1);
      }
      polynomial[e[variable]] += value;
    }
    if (!eligible) {
      continue;
    }
    trim(polynomial);
    if (polynomial.empty()) {
      continue;
    }
    if (polynomial.size() == 1) {
      return;
    }
    constraints.push_back(polynomial);
  }
  require(!constraints.empty(),
          "zero-dimensional lex fiber lacks a triangular equation");
  auto best = std::min_element(
      constraints.begin(), constraints.end(),
      [](const auto &a, const auto &b) { return a.size() < b.size(); });
  for (auto &root : roots(*best)) {
    bool good = true;
    for (auto &polynomial : constraints) {
      F value;
      for (auto it = polynomial.rbegin(); it != polynomial.rend(); ++it) {
        value = value * root + *it;
      }
      if (value) {
        good = false;
        break;
      }
    }
    if (!good) {
      continue;
    }
    v[variable] = root;
    triangular_solutions(gb, variable - 1, v, out, nodes);
  }
}
inline Inversion
invert_coordinates(const Construction &c, const SymmetricSections &s,
                   const std::array<uint64_t, 8> &coordinates) {
  size_t pivot = 0;
  while (pivot < 8 && !coordinates[pivot]) {
    pivot++;
  }
  require(pivot < 8, "zero inversion coordinates");
  std::ostringstream input;
  input << "ring rinv=" << FP << ",(x1,y1,x2,y2,x3,y3,t),dp; option(redSB);\n";
  for (int i = 0; i < 8; i++) {
    input << "poly z" << i << "=" << six_string(s.polynomials[i]) << ";\n";
  }
  input << "ideal J=";
  for (int j = 0; j < 3; j++) {
    if (j) {
      input << ",";
    }
    bool first = true;
    for (auto &[e, a] : c.quartic.terms) {
      if (!first) {
        input << "+";
      }
      first = false;
      input << a.constant();
      if (e[0]) {
        input << "*x" << (j + 1) << "^" << e[0];
      }
      if (e[1]) {
        input << "*y" << (j + 1) << "^" << e[1];
      }
    }
  }
  for (int i = 0; i < 8; i++) {
    if (size_t(i) == pivot) {
      continue;
    }
    input << "," << coordinates[pivot] << "*z" << i << "-" << coordinates[i]
          << "*z" << pivot;
  }
  input
      << ",t*z" << pivot
      << "-1;\n"
         "ideal G=slimgb(J); print(\"CUBIC_INV_DIM "
         "\"+string(dim(G)));print(\"CUBIC_INV_LENGTH \"+string(size(G)));\n"
         "if(dim(G)==0){ring rlex="
      << FP
      << ",(x1,y1,x2,y2,x3,y3,t),lp;ideal H=fglm(rinv,G);poly h;int i,j;intvec "
         "e;for(i=1;i<=size(H);i++){print(\"CUBIC_INV_POLY\");h=H[i];while(h!="
         "0){e=leadexp(h);string "
         "line=\"CUBIC_INV_TERM\";for(j=1;j<=7;j++){line=line+\" "
         "\"+string(e[j]);}print(line+\" "
         "\"+string(leadcoef(h)));h=h-lead(h);}print(\"CUBIC_INV_END\");}}"
         "print(\"CUBIC_INV_DONE\");quit;\n";
  std::string answer = singular_run(input.str());
  require(answer.find('?') == std::string::npos &&
              answer.find("CUBIC_INV_DONE") != std::string::npos,
          "native polynomial inversion failed: " + answer.substr(0, 1600));
  require(answer.find("CUBIC_INV_DIM 0\n") != std::string::npos,
          "inversion fiber is not zero-dimensional");
  std::vector<SevenPolynomial> gb;
  SevenPolynomial current;
  std::istringstream lines(answer);
  std::string line;
  Inversion out;
  while (std::getline(lines, line)) {
    if (line.rfind("CUBIC_INV_LENGTH ", 0) == 0) {
      out.groebner_length = std::stoull(line.substr(17));
    } else if (line == "CUBIC_INV_POLY") {
      current.clear();
    } else if (line == "CUBIC_INV_END") {
      gb.push_back(current);
    } else if (line.rfind("CUBIC_INV_TERM ", 0) == 0) {
      std::istringstream row(line);
      std::string tag;
      row >> tag;
      std::array<int, 7> e{};
      for (auto &power : e) {
        row >> power;
        require(power >= 0 && power <= 256, "lex exponent bound");
      }
      long coefficient = 0;
      row >> coefficient;
      require(bool(row), "malformed prime-field lex coefficient");
      prime_add(current, e,
                uint64_t((coefficient % long(FP) + long(FP)) % long(FP)));
    }
  }
  require(!gb.empty(), "empty lex certificate");
  out.lex_length = gb.size();
  std::array<F, 7> values{};
  std::vector<std::array<F, 7>> solutions;
  size_t nodes = 0;
  triangular_solutions(gb, 6, values, solutions, nodes);
  require(!solutions.empty(),
          "no inverse divisor in the degree-six splitting field");
  out.ordered_solutions = solutions.size();
  std::set<std::vector<std::vector<uint64_t>>> seen;
  for (auto &solution : solutions) {
    std::array<V3, 3> points;
    std::vector<std::vector<uint64_t>> key;
    for (int j = 0; j < 3; j++) {
      points[j] = {solution[2 * j], solution[2 * j + 1], F(1)};
      require(!c.quartic.eval(points[j]),
              "recovered inverse support is off the quartic");
      for (int k = 0; k < 2; k++) {
        require(points[j][k].frob(6) == points[j][k],
                "inverse support is outside the degree-six splitting field");
      }
      key.push_back(point_key(points[j]));
    }
    auto z = section_values(s, points);
    require(bool(z[pivot]) && solution[6] * z[pivot] == F(1),
            "inverse saturation identity failed");
    for (int i = 0; i < 8; i++) {
      require(F(long(coordinates[pivot])) * z[i] ==
                  F(long(coordinates[i])) * z[pivot],
              "recovered divisor does not map to the requested coordinates");
    }
    std::sort(key.begin(), key.end());
    if (seen.insert(key).second) {
      out.divisors.push_back(points);
    }
  }
  require(out.divisors.size() <= 2,
          "Kummer inverse has more than two effective divisors");
  return out;
}
} // namespace cubic_native
