# Why the GHS-weak rows are below rho, and which further curves are weak

2026-11-19. Every number is `[pcc:unverified]` (no validator, no replay certificate; counted cost on a shared 4-CPU host, one thread per
process). No leaderboard edit. Protocol: [PROTOCOL.md](PROTOCOL.md) (frozen first; sha256 in `raw/frozen_hashes.txt`).
Single target, charged cold; nothing multi-target.

## Result

1. **The leaderboard rows are not binary.** They are E0/F_{q^3}, q prime, q = 7 mod 12, 4 | #E0(K), through a genus-3 *plane quartic*
   over F_q (odd characteristic). Weak means: Jac(C)(F_q) has size about q^3 = #E0, so the whole subgroup transfers (g = n = 3), and index
   calculus on a non-hyperelliptic genus-3 curve costs about q^1 (Diem; lines give 4-term relations) against rho's q^(3/2). The
   baseline A0 (all q+1 points as factor base) on the *same* curves has exponent 1.89 [`fit_criterion.json`, P3] and is never below rho: the
   gain is the small factor base with large primes, not the cover alone. Fitted exponents (v2 sizes, w = 0): B0 1.12, D 0.92, DL 0.85, DL1 0.79
   (q >= 4051 only: 1.21, 1.04, 0.98, 0.94) against rho 1.50 (plain) and 1.46 (neg). Crossover q* of B0 vs plain rho about 3,300 to 4,800.
2. **A new field, three new instances each, measured end to end** (q = 98299, 196579, 393187; seeds 1..3 of the standard stream, never in an
   earlier grid; matched plain and negation rho on the same instances, 4 targets each; rho calibration passed in all six cells). IC is below
   both rho arms in all 9 instances and at all three prices. IC/rho at w = 0 (field mean; distance to rho remaining = 1 - ratio is not
   needed, every ratio is below 1):

   | q | B0 / plain | B0 / neg | DL1 / plain | DL1 / neg |
   |--:|--:|--:|--:|--:|
   | 98299 | 0.166 | 0.290 | 0.00880 | 0.0153 |
   | 196579 | 0.145 | 0.140 | 0.00644 | 0.00620 |
   | 393187 | 0.107 | 0.161 | 0.00382 | 0.00575 |

   At w = 0.1 and w = 1 (`measure_results.json`): B0 0.106 to 0.183 (plain) and 0.106 to 0.263 (neg); DL1 0.0049 to 0.0176 (plain) and
   0.0066 to 0.0194 (neg). Figure: `ic_over_rho.svg`. These are new curves of an already-known weak family, not a new weakness class.
3. **Binary (desk result only, no binary solver at competitive size exists here).** Closed-form magic-number census over F_{2^N}, N = n*l,
   verified against exhaustive recounts at N = 6, 9, 10, 12, 15 (all 16 towers, deviation 0). With the cost model of PROTOCOL (kappa = 1,
   optimistic to IC) plus a transfer condition g >= n added after the fact (D1), the weak classes are:
   n = 3 class B (genus 3, Tr_{K/k}(b) = 0), first at N = 60 (l = 20), fraction of b about 2^-l; **n = 4, Tr_{K/k}(b) = 0 outside F_{2^{N/2}}
   (Ord = (x+1)^3, genus 4)**, first at N = 44 (l = 11), fraction about 2^-(N/4), the largest weak set found; n = 7 (genus 7, from N = 56),
   n = 8 (genus 8, from N = 64) and n = 6 (genus 8, from N = 84). Counts of weak b per N in
   `census_summary.json` (`T_kappa1`: e.g. N = 44: 8.59e9 b, fraction 4.9e-4; N = 48: 6.87e10, 2.4e-4; N = 60: 3.6e13 incl. 1.1e12 of
   n = 3 class B; N = 96: 4.8e21). With kappa = 10 the first weak N is 70, with kappa = 100 it is 77. These are cost-model predictions,
   not measured: no solver for hyperelliptic genus >= 3 covers at these sizes exists in the checkout, and whether Jac(C) really contains
   the large subgroup for n = 4, genus 4 is the open check (transfer for g = n assumed, not tested).

## Predictions and outcomes (as registered; none edited)

| id | outcome | evidence |
|:--|:--|:--|
| P1 held-out classification (fit q <= 4051) | **PASS** 63/64 cells (98.4%); the one miss is B0 vs neg | `fit_criterion.json` |
| P2 crossover within factor 4 | **PASS** q* 3271 (window) vs 4846 (full), factor 1.48 | same |
| P3 A0 above 3/2, others below 3/2 | **PASS** (A0 1.89, A0r 1.99; others 0.79..1.12) | same |
| P3 others in [1.0, 1.4] | **FAIL** (D 0.92, DL 0.85, DL1 0.79 over the full range; low-q overhead, not asymptotic) | same |
| P4 genus-3 hyperelliptic cubic: slope in (-0.25, 0) | **FAIL** slope +0.034 (IC exponent 1.41 vs R_ideal 1.38 over q = 2^14..2^20; no win, min IC/R 0.52) | same |
| P5 closed form and brute force | **PASS** deviation 0 at 16 towers; n = 3 forms equal the committed census | `census_summary.json` |
| P6 only n = 3 class B weak; no n >= 5 with g >= 7 | **FAIL** (n = 4 genus 2/4, n = 6, 7, 8 classes also pass C-fin; even n and g < n classes are artefacts of dropping the transfer condition) | `census_summary.json` (`asregistered_*`) |
| P7 IC below both rho, every instance, within factor 1.5 of prediction | **below-rho part PASS** (9/9 instances, 3 prices); **band part FAIL** (measured/predicted 0.42 to 0.69 at w = 0 for B0 and DL1: the log-linear fit over-predicts the ratio; IC is better than predicted) | `measure_results.json` |
| P8 admissibility 2 to 60 candidates per instance | **FAIL** mean 128, 95, 108 candidates (about 1% of m = 2 candidates admissible) | `raw/instances.jsonl` |

## Deviations and limits

* D1 (post hoc, after seeing P6): the transfer condition g >= n (the transfer E(K) -> Jac(C)(k) carries the large subgroup only if
  dim Jac >= n, otherwise ell | #Jac(C)(k) has probability about q^(g-n)) is not in the registered C-fin. The `T_*` tables use it; the
  `asregistered_*` tables do not. The sets differ a lot; the T reading is the defensible one but was not frozen.
* The held-out design is structural, not blind (author had read the leaderboard outcome); stated in PROTOCOL.
* Pricing of w in `analyze_measure.py` applies w to adds and to w*gae*(hash+cmp) for both IC and rho (convention of
  `ic_gw_nrm_lz_20261115/analyze.py`); the v2 summary columns may differ slightly in the hash term. w = 0 is unambiguous.
* rho: 12 runs per field and variant only. Calibration (steps/sqrt(ell): plain 1.13 to 1.42, neg 0.78 to 1.14) passed in all cells.
* Not done: binary solver run (none exists at size), isogeny-class (Hess) walks to lower m, odd-characteristic families other than
  q = 7 mod 12 or n != 3 (n = 4, 5 floors are in `research/ghs_cover_census_n4_20261024`, `..._n5_...`), second-host replay. KB
  (`research-kb`) was not queried: no tool for it in this session.
* Host: shared 4-CPU container, load 1 to 3 during runs; counted cost does not depend on load, wall times are not reported.

## Reproduce

```sh
cd research/ic_ghs_weak_families_20261119
(cd ../ic_gw_nrm_lz_20261115/src && rustc --edition 2021 -O v3.rs --cfg combo -o ../../ic_ghs_weak_families_20261119/bin/v3)  # sha256 ef7f27c0...ab44
python3 analysis/fit_criterion.py .      # P1-P4, writes fit_criterion.json (predictions frozen before the run, sha256 896b8ffd...)
python3 analysis/census.py .             # P5, P6, census_summary.json (about 2 s)
./run_measure.sh bin/v3 raw              # about 25 CPU minutes, single thread; raw/ is gitignored
python3 analysis/analyze_measure.py . && python3 analysis/plot.py .
```
Raw units (28 JSONL, 236 KB, gitignored): `raw/SHA256SUMS` itself sha256 `f6a0641b12c4c331e51cd74884f4a8baf386bcdf179552caba249397354fd6c4`.
