// bwfinish: block-Wiedemann finish for a zero-dimensional ideal whose reduced grevlex basis
// satisfies the BNS22 stability assumption, read from hnfsolve --dump-P (P = diag(y^d) + Q).
//
// In the staircase basis ordered by chains (index off[k]+e <-> y^e * head_k), multiplication
// by y is a shift inside each chain plus, at each chain top, the normal form of
// y^{d_k} head_k = -sum_j Q[k][j](y) head_j. So M = Shift - Q^T E_top, and a block product
// M V (V: D x b) costs one dense GEMM (D x t)(t x b) plus a shift. The sequence
// S_i = U M^i V (b x b, i < L) is computed by exact fp64 GEMM (operands centred mod p,
// split into 13-bit limbs when needed so every accumulation stays below 2^53).
// A left approximant basis of [F; -I] (F = sum S_i x^i) gives the minimal left generator
// a(x); det a(x) = lc * rev_D(charpoly of M) in the generic case; the eliminant is its
// reversal, checked to have degree D. Points: rational roots y0, then kernel of P(y0).
//
// usage: bwfinish --P FILE [--b 32] [--seed 1] [--planted FILE] [--elim-out F] [--threads N]
#include <NTL/lzz_pX.h>
#include <NTL/lzz_pXFactoring.h>
#include <NTL/mat_lzz_p.h>
#include <NTL/BasicThreadPool.h>
#include <cblas.h>
#include <sys/resource.h>
#include <sys/time.h>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <random>
#include <sstream>
#include <string>
#include <vector>
#include "mat_lzz_pX_approximant.h"
#include "mat_lzz_pX_determinant.h"
#include "mat_lzz_pX_forms.h"

NTL_CLIENT
using namespace PML;

static double wall() { struct timeval tv; gettimeofday(&tv, nullptr); return tv.tv_sec + 1e-6 * tv.tv_usec; }
static double cpu() { struct rusage r; getrusage(RUSAGE_SELF, &r); return r.ru_utime.tv_sec + 1e-6 * r.ru_utime.tv_usec + r.ru_stime.tv_sec + 1e-6 * r.ru_stime.tv_usec; }
static long maxrss_kib() { struct rusage r; getrusage(RUSAGE_SELF, &r); return r.ru_maxrss; }
struct Phase { std::string name; double w, c; };
static std::vector<Phase> phases; static double pw, pc;
static void phase_start() { pw = wall(); pc = cpu(); }
static void phase_end(const char *n) { phases.push_back({n, wall() - pw, cpu() - pc}); fprintf(stderr, "[bwfinish] %s: %.2f s wall, %.2f s cpu, maxrss %ld KiB\n", n, wall() - pw, cpu() - pc, maxrss_kib()); }

static long P_MOD; static double PD, PINV;
static inline double redc(double x) { double q = std::floor(x * PINV + 0.5); x -= q * PD; return x; }  // centred
static inline long tolong(double x) { long v = (long)x % P_MOD; if (v < 0) v += P_MOD; return v; }

// exact C = A (m x k) * B (k x n) mod p, all row-major, entries centred in (-p/2, p/2].
// A is given as nl limbs (A = sum A_l 2^{13 l}); B is plain. Result centred.
static void mm_mod(std::vector<double> &C, const std::vector<std::vector<double>> &Al, const double *B,
                   long m, long k, long n, bool accumulate_neg_into = false) {
    // when accumulate_neg_into, C <- C - A*B (C already centred), else C <- A*B
    std::vector<double> T((size_t)m * n);
    std::vector<double> R((size_t)m * n, 0.0);
    for (size_t l = 0; l < Al.size(); l++) {
        cblas_dgemm(CblasRowMajor, CblasNoTrans, CblasNoTrans, m, n, k, 1.0, Al[l].data(), k, B, n, 0.0, T.data(), n);
        double sc = std::ldexp(1.0, 13 * (int)l);
        for (size_t i = 0; i < T.size(); i++) R[i] = redc(R[i] + redc(redc(T[i]) * sc));
    }
    if (accumulate_neg_into) { for (size_t i = 0; i < R.size(); i++) C[i] = redc(C[i] - R[i]); }
    else C.swap(R);
}

static std::vector<std::vector<double>> limbs(const std::vector<double> &A, int nl) {
    std::vector<std::vector<double>> L(nl, std::vector<double>(A.size()));
    for (size_t i = 0; i < A.size(); i++) {
        double x = A[i];
        if (nl == 1) { L[0][i] = x; continue; }
        // x centred, |x| < 2^25: x = hi*2^13 + lo, lo centred in [-2^12, 2^12)
        double lo = x - std::ldexp(std::floor(std::ldexp(x, -13) + 0.5), 13);
        double hi = (x - lo) / 8192.0;
        L[0][i] = lo; L[1][i] = hi;
    }
    return L;
}

int main(int argc, char **argv) {
    std::string Ppath, planted, elimout; long b = 32, seed = 1, threads = 4;
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        if (a == "--P" && i + 1 < argc) Ppath = argv[++i];
        else if (a == "--b" && i + 1 < argc) b = atol(argv[++i]);
        else if (a == "--seed" && i + 1 < argc) seed = atol(argv[++i]);
        else if (a == "--planted" && i + 1 < argc) planted = argv[++i];
        else if (a == "--elim-out" && i + 1 < argc) elimout = argv[++i];
        else if (a == "--threads" && i + 1 < argc) threads = atol(argv[++i]);
        else { fprintf(stderr, "usage: bwfinish --P FILE [--b 32] [--seed 1] [--planted F] [--elim-out F] [--threads N]\n"); return 2; }
    }
    openblas_set_num_threads((int)threads); SetNumThreads(threads);
    double w0 = wall(), c0 = cpu();

    // ---- load ----
    phase_start();
    FILE *f = fopen(Ppath.c_str(), "rb"); if (!f) { perror("open P"); return 2; }
    char mg[8]; if (fread(mg, 1, 8, f) != 8 || memcmp(mg, "HNFP1", 5)) { fprintf(stderr, "bad magic\n"); return 2; }
    long long hdr[5]; if (fread(hdr, 8, 5, f) != 5) return 2;
    const long p = hdr[0], n = hdr[1], t = hdr[2], D = hdr[3], Y = hdr[4];
    std::vector<long long> cl(t), off(t); std::vector<unsigned long long> hm(t);
    if (fread(cl.data(), 8, t, f) != (size_t)t || fread(hm.data(), 8, t, f) != (size_t)t || fread(off.data(), 8, t, f) != (size_t)t) return 2;
    P_MOD = p; PD = (double)p; PINV = 1.0 / PD; zz_p::init(p);
    const int nl = ((double)(p / 2) * (p / 2) * (double)std::max(t, D) < 4.0e15) ? 1 : 2;
    // QT: D x t row-major (QT[r][k] = Q[k][r]); stored as limbs, centred
    std::vector<double> QT((size_t)D * t);
    {
        std::vector<unsigned> row(D);
        for (long k = 0; k < t; k++) {
            if (fread(row.data(), 4, D, f) != (size_t)D) { fprintf(stderr, "short P\n"); return 2; }
            for (long r = 0; r < D; r++) { double v = row[r]; if (v > p / 2) v -= p; QT[(size_t)r * t + k] = v; }
        }
    }
    fclose(f);
    std::vector<std::vector<double>> QTl;
    if (nl == 1) { QTl.emplace_back(); QTl[0].swap(QT); }   // no copy
    else { QTl = limbs(QT, nl); std::vector<double>().swap(QT); }
    std::vector<long> top(t); for (long k = 0; k < t; k++) top[k] = off[k] + cl[k] - 1;
    fprintf(stderr, "[bwfinish] p=%ld n=%ld t=%ld D=%ld b=%ld limbs=%d\n", p, n, t, D, b, nl);
    phase_end("load");

    // ---- sequence ----
    phase_start();
    const long L = 2 * ((D + b - 1) / b) + 4;
    std::mt19937_64 rng(seed);
    auto rnd = [&]() { double v = (double)(rng() % (unsigned long long)p); if (v > p / 2) v -= p; return v; };
    std::vector<double> V((size_t)D * b), W((size_t)D * b), Vtop((size_t)t * b);
    std::vector<double> U((size_t)b * D);
    for (auto &x : V) x = rnd();
    for (auto &x : U) x = rnd();
    std::vector<std::vector<double>> Ul = limbs(U, nl);
    std::vector<std::vector<long>> S(L, std::vector<long>(b * b));
    std::vector<double> Sd;
    for (long i = 0; i < L; i++) {
        mm_mod(Sd, Ul, V.data(), b, D, b);
        for (long q = 0; q < b * b; q++) S[i][q] = tolong(Sd[q]);
        if (i == L - 1) break;
        // W = Shift(V) - QT * Vtop
        for (long k = 0; k < t; k++) {
            double *dst = &W[(size_t)off[k] * b];
            std::fill(dst, dst + b, 0.0);
            memcpy(dst + b, &V[(size_t)off[k] * b], sizeof(double) * b * (cl[k] - 1));
            memcpy(&Vtop[(size_t)k * b], &V[(size_t)top[k] * b], sizeof(double) * b);
        }
        mm_mod(W, QTl, Vtop.data(), D, t, b, true);
        V.swap(W);
        if ((i & 511) == 0) fprintf(stderr, "[bwfinish] step %ld / %ld (%.1f s)\n", i, L, wall() - pw);
    }
    phase_end("sequence");

    // ---- matrix generator ----
    phase_start();
    Mat<zz_pX> pmat; pmat.SetDims(2 * b, b);
    for (long r = 0; r < b; r++) for (long c = 0; c < b; c++) {
        zz_pX &e = pmat[r][c];
        for (long i = 0; i < L; i++) SetCoeff(e, i, conv<zz_p>(S[i][r * b + c]));
        e.normalize();
    }
    for (long r = 0; r < b; r++) SetCoeff(pmat[b + r][r], 0, conv<zz_p>(-1));
    Mat<zz_pX> app; VecLong shift(2 * b, 0);
    pmbasis(app, pmat, L, shift);
    // pick the b rows of smallest shifted degree
    std::vector<long> idx(2 * b); for (long i = 0; i < 2 * b; i++) idx[i] = i;
    std::sort(idx.begin(), idx.end(), [&](long x, long y) { return shift[x] < shift[y]; });
    Mat<zz_pX> A; A.SetDims(b, b);
    for (long r = 0; r < b; r++) for (long c = 0; c < b; c++) A[r][c] = app[idx[r]][c];
    long sumdeg = 0; for (long r = 0; r < b; r++) sumdeg += shift[idx[r]];
    phase_end("generator");

    phase_start();
    zz_pX da;
    if (!determinant_generic_knowing_degree(da, A, sumdeg) || deg(da) != sumdeg) {
        fprintf(stderr, "[bwfinish] generic det failed; linsolve fallback\n");
        determinant_via_linsolve(da, A);
    }
    // eliminant = rev_D(det A), made monic
    zz_pX h; h.SetLength(D + 1);
    for (long i = 0; i <= D; i++) SetCoeff(h, D - i, coeff(da, i));
    h.normalize();
    long hdeg = deg(h);
    if (hdeg > 0) MakeMonic(h);
    phase_end("determinant");
    fprintf(stderr, "[bwfinish] sum of generator row degrees %ld, deg det %ld, eliminant degree %ld\n", sumdeg, deg(da), hdeg);
    if (!elimout.empty()) { std::ofstream o(elimout); o << "["; for (long i = 0; i <= deg(h); i++) { if (i) o << ","; o << rep(coeff(h, i)); } o << "]\n"; }
    if (hdeg != D) { printf("{\"error\": \"eliminant degree %ld != D %ld\", \"deg_det\": %ld}\n", hdeg, D, deg(da)); return 4; }

    // ---- rational roots and points ----
    phase_start();
    zz_pXModulus H(h); zz_pX xp; PowerXMod(xp, p, H);
    zz_pX g = xp - zz_pX(INIT_MONO, 1); GCD(g, g, h);
    vec_zz_p roots; if (deg(g) > 0) FindRoots(roots, g);
    phase_end("rational_roots");

    phase_start();
    // need Q again as integers (from limbs)
    auto Qkr = [&](long k, long r) -> long { double v = QTl[0][(size_t)r * t + k]; if (nl > 1) v += 8192.0 * QTl[1][(size_t)r * t + k]; return tolong(v); };
    long head_one = -1; std::vector<long> head_x(n - 1, -1);
    for (long k = 0; k < t; k++) { if (hm[k] == 0) head_one = k; for (long v = 0; v < n - 1; v++) if (hm[k] == (1ULL << (8 * v))) head_x[v] = k; }
    std::vector<std::vector<long>> points;
    for (long i = 0; i < roots.length(); i++) {
        zz_p y0 = roots[i];
        std::vector<zz_p> pw(40); pw[0] = 1; for (int e = 1; e < 40; e++) pw[e] = pw[e - 1] * y0;
        Mat<zz_p> AT; AT.SetDims(t, t);  // AT[col][row] = P(y0)[row][col]
        for (long j = 0; j < t; j++) {
            AT[j][j] += pw[cl[j]];
            for (long k = 0; k < t; k++) { zz_p s(0); for (long e = 0; e < cl[k]; e++) s += conv<zz_p>(Qkr(j, off[k] + e)) * pw[e]; AT[k][j] += s; }
        }
        Mat<zz_p> K; kernel(K, AT);
        if (K.NumRows() != 1 || head_one < 0 || IsZero(K[0][head_one])) { fprintf(stderr, "[bwfinish] root %ld: kernel dim %ld\n", rep(y0), K.NumRows()); continue; }
        zz_p s = inv(K[0][head_one]);
        std::vector<long> pt(n);
        for (long v = 0; v < n - 1; v++) pt[v] = head_x[v] >= 0 ? rep(K[0][head_x[v]] * s) : -1;
        pt[n - 1] = rep(y0);
        points.push_back(pt);
    }
    phase_end("points");
    (void)Y;

    std::string pst = "none";
    if (!planted.empty()) {
        std::ifstream pf(planted); std::string l1, l2; std::getline(pf, l1); std::getline(pf, l2);
        std::vector<long> pv; std::stringstream ss(l2); std::string tok; while (std::getline(ss, tok, ',')) pv.push_back(atol(tok.c_str()));
        pst = "not_found"; for (auto &pt : points) if ((long)pv.size() == n && pt == pv) pst = "recovered";
    }
    printf("{\"p\": %ld, \"n\": %ld, \"D\": %ld, \"t\": %ld, \"b\": %ld, \"limbs\": %d, \"L\": %ld, \"elim_degree\": %ld, \"rational_roots\": %ld, \"points\": [", p, n, D, t, b, nl, L, hdeg, roots.length());
    for (size_t i = 0; i < points.size(); i++) { printf("%s[", i ? ", " : ""); for (long v = 0; v < n; v++) printf("%s%ld", v ? ", " : "", points[i][v]); printf("]"); }
    printf("], \"planted\": \"%s\", \"phases\": {", pst.c_str());
    for (size_t i = 0; i < phases.size(); i++) printf("%s\"%s\": {\"wall_s\": %.3f, \"cpu_s\": %.3f}", i ? ", " : "", phases[i].name.c_str(), phases[i].w, phases[i].c);
    printf("}, \"total_wall_s\": %.3f, \"total_cpu_s\": %.3f, \"maxrss_kib\": %ld, \"threads\": %ld}\n", wall() - w0, cpu() - c0, maxrss_kib(), threads);
    return 0;
}
