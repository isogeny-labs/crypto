// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "exact.hpp"
#include <openssl/sha.h>
namespace jsr {
inline std::string sha256(const std::string &s) {
  unsigned char h[SHA256_DIGEST_LENGTH];
  SHA256(reinterpret_cast<const unsigned char *>(s.data()), s.size(), h);
  static const char d[] = "0123456789abcdef";
  std::string r;
  for (unsigned char c : h) {
    r += d[c >> 4];
    r += d[c & 15];
  }
  return r;
}
inline Poly S3(const Poly &A, const Poly &B, const Poly &C, int a = 2,
               int b = 3) {
  auto c = [&](int v) { return Poly::constant(A.n, v); };
  return power(A - B, 2) * power(C, 2) -
         scale(((A + B) * (A * B + c(a)) + c(2 * b)) * C, 2) +
         power(A * B - c(a), 2) - scale(A + B, 4 * b);
}
struct Cell {
  int r, mu;
  std::string arm;
  std::vector<int> active;
  std::vector<std::string> names;
  E weights;
  Order order;
  std::vector<Poly> F;
  std::string id;
};
inline Cell cell(int r, int mu, const std::string &arm,
                 const std::vector<int> &centers = {11, 13, 17, 19, 23, 29},
                 int target = 5) {
  require((r == 2 || r == 4) && (mu == 1 || mu == 2), "frozen cell");
  Cell c;
  c.r = r;
  c.mu = mu;
  c.arm = arm;
  c.id = "J-r" + std::to_string(r) + "-mu" + std::to_string(mu) + "-" + arm;
  std::vector<std::string> ns{"z1", "z2", "z3", "z4", "h1", "h2"};
  std::vector<Poly> x;
  require(centers.size() == 6, "six centers required");
  for (size_t i = 0; i < 6; ++i)
    x.push_back(Poly::variable(7, i + 1) + Poly::constant(7, centers[i]));
  std::vector<Poly> fs{S3(x[0], x[1], x[4]), S3(x[2], x[3], x[5]),
                       S3(x[4], x[5], Poly::constant(7, target))};
  std::vector<int> which;
  if (arm == "F1_single") {
    c.active = {0, 1, 4};
    which = {0};
  } else if (arm == "F2_single") {
    c.active = {2, 3, 5};
    which = {1};
  } else if (arm == "F3_single") {
    c.active = {4, 5};
    which = {2};
  } else {
    require(arm == "joint_A" || arm == "joint_AA" || arm == "nearby_A",
            "frozen arm");
    c.active = {0, 1, 2, 3, 4, 5};
    which = {0, 1, 2};
  }
  c.names = {"p"};
  c.order.w = {1000};
  for (int i : c.active) {
    c.names.push_back(ns[i]);
    c.weights.push_back(i < 4 ? 1 : r);
    c.order.w.push_back(50 * (i < 4 ? 1 : r));
  }
  for (int j : which) {
    Poly f(c.names.size());
    for (auto &t : fs[j].t) {
      E e(c.names.size(), 0);
      for (size_t i = 0; i < c.active.size(); ++i)
        e[i + 1] = t.first[c.active[i] + 1];
      Q coefficient = t.second;
      if (arm == "nearby_A") {
        E six(t.first.begin() + 1, t.first.end());
        std::string h =
            sha256("nearby-v1|" + std::to_string(j + 1) + "|" + estr(six));
        Z v(h, 16);
        coefficient = 1 + mpz_fdiv_ui(v.get_mpz_t(), 19);
      }
      f.add(e, coefficient);
    }
    c.F.push_back(f);
  }
  return c;
}
inline std::vector<E> initial_vertices(const Cell &c) {
  std::set<E> v;
  v.insert(E(c.names.size(), 0));
  for (auto &f : c.F)
    for (auto &t : f.t) {
      E e = t.first;
      require(e[0] == 0, "input p exponent");
      for (auto &x : e)
        x *= c.mu;
      v.insert(e);
    }
  return {v.begin(), v.end()};
}
inline std::string canonical_input(const Cell &c) {
  std::string s = "joint-smallroot-native-input-v1\nQQ\n";
  for (auto &n : c.names)
    s += n + ",";
  s += "\nweights=" + estr(c.order.w) + "\nmu=" + std::to_string(c.mu) +
       "\nskip_reduce=false\ntshifts=false\n";
  for (size_t i = 0; i < c.F.size(); ++i)
    s += "F" + std::to_string(i + 1) + "\n" + c.F[i].codec(c.order) + "END\n";
  s += "convex-integral-vertices\n";
  for (auto &e : initial_vertices(c))
    s += estr(e) + "\n";
  return s;
}
inline std::vector<Cell> panel() {
  std::vector<Cell> v;
  for (int r : {2, 4})
    for (int mu : {1, 2})
      for (auto arm : {"F1_single", "F2_single", "F3_single", "joint_A",
                       "joint_AA", "nearby_A"})
        v.push_back(cell(r, mu, arm));
  return v;
}
// Prepared finite control only. Never invoked by structural executable.
struct Point {
  long x = 0, y = 0;
  bool infinity = true;
};
inline long mod(long x) { return (x % 101 + 101) % 101; }
inline long inverse(long x) {
  x = mod(x);
  require(x != 0, "exceptional group denominator");
  long a = x, b = 101, u = 1, v = 0;
  while (b) {
    long q = a / b, t = a - q * b;
    a = b;
    b = t;
    t = u - q * v;
    u = v;
    v = t;
  }
  require(a == 1, "noninvertible denominator");
  return mod(u);
}
inline Point add(Point P, Point T) {
  if (P.infinity)
    return T;
  if (T.infinity)
    return P;
  if (P.x == T.x && mod(P.y + T.y) == 0)
    return {};
  long l = P.x == T.x ? mod((3 * P.x * P.x + 2) * inverse(2 * P.y))
                      : mod((T.y - P.y) * inverse(T.x - P.x));
  long x = mod(l * l - P.x - T.x);
  return {x, mod(l * (P.x - x) - P.y), false};
}
inline Point sign(Point P, int s) {
  if (s < 0)
    P.y = mod(-P.y);
  return P;
}
inline long s3mod(long A, long B, long C) {
  return mod((A - B) * (A - B) * C * C - 2 * ((A + B) * (A * B + 2) + 6) * C +
             (A * B - 2) * (A * B - 2) - 12 * (A + B));
}
inline std::string prepared_control() {
  Point P{1, 39, false}, T{3, 6, false}, R{41, 15, false};
  for (auto p : {P, T, R})
    require(mod(p.y * p.y - p.x * p.x * p.x - 2 * p.x - 3) == 0,
            "control curve");
  Point sum = add(P, T);
  require(!sum.infinity && sum.x == R.x && sum.y == R.y, "P+T control");
  std::string out;
  for (int a : {-1, 1})
    for (int b : {-1, 1})
      for (int c : {-1, 1}) {
        bool zero = add(add(sign(P, a), sign(T, b)), sign(R, c)).infinity;
        require(s3mod(P.x, T.x, R.x) == 0, "sign polynomial control");
        out += std::to_string(a) + "," + std::to_string(b) + "," +
               std::to_string(c) + ":group_zero=" + (zero ? "true" : "false") +
               "\n";
      }
  Point Q = add(R, R);
  require(!Q.infinity, "target doubling infinity");
  require(s3mod(1, 3, 41) == 0 && s3mod(41, 41, Q.x) == 0, "tree control");
  require(mod(s3mod(1, 3, 41) + 1) != 0, "corrupted control falsely accepted");
  return out + "target=" + std::to_string(Q.x) + "," + std::to_string(Q.y) +
         "\ncorrupted_F1_rejected=true\nsubgroup=not_assigned\n";
}
} // namespace jsr
