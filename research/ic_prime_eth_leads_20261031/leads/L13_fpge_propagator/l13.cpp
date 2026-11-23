// L13: F_p Gaussian-elimination external propagator for m=3 prime-field point decomposition (bits-s3 style).
// Single binary: curve/instance generator, CNF emitter, oracle, propagator (GE / GE+univariate quadratic),
// soundness gate, solver driver (cadical library arms; kissat via CLI).
// Build: clang++ -O2 -std=c++17 -I/opt/homebrew/include -o l13 l13.cpp /opt/homebrew/lib/libcadical.a
#include <cstdio>
#include <cstdlib>
#include <cstdint>
#include <cstring>
#include <vector>
#include <map>
#include <set>
#include <unordered_map>
#include <algorithm>
#include <random>
#include <string>
#include <array>
#include <functional>
#include <sys/resource.h>
#include <sys/time.h>
#include <unistd.h>
#include "cadical.hpp"
using namespace std;
typedef int64_t i64;
typedef uint64_t u64;
typedef uint32_t u32;

static u64 g_ops = 0;  // propagator work counter (F_p ops)

// ---------------------------------------------------------------- field / curve
struct Curve {
  i64 p = 0, A = 0, B = 0;
  int n = 0;
  vector<int> inv, sq;  // inv[x] (0 for x=0); sq[x] = a root or -1
  i64 f(i64 x) const { return ((x * x % p) * x % p + A * x + B) % p; }
  void tables() {
    inv.assign(p, 0);
    sq.assign(p, -1);
    for (i64 x = 1; x < p; x++) {
      i64 y = 1, b = x, e = p - 2;
      while (e) { if (e & 1) y = y * b % p; b = b * b % p; e >>= 1; }
      inv[x] = (int)y;
    }
    for (i64 y = p - 1; y >= 0; y--) sq[y * y % p] = (int)y;
  }
  vector<int> roots(i64 c) const {  // y with y^2=c
    c %= p; if (c < 0) c += p;
    if (sq[c] < 0) return {};
    if (sq[c] == 0) return {0};
    return {sq[c], (int)(p - sq[c])};
  }
};
static bool is_prime(i64 x) { if (x < 2) return false; for (i64 d = 2; d * d <= x; d++) if (x % d == 0) return false; return true; }
static i64 md(i64 a, i64 p) { a %= p; return a < 0 ? a + p : a; }

struct Pt { bool inf = true; i64 x = 0, y = 0; };
static Pt ecadd(const Curve& C, Pt a, Pt b) {
  if (a.inf) return b;
  if (b.inf) return a;
  i64 p = C.p, lam;
  if (a.x == b.x) {
    if ((a.y + b.y) % p == 0) return Pt();
    lam = (3 * a.x % p * a.x + C.A) % p * C.inv[(2 * a.y) % p] % p;
  } else lam = md(b.y - a.y, p) * C.inv[md(b.x - a.x, p)] % p;
  Pt r; r.inf = false;
  r.x = md(lam * lam - a.x - b.x, p);
  r.y = md(lam * (a.x - r.x) - a.y, p);
  return r;
}
static Pt ecneg(const Curve& C, Pt a) { if (!a.inf) a.y = md(-a.y, C.p); return a; }

static i64 s3val(const Curve& C, i64 x1, i64 x2, i64 x3) {  // scalar Semaev S3, independent of Poly code
  i64 p = C.p, A = C.A, B = C.B;
  i64 u = x1 * x2 % p, s = (x1 + x2) % p, d = md(x1 - x2, p);
  i64 t1 = d * d % p * (x3 * x3 % p) % p;
  i64 inner = (s * ((u + A) % p) + 2 * B) % p;
  i64 t2 = 2 * inner % p * x3 % p;
  i64 w = md(u - A, p);
  i64 t3 = w * w % p, t4 = 4 * B % p * s % p;
  return md(t1 + t3 - t2 - t4, p);
}

// ---------------------------------------------------------------- polynomials over F_p in NW word variables
static const int NW = 8;  // x1,x2,x3,t,y1,y2,y3,yt
enum { WX1, WX2, WX3, WT, WY1, WY2, WY3, WYT };
struct Term { u32 m; i64 c; };
typedef vector<Term> Poly;
static void norm(Poly& P, i64 p) {
  sort(P.begin(), P.end(), [](const Term& a, const Term& b) { return a.m < b.m; });
  Poly Q;
  for (auto& t : P) {
    if (!Q.empty() && Q.back().m == t.m) Q.back().c = (Q.back().c + t.c) % p;
    else Q.push_back({t.m, t.c % p});
  }
  P.clear();
  for (auto& t : Q) { i64 c = md(t.c, p); if (c) P.push_back({t.m, c}); }
}
static Poly pconst(i64 c, i64 p) { Poly P; c = md(c, p); if (c) P.push_back({0, c}); return P; }
static Poly pvar(int w) { return Poly{{(u32)1 << (4 * w), 1}}; }
static Poly padd(const Poly& a, const Poly& b, i64 p, i64 sb = 1) {
  Poly P = a; for (auto& t : b) P.push_back({t.m, md(sb * t.c, p)}); norm(P, p); return P;
}
static Poly pscale(const Poly& a, i64 k, i64 p) { Poly P = a; for (auto& t : P) t.c = md(t.c * k, p); norm(P, p); return P; }
static Poly pmul(const Poly& a, const Poly& b, i64 p) {
  Poly P;
  for (auto& s : a) for (auto& t : b) {
    u32 m = 0;
    for (int w = 0; w < NW; w++) { int e = ((s.m >> (4 * w)) & 15) + ((t.m >> (4 * w)) & 15); if (e > 15) { fprintf(stderr, "exp overflow\n"); exit(2); } m |= (u32)e << (4 * w); }
    P.push_back({m, s.c * t.c % p});
  }
  norm(P, p); return P;
}
static Poly s3poly(const Curve& C, const Poly& X, const Poly& Y, const Poly& Z) {
  i64 p = C.p;
  Poly u = pmul(X, Y, p), s = padd(X, Y, p), d = padd(X, Y, p, -1), d2 = pmul(d, d, p);
  Poly t1 = pmul(d2, pmul(Z, Z, p), p);
  Poly inner = padd(pmul(s, padd(u, pconst(C.A, p), p), p), pconst(2 * C.B, p), p);
  Poly t2 = pscale(pmul(inner, Z, p), 2, p);
  Poly w = padd(u, pconst(C.A, p), p, -1), t3 = pmul(w, w, p), t4 = pscale(s, 4 * C.B % p, p);
  return padd(padd(t1, t3, p), padd(t2, t4, p), p, -1);
}
static Poly curvepoly(const Curve& C, int wx, int wy) {  // y^2 - x^3 - A x - B
  i64 p = C.p; Poly X = pvar(wx), Y = pvar(wy);
  Poly r = pmul(Y, Y, p);
  r = padd(r, pmul(pmul(X, X, p), X, p), p, -1);
  r = padd(r, pscale(X, C.A, p), p, -1);
  r = padd(r, pconst(C.B, p), p, -1);
  return r;
}
static i64 peval(const Poly& P, const i64* v, i64 p) {
  i64 s = 0;
  for (auto& t : P) {
    i64 c = t.c;
    for (int w = 0; w < NW; w++) { int e = (t.m >> (4 * w)) & 15; for (int k = 0; k < e; k++) c = c * v[w] % p; }
    s = (s + c) % p;
  }
  return s;
}

// ---------------------------------------------------------------- CNF builder
struct Cnf {
  int nv = 1;
  vector<vector<int>> cl;
  Cnf() { cl.push_back({1}); }
  int fresh() { return ++nv; }
  void clause(const vector<int>& c) {
    vector<int> d;
    for (int l : c) { if (l == 1) return; if (l == -1) continue; d.push_back(l); }
    cl.push_back(d);
  }
  int AND(int a, int b) {
    if (a == -1 || b == -1) return -1;
    if (a == 1) return b; if (b == 1) return a;
    if (a == b) return a; if (a == -b) return -1;
    int c = fresh(); clause({-c, a}); clause({-c, b}); clause({c, -a, -b}); return c;
  }
  int OR(int a, int b) { return -AND(-a, -b); }
  int XOR(int a, int b) {
    if (a == -1) return b; if (b == -1) return a;
    if (a == 1) return -b; if (b == 1) return -a;
    if (a == b) return -1; if (a == -b) return 1;
    int c = fresh(); clause({-c, a, b}); clause({-c, -a, -b}); clause({c, -a, b}); clause({c, a, -b}); return c;
  }
  int MUX(int s, int t, int f) { if (s == 1) return t; if (s == -1) return f; if (t == f) return t; return OR(AND(s, t), AND(-s, f)); }
  typedef vector<int> W;
  W fresh_w(int n) { W w(n); for (auto& l : w) l = fresh(); return w; }
  W cw(i64 v, int n) { W w(n); for (int i = 0; i < n; i++) w[i] = ((v >> i) & 1) ? 1 : -1; return w; }
  static int at(const W& a, size_t i) { return i < a.size() ? a[i] : -1; }
  W add(const W& a, const W& b) {
    size_t n = max(a.size(), b.size()); W r(n + 1); int c = -1;
    for (size_t i = 0; i < n; i++) {
      int x = at(a, i), y = at(b, i), xy = XOR(x, y);
      r[i] = XOR(xy, c);
      c = OR(AND(x, y), AND(c, xy));
    }
    r[n] = c; return r;
  }
  W mul(const W& a, const W& b) {
    W r(a.size() + b.size(), -1);
    for (size_t j = 0; j < b.size(); j++) {
      W pp(r.size(), -1);
      for (size_t i = 0; i < a.size(); i++) pp[i + j] = AND(a[i], b[j]);
      W s = add(r, pp); s.resize(r.size()); r = s;
    }
    return r;
  }
  W mulc(const W& a, i64 c) {
    W r(a.size() + 64, -1);
    for (int j = 0; j < 40; j++) if ((c >> j) & 1) {
      W pp(r.size(), -1);
      for (size_t i = 0; i < a.size(); i++) pp[i + j] = a[i];
      W s = add(r, pp); s.resize(r.size()); r = s;
    }
    return r;
  }
  void eqv(const W& a, const W& b) {
    size_t n = max(a.size(), b.size());
    for (size_t i = 0; i < n; i++) { int x = at(a, i), y = at(b, i); clause({x, -y}); clause({-x, y}); }
  }
  int lt(const W& a, const W& b) {
    int l = -1; size_t n = max(a.size(), b.size());
    for (size_t i = 0; i < n; i++) { int x = at(a, i), y = at(b, i); l = MUX(XOR(x, y), y, l); }
    return l;
  }
  void neq(const W& a, const W& b) {
    vector<int> c; size_t n = max(a.size(), b.size());
    for (size_t i = 0; i < n; i++) c.push_back(XOR(at(a, i), at(b, i)));
    clause(c);
  }
  // modular layer (all operands canonical < p)
  i64 p = 0; int n = 0;
  W canon_fresh() { W r = fresh_w(n); clause({lt(r, cw(p, n))}); return r; }
  W mulmod(const W& a, const W& b) {
    W prod = mul(a, b), q = fresh_w(n), r = canon_fresh();
    eqv(prod, add(mulc(q, p), r)); return r;
  }
  W mulmodc(const W& a, i64 c) {
    W prod = mulc(a, c), q = fresh_w(n + 2), r = canon_fresh();
    eqv(prod, add(mulc(q, p), r)); return r;
  }
  W addmod(const W& a, const W& b) {
    W s = add(a, b); int q = fresh(); W r = canon_fresh();
    W qp(n, -1); W pw = cw(p, n); for (int i = 0; i < n; i++) qp[i] = AND(q, pw[i]);
    eqv(s, add(qp, r)); return r;
  }
  W submod(const W& a, const W& b) {  // a-b mod p : a + q p = b + r
    int q = fresh(); W r = canon_fresh();
    W qp(n, -1); W pw = cw(p, n); for (int i = 0; i < n; i++) qp[i] = AND(q, pw[i]);
    eqv(add(a, qp), add(b, r)); return r;
  }
};

struct Inst {
  Curve C; int s = 0;
  i64 xr = 0;
  Cnf cnf;
  vector<vector<int>> W;  // word literals (NW words), LSB first, may include consts +-1
};

static void curve_eq(Cnf& c, const Curve& C, const Cnf::W& x, const Cnf::W& y) {
  auto y2 = c.mulmod(y, y), x2 = c.mulmod(x, x), x3 = c.mulmod(x2, x);
  auto ax = c.mulmodc(x, C.A), s1 = c.addmod(x3, ax), rhs = c.addmod(s1, c.cw(C.B, c.n));
  c.eqv(y2, rhs);
}
static void s3_gadget(Cnf& c, const Curve& C, const Cnf::W& x1, const Cnf::W& x2, const Cnf::W& x3) {
  int n = c.n;
  auto u = c.mulmod(x1, x2), s = c.addmod(x1, x2), d = c.submod(x1, x2), d2 = c.mulmod(d, d);
  auto x32 = c.mulmod(x3, x3), t1 = c.mulmod(d2, x32);
  auto ua = c.addmod(u, c.cw(C.A, n)), sua = c.mulmod(s, ua), inner = c.addmod(sua, c.cw(2 * C.B % C.p, n));
  auto inner2 = c.mulmodc(inner, 2), t2 = c.mulmod(inner2, x3);
  auto w = c.submod(u, c.cw(C.A, n)), t3 = c.mulmod(w, w), t4 = c.mulmodc(s, 4 * C.B % C.p);
  auto lhs = c.addmod(t1, t3), rhs = c.addmod(t2, t4);
  c.eqv(lhs, rhs);
}
static void build_inst(Inst& I) {
  Cnf& c = I.cnf; const Curve& C = I.C; c.p = C.p; c.n = C.n; int n = C.n;
  I.W.assign(NW, {});
  auto xw = [&]() { Cnf::W w(n, -1); for (int i = 0; i < I.s; i++) w[i] = c.fresh(); return w; };
  Cnf::W x1 = xw(), x2 = xw(), x3 = xw();
  Cnf::W y1 = c.canon_fresh(), y2 = c.canon_fresh(), y3 = c.canon_fresh(), t = c.canon_fresh(), yt = c.canon_fresh();
  I.W[WX1] = x1; I.W[WX2] = x2; I.W[WX3] = x3; I.W[WT] = t; I.W[WY1] = y1; I.W[WY2] = y2; I.W[WY3] = y3; I.W[WYT] = yt;
  curve_eq(c, C, x1, y1); curve_eq(c, C, x2, y2); curve_eq(c, C, x3, y3); curve_eq(c, C, t, yt);
  c.clause({c.lt(x1, x2)}); c.clause({c.lt(x2, x3)});  // lex symmetry break (strict => distinct)
  c.neq(t, x3);
  s3_gadget(c, C, x1, x2, t);
  s3_gadget(c, C, t, x3, c.cw(I.xr, n));
}
static void write_dimacs(const Inst& I, const string& path) {
  FILE* f = fopen(path.c_str(), "w");
  fprintf(f, "p cnf %d %zu\n", I.cnf.nv, I.cnf.cl.size());
  for (auto& c : I.cnf.cl) { for (int l : c) fprintf(f, "%d ", l); fprintf(f, "0\n"); }
  fclose(f);
}

// ---------------------------------------------------------------- propagator core (CaDiCaL-independent)
struct Action { int lit; vector<int> clause; };  // clause contains lit (or, for conflicts, a falsified lit)
struct Result { bool conflict = false; vector<int> cclause; vector<Action> props; int nderived = 0; };

static inline int litval(const vector<signed char>& val, int l) {
  int v = abs(l); if (v == 1) return l > 0 ? 1 : -1;
  int t = val[v]; if (!t) return 0; return (l > 0) == (t > 0) ? 1 : -1;
}

struct GEProp {
  Curve C; int mode = 0;  // 0 = GE, 1 = GE + univariate quadratic roots
  vector<Poly> cons; vector<u32> cvars;
  vector<vector<int>> W;
  u64 st_calls = 0, st_props = 0, st_confl = 0, st_derived = 0;
  void init(const Curve& c, const vector<vector<int>>& words, i64 xr, int md_) {
    C = c; W = words; mode = md_; i64 p = C.p;
    Poly X1 = pvar(WX1), X2 = pvar(WX2), X3 = pvar(WX3), T = pvar(WT);
    cons.clear();
    cons.push_back(s3poly(C, X1, X2, T));
    cons.push_back(s3poly(C, T, X3, pconst(xr, p)));
    cons.push_back(curvepoly(C, WX1, WY1)); cons.push_back(curvepoly(C, WX2, WY2));
    cons.push_back(curvepoly(C, WX3, WY3)); cons.push_back(curvepoly(C, WT, WYT));
    cvars.clear();
    for (auto& P : cons) { u32 m = 0; for (auto& t : P) for (int w = 0; w < NW; w++) if ((t.m >> (4 * w)) & 15) m |= 1u << w; cvars.push_back(m); }
  }
  vector<int> provlits(u32 mask, const vector<signed char>& val) const {
    vector<int> r;
    for (int w = 0; w < NW; w++) if (mask >> w & 1) for (int l : W[w]) {
      if (abs(l) == 1) continue;
      int t = litval(val, l);
      if (t == 0) { fprintf(stderr, "internal: provenance word %d has unassigned bit\n", w); exit(2); }
      r.push_back(t > 0 ? -l : l);
    }
    return r;
  }
  struct Row { vector<i64> a; u32 prov; };

  Result compute(const vector<signed char>& val) {
    st_calls++;
    Result R; i64 p = C.p; int n = C.n;
    u32 amask = 0; i64 wv[NW] = {0}; u32 prov[NW] = {0};
    for (int w = 0; w < NW; w++) {
      bool full = true; i64 v = 0;
      for (int i = 0; i < n; i++) { int t = litval(val, W[w][i]); if (!t) { full = false; break; } if (t > 0) v |= (i64)1 << i; }
      if (full) { amask |= 1u << w; wv[w] = v; prov[w] = 1u << w; }
    }
    u32 trailmask = amask;
    for (int w = 0; w < NW; w++) if ((trailmask >> w & 1) && wv[w] >= p) {  // non-canonical word: contradicts CNF
      R.conflict = true; R.cclause = provlits(1u << w, val); st_confl++; return R;
    }
    struct Dom { bool has = false; vector<int> v; u32 prov = 0; };
    Dom dom[NW];
    auto conflict = [&](u32 pm) { R.conflict = true; R.cclause = provlits(pm, val); st_confl++; };
    bool changed = true; int guard = 0;
    while (changed && guard++ < 32) {
      changed = false;
      vector<Poly> reds; vector<u32> pvs;
      for (size_t k = 0; k < cons.size(); k++) {
        u32 pk = 0; for (int w = 0; w < NW; w++) if ((amask & cvars[k]) >> w & 1) pk |= prov[w];
        i64 pw[NW][5];
        for (int w = 0; w < NW; w++) if (amask >> w & 1) { pw[w][0] = 1; for (int e = 1; e < 5; e++) { pw[w][e] = pw[w][e - 1] * wv[w] % p; g_ops++; } }
        Poly rp;
        for (auto& t : cons[k]) {
          i64 c = t.c; u32 m = 0;
          for (int w = 0; w < NW; w++) { int e = (t.m >> (4 * w)) & 15; if (!e) continue; if (amask >> w & 1) { c = c * pw[w][e] % p; g_ops++; } else m |= (u32)e << (4 * w); }
          rp.push_back({m, c});
        }
        norm(rp, p);
        if (rp.empty()) continue;
        if (rp.size() == 1 && rp[0].m == 0) { conflict(pk); return R; }
        reds.push_back(rp); pvs.push_back(pk);
      }
      vector<pair<int, i64>> newdet; vector<u32> newprov;  // (word,value)
      // univariate quadratic rule
      if (mode == 1) {
        for (size_t k = 0; k < reds.size(); k++) {
          u32 um = 0; int maxe = 0;
          for (auto& t : reds[k]) for (int w = 0; w < NW; w++) { int e = (t.m >> (4 * w)) & 15; if (e) { um |= 1u << w; maxe = max(maxe, e); } }
          if (__builtin_popcount(um) != 1 || maxe > 2) continue;
          int u = __builtin_ctz(um); i64 c[3] = {0, 0, 0};
          for (auto& t : reds[k]) c[(t.m >> (4 * u)) & 15] = t.c;
          vector<int> rt;
          if (c[2] == 0) {
            if (c[1] == 0) { conflict(pvs[k]); return R; }  // c0 != 0 with no unknown dependence
            rt.push_back((int)(md(-c[0], p) * C.inv[c[1]] % p)); g_ops += 3 * n;
          } else {
            i64 disc = md(c[1] * c[1] - 4 * c[2] % p * c[0], p); g_ops += 4;
            auto sr = C.roots(disc); g_ops += 3 * n;
            i64 i2a = C.inv[2 * c[2] % p]; g_ops += 3 * n;
            for (int r : sr) rt.push_back((int)(md(-c[1] + r, p) * i2a % p));
            sort(rt.begin(), rt.end()); rt.erase(unique(rt.begin(), rt.end()), rt.end());
          }
          Dom& D = dom[u];
          if (!D.has) { D.has = true; D.v = rt; D.prov = pvs[k]; }
          else { vector<int> nv; for (int a : D.v) if (find(rt.begin(), rt.end(), a) != rt.end()) nv.push_back(a); D.v = nv; D.prov |= pvs[k]; }
          if (D.v.empty()) { conflict(D.prov); return R; }
          if (D.v.size() == 1) { newdet.push_back({u, D.v[0]}); newprov.push_back(D.prov); }
        }
      }
      // Gaussian elimination over linearised monomials
      {
        map<u32, int> colidx; vector<u32> monos;
        for (auto& rp : reds) for (auto& t : rp) if (t.m) if (!colidx.count(t.m)) { colidx[t.m] = 0; monos.push_back(t.m); }
        auto deg = [](u32 m) { int d = 0; for (int w = 0; w < NW; w++) d += (m >> (4 * w)) & 15; return d; };
        sort(monos.begin(), monos.end(), [&](u32 a, u32 b) { int da = deg(a), db = deg(b); return da != db ? da > db : a > b; });
        for (size_t j = 0; j < monos.size(); j++) colidx[monos[j]] = (int)j;
        int nc = (int)monos.size();
        vector<Row> rows;
        for (size_t k = 0; k < reds.size(); k++) {
          Row r; r.a.assign(nc + 1, 0); r.prov = pvs[k];
          for (auto& t : reds[k]) { if (t.m) r.a[colidx[t.m]] = t.c; else r.a[nc] = t.c; }
          rows.push_back(r);
        }
        int rank = 0;
        for (int j = 0; j < nc && rank < (int)rows.size(); j++) {
          int pr = -1; for (int i = rank; i < (int)rows.size(); i++) if (rows[i].a[j]) { pr = i; break; }
          if (pr < 0) continue;
          swap(rows[pr], rows[rank]);
          i64 iv = C.inv[rows[rank].a[j]]; g_ops += 3 * n;
          for (auto& x : rows[rank].a) x = x * iv % p;
          g_ops += nc + 1;
          for (int i = 0; i < (int)rows.size(); i++) if (i != rank && rows[i].a[j]) {
            i64 f = rows[i].a[j];
            for (int q = 0; q <= nc; q++) rows[i].a[q] = md(rows[i].a[q] - f * rows[rank].a[q], p);
            g_ops += nc + 1;
            rows[i].prov |= rows[rank].prov;
          }
          rank++;
        }
        for (int i = rank; i < (int)rows.size(); i++) if (rows[i].a[nc]) { conflict(rows[i].prov); return R; }
        for (int i = 0; i < rank; i++) {
          int cnt = 0, pc = -1; for (int j = 0; j < nc; j++) if (rows[i].a[j]) { cnt++; if (pc < 0) pc = j; }
          if (cnt != 1) continue;
          u32 m = monos[pc]; if (__builtin_popcount(m) != 1) continue;  // pure word (single bit set => word with exp 1)
          int w = -1; for (int q = 0; q < NW; q++) if ((m >> (4 * q)) & 15) w = q;
          if (((m >> (4 * w)) & 15) != 1) continue;
          newdet.push_back({w, md(-rows[i].a[nc], p)}); newprov.push_back(rows[i].prov);
        }
      }
      for (size_t k = 0; k < newdet.size(); k++) {
        int w = newdet[k].first; i64 v = newdet[k].second; u32 pm = newprov[k];
        if (amask >> w & 1) { if (wv[w] != v) { conflict(pm | prov[w]); return R; } continue; }
        if (dom[w].has && find(dom[w].v.begin(), dom[w].v.end(), (int)v) == dom[w].v.end()) { conflict(pm | dom[w].prov); return R; }
        amask |= 1u << w; wv[w] = v; prov[w] = pm; changed = true; R.nderived++;
      }
    }
    // emit actions
    set<int> seen;
    auto emit = [&](int w, i64 v, u32 pm, u32 bitsel) {
      vector<int> pl;
      for (int i = 0; i < n; i++) {
        if (!(bitsel >> i & 1)) continue;
        int l = W[w][i]; int want = (v >> i) & 1; int des = want ? l : -l;
        int cur = litval(val, des);
        if (abs(l) == 1) { if (cur < 0) { conflict(pm); return false; } continue; }
        if (cur > 0) continue;
        if (pl.empty() && pm) pl = provlits(pm, val);
        vector<int> cl = pl; cl.push_back(des);
        if (cur < 0) { R.conflict = true; R.cclause = cl; st_confl++; return false; }
        if (seen.insert(des).second) { R.props.push_back({des, cl}); st_props++; }
      }
      return true;
    };
    for (int w = 0; w < NW; w++) {
      if (trailmask >> w & 1) continue;
      if (amask >> w & 1) { st_derived++; if (!emit(w, wv[w], prov[w], (1u << n) - 1)) return R; }
      else if (dom[w].has && dom[w].v.size() == 2) {
        u32 same = ~(u32)(dom[w].v[0] ^ dom[w].v[1]) & ((1u << n) - 1);
        if (!emit(w, dom[w].v[0], dom[w].prov, same)) return R;
      }
    }
    return R;
  }
};

// ---------------------------------------------------------------- gate checker (word-level model set S of the CNF)
struct Models {
  vector<array<i64, NW>> S;
  vector<pair<int, int>> varmap;  // var -> (word, bit)
  void build(const Inst& I) {
    const Curve& C = I.C; i64 p = C.p; int lim = 1 << I.s;
    for (i64 x1 = 0; x1 < lim && x1 < p; x1++) for (i64 x2 = x1 + 1; x2 < lim && x2 < p; x2++) {
      auto r1 = C.roots(C.f(x1)), r2 = C.roots(C.f(x2)); if (r1.empty() || r2.empty()) continue;
      for (i64 t = 0; t < p; t++) {
        if (s3val(C, x1, x2, t) != 0) continue;
        auto rt = C.roots(C.f(t)); if (rt.empty()) continue;
        for (i64 x3 = x2 + 1; x3 < lim && x3 < p; x3++) {
          if (x3 == t) continue;
          auto r3 = C.roots(C.f(x3)); if (r3.empty()) continue;
          if (s3val(C, t, x3, I.xr) != 0) continue;
          for (int y1 : r1) for (int y2 : r2) for (int y3 : r3) for (int yt : rt) S.push_back({x1, x2, x3, t, y1, y2, y3, yt});
        }
      }
    }
    varmap.assign(I.cnf.nv + 1, {-1, -1});
    for (int w = 0; w < NW; w++) for (int i = 0; i < C.n; i++) { int l = I.W[w][i]; if (abs(l) != 1) varmap[abs(l)] = {w, i}; }
  }
  bool lit_true(const array<i64, NW>& m, int l) const {
    if (abs(l) == 1) return l > 0;
    auto pr = varmap[abs(l)]; if (pr.first < 0) { fprintf(stderr, "clause has non-word var\n"); exit(2); }
    bool b = (m[pr.first] >> pr.second) & 1; return l > 0 ? b : !b;
  }
  bool implied(const vector<int>& cl) const {  // satisfied by every model in S
    for (auto& m : S) { bool ok = false; for (int l : cl) if (lit_true(m, l)) { ok = true; break; } if (!ok) return false; }
    return true;
  }
};

// ---------------------------------------------------------------- CaDiCaL wrapper
struct Prop : public CaDiCaL::ExternalPropagator {
  GEProp* g = nullptr; bool noop = false; const Models* check = nullptr;
  vector<signed char> val; vector<int> trail; vector<size_t> lvl;
  vector<Action> queue; size_t qpos = 0; vector<int> pendingConf;
  bool dirty = true; u32 lastMask = ~0u;
  unordered_map<int, vector<int>> reasons; int rcur = 0; size_t rpos = 0;
  vector<int> ext; size_t epos = 0;
  u64 n_prop = 0, n_conf = 0, n_calls = 0; Curve C; const vector<vector<int>>* W = nullptr; vector<Poly>* chkpolys = nullptr;
  void verify(const vector<int>& cl, const char* kind) {
    if (check && !check->implied(cl)) { fprintf(stderr, "SOUNDNESS FAIL: %s clause not implied by CNF models\n", kind); exit(3); }
  }
  void notify_assignment(const vector<int>& lits) override { for (int l : lits) { val[abs(l)] = l > 0 ? 1 : -1; trail.push_back(l); } if (!noop) dirty = true; }
  void notify_new_decision_level() override { lvl.push_back(trail.size()); }
  void notify_backtrack(size_t nl) override {
    size_t to = nl < lvl.size() ? lvl[nl] : trail.size();
    while (trail.size() > to) { val[abs(trail.back())] = 0; trail.pop_back(); }
    if (nl < lvl.size()) lvl.resize(nl);
    queue.clear(); qpos = 0; pendingConf.clear(); dirty = true; lastMask = ~0u;
  }
  bool cb_check_found_model(const vector<int>& model) override {
    if (!chkpolys) return true;
    i64 v[NW]; int n = C.n;
    vector<signed char> mv(val.size() + 2, 0);
    for (int l : model) if (abs(l) < (int)mv.size()) mv[abs(l)] = l > 0 ? 1 : -1;
    for (int w = 0; w < NW; w++) { v[w] = 0; for (int i = 0; i < n; i++) { int t = litval(mv, (*W)[w][i]); if (t > 0) v[w] |= (i64)1 << i; } }
        for (auto& P : *chkpolys) if (peval(P, v, C.p) != 0) { fprintf(stderr, "SOUNDNESS FAIL: model violates polynomial constraint (poly/CNF mismatch)\n"); exit(3); }
    return true;
  }
  void refresh() {
    if (noop) return;
    u32 mask = 0;
    for (int w = 0; w < NW; w++) { bool f = true; for (int l : (*W)[w]) if (litval(val, l) == 0) { f = false; break; } if (f) mask |= 1u << w; }
    if (!dirty && mask == lastMask) return;
    dirty = false; lastMask = mask; n_calls++;
    Result R = g->compute(val);
    if (R.conflict) { verify(R.cclause, "conflict"); if (pendingConf.empty()) pendingConf = R.cclause; n_conf++; }
    for (auto& a : R.props) { verify(a.clause, "reason"); queue.push_back(a); }
  }
  int cb_propagate() override {
    if (noop) return 0;
    refresh();
    while (qpos < queue.size()) {
      Action& a = queue[qpos++]; int v = litval(val, a.lit);
      if (v > 0) continue;
      if (v < 0) { if (pendingConf.empty()) pendingConf = a.clause; return 0; }
      reasons[a.lit] = a.clause; n_prop++; return a.lit;
    }
    return 0;
  }
  int cb_add_reason_clause_lit(int pl) override {
    if (rcur != pl) { rcur = pl; rpos = 0; }
    auto& c = reasons[pl];
    if (rpos < c.size()) return c[rpos++];
    rcur = 0; return 0;
  }
  bool cb_has_external_clause(bool& forgettable) override {
    if (noop) return false;
    refresh();
    if (pendingConf.empty()) return false;
    ext = pendingConf; pendingConf.clear(); epos = 0; forgettable = true; return true;
  }
  int cb_add_external_clause_lit() override { if (epos < ext.size()) return ext[epos++]; return 0; }
};

// ---------------------------------------------------------------- instance generation / oracle
struct Case { Curve C; int s; vector<int> lx; };
static Case gen_curve(int b, int s, u64 seed) {
  mt19937_64 rng(seed * 1000003ULL + b * 7919ULL + s);
  while (true) {
    i64 lo = (i64)1 << (b - 1), hi = ((i64)1 << b) - 1, p;
    do { p = lo + rng() % (hi - lo + 1); } while (!is_prime(p) || p < 11);
    Curve C; C.p = p; C.n = b; C.A = rng() % p; C.B = rng() % p;
    if (md(4 * C.A % p * C.A % p * C.A + 27 * C.B % p * C.B, p) == 0) continue;
    C.tables();
    Case K; K.C = C; K.s = s;
    for (i64 x = 0; x < ((i64)1 << s) && x < p; x++) if (C.sq[C.f(x)] >= 0) K.lx.push_back((int)x);
    if (K.lx.size() >= 3) return K;
  }
}
static bool oracle_sat(const Curve& C, int s, i64 xr) {
  i64 p = C.p, lim = (i64)1 << s;
  for (i64 x1 = 0; x1 < lim && x1 < p; x1++) { if (C.sq[C.f(x1)] < 0) continue;
    for (i64 x2 = x1 + 1; x2 < lim && x2 < p; x2++) { if (C.sq[C.f(x2)] < 0) continue;
      for (i64 t = 0; t < p; t++) {
        if (s3val(C, x1, x2, t) != 0 || C.sq[C.f(t)] < 0) continue;
        for (i64 x3 = x2 + 1; x3 < lim && x3 < p; x3++) if (x3 != t && C.sq[C.f(x3)] >= 0 && s3val(C, t, x3, xr) == 0) return true;
      } } }
  return false;
}
// planted target: returns false if no triple; fills xr and planted word values
static bool planted_target(const Case& K, mt19937_64& rng, i64& xr, array<i64, NW>& pl) {
  const Curve& C = K.C;
  for (int tries = 0; tries < 500; tries++) {
    vector<int> idx; while (idx.size() < 3) { int v = K.lx[rng() % K.lx.size()]; if (find(idx.begin(), idx.end(), v) == idx.end()) idx.push_back(v); }
    sort(idx.begin(), idx.end());
    Pt P[3];
    for (int k = 0; k < 3; k++) { auto r = C.roots(C.f(idx[k])); P[k].inf = false; P[k].x = idx[k]; P[k].y = r[rng() % r.size()]; }
    Pt T = ecadd(C, P[0], P[1]); if (T.inf || T.x == idx[2]) continue;
    Pt Rr = ecadd(C, T, P[2]); if (Rr.inf) continue;
    xr = Rr.x; pl = {idx[0], idx[1], idx[2], T.x, P[0].y, P[1].y, P[2].y, T.y};
    return true;
  }
  return false;
}
static bool uniform_target(const Case& K, mt19937_64& rng, i64& xr) {
  const Curve& C = K.C;
  for (int t = 0; t < 1000; t++) { i64 x = rng() % C.p; if (C.sq[C.f(x)] >= 0) { xr = x; return true; } }
  return false;
}
static bool group_verify(const Curve& C, i64 x1, i64 x2, i64 x3, i64 xr) {  // exact group arithmetic check of a decoded model
  auto r1 = C.roots(C.f(x1)), r2 = C.roots(C.f(x2)), r3 = C.roots(C.f(x3));
  if (x1 == x2 || x1 == x3 || x2 == x3 || r1.empty() || r2.empty() || r3.empty()) return false;
  for (int a : r1) for (int b : r2) for (int c : r3) {
    Pt P1{false, x1, a}, P2{false, x2, b}, P3{false, x3, c};
    Pt T = ecadd(C, P1, P2); if (T.inf || T.x == x3) continue;
    Pt R = ecadd(C, T, P3); if (!R.inf && R.x == xr) return true;
  }
  return false;
}

// ---------------------------------------------------------------- solving
static double cpu_self() { struct rusage r; getrusage(RUSAGE_SELF, &r); return r.ru_utime.tv_sec + r.ru_utime.tv_usec * 1e-6 + r.ru_stime.tv_sec + r.ru_stime.tv_usec * 1e-6; }
static double cpu_child() { struct rusage r; getrusage(RUSAGE_CHILDREN, &r); return r.ru_utime.tv_sec + r.ru_utime.tv_usec * 1e-6 + r.ru_stime.tv_sec + r.ru_stime.tv_usec * 1e-6; }

struct Out { int res = 0; i64 conflicts = 0, decisions = 0, props = 0; double cpu = 0; u64 ops = 0, pprops = 0, pconf = 0, pcalls = 0; bool group_ok = true; bool decoded = false; };

static Out run_cadical(const Inst& I, int mode /*0 plain,1 noop,2 ge,3 geq*/, i64 cap, const Models* chk, const array<i64, NW>* assume_planted, int seed = 1) {
  Out o; CaDiCaL::Solver S; S.set("quiet", 1); S.set("seed", seed);
  for (auto& c : I.cnf.cl) { for (int l : c) S.add(l); S.add(0); }
  GEProp g; Prop P; vector<Poly> polys;
  if (mode >= 1) {
    P.val.assign(I.cnf.nv + 2, 0); P.val[1] = 1; P.C = I.C; P.W = &I.W;
    P.noop = (mode == 1);
    if (mode >= 2) { g.init(I.C, I.W, I.xr, mode - 2); P.g = &g; polys = g.cons; P.chkpolys = &polys; }
    else { P.chkpolys = nullptr; }
    P.check = chk;
    S.connect_external_propagator(&P);
    for (int w = 0; w < NW; w++) for (int l : I.W[w]) if (abs(l) != 1) S.add_observed_var(abs(l));
  }
  if (assume_planted) for (int w = 0; w < NW; w++) for (int i = 0; i < I.C.n; i++) { int l = I.W[w][i]; if (abs(l) == 1) continue; S.assume(((*assume_planted)[w] >> i & 1) ? l : -l); }
  if (cap > 0) S.limit("conflicts", (int)cap);
  g_ops = 0; double t0 = cpu_self();
  o.res = S.solve();
  o.cpu = cpu_self() - t0;
  o.conflicts = S.get_statistic_value("conflicts"); o.decisions = S.get_statistic_value("decisions"); o.props = S.get_statistic_value("propagations");
  o.ops = g_ops; o.pprops = P.n_prop; o.pconf = P.n_conf; o.pcalls = P.n_calls;
  if (o.res == 10) {
    auto wv = [&](int w) { i64 v = 0; for (int i = 0; i < I.C.n; i++) { int l = I.W[w][i]; bool t = abs(l) == 1 ? l > 0 : (S.val(abs(l)) > 0) == (l > 0); if (t) v |= (i64)1 << i; } return v; };
    o.decoded = true; o.group_ok = group_verify(I.C, wv(WX1), wv(WX2), wv(WX3), I.xr);
  }
  if (mode >= 1) S.disconnect_external_propagator();
  return o;
}
static Out run_kissat(const Inst& I, i64 cap, const string& tmpdir, const string& tag) {
  Out o; string f = tmpdir + "/" + tag + ".cnf"; write_dimacs(I, f);
  string cmd = "kissat --conflicts=" + to_string(cap) + " " + f + " 2>&1";
  double c0 = cpu_child(); FILE* pp = popen(cmd.c_str(), "r"); char line[4096]; vector<int> model; bool got = false;
  o.res = 0; vector<signed char> mv(I.cnf.nv + 2, 0);
  while (fgets(line, sizeof line, pp)) {
    if (!strncmp(line, "s SATISFIABLE", 13)) o.res = 10; else if (!strncmp(line, "s UNSATISFIABLE", 15)) o.res = 20;
    else if (line[0] == 'v') { char* q = line + 1; char* e; while (true) { long l = strtol(q, &e, 10); if (e == q) break; q = e; if (l) { if (labs(l) < (long)mv.size()) mv[labs(l)] = l > 0 ? 1 : -1; got = true; } } }
    else if (line[0] == 'c') {
      long long v; char nm[64];
      if (sscanf(line, "c %63[a-z_]: %lld", nm, &v) == 2) { if (!strcmp(nm, "conflicts")) o.conflicts = v; else if (!strcmp(nm, "decisions")) o.decisions = v; else if (!strcmp(nm, "propagations")) o.props = v; }
    }
  }
  pclose(pp); o.cpu = cpu_child() - c0;
  if (o.res == 10 && got) {
    auto wv = [&](int w) { i64 v = 0; for (int i = 0; i < I.C.n; i++) { int l = I.W[w][i]; bool t = abs(l) == 1 ? l > 0 : (mv[abs(l)] > 0) == (l > 0); if (t) v |= (i64)1 << i; } return v; };
    o.decoded = true; o.group_ok = group_verify(I.C, wv(WX1), wv(WX2), wv(WX3), I.xr);
  }
  remove(f.c_str());
  return o;
}

static const char* RN(int r) { return r == 10 ? "SAT" : r == 20 ? "UNSAT" : "UNK"; }
static const char* ARMN[] = {"cadical_plain", "cadical_noop", "cadical_ge", "cadical_geq", "kissat"};

// ---------------------------------------------------------------- commands
// inst: run all arms on all targets of one (b,s,seed) curve
static int cmd_inst(int argc, char** argv) {
  int b = 6, s = 2, np = 6, nu = 6; u64 seed = 1; i64 cap = 100000; string arms = "plain,noop,ge,geq,kissat", out = "-", tmp = "tmp"; bool check = false;
  for (int i = 2; i + 1 < argc + 1; i++) {
    string a = argv[i];
    if (a == "--check") { check = true; continue; }
    if (i + 1 >= argc) break; string v = argv[i + 1];
    if (a == "--b") b = stoi(v); else if (a == "--s") s = stoi(v); else if (a == "--seed") seed = stoull(v); else if (a == "--np") np = stoi(v);
    else if (a == "--nu") nu = stoi(v); else if (a == "--cap") cap = stoll(v); else if (a == "--arms") arms = v; else if (a == "--out") out = v; else if (a == "--tmp") tmp = v;
    else continue; i++;
  }
  FILE* fo = out == "-" ? stdout : fopen(out.c_str(), "a");
  Case K = gen_curve(b, s, seed); mt19937_64 rng(seed * 77 + b * 13 + s);
  int bad = 0, rows = 0;
  for (int ti = 0; ti < np + nu; ti++) {
    bool planted = ti < np; i64 xr = 0; array<i64, NW> pl{};
    if (planted) { if (!planted_target(K, rng, xr, pl)) continue; } else { if (!uniform_target(K, rng, xr)) continue; }
    Inst I; I.C = K.C; I.s = s; I.xr = xr; build_inst(I);
    bool osat = oracle_sat(K.C, s, xr);
    if (planted && !osat) { fprintf(stderr, "GATE FAIL: planted target not SAT by oracle\n"); return 3; }
    Models M; if (check) M.build(I);
    for (auto& arm : {string("plain"), string("noop"), string("ge"), string("geq"), string("kissat")}) {
      if (arms.find(arm) == string::npos) continue;
      Out o;
      if (arm == "plain") o = run_cadical(I, 0, cap, nullptr, nullptr);
      else if (arm == "noop") o = run_cadical(I, 1, cap, nullptr, nullptr);
      else if (arm == "ge") o = run_cadical(I, 2, cap, check ? &M : nullptr, nullptr);
      else if (arm == "geq") o = run_cadical(I, 3, cap, check ? &M : nullptr, nullptr);
      else o = run_kissat(I, cap, tmp, "k" + to_string(getpid()));
      bool agree = (o.res == 0) || ((o.res == 10) == osat);
      if (!agree || (o.res == 10 && !o.group_ok)) { bad++; fprintf(stderr, "GATE FAIL: arm=%s b=%d s=%d seed=%llu t=%d res=%s oracle=%s group_ok=%d\n", arm.c_str(), b, s, (unsigned long long)seed, ti, RN(o.res), osat ? "SAT" : "UNSAT", (int)o.group_ok); }
      fprintf(fo, "{\"b\":%d,\"s\":%d,\"seed\":%llu,\"t\":%d,\"kind\":\"%s\",\"p\":%lld,\"A\":%lld,\"B\":%lld,\"xr\":%lld,\"arm\":\"%s\",\"res\":\"%s\",\"oracle\":\"%s\",\"agree\":%d,\"group_ok\":%d,\"conflicts\":%lld,\"decisions\":%lld,\"props\":%lld,\"cpu\":%.4f,\"ops\":%llu,\"pprops\":%llu,\"pconf\":%llu,\"pcalls\":%llu,\"cap\":%lld,\"nvars\":%d,\"ncl\":%zu,\"checked\":%d}\n",
              b, s, (unsigned long long)seed, ti, planted ? "planted" : "uniform", (long long)K.C.p, (long long)K.C.A, (long long)K.C.B, (long long)xr, arm.c_str(), RN(o.res), osat ? "SAT" : "UNSAT", (int)agree, (int)o.group_ok,
              (long long)o.conflicts, (long long)o.decisions, (long long)o.props, o.cpu, (unsigned long long)o.ops, (unsigned long long)o.pprops, (unsigned long long)o.pconf, (unsigned long long)o.pcalls, (long long)cap, I.cnf.nv, I.cnf.cl.size(), (int)(check && (arm == "ge" || arm == "geq")));
      fflush(fo); rows++;
    }
  }
  if (fo != stdout) fclose(fo);
  fprintf(stderr, "inst b=%d s=%d seed=%llu rows=%d bad=%d\n", b, s, (unsigned long long)seed, rows, bad);
  return bad ? 3 : 0;
}

// selftest: scalar Semaev vs group law; Poly eval vs scalar
static int cmd_selftest(int argc, char** argv) {
  (void)argc; (void)argv; int bad = 0; long checks = 0;
  for (int b : {5, 6, 7, 8, 9}) for (u64 seed = 1; seed <= 6; seed++) {
    Case K = gen_curve(b, 2, seed + 100); const Curve& C = K.C; i64 p = C.p;
    Poly Pp = s3poly(C, pvar(WX1), pvar(WX2), pvar(WT));
    for (i64 x1 = 0; x1 < p; x1++) { if (C.sq[C.f(x1)] < 0) continue;
      for (i64 x2 = 0; x2 < p; x2++) { if (x2 == x1 || C.sq[C.f(x2)] < 0) continue;
        set<i64> xs;
        for (int a : C.roots(C.f(x1))) for (int c : C.roots(C.f(x2))) { Pt T = ecadd(C, Pt{false, x1, a}, Pt{false, x2, c}); if (!T.inf) xs.insert(T.x); }
        for (i64 t = 0; t < p; t++) {
          if (C.sq[C.f(t)] < 0) continue;
          bool z = s3val(C, x1, x2, t) == 0, g = xs.count(t) > 0;
          i64 v[NW] = {x1, x2, 0, t, 0, 0, 0, 0};
          bool zp = peval(Pp, v, p) == 0;
          checks++;
          if (z != g || zp != z) { bad++; if (bad < 10) fprintf(stderr, "MISMATCH b=%d p=%lld x1=%lld x2=%lld t=%lld s3=%d group=%d poly=%d\n", b, (long long)p, (long long)x1, (long long)x2, (long long)t, z, g, zp); }
        } } }
  }
  printf("selftest: %ld checks, %d mismatches\n", checks, bad);
  return bad ? 3 : 0;
}

// state-sampling gate: random partial assignments of word bits drawn from models of S (and corrupted ones);
// every clause emitted by GEProp must be implied by the model set S; consistent states derived from a model must never conflict.
static int cmd_gate_states(int argc, char** argv) {
  int b = atoi(argv[2]), s = atoi(argv[3]); u64 seed = atoll(argv[4]); int nstate = atoi(argv[5]);
  Case K = gen_curve(b, s, seed); mt19937_64 rng(seed * 31 + 5);
  long total = 0, nprops = 0, nconf = 0, nderived_states = 0, bad = 0, planted_states = 0; int ninst = 0;
  for (int ti = 0; ti < 12; ti++) {
    i64 xr; array<i64, NW> pl{};
    bool planted = ti < 6;
    if (planted) { if (!planted_target(K, rng, xr, pl)) continue; } else { if (!uniform_target(K, rng, xr)) continue; }
    Inst I; I.C = K.C; I.s = s; I.xr = xr; build_inst(I);
    Models M; M.build(I);
    if (planted) { bool found = false; for (auto& m : M.S) if (m == pl) found = true; if (!found) { fprintf(stderr, "GATE FAIL: planted model missing from S\n"); return 3; } }
    ninst++;
    for (int mode = 0; mode < 2; mode++) {
      GEProp g; g.init(I.C, I.W, I.xr, mode);
      for (int it = 0; it < nstate; it++) {
        vector<signed char> val(I.cnf.nv + 2, 0); val[1] = 1;
        array<i64, NW> base;
        bool haveModel = !M.S.empty();
        if (haveModel && (rng() % 4)) base = M.S[rng() % M.S.size()];
        else { for (int w = 0; w < NW; w++) base[w] = rng() % K.C.p; haveModel = false; }
        if (planted && rng() % 3 == 0) { base = pl; haveModel = true; planted_states++; }
        bool corrupt = haveModel && (rng() % 3 == 0);
        int dens = 1 + rng() % 4;  // word-level full-assignment probability dens/5
        for (int w = 0; w < NW; w++) {
          bool full = (int)(rng() % 5) < dens;
          for (int i = 0; i < K.C.n; i++) {
            int l = I.W[w][i]; if (abs(l) == 1) continue;
            if (full || rng() % 4 == 0) { bool bit = (base[w] >> i) & 1; if (corrupt && rng() % 12 == 0) bit = !bit; val[abs(l)] = (bit == (l > 0)) ? 1 : -1; }
          }
        }
        Result R = g.compute(val); total++;
        bool consistent = haveModel && !corrupt;
        if (R.conflict) {
          nconf++;
          if (!M.implied(R.cclause)) { bad++; fprintf(stderr, "UNSOUND conflict clause (mode %d)\n", mode); }
          // conflict clause must be falsified by the state
          for (int l : R.cclause) if (litval(val, l) >= 0) { bad++; fprintf(stderr, "conflict clause not falsified\n"); break; }
          if (consistent) { bad++; fprintf(stderr, "CONFLICT on consistent state (mode %d)\n", mode); }
        }
        for (auto& a : R.props) {
          nprops++;
          if (!M.implied(a.clause)) { bad++; fprintf(stderr, "UNSOUND reason clause (mode %d)\n", mode); }
          bool has = false; for (int l : a.clause) if (l == a.lit) has = true;
          if (!has) { bad++; fprintf(stderr, "reason lacks propagated lit\n"); }
          for (int l : a.clause) if (l != a.lit && litval(val, l) >= 0) { bad++; fprintf(stderr, "reason premise not true\n"); break; }
          if (litval(val, a.lit) != 0) { bad++; fprintf(stderr, "propagated lit already assigned\n"); }
          if (consistent) { // forced literal must hold in the base model (it is in S, so every clause is satisfied; premises true => lit true)
            bool bit = (base[M.varmap[abs(a.lit)].first] >> M.varmap[abs(a.lit)].second) & 1; if ((a.lit > 0) != bit) { bad++; fprintf(stderr, "forced lit contradicts consistent model\n"); }
          }
        }
        if (R.nderived) nderived_states++;
      }
    }
  }
  printf("gate_states b=%d s=%d seed=%llu: instances=%d states=%ld planted_states=%ld conflicts_emitted=%ld props_emitted=%ld states_with_derived_words=%ld bad=%ld\n", b, s, (unsigned long long)seed, ninst, total, planted_states, nconf, nprops, nderived_states, bad);
  return bad ? 3 : 0;
}

// planted-assumption solver test: assume all word bits of the planted model; every propagator arm must return SAT and not abort
static int cmd_gate_planted(int argc, char** argv) {
  int b = atoi(argv[2]), s = atoi(argv[3]); u64 seed = atoll(argv[4]);
  Case K = gen_curve(b, s, seed); mt19937_64 rng(seed * 91 + 3); int bad = 0, n = 0;
  for (int ti = 0; ti < 10; ti++) {
    i64 xr; array<i64, NW> pl{}; if (!planted_target(K, rng, xr, pl)) continue;
    Inst I; I.C = K.C; I.s = s; I.xr = xr; build_inst(I); Models M; M.build(I);
    for (int mode : {0, 1, 2, 3}) {
      Out o = run_cadical(I, mode, 0, mode >= 2 ? &M : nullptr, &pl);
      n++; if (o.res != 10) { bad++; fprintf(stderr, "PLANTED FAIL mode=%d res=%d\n", mode, o.res); }
    }
  }
  printf("gate_planted b=%d s=%d seed=%llu runs=%d bad=%d\n", b, s, (unsigned long long)seed, n, bad);
  return bad ? 3 : 0;
}

int main(int argc, char** argv) {
  if (argc < 2) { fprintf(stderr, "usage: l13 inst|selftest|gate_states|gate_planted ...\n"); return 1; }
  string c = argv[1];
  if (c == "inst") return cmd_inst(argc, argv);
  if (c == "selftest") return cmd_selftest(argc, argv);
  if (c == "gate_states") return cmd_gate_states(argc, argv);
  if (c == "gate_planted") return cmd_gate_planted(argc, argv);
  return 1;
}
