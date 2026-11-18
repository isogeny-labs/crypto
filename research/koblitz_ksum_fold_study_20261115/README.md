# Frobenius-folded k-sum against the lazy orbit-member incumbent, K1 n23 and n29 (2026-11-15) `[pcc:unverified]`

Frozen paired study ([`PROTOCOL.md`](PROTOCOL.md), commit `9fa38942e4`, hashes in `REGISTRATION.sha256`, written before any study session).
Candidate `kf3-c1g-sr` (`ksum-frobenius:m=3` on `compact-orbit-scan:columns=1,generator_orbit=1`, `stride-renew`) against the strongest existing IC arm,
the lazy orbit-member pair table (`mitm-frobenius-lazy-counted:m=2` on `compact-orbit-scan`, `stride-renew`), and against the K1 recipe, in the same sessions on the same targets.
Every arm is a plain method priced identically by the AGENTS.md section 5a band (w = 0, 0.1, 1 GAE per unpriced native operation, table work priced). Every number is an unverified assertion.

**Framing.** With one orbit and m = 3 this is a Frobenius-folded generic (square-root-class) collision method. It has no subexponential component and cannot beat rho asymptotically; this is a constant at two tiny sizes. A win is a measured improvement over the current best IC arm on that curve and nothing more.

## K1 n23 (`icv1-f2m23-tm5197-1f85e9e1`)

Mean S per arm (`ecbench band`, 3 sessions x 48 targets x 3 rounds, pooled):

| arm | S(w=0) | S(w=0.1) | S(w=1) |
|:--|--:|--:|--:|
| rho-strong | 0.886 | 0.950 | 1.533 |
| rho-strong-aa | 0.886 | 0.950 | 1.533 |
| rho-neg | 1.182 | 1.282 | 2.185 |
| k1-l9 | 43.823 | 52.783 | 133.419 |
| lazy-c3 | 1.152 | 1.508 | 4.711 |
| lazy-c4 | 0.997 | 1.314 | 4.166 |
| kf3-c1g-sr | 0.782 | 1.050 | 3.467 |
| kf3-c1g-w | 1.199 | 1.467 | 3.882 |

Registered test H1: `R = cost(kf3-c1g-sr) / cost(strictest lazy incumbent)`, paired by target, cluster-bootstrap 95 % CI, pooled over the 3 replicates; per-replicate point estimates and upper ends; A/A = `rho-strong-aa / rho-strong`.

| w | strictest incumbent | R pooled [95 % CI] | R per replicate (upper 95 %) | A/A [95 %] | H1 verdict | R vs `kf3-c1g-w` strictest (exploratory) |
|:--|:--|--:|:--|--:|:--|--:|
| 0 | lazy-c4 | 0.784 [0.747, 0.821] | 0.765 (0.826), 0.814 (0.882), 0.774 (0.843) | 1.000 [1.000, 1.000] | **holds** | 1.202 [1.154, 1.254] |
| 0.1 | lazy-c4 | 0.799 [0.762, 0.837] | 0.780 (0.841), 0.828 (0.897), 0.790 (0.858) | 1.000 [1.000, 1.000] | **holds** | 1.117 [1.072, 1.164] |
| 1 | lazy-c4 | 0.832 [0.795, 0.869] | 0.813 (0.874), 0.860 (0.928), 0.823 (0.892) | 1.000 [1.000, 1.000] | **holds** | 0.932 [0.893, 0.973] |

Each incumbent, the K1 recipe, and the distance remaining to the matched `rho.signed_frobenius_strong` (cost ratio, below 1 = cheaper than rho):

| w | R vs each lazy incumbent | R vs K1 recipe (H2) | `kf3-c1g-sr` / rho-strong | strictest lazy / rho-strong | K1 / rho-strong | `rho-neg` / rho-strong |
|:--|:--|--:|--:|--:|--:|--:|
| 0 | lazy-c3: 0.679 [0.640, 0.721]; lazy-c4: 0.784 [0.747, 0.821] | 0.018 [0.017, 0.018] | 0.883 [0.852, 0.913] | 1.126 [1.082, 1.172] | 49.483 [48.572, 50.474] | 1.334 [1.261, 1.412] |
| 0.1 | lazy-c3: 0.697 [0.657, 0.739]; lazy-c4: 0.799 [0.762, 0.837] | 0.020 [0.019, 0.021] | 1.105 [1.067, 1.143] | 1.382 [1.328, 1.440] | 55.541 [54.364, 56.800] | 1.349 [1.272, 1.430] |
| 1 | lazy-c3: 0.736 [0.695, 0.780]; lazy-c4: 0.832 [0.795, 0.869] | 0.026 [0.025, 0.027] | 2.262 [2.171, 2.357] | 2.718 [2.594, 2.848] | 87.038 [84.032, 90.317] | 1.425 [1.333, 1.523] |

Band ordering (`ecbench band --ranks`, ranked only where the order holds at every w): `kf3-c1g-sr` vs rho-strong: **unresolved inside the band**; `lazy-c4` vs rho-strong: rho-strong < lazy-c4; `kf3-c1g-sr` vs `lazy-c4`: kf3-c1g-sr < lazy-c4; `rho-neg` vs rho-strong: rho-strong < rho-neg.

Secondary pricing (`*_full`, the lazy studies' own): not measured: src/cryptanalysis/ecbench/full_prices.json has no row for icv1-f2m23-tm5197-1f85e9e1 (a = 1), every *_full record errored.

## K1 n29 (`icv1-f2m29-tm40309-30c52b96`)

Mean S per arm (`ecbench band`, 3 sessions x 48 targets x 3 rounds, pooled):

| arm | S(w=0) | S(w=0.1) | S(w=1) |
|:--|--:|--:|--:|
| rho-strong | 4.567 | 4.685 | 5.745 |
| rho-strong-aa | 4.567 | 4.685 | 5.745 |
| rho-neg | 1.892 | 2.007 | 3.045 |
| k1-l10 | 3189.471 | 3510.404 | 6398.802 |
| lazy-c1 | 1.569 | 2.063 | 6.507 |
| lazy-c2 | 2.373 | 3.249 | 11.133 |
| kf3-c1g-sr | 8.322 | 11.597 | 41.073 |
| kf3-c1g-w | 11.096 | 14.371 | 43.848 |

Registered test H1: `R = cost(kf3-c1g-sr) / cost(strictest lazy incumbent)`, paired by target, cluster-bootstrap 95 % CI, pooled over the 3 replicates; per-replicate point estimates and upper ends; A/A = `rho-strong-aa / rho-strong`.

| w | strictest incumbent | R pooled [95 % CI] | R per replicate (upper 95 %) | A/A [95 %] | H1 verdict | R vs `kf3-c1g-w` strictest (exploratory) |
|:--|:--|--:|:--|--:|:--|--:|
| 0 | lazy-c1 | 5.303 [5.226, 5.384] | 5.301 (5.434), 5.308 (5.444), 5.302 (5.432) | 1.000 [1.000, 1.000] | **regression** | 7.071 [6.966, 7.178] |
| 0.1 | lazy-c1 | 5.622 [5.540, 5.704] | 5.619 (5.758), 5.626 (5.768), 5.620 (5.756) | 1.000 [1.000, 1.000] | **regression** | 6.966 [6.865, 7.068] |
| 1 | lazy-c1 | 6.312 [6.223, 6.403] | 6.310 (6.460), 6.318 (6.473), 6.309 (6.456) | 1.000 [1.000, 1.000] | **regression** | 6.739 [6.644, 6.835] |

Each incumbent, the K1 recipe, and the distance remaining to the matched `rho.signed_frobenius_strong` (cost ratio, below 1 = cheaper than rho):

| w | R vs each lazy incumbent | R vs K1 recipe (H2) | `kf3-c1g-sr` / rho-strong | strictest lazy / rho-strong | K1 / rho-strong | `rho-neg` / rho-strong |
|:--|:--|--:|--:|--:|--:|--:|
| 0 | lazy-c1: 5.303 [5.226, 5.384]; lazy-c2: 3.507 [3.496, 3.518] | 0.003 [0.003, 0.003] | 1.822 [1.808, 1.836] | 0.344 [0.338, 0.350] | 698.351 [672.864, 724.810] | 0.414 [0.398, 0.432] |
| 0.1 | lazy-c1: 5.622 [5.540, 5.704]; lazy-c2: 3.569 [3.559, 3.580] | 0.003 [0.003, 0.003] | 2.475 [2.453, 2.497] | 0.440 [0.433, 0.448] | 749.293 [721.662, 778.102] | 0.428 [0.411, 0.447] |
| 1 | lazy-c1: 6.312 [6.223, 6.403]; lazy-c2: 3.689 [3.678, 3.701] | 0.006 [0.006, 0.007] | 7.149 [7.019, 7.279] | 1.133 [1.106, 1.159] | 1113.757 [1068.770, 1160.140] | 0.530 [0.503, 0.559] |

Band ordering (`ecbench band --ranks`, ranked only where the order holds at every w): `kf3-c1g-sr` vs rho-strong: **rho-strong < kf3-c1g-sr**; `lazy-c1` vs rho-strong: unresolved inside the band; `kf3-c1g-sr` vs `lazy-c1`: lazy-c1 < kf3-c1g-sr; `rho-neg` vs rho-strong: rho-neg < rho-strong.

Secondary pricing H4 (`*_full` arms, recorded S, pooled): `kf3-c1g-sr-full / lazy-c1-full` = 5.355 [5.273, 5.439], per replicate 5.352, 5.359, 5.353; `kf3-c1g-sr-full / rho-strong-full` = 2.138 [2.120, 2.155]; `lazy-c1-full / rho-strong-full` = 0.399 [0.392, 0.406]. Same direction as the primary reading.

## Verdicts

| curve | w | H1 (candidate beats the strictest lazy incumbent) |
|:--|:--|:--|
| K1 n23 | 0 | **holds** |
| K1 n23 | 0.1 | **holds** |
| K1 n23 | 1 | **holds** |
| K1 n29 | 0 | **regression** |
| K1 n29 | 0.1 | **regression** |
| K1 n29 | 1 | **regression** |

- **K1 n23: H1 holds at w = 0, 0.1 and 1** (pooled upper 95 % < 1, every replicate upper < 1, beyond the A/A interval). `kf3-c1g-sr` costs about 0.78-0.83x the best lazy incumbent (`lazy-c4`) and 0.68-0.74x `lazy-c3`. It is a measured improvement over the strongest existing IC arm on this curve, not a win over rho: against `rho.signed_frobenius_strong` it is below 1 at w = 0 (0.883) and above 1 at w = 0.1 (1.105) and w = 1 (2.262), and `ecbench band --ranks` leaves the pair unresolved inside the band. It costs 0.018-0.026x the K1 recipe.
- **K1 n29: H1 does not hold; the candidate regresses** 5.3x to 6.3x against `lazy-c1` at every w, with every replicate agreeing. There the base is tiny (r = 2^15.4) and the m = 3 table costs more than the pair table's few relations; `lazy-c1 / rho-strong` is 0.344 / 0.440 / 1.133 at w = 0 / 0.1 / 1 (ordering unresolved inside the band).
- Both are single-curve readings at two tiny sizes with tuned configurations from a disjoint-seed sizing pilot; the exponent is untouched.

## Deviations and disclosures

- Local route (`--cpus none --allow-busy`, L0), not isolab-eks; counts are deterministic, wall times descriptive. The sibling lazy studies did the same.
- `*_full` arms errored on n23 in every session (no `full_prices.json` row for the a = 1 n = 23 curve: 576 records a session, replays reproduce the same error). The H4 secondary reading therefore exists for n29 only; no source change was made (src is frozen for this task). The primary tests do not use `*_full` arms.
- Protocol H0 said `lower_bound = false`; a plain method with unpriced counters is a lower bound at w = 0 by construction, which is why the primary reading is the band. All 6 sessions: audits `ok`, 0 problems, replays reproduced (`audits/`); the only errors are the n23 `*_full` records above.
- `rho-strong-aa / rho-strong` is exactly 1.000 [1.000, 1.000] at every w and curve (deterministic counts, same seeds), so the A/A criterion adds nothing beyond requiring R < 1; wall-time A/A was not used.
- 3 measured rounds with `order = alternate` cannot be balanced (one arm order is used once more); S is a count, so order cannot move it.
- `verify` has no `--exit-code` flag (it exits non-zero on a problem by default); the flag in the task text was not passed.
- The first launch of `run.sh` passed that unsupported flag and was killed before any session finished; sessions were deleted and the run restarted (the record shows both launches). `run.sh`, `analyze.py`, `make_report.py` and the figures were written after the freeze commit; they change no registered quantity.
- Sizing pilot (not data): a first K1 n29 sizing with dimensions 5-9 and `max_trials` 1.35e10 did not finish and was killed; n29 K1 was re-sized at dimensions 8-12 with `max_trials` 2e7 (the study uses 2e7 at n29 and 1.35e10 at n23, as the pilot).
- No `VALIDATION.json`, so no leaderboard or `benchmarks/README.md` row. Conductor was not used (no source edits, research directory only).

## Files

`PROTOCOL.md`, `REGISTRATION.sha256`, `registered-at.txt` (frozen), `specs/`, `make_specs.py`, `make_sizing.py`, `run.sh`, `analyze.py`, `make_report.py`, `results.json`, `summary.json` (`crypto.koblitz_ksum_fold_summary/v1`), `tables/` (bands), `audits/`, `logs/`, `figures/` (SVG, PNG), `REPORT.pdf` / `REPORT.typ`, `sessions/*/records.jsonl.gz` (full records; the session sha256 list is in `sessions.sha256`).
