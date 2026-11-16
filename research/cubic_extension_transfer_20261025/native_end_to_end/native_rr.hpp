#pragma once
#include "native_construction.hpp"
#include <cctype>

namespace cubic_native {
using Univariate = std::vector<F>;
inline void trim(Univariate &a) {
  while (!a.empty() && !a.back())
    a.pop_back();
}
inline Univariate poly_add(Univariate a, const Univariate &b) {
  if (a.size() < b.size())
    a.resize(b.size());
  for (size_t i = 0; i < b.size(); i++)
    a[i] += b[i];
  trim(a);
  return a;
}
inline Univariate poly_scale(Univariate a, const F &c) {
  for (auto &x : a)
    x = x * c;
  trim(a);
  return a;
}
inline Univariate poly_mul(const Univariate &a, const Univariate &b) {
  if (a.empty() || b.empty())
    return {};
  Univariate c(a.size() + b.size() - 1);
  for (size_t i = 0; i < a.size(); i++)
    for (size_t j = 0; j < b.size(); j++)
      c[i + j] += a[i] * b[j];
  trim(c);
  return c;
}
inline std::pair<Univariate, Univariate> poly_divmod(Univariate a,
                                                     const Univariate &b) {
  require(!b.empty(), "zero polynomial divisor");
  trim(a);
  Univariate q(a.size() >= b.size() ? a.size() - b.size() + 1 : 0);
  while (!a.empty() && a.size() >= b.size()) {
    size_t d = a.size() - b.size();
    F c = a.back() / b.back();
    q[d] += c;
    for (size_t j = 0; j < b.size(); j++)
      a[d + j] = a[d + j] - c * b[j];
    trim(a);
  }
  trim(q);
  return {q, a};
}
inline Univariate poly_mod(const Univariate &a, const Univariate &m) {
  return poly_divmod(a, m).second;
}
inline Univariate poly_mulmod(const Univariate &a, const Univariate &b,
                              const Univariate &m) {
  return poly_mod(poly_mul(a, b), m);
}
inline Univariate eval_mod(const Polynomial &p, const Univariate &x,
                           const Univariate &y, const Univariate &mod) {
  std::array<std::vector<Univariate>, 2> power;
  power[0] = {{F(1)}};
  power[1] = {{F(1)}};
  int dx = 0, dy = 0;
  for (auto &[e, c] : p.terms) {
    (void)c;
    dx = std::max(dx, e[0]);
    dy = std::max(dy, e[1]);
  }
  for (int i = 1; i <= dx; i++) {
    power[0].push_back(poly_mulmod(power[0].back(), x, mod));
  }
  for (int i = 1; i <= dy; i++) {
    power[1].push_back(poly_mulmod(power[1].back(), y, mod));
  }
  Univariate out;
  for (auto &[e, c] : p.terms)
    out = poly_add(
        out, poly_scale(poly_mulmod(power[0][e[0]], power[1][e[1]], mod), c));
  return poly_mod(out, mod);
}
inline std::vector<uint64_t> point_key(const V3 &p) {
  std::vector<uint64_t> k;
  for (auto &x : p) {
    auto c = x.coeffs();
    k.insert(k.end(), c.begin(), c.end());
  }
  return k;
}
struct EffectiveDivisor {
  std::map<std::vector<uint64_t>, Contact> support;
  void add(V3 point, int m) {
    point = projective(point);
    auto key = point_key(point);
    auto it = support.find(key);
    if (it == support.end()) {
      require(m > 0, "negative new divisor component");
      support.emplace(key, Contact{point, m});
    } else {
      it->second.multiplicity += m;
      require(it->second.multiplicity >= 0, "negative divisor multiplicity");
      if (!it->second.multiplicity)
        support.erase(it);
    }
  }
  void add(const std::vector<Contact> &points) {
    for (auto &p : points)
      add(p.point, p.multiplicity);
  }
  int degree() const {
    int d = 0;
    for (auto &[key, c] : support) {
      (void)key;
      d += c.multiplicity;
    }
    return d;
  }
};
inline EffectiveDivisor infinity_divisor(const Construction &c) {
  EffectiveDivisor H;
  H.add({F(1), F(0), F(0)}, 2);
  Univariate q{c.quartic.coefficient({0, 4, 0}),
               c.quartic.coefficient({1, 3, 0}),
               c.quartic.coefficient({2, 2, 0})};
  for (auto &t : roots(q)) {
    int m = (F(2) * q[2] * t + q[1]) ? 1 : 2;
    H.add({t, F(1), F(0)}, m);
  }
  require(H.degree() == 4, "canonical tangent section did not split");
  return H;
}
using Series = std::vector<F>;
inline Series series_mul(const Series &a, const Series &b) {
  Series out(a.size());
  for (size_t i = 0; i < a.size(); i++)
    for (size_t j = 0; i + j < a.size(); j++)
      out[i + j] += a[i] * b[j];
  return out;
}
inline Series series_pow(const Series &a, int n) {
  Series r(a.size());
  r[0] = 1;
  for (int i = 0; i < n; i++)
    r = series_mul(r, a);
  return r;
}
inline Series series_monomial(const std::array<Series, 3> &s,
                              const Exponent &e) {
  return series_mul(series_mul(series_pow(s[0], e[0]), series_pow(s[1], e[1])),
                    series_pow(s[2], e[2]));
}
inline Series series_evaluate(const Polynomial &g,
                              const std::array<Series, 3> &s) {
  Series r(s[0].size());
  for (auto &[e, c] : g.terms) {
    auto v = series_monomial(s, e);
    for (size_t i = 0; i < r.size(); i++)
      r[i] += c * v[i];
  }
  return r;
}
inline std::array<Series, 3> local_jets(const Polynomial &g, V3 point,
                                        int length) {
  require(length > 0 && !g.eval(point), "invalid local-jet support");
  point = projective(point);
  int pivot = 0;
  while (!point[pivot])
    pivot++;
  int dependent = -1, independent = -1;
  for (int j = 0; j < 3; j++) {
    if (j != pivot && g.derivative(j).eval(point)) {
      dependent = j;
      break;
    }
  }
  require(dependent >= 0, "no smooth local parameter");
  for (int j = 0; j < 3; j++) {
    if (j != pivot && j != dependent)
      independent = j;
  }
  std::array<Series, 3> s{Series(length), Series(length), Series(length)};
  for (int j = 0; j < 3; j++)
    s[j][0] = point[j];
  if (length > 1)
    s[independent][1] = 1;
  F unit = g.derivative(dependent).eval(point);
  for (int n = 1; n < length; n++) {
    auto v = series_evaluate(g, s);
    s[dependent][n] = -v[n] / unit;
  }
  for (auto &v : series_evaluate(g, s))
    require(!v, "implicit quartic jet identity fails");
  return s;
}
inline std::vector<Exponent> cubic_monomials() {
  std::vector<Exponent> m;
  for (int i = 0; i <= 3; i++)
    for (int j = 0; j <= 3 - i; j++)
      m.push_back({i, j, 3 - i - j});
  return m;
}
inline std::vector<std::vector<F>> nullspace(std::vector<std::vector<F>> a,
                                             size_t columns) {
  std::vector<size_t> pivots;
  size_t rank = 0;
  for (size_t col = 0; col < columns; col++) {
    size_t row = rank;
    while (row < a.size() && !a[row][col])
      row++;
    if (row == a.size())
      continue;
    std::swap(a[row], a[rank]);
    F unit = a[rank][col];
    for (auto &x : a[rank])
      x = x / unit;
    for (size_t i = 0; i < a.size(); i++) {
      if (i != rank) {
        F k = a[i][col];
        for (size_t j = 0; j < columns; j++)
          a[i][j] = a[i][j] - k * a[rank][j];
      }
    }
    pivots.push_back(col);
    rank++;
  }
  std::vector<std::vector<F>> out;
  for (size_t col = 0; col < columns; col++)
    if (std::find(pivots.begin(), pivots.end(), col) == pivots.end()) {
      std::vector<F> v(columns);
      v[col] = 1;
      for (size_t i = 0; i < rank; i++)
        v[pivots[i]] = -a[i][col];
      out.push_back(v);
    }
  return out;
}
inline std::vector<Polynomial> cubics_through(const Construction &c,
                                              const EffectiveDivisor &D,
                                              size_t dimension) {
  auto monomials = cubic_monomials();
  std::vector<std::vector<F>> rows;
  for (auto &[key, C] : D.support) {
    (void)key;
    auto s = local_jets(c.quartic, C.point, C.multiplicity);
    std::vector<Series> values;
    for (auto &e : monomials)
      values.push_back(series_monomial(s, e));
    for (int n = 0; n < C.multiplicity; n++) {
      std::vector<F> row;
      for (auto &v : values)
        row.push_back(v[n]);
      rows.push_back(row);
    }
  }
  auto basis = nullspace(rows, 10);
  require(basis.size() == dimension, "cubic Riemann-Roch dimension mismatch");
  std::vector<Polynomial> out;
  for (auto &v : basis) {
    for (auto &row : rows) {
      F value;
      for (size_t j = 0; j < 10; j++)
        value += row[j] * v[j];
      require(!value, "cubic violates a divisor jet");
    }
    Polynomial p;
    for (size_t j = 0; j < 10; j++) {
      require(v[j].frob(9) == v[j], "cubic basis failed degree-nine descent");
      p.add(monomials[j], v[j]);
    }
    out.push_back(p);
  }
  return out;
}
inline std::string number_string(const F &c) {
  std::ostringstream s;
  bool first = true;
  auto cc = c.coeffs();
  for (size_t i = 0; i < cc.size(); i++)
    if (cc[i]) {
      if (!first)
        s << "+";
      first = false;
      s << cc[i];
      if (i)
        s << "*a^" << i;
    }
  return first ? "0" : "(" + s.str() + ")";
}
inline F parse_number(std::string s) {
  s = std::regex_replace(s, std::regex("[() *\\t\\r\\n]"), "");
  require(!s.empty() && s.find('/') == std::string::npos,
          "unsupported Singular number format");
  F out;
  size_t i = 0;
  while (i < s.size()) {
    int sign = 1;
    if (s[i] == '+' || s[i] == '-') {
      sign = s[i] == '-' ? -1 : 1;
      i++;
    }
    size_t j = i;
    while (j < s.size() && s[j] != '+' && s[j] != '-')
      j++;
    std::string term = s.substr(i, j - i);
    std::smatch match;
    require(std::regex_match(term, match,
                             std::regex("([0-9]*)(a(?:\\^?[0-9]+)?)?")) &&
                !term.empty(),
            "cannot parse Singular coefficient: " + term);
    long coefficient = match[1].str().empty() ? 1 : std::stol(match[1]);
    int power = 0;
    std::string variable = match[2];
    if (!variable.empty()) {
      std::string e = variable.substr(1);
      if (!e.empty() && e[0] == '^')
        e.erase(0, 1);
      power = e.empty() ? 1 : std::stoi(e);
    }
    require(power < 128, "Singular coefficient degree bound");
    out += F(sign * coefficient) * F::generator().pow(power);
    i = j;
  }
  return out;
}
inline std::string affine_extension_string(const Polynomial &p) {
  std::ostringstream s;
  bool first = true;
  for (auto &[e, c] : p.terms) {
    if (!first)
      s << "+";
    first = false;
    s << number_string(c);
    if (e[0])
      s << "*x^" << e[0];
    if (e[1])
      s << "*y^" << e[1];
  }
  return first ? "0" : s.str();
}
inline std::string singular_field() {
  std::ostringstream s;
  s << "ring rr=(" << FP << ",a),(y,x),lp; minpoly=";
  bool first = true;
  for (long i = 0; i <= fq_nmod_ctx_degree(FC); i++) {
    auto c = nmod_poly_get_coeff_ui(FC->modulus, i);
    if (!c)
      continue;
    if (!first)
      s << "+";
    first = false;
    s << c;
    if (i)
      s << "*a^" << i;
  }
  s << ";\n";
  return s.str();
}
struct ResidualDivisor {
  Univariate u, v;
};
inline ResidualDivisor residual_identity(const Construction &c,
                                         const Polynomial &C0,
                                         const EffectiveDivisor &known) {
  std::string script =
      singular_field() + "poly f=" + affine_extension_string(c.quartic) +
      "; poly c=" + affine_extension_string(C0) +
      "; ideal J=f,c; ideal G=std(J); poly H; int i; intvec e;\n"
      "for(i=1;i<=size(G);i++){print(\"CUBIC_POLY\");H=G[i];while(H!=0){e="
      "leadexp(H);print(\"CUBIC_TERM \"+string(e[1])+\" \"+string(e[2])+\" "
      "\"+string(leadcoef(H)));H=H-lead(H);}print(\"CUBIC_END\");}print("
      "\"CUBIC_DONE\");quit;\n";
  std::string answer = singular_run(script);
  require(answer.find('?') == std::string::npos &&
              answer.find("CUBIC_DONE") != std::string::npos,
          "Singular residual calculation failed: " + answer.substr(0, 1000));
  using Bivariate = std::map<std::pair<int, int>, F>;
  std::vector<Bivariate> basis;
  Bivariate current;
  std::istringstream lines(answer);
  std::string line;
  while (std::getline(lines, line)) {
    if (line == "CUBIC_POLY")
      current.clear();
    else if (line == "CUBIC_END")
      basis.push_back(current);
    else if (line.rfind("CUBIC_TERM ", 0) == 0) {
      std::istringstream input(line);
      std::string tag, value;
      int y = 0, x = 0;
      input >> tag >> y >> x;
      std::getline(input, value);
      require(x >= 0 && y >= 0, "negative residual exponent");
      current[{y, x}] = parse_number(value);
    }
  }
  Univariate u, v;
  for (auto &p : basis) {
    bool univariate = true, linear = true;
    F lead;
    Univariate rest;
    for (auto &[e, a] : p) {
      if (e.first) {
        univariate = false;
        if (e.first != 1 || e.second != 0)
          linear = false;
        else
          lead += a;
      } else {
        if (rest.size() <= size_t(e.second))
          rest.resize(e.second + 1);
        rest[e.second] += a;
      }
    }
    trim(rest);
    if (univariate && rest.size() > 1 && (u.empty() || rest.size() < u.size()))
      u = rest;
    if (linear && lead)
      v = poly_scale(rest, -F(1) / lead);
  }
  require(!u.empty() && !v.empty(),
          "identity residual is not in a primitive affine chart");
  u = poly_scale(u, F(1) / u.back());
  v = poly_mod(v, u);
  require(eval_mod(c.quartic, {F(0), F(1)}, v, u).empty() &&
              eval_mod(C0, {F(0), F(1)}, v, u).empty(),
          "identity intersection RUR does not satisfy curve/cubic");
  Univariate removed{F(1)};
  for (auto &[key, C] : known.support) {
    (void)key;
    if (!C.point[2])
      continue;
    F x = C.point[0] / C.point[2];
    for (int i = 0; i < C.multiplicity; i++)
      removed = poly_mul(removed, {-x, F(1)});
  }
  auto division = poly_divmod(u, removed);
  require(division.second.empty() && division.first.size() == 4,
          "identity residual degree is not three");
  u = division.first;
  v = poly_mod(v, u);
  for (auto &a : u)
    require(a.frob(9) == a, "identity u failed degree-nine descent");
  for (auto &a : v)
    require(a.frob(9) == a, "identity v failed degree-nine descent");
  return {u, v};
}
inline M3 coefficient_matrix(const std::array<Univariate, 3> &rows) {
  M3 m{};
  for (int i = 0; i < 3; i++)
    for (size_t j = 0; j < rows[i].size(); j++) {
      require(j < 3, "residue coefficient degree");
      m[i][j] = rows[i][j];
    }
  return m;
}
inline F norm_residue(const Univariate &h, const Univariate &u) {
  M3 m{};
  for (int i = 0; i < 3; i++) {
    Univariate t(i + 1);
    t[i] = 1;
    auto product = poly_mulmod(h, t, u);
    for (size_t j = 0; j < product.size(); j++)
      m[j][i] = product[j];
  }
  return det(m);
}
inline Polynomial line_polynomial(const V3 &l) {
  Polynomial p;
  for (int i = 0; i < 3; i++)
    p = p + Polynomial::variable(i).scale(l[i]);
  return p;
}
struct WeilBasis {
  int index, a, b;
  std::array<Polynomial, 3> numerators;
  F at_zero;
};
struct RiemannData {
  EffectiveDivisor infinity, identity_constraints;
  Polynomial identity_cubic;
  ResidualDivisor identity;
  std::vector<WeilBasis> functions;
};
inline RiemannData riemann_data(const Construction &c) {
  RiemannData rr;
  rr.infinity = infinity_divisor(c);
  rr.identity_constraints = rr.infinity;
  for (int i : {9, 19, 26})
    rr.identity_constraints.add(c.contacts.at(i));
  rr.identity_constraints.add({F(1), F(0), F(0)}, -1);
  require(rr.identity_constraints.degree() == 9,
          "identity cubic constraint degree");
  rr.identity_cubic = cubics_through(c, rr.identity_constraints, 1)[0];
  rr.identity =
      residual_identity(c, rr.identity_cubic, rr.identity_constraints);
  Univariate x{F(0), F(1)}, one{F(1)};
  F denominator = det(coefficient_matrix({one, x, rr.identity.v})).pow(2);
  require(bool(denominator), "identity divisor lies in determinant pole");
  for (int index : {8, 16, 24, 32, 40, 48, 56}) {
    int aa = -1, bb = -1;
    for (int b : {9, 19, 26})
      if (odd(b ^ index)) {
        aa = b ^ index;
        bb = b;
        break;
      }
    require(aa >= 0, "two-torsion contact pair missing");
    EffectiveDivisor constraints = rr.infinity;
    constraints.add(c.contacts.at(aa));
    constraints.add(c.contacts.at(bb));
    constraints.add({F(1), F(0), F(0)}, -1);
    require(constraints.degree() == 7, "Weil cubic constraint degree");
    auto cubics = cubics_through(c, constraints, 3);
    std::array<Univariate, 3> values;
    for (int j = 0; j < 3; j++)
      values[j] = eval_mod(cubics[j], x, rr.identity.v, rr.identity.u);
    auto la = eval_mod(line_polynomial(c.lines.at(aa)), x, rr.identity.v,
                       rr.identity.u),
         lb = eval_mod(line_polynomial(c.lines.at(bb)), x, rr.identity.v,
                       rr.identity.u);
    F norm = norm_residue(poly_mulmod(la, lb, rr.identity.u), rr.identity.u);
    require(bool(norm), "identity divisor meets a Weil denominator");
    F value = det(coefficient_matrix(values)).pow(2) / (denominator * norm);
    require(value && value.frob(9) == value,
            "Weil identity normalization failed");
    rr.functions.push_back(
        {index, aa, bb, {cubics[0], cubics[1], cubics[2]}, value});
  }
  require(rr.functions.size() == 7, "eight-section Weil basis count");
  return rr;
}
inline std::vector<F> source_signature(const Construction &c, Point P,
                                       bool odd_reference = true) {
  require(!P.infinity,
          "identity source requires separate trivial-target handling");
  for (auto &phi : c.chain)
    P = phi(P);
  std::array<EC, 3> curves;
  std::array<Point, 3> points, q1, q2, q3, shifted;
  std::array<V3, 3> at0;
  for (int j = 0; j < 3; j++) {
    curves[j] = {c.prepared.a.frob(j), c.prepared.b.frob(j)};
    q1[j] = Point(c.two_roots[0].frob(j), F(0));
    q2[j] = Point(c.two_roots[1].frob(j), F(0));
    q3[j] = Point(c.two_roots[2].frob(j), F(0));
    points[j] = Point(P.x.frob(j), P.y.frob(j));
    at0[j] = {F(1), c.two_roots[2].frob(j), F(0)};
  }
  if (odd_reference) {
    auto codes = bits(c.odd_index);
    for (int j = 0; j < 3; j++) {
      if (codes[j] == 1)
        points[j] = curves[j].add(points[j], q1[j]);
      if (codes[j] == 2)
        points[j] = curves[j].add(points[j], q2[j]);
      if (codes[j] == 3)
        points[j] = curves[j].add(points[j], q3[j]);
    }
  }
  std::array<V3, 3> vv;
  for (int j = 0; j < 3; j++) {
    shifted[j] = curves[j].add(points[j], curves[j].neg(q3[j]));
    require(!shifted[j].infinity, "source-signature point pole");
    vv[j] = {F(1), shifted[j].x, shifted[j].y};
  }
  F theta0 = contract(c.theta, at0), den = contract(c.theta, vv);
  require(theta0 && den, "source theta evaluation pole");
  std::vector<F> out;
  for (int index : {8, 16, 24, 32, 40, 48, 56}) {
    auto codes = bits(index);
    auto null = at0, vs = vv;
    F normal(1);
    for (int j = 0; j < 3; j++) {
      int code = codes[j];
      require(code != 3, "unsupported half-characteristic signature");
      if (!code)
        continue;
      null[j] = {F(1), c.two_roots[2 - code].frob(j), F(0)};
      Point U = curves[j].add(shifted[j], code == 1 ? q1[j] : q2[j]);
      require(!U.infinity && !points[j].infinity,
              "source translated point pole");
      vs[j] = {F(1), U.x, U.y};
      F X = points[j].x;
      normal = normal * (X - c.two_roots[2 - code].frob(j)) /
               (X - c.two_roots[2].frob(j));
    }
    F ratio = (contract(c.theta, null) / theta0).pow(2);
    F value = normal.pow(3) * (contract(c.theta, vs) / den).pow(2) / ratio;
    require(value.frob(9) == value,
            "source signature outside degree-nine field");
    out.push_back(value);
  }
  return out;
}
} // namespace cubic_native
