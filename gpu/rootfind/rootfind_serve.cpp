/* rootfind_serve.cpp -- persistent worker for the IC pipeline oracle
 * `rootfind-subtract-gpu` (src/cryptanalysis/ic_framework/rootfind_gpu.rs).
 *
 *   rootfind_serve --serve-bin        binary protocol on stdin/stdout
 *   rootfind_serve --daemon SOCKET    the same protocol on a Unix socket, one connection
 *                                     at a time, one CUDA context for all of them
 *   rootfind_serve --info             print the backend and exit
 *   rootfind_serve --bench-pairs      CUDA: time and compare the pair-table key kernels
 *
 * One process holds one rf64_session (rootfind_api.h): the factor base and
 * its lookup table are set up once, on the device for the CUDA build
 * (`make serve-cuda`), and each batch of targets is one kernel launch.
 * The host build (`make serve`) runs the same per-thread function on the
 * CPU (emulation backend) and is what the Rust tests drive.
 *
 * Protocol (little-endian; the worker only proposes relations, the caller
 * re-checks every one it uses in the group):
 *
 *   INIT  u32 'RFS3', u32 n_fb, u64 p, u32 backend, u32 lookup, u32 chunk,
 *         u32 max_targets, u64 x[n_fb], u64 y[n_fb], u32 n_probe, u64 px[n_probe],
 *         u64 curve_a   (canonical; n_probe = 0: look candidates up in the base; else
 *         probe mode against px; lookup = 4 (RF_LOOKUP_PAIRS_DEVICE): probe mode
 *         against the base's own negation-folded pair table, built by the library,
 *         hits carrying the first pair's build index -- rootfind_api.h)
 *   ->    u32 'RFS3', i32 status, u32 backend, u32 lookup, u32 chunk, u32 entries
 *   BATCH u32 'RFB1', u32 n, u64 x[n], u64 y[n]
 *   ->    u32 'RFB1', i32 status, u32 n_found, u32 n_written, f64 kernel_s,
 *         rf_relation[n_written]  (16 bytes: target, i, j, eps, tau, pad)
 *
 * A batch whose relations overflow the session's buffer answers status
 * RF_ERR_OVERFLOW (1) with a truncated list; the caller must not read it
 * as complete.  End of input ends the process (--serve-bin) or the connection
 * (--daemon; the session is destroyed and the next connection starts afresh).
 */
#include <cerrno>
#include <csignal>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <vector>

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

#include "rootfind_api.h"

static const uint32_t MAGIC_INIT = 0x33534652u;  /* "RFS3" */
static const uint32_t MAGIC_BATCH = 0x31424652u; /* "RFB1" */

static double now_s();
/* RF_SERVE_TIMING=1: print each INIT's receive and session-create times to stderr.
 * Off by default, so a worker's stderr stays deterministic. */
static const bool g_timing = std::getenv("RF_SERVE_TIMING") != nullptr;

static bool read_all(FILE *in, void *buf, size_t n) { return n == 0 || std::fread(buf, 1, n, in) == n; }
static void write_all(FILE *out, const void *buf, size_t n) { if (n) std::fwrite(buf, 1, n, out); }

/* One client: INIT and BATCH requests until end of input. */
static void serve(FILE *in, FILE *out) {
    rf64_session *s = nullptr;
    uint32_t cap = 0;
    std::vector<uint64_t> x, y, px;
    std::vector<rf_relation> rels;
    for (;;) {
        uint32_t magic = 0;
        if (!read_all(in, &magic, 4)) break;
        if (magic == MAGIC_INIT) {
            const double ti0 = now_s();
            uint32_t n_fb = 0, w[4], n_probe = 0;
            uint64_t p = 0;
            if (!read_all(in, &n_fb, 4) || !read_all(in, &p, 8) || !read_all(in, w, 16)) break;
            x.resize(n_fb); y.resize(n_fb);
            if (!read_all(in, x.data(), 8ull * n_fb) || !read_all(in, y.data(), 8ull * n_fb)) break;
            if (!read_all(in, &n_probe, 4)) break;
            px.resize(n_probe);
            if (!read_all(in, px.data(), 8ull * n_probe)) break;
            uint64_t curve_a = 0;
            if (!read_all(in, &curve_a, 8)) break;
            if (s) { rf64_session_destroy(s); s = nullptr; }
            rf_options opt{(int)w[0], (int)w[1], w[2]};
            const uint32_t max_targets = w[3];
            cap = 4u * max_targets + 4096u;
            rf_result info{};
            const double ti1 = now_s();
            uint32_t entries = 0;
            int rc = w[1] == RF_LOOKUP_PAIRS_DEVICE
                         ? rf64_session_create_pairs(p, curve_a, x.data(), y.data(), n_fb, max_targets, cap, &opt, &s,
                                                     &info, &entries)
                         : rf64_session_create(p, x.data(), y.data(), n_fb, n_probe ? px.data() : nullptr, n_probe,
                                               max_targets, cap, &opt, &s, &info);
            if (w[1] == RF_LOOKUP_PAIRS_DEVICE) n_probe = entries;
            if (g_timing)
                std::fprintf(stderr, "rootfind_serve: timing init n_probe=%u lookup=%d receive %.4f s create %.4f s\n",
                             n_probe, info.lookup_used, ti1 - ti0, now_s() - ti1);
            if (rc == RF_OK) {
                std::fprintf(stderr, "rootfind_serve: %s; n_fb=%u n_probe=%u lookup=%d chunk=%u max_targets=%u\n",
                             rf_backend_info(), n_fb, n_probe, info.lookup_used, info.chunk_used, max_targets);
            }
            uint32_t reply[6] = {MAGIC_INIT, (uint32_t)rc, (uint32_t)info.backend_used,
                                 (uint32_t)info.lookup_used, info.chunk_used, entries};
            write_all(out, reply, sizeof reply);
            std::fflush(out);
        } else if (magic == MAGIC_BATCH) {
            uint32_t n = 0;
            if (!read_all(in, &n, 4)) break;
            x.resize(n); y.resize(n);
            if (!read_all(in, x.data(), 8ull * n) || !read_all(in, y.data(), 8ull * n)) break;
            rf_result res{};
            rels.resize(cap ? cap : 1);
            int rc = s ? rf64_session_decompose(s, x.data(), y.data(), n, rels.data(), cap, &res) : RF_ERR_BAD_ARG;
            uint32_t head[4] = {MAGIC_BATCH, (uint32_t)rc, res.n_found, res.n_written};
            write_all(out, head, sizeof head);
            write_all(out, &res.seconds, 8);
            write_all(out, rels.data(), sizeof(rf_relation) * (size_t)res.n_written);
            std::fflush(out);
        } else {
            std::fprintf(stderr, "rootfind_serve: bad magic %#x\n", magic);
            break;
        }
    }
    rf64_session_destroy(s);
}

static double now_s() {
    timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return t.tv_sec + 1e-9 * t.tv_nsec;
}

static int daemon_main(const char *path) {
    std::signal(SIGPIPE, SIG_IGN);
    const double t0 = now_s();
    /* Create the CUDA context now, so no client pays for it. */
    const int wrc = rf_warmup();
    std::fprintf(stderr, "rootfind_serve: emulation threads %u\n", rf_emu_threads());
    std::fprintf(stderr, "rootfind_serve: daemon backend: %s (context %s, %.3f s)\n", rf_backend_info(),
                 wrc == 0 ? "ready" : "FAILED", now_s() - t0);
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) { std::perror("socket"); return 1; }
    sockaddr_un addr{};
    addr.sun_family = AF_UNIX;
    if (std::strlen(path) >= sizeof addr.sun_path) { std::fprintf(stderr, "socket path too long\n"); return 1; }
    std::strcpy(addr.sun_path, path);
    unlink(path);
    if (bind(fd, (sockaddr *)&addr, sizeof addr) != 0 || listen(fd, 8) != 0) { std::perror("bind/listen"); return 1; }
    std::fprintf(stderr, "rootfind_serve: daemon listening on %s (ready after %.3f s)\n", path, now_s() - t0);
    std::fflush(stderr);
    for (;;) {
        int c = accept(fd, nullptr, nullptr);
        if (c < 0) {
            if (errno == EINTR) continue;
            std::perror("accept");
            return 1;
        }
        int c2 = dup(c);
        FILE *in = fdopen(c, "rb"), *out = fdopen(c2, "wb");
        if (in && out) serve(in, out);
        if (in) std::fclose(in); else close(c);
        if (out) std::fclose(out); else close(c2);
    }
}

int main(int argc, char **argv) {
    if (argc > 1 && std::strcmp(argv[1], "--info") == 0) {
        std::printf("%s\n", rf_backend_info());
        return 0;
    }
    if (argc > 2 && std::strcmp(argv[1], "--daemon") == 0) return daemon_main(argv[2]);
    if (argc > 1 && std::strcmp(argv[1], "--bench-pairs") == 0) {
        /* p: the 40-bit curve's field; n: base sizes of the measured runs */
        const uint64_t p = 1099511627689ull;
        int bad = 0;
        for (uint32_t n : {1024u, 2048u, 4096u}) {
            double tc = 0, tr = 0;
            uint64_t mm = 0;
            if (rf_pairs_bench(p, n, 20261115ull + n, &tc, &tr, &mm) != 0) {
                std::printf("bench-pairs: n=%u failed (no CUDA backend?)\n", n);
                return 1;
            }
            std::printf("bench-pairs: n=%u keys=%llu per-cell %.4f s batched-runs %.4f s (%.1fx) mismatches %llu\n", n,
                        (unsigned long long)n * (n + 1), tc, tr, tr > 0 ? tc / tr : 0.0, (unsigned long long)mm);
            bad |= mm != 0;
        }
        return bad;
    }
    if (argc < 2 || std::strcmp(argv[1], "--serve-bin") != 0) {
        std::fprintf(stderr, "usage: %s --serve-bin | --daemon SOCKET | --info | --bench-pairs\n", argv[0]);
        return 2;
    }
    serve(stdin, stdout);
    return 0;
}
