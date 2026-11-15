/* rootfind_serve.cpp -- persistent worker for the IC pipeline oracle
 * `rootfind-subtract-gpu` (src/cryptanalysis/ic_framework/rootfind_gpu.rs).
 *
 *   rootfind_serve --serve-bin      binary protocol on stdin/stdout
 *   rootfind_serve --info           print the backend and exit
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
 *   INIT  u32 'RFS1', u32 n_fb, u64 p, u32 backend, u32 lookup, u32 chunk,
 *         u32 max_targets, u64 x[n_fb], u64 y[n_fb]          (canonical)
 *   ->    u32 'RFS1', i32 status, u32 backend, u32 lookup, u32 chunk, u32 0
 *   BATCH u32 'RFB1', u32 n, u64 x[n], u64 y[n]
 *   ->    u32 'RFB1', i32 status, u32 n_found, u32 n_written, f64 kernel_s,
 *         rf_relation[n_written]  (16 bytes: target, i, j, eps, tau, pad)
 *
 * A batch whose relations overflow the session's buffer answers status
 * RF_ERR_OVERFLOW (1) with a truncated list; the caller must not read it
 * as complete.  End of input ends the process.
 */
#include <cstdio>
#include <cstring>
#include <vector>

#include "rootfind_api.h"

static const uint32_t MAGIC_INIT = 0x31534652u;  /* "RFS1" */
static const uint32_t MAGIC_BATCH = 0x31424652u; /* "RFB1" */

static bool read_all(void *buf, size_t n) { return n == 0 || std::fread(buf, 1, n, stdin) == n; }
static void write_all(const void *buf, size_t n) { if (n) std::fwrite(buf, 1, n, stdout); }

int main(int argc, char **argv) {
    if (argc > 1 && std::strcmp(argv[1], "--info") == 0) {
        std::printf("%s\n", rf_backend_info());
        return 0;
    }
    if (argc < 2 || std::strcmp(argv[1], "--serve-bin") != 0) {
        std::fprintf(stderr, "usage: %s --serve-bin | --info\n", argv[0]);
        return 2;
    }
    rf64_session *s = nullptr;
    uint32_t cap = 0, max_targets = 0;
    std::vector<uint64_t> x, y;
    std::vector<rf_relation> rels;
    for (;;) {
        uint32_t magic = 0;
        if (!read_all(&magic, 4)) break;
        if (magic == MAGIC_INIT) {
            uint32_t n_fb = 0, w[4];
            uint64_t p = 0;
            if (!read_all(&n_fb, 4) || !read_all(&p, 8) || !read_all(w, 16)) break;
            x.resize(n_fb); y.resize(n_fb);
            if (!read_all(x.data(), 8ull * n_fb) || !read_all(y.data(), 8ull * n_fb)) break;
            if (s) { rf64_session_destroy(s); s = nullptr; }
            rf_options opt{(int)w[0], (int)w[1], w[2]};
            max_targets = w[3];
            cap = 4u * max_targets + 4096u;
            rf_result info{};
            int rc = rf64_session_create(p, x.data(), y.data(), n_fb, max_targets, cap, &opt, &s, &info);
            if (rc == RF_OK) {
                std::fprintf(stderr, "rootfind_serve: %s; n_fb=%u lookup=%d chunk=%u max_targets=%u\n",
                             rf_backend_info(), n_fb, info.lookup_used, info.chunk_used, max_targets);
            }
            uint32_t reply[6] = {MAGIC_INIT, (uint32_t)rc, (uint32_t)info.backend_used,
                                 (uint32_t)info.lookup_used, info.chunk_used, 0};
            write_all(reply, sizeof reply);
            std::fflush(stdout);
        } else if (magic == MAGIC_BATCH) {
            uint32_t n = 0;
            if (!read_all(&n, 4)) break;
            x.resize(n); y.resize(n);
            if (!read_all(x.data(), 8ull * n) || !read_all(y.data(), 8ull * n)) break;
            rf_result res{};
            rels.resize(cap ? cap : 1);
            int rc = s ? rf64_session_decompose(s, x.data(), y.data(), n, rels.data(), cap, &res) : RF_ERR_BAD_ARG;
            uint32_t head[4] = {MAGIC_BATCH, (uint32_t)rc, res.n_found, res.n_written};
            write_all(head, sizeof head);
            write_all(&res.seconds, 8);
            write_all(rels.data(), sizeof(rf_relation) * (size_t)res.n_written);
            std::fflush(stdout);
        } else {
            std::fprintf(stderr, "rootfind_serve: bad magic %#x\n", magic);
            break;
        }
    }
    rf64_session_destroy(s);
    return 0;
}
