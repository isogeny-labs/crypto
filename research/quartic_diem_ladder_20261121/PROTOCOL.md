# Ladder for lead #1: Diem 2LP index calculus on prime-order plane quartics against matched rho

Status: FROZEN before any measured run (sha256 of this file and `analyze.py` in
`frozen.sha256`). Every number this study produces is `[pcc:unverified]`. A row that reads
"below rho" here is a **candidate**, never a leaderboard or `benchmarks/` row, until it passes
the validation gate (replay certificate, validator report, VALIDATION.json).

Parents: `research/quartic_diem_precheck2_20261121/` (PROCEED-TO-LADDER; instrument arm
`2lp-new`), `research/quartic_diem_precheck_20261120/`,
`research/g2plus_leads_20261116/L10_c34_picard_matched_rho/ASSESSMENT.md` and
`problems/quartic-matched-rho-overpricing.md` (the lab's quartic negation rho is overpriced
about 1.73x). The owner approved step A (sampled factor base) and step B (this ladder) on
2026-11-21.

## 1. Questions

- **Q-A (step A).** On the same curves and solver seeds, how much does building the factor base
  by sampling (no enumeration of all rational points) change the counted cost of the pre-check-2
  instrument, and how does that change scale with `p`?
- **Q-B (step B).** For both arms, what is IC/rho at each prime of the ladder, priced at
  `w = 0, 0.1, 1` GAE per native operation (AGENTS.md §5a), against the matched rho reference of
  §4? What are the fitted exponents, and where (if anywhere) is IC below rho at every weight?

## 2. Instrument (built and tested before this protocol; development data only)

Source: `src/cryptanalysis/quartic_ladder.rs` and `src/bin/genus_ladder.rs` on top of
`2190b15c4c` (the pre-check-2 commit; the pre-check-2 working-tree hashes are its
`source.sha256`). Working-tree hashes at freeze: `source.sha256`. Binary
`genus_ladder.qsfb` (sha256 in `binary.sha256`), release profile, separate `CARGO_TARGET_DIR`.

- **`2lp-new`** (`Q_diem_2lp_new`): unchanged pre-check-2 PRIMARY. Enumerate all rational
  points (`O(p log p)`, `rel_enum`), shuffled draw of `nb = ceil(3 sqrt p)` base points,
  algebraic descent, linked solve (4 rows), T kick, scalar Wiedemann on `AᵀDA`, certificate
  `[t]G = T`.
- **`2lp-sfb`** (`Q_diem_2lp_sfb`, `QFactorBase::Sampled`): identical except the set-up. No
  enumeration: x-coordinates `y` of the chart `[1, y, z]` are drawn in a seeded order without
  repetition (`y_i = alpha i + beta mod p`); for each, `F(1, y, z)` is formed and its rational
  roots found by counted Cantor-Zassenhaus (the same per-`y` work as the enumeration), until `nb`
  points are in the base (counted in `rel_enum`). The point index holds only `P0`, the base and
  the points met so far. A residual point of a line or a descent point that is not in the index
  is checked to lie on the curve (counted, about 30 multiplications), gets its monomial values
  (counted, 6 multiplications) and is inserted; it is a large prime by definition. A descent
  that would fall back to the scan is a failed trial (counted in `decomp_fallbacks`).
- **New counters, both arms** (no effect on the pre-check-2 counts): `idx_lookups` (every
  point-index hash lookup) and `idx_inserts` (the enumerated arm counts one insertion per
  enumerated point). They are native operations and are priced in the band (§5).
- **Tests (pass before freeze, `cargo test --release --lib quartic_ladder`):**
  `sampled_index_same_relations_as_enumerated` (the sampled arm given the enumerated arm's
  base reproduces `2lp-new` exactly: relations, rows by large-prime count, decomposition
  trials, core, attempts, linear algebra, group operations, lookups and the logarithm, at
  `q = 101, 251, 1031, 2053`, 3 seeds); `sampled_membership_matches_enumeration_exhaustive`
  (at `q = 31, 37, 53, 101, 131`: sampling every fibre returns exactly the affine points of the
  enumeration; with a `3 sqrt q` sampled base the classification of every rational point
  — base column / `P0` / large prime — agrees with the enumerated index; every line through two
  base points and the descent of 400 random classes give the same points; every indexed point is
  rational); `sampled_fb_certifies` (`q = 101, 251, 1031`, 4 seeds, relations verified by Pic
  arithmetic on 2). The eleven pre-check-2 tests still pass.

## 3. Curves, seeds and commands

Prime-order (`h = 1`) smooth plane quartics with a rational point, admitted by the unchanged
path `make_instance(q, s, s, 1)`, **5 curves per prime**, solver seeds 1 and 2, both arms on
every (curve, seed), in the order `2lp-new`, `2lp-sfb`.

- `p = 1031 ... 65537`: the 4 curves of pre-check 2 at each prime, then the next untried curve
  seeds in increasing order until a fifth is admitted (lists below). The first 5 admitted are
  used.
- `p = 131101` (the first prime above `2^17`): two processes of one thread; odd seeds
  `1, 3, ..., 799` keep their first 3 admitted curves, even seeds `2, 4, ..., 800` their first
  2. Admission dominates the budget (about 34 s per seed tried, about 2.5% admitted).
- `p = 262147` and above are **not run**: admission alone (about 70 s per seed, about 40 seeds
  per curve) would exceed the budget. This is disclosed, not a result.

Main session (`raw/run.jsonl`, 2 threads):

```sh
genus_ladder.qsfb run --primes 1031,2053,4099,8209,16411,32771,65537 --genera 2 --arms - \
  --curve-seeds 1031=1,2053=1,4099=1,8209=1,16411=1,32771=1,65537=1 \
  --quartic --quartic-arms 2lp-new,2lp-sfb --quartic-inhom 4 \
  --quartic-seeds 1031=54/115/244/290/291..900,2053=43/72/73/105/106..700,4099=35/205/220/257/258..900,8209=10/16/114/165/166..800,16411=93/158/252/366/367..1000,32771=32/163/217/221/222..900,65537=13/57/156/166/59/61/.../799 \
  --qc 3.0 --max-h 1 --curves 5 --seeds 2 --rho-cap 2e8 --threads 2 --out raw/run.jsonl
```

(`a..b` is written out in full on the command line; `65537` continues the odd list after 57,
since pre-check 2 tried odd seeds to 57 and even seeds to 166.) `--rho-cap 2e8` runs the
measured negation rho `Q_rho_neg` exactly where `sqrt(pi r / 4) <= 2e6`, that is at
`p <= 16411`, on every (curve, seed).

Then (`raw/run131101_odd.jsonl`, `raw/run131101_even.jsonl`, one thread each):

```sh
genus_ladder.qsfb run --primes 131101 --genera 2 --arms - --curve-seeds 131101=1 \
  --quartic --quartic-arms 2lp-new,2lp-sfb --quartic-inhom 4 \
  --quartic-seeds 131101=<1/3/.../799 | 2/4/.../800> --qc 3.0 --max-h 1 \
  --curves <3 | 2> --seeds 2 --rho-cap 2e8 --threads 1 --out raw/run131101_<odd|even>.jsonl
```

If a 131101 process has not finished its search after 110 minutes of wall time, it is stopped
and rerun once with `--quartic-seeds` set to the curves it admitted so far (its `instance`
records), so that the runs complete. A prime with fewer than 2 admitted curves is dropped and
disclosed.

## 4. The rho reference (stated exactly)

Generic plane quartics have no automorphism beyond `-1` acting on `Pic^0`, so the matched
walk is negation rho, `gamma = 2` (the `<rho>` class walk of L10 applies to Picard curves only).

- **Primary reference, every prime:** the ideal negation rho `R = sqrt(pi r / 4)` group
  additions on `Pic^0`, with **nothing** unpriced charged to rho (canonicalisation, hashing and
  distinguished-point storage are free). In GAE it is the same at every `w`. This is the
  L10-corrected reference: it does not carry the `~1.73x` negation overhead of the lab's
  executed walk. It favours rho.
- **FPM view (secondary):** `R_FPM = R * A`, where `A = sum(cantor_mul + 10 cantor_inv) /
  sum(group_ops)` over the IC runs of the same curve (the measured FPM cost of one Pic operation
  in the same counters). IC is FPM mid = `cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul`.
- **Calibration (p <= 16411):** the executed `Q_rho_neg` on every (curve, seed). Reported:
  steps / `R`, its FPM / `R_FPM` (L10 predicts about 1.73 times the step-count ratio), and IC
  over measured rho as a sensitivity. Not used for the verdict.

## 5. Metrics and pricing

Per run, counted units only (wall time recorded, never used):

- **Primary (§5a band):** `GAE_w = group_ops + w * N_all`, `N_all = rel_mul + rel_add + la_mul +
  la_add + idx_lookups + idx_inserts` (every counted native operation, additions and hash
  operations included), `w in {0, 0.1, 1}`.
- Sensitivity: `GAE_w(mul) = group_ops + w (rel_mul + la_mul)` (the pre-check definition); FPM
  mid against `R_FPM`; a **count-based** calibrated weight `w_cnt = 1 / A` (one native operation
  priced as one field multiplication, a Pic operation as `A`) at `w_cnt` and `10 w_cnt`. The
  last is labelled count-based, not a timing calibration.
- Descriptors: `rel_enum_mul` and its share, `fb_samples`, `idx_points`, `relations`,
  `attempts`, `decomp_fallbacks`, `la_skipped`, `fb_cols`.

Only runs with `certified = true` and `ok = true` enter. A prime with fewer than 2 certified
curves in an arm is dropped from that arm and disclosed.

## 6. Analysis (fixed; `analyze.py`)

- **IC/rho.** Per run `GAE_w / R`; curve level = geometric mean over the 2 seeds; per prime =
  geometric mean over curves, 95% percentile interval of 4000 cluster-bootstrap replicates
  resampling curves within the prime (numpy `default_rng(20261122)`, one generator in script
  order).
- **Exponents.** OLS slope of `ln metric` on `ln p` at curve level (FPM mid, `rel_mul`,
  `rel_enum_mul`, `la_mul`, `N_all`, `GAE_w`), and of `ln(IC/rho)`, with the same cluster
  bootstrap. Sensitivity: the fit without the top prime. The reference has slope 1.5 by
  construction (`r ~ p^3`).
- **Per-component exponents** (validator F9/F11 of pre-check 2): `group_ops`,
  `decomp_trials`, `group_ops / decomp_trials`, `relations`, `attempts`, `idx_points`, besides
  the totals, so that a slope diluted by fixed per-trial Pic work is visible.
- **Extrapolated crossover** `p*` where the fitted `IC/rho` reaches 1, for every `w` whose
  ratio slope is negative AND whose implied IC exponent (ratio slope + 1.5) is at least 1. A
  sub-1 IC slope is never extrapolated (validator F11); the crossover is then withheld and
  flagged. Reported with the bootstrap interval of `log2 p*`, as a prediction only.
- Every pooled ratio is reported with its `w = 0` reading beside it (validator F9).
- **Step A (paired).** For each (curve, seed) certified in both arms, `sfb / new` for FPM mid,
  `rel_mul`, `rel_enum_mul`, `la_mul`, `N_all`, `GAE_w`, `relations`, `attempts`,
  `decomp_trials`, `group_ops`; curve geometric mean; per-prime geometric mean with cluster
  bootstrap; pooled; slope of the ln ratio on `ln p`.

## 6b. Record content and binding (validator findings on pre-check 2)

- **F3.** With `2lp-sfb` among the arms, every `run` row carries the instance in full (`f`,
  `p0`, `order`, `r`, `h`, `t_secret`, and the classes `G` and `T` as printed by the
  instrument) and the `recovered` logarithm (IC and rho rows), so a certificate `[t]G = T` can
  be re-checked independently of the booleans. (This required a second build of the binary
  before freeze; the hash in `binary.sha256` is of that build.)
- **F1.** This session does not commit. Binding is by `frozen.sha256` (this file and
  `analyze.py`), `source.sha256` and `binary.sha256`, written before the measured runs; the
  hash of this file is copied into a line of `RESULTS.md`. Development data are kept in
  `dev/` of this study (curves at `p = 4093, 8191`, not grid primes, not in any fit).
- The pre-check-2 validator found the T kick alone not significant; the linked policy carries
  the retry cut. Both arms here keep the pre-check-2 settings (kick and linked policy) unchanged.

## 7. Predictions (written before the data, from pre-check 2's numbers)

- P1: `rel_enum_mul(sfb) / rel_enum_mul(new) < 0.05` at 65537 and above (about `3 sqrt p`
  fibres instead of `p`).
- P2: paired `rel_mul` sfb/new at 65537 in `[0.15, 0.45]`.
- P3: pooled paired `la_mul` sfb/new inside `[0.67, 1.5]` (no systematic LA change).
- P4: the FPM-mid exponent of sfb is below that of new.
- P5: IC/rho < 0.01 at `w = 0` at every prime in both arms; IC/rho > 1 at `w = 1` at every prime
  in both arms (the all-native band charges every native addition 1 GAE, and `la_add` is the
  largest counter).
- P6: the slope of `ln(IC/rho)` is negative at every `w` (IC exponent about 1.1 to 1.2 against
  1.5).

## 8. Frozen decision rules

- **Per (arm, prime), primary band:** BELOW-RHO CANDIDATE iff the upper 95% bound of IC/rho is
  below 1 at all three of `w = 0, 0.1, 1`. ABOVE RHO iff the lower 95% bound exceeds 1 at all
  three. Otherwise UNRESOLVED (the order changes inside the band; §5a forbids a ranking). The
  sensitivity readings (mul-only, FPM, count-calibrated) are reported beside it and never change
  it.
- **Gap closing at `w`:** the upper 95% bound of the slope of `ln(IC/rho)` is below 0.
- **Step A accepted** as the lead-#1 instrument iff all `2lp-sfb` runs certify, the three
  tests pass, and the upper 95% bound of the pooled paired FPM mid sfb/new is below 1.
  **Step A rejected** if any `2lp-sfb` run fails to certify or returns a wrong logarithm.
- **Lead #1 STOP** if the upper 95% bound of the FPM-mid exponent of the accepted arm is at
  least 1.40 (the pre-check K2 threshold).
- A BELOW-RHO CANDIDATE is reported as a candidate for the validation gate. It is not a
  leaderboard row, not a `benchmarks/` row, and not a tier claim (it is planted-target, warm,
  one session, no replay certificate; docs/ic/SUCCESS_CRITERIA.md §1 to §3).

## 9. Budget (fixed)

At most 2 threads, about 3 hours of wall time for the measured runs (searches included). The
genus-4 search (`research/genus_ladder_scaling_20261119`) and the validator of
`research/quartic_diem_precheck2_20261121` own the rest of the host and are not touched. If the
session is cut, complete primes are analysed and missing cells listed. Nothing is rerun with
other parameters under this protocol.

## 10. What this does not do

- No validation gate, no replay certificate, no cold public target (targets are planted by the
  admission path). No leaderboard, `benchmarks/` or tier claim.
- The relation phase still visits `Theta(p)` lines and large-prime points (Diem's algorithm is
  `O~(p)` in genus 3); only the `O(p log p)` enumeration is removed.
- LA is scalar Wiedemann on `AᵀDA`; the residual 1-in-5 second attempts of pre-check 2 are not
  fixed here.
- Primes above 131101 are not run (admission cost).
