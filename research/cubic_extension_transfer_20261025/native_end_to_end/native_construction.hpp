#pragma once
// Native construction layer of Tian's cubic cover. Historical C++ arithmetic
// is included unchanged; its executable entry point is never called here.
#define main cubic_obstruction_reference_main
#include "../native_followup/obstruction.cpp"
#undef main
#include <deque>
#include <flint/ulong_extras.h>
#include <map>
#include <sstream>
#include <sys/wait.h>
#include <unistd.h>

namespace cubic_native {
using Exponent = std::array<int, 3>;
struct Polynomial {
  std::map<Exponent, F> terms;
  static Polynomial constant(const F &c) {
    Polynomial p;
    if (c)
      p.terms[{0, 0, 0}] = c;
    return p;
  }
  static Polynomial variable(int i) {
    Polynomial p;
    Exponent e{};
    e[i] = 1;
    p.terms[e] = F(1);
    return p;
  }
  void add(const Exponent &e, const F &c) {
    if (!c)
      return;
    auto it = terms.find(e);
    if (it == terms.end())
      terms.emplace(e, c);
    else {
      it->second += c;
      if (!it->second)
        terms.erase(it);
    }
  }
  F coefficient(Exponent e) const {
    auto it = terms.find(e);
    return it == terms.end() ? F(0) : it->second;
  }
  friend Polynomial operator+(Polynomial a, const Polynomial &b) {
    for (auto &[e, c] : b.terms)
      a.add(e, c);
    return a;
  }
  friend Polynomial operator-(Polynomial a, const Polynomial &b) {
    for (auto &[e, c] : b.terms)
      a.add(e, -c);
    return a;
  }
  friend Polynomial operator*(const Polynomial &a, const Polynomial &b) {
    Polynomial out;
    for (auto &[e, c] : a.terms)
      for (auto &[f, d] : b.terms)
        out.add({e[0] + f[0], e[1] + f[1], e[2] + f[2]}, c * d);
    return out;
  }
  Polynomial scale(const F &c) const { return *this * constant(c); }
  Polynomial pow(int n) const {
    require(n >= 0, "negative polynomial exponent");
    Polynomial b = *this, r = constant(F(1));
    while (n) {
      if (n & 1)
        r = r * b;
      b = b * b;
      n >>= 1;
    }
    return r;
  }
  F eval(const V3 &v) const {
    F r;
    for (auto &[e, c] : terms)
      r += c * v[0].pow(e[0]) * v[1].pow(e[1]) * v[2].pow(e[2]);
    return r;
  }
  Polynomial derivative(int i) const {
    Polynomial p;
    for (auto &[e, c] : terms)
      if (e[i]) {
        auto d = e;
        --d[i];
        p.add(d, c * F(e[i]));
      }
    return p;
  }
  Polynomial substitute(const M3 &m) const {
    std::array<Polynomial, 3> l;
    for (int i = 0; i < 3; i++)
      for (int j = 0; j < 3; j++)
        l[i] = l[i] + variable(j).scale(m[i][j]);
    Polynomial out;
    for (auto &[e, c] : terms)
      out = out + (l[0].pow(e[0]) * l[1].pow(e[1]) * l[2].pow(e[2])).scale(c);
    return out;
  }
  Polynomial normalized() const {
    require(!terms.empty(), "zero quartic");
    return scale(F(1) / terms.begin()->second);
  }
  void prime_field() const {
    for (auto &[e, c] : terms) {
      (void)e;
      require(c.frob() == c, "quartic coefficient failed prime-field descent");
    }
  }
  json encode() const {
    json out = json::array();
    for (auto &[e, c] : terms)
      out.push_back({{"exponent", e}, {"coefficient", c.constant()}});
    return out;
  }
};
inline V3 row_times(const V3 &v, const M3 &m) {
  V3 r{};
  for (int j = 0; j < 3; j++)
    for (int i = 0; i < 3; i++)
      r[j] += v[i] * m[i][j];
  return r;
}
inline V3 column_times(const M3 &m, const V3 &v) {
  V3 r{};
  for (int i = 0; i < 3; i++)
    r[i] = dot(m[i], v);
  return r;
}
inline V3 projective(V3 v) {
  for (auto &c : v)
    if (c) {
      F t = c;
      for (auto &d : v)
        d = d / t;
      return v;
    }
  throw std::runtime_error("zero projective vector");
}
inline bool proportional(const V3 &a, const V3 &b) {
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++)
      if (a[i] * b[j] != a[j] * b[i])
        return false;
  return true;
}
inline M3 identity() {
  M3 m{};
  for (int i = 0; i < 3; i++)
    m[i][i] = 1;
  return m;
}
inline bool equal(const M3 &a, const M3 &b) {
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++)
      if (a[i][j] != b[i][j])
        return false;
  return true;
}
inline bool positive_obstruction(long value) {
  require(value > 0 && uint64_t(value) < FP,
          "obstruction is not a nonzero prime-field element");
  return F(value).pow((FP - 1) / 2) == F(1);
}
inline M3 scale(M3 m, const F &c) {
  for (auto &r : m)
    for (auto &x : r)
      x = x * c;
  return m;
}
inline Actions rational_actions(const EC &E, const Normal &n) {
  Actions out;
  out[0] = identity();
  for (int c = 1; c < 9; c++) {
    const Point &P = n.points[c];
    F u = P.x, v = P.y, m = (F(3) * u * u + E.a) / (F(2) * v);
    require(m * m == F(3) * u, "non-flex three-torsion input");
    M3 N{V3{m * u - v, m * u * u - F(3) * u * v, F(3) * v * v - u * m * v},
         V3{-m, F(2) * v - m * u, m * v}, V3{F(-1), -u, -v}};
    // The universal lift is N/(2v), and the actual normalized section has
    // y coefficient alpha. Multiplication by -alpha gives that section's lift.
    out[c] = scale(N, -n.sections[c][2]);
    require(equal(matmul(matmul(out[c], out[c]), out[c]), identity()),
            "rational torsion lift is not order three");
  }
  for (int c = 0; c < 9; c++)
    for (int d = 0; d < 9; d++) {
      int exponent = 2 * ((c / 3) * (d % 3) - (c % 3) * (d / 3));
      F factor = n.pairing.pow((exponent % 3 + 3) % 3);
      require(equal(matmul(out[c], out[d]), scale(out[sumcode(c, d)], factor)),
              "rational torsion lifts fail theta-group multiplication");
    }
  return out;
}
using Lines = std::map<int, V3>;
inline Lines gradients(const Tensor &T, const std::vector<F> &rt, const EC &E) {
  std::array<V3, 4> v{V3{F(1), rt[2], F(0)}, V3{F(1), rt[1], F(0)},
                      V3{F(1), rt[0], F(0)}, V3{F(0), F(0), F(-1)}};
  std::array<V3, 4> d{V3{F(0), F(0), F(3) * rt[2] * rt[2] + E.a},
                      V3{F(0), F(0), F(3) * rt[1] * rt[1] + E.a},
                      V3{F(0), F(0), F(3) * rt[0] * rt[0] + E.a},
                      V3{F(0), F(1), F(0)}};
  Lines out;
  for (int index = 0; index < 64; index++)
    if (odd(index)) {
      auto b = bits(index);
      std::array<V3, 3> vv;
      for (int j = 0; j < 3; j++)
        for (int k = 0; k < 3; k++)
          vv[j][k] = v[b[j]][k].frob(j);
      require(!contract(T, vv),
              "odd theta does not vanish in gradient calculation");
      V3 line{};
      for (int j = 0; j < 3; j++) {
        auto dd = vv;
        for (int k = 0; k < 3; k++)
          dd[j][k] = d[b[j]][k].frob(j);
        line[j] = contract(T, dd);
      }
      require(std::any_of(line.begin(), line.end(),
                          [](const F &x) { return bool(x); }),
              "zero odd gradient");
      out[index] = line;
    }
  require(out.size() == 28, "odd-gradient count");
  return out;
}
inline Polynomial riemann_quartic(const Lines &lines) {
  M3 first{lines.at(63), lines.at(38), lines.at(52)};
  V3 co = row_times(lines.at(41), inverse(first));
  M3 H = first;
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++)
      H[i][j] = H[i][j] * co[i];
  M3 A;
  int labels[3] = {13, 26, 19};
  for (int i = 0; i < 3; i++) {
    A[i] = row_times(lines.at(labels[i]), inverse(H));
    F a = A[i][0];
    require(bool(a), "Aronhold normalization pole");
    for (auto &x : A[i])
      x = x / a;
  }
  M3 reciprocal{}, transpose_scaled{};
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++)
      reciprocal[i][j] = F(1) / A[j][i];
  V3 minus{F(-1), F(-1), F(-1)}, U = column_times(inverse(reciprocal), minus);
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++)
      transpose_scaled[i][j] = A[j][i] * U[j];
  V3 k = column_times(inverse(transpose_scaled), minus);
  std::array<V3, 4> left{}, right{};
  left[0] = {F(1), F(1), F(1)};
  right[0] = minus;
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++) {
      left[i + 1][j] = F(1) / A[i][j];
      right[i + 1][j] = -k[i] * A[i][j];
    }
  M3 Xi = matmul(inverse(M3{left[0], left[1], left[2]}),
                 M3{right[0], right[1], right[2]});
  for (int i = 0; i < 4; i++)
    require(row_times(left[i], Xi) == right[i],
            "four-row Riemann system is inconsistent");
  auto X = Polynomial::variable(0), Y = Polynomial::variable(1),
       Z = Polynomial::variable(2);
  std::array<Polynomial, 3> v{X, Y, Z}, a{}, b{};
  M3 XiH = matmul(Xi, H);
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++) {
      a[i] = a[i] + v[j].scale(H[i][j]);
      b[i] = b[i] + v[j].scale(XiH[i][j]);
    }
  return (a[0] * b[0] + a[1] * b[1] - a[2] * b[2]).pow(2) -
         (a[0] * a[1] * b[0] * b[1]).scale(F(4));
}
struct Contact {
  V3 point;
  int multiplicity;
};
inline std::vector<Contact> bitangent_contacts(const Polynomial &g,
                                               const V3 &line) {
  int pivot = 0;
  while (pivot < 3 && !line[pivot])
    pivot++;
  require(pivot < 3, "zero bitangent");
  std::vector<int> free;
  for (int i = 0; i < 3; i++)
    if (i != pivot)
      free.push_back(i);
  // Substitute (t,1) on the line, without enumerating extension-field points.
  std::array<std::array<F, 2>, 3> v{};
  v[free[0]][1] = 1;
  v[free[1]][0] = 1;
  v[pivot][1] = -line[free[0]] / line[pivot];
  v[pivot][0] = -line[free[1]] / line[pivot];
  auto multiply = [](const std::vector<F> &a, const std::vector<F> &b) {
    std::vector<F> c(a.size() + b.size() - 1);
    for (size_t i = 0; i < a.size(); i++)
      for (size_t j = 0; j < b.size(); j++)
        c[i + j] += a[i] * b[j];
    return c;
  };
  std::vector<F> f(5);
  for (auto &[e, c] : g.terms) {
    std::vector<F> a{c};
    for (int i = 0; i < 3; i++)
      for (int j = 0; j < e[i]; j++)
        a = multiply(a, {v[i][0], v[i][1]});
    for (size_t i = 0; i < a.size(); i++)
      f[i] += a[i];
  }
  int degree = 4;
  while (degree >= 0 && !f[degree])
    degree--;
  require(degree >= 0 && degree % 2 == 0,
          "line does not cut an even quartic divisor");
  F leading = f[degree];
  std::vector<F> h;
  if (degree == 4) {
    F b = f[3] / (F(2) * leading), c = (f[2] / leading - b * b) / F(2);
    h = {c, b, F(1)};
  } else if (degree == 2)
    h = {f[1] / (F(2) * leading), F(1)};
  else
    h = {F(1)};
  auto square = multiply(h, h);
  for (int i = 0; i <= 4; i++)
    require(f[i] == (i < int(square.size()) ? leading * square[i] : F(0)),
            "bitangent square identity fails");
  std::vector<Contact> out;
  int total = 0;
  auto rr = roots(h);
  for (auto &t : rr) {
    V3 P{};
    for (int i = 0; i < 3; i++)
      P[i] = v[i][0] + v[i][1] * t;
    int m = (degree == 4 && F(2) * t + h[1] == F(0)) ? 2 : 1;
    out.push_back({projective(P), m});
    total += m;
  }
  if (degree < 4) {
    V3 P{};
    for (int i = 0; i < 3; i++)
      P[i] = v[i][1];
    int m = (4 - degree) / 2;
    out.push_back({projective(P), m});
    total += m;
  }
  require(total == 2, "contact support needs a larger splitting field");
  for (auto &C : out)
    require(!g.eval(C.point) && !dot(line, C.point),
            "invalid bitangent contact point");
  return out;
}
inline std::string affine_string(const Polynomial &g, int chart) {
  std::ostringstream s;
  bool first = true;
  int vars[2], k = 0;
  for (int i = 0; i < 3; i++)
    if (i != chart)
      vars[k++] = i;
  for (auto &[e, c] : g.terms) {
    if (!first)
      s << "+";
    first = false;
    s << c.constant();
    if (e[vars[0]])
      s << "*u^" << e[vars[0]];
    if (e[vars[1]])
      s << "*v^" << e[vars[1]];
  }
  return first ? "0" : s.str();
}
inline std::string singular_run(const std::string &script) {
  char path[] = "/tmp/cubic-native-singular-XXXXXX";
  int file = mkstemp(path);
  require(file >= 0, "cannot create Singular input");
  size_t done = 0;
  while (done < script.size()) {
    ssize_t n = write(file, script.data() + done, script.size() - done);
    if (n <= 0) {
      close(file);
      unlink(path);
      throw std::runtime_error("cannot write Singular input");
    }
    done += size_t(n);
  }
  close(file);
  int p[2];
  if (pipe(p)) {
    unlink(path);
    throw std::runtime_error("Singular pipe failed");
  }
  pid_t child = fork();
  if (child < 0) {
    close(p[0]);
    close(p[1]);
    unlink(path);
    throw std::runtime_error("Singular fork failed");
  }
  if (!child) {
    close(p[0]);
    dup2(p[1], STDOUT_FILENO);
    dup2(p[1], STDERR_FILENO);
    close(p[1]);
    execlp("Singular", "Singular", "-q", path, static_cast<char *>(nullptr));
    _exit(127);
  }
  close(p[1]);
  std::string output;
  char buf[4096];
  ssize_t n;
  while ((n = read(p[0], buf, sizeof(buf))) > 0) {
    output.append(buf, size_t(n));
    if (output.size() > 1048576) {
      kill(child, SIGKILL);
      break;
    }
  }
  close(p[0]);
  int status = 0;
  waitpid(child, &status, 0);
  unlink(path);
  require(WIFEXITED(status) && WEXITSTATUS(status) == 0,
          "native Singular invocation failed: " + output.substr(0, 1000));
  return output;
}
inline bool smooth(const Polynomial &g) {
  g.prime_field();
  std::ostringstream s;
  for (int i = 0; i < 3; i++)
    s << "ring r" << i << "=" << FP
      << ",(u,v),dp; poly f=" << affine_string(g, i)
      << "; ideal J=std(ideal(f,diff(f,u),diff(f,v))); if(size(J)==1 && "
         "J[1]==1){print(\"CUBIC_PATCH_OK"
      << i << "\");}else{print(\"CUBIC_PATCH_FAIL" << i << "\");}\n";
  s << "quit;\n";
  std::string output = singular_run(s.str());
  if (output.find('?') != std::string::npos)
    throw std::runtime_error("Singular reported an error: " +
                             output.substr(0, 1000));
  for (int i = 0; i < 3; i++)
    if (output.find("CUBIC_PATCH_OK" + std::to_string(i) + "\n") ==
        std::string::npos)
      return false;
  return true;
}
inline std::pair<Polynomial, M3> tangent_chart(const Polynomial &g,
                                               const V3 &odd_line) {
  for (int attempt = 0; attempt < 512; attempt++) {
    F x(long(n_randint(FR, FP)));
    std::vector<F> f(5);
    for (auto &[e, c] : g.terms)
      f[e[1]] += c * x.pow(e[0]);
    auto ys = roots(f);
    for (auto &y : ys) {
      if (y.frob() != y)
        continue;
      V3 O{x, y, F(1)};
      if (!dot(odd_line, O))
        continue;
      V3 line{};
      for (int i = 0; i < 3; i++)
        line[i] = g.derivative(i).eval(O);
      int pivot = 0;
      while (pivot < 3 && !line[pivot])
        pivot++;
      require(pivot < 3, "singular rational curve point");
      V3 U{};
      bool independent = false;
      for (int j = 0; j < 3; j++)
        if (j != pivot) {
          V3 v{};
          v[j] = 1;
          v[pivot] = -line[j] / line[pivot];
          if (!proportional(v, O)) {
            U = v;
            independent = true;
            break;
          }
        }
      require(independent, "tangent basis failed");
      V3 W{};
      W[pivot] = 1;
      for (int retry = 0; retry < 16; retry++) {
        F t(long(n_randint(FR, FP)));
        V3 V = U;
        for (int i = 0; i < 3; i++)
          V[i] += t * O[i];
        if (!g.eval(V))
          continue;
        M3 M{};
        for (int i = 0; i < 3; i++) {
          M[i][0] = O[i];
          M[i][1] = V[i];
          M[i][2] = W[i];
        }
        require(bool(det(M)), "tangent coordinate matrix singular");
        Polynomial h = g.substitute(M);
        F a = h.coefficient({2, 2, 0});
        if (!a)
          break;
        h = h.scale(F(1) / a);
        require(!h.coefficient({4, 0, 0}) && !h.coefficient({3, 1, 0}) &&
                    h.coefficient({2, 2, 0}) == F(1) &&
                    bool(h.coefficient({0, 4, 0})) &&
                    bool(h.coefficient({3, 0, 1})),
                "ordinary tangent chart identity");
        h.prime_field();
        return {h, M};
      }
    }
  }
  throw std::runtime_error("ordinary tangent chart budget exhausted");
}
inline bool on_curve(const EC &E, const Point &P) {
  return P.infinity || P.y * P.y == P.x.pow(3) + E.a * P.x + E.b;
}
struct Isogeny {
  EC source, target;
  F kernel_x;
  Point T;
  Isogeny(const EC &E, const F &x)
      : source(E), target(E.quotient3(x)), kernel_x(x),
        T(x, (x.pow(3) + E.a * x + E.b).sqrt()) {
    require(source.mul(3, T).infinity, "invalid three-isogeny kernel");
  }
  Point operator()(const Point &P) const {
    require(on_curve(source, P), "isogeny input not on source");
    if (P.infinity || P.x == kernel_x)
      return Point();
    Point A = source.add(P, T), B = source.add(P, source.neg(T));
    require(!A.infinity && !B.infinity, "unexpected Velu pole");
    Point Q(P.x + A.x + B.x - F(2) * kernel_x, P.y + A.y + B.y);
    require(on_curve(target, Q), "Velu image not on codomain");
    return Q;
  }
  void verify(const Point &P, const Point &Q, uint64_t r) const {
    Point a = (*this)(P), b = (*this)(Q);
    require(!a.infinity && target.mul(r, a).infinity &&
                target.mul(r, b).infinity,
            "three-isogeny subgroup preservation");
    require((*this)(source.add(P, Q)) == target.add(a, b),
            "three-isogeny addition identity");
    for (int m : {2, 3, 7, 11})
      require((*this)(source.mul(m, P)) == target.mul(m, a),
              "three-isogeny scalar identity");
    require((*this)(T).infinity && (*this)(source.neg(T)).infinity,
            "three-isogeny kernel image");
  }
};
struct Source {
  uint64_t p, r;
  F z;
  EC curve;
  Point P, Q;
};
inline uint64_t ceil_sqrt(uint64_t n) {
  uint64_t lo = 0, hi = 1;
  while ((__uint128_t)hi * hi < n)
    hi *= 2;
  while (lo + 1 < hi) {
    uint64_t m = (lo + hi) / 2;
    if ((__uint128_t)m * m >= n)
      hi = m;
    else
      lo = m;
  }
  return n == 0 ? 0 : hi;
}
inline Source source_fixture(const std::string &path) {
  std::ifstream f(path);
  require(bool(f), "cannot read source fixture");
  std::array<std::string, 9> line;
  for (auto &s : line)
    require(bool(std::getline(f, s)), "truncated public source fixture");
  Source s;
  s.p = std::stoull(line[0]);
  s.r = std::stoull(line[1]);
  require(s.p == FP && s.p > 3 && s.p <= 4096 && n_is_prime(s.p) &&
              n_is_prime(s.r),
          "prime/size gate");
  require(s.p % 3 == 1 && s.r % 9 == 4, "cubic congruence gate");
  std::vector<F> c;
  for (auto a : parse_poly(line[3]))
    c.push_back(F(a));
  auto zr = roots(c);
  require(zr.size() == 3, "cubic field embedding unavailable");
  s.z = zr[0];
  require(s.z.frob(3) == s.z && s.z.frob() != s.z,
          "cubic field embedding degree");
  auto ab = parse_curve(line[4], s.z);
  s.curve = {ab.first, ab.second};
  s.P = Point(eval_poly(parse_poly(line[5]), s.z),
              eval_poly(parse_poly(line[6]), s.z));
  s.Q = Point(eval_poly(parse_poly(line[7]), s.z),
              eval_poly(parse_poly(line[8]), s.z));
  require(s.curve.a.frob(3) == s.curve.a && s.curve.b.frob(3) == s.curve.b,
          "source coefficients outside cubic field");
  require(bool(F(4) * s.curve.a.pow(3) + F(27) * s.curve.b * s.curve.b),
          "singular elliptic source");
  require(s.curve.j().frob() != s.curve.j(), "subfield source rejected");
  uint64_t q = s.p * s.p * s.p, bound = ceil_sqrt(4 * q);
  require(s.r >= q + 1 - bound && s.r <= q + 1 + bound,
          "claimed source order outside Hasse interval");
  int64_t trace = int64_t(q + 1) - int64_t(s.r);
  require(trace % int64_t(s.p) != 0, "supersingular/ordinary gate");
  require(!s.P.infinity && on_curve(s.curve, s.P) && on_curve(s.curve, s.Q) &&
              s.curve.mul(s.r, s.P).infinity && s.curve.mul(s.r, s.Q).infinity,
          "source subgroup certificate");
  require(2 * s.r > q + 1 + bound,
          "Hasse bound does not certify prime full source order");
  // Deliberately never read the fixture's tenth (known logarithm) line.
  return s;
}
struct Construction {
  Source source;
  EC prepared;
  Point P, Q;
  std::vector<Isogeny> chain;
  Normal normal;
  Tensor theta;
  std::vector<F> two_roots;
  M3 descent;
  Polynomial quartic;
  Lines lines;
  std::map<int, std::vector<Contact>> contacts;
  int odd_index = -1, kernel_index = -1;
  json walk;
};
inline json point_json(const Point &P) {
  if (P.infinity)
    return {{"infinity", true}};
  return {{"infinity", false}, {"x", P.x.coeffs()}, {"y", P.y.coeffs()}};
}
inline Construction construct(const Source &s, const json &kernels,
                              int max_steps = 2, bool audit_actions = true) {
  require(kernels.size() == 12, "twelve transverse kernels required");
  struct Node {
    EC E;
    Point P, Q;
    std::vector<Isogeny> chain;
  };
  std::deque<Node> todo;
  todo.push_back({s.curve, s.P, s.Q, {}});
  json walk = json::array();
  std::set<std::vector<uint64_t>> seen;
  while (!todo.empty()) {
    Node node = std::move(todo.front());
    todo.pop_front();
    auto key = node.E.a.coeffs();
    auto b = node.E.b.coeffs();
    key.insert(key.end(), b.begin(), b.end());
    if (!seen.insert(key).second)
      continue;
    auto all_x = node.E.three_x();
    std::vector<F> rational;
    for (auto &x : all_x)
      if (x.frob(3) == x)
        rational.push_back(x);
    json entry = {{"depth", node.chain.size()},
                  {"a", node.E.a.coeffs()},
                  {"b", node.E.b.coeffs()},
                  {"rational_three_lines", rational.size()},
                  {"kernels", json::array()}};
    if (rational.size() == 4) {
      Normal n = normal3(node.E);
      for (int c : {1, 3})
        require(Point(n.points[c].x.frob(3), n.points[c].y.frob(3)) ==
                    node.E.neg(n.points[c]),
                "prepared three-torsion Frobenius type");
      Actions a = rational_actions(node.E, n);
      if (audit_actions) {
        Actions sampled = action_matrices(node.E, n);
        for (int i = 0; i < 9; i++)
          require(equal(a[i], sampled[i]),
                  "rational lift differs from independent interpolation");
      }
      std::array<Actions, 3> actions;
      for (int j = 0; j < 3; j++)
        for (int c = 0; c < 9; c++)
          for (int u = 0; u < 3; u++)
            for (int v = 0; v < 3; v++)
              actions[j][c][u][v] = a[c][u][v].frob(j);
      auto rt = roots({node.E.b, node.E.a, F(0), F(1)});
      require(rt.size() == 3, "two-torsion splitting");
      for (int kernel = 0; kernel < 12; kernel++) {
        json trial = {{"index", kernel}};
        try {
          Tensor T = invariant(actions, kernels[kernel]);
          long value = chi(T, rt, node.E, s.z);
          bool good = positive_obstruction(value);
          trial["chi18"] = value;
          trial["prime_field_character"] = good ? 1 : -1;
          trial["square"] = good;
          if (!good) {
            entry["kernels"].push_back(trial);
            continue;
          }
          Lines geometric = gradients(T, rt, node.E);
          Polynomial f = riemann_quartic(geometric);
          M3 B{};
          for (int i = 0; i < 3; i++)
            for (int j = 0; j < 3; j++)
              B[i][j] = s.z.pow(j).frob(i);
          require(bool(det(B)), "Weil cotangent basis singular");
          Polynomial g = f.substitute(B).normalized();
          g.prime_field();
          require(smooth(g), "constructed quartic is singular");
          Lines descended;
          int odd_index = -1, odd_count = 0;
          for (auto &[i, l] : geometric) {
            V3 line = projective(row_times(l, B));
            bool rational_line = true;
            for (auto &x : line) {
              require(x.frob(9) == x, "bitangent outside degree-nine field");
              rational_line &= x.frob() == x;
            }
            if (rational_line) {
              odd_index = i;
              odd_count++;
            }
            descended[i] = line;
          }
          require(odd_count == 1,
                  "unique rational odd-theta reference missing");
          auto chart = tangent_chart(g, descended.at(odd_index));
          B = matmul(B, chart.second);
          Construction out{s,  node.E, node.P,    node.Q, node.chain,
                           n,  T,      rt,        B,      chart.first,
                           {}, {},     odd_index, kernel, {}};
          for (auto &[i, l] : geometric) {
            out.lines[i] = projective(row_times(l, B));
            out.contacts[i] = bitangent_contacts(out.quartic, out.lines[i]);
          }
          require(smooth(out.quartic),
                  "tangent transformation lost smoothness");
          trial["constructed"] = true;
          entry["kernels"].push_back(trial);
          walk.push_back(entry);
          out.walk = walk;
          return out;
        } catch (const std::exception &e) {
          trial["rejection"] = e.what();
          entry["kernels"].push_back(trial);
        }
      }
    }
    walk.push_back(entry);
    if (int(node.chain.size()) >= max_steps)
      continue;
    for (auto &x : rational) {
      Isogeny phi(node.E, x);
      phi.verify(node.P, node.Q, s.r);
      Node next{phi.target, phi(node.P), phi(node.Q), node.chain};
      next.chain.push_back(phi);
      todo.push_back(std::move(next));
    }
  }
  throw std::runtime_error("bounded construction exhausted: " + walk.dump());
}
} // namespace cubic_native
