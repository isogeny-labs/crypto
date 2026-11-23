# Prime medium-tier calibration of the matched rho reference

**Preregistered 2026-11-15, before any session ran.** This file, the spec
in [`specs/prime-medium.json`](specs/prime-medium.json), the curves in
[`curves.json`](curves.json) and their generator
[`gen_curves.py`](gen_curves.py) are committed before the first session.
Results go in a separate `README.md` and `sessions/`, never back into this
file. **Status: spec only. Not run.** The run goes through the isolab-eks
route (AGENTS.md §9a) after review.

## Why

Every improvement claim against rho rests on the matched rho reference.
For prime curves the frontier
([`docs/bounds/FRONTIER.md`](../../docs/bounds/FRONTIER.md)) has
`rho.negation`, `rho.plain` and `bsgs.negation` bounds only at the toy tier:
log2 r 15.4, 16.0, 17.0 and 21.0, from
[`research/ecbench_calibration_20261002`](../ecbench_calibration_20261002).
Koblitz curves already have both toy and medium calibration records. The
2026-11-10 lead sweep names this gap as
`LEAD-RHO-matched-rho-calibration` ("the missing prime medium tier"). Its
recorded next step: *"Seal ecbench bounds for rho.negation, rho.plain and
bsgs.negation on prime curves at log2 r 32–48."*

That sweep's stdlib toy run
([`../lead_sweep_20261110/LEAD-RHO-matched-rho-calibration/RESULT.md`](../lead_sweep_20261110/LEAD-RHO-matched-rho-calibration/RESULT.md))
found two things:

- The plain arm shows no excess over the null: excess about 1.00–1.05.
- The negation statistic is degenerate. Its first-repeat count was
  dominated by fruitless-cycle traps (excess 0.60 falling to 0.07), so it
  says nothing about the useful collision rate.

This calibration uses ecbench's walks, which handle cycles: `rho.negation`
with Wiener–Zuccherato look-ahead and cycle escape, and `rho.negation_v2`
with Brent detection and lost-walk counters. It counts genuine solves, so
the negation measurement is meaningful here.

This is a **validation of the measurement instrument at a new size tier**,
not an attack claim. No ratio here is a speedup.

## Boundaries (derived, written before measuring)

All five curves have cofactor 1 and j ∉ {0, 1728}. The automorphism group
is therefore {±1}, A = 2, and the floor is `√(π/4)` ≈ 0.8862 in `S`
(`S` = total group operations / √r). The leading-order expectations, from
[`../ecbench_calibration_20261002/PROTOCOL.md`](../ecbench_calibration_20261002/PROTOCOL.md):

| arm | method | expected `S` (leading order) |
|---|---|---|
| `rho-neg` (reference) | `rho.negation` | `√(π/4)` ≈ 0.8862 |
| `rho-neg-v2` | `rho.negation_v2` | `√(π/4)` ≈ 0.8862, plus charged v2 set-up |
| `rho-plain` | `rho.plain` | `√(π/2)` ≈ 1.2533 |
| `bsgs-neg` | `bsgs.negation` | 1.0 |
| `bsgs-neg-v2` | `bsgs.negation_v2` | 1.0, plus charged v2 set-up |
| `rho-neg-aa` (control) | `rho.negation` | equal to `rho-neg` (A/A) |

Each expectation omits the `O(log r)` set-up. Measured `S` should therefore
exceed it at small r and approach it as r grows.

## Frozen inputs

- **Curves:** [`curves.json`](curves.json), five prime-order curves at
  log2 r = 31.906, 35.292, 39.244, 43.203 and 47.588. They come from
  `gen_curves.py` with seed 20261115: the first (p, a, b) drawn with
  #E(F_p) prime and j ∉ {0, 1728}. Orders are by PARI SEA (`ellcard`).
  Each generator G was checked with independent pure-Python arithmetic
  (G on the curve, [r]G = O).
- **Workloads:** 8 independent single-target workloads per curve,
  `target_seed` 20261115.
- **Measurement:** 2 interleaved rounds (`order: alternate`), warmup 0,
  seed 20261115, `isolation_required: L2`, 3600 s per method run.
- **Sizes:** five, which is at least the four a scaling bound needs
  (`ecbench bound fit` gives an exponent bound only with ≥ 4 sizes).

## Predictions (fixed in advance)

1. **Convergence to the floor.** For `rho-neg`, the mean `S` falls toward
   0.8862 as r grows. At the largest curve it lies in [0.85, 1.20], and the
   95% interval of the mean contains a value within 1.15× of 0.8862.
2. **Plain and negation ratio.** `S(rho-plain) / S(rho-neg)` lies within
   [1.25, 1.60] at every size. Theory is √2 ≈ 1.414, with negation set-up
   making the low end plausible at small r.
3. **BSGS.** `S(bsgs-neg)` lies within [0.90, 1.30] at the largest curve.
4. **v2 accounting.** At every size `S(rho-neg-v2) ≥ S(rho-neg)`, because
   v2 charges set-up the frozen id omits, and the gap shrinks as r grows.
5. **A/A noise floor.** `rho-neg` and `rho-neg-aa` agree within their
   intervals at every size (|z| < 3).
6. **Scaling.** The fitted exponent α for `rho-neg` over the five sizes has
   a 95% interval containing 0.5.

## Interpretation (fixed in advance)

- **All six hold:** the matched rho reference is calibrated at the medium
  tier. Seal `ecbench bound` records for each arm and rebuild the frontier.
- **Prediction 1 or 6 fails** (the reference does not converge to the
  floor, or α is not 0.5): the reference itself is suspect at the size where
  claims are made. Stop. Do not seal; open an instrument issue.
- **Only prediction 2, 3 or 4 fails:** record the failing cell. Seal the
  arms that passed, mark the failing one, and preregister a follow-up (as
  `PROTOCOL-2.md` did for the 2026-10-02 round).
- **Prediction 5 fails** (A/A disagree): the noise floor is too high and the
  session is void. Rerun with more targets before reading any other cell.

## Inadmissible after data

Changing the curves, seeds, targets, arms, methods, unit, accounting
version, rounds or any threshold above after a session has run.

## Plan check (done before commit)

`ecbench plan --spec specs/prime-medium.json` (output in
[`plan.txt`](plan.txt)) gives spec `ECS1he8e77a3c8c63` with 480
executions. All six arms resolve to method ids; the A/A control shares
`ECM1hefea4e0ebe79` with the reference. There are 40 workloads (5 curves × 8
targets), and every curve reads A = 2, matching the `√(π/4)` floor above.
All five curve slugs read `registered=false`, which is the first step of the
run route below.

## Run route (not taken here)

1. Register the five curves (AGENTS.md §11) and run
   `ecbench plan --spec specs/prime-medium.json`, checking every slug is
   `registered=true`.
2. Fill `.resume/RESUME.md` (`scripts/checkpoint.sh init`).
3. Open a `bench.record/v1` (`python3 tools/benchrec.py init`).
4. Freeze the spec into an isolab-eks request on an image built from the
   tested commit. `experiment-director` writes the task card,
   `executor-mechanical` runs it, and the director assesses the run.
5. Audit with exact replays, fit bounds (`ecbench bound fit`, then
   `bound check`), rebuild the frontier, and close the benchmark record and
   the progress report in the same PR.

[pcc:unverified]: nothing here has run; this file records a plan.
