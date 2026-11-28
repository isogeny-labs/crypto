# Genus 4: cheaper IC arms against window-free plain rho at w = 1 — results (2026-11-17)

Status: every number is `[pcc:unverified]` (one local run, no independent replay; no leaderboard or
`benchmarks/README.md` row). Protocol, run script and analysis frozen before any data (`frozen.sha256`);
binary `binary.sha256` (source 662b4c4bb9). After launch and before any result was read, `analyze.py`
was changed to read gzipped data (input only; session log 2026-11-17 ~20:50). Tables `tables.md`,
`analysis.json`, `figure.svg`, `REPORT.pdf`. Data `p8209.jsonl.gz`, `p16411.jsonl.gz` (270 runs).

## Gates

| gate | result |
|---|---|
| G1: every IC run certified, log equal to the planted value (5 arms × 54 runs) | **pass** |
| G2: arm w equals the lean-rho study's arm w in every count, same keys | **pass** (same instrument) |

## Arms against the cheapest rho (window-free plain rho, Q, at every cell), Q3

| arm | p = 8209, w = 0.1 | p = 8209, w = 1 | p = 16411, w = 0.1 | p = 16411, w = 1 |
|---|---|---|---|---|
| w (baseline) | 0.210 [0.168, 0.265] | 2.075 [1.653, 2.623] | 0.163 [0.120, 0.240] | 1.612 [1.187, 2.378] |
| s (+ SGE) | 0.192 [0.154, 0.242] | 1.897 [1.513, 2.395] | 0.151 [0.111, 0.223] | 1.495 [1.101, 2.206] |
| f (+ Ferrari) | 0.145 [0.116, 0.183] | 1.429 [1.140, 1.806] | 0.101 [0.074, 0.148] | 0.994 [0.732, 1.459] |
| **x (+ Ferrari + SGE)** | **0.127 [0.101, 0.160]** | **1.248 [0.995, 1.576]** | **0.089 [0.066, 0.130]** | **0.876 [0.645, 1.284]** |
| z (sieve, 2 LP) | 0.166 [0.133, 0.209] | 1.661 [1.325, 2.094] | 0.178 [0.119, 0.263] | 1.784 [1.190, 2.634] |

Q4 at w = 1 (rho's closed-form hash and coefficient additions also priced): x is 0.312 [0.249, 0.394]
and 0.219 [0.161, 0.321]. By the decision rule (Q3 and Q4 upper < 1) no arm is below rho at w = 1 at
either prime; every arm is below rho at w = 0.1 at both.

## Where the native work goes (paired by key, ratio to arm w)

| arm | p | native | relation phase | linear algebra |
|---|---|---|---|---|
| s | 8209 / 16411 | 0.914 / 0.927 | 1.000 / 1.000 | 0.810 / 0.851 |
| f | 8209 / 16411 | 0.688 / 0.616 | **0.433 / 0.249** | 0.997 / 0.998 |
| x | 8209 / 16411 | **0.601 / 0.543** | 0.433 / 0.249 | 0.804 / 0.850 |
| z | 8209 / 16411 | 0.801 / 1.108 [0.786, 1.535] | 0.297 / 0.310 | 1.410 / 1.938 |

- **The Ferrari quartic test is the lever.** It cuts the relation phase to 0.43× (8209) and 0.25× (16411)
  of arm w, and the saving grows with p, because each test replaces a splitting computation whose cost
  grows with log p.
- SGE trims the linear algebra by 15–19%; combined with Ferrari it gives the best arm (x): 0.60× and 0.54×
  of arm w's native work.
- The sieve cuts the relation phase as far as Ferrari does (0.30×) but its linear algebra grows 1.4–1.9×
  (more, wider relations), and at 16411 its total is above arm w's on average with a wide interval.

## Scaling (derived after the frozen analysis; point estimates only)

Arm x's Q3 w = 1 ratio falls from 1.248 to 0.876 between p = 8209 and 16411, a log-log slope of −0.51
(arm w: −0.36), close to the double-large-prime expectation of −0.5 at genus 4. Taken at face value the
point estimates cross parity near p ≈ 1.3 × 10^4; the 16411 interval [0.645, 1.284] does not exclude
parity, so this is not a below-rho result.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | z's native / w's in [0.60, 0.85] at both primes | **held at 8209 (0.801); failed at 16411 (1.108)** |
| P2 | s's LA / w's LA in [0.35, 0.70]; total in [0.85, 1.00] | **LA failed (0.81, 0.85); total held (0.914, 0.927)** |
| P3 | f's relation phase / w's < 1 at both primes | **held (0.433, 0.249)** |
| P4 | best arm: above rho at 8209 (lower > 1); 16411 point in [0.9, 1.5] | **failed on the good side at both: x's 8209 lower end is 0.995; 16411 point 0.876** |
| P5 | every arm below rho at w = 0.1, both primes | **held** |

## What this means

- Measured improvement over the index-calculus baseline: arm x cuts arm w's native work to 0.60× / 0.54×
  and its distance to window-free plain rho at w = 1 from 2.08× / 1.61× to 1.25× / 0.88× (point estimates).
- Distance remaining at w = 1: 1.25× at p = 8209; at p = 16411 parity is inside the interval. At
  w ≤ 0.1 (and on measured timings) the IC is far below rho at both primes.
- Next: arm x with more targets at 16411 to tighten the interval; Ferrari inside the sieve (the sieve's
  relation phase with Ferrari's test); the sieve's LA blow-up.

## Deviations

Local host (no AWS identity, not isolab-eks); session kept active during the run (owner choice); the
`analyze.py` input change above; four files of this program's code restored from 3384161241 and one
compile fix in `f4_fp.rs` (both in this PR, before the binary was built).
