# Many-target crossover: amortised compact-orbit IC against batch rho (2026-10-30)

**Verdict: dead where it can be ranked (n = 41..61); undecidable at n = 73/83 only for want of counts, and the
n = 83 wall-clock evidence points the same way.** Amortising relation collection over k targets never makes IC
cheaper than batch rho at any k. G = min over k of IC(k)/batch-rho(k), the distance remaining:

- **Clock-priced `ic price` units (same process, both arms):** G = 3.0 to 4.1 at n = 41..61 (best swept recipe),
  3.7 to 6.8 for the recipe tuned for k = 32. No size has G < 1; above 2^36 G is flat to rising.
- **Section 5a count band (Table 1b):** with IC priced at its *emitted* additions only (a lower bound) and rho
  at 0, 0.1 and 1 GAE per canonicalisation, G stays above 1 at n = 41, 43, 53, 61 (3.1 to 4.8 at w = 0, still
  1.6 to 2.4 at w = 1), so the order holds across the band at those sizes. At n <= 45 and at n = 47 the IC
  lower bound is below rho because descent and construction work is not emitted as counts: **those sizes are
  unresolved by counts** (clock-priced G there is 5.3 to 24, which is a calibration, not a band result).
- **n = 83 (wall/CPU seconds, practicality evidence only, not a metric):** G = 4.8 to 5.1 for K3300, and a
  precomputation-rho table with the same setup time has a smaller per-target cost than IC at every k (Table 4).
  The producer emits no counted S0 at n = 73/83 (implementation gap), so §5a cannot rank them.

Class: accounting / audit. Not an improvement to the IC baseline (AGENTS.md section 6). Nothing was run;
every number is extracted from recorded runs with the native `crossover/` program in this directory.
Owner request: task from the IC research-program coordinator, Lead 2 (REVIEW.md M1 context).

## 1. Model

Per target set of k targets on one curve, total cost in one unit:

- **IC(k) = S0 + k d.** S0 = target-independent work (factor base, relation collection, linear algebra,
  constructions); d = per-target descent plus its check. Taken from `ic price` reports of a 32-target
  batch (`median.shared_units`, `median.per_target_units`). The shared part is exactly total minus
  32 d (checked at n = 61: 186.0e6 + 32 x 585,306 = 204.7e6 in the pinned run).
- **Batch rho(k) = rho1 K(k)**, with `K(k) = sum_{i<k} C(2i,i)/4^i = 2k C(2k,k)/4^k`
  (about `2 sqrt(k/pi)`), rho1 = the single-target walk cost (signed Frobenius and negation, so
  N' = r/(2n)). This is the Kuhn-Struik law as implemented in `src/bin/ic/rho.rs` `batch_law`
  (`K(32)/32 = 0.19869`, the repo's `L(32)`). Per-target share `K(k)/k -> 1.128/sqrt(k)`, so "per target
  proportional to sqrt(N/k)" is right asymptotically; at k = 32 the exact share is 12% above 1/sqrt(32).
  rho1 is calibrated on the measured 32-target batch (`rho_batch.s_priced_per_target x sqrt(r) x 32 / K(32)`);
  the check against the 32 independent cold single-target runs (set M1) is the "KS check" column,
  0.80 to 1.19.
- **Window.** With x = sqrt(k) and large k, IC < batch iff `d x^2 - 1.128 rho1 x + S0 < 0`. A window exists iff
  `sigma delta < 0.318`, with `sigma = S0/rho1`, `delta = d/rho1`. The best ratio is `G ~ 1.773 sqrt(sigma delta)`
  (verified numerically against the grid minimum). The window is bounded by two walls: IC cannot win once
  `k > k_cap = (1.128/delta)^2` (descent alone exceeds the batch rho share), and cannot win before
  `k_lo ~ (sigma/1.128)^2` (setup not yet amortised). A window needs `k_lo < k_cap`. So **G is the distance
  remaining**: S0 x d must fall by G^2 (9 to 17x at n = 41..61, about 23x at n = 83) before any k* exists.
- **Literature check (done, with one gap).** Kuhn-Struik, SAC 2001 (LNCS 2259, pp. 212-229): multiple-log
  rho; I could not retrieve its text from this sandbox, so the exact form of `K(k)` rests on the repo's
  transcription plus the measured KS check above. Hitchcock et al. and Fouque-Joux-Mavromati
  ([ePrint 2013/761](https://eprint.iacr.org/2013/761)): all logs of L users in time O~(sqrt(N L)), the
  abstract states no restriction on L. Yun, EUROCRYPT 2015 ([ePrint 2014/637](https://eprint.iacr.org/2014/637)):
  generic lower bound matching O(sqrt(np)) for n logs in a group of order p; I could not read the parameter
  condition. Consequence: sqrt(k N) is the right comparator and the exponent cannot be beaten by a generic
  table, but a non-generic method (IC) is not bound by it, which is why the measured S0 and d decide.
  **Literature gap.** The PDFs of Fouque-Joux-Mavromati and Yun returned HTTP 403 here and the
  Kuhn-Struik text was not reachable, so any range restriction on k (for example k up to N'^(1/4), about 290,
  800, 1,100 and 2,700 at n = 41, 53, 61, 83) is **not confirmed by a source I could read**. Galbraith,
  *Mathematics of Public Key Cryptography*, ch. 14 attributes a sqrt(kN)-type collision estimate to Kuhn-Struik
  Theorem 1, which supports the form only. The Table 1 optima (k_opt 148, 626, 444) are below those
  values; the Table 2 best-recipe optima (759, 1,830, 2,730) and the n = 83 optimum (16,800) are up to 6x above
  them. If the law underestimated batch rho's true cost there by a factor f, those G would fall by f; f of 3 or
  more would be needed to bring a best-recipe row to 1, and nothing recorded suggests it. Table 1 and Table 1b
  do not depend on this caveat. The KS-check column (0.80 to 1.19) is the empirical support at k = 32.

## 2. What is measured and what is extrapolated

| component | n = 19..61 | n = 73 | n = 83 |
|:--|:--|:--|:--|
| S0 (setup + relation collection + LA) | **measured**, clock-priced `ic price` units plus emitted counts, 4 sets x 9 sizes + recipe sweep | ledger (R1: 5.1 h single thread, 18,398 s, K600); **not re-measured** | **measured**, `/usr/bin/time` user CPU of the IC process, 4 records (2 runs x K600/K3300) |
| d (per-target descent) | **measured**, mean over 32 targets per set | one target only (249 to 544 ms, 457,561 probes); heavy-tailed, so a single sample is weak | **measured**, mean of 16 targets (heavy tail: 0.6 to 165 ms), bootstrap |
| rho1 | **measured** batch of 32 plus 32 cold singles, same process | **scaled** from n = 83 by sqrt(N'73/N'83) = 3.38 (the recorded rho intervals on the shared host vary 46 to 683 s and are not usable) | **measured**, 16 plain-rho runs on the same runner |
| trend fit and extrapolation to n = 73/83 | **extrapolated** (section 5) | | |
| counted S0 at n = 73/83 | | not emitted | **not emitted**: the IC producer reports wall and states, not GAE, for the rank stage (implementation gap) |

Units. Tables 1 to 2 are in **clock-priced `ic price` units**: every phase of both arms is timed on an
exclusive clock in the same process and divided by the measured time of one batched affine addition
(LEADERBOARD Tables A and C: "a clock reading, not a count"). That is a calibration, and AGENTS.md section 5a
says a calibration may be added as a further point but never replaces the 0 / 0.1 / 1 GAE band. So the
band is applied separately, from native operation counts, in **Table 1b**:

- IC counts: one batched addition per summand scanned in collection (`counts.collect.summands_scanned`), per
  stored pair (`counts.build.stored_pairs`) and per descent trial (`counts.descent.trials_per_target`).
  Collection scans via `add_many_lazy*`, so a summand is one batched addition (price.rs implied 2.6 units per
  summand, 9.2 per stored pair, so each is at least one addition and a lookup). These are **lower bounds**: linear
  algebra, selection, projections, constructions and, for m = 3 descents, the summands scanned per trial are not
  emitted as counts.
- Rho counts: `rho_batch.gae` (additions) plus one canonicalisation per step (`canonicalisations_uncharged`) at
  weight w = 0, 0.1, 1 GAE.
- Variant A prices w on rho only (the IC-favourable reading). Variant B also prices one lookup per IC addition at
  w.
- Ranking rule: an order is claimed only where it holds at every weight with IC at its lower bound.

The columns "G, rho step = 1.0 / 2.0" in Table 1 reprice only the rho step inside the clock-priced units
(1 + w units, w = 0 and 1) and are a sensitivity, not the band.

## 3. Results

### Table 1: k = 32 recipe, clock-priced `ic price` units, four target sets per size

Uncertainty: 2,000 bootstrap draws; each draw picks one of the four sets (which fixes S0, the per-target trial
counts and that set's batch-rho price, so rho1 varies only across the four sets), then resamples that set's 32
targets with replacement for d. S0 is deterministic per recipe (counts identical across repetitions).

| curve | log2 r | S0/rho1 | d/rho1 | G = min IC/batch [95%] | k_opt | G, S0 without constructions | G, rho step 1.0 | G, rho step 2.0 |
|:--|--:|--:|--:|:--|--:|--:|--:|--:|
| K_1/GF(2^19) | 18.0 | 95.1 | 1.903 | 23.90 [19.74, 28.10] | 50 | 16.40 | 42.98 | 21.49 |
| K_1/GF(2^23) | 22.0 | 51.0 | 0.904 | 12.06 [7.38, 18.62] | 57 | 7.59 | 22.20 | 11.10 |
| K_1/GF(2^45) | 24.8 | 124.5 | 0.784 | 17.52 [14.15, 21.48] | 159 | 6.80 | 37.80 | 18.90 |
| K_0/GF(2^37) | 27.8 | 40.1 | 0.587 | 8.62 [6.70, 10.73] | 69 | 4.79 | 18.02 | 9.01 |
| K_1/GF(2^43) | 32.1 | 36.3 | 0.477 | 7.39 [5.91, 9.09] | 76 | 4.69 | 15.87 | 7.94 |
| K_1/GF(2^47) | 36.6 | 33.6 | 0.261 | 5.25 [4.34, 6.21] | 129 | 3.63 | 11.26 | 5.63 |
| K_0/GF(2^41) | 39.0 | 25.1 | 0.170 | 3.66 [2.84, 4.85] | 148 | 3.17 | 8.24 | 4.12 |
| K_0/GF(2^53) | 44.3 | 55.7 | 0.089 | 3.95 [2.22, 5.81] | 626 | 3.63 | 8.49 | 4.24 |
| K_0/GF(2^61) | 47.2 | 80.8 | 0.182 | 6.80 [5.36, 9.49] | 444 | 6.54 | 15.65 | 7.82 |

Full output, raw S0, d, rho1 in units: [results.md](results.md), [crossover_results.json](crossover_results.json).

### Table 1b: section 5a count band, G from native operation counts (IC counts are lower bounds)

| curve | log2 r | S0c/rho1c | dc/rho1c | G, A, w=0 [range over 4 sets] | A, w=0.1 | A, w=1 | B, w=0 | B, w=0.1 | B, w=1 | reading |
|:--|--:|--:|--:|:--|--:|--:|--:|--:|--:|:--|
| K_1/GF(2^19) | 18.0 | 3.3 | 0.015 | 0.39 [0.33, 0.44] | 0.37 | 0.28 | 0.39 | 0.41 | 0.57 | unresolved (counts incomplete) |
| K_1/GF(2^23) | 22.0 | 2.1 | 0.065 | 0.69 [0.38, 0.97] | 0.65 | 0.43 | 0.69 | 0.72 | 0.87 | unresolved |
| K_1/GF(2^45) | 24.8 | 2.7 | 0.079 | 0.82 [0.74, 0.90] | 0.76 | 0.48 | 0.82 | 0.84 | 0.96 | unresolved |
| K_0/GF(2^37) | 27.8 | 3.3 | 0.354 | 1.94 [1.40, 2.41] | 1.79 | 1.05 | 1.94 | 1.97 | 2.11 | rho ahead in the band (w = 1 marginal) |
| K_1/GF(2^43) | 32.1 | 8.7 | 0.482 | 3.62 [3.12, 4.03] | 3.31 | 1.86 | 3.62 | 3.64 | 3.73 | rho ahead in the band |
| K_1/GF(2^47) | 36.6 | 12.1 | 0.000 | 0.07 [0.06, 0.07] | 0.06 | 0.03 | 0.07 | 0.07 | 0.07 | unresolved: m = 3 descent emits no scan count (164 trials), lower bound is empty |
| K_0/GF(2^41) | 39.0 | 21.5 | 0.212 | 3.78 [3.50, 4.15] | 3.44 | 1.90 | 3.78 | 3.79 | 3.80 | rho ahead in the band |
| K_0/GF(2^53) | 44.3 | 34.2 | 0.086 | 3.12 [2.00, 4.21] | 2.84 | 1.56 | 3.12 | 3.13 | 3.13 | rho ahead in the band |
| K_0/GF(2^61) | 47.2 | 54.3 | 0.132 | 4.77 [4.31, 5.89] | 4.34 | 2.39 | 4.77 | 4.77 | 4.77 | rho ahead in the band |

"Unresolved" rows are exactly those where IC's emitted counts are so incomplete that its lower bound sits below
rho (n <= 45: constructions and projections dominate the clock price, 7 to 70% of S; n = 47: the descent is
m = 3 and its per-trial scanning is not emitted). By section 5a those cannot be ranked from counts; I do not
call them rho wins, and the clock-priced G there (Table 1: 5.3 to 24) is a calibration. At n = 41, 43, 53, 61
the IC lower bound alone is already above rho at every weight, so the ranking is band-stable there. The
S0c/rho1c column also shows that the setup, in additions alone, is 21 to 54 single-target rho walks at
n = 41, 53, 61 and grows with n there (21.5, 34.2, 54.3).

### Table 2: best recipe in the recorded (columns, descent arity) sweep (re-ranked for large k, not for k = 32)

| log2 r | n | best recipe | G | k_opt |
|--:|--:|:--|--:|--:|
| 18.0 | 19 | c8-m2 | 23.64 | 51 |
| 22.0 | 23 | c8-m2 | 12.00 | 57 |
| 24.8 | 45 | c8-m2 | 17.35 | 163 |
| 27.8 | 37 | c8-m2 | 8.50 | 76 |
| 32.1 | 43 | c24-m2 | 6.11 | 167 |
| 36.6 | 47 | c80-m2 | 3.53 | 624 |
| 39.0 | 41 | c152-m2 | 2.98 | 759 |
| 44.3 | 53 | c376-m2 | 3.52 | 1,830 |
| 47.2 | 61 | c640-m2 | 4.10 | 2,730 |

Sweep points are single repetitions on one seed set, so the minimum over the sweep is optimistic by
selection; the Table 1 interval widths (about +/-25 to 40%) are the guide to their noise. The sweep shows the
two walls directly: enlarging the factor base lowers d/rho1 (n = 61: 0.35 to 0.010) and raises S0/rho1
(76 to 830), so the product sigma delta has a minimum of 2.8 to 5.3 at n = 41..61 and never reaches 0.318.

### Table 3: n = 83, wall/CPU seconds on one runner per row (not counted units)

| run / recipe | S0 CPU s | d mean ms [95% boot] | rho1 s [95% boot] | G [95% boot] | k_opt | k_cap |
|:--|--:|:--|:--|:--|--:|--:|
| run 2, K3300 (EPYC 7763) | 825 | 49.3 [31.5, 69.3] | 2.375 [1.72, 3.03] | 4.76 [3.42, 6.81] | 16,781 | 2,961 |
| run 1, K3300 (EPYC 9V45) | 604 | 37.3 [23.9, 52.4] | 1.636 [1.21, 2.11] | 5.14 [3.69, 7.30] | 16,188 | 2,453 |
| run 2, K600 | 804 | 1,967 [539, 3,552] | 1.621 [1.19, 2.08] | 43.5 [22.8, 66.7] | 410 | 1 |
| run 1, K600 | 816 | 2,025 [574, 3,675] | 2.375 [1.71, 3.07] | 30.4 [16.5, 45.9] | 403 | 2 |

K3300 beats K600 by 6x to 9x in G (S0 is about equal, d falls 40x), the same S0-versus-d trade as Table 2,
and there the large table (10.8 GB RSS) is what lowers d. k_cap < k_lo for every row: at K3300 the descent
alone is above a batch-rho share for any k > 2,500 to 3,000, while the setup needs k >~ 10^5 to amortise.

### Table 4: n = 83, total CPU seconds for k targets, every recorded arm (run 2, K3300, EPYC 7763)

| k | IC | batch rho (Kuhn-Struik) | BL, setup 651 s (a) | BL, setup 680 s (a2) | BL, 10.5 GB table (b) | cheapest |
|--:|--:|--:|--:|--:|--:|:--|
| 1 | 825 | 2 | 651 | 680 | 21,036 | batch rho |
| 100 | 830 | 27 | 651 | 681 | 21,036 | batch rho |
| 10^4 | 1,318 | 268 | 693 | 769 | 21,048 | batch rho |
| 10^5 | 5,751 | 848 | 1,071 | 1,572 | 21,154 | batch rho |
| 10^6 | 50,081 | 2,680 | 4,856 | 9,593 | 22,219 | batch rho |
| 10^7 | 493,384 | 8,476 | 42,704 | 89,809 | 32,868 | batch rho |

BL arm (a) has a *smaller* setup (651 s) and a *smaller* per-target online cost (4.2 ms mean) than IC (825 s,
49 ms) at every k, so IC is Pareto-dominated by a precomputation-rho table of one-fortieth of IC's memory
(0.3 GB against 10.8 GB). The same holds in the other three runs (K600 rows are worse). Batch rho beats everything
up to 10^7 targets. Online-per-target BL numbers are from `research/20261016-n83-precomputed-rho-control`
(wall seconds, hosted runners).

### Table 5: n = 73 (ledger, mixed hosts; rho1 scaled; low confidence)

S0 = 18,398 s (K600, single-thread rank), rho1 = 8.0 s (scaled from n = 83), d = 0.249 / 0.326 / 0.544 s
(three repeats of the same target) gives G = 15.0 / 17.1 / 22.1 at k_opt of 34,000 to 74,000. K600 is the
untuned recipe at this size. At n = 83 the K3300 recipe lowers G by 6 to 9x against K600; a K3300-class recipe
at n = 73 has not been run, and its S0 there is unknown, so no n = 73 estimate better than "above 1, with the
n = 83 K3300 value 4.8 to 5.1 as the best proxy" is supported.

## 4. k*(n)

No crossover k* exists at any measured size (G > 1 everywhere), so the table of k* is the table of where
IC comes closest: **k_opt = 150 to 760 (n = 41..53 chosen recipes), 760 to 2,730 (best recipes), 16,800 (n = 83
K3300)**, with G = 3.0 to 4.1, 4.8 to 5.1 as above. The requirement for any k* to exist is
sigma x delta < 0.318; measured minimum 2.8 (n = 41, c152-m2), 4.0 (n = 47), 3.9 (n = 53), 5.3 (n = 61),
7.2 (n = 83 wall). The product has not decreased since n = 41.

## 5. Trend and its limits

- The fit of ln G on log2 r over all nine sizes gives slope -0.054 per bit [-0.067, -0.042] and
  G(n = 73) = 2.3 [1.6, 3.0], G(n = 83) = 2.7 [2.0, 3.5]; over n >= 37, -0.047 per bit, G(73) = 2.6 [1.7, 3.6],
  G(83) = 3.0 [2.1, 4.0]. **I do not rely on this fit.** It is driven by the sizes below 2^36, where the workflow constructions
  (big-integer arithmetic, 7 to 70% of S) dominate and fall away. At log2 r = 36.6, 39.0, 44.3, 47.2 the best-recipe
  G is 3.5, 3.0, 3.5, 4.1 and the k = 32 recipe's is 5.25, 3.66, 3.95, 6.80: flat to rising. A flat or rising continuation gives
  G(73), G(83) of roughly 4 to 11. The one n = 83 measurement (4.8 to 5.1, tuned K3300, wall) is inside that range.
  Every bracket excludes G = 1.
- Even the optimistic fit reaches G = 1 only after about 18 more bits of log2 r beyond n = 83's 52.9
  (ln 2.7 / 0.054), i.e. at log2 r near 71, and needs the slope to persist from sizes where it is not observed.

## 6. Skeptical reading

1. **Does per-target descent dominate (kill the lead)?** It does not dominate at the optimum (by construction
   S0 = k d there), but it is the wall on the far side: the n = 53/61 k = 32 recipes have d/rho1 of 0.09 to
   0.18, i.e. already one tenth of a *single* rho per target, which is overtaken by the batch share at
   k > 40 to 160. Raising K moves that wall out (k_cap = 2,500 to 3,000 at n = 83 K3300) but moves the setup wall
   out faster. Neither wall's position improved with n beyond what the factor base size buys.
2. **Equal advice.** Precomputation rho shares IC's amortisation structure, which is why Table 4's arm (a) beats
   IC at every k at n = 83. The equal-advice instrument `equal_advice_wide_20261018` (M1) was **not run**: it
   needs 24 to 48 GiB and 40 to 100 core-hours of table walks, the host is loaded, and its output is single-target
   online wall. The n = 83 control above is the recorded substitute and is wall-clock on hosted runners.
3. **Models, not measurements, at the top.** S0 at n = 73 and 83 has no counted-unit emission; n = 73 has one
   target; rho1(73) is a scaling. Only n <= 61 has emitted counts and clock-priced units, and G there is already 3 to 7.
4. **Selection.** Table 2 minimises over about 20 recipes per size; Table 1 uses the recipe tuned for k = 32.
   The truth for a recipe tuned for each k is between them; both exceed 2.9.
5. **Heavy tails.** d at n = 83 ranges 0.6 to 165 ms across 16 targets; the bootstrap interval for G is
   [3.4, 7.3]. A single-target ledger d (n = 73) is not an estimate of the mean.
6. **Frobenius/negation.** Both arms use the same equivalence (N' = r/2n); the batch-rho floor in the
   repo already includes it (`rho_counted_over_floor` 1.02 to 1.08).
7. **Not an improvement.** An accounting row. The gap reported is distance remaining, not a pass/fail gate.

## 7. Next measurements, in order, if the lead is kept alive

1. Emit counted S0 at n = 73/83 (relation checks, probes, states) from the IC producer so the n = 73/83 rows are
   in GAE; today they are wall seconds.
2. A K sweep at n = 73 (K = 600, 1200, 2400, 3300) with 16 targets each, same runner, to see whether the
   S0-versus-d product has a minimum below 0.318 at larger r; needs about 10 GiB per process and several
   hours of rank per point.
3. The only route that moves G is a >= 9x lower S0 x d at fixed n (Table 2 needs G^2 of 9 to 17):
   the Z[tau] multiplier base (PR 2481) reduces relation count (S0) by a constant; descent d has had no
   comparable work. Neither changes the plateau unless both do.

## Files

- `crossover/` native Rust (std plus `serde_json`), reads the recorded JSON only; build
  `cargo build --release --offline` in that directory, run
  `crossover <research/ic_exponent_20260926> <research/20261016-n83-precomputed-rho-control>` from this directory.
  Seed 20261030, 2,000 bootstrap draws.
- `results.md`, `crossover_results.json`, `g_vs_log2r.svg`, `report.pdf`.
- Inputs (frozen, unmodified): `research/ic_exponent_20260926/runs/*/{measure,sweep}`,
  `research/20261016-n83-precomputed-rho-control/records/*`,
  `experiments/koblitz-single-target-n73-20261003/README.md`.
