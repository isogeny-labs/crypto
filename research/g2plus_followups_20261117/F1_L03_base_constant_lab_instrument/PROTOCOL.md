# F1 - L03 follow-up: 2LP base constant cc = 1 against cc = 2 in the lab genus_ladder instrument, genus 3 (frozen protocol, 2026-11-17)

Follow-up to `research/g2plus_leads_20261116/L03_base_rebalance` (a standalone
simulator). Here the same question is re-measured **in the lab instrument itself**,
`genus_ladder`, at its native per-trial cost (no synthetic relation discount).
Every figure `[pcc:unverified]`. Synthetic random curves only; no deployed-curve
statement. **Not a benchmark**: no leaderboard row, no `benchmarks/` row, no speed
claim, no isolated host, no validator replay, no matched-rho ordering claim. Counted
field-operation units only; wall time is never a metric.

The coordinator (this document, SPEC.md, PILOT.md) was frozen before any decision run.
The decision experiment is run by a separate executor from the exact commands in §9.

## 0. Instrument exposes the base constant - no source edit, no port

The lab binary **does** expose the double-large-prime base constant as the CLI flag
`--cc` (default `2.0`). The driver computes the factor-base size that the solver uses
as `nb = fb_size_2lp(p, g, cc) = ceil(cc * p^(1 - 1/g))` and passes it to `run_ic`
as `IcParams.nb`, which caps the number of factor-base columns (`want = prm.nb` for
the `DoubleLp` arm; `src/cryptanalysis/genus_ladder.rs:1303-1324`,
`src/bin/genus_ladder.rs:368-372`). The two `fb_size_2lp(p,g,2.0)` call sites in
`genus_ladder.rs` (lines 2249, 2503) are inside `#[cfg(test)]` helpers and are not on
the driver path. **Therefore the measurement is done directly with the unmodified lab
binary at `/home/user/crypto-c0b657b/target/release/genus_ladder`; no source is
copied and no count-identity-against-a-port gate is required.** (The pilot still
anchors the binary to the lab's own recorded counts; §Q4.) For genus 3,
`p^(1 - 1/g) = p^(2/3)`, so `cc = 1` is half the columns of `cc = 2`.

This was verified on the pilot (PILOT.md): at `p = 1031`, `--cc 1.0` gave
`fb_cols = 103 = ceil(1031^(2/3))` and `--cc 2.0` gave `fb_cols = 205`; at
`p = 8209`, `407` and `814`. The flag changes the measured base exactly.

## 1. Hypothesis

**H1 (measured, no synthetic discount).** In the lab `genus_ladder` instrument, on
genus-3 curves, the double-large-prime factor-base constant `cc = 2.0` (the lab
default and the baseline of every prior genus-ladder study) is **past** its
field-operation-count optimum: reducing it to `cc = 1.0` lowers the total counted
cost `FPM_mid` of arm `c` (`C_double_lp+pre`) on the same curves and seeds, because
the linear-algebra term (priced `g^2 * la_mul`) dominates at `cc = 2` and falls faster
in the base size than the relation-collection term grows.

This is exactly the **only non-synthetic part** of the L03 result (its `G_1`, the
`d = 1` re-tuning, 36-40% mid-band). The L03 `H1`/`G_10` reading rested on a synthetic
per-trial discount `d = 10`; **this follow-up does not use `d` at all** (confounder
C1), so nothing here is conditional on a cheaper relation engine.

Supporting structural claims, reported but not themselves the decision:
- **S1 (optimum is interior).** Across `cc in {0.5, 1, 1.5, 2, 3}` the `FPM_mid` of
  arm `c` is U-shaped in `cc` with a minimum at `cc <= 1.5`; `cc = 2` is on the rising
  (LA-dominated) side and `cc = 0.5` on the rising (trial-dominated / exhaustion) side.
- **S2 (LA share).** At `cc = 2` the linear-algebra share `g^2 * la_mul / FPM_mid`
  exceeds 0.70 at every prime, and falls at `cc = 1`.

## 2. Exact claim (what "supported" asserts)

On genus-3 curves at `p in {8209, 16411, 32771}`, for arm `c` (`C_double_lp+pre`) in
the lab `genus_ladder` instrument at its native cost, the paired mid-band fractional
cost reduction from `cc = 2.0` to `cc = 1.0`,

  `G(p) = 1 - mean_over_pairs[ FPM_mid(cc=1) / FPM_mid(cc=2) ]`,

is positive and at least 0.15 at every prime, with a 95% paired-bootstrap confidence
interval excluding 0 at every prime, and by a margin exceeding the A/A seed-noise floor
(§4, §6). Reported alongside, never as the pass/fail: `G` at band low and high, the
relation-phase and linear-algebra shares at each `cc`, the argmin `cc` of the secondary
sweep, and the ratio of arm `c` to the instrument's own negation-rho arm at each `cc`
(distance remaining, not a gate).

## 3. Arms (each arm pays its own set-up; the instrument charges it)

The instrument builds every arm's factor base and walk set-up from scratch inside
`run_ic`/`run_rho` and counts it in that arm's own counters (character table in
`rel_mul`, jump set-up in `setup_ops`/`group_ops`), so there is no cross-arm subsidy
(lesson c). The arms:

| role | arm | `cc` | solver seeds | note |
|---|---|---|---|---|
| baseline | `c` (`C_double_lp+pre`) | **2.0** | 1,2,3,4 | lab default; the thing we re-tune against |
| treatment | `c` | **1.0** | 1,2,3,4 | half the columns |
| positive control / secondary sweep | `c` | 0.5, 1.5, 3.0 | 1,2 | establishes the interior optimum (S1) |
| negative control (A/A noise floor) | `c` | 2.0 | 1,2,3,4 | split into seed-group {1,2} vs {3,4}; same `cc`, different solver seeds |
| reference | `R` (`R_rho_neg`) | n/a | 1,2,3,4 | negation-rho on the same curves; `cc`-independent, run once |

Baseline justification: `cc = 2.0` is the exact setting of `research/genus_ladder_*`
and of the lab's `C_double_lp+pre` arm; nothing is added to or removed from it. The
A/A arm reuses the baseline's four `cc = 2` runs - no extra runs - and pairs seed-group
A = {seed 1, seed 2} against seed-group B = {seed 3, seed 4} on the same curves to
measure how much `G` moves from solver-seed noise alone (confounder C6). The rho
reference is `cc`-independent (verified Q5), so it is run once (in the `cc = 2.0`
invocation) and reused for every `cc`.

## 4. Pairing and statistic (fixed before data)

Pairing key: `(p, curve_seed, seed)`. For the treatment test, each `cc = 1` run is
paired with the `cc = 2` run of the same key; the per-pair ratio is
`FPM_band(cc=1) / FPM_band(cc=2)` for `band in {low, mid, high}`. Statistic: the mean
of the per-pair ratios, pooled over the 3 curves x 4 seeds = 12 pairs **per prime**;
`G(p) = 1 - mean`. 95% interval by hierarchical bootstrap: resample the 3 curve_seeds
with replacement, then the 4 seeds within each resampled curve, 2000 resamples, RNG
seed 20261117 (`random.Random(20261117)` in the stdlib). The same procedure gives the
A/A statistic `G_AA(p) = 1 - mean_over_pairs[ FPM_mid(seed in B) / FPM_mid(seed in A) ]`
pairing seed 3<->1, 4<->2 on the same curve at `cc = 2`, with its own 95% CI.

`FPM_band = cantor_mul + I * cantor_inv + rel_mul + kappa * la_mul`, the instrument's
own unit (`src/bin/genus_ladder_analyze.rs:12-16`): band low `(I, kappa) = (1, 1)`,
band **mid (primary)** `(10, g^2) = (10, 9)`, band high `(30, 9)`. Relation-phase term
`= cantor_mul + I * cantor_inv + rel_mul`; linear-algebra term `= kappa * la_mul`;
LA share `= kappa * la_mul / FPM_band`. These are exact integer/rational combinations
of recorded counters; no re-simulation. The rho ratio at each `cc` is
`FPM_band(c) / FPM_band(R_rho_neg)` on the matched `(p, curve_seed, seed)` (rho has
`rel_mul = la_mul = 0`, so its `FPM = cantor_mul + I * cantor_inv`), reported with a
bootstrap CI, parameter-labelled, never decisive.

## 5. Metrics and unit

Unit = the lab instrument's own FPM (field-multiplication-equivalents), defined in §4,
**not** the generic FPM of the task note - the task note says to use the lab
instrument's unit when it defines one, and it does. Exact counts; seeds recorded in
every record; no wall time. Secondary reported counters (no decision): `trials`,
`relations`, `rels_by_lp`, `core_rows`, `core_cols`, `fb_cols`, `la_add`,
`la_wied_mul`, `attempts`, `exhausted`, `certified`.

## 6. Correctness gates (all must pass; each shown reachable in PILOT.md)

A run or cell that fails a gate is excluded and the exclusion reported; if a gate fails
on a decision cell the decision is **inconclusive** for that prime.

- **Q1 - certification/recovery.** Every IC run has `certified = true`, `ok = true`
  (`recovered == t_planted`), `exhausted = false`. Reachable: pilot, all IC runs at
  `p in {1031, 8209}` passed.
- **Q2 - base-size identity.** Every IC run has `fb_cols == ceil(cc * p^(2/3))`.
  Reachable: pilot, `103/205` at 1031, `407/814` at 8209, exact.
- **Q3 - pairing integrity.** For each prime, the multiset of `curve_seed` values is
  identical across all `cc` invocations and across the A/A seed-groups (same deterministic
  curve search, `--max-h 8 --seed-search 600 --curves 3`). Reachable: pilot, curve_seeds
  identical across `cc = 1` and `cc = 2` at 1031.
- **Q4 - instrument fidelity (count-identity with the lab's own record).** The
  `cc = 2.0`, `p = 8209`, first curve, `seed = 1` run reproduces the lab `scale_g3`
  cell's `trials = 42975` exactly (L03 ASSESSMENT cites lab `(8209, c6, s1): 42975`).
  Reachable: pilot reproduced `trials = 42975`. (This anchors our binary to the lab
  instrument without any port.)
- **Q5 - rho independence from `cc`.** `R_rho_neg` counters are byte-identical across
  `cc` invocations on the same `(p, curve_seed, seed)` (rho ignores `nb`), and every
  rho run certifies. Reachable: pilot, rho `cantor_mul` identical across `cc = 1`/`2`
  at matched keys at 1031 (e.g. cseed 8 seed 1: `10714836` both).
- **Q6 - relation count-once and rank stop.** The instrument deduplicates relations by
  its DSU over large primes and counts each relation row once; the stop rule is a
  rank/solve condition, not a relation count: `certified = true` means the peeled
  system reached full rank on the `t` column and the planted `t` was recovered, which
  is the rank check **before** the cost is read. We re-implement no linear algebra; we
  cite the instrument's. Report `attempts`, `relations`, `core_rows`, `core_cols`.
  Reachable: pilot, `attempts` = 1 and all runs certified at 1031 and 8209.
- **Q7 - noise floor estimable.** The A/A arm yields a non-degenerate per-pair ratio
  distribution (seed-to-seed `la_mul` CV > 0 at `cc = 2`). Reachable: pilot, `cc = 2`
  `la_mul` differs across seeds at 1031 (`415657` vs `409798`, cseed 8).

No threshold in Q1-Q7 was set below a value seen on the pilot; none is a rate gate
calibrated at a smaller size than the study uses (certification/non-exhaustion were
checked at `p = 8209` and at `p = 32771`, the study's own sizes; see PILOT.md budget).

## 7. Decision rule (frozen, numeric; applied by the assessor as written)

Primary reading: band mid, treatment test (§4), point estimate and 95% paired CI.

- **supported** iff **all** hold:
  1. `G(p) >= 0.15` (point estimate) at every `p in {8209, 16411, 32771}`;
  2. the 95% paired CI for `G(p)` excludes 0 at every prime;
  3. at the largest prime (32771) the treatment effect exceeds seed noise: the 95% CI
     for `G(32771)` lies entirely above the 95% CI for `|G_AA(32771)|` (the A/A
     interval does not reach the treatment interval).
- **killed** iff `G(32771) < 0.05` (point estimate) **or** the 95% CI for `G(32771)`
  includes 0.
- **inconclusive** otherwise (including any decision cell failing a §6 gate, or
  `|G_AA(p)|`'s 95% upper bound exceeding 0.05 at any prime, which would mean seed
  noise is too large to trust the paired reading).

Thresholds 0.15 and 0.05 are fixed here, before any decision run, and never moved after
data (lesson: never move a threshold after data). **Honest framing** (carried from the
L03 assessment's concern): `G >= 0.15` is a weak bar given L03 measured 36-40%; the
scientific content is the *magnitude* of `G`, the LA-share decomposition (S2), the
interior-optimum shape (S1) and the rho ratio, not the binary verdict. All of these are
reported regardless of the verdict.

## 8. Confounders and isolation

- **C1 synthetic-discount confound (the L03 H1 weakness).** Removed: no `d`. All cost
  is the instrument's native per-trial cost. The claim is purely the measured
  re-tuning.
- **C2 curve/seed variance.** Paired design, common curves and seeds across `cc`
  (Q3); hierarchical bootstrap over curves then seeds.
- **C3 winner's curse / exhaustion.** `cc = 1` vs `cc = 2` are fixed a priori, not
  argmin-selected, so the decision has no selection bias; the secondary-sweep argmin is
  descriptive only. Non-exhaustion required (Q1); `--max-trials 2e8`. If a secondary
  cell (notably `cc = 0.5` at 32771) exhausts, it is recorded `exhausted` and dropped
  from S1, never rerun with other parameters.
- **C4 LA pricing.** Reported at all three bands; `la_add` (the +-1 additions that
  carry matrix weight but are unpriced in FPM) reported as a sensitivity. The direction
  of the effect is pricing-monotone: more columns -> larger core -> more `la_mul`, so a
  mispricing would not reverse the sign (it changes the magnitude only).
- **C5 rho variance.** Rho is high-variance (pilot `cantor_mul` spans `8.8e5..1.8e7`
  across seeds); its ratio is parameter-labelled with a CI and is never part of the
  verdict.
- **C6 solver-seed noise.** A/A arm, §4, and decision clause 3 and the inconclusive
  guard.
- **C7 instrument fidelity.** Q4 count-identity with the lab's recorded `scale_g3`
  cell.

## 9. Exact commands (executor runs these; outputs under raw/)

Binary: `/home/user/crypto-c0b657b/target/release/genus_ladder` (read-only; do not
rebuild). Record its sha256 first. One thread per run; the primes are run in one
invocation per `cc` (the driver loops primes/curves/seeds internally).

```sh
BIN=/home/user/crypto-c0b657b/target/release/genus_ladder
D=/home/user/crypto/research/g2plus_followups_20261117/F1_L03_base_constant_lab_instrument
sha256sum $BIN > $D/raw/binary.sha256

# Baseline + treatment (primary), 3 curves x 4 seeds; rho reference lives in the cc=2 run.
$BIN run --primes 8209,16411,32771 --genera 3 --curves 3 --seeds 4 --arms cR \
    --cc 2.0 --max-h 8 --threads 1 --seed-search 600 --rho-cap 3e7 \
    --max-trials 200000000 --out $D/raw/cc2.jsonl
$BIN run --primes 8209,16411,32771 --genera 3 --curves 3 --seeds 4 --arms c \
    --cc 1.0 --max-h 8 --threads 1 --seed-search 600 \
    --max-trials 200000000 --out $D/raw/cc1.jsonl

# Secondary sweep (S1), 3 curves x 2 seeds, arm c only, one invocation per cc.
for cc in 0.5 1.5 3.0; do
  $BIN run --primes 8209,16411,32771 --genera 3 --curves 3 --seeds 2 --arms c \
      --cc $cc --max-h 8 --threads 1 --seed-search 600 \
      --max-trials 200000000 --out $D/raw/cc${cc}.jsonl
done

sha256sum $D/raw/cc*.jsonl > $D/raw/data.sha256
python3 $D/analyze.py $D/raw/cc1.jsonl $D/raw/cc2.jsonl \
    --secondary $D/raw/cc0.5.jsonl $D/raw/cc1.5.jsonl $D/raw/cc3.0.jsonl \
    --out $D/analysis/analysis.json --tables $D/analysis/tables.md 2>&1 \
    | tee $D/raw/analyze.log
python3 $D/analyze.py --selftest   # Q-unit, must print OK before the line above is trusted
```

Budget note (PILOT.md §budget): the budget-critical cell is `cc` small at
`p = 32771` (small base -> many trials). The `cc = 0.5` cells at 32771 may hit
`--max-trials`; they are recorded `exhausted` and excluded from S1 only. If any
**primary** (`cc in {1, 2}`) cell at 32771 approaches 15 CPU-minutes or exhausts, the
executor stops and reports it; the decision then rests on the primes that completed,
with 32771 reported as "not completed within budget" (distance remaining), never
rerun with moved parameters.

## 10. Outputs and what is NOT done here

Outputs: `raw/cc*.jsonl` (one config line + one run/skip line per arm/curve/seed),
`raw/*.sha256`, `raw/analyze.log`, `analysis/analysis.json`, `analysis/tables.md`
(the verdict line). Each file kept small (per-run rows only; no per-relation dumps).

Not done by this follow-up (main session's responsibility per the hard rules): resume
checkpoint, bench record, session log, git commits, any leaderboard/benchmark row,
any §5a GAE validation band or rho ordering claim. This follow-up does not run the
decision experiment; the coordinator freezes PROTOCOL.md, SPEC.md, PILOT.md and
`frozen.sha256` only.
