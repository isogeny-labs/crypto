# L5 protocol: how the frame and seed selectors scale with t, and what they are worth at 255 bits (frozen before data, 2026-11-22)

Wave 7, lead L5. Brief: [`../BRIEF.md`](../BRIEF.md). Phase 1 (design only): this file, the scripts in
[`scripts/`](scripts/), and a smoke run under [`smoke/`](smoke/) ("SMOKE, NOT DATA"). No data has been taken.
Every result sentence that comes out of this protocol will carry [pcc:unverified]. Everything stated at 255 bits
is a **derived extrapolation**, labelled so, never a measurement.

## Question (one line)

How does the coverage gain from choosing the coordinate frame (or isogeny-tower seed) by the log-free coset
selector J fall with the factor-base exponent t, what does that selection cost in group operations, and what
are the gain and the cost on the real Curve25519 at the t an algebraic (m = 2 Semaev) attack would use?

## What is already established (imported, not re-tested)

- `curve25519_frame_search_20261103`: every degree-2 coordinate is a Möbius frame of x; on the algebraic
  track, choosing the translation frame by argmax J (ties: cols, then lowest L) over K = 2000 random frames
  raises exact coverage at unchanged per-target F4 cost. Selected ÷ mean frame at k = 20: 3.38 (t = 4),
  2.84 (t = 5), 1.98 (t = 6), …, about 1.3–1.5 at t = 8 on all five miniatures. "Nothing is extrapolated to
  255 bits" there. Search cost at k = 20, t = 6 about 1.6·10⁶ group operations, against 547–936 for a whole
  rho solve.
- `curve25519_descent_skip_20261114` Part B: the parity of the coset label equals χ(x_M) (1,547/1,547
  points); the one-bit score S1 reaches 91.9% of the full-label choice at t = 5.
- `curve25519_isogeny_seed_20261103`: the same J selector over 2,000 tower seeds gives 1.62–3.24× the mean
  seed at t = 5–7; k = 20, t = 6 seed 1093 covers 528 against seed 0's 172; the K = 2000 search there cost
  1.35·10⁷ group operations; end-to-end the gain was 0.641× (S, w = 0.1) because selected seeds have more
  columns. `curve25519_isogeny_seeds_20261114`: per-seed blind rate range 0.85–4.80 per 10⁹ mul-adds (≈ 5.7×).
- **Distance to rho (imported, not re-measured):** best algebraic arm at k = 20 vs rho-neg 18.3× (w = 0),
  6.9·10⁵× (w = 0.1), 4.4·10⁶× (w = 1) (`curve25519_isogeny_seed_20261103`, ecbench session
  `ECBS1h405f20219036`); generic baseline 7.8× rho at k = 20 (`curve25519_frame_search_20261103`). No rho arm
  is measured in L5; a few-× coverage factor cannot change these orders across the §5a band.

## Mechanism (the hypothesis this protocol tests)

For a translation frame of length 2^t, the base has c ≈ 2^(t−1) columns (on-curve abscissae) and
J = m₁² + m₂² + m₃² + m₀(2m₀+1) + m₄(2m₄+1) − c, where m₀..m₄ count columns in the folded label classes
{0}, {1,7}, {2,6}, {3,5}, {4} (derived from the frame-search J definition; checked exactly against it in PC2).
With class probabilities (1/8, 1/4, 1/4, 1/4, 1/8), E[J] ≈ c²/4 and the gradient of J/c² along the simplex is
constant, so **class-composition fluctuations enter J only at second order** (relative O(1/c) = O(2^−t)), while
the column count enters at first order: CV(J) ≈ 2·CV(c) ≈ 2·√D·2^(−t/2), with D = Var(c)/(2^t ρ(1−ρ)) the
dispersion index of the on-curve count (D ≈ 1 if χ(x³+Ax²+x) behaves like coin flips on an interval).
Selecting the best of K frames then gives, in the Gaussian regime,
**gain − 1 ≈ z_K · 2√D · 2^(−t/2)** (z_2000 = 3.44, z_100 = 2.51), i.e. α = 1/2 in gain − 1 ∝ 2^(−αt).
At the real Curve25519 with t* ≈ 128 this predicts gain − 1 ≈ 3.7·10⁻¹⁹ (K = 2000) — derived, to be tested
only through its ingredients (α, D, curve-size independence) that L5 can measure.

## Curves (verified by `scripts/verify_curves.py`; all four checked in `smoke/verify_curves_all.json`)

| name | p | A | #E | r | twist | role |
|---|---|---|---|---|---|---|
| k20 | 1048573 | 926 | 8·131203 | prime | 4·261881 | frame arm, iso arm, PC |
| k24f | 16777141 = 2²⁴ − 75 | 7842 | 8·2098009 | prime | 4·4192553 | frame arm (the brief's faithful k = 24 member) |
| k24r | 16777213 | 318 | 8·2097349 | prime | 4·4193909 | frame arm; iso arm (the engine builds only family-rule curves) |
| Curve25519 | 2²⁵⁵ − 19 | 486662 | 8r, r = 2²⁵² + 2774…8493 | probable prime (MR, 64 derived bases + 13 small) | 4r′, r′ probable prime | arm R |

Miniature #E by an exact vectorised Legendre sum over all x; Curve25519 #E = 8r from a point of exact order 8r
(> 4√p, unique in the Hasse interval). Cyclic Z/8r is checked by `build()` (frame-search code). Note: the
k = 24 member of the frame-search census was k24r (A = 318), not the brief's k24f; both are run.

## Arms, cells and instrument (exact commands: `scripts/run_all.sh`)

All on one core, sequential, `python3 -I`. Selector definitions are copied (not imported) from the cited
studies into `scripts/l5common.py`; `scripts/frame_census_orig.py` is a verbatim copy (sha256 `afcf2cc3…`).

**Arm F, frame selector vs t** (`selector_scan.py`). Curves k20 (t = 4..12), k24f and k24r (t = 4..14).
Per (curve, t): N = 4000 translation frames (1, L, 0, 1), L uniform in [0, p), plus bits_lo0 (L = 0). Per frame:
c, m₀..m₄, J, S1 (one-bit score), exact covered for t ≤ 9 (k20) / t ≤ 10 (k24). Upper t limits keep frames
nearly disjoint (expected pairwise overlap 2·2^t/p ≤ 0.8%). Selectors scored per cell:
J-selector (key J, cols, −L; the frame-search rule), c-only (key c, −L), composition-only (key and value
Φ = 4J/c²), S1 (key S1, cols, −L), decoy (key = an independent surrogate frame's J), surrogate (iid model).

**Arm I, seed selector vs t** (`run_iso.sh` → archived engine `curve25519_family-5abc70ba iso-census`, the
isogeny-seed study's instrument). k = 20 and k = 24 (= k24r), t = 4..9, seeds 0..999, J by [r]-images,
exact covered, `build_group_ops`, `selector_group_ops`.

**Arm R, real Curve25519** (`real25519_sample.py`). Translation frames u ∈ [L, L + 2^t), L uniform in [0, p).
Folded class of each on-curve u from x([r]P) by an x-only RFC 7748 ladder (counted: xDBL + xADD per bit of r,
506 per label) against x(E[8]) = {∞, 0, x(T4), x(T8), x(3T8)}.
- R1 (enumerated, labelled): t = 6, 8, 10 with N = 2000, 1000, 400 frames.
- R1c (enumerated, columns only, Legendre symbols): t = 12, 14, 16 with N = 400, 200, 50 frames.
- R2 (sampled at t* = 128): 64 frames, 4096 on-curve samples each (y uniform in [0, 2^128)), split into two
  halves A/B for cross-validation; trials per half recorded for the on-curve rate.

**Controls**: `pc_replay.py` (PC1, PC2), `pc3_pools.py` (PC3 + bootstrap calibration), `iso_pc.py` (arm I
instrument identity).

**Seeds (derived rule)**: every RNG seed is the first 8 bytes of sha256("L5|" + "|".join(parts)) with parts
naming the arm, curve and t (`l5common.derived_seed`); the only exceptions are the PC1 replay, which uses the
frame-search seed 20261103 + 20 by design, and the engine's own seed indices 0..999 in arm I.

**Counted units**: group operations (affine additions/doublings in double-and-add on miniatures; ladder
xDBL/xADD on Curve25519; the engine's `GroupOps` in arm I) and Legendre symbols, which are unpriced native
work and are reported at 0, 0.1 and 1 GAE per symbol (§5a). No timing is a result.

## Estimators and intervals

- **Gain at K** (per selector, value): U-statistic E[value of the argmax-key frame over a uniform random
  K-subset of the N frames] ÷ mean value over the N frames (exact weights C(i−1, K−1)/C(N, K)). Primary
  K = 2000 (frame arm, N = 4000), K = 500 (arm I, N = 1000), K = 200 (real-vs-miniature comparison).
  Also reported at K = 10, 100, 500, 1000.
- **Intervals**: bootstrap over frames (B = 500, derived seeds), replicates re-centred on the point estimate
  (the bootstrap of an extreme statistic is biased low) and their spread multiplied by a calibration factor
  f = max(1, SD of the U-statistic over 40 independent N = 4000 sets ÷ bootstrap SD), measured at k20,
  t = 5, 6 from 80 independent pools (`pc3_pools.py`). Intervals are 2.5–97.5% of the calibrated replicates.
- **Excess** e = gain − 1 (the quantity that matters at 255 bits).
- **Functional forms** (preregistered), fitted by weighted least squares with bootstrap SEs of the
  transformed response:
  - M1: ln gain = a + b·t (2 parameters);
  - M2: ln e = a − α t ln 2 (2 parameters, α free);
  - M3: ln e = a − (t/2) ln 2 (1 parameter; the mechanism's α = 1/2).
- **Model-comparison rule (decides the form)**: fit on the training t (arm F: t ≤ 10; arm I: t ≤ 7), predict the
  held-out t (arm F: k20 t = 11, 12; k24f, k24r t = 11..14; arm I: t = 8, 9). Score per curve =
  RMS over held-out t of ln(e_pred / e_obs) (∞ if e_pred ≤ 0). The winner minimises the worst score over
  curves; a model with fewer parameters whose worst score is within 0.1 of the winner's wins instead. The form
  is **adequate** if the winner's worst score ≤ ln 2. The winner is then refitted on all t for extrapolation;
  α is always reported from M2 on all t with its bootstrap interval.
- **Derived t\***: log₂ mean J(t) = 2t + b fitted on k24f (slope fixed at 2); t* = ⌈(log₂ r − b)/2⌉ for m = 2
  (coverage J/r = 1 at r ≈ 2²⁵²; expected 128 since J ≈ c²/4 = 2^(2t−4)). For m summands (rough, derived):
  t ≈ (log₂ r + 3 + log₂ m!)/m, i.e. 86 (m = 3) and 65 (m = 4). This t is derived here; no cited study states
  a 255-bit t.

## Predictions (numeric thresholds, fixed now)

Positive controls (the detector can see an effect of the size that matters, ≈ 2–3×):
- **P0-PC1 exact replay.** The verbatim frame-search instrument with its k = 20 seed reproduces the published
  selected frame, covered count, gain_vs_mean (2.8377 at t = 5, 1.9775 at t = 6) and gain_vs_bits_lo0
  (4.714, 3.309) exactly.
- **P0-PC2 instrument equivalence.** L5's fast instrument gives the same cols, J and covered on every one of
  those 3 × 2001 frames (0 mismatches).
- **P0-PC3 reproduction within the interval.** Over 80 independent fresh pools of K = 2000 frames at k20, the
  published realised gains 2.84 (t = 5) and 1.98 (t = 6) lie inside the 2.5–97.5% range of the realised gains.
- **P0-iso.** Every arm I row for (k, t, seed) also in the isogeny-seed study's data (t = 4..7) is identical.

Functional form (arm F, J-selector, value J, K = 2000):
- **P1.** M1 does not win; M2's α on all t lies in [0.35, 0.65] on each of k20, k24f, k24r.
- **P2.** The winning form is adequate (worst held-out score ≤ ln 2).
- **P3a.** The composition-only selector's excess falls faster: M2 α_Φ ≥ 0.8 on each curve (predicted ≈ 1).
- **P3b.** At every t ≥ 8 the c-only selector's excess is ≥ 0.7 of the J-selector's (the gain is mostly a
  column-count effect).
- **P4 (random control).** Real excess ÷ iid-surrogate excess, geometric mean over t ≥ 6 per curve, lies in
  [0.75, 1.33] (no structure beyond binomial/multinomial fluctuation).
- **P8 (negative control).** The decoy selector (key independent of the frame) has |z| ≤ 3.5 in every cell,
  z = (gain − 1)/(√Σw² · SD(J)/mean(J)) (its exact null spread).
- **P9 (K scaling).** (e at K = 2000) ÷ (e at K = 100), geometric mean over t ≥ 8 per curve, in [1.1, 1.6]
  (Gaussian tail: 1.37).
- **P5 (cost).** Counted double-and-add [r]P costs between 1.0 and 1.6·log₂ r group operations per base point
  in every cell, and the label from [r]P equals log mod 8 on every checked point (200 per cell). Reported, no
  threshold: per-frame full-label cost c·ops + 2^t Legendre; one-bit (S1) and c-only selectors 0 group
  operations, 2^t + c Legendre symbols.

Arm I (seed selector):
- **P7.** M2's α for the seed selector's J-excess (K = 500) lies in [0.35, 0.65] on k20 and k24r.
  Reported, no threshold: winner of the same model rule, dispersion of columns, build and [r]P selector
  group operations per seed and per base point.

Arm R (real Curve25519):
- **P6.** At t = 6, 8, 10 the real-curve J-selector excess at K = 200 is compatible with k24f's: its 95%
  interval intersects [e_k24f/1.5, 1.5·e_k24f] at every t.
- **R1.** The column dispersion index D lies in [0.7, 1.4] at t = 12, 14, 16 (character sums on intervals
  behave like coin flips up to 2¹⁶).
- **R2.** At t* = 128 the 5-class counts of the 64 sampled frames are homogeneous (χ², df = 252, p ≥ 0.001),
  the on-curve rates are homogeneous (χ², df = 63, p ≥ 0.001), and pooled class proportions match
  (1/8, 1/4, 1/4, 1/4, 1/8) (χ², df = 4, p ≥ 0.001).
- **R3.** The frame selected by Ĵ_rel on half A has a half-B excess inside the 0.5–99.5% range of the
  half-B excesses of all 64 frames (sampled selection finds nothing). **R3b**: (−1)^label = χ(u) on every
  labelled point of arm R.
- **R4 (derived).** The winning form's extrapolated excess at t* (K = 2000), upper 95% end, is ≤ 10⁻¹⁵
  (mechanism point prediction ≈ 3.7·10⁻¹⁹).

Derived 255-bit numbers reported (no threshold, labelled DERIVED): t*; rho = √(πr/4) ≈ 2^125.83 group
operations; per-frame cost of full enumeration at t* (full label ≈ 2^127·506 ≈ 2^136 group operations,
≈ 1.1·10³ rho; one-bit/c-only ≈ 2^128 + 2^127 Legendre symbols = 0, 0.68 and 6.8 rho at w = 0, 0.1, 1);
samples per frame needed to resolve the true excess, n = (s₁ z_K / e)² with s₁ measured in R2; the minimum
IC cost (÷ rho) at which a selection costing one frame enumeration could pay, C_sel·(1 + 1/e); excess at
K = 2⁶⁴ by Gaussian-tail scaling; and the number of traces admitting an auxiliary curve with 2^t* | #E′
(≈ 4√p / 2^t* ≈ 2^1.5), which the isogeny-tower base needs at t* (`prime_tower.rs`) — a derived note that the
seed arm's base may not be constructible at 255 bits, not a measurement.

## What counts as failure

Any prediction whose threshold is missed is reported as failed and not retuned. If P1/P2 fail (no adequate
form), the 255-bit numbers are reported as the envelope of all three models, labelled "form inadequate". If
PC1/PC2/P0-iso fail, the instrument is wrong and no arm F/I result is reported until the cause is found and
stated. If P0-PC3 fails, the frame-search gains are not reproduced and that is the headline. If P6 or R1
fail, the curve-size independence that the extrapolation rests on is broken; the extrapolation is then
reported only as conditional on the miniature behaviour.

## Claim scope

Component-level, algebraic track only. Not a leaderboard row (no replay certificate, validator report or
matched rho arm: the validation gate is not attempted). No relabelling of rho or BSGS. Distance to rho is
imported (above). A coverage gain of a few × at t ≤ 8 cannot change any order across the §5a band; at 255 bits
the expected gain is ≈ 1 + 10⁻¹⁹.

## Runtime and memory estimate (one core)

From smoke timings: arm F ≈ 14 min (k24 log tables 9 s each; per 100 frames 1.2 s at t = 10 with covered,
5.4 s at t = 14); arm I ≈ 19 min (0.34 s/seed at k20 t = 9, 0.5 s/seed at k24 t = 9); arm R ≈ 31 min (1.2 ms
per ladder label, ~0.1 ms per Legendre symbol); PC3 pools ≈ 0.5 min; analysis (B = 500) ≈ 5 min.
**Total ≈ 70 min CPU, peak memory < 1 GB** (k24 log table 134 MB). Budget 3 h / 4 GB. Data on disk ≈ 10 MB,
every file < 1 MB; figure PNG < 60 KB.

## Deviations (stated now)

No Conductor task, no isolab-eks route, no S3 resume checkpoint (AWS route and Conductor absent in this
container; all units are deterministic counts run locally). research-kb and the lead atlas are unreachable
here; "Sources checked" lists the repository files read. No commit (the main session commits).

## Sources checked (repository files read)

- `research/curve25519_wave7_untried_20261122/BRIEF.md`
- `research/curve25519_frame_search_20261103/{README.md, PROTOCOL.md, scripts/frame_census.py, scripts/analyse.py,
  scripts/run_confirm.sh, scripts/run_confirm_rulefix.sh, scripts/run_engine.sh, confirm/k20.log, confirm/k24.log,
  analysis/summary.json}`
- `research/curve25519_descent_skip_20261114/{README.md, PROTOCOL.md, frame_selection.py, frame_selection.out.txt,
  parity_check.py, parity_check.out.txt, rate_ratio.py, rate_ratio.out.txt}`
- `research/curve25519_isogeny_seed_20261103/{README.md, PROTOCOL.md, scripts/analyse.py, scripts/run_census.sh,
  analysis/summary.json, data/k20.jsonl (head)}`
- `research/curve25519_isogeny_seeds_20261114/{README.md, PROTOCOL.md, summary.txt, run_seed.sh, summarise_seeds.py,
  tower_cost.jsonl (head)}`
- `research/curve25519_wave6_runs_20261115/L1_faithful/{faithful_search.txt, PROTOCOL.md (grep)}`
- `src/bin/curve25519_family.rs` (subcommand table, `iso_census`), `src/cryptanalysis/ic_framework/prime_tower.rs` (header)
