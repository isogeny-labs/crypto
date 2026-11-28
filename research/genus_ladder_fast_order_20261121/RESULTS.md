# Fast group order (N_1 + baby-step giant-step) and genus 3 at p = 65537 … 262147 — results (2026-11-21)

Status: `[pcc:unverified]` (shared host, no independent replay). Protocol [`PROTOCOL.md`](PROTOCOL.md)
(frozen before data, `frozen.sha256`); binary `binary.sha256`; data `refind_g3.jsonl`, `ext_g3.jsonl`,
`ext_g3_262147.jsonl`, `timing.jsonl`, `probes.log`; joined analysis `analysis.json` (six primes) and
`analysis_4primes.json` (8209 … 131101); `tables.md`; `fig_order.svg`; `data.sha256`.

## What changed in the code
* `JCurve::jacobian_order_with(OrderMethod, seed)` with `OrderMethod::{Enumerate, Bsgs1, Bsgs2}`;
  `jacobian_order` calls the Enumerate path (same code, same results). `Bsgs1` enumerates `N_1` only and
  runs the existing one-dimensional BSGS (same probe discarding, 32 random divisors, exactly one survivor)
  over the Weil window of `#J` left by the unknown `c_2, c_3`. `Bsgs2` is the `N_1 + N_2` window, which
  for g >= 3 is what `Enumerate` already did (so they are the same routine; the generalisation to genus 4
  is today's code).
* `make_instance_with(.., method)`; driver flag `--order-method enumerate|bsgs1|bsgs2` (the config line and
  instance lines carry `order_method` / `order_search_wall_s` only when the flag is set, so a run without it
  writes the frozen layout).
* `count_points(2)`: Horner over F_p[√ν] with Barrett reduction (one reduction per coordinate and step,
  `ν·b` precomputed). Counts identical to the pre-change enumeration (test
  `count_points2_matches_reference`, plus the two probes below); 2.3–2.5× faster.
* Probe for timings: `PROBE_ORDER=p,g,method,cs cargo test --release --lib order_probe -- --ignored --nocapture`
  (the variable is named `PROBE_ORDER` in the committed source; my run scripts used a binary built before the
  rename, where it was `ORDER_PROBE`; the numbers do not depend on the name).

## Part 1: equality (Bsgs1 / Bsgs2 vs Enumerate)
| set | instances | Enumerate = Bsgs1 = recorded `#J` | where |
|---|--:|---|---|
| `genus_ladder_20261114/analysis.json`, hyperelliptic genus 3, p = 67 … 4099 | 28 | 28/28 (Bsgs2 also, p <= 1031) | test `bsgs1_matches_enumerate_on_recorded_instances` (every run) |
| `genus_ladder_scale_20261115/g3_analysis.json`, p = 8209, 16411, 32771 | 9 | 9/9 | `#[ignore]` test `..._on_scale_instances` (110 s), run once |
| 50 random curves at p = 1031, seeds 9000–9049, all outcomes | 50 | 50/50 identical outcome string (all 50 were `ok`; no undetermined/ambiguous outcome occurred) | test `bsgs1_matches_enumerate_on_random_curves` |
| `count_points(2)` new vs reference | 5 curves in the test; p = 8209 (67409019) and p = 16411 (269337584) in the probe | all equal | `count_points2_matches_reference`, `probes.log` |

`cargo test --release --lib genus_ladder::` passes: 13 passed, 0 failed, 2 ignored (probe, scale test),
including `sge_none_reproduces_frozen_counts`. (Note: the study's `analysis.json` lists 56 genus-3 rows
because the plane-quartic instances carry g = 3; the 28 hyperelliptic ones are the genus-3 curves.)
Genus 4: `Bsgs1` equality with `Enumerate` was not tested, because `Bsgs1` is slower there (below).

## Part 2: re-finding the earlier instances (O5)
`--order-method bsgs1`, p = 8209, 16411, 32771, `--curves 3 --seed-search 600`: 9/9 admitted instance lines
equal those of `scale_g3.jsonl` field for field (curve seed, f, order, h, r), and each cell's `tried`
(23, 43, 31) and `rejects` (20, 40, 28 `order_not_near_prime`) are identical. Search wall of the cell: 6.5,
19.1, 26.6 s (3 threads, while a build ran), against 204, 1076, 2429 s in the scaling study. O5 held.

## Order-search timings (single thread; `timing.jsonl`)
| p | genus 3 Enumerate (s/order) | genus 3 Bsgs1 (s/order) | genus 4 Enumerate | genus 4 Bsgs1 |
|--:|--:|--:|--:|--:|
| 8209 | 1.66 | 0.155 | 5.9 | 25.8 |
| 16411 | 6.70 | 0.301 | 17.4 | 78.2 |
| 32771 | 26.9 | 0.560 | – | not run |
| 65537 | – | 1.26 | – | – |
| 131101 | – | 2.83 | – | – |
| 262147 | – | 5.94 | – | – |
| 524309 | – | 12.1 | – | – |
| 1048583 | – | 23.2 | – | – |

The "Enumerate" column already includes the 2.4× faster `count_points(2)` (before it: 4.03 s vs 1.61 s for the
count at p = 8209, 16.3 vs 7.2 s at 16411). Wall per order is the cell's search wall over the seeds tried
(3–6 curves; most are rejected after their order is known, and every rejected curve needs the order).
Bsgs1 genus 3 grows linearly in p (m ≈ 5.5 p baby steps), about 2× per doubling.

Largest p per genus (computed by `bsgs_table_size`, table limit 2^26 entries):
* genus 3, Bsgs1: table fits to p ≈ 1.2e7 (m = 5.48 p); the binding limit is the 63-bit order (`JCurve::new`:
  (√p+1)^6 < 9e18, p < 2 077 200). Measured to p = 1 048 583 at 23 s per order.
* genus 4: the task's estimate m ≈ 63 p^1.75 is not what the code does. With `N_1, N_2` known the window
  gives m ≈ 10.6 p^1.25 (8.3e5 at 8209, 2.0e6 at 16411, 2.7e7 at p = 131101), under 2^26 up to p ≈ 276 000, so the
  table is no limit; the 63-bit order limit is p < 54 305. `Bsgs1` at genus 4 has m ≈ 7.5 p^1.5 (5.6e6 at 8209,
  1.6e7 at 16411, 4.5e7 at 32771) and fits to p ≈ 43 000.
* Genus 4 negative result: `Bsgs1` is 4.4× (8209) and 4.5× (16411) slower than the N_1 + N_2 route, because the
  BSGS over the wider window costs more than the enumeration it replaces; nothing beat the current routine.
  The only gain at genus 4 is the faster `count_points(2)` (2.4×); Enumerate scales as p² (5.9 s at 8209, 17.4 s
  at 16411, extrapolated ≈ 70 s at 32771 and ≈ 190 s at the 63-bit limit p < 54 305, extrapolation unmeasured), so
  genus 4 can be taken to about 2^15.7 with the routine as is.

## Part 3: extension run (genus 3, 3 curves × 2 seeds, arms cR + SGE(8,3))
Command as in the protocol, with the declared `--rho-cap 3e9`; the first call covered 65537 and 131101 (383 s
wall), the second 262147 (710 s; the condition "131101 cell under two hours" held). 54 runs in the three prime
cells, all certified with the planted t. 2LP + prefilter and SGE rows come from the same driver call.

| p | 2LP+pre / rho | 2LP+pre+SGE(8,3) / rho (FPM primary) | order search per admitted instance (cell wall / 3) | merged-arm solver wall per run |
|--:|--:|--:|--:|--:|
| 8209 | 0.591 [0.459, 0.846] | 0.290 [0.224, 0.428] | – | – |
| 16411 | 0.741 [0.534, 1.177] | 0.383 [0.266, 0.726] | – | – |
| 32771 | 0.537 [0.422, 0.687] | 0.283 [0.221, 0.361] | – | – |
| 65537 | 0.745 [0.596, 1.218] | 0.416 [0.336, 0.678] | 63.3 s (157 curves tried) | 7.0 s |
| 131101 | 0.850 [0.590, 1.385] | **0.507 [0.354, 0.823]** | 20.3 s (21 tried; 2.8 s for the admitted orders alone) | 22.0 s |
| 262147 | 0.460 [0.261, 0.973] | 0.293 [0.167, 0.623] | 44.0 s (23 tried) | 56.4 s |

Rho wall per run: 11.7, 29.5, 134 s. The SGE arm is below rho under the FPM band at all six primes; under the GAE
band the verdict stays unresolved (`w = 1`: 9–17× rho; `w = 0`: 0.05–0.10). Exponents of the FPM-primary cost in p:
over 8209 … 131101 (five primes): unmerged 1.349 [1.345, 1.353], SGE arm 1.418 [1.411, 1.425], rho 1.243 [1.069, 1.421];
over all six primes: SGE arm 1.422 [1.417, 1.427], rho 1.369 [1.183, 1.536].

## Scorecard
| id | prediction | outcome |
|---|---|---|
| O1 | every run certifies | held (54/54 in the extension, 0 not certified in the 144 joined runs) |
| O2 | SGE arm over rho <= 0.25 at 131101 and below 0.283 | **falsified**: 0.507 [0.354, 0.823] at 131101; the ratio rose from 0.283 (32771) to 0.416 (65537) and 0.507, then 0.293 at 262147 |
| O3 | exponents over 8209 … 131101: SGE in [1.30, 1.42], rho in [1.40, 1.55] | SGE held (1.418, upper edge); **rho falsified** (1.243 [1.069, 1.421], below the interval) |
| O4 | order search per admitted instance cheaper than the solver run at 131101 | held narrowly at 131101 (20.3 s vs 22.0 s per run; vs 44 s per instance's two runs; 2.8 s for admitted orders alone) and at 262147 (44.0 vs 56.4); **not at 65537** (63.3 s vs 7.0 s, 154 rejected curves) |
| O5 | earlier instances reproduced | held (9/9) |

## Reading
The order routine is no longer the bottleneck of the genus-3 study: 27 s per order at 32771 became 0.56 s, and
the whole extension to 262147 ran in minutes. The measured genus-3 ratio of the SGE arm does not keep falling:
0.28–0.52 across 8209 … 262147, with a rise to 0.507 at 131101 and an interval at 262147 that spans 0.17–0.62.
Two things limit the reading: three curves per prime give wide intervals (the rho mean at each prime rests on 6 runs, and
its exponent interval [1.07, 1.42] is too wide to separate it from the SGE arm's 1.418), and the SGE arm's exponent (1.42)
is above the rho exponent point estimate over five primes (1.24), which is the opposite of what a falling ratio needs.
Whether the unmerged and merged exponent drift is real or rho noise needs more curves per prime; I did not run them.
Distance remaining to rho at the largest prime: SGE arm 0.29 [0.17, 0.62] under FPM, about 9× under the GAE `w = 1` band.

## Limits
Three curves per prime, two seeds; shared 4-vCPU host (walls are not isolated; order walls are single-thread,
solver walls with four threads running); no independent replay; the order-search-per-admitted figure depends on the
number of rejected curves of the cell (13 to 154); `Bsgs2` equals `Enumerate` in code; genus 4 `Bsgs1` equality not
tested. rustfmt --check clean; `cargo clippy --release --lib --bins --tests` shows no warning in `genus_ladder.rs` or `src/bin/genus_ladder.rs` (other files warn).
