# RESULTS: faster Wiedemann per counted operation [pcc:unverified]

Every figure is `[pcc:unverified]`: one VM (4 vCPU, no hardware PMU), single thread, wall clock. Protocol, count reference and
binary hashes were frozen before the solver change (`PROTOCOL.md`, `frozen.sha256`). Tables: `tables.md`. Figure: `fig_wied.svg`.
Raw timings: `raw/` (hashes in `data.sha256`). Driver: `run_all.sh`; analysis: `analyze.py`. Instrument: `examples/kappa_g3.rs`, unchanged.

## What changed
* `src/cryptanalysis/genus_ladder.rs`: local `wiedemann` and `wiedemann_coord` (same signatures; they replace the imports of the same
  names, call sites untouched), with `Lines`/`Sys` (CSR split into +1, -1 and generic entries; Shoup quotients for the generic ones;
  unreduced u64 row sums with one Barrett reduction per row; `A^T y` as a gather over a transposed CSR; the diagonal `D` fused into
  `A x`), a u128 dot-product Berlekamp-Massey and Horner/scale loops with Shoup multipliers. Every `Nat` increment and every RNG
  draw is the same as `ic_control`'s. `ic_control::wiedemann` is unchanged and serves as the oracle.
* `src/cryptanalysis/chained_transfer/ic_control.rs`: only `mulmod` (r >= 2^50 with reduced operands: one x86 `div`, `mulmod_wide`,
  instead of `u128 %`; r < 2^50 keeps the f64 path) and `addm` (branchless `min`), signatures unchanged.
* Tests added: `genus_ladder::wied_speed::wiedemann_speedup_keeps_counts_and_outputs` (24 runs vs `count_reference.json`),
  `genus_ladder::fast_wiedemann_oracle::fast_wiedemann_equals_reference` (local vs `ic_control` solver: solution, counters and RNG
  stream after the call, 8 moduli from 2^16 to 2^63, 3 sizes, dense/sparse projection, `wiedemann_coord` square and not),
  `ic_control::tests::mulmod_addm_match_u128_across_moduli` (u128 `%` equality, r from 3 to 2^63 - 1 including 2^50 - 1, 2^50, 2^50 + 1).

## W1: counts and outputs identical: held
* Fixture: `count_reference.json` (24 entries, genus 3 and 4, p in {1031, 2053, 8209}, arms Full and DoubleLp, with and without
  SGE(8,3); every counted field and `recovered`; 24/24 certified) recorded from the unmodified test binary
  (sha256 `ed4de8c6...`, HEAD 8ad232bb21 plus the test module only). After the change the test passes (88 s run).
* Existing tests: `genus_ladder::` 25 passed, 0 failed (3 ignored), including `sge_none_reproduces_frozen_counts` and
  `excess_none_and_noop_reproduce_baseline_counts`.
* Modules that import `ic_control`: `chained_transfer::` 96 passed, `quartic_ladder::` 9, `smith_canonical::` 10, `prime_radical_root::` 31,
  0 failed in each (the other importers are binaries and examples; not run). The shared helper change was therefore kept.
* Timing instances: the meta line (la_mul, la_add, la_wied_mul, la_graph_mul, la_peel_mul, la_sge_mul, rel_mul, cantor_mul/inv, trials,
  core shape, nnz, r) of the before and after series is equal for all three p (`tables.md`); the instrument asserts a certified
  one-attempt solve and the same `la_mul` on every trial. The recovered t is not printed by the instrument; certification is the evidence there.

## W2 and the timing (median of 15 trials after 2 warm-ups; load average 0.87-1.11, no trial at or above 1.5)

| p | t_R before ns/la_mul | after | speedup (min/max extremes) | A/A before / after | kappa in situ before -> after |
|---|---|---|---|---|---|
| 65537 | 100.7 | 16.6 | 6.06x (3.8-12.1) | 1.6% / 14.1% | 18.2 -> 2.86 |
| 131101 | 120.4 | 20.3 | 5.93x (4.5-9.4) | 4.8% / 5.1% | 16.6 -> 2.96 |
| 262147 | 104.4 | 19.0 | 5.49x (4.4-6.9) | 1.3% / 2.1% | 16.3 -> 2.66 |

LA wall (median): 5.25 -> 0.87 s, 17.9 -> 3.01 s, 44.5 -> 8.11 s. W2 predicted >= 1.5x at 262147 and >= 1.2x at 65537: held, with a
large margin; the measured speedups are about 4x the prediction. Per counted `la_add`, 262147 goes from 9.0 to 1.6 ns.
t_F in situ is 5.5-7.2 ns before and 5.8-7.2 after (the Cantor code is untouched; the difference is noise, A/A of t_F is in `analysis.json`).

## W3: re-priced frontier row: held
R2's SGE(8,3) arm over negation rho at p = 262147, from the recorded counts exactly as the kappa study (reproduced first:
0.479 / 0.625 / 0.568 at the old in-situ kappa 17.9 / 17.0 / 15.8):

| p | kappa = 9 | old in-situ kappa | new in-situ kappa (this study) |
|---|---|---|---|
| 65537 | 0.300 | 0.479 | 0.176 (kappa 2.86) |
| 131101 | 0.399 | 0.625 | 0.228 (kappa 2.96) |
| 262147 | 0.377 | 0.568 | 0.198 (kappa 2.66) |

Predicted <= 0.48 at 262147; measured 0.198. These ratios use the LA wall of the whole LA phase (graph build, peel, SGE, Wiedemann,
certification) per counted `la_mul`, as in the kappa study. Gap to the matched rho remains a distance, not a pass/fail: the arm is now
about 5x below negation rho at this price. This is not a T0-T3 claim and not a leaderboard row (no validation gate evidence).

## Limits
One VM, single thread, wall clock, no PMU; one instance per p; t_R is a difference of two whole-run wall times (the relation phase
is 13 s of a 57 s run at 262147, so its noise enters the difference; at 65537 the after-LA is under 1 s and its A/A is 14%, the
speedup there is the least precise). Before and after series ran back to back, before first, so slow drift of the host is not
separated from the change; the before series reproduces the kappa study's earlier numbers within 5% (100.7 vs 99.6, 120.4 vs
114.9, 104.4 vs 107.1), which is the only drift check. Other work shared the VM only while compiling and testing, which finished before every timing; the
timed processes ran alone. The 3 unmodified-instance fixture p values are small, so the fixture exercises r above 2^50 only where genus 4 gives it
(the oracle and the `mulmod` tests cover r to 2^63). Not run: other hosts, other genera at scale, other callers' timing
(`quartic_ic`, `full_jacobian_ic` still use `ic_control::wiedemann`; they gain only the `mulmod`/`addm` change).

## Deviations
* The brief placed the solver and `mulmod`/`addm` in `genus_ladder.rs` and `src/cryptanalysis/ic_control.rs`; they are in
  `chained_transfer/ic_control.rs`. Scope was agreed with the coordinator: local solver in `genus_ladder.rs`, only `mulmod`/`addm` changed in `ic_control.rs`.
* 15 measured trials per p (the full budget), 2 warm-ups. A/A is split-half over trials, as in the kappa study.
* An accidental crate-wide `cargo fmt -- <file>` early on reformatted 12 files outside the scope; the coordinator reverted them.
  From then on only `rustfmt --edition 2021` on my two files.
* `git diff --stat src/` shows only `genus_ladder.rs` and `ic_control.rs` (read-only check at the coordinator's request).
* rustfmt clean on both files; `cargo clippy --release -j2 --lib --bin genus_ladder` reports no warning in either file.
