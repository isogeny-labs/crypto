# H1: cofactor-projected factor base (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host; no replay certificate, validator report or §5a band. Not a leaderboard row. Metric is `trials_total` (relation-sampler trials), an operation count, not wall time.

**Mechanism (read from `find_one_relation_counted_with_rng`).** The baseline factor base is the set of small-x points of the whole curve; every target R = aG + bQ lies in the prime-order subgroup <G>. A pair F_i ± F_j can only equal R if it lies in <G>, which holds for about 1 in h pairs, so trials scale with p ≈ h·r (baseline 1: trials/p about 0.5). H1 uses entries [h]F instead (`build_factor_base_projected`), so every pair lies in <G>. Opt-in flag `--project-cofactor` on `ic solve-prime` / `solve-prime-file`; the default path is unchanged.

**Run.** Same 144 instances and seeds 1..3 as baseline 1 (generator seed `e222m221-v1`), shuffled, same binary, flag on vs off: 432 runs per arm, all 864 recovered the planted logarithm.

**Result (paired by instance and seed, flag-on trials / flag-off trials):**

| family | cofactor | median ratio | IQR | trials/p off -> on |
|---|---|---|---|---|
| E (Edwards analog) | 4 | 0.250 | 0.220 to 0.304 | 0.564 -> 0.139 |
| M (Montgomery analog) | 8 | 0.141 | 0.112 to 0.178 | 0.517 -> 0.070 |
| W (cofactor-1 control) | 1 | 1.000 | 1.000 to 1.000 | 0.568 -> 0.568 |

With the flag on, E and M cost the cofactor-1 control at equal r: ratio to W 0.92 to 1.08 over rungs 10 to 17 (bootstrap intervals contain 1 except M at 12, 0.84 to 0.97). The flag-off arm reproduces baseline 1 exactly (identical medians), so the default path is unchanged. The prediction was a drop by about h in trials/p on E and M and none on the control; E meets it (1/4), M reaches 1/7.1 against 1/8. The control being exactly 1.000 shows the change does nothing when h = 1.

**Reading.** The improvement is real against the baseline at equal group size and removes the p/r penalty of baseline 1. It brings the cofactor-h curves down to the cofactor-1 cost, not below it, so the distance to matched rho on E and M falls to the control's (lower-bound table: E 18.5x to 360x becomes about 4.8x to 82x). The factor-base entries [h]F are no longer small-x points, so any lever that relied on small x (the sweep over F_i's x) is unchanged in the code but its meaning is different; nothing was optimised for that.

**Tests.** `cargo test --release --bin ic prime_` 20 passed (the file-solver test now also runs the flag and expects scalar 190); `cargo test --release --lib ec_index_calculus` 43 passed.

**Not done.** Applying the projection to the other producers (`prime-orbit`, `ic bench`, S4); sizes above the 10^6 cap; the native E-222 / M-221 pricing (cofactors 4 and 8 give the 4x and about 7x to 8x factors here); isolab-eks run, replay and matched-rho validation; Conductor task, KB queries, resume checkpoint, benchrec record.
