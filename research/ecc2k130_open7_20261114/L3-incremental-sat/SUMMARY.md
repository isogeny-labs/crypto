# L3 — incremental SAT across decomposition targets: NO-CHANGE

The subagent could not write this file, so the main session saved its report. Full write-up:
`RESULTS.md`, `results.json`, `figures/`. All claims `[pcc:unverified]`.

Main-session checks:
- `raw/SHA256SUMS` verifies.
- The frozen PROTOCOL.md hash `2d1b64cd…` matches the version committed in `2416ce330f`
  (08:20Z). The first scored rows were written at 08:36Z.
- The B/A0 conflict geometric means in `results.json` are 0.883, 0.904 and 1.020, matching
  the report.
- Amendment 1 moved C3 from m=29 to m=23 before any C3 data. Amendment 2 cut C2 to 4 arms and
  20 targets *during* the C2 run. C2 therefore carries less weight; it is not the deciding cell.

## Setup
- Harness: Rust, CaDiCaL 3.0.1 through `cadical-sys 0.8.0`.
- Instance: Koblitz y²+xy=x³+1, type-II ONB, factor base HW(x) ≤ w, chained S3 links.
- Every target is enumerated to completion and its model set compared with brute force:
  812 of 812 rows match, 0 censored.
- Arms:
  - A0: cold solve, R folded in as constants (current practice).
  - AA: A0 with clauses shuffled (A/A noise floor).
  - A1: cold solve, template with R as assumptions.
  - B: one persistent solver across targets.
  - BS: B with clauses shuffled (A/A for B).
  - CI: rebuilt each target, keeping only learnt clauses with no target bits.
  - CA: rebuilt each target, keeping all learnt clauses.
  - CK: B flushed every 4 targets.
- Target stream: 40 targets plus a 5-target repeat block, the same for every arm.

## Result: paired geometric-mean conflict ratio [95% CI]

| Config | B/A0 (decides) | AA/A0 | A1/A0 (encoding cost) | B/A1 (reuse) | B/A0 propagations |
|---|---|---|---|---|---|
| C1 m=11 k=2 w=2 | 0.88 [0.84, 0.93] | 1.02 | 2.07 | 0.43 | 1.15 |
| C2 m=11 k=3 w=2 (cut) | 0.90 [0.66, 1.24] | 0.98 | 4.09 | 0.22 | 0.68 |
| **C3 m=23 k=2 w=2** | **1.02 [0.93, 1.12]** | 1.00 | 2.21 | 0.46 | **2.23 [2.04, 2.46]** |

All 39 C3 targets are non-decomposable, the regime of a real relation search.

- **Reuse only pays back what the assumption encoding loses.** Keeping R as assumptions instead
  of constants costs 2.1–4.1× the conflicts. The persistent solver recovers about that much and
  ends level with the cold baseline.
- **Most of the gain is warm heuristic state, not learnt clauses.** At C3, against A1:
  - only target-free learnt clauses kept (CI): 0.76;
  - all learnt clauses kept (CA): 0.69;
  - the whole solver kept (B): 0.46.
- **Learnt clauses are target-specific.** At C3, 93.9% of 3.05M learnt clauses mention target
  bits, and only 6.1% are target-independent. At k=3, where the first S3 link has no target in
  it, 52% are target-independent (B/A0 0.54 on non-decomposable targets, wide CI).
- **§5a band, propagations per target at C3:** A0 costs 1.84e6 at 0.1 GAE and 1.84e7 at 1 GAE;
  B costs 3.80e6 and 3.80e7. B never ranks ahead across the band. Conflicts have no calibrated
  GAE price. There is no end-to-end number, so no gap to rho.
- **Controls:** the A/A arms agree within ±5–10%. The repeat-block positive control passes
  (B/A1 on repeats: 0.44, 0.13, 0.32).

## Deviations
- C3 uses m=23, and 2 is not primitive mod 23. The m=29 smoke run hit the conflict budget on
  its first target.
- C4 (m=53) was dropped.
- The encoder is a direct m²-AND ONB multiplier, not the codegen Karatsuba circuit.
- The solver is CaDiCaL, which has no native XOR, instead of CryptoMiniSat.

## Open threads
1. A cold solve with R as constants but warm phases and scores. The heuristic-state finding
   suggests it could beat both A0 and B.
2. k ≥ 3 at m=23 with all arms. This is the only regime where target-free clauses are common.
3. A second instrument (CryptoMiniSat with native XOR) and a first-model-only metric.
4. The replication of `sat_ladder_scaling_20261110` is still pending. Its large REUSE/FRESH
   effect should be read in light of this lane's heuristic-state finding.
