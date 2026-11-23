# Genus 4 at p = 8209 (3 new curves) and p = 16411 (3 curves) — results (2026-11-16)

Status: every number is `[pcc:unverified]` (one local run, no independent replay; no leaderboard or
`benchmarks/README.md` row). Protocol and binary frozen before data (`frozen.sha256`, `binary.sha256`,
commit 62307fead3). `ANALYSIS_ADDENDUM.md` (window-free plain references, after validator finding B13)
was committed before any data was read; `analyze.py` was written after all runs finished and implements
the frozen readings plus the addendum. Tables: `tables.md`; `analysis.json`; `figure.svg`; `REPORT.pdf`.

## Gates and execution

| item | result |
|---|---|
| G1: every log certified and equal to the planted value (w, R, P) | **pass** |
| G2: admitted instances have h ≤ 2; the 8209 curves are distinct from 363, 393, 407 | **pass** (457, 474, 519; 16411: 172, 180, 2640) |
| Container restart (~21:19) during curve 172 | curve re-run whole (`resume.sh`); its 9 pre-restart rows are identical to the re-run in every field but wall time; 3 missing rows supplied by the re-run |

## Arm w against the cheapest rho, primary reading Q3

| cell | frozen references (R, P) w = 0 / 0.1 / 1 | addendum references (R, P, P0f) w = 1 | + P0u (sensitivity) w = 1 |
|---|---|---|---|
| 8209, new curves (9 targets) | 0.003 / 0.102 / 0.149 [0.115, 0.218] | **1.050 [0.683, 2.006]** | 2.100 [1.365, 4.011] |
| 8209, pooled with rho-refs (18 targets) | 0.003 / 0.100 / 0.145 [0.114, 0.190] | **1.049 [0.785, 1.513]** | 2.099 [1.570, 3.027] |
| 16411 (9 targets) | 0.002 / 0.080 / 0.115 [0.077, 0.225] | **1.153 [0.772, 2.252]** | 2.307 [1.543, 4.504] |

At w = 0.1 against the addendum references: 0.193 [0.125, 0.368] (8209 new), 0.193 [0.144, 0.278]
(pooled), 0.212 [0.142, 0.413] (16411): below rho. Q4 (rho's closed-form hash and coefficient additions
also priced) at w = 1 against P0f: 0.420 / 0.420 / 0.461, upper ends 0.60–0.90. Qcal (measured timings):
0.008, 0.008, 0.006.

**Verdict.** Under the frozen references, arm w is below rho at w = 0, 0.1 and 1 at both primes (P1, P2
held). That verdict does not survive the cheaper reference the validator identified: against plain rho
without its unused window (P0f, exact), **w = 1 is not below rho at either prime** (1.05–1.15, intervals
straddle 1), and with plain rho's canonicalisation count unfloored (P0u) it is about 2.1–2.3× above.
At w = 0 and w = 0.1, arm w is below every reference at both primes. Q4, which also prices rho's hashing
and coefficient arithmetic, stays below at w = 1 against every reference; Q4's rho-side terms are
closed-form estimates, not counters, so Q3 remains the primary reading.

## The negation rho is not the cheapest rho at p = 16411 (P4 failed)

| arm | p | runs | steps / theory (√(πr/4) negation, √(πr/2) plain) |
|---|---|---|---|
| negation rho R | 2053 / 4099 / 8209 / 16411 | 12 / 12 / 42 / 18 | 1.15 / 0.97 / 1.05 / **1.42** |
| plain rho P | 8209 / 16411 | 24 / 6 | 1.04 / 0.89 |

Plain rho meets its theoretical step count; the lab's negation walk needs 1.42× its expected steps at
p = 16411 (about 3.5 standard errors above 1 for 18 runs), so its √2 advantage over plain rho is gone and
plain rho is the cheaper reference there (P/R = 0.67 in Q1). The per-curve P/R scaling of the protocol
rests on two plain runs per curve and ranges 0.13–1.93, so the 16411 cell carries reference uncertainty
that the bootstrap (which holds per-curve ratios fixed) does not show. Cause of the negation walk's loss
is not established here; candidates are fruitless cycles longer than the 16-entry window and walks lost
to the step cap, both of which grow with walk length (dp bits = ⌊bits(r)/4⌋).

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | 8209 new curves: below rho at w = 0, 0.1, 1 in Q3 | **held** under frozen references; **not** at w = 1 against P0f (addendum) |
| P2 | 16411: below rho at w = 0, 0.1, 1 in Q3; w = 1 point in [0.05, 0.20] | **held** (0.115) under frozen references; **not** at w = 1 against P0f |
| P3 | Q1, 16411, w = 1 in [0.20, 0.45] | **held** (0.371) |
| P4 | P/R ≥ 1.2 in every reading at both primes | **failed** at 16411 (P/R 0.67–0.69): the negation walk underperforms |

## What this means for the program

- The genus-4 positive control is robust at w ≤ 0.1 (and on measured timings, Qcal ≈ 0.006–0.008) at
  both primes; at w = 1 it is at parity with a window-free plain rho and above an unfloored one.
- No scaling with p is visible between 8209 and 16411 at w = 1 against P0f (1.05 → 1.15); the frozen
  lean-rho study (`research/genus4_lean_rho_20261116`, three primes, B and Q on every target) is the
  test of the slope.
- The lab's negation rho needs repair before it can serve as the rho reference at larger p.

## Deviations

Local shared host, not isolab-eks; no S3 checkpoint or raw upload; no Conductor client; the scale
study's analysis was written after the run (its protocol named the readings; the addendum named the
extra references before data). The container restart and the resume are recorded above.
