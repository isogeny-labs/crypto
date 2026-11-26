# Results: skipping the full-core fallback in quartic Diem `2lp-sfb-sge`

Every number here is **[pcc:unverified]**. The run is on one shared host with planted targets, and there is no validator report yet.

## Binding

- **Protocol.** PROTOCOL.md (sha256 `7caf53a0c768…`) was frozen in 8297532db36 before any grid run, together with `seeds.json`, `analyze.py` and the run scripts.
- **Binary.** `genus_ladder.nofb`, sha256 `55ebb440…31602`, built from the tree committed as 31a1e2f53ef (`source.sha256`).
- **Data.** `raw/run.jsonl` and `raw/run131101.jsonl` (`data.sha256`). The analysis is `analysis.json` (sha256 `a9575d5fb1e2…`).
- **Bench record.** `benchmarks/records/20261126T0333Z-quartic-diem-nofb.json`.
- **Runs.** 2026-11-26, 03:33 to 04:03 UTC, at most 2 threads, run locally (PROTOCOL §7).

## Integrity (all pass)

- 160 runs: 80 per arm, all paired. Every run certified, with `recovered == t_secret`, and both arms recover the same log.
- **C0.** The baseline rows for seeds 1 and 2 are identical to the parent study's `2lp-sfb-sge` rows on all 24 compared fields (40 of 40).
- **Pair identities (PROTOCOL §2).** No violations on any of the 80 pairs:
  - same attempts and relation phase in both arms;
  - equal linear algebra and group operations where there was no fallback;
  - strictly less linear algebra where there was one.

## Rescues

The baseline fell back 16 times, on 15 pairs. **The fallback certified none of them: 0 rescues.** The new arm skipped the same 16 fallbacks, and no pair needed a different number of attempts.

## Decision (PROTOCOL §5, primary panel = held-out seeds 3, 4): **IMPROVEMENT [pcc:unverified], order holds across the band**

Pooled paired GAE_w ratio, `nofb` over `sge`, cluster bootstrap 95%:

| w | held-out seeds 3–4 | all seeds 1–4 |
|---|---|---|
| 1 | **0.833 [0.758, 0.907]** | 0.826 [0.773, 0.881] |
| 0.1 | 0.833 [0.761, 0.908] | 0.826 [0.771, 0.880] |
| 0 | **0.9971 [0.9959, 0.9984]** | 0.9972 [0.9962, 0.9980] |

The upper bound at w = 0 is below 1. The skipped fallbacks also drop their failed `[t]G = T` checks, so the order holds across the whole band, not only under a tolerance.

Per prime at w = 1, all seeds:

| p | 4099 | 16411 | 65537 | 131101 |
|---|---|---|---|---|
| GAE ratio | 1.000 | 0.683 [0.571, 0.783] | 0.903 [0.814, 1.000] | 0.755 [0.623, 0.897] |
| `la` ratio | 1.000 | 0.592 | 0.872 | 0.700 |

At 4099 neither arm ever fell back, so the arms are identical there. On the 15 fallback pairs the new arm's linear algebra is 20.2–27.3% of the baseline's.

## Preregistered predictions (PROTOCOL §4)

1. No rescue: **held**, with 0 of 16.
2. Equality where the baseline has no fallback: **held**, with no identity violation.
3. Linear algebra 15–40% of the baseline on fallback pairs: **held**, at 20.2–27.3%.
4. Pooled held-out GAE₁ ratio in [0.60, 0.95]: **held**, at 0.833.

## IC / ideal negation rho (GAE_w, 5 curves × 4 seeds, boot95)

| p | sge w=0.1 | nofb w=0.1 | sge w=1 | nofb w=1 |
|---|---|---|---|---|
| 4099 | 0.886 | 0.886 | 8.83 | 8.83 |
| 16411 | 0.756 | 0.517 [0.472, 0.572] | 7.55 | 5.16 [4.72, 5.73] |
| 65537 | 0.298 | 0.269 [0.264, 0.275] | 2.98 | 2.69 [2.63, 2.74] |
| 131101 | 0.263 | 0.199 [0.184, 0.217] | 2.63 [2.04, 3.47] | **1.98 [1.84, 2.16]** |

- At w = 0, IC/rho is about 2e-4 at 131101 for both arms.
- Every prime is **UNRESOLVED** for both arms: below rho at w = 0, above it at w = 1.
- **Distance remaining at 131101, w = 1: 1.98×.** It was 7.75× for `2lp-sfb` in the parent ladder, then 2.78× (parent) and 2.63× (this run) for `2lp-sfb-sge`.

## What is left

- **Remaining cost.** With the fallback gone, a run that needs a second attempt costs about twice a single-attempt run. Those runs exist because attempt 1 leaves t undetermined (parent ERRATA E6).
- **Next lever.** Avoid the undetermined first attempt. For example, test whether the t column is reached before solving, or collect a few more relations at the first trigger. That is a new protocol.
- **Scope of the claim.** This is a component speed-up of the index-calculus baseline [pcc:unverified], not a rho claim. The curves and seeds 1–2 are reused from the parent study, as declared; seeds 3–4 are held-out.

## Figure

`fig_nofb.svg` and `fig_nofb.png`. Report: `reports/run-20261126T0333Z/report.pdf`.
