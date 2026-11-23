---
title: "Where the counted cost S of m = 3 rootfind comes from, and three leads to lower it"
date: 2026-11-15
---

# Cost leads for m = 3 rootfind-subtract (desk study, no experiments, no code)

Standing: every number below is `[pcc:unverified]`. The measured figures are recomputed from the committed
sessions of `research/ic_gpu_rootfind_la_20261115` by the session that wrote this note; no validator has read
this note. The model figures are a back-of-envelope fit, not measurements. Nothing here is a result, a row or
a claim of improvement; each lead is a prediction to test.

## Sources checked

- Knowledge base: `scripts/research-kb status` exits 127 ("no executable found"; there is no
  `crypto-autoresearcher` checkout in this container). Recorded as a gap; no `source_id` was read.
- Lead atlas: `tools/lead_atlas.py` is absent from `crypto` and the sibling `crypto-autoresearcher` checkout
  (`knowledge/leads/ecdlp/`) is absent. Recorded as a gap. So "not tried" below means "not found in the
  repository files I read", not "absent from the atlas".
- `crypto:research/ic_gpu_rootfind_la_20261115/REPORT.md`, `RESULTS.md`, `results.json`, `specs/rf3la.json`,
  `raw/out_rla.tgz` (member `sessions/rf3la/records.jsonl`, 132 records; the per-phase counters below are
  read from it).
- `crypto:research/ic_la_m3_20261115/REPORT.md` (S / rho 52 to 1,230 at w = 0, 34 to 38 bits).
- `crypto:src/cryptanalysis/ic_framework/plugins.rs` (`RootfindSubtractOracle::decompose`, lines 3368 to 3430:
  the sweep stops at the first hit; `prime-abscissa` `size=auto`, lines 160 to 170, default
  `scale = 0.5`, so `|F_x| = 0.5 r^(1/3)`; `glv-orbit` base, line 19).
- `crypto:src/cryptanalysis/ic_boundary.rs` (`PairTable::build_negation_folded`, line 8636: keys are the sum's
  abscissa, `or_insert` drops a repeated key; `KSumTable::keep_collisions`, line 9061).
- `crypto:src/cryptanalysis/ic_framework/rootfind.rs` (module comment: charged and uncharged work).
- `crypto:research/ic_free_relation_census_20261011/RESULTS.md` (Koblitz fixtures: free relations cover 49 of
  599 rank at n = 61 and almost none at n = 73, 83; rank is one third of the colliding rows).
- `crypto:research/ic_m4_mitm_scaling_20261031/README.md`, `research/ic_m3_cycle_n37_20261101/REPORT.md`,
  `research/ic_largeprime_multitarget_20261102/README.md` (large primes are a generic k-sum, bounded by Shoup),
  `research/ic_arity_sweep_20261027/README.md` (no arity 2 to 9 crosses rho).
- `crypto:research/prime_rho_automorphism_20261015/README.md` (`rho.cubic` / `rho.negation` = 0.647 after the
  erratum), `src/cryptanalysis/ic_framework/cm_orbit.rs` (a scalar-orbit control, not credited to the DLP search).
- `crypto:docs/ic/SUCCESS_CRITERIA.md` (cold end-to-end S, matched rho, §5a band).
- Searched and found nothing: `grep` of `research/ic_*` and `docs/ic` for a partial-sweep, sweep-fraction or
  sweep-window parameter; `size=auto` or a `scale` sweep in any m = 3 rootfind study (the studies use fixed
  2048 and 4096); a prime-field m = 3 rootfind arm with an automorphism-folded pair table.

## Where S comes from

Setting: `rootfind-subtract:m=3`, `prime-abscissa:size=a`, `walk-prime-batch-1024`, `incremental-gauss`,
prime_explicit curves of 37.4 and 39.3 bits, 6 CPU records per cell (3 targets, 2 rounds). S is the counted
group-addition total at w = 0. Means over the 6 records. [pcc:unverified]

| log2 r | a = \|F_x\| | S (GAE) | pair-table build | sweep (relation phase) | other | trials | rows | adds per row | S / rho |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 37.4 | 2048 | 123.8 M | 4.20 M (3.4%) | 119.6 M (96.6%) | 0.07% | 30,579 | 1,877 | 63,661 | 340 |
| 37.4 | 4096 | 70.6 M | 16.78 M (23.8%) | 53.9 M (76.2%) | 0.1% | 9,475 | 3,753 | 14,330 | 194 |
| 39.3 | 2048 | 474.5 M | 4.20 M (0.9%) | 470.3 M (99.1%) | 0.02% | 116,168 | 1,880 | 250,131 | 737 |
| 39.3 | 4096 | 246.6 M | 16.78 M (6.8%) | 229.8 M (93.2%) | 0.03% | 30,882 | 3,760 | 61,083 | 383 |

rho.negation: 364 k GAE (37.4) and 643 k GAE (39.3), 6 records each. "Other" is the target set-up
(2,080 scalar multiplications and about 74 k doublings a run, the walk) and verification (48 GAE). Linear
algebra is 0 GAE at w = 0 (its `row_ops`, 42 to 48 M at a = 4096 with echelon, are priced only at w > 0).

What the counters say:

1. **The pair-table build is exactly a(a+1) additions** (16,781,312 at a = 4096). It is a fixed cost, 3% to 24%
   of S here.
2. **The sweep is the rest.** Each trial subtracts the signed base points from the target, T - P_i, one
   charged addition each, and probes the table. Trials end at the first hit, so a trial averages 3,908 (a =
   2048, 37.4 bits) to 7,438 (a = 4096, 39.3 bits) additions, below the 2a = 4,096 or 8,192 of a full sweep.
   Lookups equal sweep additions (63,624 lookups against 63,661 additions per row at 2048), so the lookups,
   uncharged at w = 0, double the sweep at w = 1.
3. **The yield is a three-way birthday count and the data fit it.** A target decomposes with expected count
   mu = (2a)^3 / (6 r) (a signed triple of base points). Measured rows per trial against 1 - exp(-mu):
   0.0614 / 0.0617 (37.4, 2048), 0.3961 / 0.3993 (37.4, 4096), 0.0162 / 0.0163 (39.3, 2048), 0.1218 / 0.1231
   (39.3, 4096). So a lookup hits with probability about 3 mu / (2a) = 2 a^2 / r.
4. **Rows needed: 0.917 a.** Measured rank at the stop is 0.916 to 0.918 of a in all four cells. That matches
   the 3-XORSAT threshold of 0.918 for rows of three base columns, so it is not a tuning slack.
5. **Model.** S(a) = a(a+1) + 0.917 a x (adds per row), adds per row = (sweep adds per trial) / P(hit), with the
   first-hit stop. It reproduces the four S values within 1.5% (model 123.1, 70.0, 470.9 and 243.5 M against
   123.8, 70.6, 474.5 and 246.6 M). Full-sweep optimum: a about 5,500 at 37.4 bits (model 63.5 M) and about
   8,700 at 39.3 bits (156.7 M); S grows as r^(2/3), as for any m = 3 pair-table method, against rho's r^(1/2).
6. **The waste.** A decomposition {i, j, k} is found three times in a full sweep, once from each of its
   summands (the table holds all pairs). A hit stops the trial, so the other two finds are not used, and
   trials without a decomposition (60% at a = 4096, 37.4 bits; 84% at 2048, 39.3 bits) always run the whole
   2a additions. Per addition, the observed yield is 6.9e-5 rows (37.4, 4096) against the lookup hit
   probability 2 a^2 / r = 1.87e-4: a factor 2.7 is not used.
7. **Calibration line (not a lead).** The same table and sweep with base points of known scalar (random
   [x_i]G) is baby-step giant-step: one hit gives the logarithm, S = a^2 + 1.5 r / a^2, minimal near
   a = (1.5 r)^(1/4) = 721 at 37.4 bits, about 1.0 M GAE, about 2.9 times rho. So the whole 194x gap of the
   index-calculus arm is the price of unknown base logarithms: 0.917 a rows at 5.3 k to 14 k additions each.
   That hybrid is not index calculus, and the success criterion does not count relabelling, so it is not
   proposed. It says that only changes to the per-row cost or the row count are real levers.

## Lead 1: sweep a rotating fraction of the points per trial

- **Mechanism.** Per trial, probe only T - P_i for a window of the 2a signed points (fraction f), chosen
  by a rotating or seeded-shuffle offset, then take a fresh target. A decomposition has three chances to fall
  in the window, and no trial pays for the two redundant finds or for the long tail of a hopeless sweep.
- **Why it lowers S.** Yield per addition rises from 1.1 mu / (2a) to 3 mu / (2a) as f falls (the lookup hit
  probability, 2 a^2 / r), at the price of one target set-up addition per trial (the walk takes about one
  addition a step, so f >= 1/32 keeps it under 2% of a trial's additions). Prediction from the fitted model,
  a unchanged, S as a fraction of the measured full-sweep S:

  | log2 r | a | f = 1/2 | 1/4 | 1/8 | 1/16 | S at f = 1/16 | S / rho at f = 1/16 |
  |--:|--:|--:|--:|--:|--:|--:|--:|
  | 37.4 | 2048 | 0.59 | 0.46 | 0.41 | 0.38 | 47 M | 130 |
  | 37.4 | 4096 | 0.71 | 0.61 | 0.57 | 0.55 | 38 M | 105 |
  | 39.3 | 2048 | 0.58 | 0.44 | 0.39 | 0.36 | 171 M | 266 |
  | 39.3 | 4096 | 0.61 | 0.48 | 0.43 | 0.41 | 100 M | 156 |

  With a re-tuned to its new optimum (3,500 at 37.4 bits, 5,600 at 39.3 bits) S is about 37 M and 92 M, so
  S / rho about 103 and 143 (from 194 and 383). Lookups, which are unpriced at w = 0 and priced at w = 1,
  fall with the sweep additions, so the order should hold across the §5a band. Linear algebra is untouched.
- **What changes in the code.** `RootfindSubtractOracle::decompose` (`plugins.rs`, about line 3368) loops
  over `reps` from index 0 to the first hit. Add a parameter (for example `sweep=1/16`, part of the method
  identity) and start the loop at `offset = (trial counter x window) mod len`, wrap, and stop after
  `ceil(f x len)` abscissae. Charges, hit confirmation and the batched inversion are unchanged; the batch
  inversion covers the window only. `rootfind-subtract-gpu` has to take the same window (its probe mode
  reports every hit, so the host side confirms in window order). Rows are not biased: with a rotating window
  every column is equally often the swept one.
- **Cheapest local test.** One `ecbench` spec, 4-core VM, `--cpus none --allow-busy` is enough for counts
  (S is deterministic): two prime_explicit curves of 34 and 36 bits (from `research/ic_la_m3_20261115/specs/la.json`),
  3 targets, 2 rounds, a in {2048, 4096}, `linalg=incremental-gauss-echelon`, arms `sweep` in {1, 1/2, 1/4, 1/8,
  1/16}, with `rho.negation` and its A/A copy as the matched reference. Per record about 1 to 4 s of CPU, so
  about 120 records is 5 to 15 minutes on one thread, 4 in parallel. Band: `ecbench band` at w = 0, 0.1, 1.
- **Falsification.** Lead 1 is false if, at a = 4096 and f = 1/8, the mean S is above 0.75 of the f = 1
  arm's on both curves (predicted 0.57 to 0.43), or if rows at the stop exceed 1.05 a (the rotating window
  skews the matrix, predicted 0.917 a), or if the recovered logarithm fails verification. A 36-bit and a
  34-bit curve test the same law as 38 and 40 bits only because mu and the optimum scale with r; the fitted
  model is checked against the new cells before the claim is read, not assumed.
- **Already tried here?** Not found. The first-hit stop and the full sweep are in every study I read
  (`rootfind.rs` module comment: "stopping at the first hit"); no sweep-fraction parameter exists.

## Lead 2: size the base to the S optimum (no code)

- **Mechanism.** The studies fixed a at 2048 and 4096. `size=auto` exists with `scale = 0.5` (a = 0.5 r^(1/3):
  2,820 at 37.4 bits, 4,435 at 39.3 bits) and no round swept it. The fitted model puts the full-sweep optimum
  well above 4096.
- **Why it lowers S.** At 39.3 bits the sweep is 93% of S at a = 4096 and the build only 7%. Model S, full
  sweep: 243.5 M (a = 4096), 177.6 M (6144), 157.5 M (8192): 0.73 and 0.65 of the measured a = 4096 S, S / rho
  about 245 at 8192 (from 383). At 37.4 bits the optimum is flat: 70.0 M (4096), 64.4 M (6144), 80.8 M
  (8192), so at most a 9% gain at 6144. After Lead 1 the optimum moves down (see Lead 1); the two leads
  interact, so test them as a grid, not one at a time.
- **What changes in the code.** Nothing. The spec sets `prime-abscissa:size=6144` and `size=8192`. Memory: the
  pair table is a^2 entries, and the a = 4096 run held 685 MB (`max_rss_kib` 685,076), so 8192 needs about 2.7 GB
  and 6144 about 1.5 GB (extrapolated; the VM has 15 GB).
- **Cheapest local test.** The same spec as Lead 1 with a in {3072, 4096, 6144, 8192} at `sweep=1`, 34 and 36
  bits, with the same rho arms. The a = 8192 build is 67 M additions, so keep the curves at 36 bits or below
  on a 4-core VM (about 10 minutes at 36 bits).
- **Falsification.** False if the S at a = 8192 is above 0.85 of the a = 4096 S at 39.3 bits (predicted 0.65), or
  if the model, refitted on the new cells, misses their S by more than 5%.
- **Already tried here?** Not found for m = 3 rootfind. `research/ic_lead_queue_20261114` (track B) names a
  base-size lead at 20 bits; it is a different oracle and size.

## Lead 3: an automorphism-folded pair table on j = 0 curves

- **Mechanism.** On a prime curve with j = 0 (automorphism group of order w = 6, p = 1 mod 3), fold the base,
  the table keys and the probes by the orbit key min(x, wx, w^2 x), as `rho.cubic` folds its walk. Columns are
  orbits, so rows needed stay 0.917 a; each pair of orbit representatives gives six keys (the rotations of
  the second point) for six additions, and each key stands for six abscissae.
- **Why it lowers S.** Table keys per addition stay 1, but a probe now hits any of the r / 6 orbit classes:
  the lookup hit probability is 6 N / r = 18 a^2 / r against 2 a^2 / r (N = 3 a^2 keys against a^2). Model:
  S = 3 a^2 + 0.917 a x r / (18 a^2) with optimum near a = 0.2 r^(1/3), S about 0.375 r^(2/3), against about
  1.13 r^(2/3) for w = 2: a factor 3 (S proportional to 1/w), while the matched `rho.cubic` gains 0.647 over
  `rho.negation`. So S / rho_cubic should fall by about 0.333 / 0.647 = 0.52 at fixed r, before Lead 1.
  Canonicalisation (2 multiplications a probe) is unpriced at w = 0 and priced at w > 0, so check the band.
- **What changes in the code.** A new pair-table build and a sweep over orbit representatives in
  `RootfindSubtractOracle` (keys `min(x, wx, w^2 x)`, rotation stored with the entry so a hit is re-added
  and checked in the group, as `lookup` does), with the `glv-orbit` base (`plugins.rs` line 19), and the
  relation row scaled by kappa^d as in the WWalk `aut=3` option (`plugins.rs` line 2690). Orbit-order bookkeeping
  is the main risk. The gap statement must use `rho.cubic` as its matched reference.
- **Cheapest local test.** `ecbench` spec on three j = 0 prime curves of 34 to 36 bits (p = 1 mod 3, 3 | r - 1,
  as `research/prime_rho_automorphism_20261015` builds them), arms: the new oracle at a in {512, 1024, 2048},
  the unfolded `rootfind-subtract:m=3` with `prime-abscissa` at 2048 and 4096, `rho.negation`, `rho.cubic`, and
  an A/A copy. Counts only, about 10 to 20 minutes.
- **Falsification.** False if S of the folded arm is above 0.6 of the unfolded arm's best S on the same curves
  (predicted 0.33), or if any recovered logarithm fails verification, or if the unpriced canonicalisations make
  the band order flip at w = 1.
- **Already tried here?** Not found for m = 3 rootfind. The orbit machinery exists for other oracles
  (`cm_orbit.rs`, `glv-orbit`, `rho.cubic`, WWalk `aut=3`); whether `glv-orbit` runs under rootfind-subtract
  is not established by my reading. This lead changes the curve family (j = 0 only), so it does not carry to
  generic prime curves.

## Considered and not proposed

- **Free relations from pair-table key collisions** (the table drops repeated keys at build time;
  `keep_collisions` exists for the k-sum table): about a^4 / (3 r) independent rows, 523 at a = 4096 and
  37.4 bits, 14% of the 3,753 rows needed; model gain about 11% at a = 4096 and 6% at the optimum after Lead 1,
  0% to 3% at 39.3 bits. The Koblitz census (`ic_free_relation_census_20261011`) found the same law and the same
  rank factor of one third. Too small to rank above the three leads; cheap to add once Lead 1 is in.
- **Harvesting all hits of a trial.** A decomposition's repeats cost the same additions; harvesting them does
  not change yield per addition unless the sweep is also shortened, which is Lead 1.
- **Four summands or large primes.** `ic_m4_mitm_scaling_20261031` and `ic_largeprime_multitarget_20261102`
  put them in the sparse regime or at the generic k-sum bound; they do not lower the per-row cost here.
- **Linear algebra.** Free at w = 0; at w = 1 echelon's 43 to 48 M `row_ops` at a = 4096 become the largest
  priced item once Lead 1 shrinks the sweep. A separate lead for the w = 1 reading, not for S at w = 0.

## Reading the gap

At a = 4096 the measured S / rho is 194 (37.4 bits) and 383 (39.3 bits). If Leads 1 and 2 hold as predicted,
the model puts S / rho near 103 and 143 at the same sizes: still above 100 times rho, still a gap of a factor
of at least 100, and the exponent difference (r^(2/3) against r^(1/2)) is not touched by any lead here. They
are constant-factor, counted-cost improvements to the index-calculus baseline, to be reported as distance
remaining.

## Skeptic review

Reviewer: a second session, 2026-11-18, reading the committed records and sources only. No experiment was run,
no code changed. Every figure below is `[pcc:unverified]`. Scratch scripts (not committed):
`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/skeptic_t1/m.py`. The research-kb
and lead atlas are unreachable here too, so "already done" below means found by grep and reading, and a
"not found" is as weak as the author's.

### What I recomputed and confirmed

- **S decomposition.** From `research/ic_gpu_rootfind_la_20261115/raw/out_rla.tgz` (member
  `sessions/rf3la/records.jsonl`, arm `rf3-cpu-echelon-F*`, 6 records a cell) I got the same S, pair-table
  build, relation-phase GAE, trials, rows, rows per trial and rank / a as the table above: S 123.8, 70.6,
  474.5 and 246.6 M; build 3.4, 23.8, 0.9 and 6.8%; rows per trial against 1 - exp(-mu) 0.0614 / 0.0617,
  0.3961 / 0.3993, 0.0162 / 0.0163, 0.1218 / 0.1231; rank / a 0.916 to 0.918. [pcc:unverified]
- **Model, out of sample.** I re-implemented the first-hit model independently (`m.py`) and applied it to
  `research/ic_gpu_rootfind_m3_20261115/results.json` (rf3 session, a = 1024 and 2048, 25 to 31 bits, 4 curves,
  which the author did not use). Model / measured S is 0.94 to 0.99 in all 8 cells. The out-of-sample fit
  supports Leads 1 and 2 as a description of the full-sweep arm. [pcc:unverified]
- **Lead 1 and 2 tables.** My code reproduces the f-fraction ratios (0.55 and 0.41 at f = 1/16), the retuned
  optima (a 3,500 / 5,600, S 37.4 M / 92.3 M, S / rho 103 / 143) and the full-sweep optima (5,500 and 8,700).
  [pcc:unverified]
- **Per-trial overhead.** Relations-phase adds minus lookups is 50 k a run at a = 4096 (about 1 walk
  addition a trial plus a fixed 2,080 scalar multiplications), so the "target set-up is 1 addition a trial"
  assumption holds. [pcc:unverified]

### Problems that apply to all three leads

1. **The baseline is not named, and it is not the best IC arm.** AGENTS.md ("Baseline") says the baseline is the
   best current verified IC pipeline at that curve and size. In this repository the `ic plan` recipe (m = 2
   pair table) is 41 to 47 times rho at 36.0 to 36.25 bits and `res2` is 2.6 to 2.7 times rho
   (`research/ic_planner_exhaustive_20261026/README.md`, table "distance remaining"). A power-law scaling of
   the plan row (r^(1/6)) puts it near 53 at 37.4 bits and 66 at 39.3 bits; my model of the m = 2 table,
   S = a^2 + 0.25 r / a, gives 66 and 92 and matches the 32-bit measurement (`research/ic_oracle_setup_blocked_20261020/analysis/table-A.md`,
   N = 1,024: 33). The leads' end points (103 and 143 for Leads 1 + 2) are still about 2 times worse than that.
   So none of the three leads, even if confirmed, improves the best IC baseline at these sizes. They improve
   the `rootfind-subtract:m=3` arm, a component or arm-level result. State that in the first line of each test
   and report the gap to `plan` as well as to rho. (`res2` is, per that README, a birthday search on Z/r that
   does not use the curve, so it is not a fair IC reference, but `plan` is.) [pcc:unverified]
2. **Exponent.** All three keep S proportional to r^(2/3) (the note says so in "Reading the gap"). None can
   reach T1 or T2 of `docs/ic/SUCCESS_CRITERIA.md`. They are constant-factor results.
3. **The rho denominator is noisy.** The rho.negation S has a coefficient of variation of 0.61 to 0.68 over the
   6 records of a cell (I computed it from the same records.jsonl); the IC arms have 0.01 to 0.03. So the
   "S / rho" figures (194, 383, ...) carry about 25 to 28% one-sigma error from the denominator, and the
   measured rho means are 0.97 (37.4) and 0.87 (39.3) of the theoretical 0.886 sqrt(r). Gains must be stated
   as paired S ratios between IC arms (tight), and the rho column needs a larger ensemble or the theory value
   with the A/A band. As written, "n = 6 rho records" cannot discriminate a 194 to 103 move in S / rho on its own.
   [pcc:unverified]
4. **The A/A copy proposed is not informative for S.** At fixed seeds S is deterministic per arm. The A/A that
   matters is the spread of the new arm over seeds (1 to 3% here), plus an A/A on rho.

### Lead 1 (rotating sweep fraction): keep, with revisions

- **Mechanism check.** `RootfindSubtractOracle::decompose` (`src/cryptanalysis/ic_framework/plugins.rs` about
  line 3631) sweeps every abscissa to the first hit, and `prepare` rejects every parameter except `m`, so no
  window exists. I grepped `src` and `research` for sweep-fraction, window and partial-sweep parameters and
  found none for this oracle. The decomposition-count argument is right: yield per probe is 2 a^2 / r
  (3 mu / (2a)) for any window, and the per-trial overhead is small. Not a relabelling: fewer counted additions
  for the same recovered logarithm [pcc:unverified].
- **Revisions.**
  - At w = 1 the gain is diluted. From `band.json` (echelon, a = 4096, 37.4 bits: S 166.6, 194.6, 446.8 at
    w = 0, 0.1, 1) the priced items at w = 1 are the sweep adds (about 54 M), their lookups (54 M), the
    build adds and inserts (16.8 M each) and the echelon `row_ops` (48 M). Scaling the sweep items by 0.55 and
    keeping the build and row ops gives a w = 1 ratio near 0.67, not 0.55 (my arithmetic from the record
    counters, not a model run). The order still holds, but the note's "S falls with the sweep" at w = 1 is too
    strong, and linear algebra becomes about 38% of the w = 1 cost. Use the echelon arm as the baseline;
    `incremental-gauss` has 361 M row_ops and would hide the sweep.
  - The batched-inversion cost per trial is priced at w > 0 (the inversion count rises about four times at
    f = 1/16: 9.5 k to about 39 k trials). Pre-register it in the falsification list.
  - Falsification as written is good (f = 1/8 above 0.75 at a = 4096). Add the row-yield check: rows per
    trial must equal 1 - exp(-mu (1 - (1 - f)^3)) within 5%; a miss means the window biases the matrix.
  - The 34- and 36-bit curves are fine because the law scales with a / r^(1/3); give the a values scaled that
    way (see Lead 2) so the grid is not run at sizes whose optimum differs.
  - On the GPU path the lanes process 1024 targets in lockstep; a window changes the kernel launch shape. S is
    unaffected, wall time is not claimed, so say that the test is CPU-only and counted.
- **Rank.** 1 (largest predicted S effect on this arm, no new data structure, deterministic and cheap).

### Lead 2 (size the base): revise

- **Evidence it is not already done.** `size=` in all m = 3 rootfind specs is 1024, 2048, 4096 only (grep of
  `research/ic_gpu_rootfind_*` and `ic_la_m3_20261115`). Larger bases (8k, 16k) exist only for the m = 2
  pair table (`ic_oracle_setup_blocked_20261020`, `ic_linalg_pipeline_20261019`) and rootfind m = 2 (16,384 in
  `ic_gpu_rootfind_ic_20261114`). The out-of-sample fit above gives some confidence in the 0.65 prediction.
- **Defect: the test does not match the falsifier.** The falsifier is stated at 39.3 bits (a = 8192 against
  4096), but the "cheapest test" runs 34- and 36-bit curves "with a in {3072, ..., 8192}". The optimum scales as
  0.98 r^(1/3) (5,500 / 5,640 at 37.4 and 8,700 / 8,870 at 39.3 in the author's own numbers), so at 36 bits it
  is near 4,000 and a = 8192 is two times past it: that test would show a loss and refute the lead for the
  wrong reason. Either run the two existing curves of `rf3la.json` (a = 6144 and 8192 cost about 4 to 10
  minutes a record on one CPU thread, by my scaling of the a = 4096 wall times, at 1.5 to 2.4 GiB RSS), or run
  36-bit curves at a in {1536, 2048, 3072, 4096} and compare against the model at the same a / r^(1/3).
- **Also.** The 37.4-bit gain is at most 9%, so only the 39.3-bit cell is a real test; the note should say the
  lead is a tuning result (engineering), not an advance. After Lead 1 the optimum moves down (3,500 / 5,600),
  so run Lead 2 as a grid with Lead 1, as the author already says, and do not claim Lead 2 separately.
- **Rank.** 3 (a tuning knob, valid only as part of the Lead 1 grid).

### Lead 3 (orbit-folded pair table on j = 0): revise, with a prior-art correction

- **Prior art the note missed.** `src/cryptanalysis/orbit_pair_table.rs` already implements the pair table over
  orbit representatives (key x^3 on j = 0) with an oracle that takes `m` and probes once per R - P at m = 3;
  `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` section 8.6 (E12) measured it on F_p
  j = 0 at 12.9 to 26 bits (table / 2.97, build / 2.77, probe 1.005 times, S / 1.18 on mean, because the table is
  26% of S there), and section 6.7 (E7) measured the m = 3 fold on j = 0 (relations to full rank ratio about
  1.4 to 7.7, mean near 3.5). The planner records the 2-summand outcome as `E_GLV` in
  `src/cryptanalysis/ic_framework/planner.rs` ("S moves only 1.00 to 1.13"). The note classes these as
  engineering and accounting. So "not found" is wrong, "new code in the rootfind oracle" is partly wrong (the
  table exists, but it is not an `ic_framework` plugin: grep finds no registration), and the lead must cite and
  reuse them.
- **Cheaper first test, no new oracle.** `glv-orbit` returns a `FactorBase` with `col_of` and `coef_of`, and
  `rootfind-subtract` reads only `fb.points`, `x_index` and `PairTable::build_negation_folded`, so
  `factor_base=glv-orbit:size=...` with `oracle=rootfind-subtract:m=3` appears to run as it stands (by reading,
  not tested). It keeps the unfolded table (about 9 a^2 additions for a orbit columns) but already divides the
  rows needed and the number of targets by 3 at the same hit rate. By my algebra, at the same point count N
  the unfolded arm costs exactly 3 times the fully folded one, S_u = 3 S_f, and the folded column base alone
  gives most of it when the sweep dominates (as here, 76 to 99%). Run that arm first; the orbit table then adds
  only the table share (divide 9 a^2 by 3). [pcc:unverified]
- **Baseline and accounting trap.** The honest comparison is S_f against S_u, both optimised in a over the same
  curve, with `no_fold=1` as the control (the plugin has that flag). The GLV note's own rule is that a fold
  that moves only the relation count with its floor is accounting. The lead survives that rule only if S itself
  falls at the best a of each arm, which my algebra predicts (1/3) and the note's 0.52 does not state clearly:
  0.52 = (1/3) / 0.647 is S_f / rho.cubic against the generic S_u / rho.negation. Against the same-curve
  unfolded arm and the same rho.cubic the ratio is 1/3. Report S_f / S_u, and S_f / rho.cubic as distance
  remaining (about 0.515 x 194 = 100 at a comparable a before Lead 1; about 50 at the ideal 0.375 r^(2/3)
  at 37.4 bits). [pcc:unverified]
- **Curves.** `research/prime_rho_automorphism_20261015` builds crater curves with composite order and subgroup
  r of 14 to 24 bits; it does not supply prime-order j = 0 curves of 34 to 36 bits. They need a CM
  construction (4p = t^2 + 3 v^2, prime r with 3 | r - 1). `rho.cubic / rho.negation = 0.647 [0.543, 0.770]` is
  from 22 to 28-bit p and its class set-up is unpriced in ecbench, so its interval (0.33 / 0.647 gives
  0.43 to 0.61) is wide and the verdict must use §5a pricing for both arms (canonicalisation, 2 multiplications
  a probe for x^3, is unpriced at w = 0 and cheap against an addition at w = 1).
- **Scope.** j = 0 only; the tier rules count transfers only when fully charged. A j = 0 gain is a family
  result and does not move the generic prime baseline.
- **Rank.** 2 (largest possible gain, 3 times, but a family-limited result with a missing curve set and prior
  measurements classed as engineering). Test it after the no-code `glv-orbit` run.

### Ranked order

1. Lead 1, as a grid with Lead 2 (sweep fraction x size), on the echelon arm, with the revisions above.
2. Lead 3, starting with `glv-orbit` + `rootfind-subtract:m=3` + `no_fold=1` control on generated 34 to 36-bit
   prime-order j = 0 curves, then the E12 orbit table.
3. Lead 2 alone: fold into 1; its own claim is a tuning gain, and its test must be re-scaled.

Net: the S model is sound (out-of-sample 0.94 to 0.99), the mechanisms are real, but the three leads together
cannot take this arm below the existing `plan` baseline at these sizes (their best case, about 100 to 140 at
37.4 to 39.3 bits against about 55 to 70 for `plan`), and no lead changes the exponent. Report them as
arm-level constant-factor results, state the `plan` gap beside the rho gap, and do not describe them as an
improvement of the best index-calculus baseline. [pcc:unverified]
