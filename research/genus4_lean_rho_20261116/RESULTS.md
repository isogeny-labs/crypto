# Genus 4: arm w against lean rho references, p = 8209 and 16411 — results (2026-11-17)

Status: every number is `[pcc:unverified]` (one local run, no independent replay; no leaderboard or
`benchmarks/README.md` row). Protocol, run script and binary frozen before data (`frozen.sha256`,
`binary.sha256`, commit 586146fabc); `analyze.py` committed before any data was read; its extension to
read the resume files (with a duplicate-row identity check) was declared in the session log before any
result was read. Tables `tables.md`, `analysis.json`, `figure.svg`, `REPORT.pdf`.

## Execution and gates

| item | result |
|---|---|
| G1: every log certified and equal to the planted value (w, B, Q; 108 runs) | **pass** |
| G2: arm w rows equal the earlier studies' rows for the same keys | **pass** |
| G3: Q's steps and group operations equal the earlier P rows (24 keys at 8209) | **pass** |
| Determinism across four container restarts | 65 runs present in two files, all identical in every field but wall time |
| p = 32771 | **dropped** (declared before data, session log 2026-11-17 01:58): curve search ~85 s/seed at ~1 % admission; the host is reclaimed when the session idles |

The remaining curves after the restarts ran with the session kept active (owner choice, option 2),
one curve per process (`percurve.sh`, `waves.sh`).

## Arm w against the cheapest rho (B, Q, R), primary reading Q3

| p | targets | w = 0 | w = 0.1 | w = 1 | cheapest at w = 1 |
|---|---|---|---|---|---|
| 8209 | 18 | 0.003 [0.003, 0.004] | 0.210 [0.169, 0.268] | **2.075 [1.675, 2.657]** | Q |
| 16411 | 9 | 0.002 [0.001, 0.002] | 0.163 [0.119, 0.243] | **1.612 [1.180, 2.409]** | Q |

Q4 (rho's closed-form hash and two coefficient additions per step also priced) at w = 1: 0.519
[0.419, 0.664] and 0.403 [0.295, 0.602]. Sensitivity with plain rho's canonicalisation floored at one per
step (the earlier studies' convention): Q3 w = 1 is 1.038 [0.838, 1.329] and 0.806 [0.590, 1.204].

**Verdict (protocol §7).** At w = 0 and w = 0.1, arm w is below the cheapest rho at both primes in every
reading. At w = 1 it is **above** rho in Q3 at both primes (2.08 and 1.61, lower ends 1.68 and 1.18) and
below in Q4; the decision rule needs both, so **w = 1 is not below rho at either prime.**

## Scaling (the question that matters)

Slope of log(arm w / cheapest rho) on log p, Q3, w = 1: **−0.364 [−0.975, 0.294]** (two primes). The point
estimate falls with p, inside the preregistered band, but the interval includes 0: with two primes and 9
to 18 targets the data cannot establish the slope. Extrapolating the point estimate, parity at w = 1 in Q3
would be reached near p ≈ 6 × 10^4 (16411 · 1.612^{1/0.364}); this is an extrapolation, not a measurement.

## The rho references

| arm | p = 8209 (36 runs) | p = 16411 (18 runs) |
|---|---|---|
| Q: plain, no window, steps / √(πr/2) | 1.05 ± 0.09 | 0.98 ± 0.12 |
| B: negation, Brent, steps / √(πr/4) | 1.40 ± 0.12 | 1.83 ± 0.22 |
| B counted compares per step | 3.00 | 3.00 |

Plain rho meets theory. Both negation walks of the lab lose their √2 and lose more at the larger prime
(R: 1.05 → 1.42 in the scale study; B: 1.40 → 1.83 here). Mean Q3 rho cost at w = 1 (GAE): 8209 —
Q 7.3e7, B 3.5e8, R 1.0e9; 16411 — Q 2.7e8, B 1.8e9, R 5.5e9. A negation walk at theory with only one
canonicalisation per step would cost (1 + w)/√2 per plain step, i.e. ≈ 1.41× Q at w = 1 in Q3 (and
≈ 0.71× Q at w = 0) — so at w = 1 even a perfect negation walk would not beat Q; Q is the right w = 1
reference.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | B's compares per step in [2.5, 3.5]; Q's 0 | **held** (3.00; 0) |
| P2 | at w = 1 the cheapest rho is Q in Q3 and Q4 at every prime | **held** |
| P3 | Q3, w = 1: 8209 in [1.6, 2.6]; 16411 in [1.0, 2.0]; 32771 in [0.6, 1.4] | **held** at 8209 (2.075) and 16411 (1.612); 32771 not run |
| P4 | slope in [−0.8, −0.2] | point −0.364 inside; interval [−0.975, 0.294] includes 0: **not established** |
| P5 | at w = 0 and 0.1, below rho at every prime in Q1–Q4 | **held** |

## What this means

- The genus-4 index-calculus advantage holds when a native operation costs at most 0.1 of a Jacobian
  addition (upper ends ≤ 0.27); on measured timings native operations cost ~0.003–0.005 of one (Qcal in
  the earlier studies), far inside that range.
- At the band's top (w = 1, a modular multiplication priced like a whole genus-4 addition), arm w loses to
  a window-free plain rho by 1.6–2.1×; the gap narrows with p at a rate the data cannot yet pin down.
- Next levers on the IC side: the sieve and SGE+coordinate Wiedemann arms (smoke: 0.70–0.74× and ~0.95×
  of arm w's native work) and the fast group law; on the rho side, the negation walk's step inflation.

## Deviations

Local shared host, not isolab-eks; no S3 checkpoint or raw upload; no Conductor client; four container
restarts (resumed by whole-curve re-runs, determinism verified); p = 32771 dropped as above.
