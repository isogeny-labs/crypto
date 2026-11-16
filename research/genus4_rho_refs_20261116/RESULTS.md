# Genus 4, p = 8209: cheapest rho, symmetric pricing, more targets — results (2026-11-16)

Status: every number is `[pcc:unverified]` (one local run, no independent
replay of this study yet, so no leaderboard or `benchmarks/README.md` row).
Protocol and binary frozen before data (`frozen.sha256`, `binary.sha256`,
commit `a99e7267f1`). `analyze.py` was committed after the run was launched and
before any of its data was read; it implements the frozen readings only. Data
`run.jsonl` (54 runs + 3 calibration records), tables `tables.md`,
`analysis.json`, figure `figure.svg`, report `REPORT.pdf`.

**Scope:** three synthetic genus-4 hyperelliptic Jacobians over F_8209 with
`h ≤ 2` (the r-bound instances), 3 targets each (9 independent targets), 2
solver seeds. A positive control of a known asymptotic effect (index calculus
beats rho on genus-4 hyperelliptic Jacobians); not an ECDLP result.

## Gates

| gate | result |
|---|---|
| G1: arm w and negation rho rows for the first target and seeds 1–2 equal the genus-4 study's rows | **pass** |
| G2: every log certified and equal to the planted value (w, R, P; 54 runs) | **pass** |

## The rho reference (red-team F1)

Plain rho costs 1.30× negation rho at w = 0 and 1.30–1.35× at w = 1 in every
reading: **negation rho is the cheapest registered rho everywhere**, so the
genus-4 study's reference was the right one. Per step, both walks do about one
group operation, one canonicalisation (floored) and **about 18 counted
window/Brent compares** (negation 17.9, plain 18.0), which the earlier band
left unpriced.

## Arm w against the cheapest rho, by reading

| reading (protocol §3) | w = 0 | w = 0.1 | w = 1 | below at every price |
|--|--|--|--|--|
| Q1: multiplications only (as the earlier studies) | 0.003 [0.002, 0.005] | 0.088 [0.064, 0.133] | **0.469 [0.341, 0.709]** | **yes** |
| Q2: IC multiplications and additions; rho as emitted | 0.003 | 0.251 [0.183, 0.380] | 1.367 [0.995, 2.068] | **no** (w = 1 unresolved) |
| Q3: every emitted native operation on both sides | 0.003 | 0.098 [0.071, 0.148] | 0.142 [0.102, 0.215] | yes |
| Q4: Q3 plus rho's un-emitted hash and coefficient additions | 0.003 | 0.089 [0.064, 0.134] | 0.123 [0.089, 0.187] | yes |
| Qcal: measured timing (w_mul,p 0.0028, w_add,p 0.0009, w_mul,r 0.0046 per Cantor addition) | – | 0.008 [0.006, 0.012] | – | yes |

Leave one curve out at w = 1: Q1 0.40–0.66 (upper ends 0.68–0.97), Q2
1.16–1.91, Q3 0.12–0.20, Q4 0.10–0.17.

### Reading it

- At **w = 0 and w = 0.1** arm w is below rho under every reading (largest
  upper end 0.380, in Q2).
- At **w = 1** the answer depends on which native operations the band prices:
  - multiplications only (Q1), or every operation on both sides (Q3, Q4), or
    the measured timings (Qcal): below rho;
  - the IC's additions priced but rho's compares not (Q2): unresolved
    (1.367 [0.995, 2.068]).
- Q3 and Q4 lean on rho's ~18 compares per step. Those compares are an
  implementation choice of the lab's rho walk (its short-cycle window and
  Brent check); a leaner rho could do fewer, so Q3/Q4 measure this rho
  implementation as much as rho itself. Q1 is the reading that depends least
  on such choices, and it is below rho at every price.
- **Owner decision (2026-11-16): the band's primary reading is Q3**, every
  emitted native operation on both sides (AGENTS.md §5a, bullet "Native
  operation means every emitted native operation, on both sides"). Under it:
  **arm w is below the cheapest rho at w = 0, 0.1 and 1 at p = 8209 for these
  instances**: 0.003 [0.002, 0.005], 0.098 [0.071, 0.148], 0.142
  [0.102, 0.215]; leave-one-curve-out upper ends at w = 1 are 0.20–0.30.
  The other readings are sensitivities: Q1, Q4 and Qcal agree; Q2 (IC
  additions priced, rho's emitted compares not) is unresolved at w = 1.
  Caveat carried with the verdict: Q3 prices this rho implementation's ~18
  compares per step; a leaner rho walk would narrow the w = 1 margin.
  Positive control of a known effect; `[pcc:unverified]` until an independent
  replay.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | P/R at w = 0 in [1.25, 1.6]; Q1 P/R ≥ 1; in Q3/Q4 at w = 1 P cheaper | **partly failed**: 1.303 and ≥ 1 held; P is not cheaper in Q3/Q4 (1.351): plain rho does as many compares per step as negation |
| P2 | w below the cheapest rho at w = 0, 0.1 in every reading | **held** |
| P3 | at w = 1: Q1 below, Q2 unresolved or above, Q4 below | **held** |
| P4 | Qcal below | **held** |

## Remaining limits

One shared host (the calibration is informational except in Qcal); p = 8209
only; 3 curves; the instance family (`h ≤ 2`) and the r-bound selection are
inherited; no independent replay of this study yet.
