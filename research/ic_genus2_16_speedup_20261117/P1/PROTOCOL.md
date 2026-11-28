# P1 protocol: degree-2 places against the lab baseline, genus 6-10

Status: written after the pilot (`pilot/`, binaries g16c; main run uses g16d, which differs only in the order-computation cost model), before the main run.
Freezing is recorded by `frozen.sha256` (sha256 of this file and `analyze.py`).
All numbers are `[pcc:unverified]`.

## Question

From genus 6 upward, does allowing degree-2 places reduce the counted cost of an
end-to-end, certified hyperelliptic DLP solve relative to the lab's degree-1
double-large-prime arm? What is each arm's distance to matched rho?

- **Arm D:** degree-2 places in the base.
- **Arm E:** degree-2 places as large primes.

## Instrument

`genus_ladder` from `fffa1b1243` plus `instrument.patch` (DESIGN.md), the binary
recorded in `main/CELLS.log` (sha256 prefix), single-threaded.

## Grid

| g | p |
|---|---|
| 6 | 31, 53, 101, 211 |
| 7 | 17, 31, 53 |
| 8 | 11, 17, 31 |
| 10 | 11, 13 (if order admission fits the cell time; otherwise reported as not run) |

- **Per cell:** 3 curves (`--curves 3`, first admitted seeds), 2 target/walk seeds.
- **Arms:**
  - `c`: the lab's degree-1 2LP with the parity prefilter, base constant 2.0.
  - `D` and `E` as above; `E` uses the same degree-1 `nb` as `c`.
  - `R`: executed negation-map rho.
- **Trial caps:** `--max-trials 5e6` (pilot used 2e7; lowered so 6 exhausted `c` runs fit a cell). An exhausted `c` run is reported as a lower
  bound (cost >= the cost at exhaustion), never as a solve.

## Metrics (analyze.py)

- **Primary:** `FPM_1 / rhoE` per arm.
  - `FPM_1 = cantor_mul + 10 cantor_inv + rel_mul + la_mul`.
  - `rhoE` is the expected sqrt(pi r / 4) negation-map steps times the executed
    walk's measured cost per step on the same instance. One executed walk's
    step count varied 0.35-1.7x of its expectation in the pilot.
- **Secondary:**
  - `FPM_g2`: LA products priced at g^2 F_p products.
  - `GAE0`: group operations only.
  - The executed-rho ratio.
- **Speed-up over the baseline:** per (curve, seed), `FPM(c) / FPM(D)` and
  `FPM(c) / FPM(E)`; per cell, the geometric mean over the 6 pairs, with min and
  max. If any `c` run in a cell is exhausted, the cell's speed-up is a lower bound.

## Predictions, stated before the main run

- **P-a.** At every cell, D and E are cheaper than c in FPM_1 (speed-up > 1).
- **P-b.** At g = 6, the speed-up of the best degree-2 arm over c falls as p grows
  from 31 to 211. The falling-factorial penalty on degree-1 bases fades, and
  D's base grows as p^2.
- **P-c.** At g = 6, E is cheaper than D in FPM_g2 for p >= 53 (D's LA is about p^4).
- **P-d.** At g >= 8, D is cheaper than E in FPM_1.

Each prediction is reported as holding, failing or mixed, with the cell numbers.
None is a pass/fail gate for the programme.

## Gates

- (a) Count identity at genus 4, field for field, by the run's binary
  (`gates/p0_identity_*.jsonl`).
- (b) Every reported solve has `certified: true`.
- (c) Relation verification gate (g=3) passed on this patch.

## What the result can and cannot say

It can say: on these curves, at these p, the counted cost of D and E relative to
the lab's arm c, and each arm's distance to matched rho.

It cannot say anything about cryptographic p, where the degree-1 falling-factorial
penalty vanishes and D's p^4 linear algebra dominates. It also cannot say anything
about wall time.
