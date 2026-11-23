/* rootfind_host.hpp -- host-side glue shared by the emulation backend,
 * the CUDA backend and the test harness: lookup-table construction,
 * canonical <-> Montgomery conversion, and the emulation driver that runs
 * rf_decompose_thread for every thread index on the CPU.
 *
 * The emulation is the *same* function the kernel runs (rootfind.cuh is
 * compiled by g++/clang++ here and by nvcc in rootfind_cuda.cu); only
 * the launch loop differs.  This mirrors the convention of gpu/ecc:
 * header-only `__host__ __device__` arithmetic, verified on the host.
 */
#ifndef GPU_ROOTFIND_HOST_HPP
#define GPU_ROOTFIND_HOST_HPP

#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <atomic>
#include <chrono>
#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>
#include <vector>

#include "rootfind.cuh"

namespace rf {

/* --- lookup tables -------------------------------------------------- */

struct SortedTableHost {
    std::vector<uint64_t> keys;
    std::vector<uint32_t> idx;
    std::vector<uint64_t> eyt;       /* keys in Eytzinger (BFS) order, 1-indexed */
    std::vector<uint32_t> eyt_rank;  /* sorted position of eyt[k] */
    SortedTable view() const {
        SortedTable t;
        t.keys = keys.data(); t.idx = idx.data(); t.n = (uint32_t)keys.size();
        t.eyt = eyt.data(); t.eyt_rank = eyt_rank.data();
        return t;
    }
};

struct HashTableHost {
    std::vector<uint64_t> keys;
    std::vector<uint32_t> vals;
    std::vector<uint64_t> slots;     /* (tag << 32) | index; low word RF_NO_INDEX if empty */
    uint32_t mask = 0;
    uint32_t capacity() const { return mask + 1; }
    HashTable view() const {
        HashTable t;
        t.keys = keys.data(); t.vals = vals.data(); t.mask = mask; t.slots = slots.data();
        return t;
    }
};

/* In-order walk of the implicit tree k -> (2k, 2k+1) assigns the sorted
 * keys to Eytzinger positions 1..n. */
inline void eytzinger_fill(const std::vector<uint64_t> &sorted, std::vector<uint64_t> &eyt,
                           std::vector<uint32_t> &rank) {
    const uint32_t n = (uint32_t)sorted.size();
    eyt.assign(n + 1, 0);
    rank.assign(n + 1, 0);
    uint32_t next = 0;
    std::vector<uint32_t> stack;
    uint32_t k = 1;
    while (k <= n || !stack.empty()) {
        while (k <= n) { stack.push_back(k); k = 2 * k; }
        k = stack.back(); stack.pop_back();
        eyt[k] = sorted[next]; rank[k] = next; next++;
        k = 2 * k + 1;
    }
}

template <class F>
SortedTableHost build_sorted(const typename F::ctx &f, const typename F::elt *fb_x, uint32_t n) {
    std::vector<std::pair<uint64_t, uint32_t>> e(n);
    for (uint32_t i = 0; i < n; i++) e[i] = std::make_pair(F::key(f, fb_x[i]), i);
    std::sort(e.begin(), e.end());
    SortedTableHost t;
    t.keys.resize(n);
    t.idx.resize(n);
    for (uint32_t i = 0; i < n; i++) { t.keys[i] = e[i].first; t.idx[i] = e[i].second; }
    eytzinger_fill(t.keys, t.eyt, t.eyt_rank);
    return t;
}

inline uint32_t hash_capacity_for(uint32_t n) {
    uint32_t cap = 16;
    while (cap < 2 * n) cap <<= 1;   /* load factor <= 1/2 */
    return cap;
}

template <class F>
HashTableHost build_hash(const typename F::ctx &f, const typename F::elt *fb_x, uint32_t n, uint32_t cap = 0) {
    if (cap == 0) cap = hash_capacity_for(n);
    HashTableHost t;
    t.mask = cap - 1;
    t.keys.assign(cap, 0);
    t.vals.assign(cap, RF_NO_INDEX);
    for (uint32_t i = 0; i < n; i++) {
        uint64_t key = F::key(f, fb_x[i]);
        uint32_t slot = rf_hash64(key, t.mask);
        while (t.vals[slot] != RF_NO_INDEX) slot = (slot + 1) & t.mask;
        t.keys[slot] = key;
        t.vals[slot] = i;
    }
    t.slots.resize(cap);
    for (uint32_t k = 0; k < cap; k++)
        t.slots[k] = ((uint64_t)HashTable::tag(t.keys[k]) << 32) | (uint64_t)t.vals[k];
    return t;
}

/* --- conversions ---------------------------------------------------- */

inline std::vector<uint64_t> to_mont64(const Fp64Ctx &f, const uint64_t *v, uint32_t n) {
    std::vector<uint64_t> out(n);
    for (uint32_t i = 0; i < n; i++) out[i] = fp64_to_mont(f, v[i]);
    return out;
}

inline std::vector<U256x64> to_mont256(const Fp256Ctx &f, const uint64_t *limbs, uint32_t n) {
    std::vector<U256x64> out(n);
    for (uint32_t i = 0; i < n; i++) {
        U256x64 c;
        for (int l = 0; l < 4; l++) c.v[l] = limbs[4 * i + l];
        out[i] = fp256_to_mont(f, c);
    }
    return out;
}

/* --- emulation driver ----------------------------------------------- */

/* CPUs this process may use: RF_EMU_THREADS if set; else hardware_concurrency,
 * capped by a cgroup CPU quota (cgroup v2 cpu.max, or v1 cfs quota/period).  A
 * container commonly sees every host thread in nproc but is allowed far fewer;
 * running a thread per host thread then oversubscribes the quota. */
inline unsigned rf_cpu_budget() {
    if (const char *e = std::getenv("RF_EMU_THREADS")) {
        long v = std::strtol(e, nullptr, 10);
        if (v > 0) return (unsigned)v;
    }
    unsigned n = std::max(1u, std::thread::hardware_concurrency());
    double quota = -1, period = -1;
    if (FILE *f = std::fopen("/sys/fs/cgroup/cpu.max", "r")) {
        char q[64] = {0};
        if (std::fscanf(f, "%63s %lf", q, &period) == 2 && std::strcmp(q, "max") != 0) quota = std::atof(q);
        std::fclose(f);
    } else if (FILE *fq = std::fopen("/sys/fs/cgroup/cpu/cpu.cfs_quota_us", "r")) {
        if (std::fscanf(fq, "%lf", &quota) != 1) quota = -1;
        std::fclose(fq);
        if (FILE *fp = std::fopen("/sys/fs/cgroup/cpu/cpu.cfs_period_us", "r")) {
            if (std::fscanf(fp, "%lf", &period) != 1) period = -1;
            std::fclose(fp);
        }
    }
    if (quota > 0 && period > 0) {
        unsigned cap = (unsigned)std::max(1.0, std::ceil(quota / period));
        n = std::min(n, cap);
    }
    return n;
}

/* A persistent worker pool for the emulation backend (the multi-threaded CPU
 * sweep, `backend=emulate`).  The threads are created once per process and wait
 * between batches; a batch hands out grains of thread indices through an atomic
 * cursor, and the calling thread works too.  One batch at a time. */
class EmuPool {
public:
    static EmuPool &get() {
        static EmuPool pool(rf_cpu_budget());
        return pool;
    }
    unsigned size() const { return (unsigned)workers_.size() + 1; }
    /* fn(start, end) over [0, total) in grains */
    template <class Fn>
    void run(uint64_t total, uint64_t grain, const Fn &fn) {
        std::lock_guard<std::mutex> one(run_mu_);
        auto body = [&]() {
            for (;;) {
                uint64_t start = next_.fetch_add(grain);
                if (start >= total) break;
                fn(start, std::min(total, start + grain));
            }
        };
        {
            std::lock_guard<std::mutex> lk(mu_);
            job_ = body;
            next_.store(0);
            busy_ = (unsigned)workers_.size();
            gen_++;
        }
        cv_.notify_all();
        body();
        std::unique_lock<std::mutex> lk(mu_);
        done_cv_.wait(lk, [&] { return busy_ == 0; });
        job_ = nullptr;
    }
    ~EmuPool() {
        {
            std::lock_guard<std::mutex> lk(mu_);
            stop_ = true;
            gen_++;
        }
        cv_.notify_all();
        for (auto &t : workers_) t.join();
    }

private:
    explicit EmuPool(unsigned n) {
        for (unsigned i = 1; i < n; i++) workers_.emplace_back([this] { loop(); });
    }
    void loop() {
        uint64_t seen = 0;
        for (;;) {
            std::function<void()> job;
            {
                std::unique_lock<std::mutex> lk(mu_);
                cv_.wait(lk, [&] { return gen_ != seen; });
                seen = gen_;
                if (stop_) return;
                job = job_;
            }
            if (job) job();
            {
                std::lock_guard<std::mutex> lk(mu_);
                if (--busy_ == 0) done_cv_.notify_one();
            }
        }
    }
    std::vector<std::thread> workers_;
    std::mutex mu_, run_mu_;
    std::condition_variable cv_, done_cv_;
    std::function<void()> job_;
    std::atomic<uint64_t> next_{0};
    uint64_t gen_ = 0;
    unsigned busy_ = 0;
    bool stop_ = false;
};

/* Run every thread index on the host, on the persistent pool for large batches.
 * Each tid's work is independent; the output order is irrelevant (callers
 * compare sets or sort). */
template <class F, class T, int CHUNK>
void emulate(const DecompArgs<F> &a, const T &table, unsigned hw_threads = 0) {
    const uint64_t total = rf_thread_count(a.n_targets, a.n_fb, CHUNK);
    if (hw_threads != 1 && total >= 4096) {
        EmuPool::get().run(total, 256, [&](uint64_t start, uint64_t end) {
            for (uint64_t tid = start; tid < end; tid++) rf_decompose_thread<F, T, CHUNK>(a, table, tid);
        });
    } else {
        for (uint64_t tid = 0; tid < total; tid++) rf_decompose_thread<F, T, CHUNK>(a, table, tid);
    }
}

template <class F, class T>
void emulate_chunk(const DecompArgs<F> &a, const T &table, uint32_t chunk) {
    switch (chunk) {
        case 16: emulate<F, T, 16>(a, table); break;
        case 32: emulate<F, T, 32>(a, table); break;
        case 64: emulate<F, T, 64>(a, table); break;
        default: break;   /* validated by the caller */
    }
}

inline bool chunk_ok(uint32_t c) { return c == 16 || c == 32 || c == 64; }

/* Choose the lookup structure when the caller says AUTO: hash whenever
 * it would fit the device's shared-memory budget (the same rule the CUDA
 * backend applies, so emulation and device agree on the table used). */
inline int resolve_lookup(int requested, uint32_t n_fb) {
    if (requested != RF_LOOKUP_AUTO) return requested;
    return hash_capacity_for(n_fb) <= RF_SHARED_HASH_CAP ? RF_LOOKUP_HASH : RF_LOOKUP_SORTED;
}

inline double now_seconds() {
    using namespace std::chrono;
    return duration<double>(steady_clock::now().time_since_epoch()).count();
}

} /* namespace rf */

#endif /* GPU_ROOTFIND_HOST_HPP */
