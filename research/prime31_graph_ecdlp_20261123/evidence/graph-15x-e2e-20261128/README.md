# M31 graph control: end-to-end wall against PR #5319 (2026-11-28)

**Goal (pinned in `docs/ACTIVE.md`).** Cut the end-to-end time of the
M31 graph-control cold solve to at most 1/15 of PR #5319's default
(`1234567`, single thread), with the same normalised output.

**Result.** Over 11 interleaved rounds, the median process wall went from
801 ms to 41 ms at 4 threads, a 19.5× reduction. The paired per-round ratio
has a median of 20.4× and a range of 12.7× to 24.1×. Ten of the 11 rounds
are at or above 16.6×.

The 4-thread figure includes the multi-core share. CPU time (user + sys)
went from 0.753 s to 0.111 s, a paired median of 6.7×.

Single-threaded, the median process wall is 88 ms: 9.1× by both wall and
CPU time. Peak RSS fell from 117 MB to 16–17 MB. [pcc:unverified]

## Change

`../../native/mitm_four_fast.rs` gets lever 8 (`couple`), which needs
levers 1, 2, 4 and 6. A third CLI argument sets the thread count, which
defaults to 1. The default lever set is still `1234567`, and the measured
invocation is `prime31-graph-mitm-fast 40 12345678 <threads>`.

- **Couple table.** Each fixture pair (i, j) gives the four signed sums
  ±P_i ± P_j. These are two couples, each a sum S and its negation −S
  sharing one x. The table stores one (x, y) per couple, from two additions
  over one shared denominator z − x. It is one batch inversion per build
  thread, with rows split by balanced ranges.
- **x-only probes.** For each couple L and query R, the complements
  x(R − L) and x(R + L) share one inverse of x_L − x_R. y is computed only
  on an index hit. Each probe is still charged one group addition.
- **Batch inversion and threads.** The batch inversion runs in fixed
  4096-couple chunks, with eight chains per AVX2 block and runtime
  detection. Threads pull chunks from a shared counter.

  The chunk boundaries are fixed, so the probe-stage counts do not depend
  on the thread count. The table build's counts still do: each extra build
  thread adds one inversion and 61 multiplications.
- **Arithmetic.** Lazy Mersenne arithmetic in AVX2. The conditional
  subtraction is `min_epu32(t, t − p)`, and x(R ± L) = s² − (x_R + x_L)
  with canonical operands throughout.
- **x-index.** A compressed-bucket index, keyed by the top 16 bits of a
  32-bit multiplicative hash, with couples ascending within each bucket.
  - **Prefilter:** a 1 MB blocked Bloom filter, with three bits in one
    64-bit word, checked four lanes at a time with `vpgatherqq`.
  - **Build:** the table-build threads produce per-bucket histograms, each
    thread scatters its own couple range, and thread-owned bucket ranges set
    the filter bits and count distinct points.
  - **Distinct count:** `y` is read only where several couples share an x.
- **Graph rank.** Delayed-reduction elimination over u32 pivots. The axpy
  skips spans where a stored pivot is known to be zero: an FB pivot at
  column c is zero on [0, c) and on every LP column, and an LP pivot is
  zero on the LP columns before it.
- **Overlap.** With two or more threads, worker 0 runs query k's sort,
  replay and rank before it starts on query k + 1's chunks. Query k + 1 is
  speculative: if query k recovers the scalar, k + 1's setup and probe
  counts are dropped. Every output therefore equals the sequential loop's.
- **Exit.** With lever 8, the process flushes stdout and exits without
  running the table destructors.

## Exactness

Ten runs give one normalised SHA-256 (`EXACTNESS.txt`, `a3f1e8a9…`), with
scalar `987654321`, 915116 distinct pair sums and 10068344 probe additions:

- the reference;
- the new binary at `1234567` and `1234`;
- `12345678` at 1, 2, 3, 4 and 8 threads;
- the minimal lever-8 set `124568` at 1 and 4 threads.

All 44 timed runs in `timing-e2e.jsonl` also carry that hash. The
relations come from the same probe set, so rows, per-query classes and
ranks are unchanged, not just the scalar.

## Cost

`timing-e2e.jsonl`, made with `../../native/bench_e2e.sh`, 11 rounds. Each
run is a separate cold process, and arm order is fixed within each round.

Process wall is measured externally with `date` in nanoseconds. It
includes fork and exec, which cost about 3 ms on this host even for
`/bin/true`. CPU time comes from the bash `time` keyword.

| arm | median process wall | paired ratio, median (min–max) | median user + sys | median cold wall | peak RSS |
|---|---:|---:|---:|---:|---:|
| #5319 `1234567`, 1 thread | 801 ms | 1 | 0.753 s | 785 ms | 117.0–117.1 MB¹ |
| `12345678`, 4 threads | 41 ms | 20.4× (12.7–24.1) | 0.111 s | 35 ms | 17.3 MB² |
| `12345678`, 2 threads | 61 ms | 12.8× (9.7–17.0) | 0.097 s | 55 ms | 16.7 MB² |
| `12345678`, 1 thread | 88 ms | 9.2× (6.9–11.7) | 0.080 s | 82 ms | 16.4 MB² |

¹ VmHWM polled every 5 ms, as in #5319 (`peak-rss-ref.jsonl`).
² VmHWM read in-process just before output (`vm_hwm_kb`).

The paired CPU-time ratio has a median of 6.7× at 4 threads (range
6.0–7.4) and 9.1× at 1 thread (range 8.4–9.6).

**Stage ledger** (`stage_us`, medians, µs):

| stage | 1 thread | 4 threads |
|---|---:|---:|
| fixture | 1338 | 1313 |
| couple table (c1) | 6954 | 3367 |
| x-index (c2) | 11731 | 5861 |
| collection + graph | 50086 + 7554 | 21494 (overlapped) |
| query setup, records | 504 | 582 |

**Steps** (single cold runs at 4 threads unless noted; noisy host, for
orientation only):

| step | effect |
|---|---|
| lever 8 baseline (couples, x-only AVX2, flat index, bitmap) | ~70–87 ms cold |
| blocked Bloom prefilter | collection 33–40 → 22–26 ms |
| zero-span skipping in the rank | graph 15 → 7.5 ms |
| chunked dynamic collection, graph overlapped with the next query | f + g 36 → ~22 ms |
| compressed-bucket index from the build's histograms | x-index 8–11 → 5–6 ms |
| `min_epu32` arithmetic, shared s² − (x_R + x_L) | 1-thread collection 66 → 52 ms |

A first overlap attempt, with a fifth thread for the graph stage,
oversubscribed the 4 vCPUs and was slower. It was replaced by worker 0
plus dynamic chunks.

## Caveats

- **Thread count.** The 15× bar is met at 4 threads, not at 1 or 2
  threads. CPU time fell 6.7× at 4 threads and 9.1× at 1 thread; the rest
  of the wall-time ratio comes from using more cores.
- **Host.** This is one shared 4-vCPU Firecracker container (Xeon,
  2.1 GHz, `BUILD_SHA256SUMS`). Run-to-run noise is large, with one round
  at 12.7×. First-touch page faults cost about 1.7 µs each here.
- **Build.** The build used the `577d6c2c` export, as for #5311 and
  #5319, because `main`'s harness pin check fails.
- **AVX2.** The AVX2 kernel is selected at runtime. The scalar fallback is
  exact, but it was not timed.
- **Scope.** The cost is a single-target cold solve at the M31 fixture
  only. No independent replay or verifier has checked these numbers yet.

## Reproduce

```sh
# in research/prime31_graph_ecdlp_20261123/native of the 577d6c2c export,
# with this PR's mitm_four_fast.rs copied in:
cargo build --locked --release --bin prime31-graph-mitm-fast
./bench_e2e.sh 11 out.jsonl \
  ref5319=REF_BIN:40 \
  cand_t4=target/release/prime31-graph-mitm-fast:"40 12345678 4" \
  cand_t2=target/release/prime31-graph-mitm-fast:"40 12345678 2" \
  cand_t1=target/release/prime31-graph-mitm-fast:"40 12345678 1"
```
