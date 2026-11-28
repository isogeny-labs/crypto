# PROTOCOL: faster Wiedemann per counted operation (frozen before any timing) [pcc:unverified]

Date: 2026-11-22. Host: this VM (4 vCPU, no hardware PMU), single thread, wall clock only.
Base tree: HEAD 8ad232bb21 (unmodified solver). All figures [pcc:unverified].

## Question
Does a restructured Wiedemann solver (same counted operations, same outputs) cut the in-situ wall time per
counted `la_mul`, at genus 3, p in {65537, 131101, 262147}? Reference: `research/genus_ladder_kappa_20261122/RESULTS.md`
(in situ 99.6 / 114.9 / 107.1 ns per `la_mul`; 262147 has r > 2^50 so `mulmod` took the `u128 %` path).

## Change under test (written after this protocol is frozen)
* `src/cryptanalysis/genus_ladder.rs`: local `wiedemann` and `wiedemann_coord` with the same signatures, same RNG draws and
  every `Nat` increment unchanged. CSR rows split by entry kind (+1 / -1 / generic, Shoup multipliers for generic), unreduced
  u64 row sums with one Barrett reduction per row, transposed product as a gather over a transposed CSR, diagonal multiply fused,
  u128 dot products in Berlekamp-Massey. `ic_control::wiedemann` stays untouched as the equality oracle in a test.
* `src/cryptanalysis/chained_transfer/ic_control.rs`: only `mulmod` (x86 `div` instead of `u128 %` for r >= 2^50) and `addm`
  (branchless), signatures unchanged. Shared by other modules: if any module's counts or outputs change, this is reverted.

## Count reference (recorded BEFORE the change, from the unmodified build)
`count_reference.json`: 24 runs (genus 3 and 4, p in {1031, 2053, 8209}, arms Full and DoubleLp, with and without SGE(8,3)),
every counted field (rel, la, la_graph, la_peel, la_wied, la_sge, la_sel, trials, relations, cantor_mul/inv, group ops, core
shape and nnz, attempts, fallbacks) plus `recovered`/`certified`. Produced by the unmodified test binary
(sha256 in `frozen.sha256`, copy kept in the scratchpad). The test `wied_speed::wiedemann_speedup_keeps_counts_and_outputs` compares.

## Method (the kappa instrument, `examples/kappa_g3.rs`, unchanged)
* Instances: first `instance` line of `research/genus_ladder_g3_highp_20261121/run_<p>.jsonl`, p = 65537, 131101, 262147;
  arm C_double_lp + prefilter, SGE(8,3), cc 2.0, seed 1.
* t_R in situ: `run_ic` at `max_trials = trials` minus `run_ic` at `max_trials = trials - 1` (stops before LA), divided by `la_mul`.
  Trials interleaved with the t_F/bare micro-benchmarks as in the kappa study; 2 warm-ups, then 15 measured trials, median.
* A/A: split-half (even vs odd trials), |median difference| / median. 1-minute load average read before and after every trial.
* Before: example binary built from the unmodified tree, copied to the scratchpad, sha256 recorded. After: same example rebuilt
  on the changed tree. Both run with NO compile or other heavy job running; the before series is run first, then the after series,
  per p in the order 65537, 131101, 262147. A trial with load average >= 1.5 before it is flagged.
* Re-pricing (W3): R2's SGE(8,3) arm over rho at 262147 recomputed from its recorded counts (`joined.jsonl.gz`) exactly as the
  kappa study re-priced, with the new in-situ kappa = ns per `la_mul` / in-situ t_F (t_F taken from the same series).

## Predictions (before data)
* W1: every count and output is identical on the fixture set and on the three timing instances (la_mul, la_add, la_wied_mul,
  la_sge_mul, rel_mul, cantor_mul, trials, recovered).
* W2: in-situ ns per `la_mul` falls by >= 1.5x at 262147 and by >= 1.2x at 65537.
* W3: R2's SGE(8,3) arm over rho at 262147 falls from 0.568 to <= 0.48.

## Limits (declared in advance)
One VM, wall clock, no PMU, one instance per p, difference of two whole-run wall times, single thread. 15 trials per p.
Failures are reported as failures.
