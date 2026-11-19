# k = 5 sum-table index calculus on batched Montgomery / AVX-512 arithmetic

Date: 2026-11-18. All numbers are `[pcc:unverified]`. They come from a
shared host with no isolation and have not been replayed independently.

**What this is.** A wall-time re-implementation of the generic-track
incumbent, `ic.pipeline` with `ksum:m=5` on a `prime-abscissa` base
(`research/prime_ksum_vs_incumbent_20261029`). It runs the same method,
end to end and complete, on the same curve, targets and factor base. The
counted cost S is unchanged: the same table and the same kind of walk.
The matched rho gap and the index-calculus leaderboard therefore do not
move. This is engineering, not an index-calculus improvement, so no
leaderboard or `benchmarks/README.md` row is added. That would also need a
replay certificate and an isolated runner, which this session lacks.

## Why this lever

The `#4708` kernel already ran at about 9 cycles per point on one core,
and its measured components put the CPU floor only about 3x lower. On a
4-core host, then, another 100x could not come from the per-point kernel.
The repository's own ladder pointed elsewhere:
- the k-sum table is the generic-track incumbent (`prime_ksum_table_20261028`,
  `prime_ksum_vs_incumbent_20261029`);
- triple tables were studied in `ic_triple_table_20260923`;
- so a bigger table is known and gives no new exponent.

What had not been done is running that incumbent's own relation phase on
fast arithmetic. The library spends 27 ms building its table at about
119 ns per addition, and 67 ms walking at about 292 ns a step.

## Method (`examples/ksum_fast_ic.rs`, commit in `source-commit.txt`)

1. **Table.** Every signed 5-sum `P_i1 ± … ± P_i5` (`i1 < … < i5`, first
   sign +), keyed by abscissa: 16·C(A, 5) entries.
   - Levels 2–4 are batched affine additions with one inversion per level
     (an AVX-512 kernel: `vpmuludq` Montgomery products, gathers, 32
     interleaved product chains).
   - The last level is abscissa-only and forms both signs from one
     denominator.
   - The table is built in parallel: per-thread keys, then a parallel
     two-level bucket scatter. The Bloom filter's word is chosen by the
     key's top bits, so each thread owns its words. Witnesses are implicit
     in the pair index.
2. **Walk.** 256 lockstep r-adding walks a thread, `R ← R + W_j` with
   `W_j = Q + 2^j G`.
   - Each step is an explicit AVX-512 kernel with one inversion for all
     lanes.
   - Every walk point is probed: an AVX-512 gather of the Bloom filter,
     then an exact bucket lookup.
3. **Relations.** Each hit is re-added in the library's group law and kept
   only if `±(aG + bQ)` equals the 5-sum. Relations are collected up to
   A + 1, as the library does, then a left kernel over F_r gives
   `x = −Σλa / Σλb`. A solve counts only if `x G = Q` in the library's
   arithmetic and `x` equals the logarithm ecbench recovered.
4. **Threads.** A spin-waiting worker pool. A rayon round trip cost 75 µs
   back to back and 146 µs from sleep on this host (`KSUM_PARBENCH`),
   against 2–3 µs for the spin pool.

## Correctness

- **Builder parity.** `KSUM_CHECKBUILD`: the AVX-512 parallel builder's
  (key, witness) multiset equals the scalar builder's exactly at A = 16,
  19, 24 and 30 (69,888 to 2,280,096 entries), with 1, 3 and 4 threads.
- **Solves.** In every configuration, every one of the 96 solves (48
  targets × 2 repetitions) is complete and checked: `x G = Q`, and `x` is
  ecbench's logarithm. Hits rejected by the group law: 0.
- **Library.** In both sessions, 96/96 `ksum5` and 96/96 `rho-neg` runs
  verified.

## Results (48 targets, 2 rounds, same host, run as library A, fast, library B)

The library pipeline runs `ksum5` on one thread:
- session A: 98.0 ms median a solve (27.2 ms table, 67.4 ms relations);
- session B: 96.6 ms; the A/B drift is 1.5 %;
- pooled median: **97.2 ms**.

| Configuration | Median a solve | Build | Walk | vs library `ksum5` [95 % bootstrap] |
|---|---|---|---|---|
| per solve, A = 19 (library's base), 1 thread | 4.74 ms | 2.77 ms | 1.86 ms | **20.5x** [19.3, 21.4] |
| per solve, A = 17 (re-tuned), 1 thread | 4.58 ms | 1.35 ms | 3.13 ms | **21.3x** [19.9, 22.6] |
| per solve, A = 19, 4 threads | 2.56 ms | 1.83 ms | 0.67 ms | **38.0x** [34.8, 41.2] |
| per solve, A = 17, 4 threads | 1.91 ms | 0.84 ms | 0.96 ms | **50.8x** [47.6, 53.4] |
| table once per curve, A = 19, 4 threads | 0.70 ms | (1.7–3.6 ms once) | 0.61 ms | **140x** [131, 146] |
| table once per curve, A = 21, 4 threads | 0.58 ms | (3.8–5.8 ms once) | 0.47 ms | **168x** [159, 177] |

How to read the rows:
- **Like for like.** The per-solve rows charge the table to every solve,
  as the pipeline does. On one thread they are the like-for-like
  comparison: **about 21x**. The 4-thread rows add the host's cores:
  **38x** at the library's base size, and **51x** with the base re-tuned
  for the new cost ratio. A table entry now costs relatively more than a
  walk step, which moves the optimum from A = 19 to A = 16–18.
- **The once-per-curve rows are a different accounting.** Each target
  still runs its own complete walk, relations, linear algebra and check,
  but the table, which depends only on the curve and base, is built once
  for all 48 targets. That fits a multi-target campaign; it is not the
  pipeline's per-solve definition. In that setting the speed-up passes
  **100x**, reaching 140–168x. Here the one-time build is 1.7–5.8 ms,
  about 0.04–0.12 ms a target over 48 targets.
- **Distance to rho.** Counted cost is unchanged: `ksum5` S = 6.95 against
  `rho-neg` S = 0.90, so 7.7x remains. The library `rho-neg` takes 7.7 ms
  a solve; it has not been re-implemented on this arithmetic, so no
  wall-time comparison with rho is made.

## Round 2: a sibling probe per walk step (post hoc, after round 1)

The walk step computes `R + W_j` through `λ = (y_W − y_R)/(x_W − x_R)`.
`R − W_j` uses the same denominator, so one more product (its slope) and
one square (its abscissa) give a second probe point. Its logarithm is
known: `(a − c_j, b − 1)`, with `(a, b)` taken before the step. A hit on
it is re-added from the kept pre-step point and verified like any other.
The same change drops two gathers: `x_W` is rebuilt from the stored
denominator, and `b` advances by the constant `d_j = 1` of the steps
`W_j = Q + 2^j G`. The kernel is otherwise unchanged; `KSUM_NOSIB=1`
switches the sibling off.

Data: `round2/raw/`, 48 targets × 2 rounds per configuration, run as
library A, fast, library B (94.9 and 94.8 ms, pooled 94.8 ms); commit in
`round2/source-commit.txt`. Every solve was checked (96/96 in every
configuration) and no hit was rejected.

| Configuration | Median a solve | Build | Walk | vs library [95 % bootstrap] |
|---|---|---|---|---|
| round-1 kernel (no sibling), A = 17, 256 lanes, 4 threads | 1.91 ms | 0.77 ms | 1.05 ms | 49.7x [48.0, 51.3] |
| sibling, A = 16, 256 lanes, 4 threads | **1.57 ms** | 0.62 ms | 0.87 ms | **60.4x** [58.5, 62.4] |
| sibling, A = 16, 480 lanes, 4 threads | 1.78 ms | 0.72 ms | 1.04 ms | 53.2x [51.0, 54.8] |
| sibling, A = 17, 480 lanes, 4 threads | 1.79 ms | 0.84 ms | 0.85 ms | 53.1x [48.3, 55.0] |
| sibling, A = 16, 480 lanes, 1 thread | 3.69 ms | 1.05 ms | 2.55 ms | **25.7x** [23.9, 26.7] |
| sibling, table once per curve, A = 21, 480 lanes, 4 threads | 0.56 ms | (once) | 0.45 ms | 168x [164, 172] |

How to read the rows:
- **Reproduction.** The round-1 kernel reproduces its round-1 result:
  49.7x here against 50.8x then, on the same host type. The ratios are
  therefore stable to a few per cent between sessions.
- **Best per-solve configuration:** the sibling probe at A = 16 with 256
  lanes on 4 threads, **60.4x** (1.57 ms). On one thread, the like-for-like
  setting, it is **25.7x**, up from 21x.
- **What did not help.** More lanes (480) were slower than 256 in this
  round. A 24-solve sweep had suggested the opposite, but this shared host
  moves by about 25 % between sweeps, so only the 96-solve rounds are used.
  A kernel with 8 vectors (64 chains) was also slower than 4 (9.4 against
  8.1 ns a lane-step).

## Round 3: fixed offsets, parallel lower levels, a sized directory (null on the ratio)

Three changes, all post hoc:
- **Fixed-offset probes** (`KSUM_OFFS`): each walk point `R` is also
  probed as `R ± T_i` through the table builder's pair kernel. A short
  sweep (24 solves) showed no gain with one or two offsets and a loss
  with four (1.57 → 2.17 ms at A = 16), because a pair of offset probes
  costs about as much as a walk step. The option stays in the code,
  default off, and was correct throughout.
- **Parallel lower levels**: levels 2–4 of the build now run on the pool
  (level 4 at A = 16: 0.29 → 0.15 ms).
- **Sized directory**: the table's directory has about 4 keys a bucket
  (10–16 bits) instead of 2^16 buckets (bucket phase: 0.27 → 0.16 ms).

The AVX-512 builder still equals the scalar builder at A = 14, 16, 19, 24
and 30 with 1 and 4 threads. Data: `round3/raw/`, 48 targets × 2 rounds
per configuration, run as library A, fast, library B (86.5 and 86.6 ms).
Every solve was checked (96/96 a configuration) and no hit was rejected.

| Configuration | Median a solve | Build | Walk | vs library [95 % bootstrap] |
|---|---|---|---|---|
| A = 16, 4 threads | 1.46 ms | 0.58 ms | 0.82 ms | 59.3x [57.4, 61.4] |
| A = 17, 4 threads | 1.69 ms | 0.83 ms | 0.78 ms | 51.2x [49.2, 53.1] |
| A = 16, 1 thread | 3.64 ms | 0.96 ms | 2.60 ms | 23.8x [22.5, 24.8] |
| A = 15, 1 thread | 4.36 ms | 0.66 ms | 3.65 ms | 19.9x [18.9, 21.0] |

**Reading.** In absolute terms the best configuration fell from 1.57 to
1.46 ms (−7 %). But the library also ran 9 % faster in this session
(86.6 against 94.8 ms) on the same host type, and the ratios (59.3x
against 60.4x on 4 threads, 23.8x against 25.7x on one) agree within
their intervals. **Round 3 is not a measured improvement in the speed-up
ratio.** The per-solve ratio has plateaued at about 60x on 4 threads and
24–26x on one. What remains is the walk, at about 2.5 ns a probe per core
(sibling included), and the table's memory traffic.

## Where the time goes, and what did not work

- **Walk.** About 7 ns a lane-step on one core with the intrinsics kernel.
  The compiler-vectorized version of the same Rust took 59–81 ns: it
  neither vectorized the gathers nor removed the modular-add branches.
- **Table build.** Bound by memory traffic and per-phase start-up, not by
  arithmetic. At A = 17 on 4 threads: keys 0.28 ms, scatter 0.21 ms,
  buckets 0.34 ms, for 99K entries.
- **Join mode.** `--mode join` replaces the table with two filters and one
  join pass over recomputed keys. It was correct (24/24, 0 rejected) but
  no faster: 2.3 ms at A = 16. Its two key passes are memory-bound and
  scale poorly across this VM's cores.
- **Measurement artefacts fixed before any kept data:**
  - per-step `getenv` and CPU-feature detection in the walk (about 60 ns a
    step);
  - per-solve page faults from fresh multi-MB allocations (malloc kept
    resident: `env.txt`);
  - a rayon wake-up of 75–150 µs per parallel phase.

## Not settled

- **Field size.** Below 32 bits only: Montgomery with R = 2^32 and
  32 × 32-bit lane products. Wider fields change the product cost.
- **One host.** One curve, one host, L0. The ratios need an isolated
  replay.
- **Raw records.** The full ecbench `records.jsonl` (2.3 MB a session) and
  the solver's JSONL could not go to the artifact store (no S3 in this
  session). `raw/` holds CSV extracts of every measured row and the
  sessions' metadata; `raw/RAW_SHA256SUMS` names the originals.

## Files

- `analyze.py` builds `summary.json`, `speedup.svg`, `speedup.png` and
  `report.pdf` from `raw/*.csv`, `round2/raw/*.csv` and `round3/raw/*.csv`
  (`round*/summary.json`, `round*/speedup.*`); `render.js` renders them.
- `raw/fast-*.csv`: one row per solve and configuration.
- `raw/library-lib{A,B}.csv`: one row per ecbench record.
- `raw/lib{A,B}/`: session, spec and host metadata.
- `targets48.txt`: the targets, with ecbench's logarithms.
- `host.txt`, `env.txt`, `binaries.sha256`, `source-commit.txt`.
