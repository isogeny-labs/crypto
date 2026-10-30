# Many-target crossover: amortised compact-orbit IC against batch rho (2026-10-30)

**Verdict: dead at every measured size; the lead does not undecide at n = 73/83.** Amortising the relation
collection over k targets never makes IC cheaper than batch rho at any k. The best amortised ratio is
G = min over k of IC(k)/batch-rho(k) = **3.0 to 4.1 at n = 41..61** (best swept recipe, counted units),
**4.8 to 5.1 at n = 83** (same-runner wall seconds, K3300), and no size has G < 1. G has stopped falling
above 2^36 and is flat to rising from there, so the data give no support for a crossover at larger n.
At n = 83 even a precomputation-rho table with the *same setup time* has a smaller per-target cost than IC
(Table 4), so IC is dominated at every k by an arm that is already in the repo.

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
  **Validity range (unverified).** I recall the Kuhn-Struik sqrt(kN) law being stated for k up to about
  N'^(1/4), but could not confirm that bound from the abstracts I could read (the Fouque-Joux-Mavromati
  abstract states none). N'^(1/4) is about 290 (n = 41), 800 (n = 53), 1,100 (n = 61), 2,700 (n = 83). The
  Table 1 optima (k_opt 148, 626, 444 at n = 41, 53, 61) are inside it; the Table 2 best-recipe optima (759,
  1,830, 2,730) and the n = 83 optimum (16,800) are up to 6x outside it. If the law underestimated batch rho's
  true cost out there by a factor f, G would fall by f; f of 3 or more would be needed to bring the best rows
  to 1, and nothing recorded suggests it. Table 1 does not depend on this caveat.

## 2. What is measured and what is extrapolated

| component | n = 19..61 | n = 73 | n = 83 |
|:--|:--|:--|:--|
| S0 (setup + relation collection + LA) | **measured**, `ic price` units, 4 sets x 9 sizes + recipe sweep | ledger (R1: 5.1 h single thread, 18,398 s, K600); **not re-measured** | **measured**, `/usr/bin/time` user CPU of the IC process, 4 records (2 runs x K600/K3300) |
| d (per-target descent) | **measured**, mean over 32 targets per set | one target only (249 to 544 ms, 457,561 probes); heavy-tailed, so a single sample is weak | **measured**, mean of 16 targets (heavy tail: 0.6 to 165 ms), bootstrap |
| rho1 | **measured** batch of 32 plus 32 cold singles, same process | **scaled** from n = 83 by sqrt(N'73/N'83) = 3.38 (the recorded rho intervals on the shared host vary 46 to 683 s and are not usable) | **measured**, 16 plain-rho runs on the same runner |
| trend fit and extrapolation to n = 73/83 | **extrapolated** (section 5) | | |
| counted S0 at n = 73/83 | | not emitted | **not emitted**: the IC producer reports wall and states, not GAE, for the rank stage (implementation gap) |

Units: `ic price` units are a clock-priced batched-addition equivalent measured in the same process for both
arms (rho step 2.3 units, 1.0 of which is the addition). So the IC arm has no unpriced counter. AGENTS.md
section 5a is applied to the rho arm only: its canonicalisation is priced at 0, 0.1 and 1 GAE per native
operation (rho step 1.0, 1.1, 2.0 units; measured 2.28). G > 1 at every weight (columns "rho step = 1.0 / 2.0"
in Table 1), so the ranking (rho ahead) holds across the band. Rho at the cheapest weight is the case least
favourable to IC and is the one in the main columns' neighbourhood; none flips the order.

## 3. Results

### Table 1: k = 32 recipe, counted units, four target sets per size (uncertainty: 2,000-draw bootstrap over sets and targets)

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
   target; rho1(73) is a scaling. Only n <= 61 is in counted units, and G there is already 3 to 7.
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
