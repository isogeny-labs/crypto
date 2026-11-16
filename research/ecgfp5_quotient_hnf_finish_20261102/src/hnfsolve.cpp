// hnfsolve: change-of-order finish for a zero-dimensional system from its
// reduced DRL Groebner basis, after Berthomieu-Neiger-Safey El Din (ISSAC 2022,
// arXiv 2202.09226), using PML/NTL.
//
//   msolve -g 2 -f sys.ms -o FIFO &   hnfsolve --gb FIFO [--planted FILE] [--elim-out F]
//
// Reads msolve's printed reduced DRL basis (last variable y is the smallest),
// builds the staircase B, checks the stability assumption (for every chain head
// nu, y^d nu with d = chain length is a leading monomial), builds the t x t
// polynomial matrix P in F_p[y] of BNS22 Sec. 3.3/5 (det P = charpoly of the
// multiplication by y), computes det P with PML (Labahn-Neiger-Zhou partial
// triangularization, degree D known), makes it monic (the eliminant h under
// shape position), finds the F_p-roots of h through gcd(h, y^p - y), and for each
// root y0 recovers the point from the kernel of P(y0) (entries at the chain
// heads 1, x_1, ..., x_{n-1}). Prints one JSON object with phase timings.
#include <NTL/lzz_pX.h>
#include <NTL/lzz_pXFactoring.h>
#include <NTL/mat_lzz_p.h>
#include <NTL/matrix.h>
#include <NTL/BasicThreadPool.h>
#include <sys/resource.h>
#include <sys/time.h>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <fstream>
#include <sstream>
#include "mat_lzz_pX_determinant.h"

NTL_CLIENT
using namespace PML;

static double wall() { struct timeval tv; gettimeofday(&tv, nullptr); return tv.tv_sec + 1e-6 * tv.tv_usec; }
static double cpu() {
    struct rusage r; getrusage(RUSAGE_SELF, &r);
    return r.ru_utime.tv_sec + 1e-6 * r.ru_utime.tv_usec + r.ru_stime.tv_sec + 1e-6 * r.ru_stime.tv_usec;
}
static long maxrss_kib() { struct rusage r; getrusage(RUSAGE_SELF, &r); return r.ru_maxrss; }

struct Phase { std::string name; double w, c; };
static std::vector<Phase> phases;
static double pw, pc;
static void phase_start() { pw = wall(); pc = cpu(); }
static void phase_end(const char *n) { phases.push_back({n, wall() - pw, cpu() - pc}); fprintf(stderr, "[hnfsolve] %s: %.2f s wall, %.2f s cpu, maxrss %ld KiB\n", n, wall() - pw, cpu() - pc, maxrss_kib()); }

// ---- buffered reader ----
struct Reader {
    FILE *f; std::vector<char> buf; size_t pos = 0, len = 0; bool eof = false;
    explicit Reader(FILE *f_) : f(f_), buf(1 << 24) {}
    int peek() { if (pos == len) { if (eof) return -1; len = fread(buf.data(), 1, buf.size(), f); pos = 0; if (len == 0) { eof = true; return -1; } } return (unsigned char)buf[pos]; }
    int get() { int c = peek(); if (c >= 0) pos++; return c; }
    std::string line() { std::string s; int c; while ((c = get()) >= 0 && c != '\n') s.push_back((char)c); return s; }
};

static const int MAXV = 8;            // exponent packing: 8 bits per variable
typedef unsigned long long mono_t;
static inline int ex(mono_t m, int v) { return (int)((m >> (8 * v)) & 0xff); }
static inline mono_t mk(const int *e, int n) { mono_t m = 0; for (int v = 0; v < n; v++) m |= (mono_t)e[v] << (8 * v); return m; }
static inline bool mdivides(mono_t a, mono_t b, int n) { for (int v = 0; v < n; v++) if (ex(a, v) > ex(b, v)) return false; return true; }

int main(int argc, char **argv) {
    std::string gbpath, plantedpath, elimout; long threads = 1;
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        if (a == "--gb" && i + 1 < argc) gbpath = argv[++i];
        else if (a == "--planted" && i + 1 < argc) plantedpath = argv[++i];
        else if (a == "--elim-out" && i + 1 < argc) elimout = argv[++i];
        else if (a == "--threads" && i + 1 < argc) threads = atol(argv[++i]);
        else { fprintf(stderr, "usage: hnfsolve --gb FILE [--planted FILE] [--elim-out FILE] [--threads N]\n"); return 2; }
    }
    if (gbpath.empty()) { fprintf(stderr, "--gb required\n"); return 2; }
    SetNumThreads(threads);
    double w0 = wall(), c0 = cpu();

    // ---------------- parse ----------------
    // msolve opens its output file once early and closes it, then reopens it to
    // write the basis; on a FIFO the first open therefore yields EOF. Reopen
    // until the first byte arrives (bounded), and start the parse clock there
    // so parse_gb never includes the time spent waiting for msolve's F4.
    FILE *f = nullptr;
    for (int attempt = 0; attempt < 16; attempt++) {
        f = fopen(gbpath.c_str(), "rb");
        if (!f) { perror("open gb"); return 2; }
        int c0c = fgetc(f);
        if (c0c != EOF) { ungetc(c0c, f); break; }
        fclose(f); f = nullptr;
        fprintf(stderr, "[hnfsolve] empty open of %s (attempt %d); reopening\n", gbpath.c_str(), attempt + 1);
    }
    if (!f) { fprintf(stderr, "gb input stayed empty\n"); return 2; }
    phase_start();
    Reader R(f);
    long p = 0; std::vector<std::string> vars;
    while (R.peek() == '#') {
        std::string l = R.line();
        if (l.find("field characteristic:") != std::string::npos) p = atol(l.substr(l.find(':') + 1).c_str());
        if (l.find("variable order:") != std::string::npos) {
            std::string s = l.substr(l.find(':') + 1); std::stringstream ss(s); std::string v;
            while (std::getline(ss, v, ',')) { v.erase(0, v.find_first_not_of(" \t")); v.erase(v.find_last_not_of(" \t") + 1); if (!v.empty()) vars.push_back(v); }
        }
    }
    const int n = (int)vars.size();
    if (p <= 1 || n < 2 || n > MAXV) { fprintf(stderr, "bad header p=%ld n=%d\n", p, n); return 2; }
    const int Y = n - 1;
    zz_p::init(p);
    std::unordered_map<std::string, int> vidx; for (int v = 0; v < n; v++) vidx[vars[v]] = v;
    if (R.get() != '[') { fprintf(stderr, "expected [\n"); return 2; }

    std::vector<mono_t> LMs;                 // all leading monomials
    std::vector<long> ypoly;                 // index into LMs of polys with y | LM
    std::vector<size_t> tail_off;            // tails of those polys, in tmono/tcoef
    std::vector<mono_t> tmono; std::vector<unsigned> tcoef;
    long nterms = 0;
    bool done = false;
    while (!done) {
        // one polynomial: terms separated by '+', ends at ',' or ']'
        bool first = true; bool keep = false; mono_t lm = 0;
        while (true) {
            // coefficient
            unsigned long long c = 0; int ch = R.peek();
            bool hasc = false;
            while (ch >= '0' && ch <= '9') { c = c * 10 + (ch - '0'); R.get(); ch = R.peek(); hasc = true; }
            if (!hasc) { fprintf(stderr, "parse error at term start (char %d)\n", ch); return 2; }
            int e[MAXV] = {0};
            while (R.peek() == '*') {
                R.get(); std::string name; while (true) { int d = R.peek(); if (d == '^' || d == '*' || d == '+' || d == ',' || d == ']' || d == '\n' || d < 0) break; name.push_back((char)R.get()); }
                long ee = 1;
                if (R.peek() == '^') { R.get(); ee = 0; while (R.peek() >= '0' && R.peek() <= '9') ee = ee * 10 + (R.get() - '0'); }
                auto it = vidx.find(name); if (it == vidx.end()) { fprintf(stderr, "unknown variable %s\n", name.c_str()); return 2; }
                if (ee > 255) { fprintf(stderr, "exponent too large\n"); return 2; }
                e[it->second] += (int)ee;
            }
            mono_t m = mk(e, n); nterms++;
            if (first) {
                if (c % p != 1) { fprintf(stderr, "basis element not monic\n"); return 2; }
                lm = m; LMs.push_back(m); keep = ex(m, Y) > 0;
                if (keep) { ypoly.push_back((long)LMs.size() - 1); tail_off.push_back(tmono.size()); }
                first = false;
            } else if (keep) { tmono.push_back(m); tcoef.push_back((unsigned)(c % p)); }
            int d = R.get();
            while (d == '\n' || d == ' ') d = R.get();
            if (d == '+') continue;
            if (d == ',') { int q; while ((q = R.peek()) == '\n' || q == ' ') R.get(); break; }
            if (d == ']') { done = true; break; }
            fprintf(stderr, "parse error: unexpected '%c'\n", d); return 2;
        }
        (void)lm;
    }
    tail_off.push_back(tmono.size());
    fclose(f);
    phase_end("parse_gb");

    // ---------------- staircase and chains ----------------
    phase_start();
    // B = monomials not divisible by any LM; enumerate by BFS from 1.
    std::unordered_map<mono_t, long> Bidx; std::vector<mono_t> B;
    auto inB = [&](mono_t m) { for (mono_t l : LMs) if (mdivides(l, m, n)) return false; return true; };
    B.push_back(0); Bidx[0] = 0;
    for (size_t i = 0; i < B.size(); i++) {
        for (int v = 0; v < n; v++) {
            mono_t m = B[i] + ((mono_t)1 << (8 * v));
            if (ex(m, v) == 0 || Bidx.count(m)) continue;
            if (inB(m)) { Bidx[m] = (long)B.size(); B.push_back(m); }
        }
        if (B.size() > 50000000) { fprintf(stderr, "staircase too large\n"); return 2; }
    }
    const long D = (long)B.size();
    // chain heads: y does not divide
    std::vector<mono_t> heads; std::unordered_map<mono_t, long> hidx;
    for (mono_t m : B) if (ex(m, Y) == 0) { hidx[m] = (long)heads.size(); heads.push_back(m); }
    const long t = (long)heads.size();
    std::vector<long> clen(t, 0);
    for (mono_t m : B) { mono_t h = m & ~((mono_t)0xff << (8 * Y)); clen[hidx[h]] = std::max<long>(clen[hidx[h]], ex(m, Y) + 1); }
    // stability: each y^{clen} * head is an LM of a y-polynomial; map head -> row
    std::vector<long> row_of_head(t, -1);
    long stab_fail = 0;
    for (size_t r = 0; r < ypoly.size(); r++) {
        mono_t lm = LMs[ypoly[r]]; mono_t h = lm & ~((mono_t)0xff << (8 * Y));
        auto it = hidx.find(h);
        if (it == hidx.end() || clen[it->second] != ex(lm, Y)) continue;
        row_of_head[it->second] = (long)r;
    }
    for (long k = 0; k < t; k++) if (row_of_head[k] < 0) stab_fail++;
    long sumdeg = 0, maxd = 0; for (long k = 0; k < t; k++) { sumdeg += clen[k]; maxd = std::max(maxd, clen[k]); }
    fprintf(stderr, "[hnfsolve] p=%ld n=%d D=%ld t=%ld |G|=%zu y-polys=%zu stab_fail=%ld maxchain=%ld terms=%ld\n", p, n, D, t, LMs.size(), ypoly.size(), stab_fail, maxd, nterms);
    if (stab_fail) { printf("{\"error\": \"stability assumption fails for %ld chain heads\", \"D\": %ld, \"t\": %ld}\n", stab_fail, D, t); return 3; }
    phase_end("staircase");

    // ---------------- build P ----------------
    phase_start();
    Mat<zz_pX> P; P.SetDims(t, t);
    {
        std::vector<std::vector<long>> tmp; // per row, dense coefficient staging is too big; fill directly
        for (long j = 0; j < t; j++) {
            long r = row_of_head[j];
            SetCoeff(P[j][j], clen[j]);  // y^{d_j} * head_j
            for (size_t q = tail_off[r]; q < tail_off[r + 1]; q++) {
                mono_t m = tmono[q]; int e = ex(m, Y); mono_t h = m & ~((mono_t)0xff << (8 * Y));
                auto it = hidx.find(h);
                if (it == hidx.end() || e >= clen[it->second]) { fprintf(stderr, "tail monomial outside staircase\n"); return 2; }
                zz_p c; conv(c, (long)tcoef[q]);
                zz_pX &ent = P[j][it->second];
                SetCoeff(ent, e, coeff(ent, e) + c);
            }
        }
    }
    std::vector<mono_t>().swap(tmono); std::vector<unsigned>().swap(tcoef);
    phase_end("build_P");

    // ---------------- determinant ----------------
    phase_start();
    zz_pX h;
    bool ok = determinant_generic_knowing_degree(h, P, D);
    const char *det_method = "lnz17_generic_knowing_degree";
    if (!ok || deg(h) != D) {
        fprintf(stderr, "[hnfsolve] generic determinant failed (deg %ld); falling back to linsolve\n", deg(h));
        determinant_via_linsolve(h, P); det_method = "linsolve_series";
    }
    if (deg(h) != D) { printf("{\"error\": \"determinant degree %ld != D %ld\"}\n", deg(h), D); return 4; }
    MakeMonic(h);
    phase_end("determinant");

    if (!elimout.empty()) { std::ofstream o(elimout); o << "["; for (long i = 0; i <= deg(h); i++) { if (i) o << ","; o << rep(coeff(h, i)); } o << "]\n"; }

    // ---------------- rational roots ----------------
    phase_start();
    zz_pXModulus H(h); zz_pX xp; PowerXMod(xp, p, H);
    zz_pX g = xp - zz_pX(INIT_MONO, 1); GCD(g, g, h);
    vec_zz_p roots; if (deg(g) > 0) FindRoots(roots, g);
    phase_end("rational_roots");

    // ---------------- points ----------------
    phase_start();
    long head_one = hidx[0];
    std::vector<long> head_x(n - 1, -1);
    for (int v = 0; v < n - 1; v++) { auto it = hidx.find((mono_t)1 << (8 * v)); if (it != hidx.end()) head_x[v] = it->second; }
    std::vector<std::vector<long>> points;
    for (long i = 0; i < roots.length(); i++) {
        Mat<zz_p> A; A.SetDims(t, t);
        for (long a = 0; a < t; a++) for (long b = 0; b < t; b++) A[b][a] = eval(P[a][b], roots[i]); // transpose
        Mat<zz_p> K; kernel(K, A);   // rows u with u * P(y0)^T = 0, i.e. P(y0) u = 0
        if (K.NumRows() != 1 || IsZero(K[0][head_one])) { fprintf(stderr, "[hnfsolve] root %ld: kernel dim %ld\n", rep(roots[i]), K.NumRows()); continue; }
        zz_p s = inv(K[0][head_one]);
        std::vector<long> pt(n);
        for (int v = 0; v < n - 1; v++) pt[v] = head_x[v] >= 0 ? rep(K[0][head_x[v]] * s) : -1;
        pt[n - 1] = rep(roots[i]);
        points.push_back(pt);
    }
    phase_end("points");

    // ---------------- planted check ----------------
    std::string planted_status = "none";
    if (!plantedpath.empty()) {
        std::ifstream pf(plantedpath); std::string l1, l2; std::getline(pf, l1); std::getline(pf, l2);
        std::vector<long> pv; std::stringstream ss(l2); std::string tok; while (std::getline(ss, tok, ',')) pv.push_back(atol(tok.c_str()));
        planted_status = "not_found";
        if ((long)pv.size() == n) for (auto &pt : points) if (pt == pv) planted_status = "recovered";
        zz_p hv = eval(h, conv<zz_p>(pv.empty() ? 0 : pv[n - 1]));
        if (planted_status != "recovered" && IsZero(hv)) planted_status = "root_of_eliminant_only";
    }

    double W = wall() - w0, C = cpu() - c0;
    printf("{\"p\": %ld, \"n\": %d, \"D\": %ld, \"t\": %ld, \"G\": %zu, \"max_chain\": %ld, \"terms\": %ld, \"det_method\": \"%s\", \"elim_degree\": %ld, \"rational_roots\": %ld, \"points\": [", p, n, D, t, LMs.size(), maxd, nterms, det_method, deg(h), (long)roots.length());
    for (size_t i = 0; i < points.size(); i++) { printf(i ? ", [" : "["); for (int v = 0; v < n; v++) printf(v ? ", %ld" : "%ld", points[i][v]); printf("]"); }
    printf("], \"planted\": \"%s\", \"phases\": {", planted_status.c_str());
    for (size_t i = 0; i < phases.size(); i++) printf("%s\"%s\": {\"wall_s\": %.3f, \"cpu_s\": %.3f}", i ? ", " : "", phases[i].name.c_str(), phases[i].w, phases[i].c);
    printf("}, \"total_wall_s\": %.3f, \"total_cpu_s\": %.3f, \"maxrss_kib\": %ld, \"threads\": %ld}\n", W, C, maxrss_kib(), threads);
    return 0;
}
