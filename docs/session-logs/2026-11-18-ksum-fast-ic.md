# 2026-11-18: next 100x, the k-sum relation phase on fast arithmetic (PR #4725)

## 17:00Z: ksum:m=5 index calculus end to end on batched Montgomery / AVX-512

### What was done
- Checked the knowledge base. k-sum tables are already the generic-track
  incumbent (`research/prime_ksum_table_20261028`,
  `research/prime_ksum_vs_incumbent_20261029`; triple tables in
  `research/ic_triple_table_20260923`), so a bigger table is no new lever.
  The new lever is the wall time of that incumbent's relation phase.
- Wrote `examples/ksum_fast_ic.rs`, a complete solver:
  - the 5-sum table, built by an AVX-512 level kernel and a parallel
    bucketed table;
  - lockstep r-adding walks using an explicit AVX-512 step and a gather
    Bloom probe;
  - group-law verification of every relation;
  - left-kernel linear algebra over F_r, with `x G = Q` and ecbench's
    logarithm checked.
  Modes: per solve, `--amortize 1`, `--mode join`.
- Ran the evidence in the order library A, fast, library B: 48 targets ×
  2 rounds, six fast configurations. Wrote
  `research/ksum_fast_ic_20261118/` (RESULTS.md, raw CSV extracts,
  summary.json, speedup.svg/png, report.pdf) and a PLAN.md status line.

### Results (`[pcc:unverified]`, medians, 96 solves a configuration, all checked)
- Library `ksum5`: 97.2 ms a solve (session A 98.0 ms, B 96.6 ms).
- Per solve, A = 19, 1 thread: 20.5x [19.3, 21.4].
- Per solve, A = 17, 4 threads: 50.8x [47.6, 53.4].
- Table once per curve, A = 21, 4 threads: 168x [159, 177].
- AVX-512 builder = scalar builder (multiset) at A = 16, 19, 24 and 30.
- 0 rejected hits.

### Decisions made
- The per-solve numbers are the headline; the once-per-curve rows are
  labelled as a different accounting.
- Re-tuned A for the new cost ratio (A = 17).
- No leaderboard row: S is unchanged.
- Replaced rayon with a spin pool: wake-ups cost 75–150 µs here.

### Open threads
- Wider fields (52/64-bit Montgomery lanes).
- Isolated replay.
- Upload the raw records and file bench records once S3 works.
- Re-implement rho on the same arithmetic, for a fair wall-time rho
  comparison.

### Errors
- First walk version: per-step `getenv` and feature detection (about
  60 ns a step), found by microbenchmark.
- The compiler-vectorized step ran at 59–81 ns against 7 ns with
  intrinsics.
- The container moved to a different host mid-task; all numbers here are
  from the 2.10 GHz host, baseline re-measured.
- Disk ran low; deleted stale test binaries in `target/release/deps`.

## 23:55Z: round 2, a sibling probe per walk step

### What was done
- In `avx::step`, each step now also yields `x(R − W_j)` from the same
  denominator: one more product and one square. Its logarithm
  `(la − 2c_j, lb − 2)` is checked against the kept pre-step point. `x_W`
  is rebuilt from the stored denominator, and the `lb` gather became the
  constant d = 1. Lane starts now come from step triples, allowing up to
  4960 lanes. Dead table code was removed and the build is warning-free.
- Ran round 2: library A, then six fast configurations on 48 targets × 2,
  then library B. Results are in `research/ksum_fast_ic_20261118/round2/`;
  RESULTS.md and report.pdf are regenerated.

### Results (`[pcc:unverified]`, all 96 solves checked in every configuration, 0 rejected)
- Library: 94.9 / 94.8 ms a solve.
- Round-1 kernel reproduced: 49.7x (round 1: 50.8x).
- Sibling, A = 16, 256 lanes, 4 threads: 60.4x [58.5, 62.4], 1.57 ms.
- Sibling, 1 thread: 25.7x.
- Table once per curve: 168x.

### Decisions made
- Kept 256 lanes and 4 vectors: 480 lanes and 8 vectors were slower in
  the 96-solve rounds.
- Rejected 24-solve sweeps as noise: the host moves by about 25 %.

### Open threads
- Extra fixed offsets `R ± T_i` per step: about 3.5 products a probe.
- Build fixed overhead: about 0.6 ms at A = 16.
- Wider fields; isolated replay; S3 upload of the raw records.

### Errors
- The container restarted mid-task; a cold `git status` took several
  minutes.
