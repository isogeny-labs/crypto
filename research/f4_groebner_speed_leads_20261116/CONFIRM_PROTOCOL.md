# Confirmation run of the shipped defaults (frozen before data, 2026-11-17 00:19 UTC)

**Why.** The first run (`reports/run-20261116T2306Z/`) measured the integrated
build with f4-memory on. The landed code ships f4-memory off (opt-in
`F4_FP_MEM=1`), so the default against the old paths was only available as a
derived, unpaired ratio (`par_alone_vs_baseline.md`). That run was also driven
by a Python runner, which AGENTS.md "Implementation language: no Python" does
not allow for benchmark drivers. This run measures the shipped default directly,
paired in one binary, with the native driver `examples/f4_ab_confirm.rs`, under a
`bench.record/v1` record.

**Binaries.** `f4_fp_bench` and `koblitz_f4_kernel_bench` built from the landed
source (f4-parallel and gf2-m4ri on, f4-memory opt-in); SHA-256 recorded in the
run directory.

**Arms.** `new`: no F4 toggle set. `old`: `F4_FP_PAR_OLD=1 F4_FP_MEM_OLD=1
KIC_M4RI_OLD=1` (the code paths before the 2026-11-16 leads). A/A: `new` against
`new`.

**Cells.** `f4_fp_bench 3 large` (pairs DEFAULT and AA) and
`koblitz_f4_kernel_bench 5 x` (pairs M4RI and AA), each at RAYON_NUM_THREADS 1
and 4. 12 blocks; every block visits every cell once in rotated order, ABBA or
BAAB by parity; 24 (old, new) pairs per cell.

**Statistic.** median(old) / median(new) per case (>1: new faster) and a 95%
percentile bootstrap over the (old, new) pairs, 4,000 resamples
(`ecbench::stats::bootstrap_ci`). Significant: the interval excludes 1 and |ln
ratio| exceeds the A/A spread (larger |ln| endpoint of the A/A interval) for the
same case and thread count. Any fingerprint difference across runs of a case
fails the run.

**Primary endpoint.** `SUM_f4_ms` (sum of the 13 cases' f4 times), DEFAULT pair,
4 threads. **Prediction:** ratio in [1.08, 1.20] and significant (the derived
first-run value is 1.136).

**Secondary, predicted before data.**
- DEFAULT `quad_n9_p65521`, 4 threads: [1.08, 1.25].
- DEFAULT `SUM_f4_ms`, 1 thread: [0.97, 1.06], not significant.
- DEFAULT `quart_n3_p31` and `cubic_n4_p31`, 4 threads: at or below 1 (the known
  small-case cost); reported whatever the outcome.
- M4RI `kernel_x`, 1 and 4 threads: [1.30, 1.60], significant.

**Conditions.** Run only after every build and test has finished; `ps` and the
load average are recorded before and after. One 4-vCPU shared cloud VM, isolation
L0, no PMU; all results `[pcc:unverified]`.
