// hnfsolve_u: dump of the multiplication-by-w structure for the u-ring presentation
// (PROTOCOL.md amendment A3). Derived from research/ecgfp5_quotient_bw_finish_20261102/src/hnfsolve.cpp.
//
//   msolve -g 2 -f uring.ms -o FIFO &   hnfsolve_u --gb FIFO --dump-P P2.bin
//
// The reduced grevlex basis of the u-ring system (variables s1..s_{n-2}, s = s_{n-2} last
// but one, w last) has the leading monomial w^2 (element w^2 - s) and otherwise pure
// s-monomial leading monomials, so the staircase is S x {1, w}: t = D/2 chains of length 2
// in w, which fails the BNS22 stability check of hnfsolve. Multiplication by w is still
// read off the basis: w * m = (w m) and w * (w m) = NF(s m), which is the chain head s m
// when s m lies in S, and otherwise the negated tail of the basis element with leading
// monomial s m ("s-stability", checked here). This writes P = diag(w^2) + Q in a sparse
// format (HNFP2): the unit rows as (k, position, coefficient) and the other rows dense.
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
static void phase_end(const char *n) { phases.push_back({n, wall() - pw, cpu() - pc}); fprintf(stderr, "[hnfsolve_u] %s: %.2f s wall, %.2f s cpu, maxrss %ld KiB\n", n, wall() - pw, cpu() - pc, maxrss_kib()); }

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
    std::string gbpath, dumppath;
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        if (a == "--gb" && i + 1 < argc) gbpath = argv[++i];
        else if (a == "--dump-P" && i + 1 < argc) dumppath = argv[++i];
        else { fprintf(stderr, "usage: hnfsolve_u --gb FILE --dump-P FILE\n"); return 2; }
    }
    if (gbpath.empty() || dumppath.empty()) { fprintf(stderr, "--gb and --dump-P required\n"); return 2; }
    FILE *f = nullptr;
    for (int attempt = 0; attempt < 16; attempt++) {
        f = fopen(gbpath.c_str(), "rb");
        if (!f) { perror("open gb"); return 2; }
        int c0c = fgetc(f);
        if (c0c != EOF) { ungetc(c0c, f); break; }
        fclose(f); f = nullptr;
        fprintf(stderr, "[hnfsolve_u] empty open of %s (attempt %d); reopening\n", gbpath.c_str(), attempt + 1);
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
    if (p <= 1 || n < 3 || n > MAXV) { fprintf(stderr, "bad header p=%ld n=%d\n", p, n); return 2; }
    const int Y = n - 1, SV = n - 2;
    std::unordered_map<std::string, int> vidx; for (int v = 0; v < n; v++) vidx[vars[v]] = v;
    if (R.get() != '[') { fprintf(stderr, "expected [\n"); return 2; }
    std::vector<mono_t> LMs; std::vector<size_t> tail_off; std::vector<mono_t> tmono; std::vector<unsigned> tcoef;
    long nterms = 0; bool done = false;
    while (!done) {
        bool first = true;
        while (true) {
            unsigned long long c = 0; int ch = R.peek(); bool hasc = false;
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
                LMs.push_back(m); tail_off.push_back(tmono.size()); first = false;
            } else { tmono.push_back(m); tcoef.push_back((unsigned)(c % p)); }
            int d = R.get();
            while (d == '\n' || d == ' ') d = R.get();
            if (d == '+') continue;
            if (d == ',') { int q; while ((q = R.peek()) == '\n' || q == ' ') R.get(); break; }
            if (d == ']') { done = true; break; }
            fprintf(stderr, "parse error: unexpected '%c'\n", d); return 2;
        }
    }
    tail_off.push_back(tmono.size());
    fclose(f);
    phase_end("parse_gb");

    phase_start();
    std::unordered_map<mono_t, long> LMidx; for (size_t r = 0; r < LMs.size(); r++) LMidx[LMs[r]] = (long)r;
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
    std::vector<mono_t> heads; std::unordered_map<mono_t, long> hidx;
    for (mono_t m : B) if (ex(m, Y) == 0) { hidx[m] = (long)heads.size(); heads.push_back(m); }
    const long t = (long)heads.size();
    // structure checks: every chain is {m, w m}; the element with LM w^2 is w^2 - s
    long bad_chain = 0;
    for (long k = 0; k < t; k++) if (!Bidx.count(heads[k] + ((mono_t)1 << (8 * Y))) || Bidx.count(heads[k] + ((mono_t)2 << (8 * Y)))) bad_chain++;
    const mono_t W2 = (mono_t)2 << (8 * Y), SM = (mono_t)1 << (8 * SV);
    auto itw = LMidx.find(W2);
    bool w2ok = itw != LMidx.end() && tail_off[itw->second + 1] - tail_off[itw->second] == 1 &&
                tmono[tail_off[itw->second]] == SM && tcoef[tail_off[itw->second]] == (unsigned)(p - 1);
    long nonpure = 0; for (mono_t l : LMs) if (ex(l, Y) > 0 && l != W2) nonpure++;
    // s-stability: for every head m with s m outside S, s m is a leading monomial
    long sparse_rows = 0, dense_rows = 0, stab_fail = 0;
    std::vector<long> kind(t);   // >= 0: index of the chain head s m (unit row); -1: dense
    std::vector<long> drow(t, -1);
    for (long k = 0; k < t; k++) {
        mono_t sm = heads[k] + SM;
        auto ih = hidx.find(sm);
        if (ih != hidx.end()) { kind[k] = ih->second; sparse_rows++; continue; }
        kind[k] = -1;
        auto il = LMidx.find(sm);
        if (il == LMidx.end()) { stab_fail++; continue; }
        drow[k] = il->second; dense_rows++;
    }
    fprintf(stderr, "[hnfsolve_u] p=%ld n=%d D=%ld t=%ld |G|=%zu terms=%ld bad_chain=%ld w2_is_w2_minus_s=%d nonpure_LM=%ld unit_rows=%ld dense_rows=%ld stab_fail=%ld\n",
            p, n, D, t, LMs.size(), nterms, bad_chain, (int)w2ok, nonpure, sparse_rows, dense_rows, stab_fail);
    if (bad_chain || !w2ok || nonpure || stab_fail) {
        printf("{\"error\": \"u-ring structure check failed\", \"D\": %ld, \"t\": %ld, \"bad_chain\": %ld, \"w2ok\": %d, \"nonpure_LM\": %ld, \"stab_fail\": %ld}\n", D, t, bad_chain, (int)w2ok, nonpure, stab_fail);
        return 3;
    }
    phase_end("staircase");

    // HNFP2, little-endian: "HNFP2\0\0\0"; int64 p, n, t, D, Y; int64 clen[t]; uint64 head_mono[t];
    // int64 off[t]; int64 nunit; nunit x (int64 k, int64 pos, int64 coef); int64 ndense;
    // ndense x (int64 k, uint32[D]). Row k of Q: P = diag(w^2) + Q, positions off[j] + e <-> w^e head_j.
    phase_start();
    FILE *o = fopen(dumppath.c_str(), "wb");
    if (!o) { perror("open dump"); return 2; }
    fwrite("HNFP2\0\0\0", 1, 8, o);
    long long hdr[5] = {p, n, t, D, Y}; fwrite(hdr, 8, 5, o);
    std::vector<long long> cl(t, 2), off(t);
    for (long k = 0; k < t; k++) off[k] = 2 * k;
    fwrite(cl.data(), 8, t, o);
    std::vector<unsigned long long> hm(heads.begin(), heads.end()); fwrite(hm.data(), 8, t, o);
    fwrite(off.data(), 8, t, o);
    long long nu = sparse_rows; fwrite(&nu, 8, 1, o);
    for (long k = 0; k < t; k++) if (kind[k] >= 0) { long long trip[3] = {k, off[kind[k]], p - 1}; fwrite(trip, 8, 3, o); }
    long long nd = dense_rows; fwrite(&nd, 8, 1, o);
    std::vector<unsigned> row(D);
    for (long k = 0; k < t; k++) {
        if (kind[k] >= 0) continue;
        long r = drow[k];
        std::fill(row.begin(), row.end(), 0u);
        for (size_t q = tail_off[r]; q < tail_off[r + 1]; q++) {
            mono_t m = tmono[q]; int e = ex(m, Y); mono_t h = m & ~((mono_t)0xff << (8 * Y));
            auto it = hidx.find(h);
            if (it == hidx.end() || e >= 2) { fprintf(stderr, "tail monomial outside staircase\n"); return 2; }
            unsigned &c = row[off[it->second] + e]; c = (unsigned)((c + (unsigned long long)tcoef[q]) % p);
        }
        long long kk = k; fwrite(&kk, 8, 1, o); fwrite(row.data(), 4, D, o);
    }
    fclose(o);
    phase_end("dump_P");
    printf("{\"dumped\": \"%s\", \"p\": %ld, \"n\": %d, \"t\": %ld, \"D\": %ld, \"unit_rows\": %ld, \"dense_rows\": %ld}\n", dumppath.c_str(), p, n, t, D, sparse_rows, dense_rows);
    return 0;
}
