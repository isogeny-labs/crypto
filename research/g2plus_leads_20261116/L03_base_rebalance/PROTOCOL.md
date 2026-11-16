# L03: re-balance the genus-3 double-large-prime factor base when relations are cheap (protocol)

Status: frozen before any data (`frozen.sha256`). Every number quoted from earlier lab work
is `[pcc:unverified]`; everything this lead produces is `[pcc:unverified]` until an
independent replay. This is not a benchmark: no leaderboard row, no `benchmarks/` row, no
speed claim, no wall-time metric. Coordinator: L03 of the g2plus pass of 2026-11-16.
Implementation: [`SPEC.md`](SPEC.md).

## 0. Prior art checked (grep; research-kb is not installed in this container)

| path | what it showed |
|---|---|
| `crypto/research/genus_ladder_20261114/{PROTOCOL,RESULTS}.md` | Defines the instrument (`src/cryptanalysis/genus_ladder.rs`), the 2LP arm with base `ceil(2 p^{1-1/g})` and the FPM unit `cantor_mul + I cantor_inv + rel_mul + kappa la_mul`, `(I, kappa) = (10, g^2)` primary. "Next steps" item 1: **"Tune the base constant"**. That item is still open. |
| `crypto/research/genus_ladder_bound_r_20261114/RESULTS.md` | Its `r` is the prime subgroup order bound to `p`, **not** a base fraction. Genus 4 and quartics, `cc = 2.0`. No base sweep. |
| `crypto/research/genus_ladder_scale_20261115/{RESULTS.md,scale_g3.jsonl}` | Source of the brief's baseline: g = 3, `C_double_lp+pre`, `cc = 2.0`, 2LP/rho = 0.591, 0.741, 0.537 at p = 8209, 16411, 32771, LA share 0.75–0.77 `[pcc:unverified]`. Its 9 genus-3 instance lines (f, order, h, r) and per-run counts are this lead's instances and calibration targets. |
| `crypto/research/genus_ladder_sge_20261115/RESULTS.md` | SGE `sge(8,3)` `la_mul` ratio **0.255 [0.242, 0.269] at genus 3**, 0.347 at g = 4, 0.274 at g = 5 (p ≤ 2053, `cc = 2.0`) `[pcc:unverified]`. The brief's "0.27–0.35" are the genus-4 and genus-5 values. One base constant only. |
| `crypto/research/genus_ladder_harvest_20261115/PROTOCOL.md`, `genus_ladder_sge_scale_20261115/PROTOCOL.md` | `cc = 2.0` fixed throughout. |
| `src/bin/genus_ladder.rs` | Has a `--cc` flag (default 2.0). Every recorded run used 2.0. No built binary in `target/`. No crate registry offline. |
| `crypto/research/leads_20261111/EXISTING_LEADS.txt` | `LEAD-ICB-large-prime-variants`, `LEAD-ICE-relation-graph-rank-large-prime`. |
| `crypto/research/lead_sweep_20261110/LEAD-ICB-large-prime-variants/RESULT.md`, `.../LEAD-ICE-relation-graph-rank-large-prime/RESULT.md` | Integer-smoothness toys in `F_p^*`: 2LP vs full smooth trial ratios. No Jacobian, no base-size optimisation, no LA cost. Not duplicates. |
| `crypto/research/g2plus_leads_20261116/L01_anchored_pencil_sieve/` | Sibling lead (only a PROTOCOL). This lead does not use L01's result. |
| `cryptanalysis/` (grep "large prime", "base size") | `experiments/ic-native-current-20261115` is finite-field IC (automatic base policy). Nothing sweeps a hyperelliptic 2LP base constant. |
| `src/cryptanalysis/genus_ladder.rs`, `chained_transfer/ic_control.rs`, `ic_boundary.rs`, `residual_walk.rs` | Code ported by SPEC.md (Cantor, smoothness, DSU, build_rows, peel, Wiedemann, counters). |

Verdict: **not a duplicate**. Nobody has varied the 2LP base constant at any genus, and
the one study that names it lists it as an open next step.

## 1. Corrections to the brief

1. **Name clash on `r`.** In the lab, `r` is the prime order of the subgroup. Here the
   base is parameterised as the lab does it: `nb = min(ceil(cc · p^{1-1/g}), p)`, `g = 3`,
   with `cc` swept. The *base fraction* is reported as `beta = fb_cols / n_full`, where
   `n_full = #{x in F_p : chi(f(x)) = +1}` (about `p/2`). At `cc = 2`, `beta` is about
   0.20, 0.16 and 0.125 at the three primes.
2. **Name clash on `c`.** `cc` is the base constant. The synthetic per-trial discount is
   called `d in {1, 3, 10, 30}`. `d = 1` is the measured cost.
3. **What is discounted.** The discount divides the whole *per-trial* relation-phase
   cost. That is the Cantor arithmetic of the walk (steps and restarts) plus the
   smoothness, root and discriminant work (`rel_trial_mul`). The lab's per-trial cost at
   `cc = 2` is about 279 Cantor FPM plus about 230 `rel_mul` `[pcc:unverified]`, so
   discounting Cantor alone would leave about 45% of the trial undiscounted. The fixed
   tables (character table, base scan), the jump-table set-up and the certificate are not
   discounted. The discount keeps the yield per trial and the relation distribution
   unchanged. A real cheaper engine (L01) may change either; this lead does not model
   that.
4. **SGE factor.** At genus 3 the lab measured 0.255, not 0.27–0.35. The parameter is
   `lambda in {1, 0.35, 0.255}`, multiplying the LA term. It was measured at `cc = 2`,
   `p ≤ 2053`. Holding it fixed across `cc` is an assumption (row weight changes with
   `cc`), so `lambda < 1` never enters the primary decision.
5. **"Trials to a full-rank system".** Precisely: the trial count at the linear-algebra
   attempt that returns a certified logarithm. The stopping rule is the lab's:
   collect until union-find cycles in the special vertex's component reach
   `nunk + 8 + nunk/100`, `nunk = fb_cols + 1`. Then build cycle rows, peel, require
   `core_rows ≥ core_cols + margin`, run Wiedemann, and certify `[t]G = T`. On failure,
   collect more and retry.
6. **Expected direction, a model to check.** Under the GTTD heuristic for 2LP, a split
   divisor has at most 2 large primes with probability `1-(1-beta)^3`, which is about
   `3·beta` for small `beta`. The relations needed scale with the number of large-prime
   vertices, which is about `p/2`. So trials ≈ `K p^2 / nb`. Lab data at `cc = 2` give
   K ≈ 0.53, 0.49, 0.46. LA ≈ `C_L nb^2` (`la_mul` is dominated by the `D` scaling and the
   `t` column in each `A^T D A` apply, about `3 nb` applies). With
   `T(nb) = R/(d·nb) + lambda·L·nb^2`, the optimum is
   `nb* = (R / (2 d lambda L))^{1/3} ∝ (d·lambda)^{-1/3}`, where the relation term is
   twice the LA term. The best total is `(3/2)(R/d)^{2/3}(2 lambda L)^{1/3} ∝ d^{-2/3} lambda^{1/3}`,
   in the regime where those power laws hold. At the lab
   point (relation share 0.25, LA share 0.75) the model gives three expectations:
   (a) `cc = 2` is above the optimum even at `d = 1`, with `cc*_1` ≈ 1.1 and a gain of
   about 32%; (b) `cc*_10` ≈ 0.5; (c) a gain at `d = 10` against `cc = 2` of about 81%.
   **These are model expectations, not thresholds.** The model can fail at small `nb`:
   the 2LP graph near the percolation threshold, longer cycles, and heavier rows.
   Measuring that failure is the reason for the simulator.
7. **The kill criterion is confounded as written.** "Total cost at the re-optimised base
   improves < 15% at `d = 10`" can be passed by pure re-tuning at `d = 1`, which has
   nothing to do with cheap relations (expectation (a)). The brief's criterion is kept
   unchanged as the primary decision (thresholds are not moved). Three things are
   reported beside it. The `d = 1` re-tuning gain `G_1`, labelled as a lab-baseline
   re-tuning. The interaction gain `G_inc` (§6, H2). And a three-factor attribution in
   which base, engine discount and SGE are never multiplied together blindly (§6).

Corrected claim: *for genus-3 imaginary hyperelliptic 2LP index calculus as implemented
in `genus_ladder.rs` (generic Cantor, parity prefilter, Wiedemann), on the lab's nine
admitted curves at p ∈ {8209, 16411, 32771}, re-optimising the base constant `cc` under
a synthetic per-trial discount `d = 10` lowers total counted FPM_mid by at least 15%
against the lab's `cc = 2`.*

## 2. Hypotheses

- **H1 (primary, the brief's kill).** `G_10 = 1 − FPM(d=10, lambda=1, cc*) / FPM(d=10, lambda=1, cc=2) ≥ 0.15`.
- **H2 (secondary, interaction).** The optimum moves down with the discount:
  `cc*_{10} < cc*_{1}` on the grid at ≥ 2 of 3 primes, and
  `G_inc = 1 − FPM(d=10, cc*_{10}) / FPM(d=10, cc*_{1})` has 95% lower bound ≥ 0.10 at
  p = 32771. Reported held or failed; no verdict on the lead.
- **H3 (secondary, cost model).** Over `cc ∈ [0.25, 2]`, the OLS slope of `ln trials` on
  `ln fb_cols` lies in [−1.3, −0.7] at every prime. The slope of `ln la_mul` on
  `ln core_cols` over all `cc` lies in [1.7, 2.4] at every prime. Reported held or failed.

## 3. Arms and variables

Single instrument, the standalone port `sim/` (SPEC.md), on every arm.

| arm | what varies | role |
|---|---|---|
| B (baseline) | `cc = 2.0`, `d = 1`, `lambda = 1` | the lab's arm `C_double_lp+pre` |
| S_cc (sweep) | `cc ∈ {2^{k/2} : k = −6..4}` = 0.125, 0.177, 0.25, 0.354, 0.5, 0.707, 1, 1.414, 2, 2.828, 4 | measured base sweep |
| D_d | `d ∈ {1, 3, 10, 30}` applied in analysis | synthetic engine discount, **labelled synthetic** |
| L_lambda | `lambda ∈ {1, 0.35, 0.255}` applied in analysis | SGE parameter, **labelled parameter** |
| A/A | `cc = 2`, selection seeds {1,2} vs evaluation seeds {3,4} | noise floor |
| PC1 (positive, instrument) | trials at `cc = 0.25` vs `cc = 2`; `la_mul` at `cc = 4` vs `cc = 2` | the instrument sees the trade-off |
| PC2 (positive, analysis) | synthetic `T(cc) = A/cc + B cc^2` tables | the selection code finds a known optimum |
| NC1 (negative) | `d` changes no counted field; the discounted term scales exactly `1/d` | the discount is pure post-processing |
| CAL (calibration A/A across instruments) | port at `cc = 2` vs lab `scale_g3.jsonl` | the port reproduces the lab instrument statistically |

Held fixed: genus 3; arm 2LP with parity prefilter; base = the first `nb` abscissae
`x = 0, 1, 2, ...` with `chi(f(x)) = +1` (the lab's rule); walk with 16 jumps
`alpha G + beta T`, `alpha, beta < min(r, 4096)`, restart every 4096 steps from
`[a]G + [b]T`; stopping rule and margins as in §1.5; Wiedemann with sparse projection
(dense on retry); `max_trials = 60,000,000`; FPM bands
`(I, kappa) ∈ {(1, 1), (10, 9), (30, 9)}`, primary `(10, 9)`.

**Common random numbers.** The run RNG is seeded from `(p, curve_seed, seed)` only,
not from `cc`. So every `cc` of one `(curve, seed)` walks the same divisor sequence (the
walk and root-finding draws do not depend on the base) up to its first LA attempt. That
pairing is deliberate; the A/A uses disjoint seeds.

## 4. Instances and seeds

- The nine genus-3 instance lines of `research/genus_ladder_scale_20261115/scale_g3.jsonl`
  (SHA-256 of that file `ed8258049ae38a5fab5ebc5dfe6138d7823aa99ed2b2b2016297c03a311382fd`).
  Extracted by `grep '"kind":"instance"'` into `instances.jsonl`, SHA-256
  `91ddea89649a852f9b8c73e6a7affa4e2adde701624db0700e282f0846517148`.
  - p = 8209: curve_seed 6, 18, 23. p = 16411: 8, 27, 43. p = 32771: 8, 16, 31.
  - Generator and target: the lab's `instance_from(curve, order, h, r, target_seed = curve_seed)`,
    ported with the lab's SplitMix64 `random_divisor`. `G`, `T` and `t` are therefore
    the lab's own values.
- Solver seeds: selection set S = {1, 2}; evaluation set E = {3, 4}.
- Sizes: 9 instances × 4 seeds × 11 `cc` = **396 runs**, as 36 processes (one per
  instance and seed, looping over `cc`). Estimated each process ≤ 3 CPU-min, peak
  memory < 500 MB, one thread.
- p = 65537 is **not** run: no admitted genus-3 instance with known `#J` exists, and
  point counting over `F_{p^3}` is out of budget. Declared limit.

## 5. Metrics and unit

Counted per run (SPEC.md §6 lists the fields): trials, candidates, prefilter rejects,
splits, relations by large-prime count, duplicate relations, large-prime vertices,
cycles, LA attempts, `fb_cols`, `n_full`, `beta`; core rows, columns, non-zeros,
non-`±1` entries and maximum row weight of the solved core; Cantor `F_p`
multiplications and inversions split into set-up, loop and certificate; `rel` mul/add
split into fixed and per-trial; `la` mul/add split into graph, peel, Wiedemann and DSU;
group operations; recovered `t`, planted `t`, certified, exhausted.

Unit, per band `(I, kappa)`, discount `d`, SGE parameter `lambda`:

```
F_fixed = (cm_setup + I ci_setup) + (cm_verify + I ci_verify) + rel_fixed_mul
F_rel   = (cm_loop  + I ci_loop)  + rel_trial_mul
F_la    = kappa · la_mul
FPM_tot(d, lambda) = F_fixed + F_rel / d + lambda · F_la
```

At `d = 1`, `lambda = 1` this is the lab's `FPM` (`cantor_mul + I cantor_inv + rel_mul + kappa la_mul`),
term for term. Sensitivity reading S_add (reported, not decisive):
`F_la' = kappa · (la_mul + 0.1 · la_add)`, because the lab's unit does not price the
`±1` additions that carry the matrix weight.

Rho reference, for the predicted IC/rho ratio only (no ranking claim; the §5a GAE band is
not computed, so no ordering against rho is stated). Two readings:
(i) the lab's measured negation-rho FPM_mid means on these same curves `[pcc:unverified]`:
1.425e8 (8209), 2.887e8 (16411), 1.008e9 (32771).
(ii) the model `sqrt(pi r / 4)` × the port's measured FPM per group operation
`(cm_loop + I ci_loop) / group_ops_loop`, averaged over the prime's curves.
The ratio is reported as the distance remaining, `ratio = FPM_tot(cc*) / rho`.

## 6. Analysis and decision rule

All computed by `analyze.py` (SPEC.md §8), written before data.

1. **Selection** (set S only). For each `p, d, lambda, band`:
   `cc*` = argmin over the grid of the mean over S runs (3 curves × 2 seeds) of `FPM_tot`.
   On an exact tie, take the larger `cc`. Flag `edge` if `cc* ∈ {0.125, 4}`.
2. **Evaluation** (set E only, `cc*` frozen from S):
   `G(p, d, lambda) = 1 − Σ_E FPM_tot(cc*) / Σ_E FPM_tot(cc = 2)`.
   The 95% interval is a paired hierarchical bootstrap: resample the 3 curves with
   replacement, then the 2 seeds within each drawn curve with replacement, keeping the
   `(curve, seed)` pairing across `cc`. B = 4000, Python `random.Random(20261116)`,
   percentile interval.
3. **Attribution** (E, band mid). Baseline `F0 = Σ_E FPM(d=1, lambda=1, cc=2)`. Factors:
   engine `E_d = FPM(d, 1, 2) / F0`; rebalance `R_d = FPM(d, 1, cc*_{d,1}) / FPM(d, 1, 2)`;
   SGE `S_{d,lambda} = FPM(d, lambda, cc*_{d,lambda}) / FPM(d, 1, cc*_{d,1})`.
   Their product is the total, each factor is reported separately, and the re-tuning
   gain `G_1 = 1 − R_1` is labelled as such.
4. **Primary decision**, `d = 10`, `lambda = 1`, band `(10, 9)`, `G_10(p)`:
   - **killed** if the point estimate `G_10 < 0.15` at ≥ 2 of the 3 primes, **or** the
     95% upper bound of `G_10 < 0.15` at p = 32771;
   - **supported** if the 95% lower bound of `G_10 ≥ 0.15` at ≥ 2 of the 3 primes,
     **including** p = 32771;
   - **inconclusive** otherwise, or if any gate in §7 fails.
   The two outcomes cannot both hold. Supported needs the 32771 lower bound ≥ 0.15
   (so the upper bound is too) and at most one prime with a point estimate below 0.15.
5. **Secondary**: H2 and H3 held or failed by the numbers in §2. `G` for every
   `(d, lambda, band)`, with edge flags. `cc*` and `beta*` against `d`. Predicted
   IC/rho ratios (both rho readings, every `d` and `lambda`, labelled synthetic for
   `d > 1` or `lambda < 1`). The S_add sensitivity `G_10`. Matrix dimension and weight
   against `cc`.

## 7. Correctness gates (all must pass before any metric is read)

| id | gate | failure → |
|---|---|---|
| Q1 | `cargo test --release` in `sim/`: all tests pass (SPEC.md §9 lists the expected values) | stop |
| Q2 | `instances.jsonl` SHA-256 equals §4 | stop |
| Q3 | `l03sim selftest`: for each instance, `G ≠ 0`, `[r]G = 0`, `[order]D = 0` for 4 seeded random divisors `D`, and `T = [t]G` | stop |
| Q4 | every metric run: `certified = true`, `recovered = t_planted`, `exhausted = false` | inconclusive |
| Q5 | `fb_cols` equals SPEC.md §5's table for every `(p, cc)` | inconclusive |
| Q6 | relation check: `--verify-relations` runs (p = 8209, all 3 curves, seed 1, all `cc`) give `bad_relations = 0` and `certified = true` | inconclusive |
| Q7 (CAL) | pooled over the 12 runs at `cc = 2` per prime, against the lab's means: Cantor FPM_mid per trial within ±5% of 280.36 / 278.62 / 278.52; `rel_mul` per trial within ±10% of 207.86 / 228.72 / 231.98; mean trials within ±15% of 43,964 / 102,969 / 242,686; mean `core_cols` within ±5% of 741.0 / 1196.8 / 1933.2; mean `la_mul` within ±20% of 6.976e6 / 1.798e7 / 4.643e7 (p = 8209 / 16411 / 32771) | inconclusive |
| Q8 (A/A) | `|mean_S FPM_mid(cc=2, d=1) / mean_E FPM_mid(cc=2, d=1) − 1| ≤ 0.05` at every prime | inconclusive |
| Q9 (PC1) | mean trials `cc = 0.25` ÷ `cc = 2` ≥ 3.0, and mean `la_mul` `cc = 4` ÷ `cc = 2` ≥ 2.0, at every prime | inconclusive |
| Q10 (PC2, NC1) | `python3 analyze.py --selftest` passes | stop |

A hook refusal is recorded verbatim in `raw/REFUSALS.txt` and is not worked around.

## 8. Confounders and their isolation

| confounder | isolation |
|---|---|
| Different RNG from the lab (`StdRng` there, SplitMix64 here) | statistical calibration gate Q7; identical code paths otherwise |
| Winner's curse from picking the best of 11 grid points | selection on S, evaluation on disjoint E |
| Pure re-tuning masquerading as a cheap-relation effect | `G_1`, `G_inc` (H2) and the attribution factors reported beside `G_10` |
| SGE factor measured at another `cc` and `p` | `lambda` only in labelled secondary rows; never in the decision |
| Discount realism (yield and distribution unchanged) | stated in the claim; `d` is post-processing (NC1) |
| FPM ignores LA additions (matrix weight) | S_add sensitivity, and `core_nnz` reported |
| Curve selection (admitted `h ≤ 8`, `r` prime) | same nine curves as the lab baseline and its rho; no new curves |
| Base made of the smallest abscissae, not random ones | the lab's rule kept on every arm; declared |
| Duplicate walk points (birthday) | counted (`dup_relations`), not dropped, as in the lab; expected < 1 per run |
| Grid edge | `edge` flag; a gain at an edge optimum is a lower bound |
| Shared host | counts only; no wall-time metric |
| Analyst freedom | `analyze.py` and every threshold frozen in SPEC and here before data |

## 9. Commands (executor runs these unchanged, from the lead directory)

```sh
cd /home/user/crypto/research/g2plus_leads_20261116/L03_base_rebalance
grep '"kind":"instance"' ../../genus_ladder_scale_20261115/scale_g3.jsonl > instances.jsonl
sha256sum instances.jsonl | tee raw/instances.sha256            # Q2
(cd sim && cargo build --release --offline 2>&1 | tee ../raw/build.log)
(cd sim && cargo test --release --offline 2>&1 | tee ../raw/test.log)   # Q1
sha256sum sim/target/release/l03sim | tee raw/binary.sha256
python3 analyze.py --selftest 2>&1 | tee raw/analyze_selftest.log        # Q10
sim/target/release/l03sim selftest --instances instances.jsonl --out raw/selftest.jsonl   # Q3
for cs in 6 18 23; do
  sim/target/release/l03sim run --instances instances.jsonl --p 8209 --curve-seed $cs \
    --seed 1 --set V --verify-relations --out raw/verify_8209_c${cs}.jsonl              # Q6
done
for p_cs in 8209:6 8209:18 8209:23 16411:8 16411:27 16411:43 32771:8 32771:16 32771:31; do
  p=${p_cs%%:*}; cs=${p_cs##*:}
  for s in 1 2 3 4; do
    set_=S; [ $s -ge 3 ] && set_=E
    sim/target/release/l03sim run --instances instances.jsonl --p $p --curve-seed $cs \
      --seed $s --set $set_ --out raw/runs_p${p}_c${cs}_s${s}.jsonl 2> raw/runs_p${p}_c${cs}_s${s}.log
  done
done
sha256sum raw/*.jsonl > raw/data.sha256
python3 analyze.py raw/runs_*.jsonl --verify raw/verify_8209_c*.jsonl \
  --out analysis/analysis.json --tables analysis/tables.md 2>&1 | tee raw/analyze.log
```

Outputs: `raw/runs_*.jsonl` (one config line, one instance line, 11 run lines each),
`raw/verify_*.jsonl`, `raw/selftest.jsonl`, `raw/*.log`, `raw/data.sha256`,
`analysis/analysis.json`, `analysis/tables.md` (contains the verdict line).

Not done by this lead (the main session's responsibility, per the hard rules): resume
checkpoint, bench record, session log, commits.
