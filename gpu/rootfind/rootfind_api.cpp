/* rootfind_api.cpp -- the C ABI (rootfind_api.h): argument validation,
 * canonical -> Montgomery conversion, table construction, and dispatch to
 * the emulation backend or (when built with -DRF_HAVE_CUDA=1 and linked
 * against rootfind_cuda.o) the CUDA backend.
 *
 * Built two ways:
 *   make lib        -> librootfind_oracle.a   (emulation only; what cargo
 *                      compiles via build.rs for the `gpu-rootfind` feature)
 *   make lib-cuda   -> librootfind_cuda.a     (emulation + CUDA; needs nvcc)
 */
#include <cstdlib>
#include <cstring>
#include <unordered_map>
#include <string>

#include "rootfind_host.hpp"

#ifndef RF_HAVE_CUDA
#define RF_HAVE_CUDA 0
#endif

#if RF_HAVE_CUDA
/* implemented in rootfind_cuda.cu */
extern "C" int rf_cuda_decompose64(const Fp64Ctx *f,
                                   const uint64_t *fb_x, const uint64_t *fb_y, uint32_t n_fb,
                                   const uint64_t *tx, const uint64_t *ty, uint32_t n_targets,
                                   int lookup, uint32_t chunk,
                                   rf_relation *out, uint32_t cap, uint32_t *n_found, double *seconds);
extern "C" int rf_cuda_decompose256(const Fp256Ctx *f,
                                    const U256x64 *fb_x, const U256x64 *fb_y, uint32_t n_fb,
                                    const U256x64 *tx, const U256x64 *ty, uint32_t n_targets,
                                    int lookup, uint32_t chunk,
                                    rf_relation *out, uint32_t cap, uint32_t *n_found, double *seconds);
extern "C" int rf_cuda_device_present(char *name, int name_len);
extern "C" int rf_cuda_session64_create(const Fp64Ctx *f, const uint64_t *fb_x, const uint64_t *fb_y,
                                        uint32_t n_fb, const uint64_t *probe_x, uint32_t n_probe,
                                        int lookup, uint32_t chunk,
                                        uint32_t max_targets, uint32_t cap, void **out);
extern "C" int rf_cuda_session64_run(void *h, const uint64_t *tx, const uint64_t *ty, uint32_t n_targets,
                                     rf_relation *out, uint32_t *n_found, double *seconds);
extern "C" void rf_cuda_session64_destroy(void *h);
extern "C" int rf_cuda_warmup(void);
extern "C" int rf_cuda_pairs_bench(uint64_t p, uint32_t n, uint64_t seed, double *t_cell, double *t_run,
                                   uint64_t *mismatch);
extern "C" int rf_cuda_session64_create_pairs(const Fp64Ctx *f, uint64_t a_m, const uint64_t *fb_x,
                                              const uint64_t *fb_y, uint32_t n_fb, uint32_t chunk,
                                              uint32_t max_targets, uint32_t cap, void **out,
                                              uint32_t *n_entries, uint32_t **pairb_host);
#endif

namespace {

template <class F>
int run_emulated(const typename F::ctx &f,
                 const std::vector<typename F::elt> &fbx, const std::vector<typename F::elt> &fby,
                 const std::vector<typename F::elt> &tx, const std::vector<typename F::elt> &ty,
                 int lookup, uint32_t chunk, rf_relation *out, uint32_t cap, rf_result *res) {
    DecompArgs<F> a;
    a.f = f;
    a.fb_x = fbx.data(); a.fb_y = fby.data(); a.n_fb = (uint32_t)fbx.size();
    a.probe_x = fbx.data(); a.probe_mode = 0;
    a.t_x = tx.data(); a.t_y = ty.data(); a.n_targets = (uint32_t)tx.size();
    a.out = out; a.out_cap = cap;
    uint32_t count = 0;
    a.out_count = &count;

    double t0 = rf::now_seconds();
    if (lookup == RF_LOOKUP_HASH) {
        rf::HashTableHost h = rf::build_hash<F>(f, fbx.data(), a.n_fb);
        HashTable view = h.view();
        t0 = rf::now_seconds();
        rf::emulate_chunk<F, HashTable>(a, view, chunk);
    } else {
        rf::SortedTableHost s = rf::build_sorted<F>(f, fbx.data(), a.n_fb);
        SortedTable view = s.view();
        t0 = rf::now_seconds();
        rf::emulate_chunk<F, SortedTable>(a, view, chunk);
    }
    res->seconds = rf::now_seconds() - t0;
    res->n_found = count;
    res->n_written = count < cap ? count : cap;
    res->backend_used = RF_BACKEND_EMULATE;
    res->lookup_used = lookup;
    res->chunk_used = chunk;
    return count > cap ? RF_ERR_OVERFLOW : RF_OK;
}

int resolve_backend(int requested) {
    if (requested == RF_BACKEND_AUTO) return rf_cuda_available() ? RF_BACKEND_CUDA : RF_BACKEND_EMULATE;
    return requested;
}

/* `default_chunk` is the per-field library default substituted for
 * chunk == 0 (RF_DEFAULT_CHUNK_64 / RF_DEFAULT_CHUNK_256); every wrapper
 * passes 0 through so the policy lives in exactly one place. */
int check_common(const void *fb_x, const void *fb_y, uint32_t n_fb, const void *tx, const void *ty,
                 uint32_t n_targets, rf_relation *out, uint32_t cap, rf_options *opt, rf_result *res,
                 uint32_t default_chunk) {
    if (!fb_x || !fb_y || !tx || !ty || !res || n_fb == 0 || n_targets == 0) return RF_ERR_BAD_ARG;
    if (cap > 0 && !out) return RF_ERR_BAD_ARG;
    std::memset(res, 0, sizeof(*res));
    if (opt->chunk == 0) opt->chunk = default_chunk;
    if (!rf::chunk_ok(opt->chunk)) return RF_ERR_BAD_CHUNK;
    if (opt->lookup != RF_LOOKUP_AUTO && opt->lookup != RF_LOOKUP_SORTED && opt->lookup != RF_LOOKUP_HASH) return RF_ERR_BAD_ARG;
    if (opt->backend != RF_BACKEND_AUTO && opt->backend != RF_BACKEND_EMULATE && opt->backend != RF_BACKEND_CUDA) return RF_ERR_BAD_ARG;
    return RF_OK;
}

} /* namespace */

extern "C" int rf64_decompose(uint64_t p,
                              const uint64_t *fb_x, const uint64_t *fb_y, uint32_t n_fb,
                              const uint64_t *tx, const uint64_t *ty, uint32_t n_targets,
                              rf_relation *out, uint32_t cap,
                              const rf_options *opt_in, rf_result *res) {
    rf_options opt = opt_in ? *opt_in : rf_options{RF_BACKEND_AUTO, RF_LOOKUP_AUTO, 0};
    int rc = check_common(fb_x, fb_y, n_fb, tx, ty, n_targets, out, cap, &opt, res, RF_DEFAULT_CHUNK_64);
    if (rc != RF_OK) return rc;
    if ((p & 1) == 0 || p <= 3 || p >= (1ull << 62)) return RF_ERR_BAD_ARG;
    for (uint32_t i = 0; i < n_fb; i++) if (fb_x[i] >= p || fb_y[i] >= p) return RF_ERR_BAD_ARG;
    for (uint32_t k = 0; k < n_targets; k++) if (tx[k] >= p || ty[k] >= p) return RF_ERR_BAD_ARG;

    Fp64Ctx f = fp64_ctx_new(p);
    std::vector<uint64_t> fbx = rf::to_mont64(f, fb_x, n_fb), fby = rf::to_mont64(f, fb_y, n_fb);
    std::vector<uint64_t> mx = rf::to_mont64(f, tx, n_targets), my = rf::to_mont64(f, ty, n_targets);
    int lookup = rf::resolve_lookup(opt.lookup, n_fb);
    int backend = resolve_backend(opt.backend);

    if (backend == RF_BACKEND_CUDA) {
#if RF_HAVE_CUDA
        uint32_t found = 0;
        double secs = 0;
        rc = rf_cuda_decompose64(&f, fbx.data(), fby.data(), n_fb, mx.data(), my.data(), n_targets,
                                 lookup, opt.chunk, out, cap, &found, &secs);
        res->n_found = found;
        res->n_written = found < cap ? found : cap;
        res->backend_used = RF_BACKEND_CUDA;
        res->lookup_used = lookup;
        res->chunk_used = opt.chunk;
        res->seconds = secs;
        if (rc != RF_OK) return rc;
        return found > cap ? RF_ERR_OVERFLOW : RF_OK;
#else
        return RF_ERR_NO_CUDA;
#endif
    }
    return run_emulated<FieldFp64>(f, fbx, fby, mx, my, lookup, opt.chunk, out, cap, res);
}

extern "C" int rf256_decompose(const uint64_t *fb_x, const uint64_t *fb_y, uint32_t n_fb,
                               const uint64_t *tx, const uint64_t *ty, uint32_t n_targets,
                               rf_relation *out, uint32_t cap,
                               const rf_options *opt_in, rf_result *res) {
    rf_options opt = opt_in ? *opt_in : rf_options{RF_BACKEND_AUTO, RF_LOOKUP_AUTO, 0};
    int rc = check_common(fb_x, fb_y, n_fb, tx, ty, n_targets, out, cap, &opt, res, RF_DEFAULT_CHUNK_256);
    if (rc != RF_OK) return rc;

    Fp256Ctx f = fp256_ctx_p256();
    /* canonical bound check: every coordinate < p */
    auto below_p = [&](const uint64_t *l) {
        for (int i = 3; i >= 0; i--) {
            if (l[i] < f.p.v[i]) return true;
            if (l[i] > f.p.v[i]) return false;
        }
        return false;
    };
    for (uint32_t i = 0; i < n_fb; i++) if (!below_p(fb_x + 4 * i) || !below_p(fb_y + 4 * i)) return RF_ERR_BAD_ARG;
    for (uint32_t k = 0; k < n_targets; k++) if (!below_p(tx + 4 * k) || !below_p(ty + 4 * k)) return RF_ERR_BAD_ARG;

    std::vector<U256x64> fbx = rf::to_mont256(f, fb_x, n_fb), fby = rf::to_mont256(f, fb_y, n_fb);
    std::vector<U256x64> mx = rf::to_mont256(f, tx, n_targets), my = rf::to_mont256(f, ty, n_targets);
    int lookup = rf::resolve_lookup(opt.lookup, n_fb);
    int backend = resolve_backend(opt.backend);

    if (backend == RF_BACKEND_CUDA) {
#if RF_HAVE_CUDA
        uint32_t found = 0;
        double secs = 0;
        rc = rf_cuda_decompose256(&f, fbx.data(), fby.data(), n_fb, mx.data(), my.data(), n_targets,
                                  lookup, opt.chunk, out, cap, &found, &secs);
        res->n_found = found;
        res->n_written = found < cap ? found : cap;
        res->backend_used = RF_BACKEND_CUDA;
        res->lookup_used = lookup;
        res->chunk_used = opt.chunk;
        res->seconds = secs;
        if (rc != RF_OK) return rc;
        return found > cap ? RF_ERR_OVERFLOW : RF_OK;
#else
        return RF_ERR_NO_CUDA;
#endif
    }
    return run_emulated<FieldP256>(f, fbx, fby, mx, my, lookup, opt.chunk, out, cap, res);
}

/* ---- persistent Fp64 session ---------------------------------------- */

struct rf64_session {
    Fp64Ctx f;
    int backend, lookup;
    uint32_t chunk, max_targets, cap;
    std::vector<uint64_t> fbx, fby;           /* Montgomery form */
    std::vector<uint64_t> px;                 /* probe set, Montgomery form; empty = factor base */
    std::vector<uint32_t> pairb;              /* pairs mode: probe index -> first pair's build index */
    rf::SortedTableHost sorted;               /* emulation backend's table */
    rf::HashTableHost hash;
    std::vector<uint64_t> mx, my;             /* per-batch scratch */
    void *cuda;                               /* RfCudaSession64 */
};

extern "C" int rf64_session_create(uint64_t p, const uint64_t *fb_x, const uint64_t *fb_y, uint32_t n_fb,
                                   const uint64_t *probe_x, uint32_t n_probe,
                                   uint32_t max_targets, uint32_t cap, const rf_options *opt_in,
                                   rf64_session **out, rf_result *info) {
    rf_options opt = opt_in ? *opt_in : rf_options{RF_BACKEND_AUTO, RF_LOOKUP_AUTO, 0};
    if (!fb_x || !fb_y || !out || n_fb == 0 || max_targets == 0) return RF_ERR_BAD_ARG;
    if ((p & 1) == 0 || p <= 3 || p >= (1ull << 62)) return RF_ERR_BAD_ARG;
    if (opt.chunk == 0) opt.chunk = RF_DEFAULT_CHUNK_64;
    if (!rf::chunk_ok(opt.chunk)) return RF_ERR_BAD_CHUNK;
    if (opt.lookup < RF_LOOKUP_AUTO || opt.lookup > RF_LOOKUP_HASH_DEVICE) return RF_ERR_BAD_ARG;
    if (opt.backend != RF_BACKEND_AUTO && opt.backend != RF_BACKEND_EMULATE && opt.backend != RF_BACKEND_CUDA) return RF_ERR_BAD_ARG;
    for (uint32_t i = 0; i < n_fb; i++) if (fb_x[i] >= p || fb_y[i] >= p) return RF_ERR_BAD_ARG;
    if (probe_x && n_probe == 0) return RF_ERR_BAD_ARG;
    if (probe_x) for (uint32_t i = 0; i < n_probe; i++) if (probe_x[i] >= p) return RF_ERR_BAD_ARG;
    rf64_session *s = new rf64_session();
    s->f = fp64_ctx_new(p);
    s->backend = resolve_backend(opt.backend);
    s->fbx = rf::to_mont64(s->f, fb_x, n_fb);
    s->fby = rf::to_mont64(s->f, fb_y, n_fb);
    if (probe_x) s->px = rf::to_mont64(s->f, probe_x, n_probe);
    const uint32_t n_keys = probe_x ? n_probe : n_fb;
    const uint64_t *keys = probe_x ? s->px.data() : s->fbx.data();
    s->lookup = rf::resolve_lookup(opt.lookup, n_keys);
    s->chunk = opt.chunk;
    s->max_targets = max_targets;
    s->cap = cap ? cap : 1;
    s->cuda = nullptr;
    int rc = RF_OK;
    if (s->backend == RF_BACKEND_CUDA) {
#if RF_HAVE_CUDA
        rc = rf_cuda_session64_create(&s->f, s->fbx.data(), s->fby.data(), n_fb,
                                      probe_x ? s->px.data() : nullptr, probe_x ? n_probe : 0,
                                      s->lookup, s->chunk, max_targets, s->cap, &s->cuda);
#else
        rc = RF_ERR_NO_CUDA;
#endif
    } else if (s->lookup == RF_LOOKUP_HASH || s->lookup == RF_LOOKUP_HASH_DEVICE) {
        s->hash = rf::build_hash<FieldFp64>(s->f, keys, n_keys);
    } else {
        s->sorted = rf::build_sorted<FieldFp64>(s->f, keys, n_keys);
    }
    if (rc != RF_OK) {
        delete s;
        return rc;
    }
    if (info) {
        std::memset(info, 0, sizeof(*info));
        info->backend_used = s->backend;
        info->lookup_used = s->lookup;
        info->chunk_used = s->chunk;
    }
    *out = s;
    return RF_OK;
}

extern "C" int rf64_session_decompose(rf64_session *s, const uint64_t *tx, const uint64_t *ty,
                                      uint32_t n_targets, rf_relation *out, uint32_t cap, rf_result *res) {
    if (!s || !tx || !ty || !res || n_targets == 0 || n_targets > s->max_targets) return RF_ERR_BAD_ARG;
    if (cap > 0 && !out) return RF_ERR_BAD_ARG;
    std::memset(res, 0, sizeof(*res));
    const uint64_t p = s->f.p;
    for (uint32_t k = 0; k < n_targets; k++) if (tx[k] >= p || ty[k] >= p) return RF_ERR_BAD_ARG;
    s->mx.resize(n_targets);
    s->my.resize(n_targets);
    for (uint32_t k = 0; k < n_targets; k++) {
        s->mx[k] = fp64_to_mont(s->f, tx[k]);
        s->my[k] = fp64_to_mont(s->f, ty[k]);
    }
    res->backend_used = s->backend;
    res->lookup_used = s->lookup;
    res->chunk_used = s->chunk;
    uint32_t found = 0;
    if (s->backend == RF_BACKEND_CUDA) {
#if RF_HAVE_CUDA
        /* the device buffer holds the session's cap; copy what fits the caller's */
        std::vector<rf_relation> tmp(s->cap);
        double secs = 0;
        int rc = rf_cuda_session64_run(s->cuda, s->mx.data(), s->my.data(), n_targets, tmp.data(), &found, &secs);
        if (rc != RF_OK) return rc;
        res->seconds = secs;
        uint32_t have = found < s->cap ? found : s->cap;
        uint32_t nw = have < cap ? have : cap;
        if (nw) std::memcpy(out, tmp.data(), nw * sizeof(rf_relation));
        res->n_found = found;
        res->n_written = nw;
        if (!s->pairb.empty())
            for (uint32_t k = 0; k < nw; k++) if (out[k].tau != 0) out[k].j = s->pairb[out[k].j];
        return found > nw ? RF_ERR_OVERFLOW : RF_OK;
#else
        return RF_ERR_NO_CUDA;
#endif
    }
    DecompArgs<FieldFp64> a;
    a.f = s->f;
    a.fb_x = s->fbx.data(); a.fb_y = s->fby.data(); a.n_fb = (uint32_t)s->fbx.size();
    a.probe_x = s->px.empty() ? s->fbx.data() : s->px.data(); a.probe_mode = s->px.empty() ? 0u : 1u;
    a.t_x = s->mx.data(); a.t_y = s->my.data(); a.n_targets = n_targets;
    a.out = out; a.out_cap = cap;
    a.out_count = &found;
    double t0 = rf::now_seconds();
    if (s->lookup != RF_LOOKUP_SORTED) rf::emulate_chunk<FieldFp64, HashTable>(a, s->hash.view(), s->chunk);
    else rf::emulate_chunk<FieldFp64, SortedTable>(a, s->sorted.view(), s->chunk);
    res->seconds = rf::now_seconds() - t0;
    res->n_found = found;
    res->n_written = found < cap ? found : cap;
    if (!s->pairb.empty())
        for (uint32_t k = 0; k < res->n_written; k++) if (out[k].tau != 0) out[k].j = s->pairb[out[k].j];
    return found > cap ? RF_ERR_OVERFLOW : RF_OK;
}

extern "C" int rf64_session_create_pairs(uint64_t p, uint64_t curve_a, const uint64_t *fb_x, const uint64_t *fb_y,
                                         uint32_t n_fb, uint32_t max_targets, uint32_t cap, const rf_options *opt_in,
                                         rf64_session **out, rf_result *info, uint32_t *n_entries) {
    rf_options opt = opt_in ? *opt_in : rf_options{RF_BACKEND_AUTO, RF_LOOKUP_AUTO, 0};
    if (!fb_x || !fb_y || !out || !n_entries || n_fb == 0 || max_targets == 0) return RF_ERR_BAD_ARG;
    if ((p & 1) == 0 || p <= 3 || p >= (1ull << 62) || curve_a >= p) return RF_ERR_BAD_ARG;
    /* build indices must fit u32: n (n + 1) < 2^32 */
    if ((uint64_t)n_fb * ((uint64_t)n_fb + 1) >= (1ull << 32)) return RF_ERR_BAD_ARG;
    if (opt.chunk == 0) opt.chunk = RF_DEFAULT_CHUNK_64;
    if (!rf::chunk_ok(opt.chunk)) return RF_ERR_BAD_CHUNK;
    if (opt.backend != RF_BACKEND_AUTO && opt.backend != RF_BACKEND_EMULATE && opt.backend != RF_BACKEND_CUDA) return RF_ERR_BAD_ARG;
    for (uint32_t i = 0; i < n_fb; i++) if (fb_x[i] >= p || fb_y[i] >= p || fb_y[i] == 0) return RF_ERR_BAD_ARG;
    rf64_session *s = new rf64_session();
    s->f = fp64_ctx_new(p);
    s->backend = resolve_backend(opt.backend);
    s->fbx = rf::to_mont64(s->f, fb_x, n_fb);
    s->fby = rf::to_mont64(s->f, fb_y, n_fb);
    const uint64_t a_m = fp64_to_mont(s->f, curve_a);
    s->chunk = opt.chunk;
    s->max_targets = max_targets;
    s->cap = cap ? cap : 1;
    s->cuda = nullptr;
    int rc = RF_OK;
    if (s->backend == RF_BACKEND_CUDA) {
#if RF_HAVE_CUDA
        uint32_t *pb = nullptr;
        rc = rf_cuda_session64_create_pairs(&s->f, a_m, s->fbx.data(), s->fby.data(), n_fb, s->chunk,
                                            max_targets, s->cap, &s->cuda, n_entries, &pb);
        if (rc == RF_OK) {
            /* the device translates hits to build indices itself (pb is null) */
            if (pb) s->pairb.assign(pb, pb + *n_entries);
            /* the probe set itself stays on the device; px only marks probe mode here */
            s->px.assign(1, 0);
        }
        std::free(pb);
        s->lookup = RF_LOOKUP_HASH_DEVICE;
#else
        rc = RF_ERR_NO_CUDA;
#endif
    } else {
        /* host build, the same keys in the same order; first pair per abscissa */
        const uint32_t n = n_fb;
        std::unordered_map<uint64_t, uint32_t> first;
        first.reserve((size_t)n * (n + 1));
        /* the device's batched runs (rf_pair_run), then the first pair per key in build order */
        std::vector<uint64_t> keys((size_t)n * (n + 1));
        for (uint32_t ai = 0; ai < n; ai++)
            for (uint32_t b0 = ai; b0 < n; b0 += RF_PAIR_CHUNK)
                rf_pair_run(s->f, a_m, s->fbx.data(), s->fby.data(), n, ai, b0, keys.data());
        for (uint32_t b = 0; b < (uint32_t)keys.size(); b++)
            if (keys[b] != RF_PAIR_EMPTY) first.emplace(keys[b], b);
        s->px.reserve(first.size());
        s->pairb.reserve(first.size());
        for (const auto &kv : first) { s->px.push_back(kv.first); s->pairb.push_back(kv.second); }
        *n_entries = (uint32_t)s->px.size();
        s->lookup = RF_LOOKUP_HASH;
        s->hash = rf::build_hash<FieldFp64>(s->f, s->px.data(), (uint32_t)s->px.size());
    }
    if (rc != RF_OK) {
        delete s;
        return rc;
    }
    if (info) {
        std::memset(info, 0, sizeof(*info));
        info->backend_used = s->backend;
        info->lookup_used = RF_LOOKUP_PAIRS_DEVICE;
        info->chunk_used = s->chunk;
    }
    *out = s;
    return RF_OK;
}

extern "C" void rf64_session_destroy(rf64_session *s) {
    if (!s) return;
#if RF_HAVE_CUDA
    if (s->cuda) rf_cuda_session64_destroy(s->cuda);
#endif
    delete s;
}

extern "C" unsigned rf_emu_threads(void) { return rf::EmuPool::get().size(); }

extern "C" int rf_warmup(void) {
#if RF_HAVE_CUDA
    if (rf_cuda_available()) return rf_cuda_warmup();
#endif
    return 0;
}

extern "C" int rf_cuda_available(void) {
#if RF_HAVE_CUDA
    return rf_cuda_device_present(nullptr, 0);
#else
    return 0;
#endif
}

extern "C" const char *rf_backend_info(void) {
    static std::string info;
#if RF_HAVE_CUDA
    char name[256] = {0};
    if (rf_cuda_device_present(name, (int)sizeof(name))) {
        info = std::string("cuda: ") + name + " (+ host emulation)";
    } else {
        info = "cuda backend linked, no device present; host emulation only";
    }
#else
    info = "host emulation only (built without RF_HAVE_CUDA)";
#endif
    return info.c_str();
}

extern "C" const char *rf_strerror(int status) {
    switch (status) {
        case RF_OK: return "ok";
        case RF_ERR_OVERFLOW: return "output buffer overflow (n_found > cap)";
        case RF_ERR_BAD_ARG: return "bad argument (null pointer, empty input, coordinate >= p, or p out of range)";
        case RF_ERR_NO_CUDA: return "CUDA backend not available";
        case RF_ERR_CUDA: return "CUDA runtime error";
        case RF_ERR_BAD_CHUNK: return "chunk must be 16, 32 or 64";
        case RF_ERR_NOMEM: return "out of memory";
        default: return "unknown status";
    }
}

int rf_pairs_bench(uint64_t p, uint32_t n, uint64_t seed, double *t_cell, double *t_run, uint64_t *mismatch) {
#if RF_HAVE_CUDA
    return rf_cuda_pairs_bench(p, n, seed, t_cell, t_run, mismatch);
#else
    (void)p; (void)n; (void)seed; (void)t_cell; (void)t_run; (void)mismatch;
    return -1;
#endif
}
