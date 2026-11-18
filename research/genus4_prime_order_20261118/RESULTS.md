# Results: genus 4 on prime-order Jacobians against the ideal negation-rho bound

Every number is `[pcc:unverified]`. The study was run once, on a local 4-core host, and no validator
has replayed it yet. The protocol, frozen before data, is `PROTOCOL.md`. The deviations, each logged
before its data was read, are in `DEVIATIONS.md`. The tables come from `analyze.py` (frozen).

## Headline

On prime-order genus-4 Jacobians, 5 curves per prime with 60 paired runs per arm and prime, these
are the IC costs over the ideal negation-rho bound `sqrt(pi r / 4)`. That bound charges nothing for
negation canonicalisation or fruitless cycles. It is the strongest rho reference there is.

| p | arm | w = 0 | w = 0.1 | w = 1 | below at all w (D1) |
|---|---|---|---|---|---|
| 16411 | x (cc 2.0) | 0.002 | 0.101 | 0.999 [0.995, 1.002] | no (parity at w = 1) |
| 16411 | **x\* (cc 1.4)** | 0.003 | 0.090 | **0.876 [0.872, 0.880]** | **yes** |
| 32771 | x (cc 2.0) | 0.001 | 0.078 | **0.769 [0.767, 0.772]** | **yes** |
| 32771 | **x\* (cc 1.4)** | 0.002 | 0.066 | **0.640 [0.638, 0.641]** | **yes** |

- **Below the strongest rho, fully charged.** Arm x\* is below the ideal negation-rho bound at all
  three prices (w = 0, 0.1 and 1 GAE per native F_p or mod-r operation) at both primes, so the §5a
  order holds across the band.
  - Arm x\* is the double large prime walk with the parity prefilter, coordinate Wiedemann, the unit-T
    walk, Ferrari and SGE, at factor-base constant 1.4.
  - Arm x, at the old constant 2.0, is at parity at p = 16411 and below at p = 32771.
- **Every curve agrees.** The per-curve ratios are within 0.01 of each other: x\* at w = 1 is
  0.870–0.879 at p = 16411 and 0.640–0.641 at p = 32771. The IC cost hardly depends on the curve,
  and the reference depends only on r.
- **Exponent.** The D2 slope of ln(IC / ideal negation) in ln p at w = 1 is −0.377 [−0.384, −0.371]
  for x and −0.454 [−0.462, −0.446] for x\*. IC cost grows as p^1.62 (x) and p^1.54 (x\*); rho grows
  as p^2. The theory for double large primes in genus 4 is p^(2 − 2/g) = p^1.5.
- **Tuning (step 2).** x\* / x at w = 1 is 0.877 [0.873, 0.881] at p = 16411 and 0.832
  [0.829, 0.835] at p = 32771. The smaller factor base cuts linear algebra from 77–81% of native work
  to 56–60%, and the gain grows with p.
  - The tuning used two held-out cofactor-2 curves and never saw the confirmation curves.
  - At p = 8209 (previous study) cc = 2.0 was optimal. The optimum falls with p, as the linear-algebra
    share rises.

## What this is and is not

- **Known in theory, now measured.** That index calculus beats rho on genus-4 hyperelliptic Jacobians
  over prime fields is known asymptotically. Double large prime IC is Õ(q^(2 − 2/g)) (Gaudry,
  Thomé, Thériault, Diem 2007), while rho is Õ(q^(g/2)). What is new here is this lab's
  implementation crossing the strongest rho reference at concrete sizes (#J ≈ 2^56 and 2^60). Every
  native operation is charged at up to 1 GAE each, and every run is certified.
- **Not an elliptic-curve result.** It is a genus-4 hyperelliptic result. It belongs on the board as
  a hyperelliptic transfer target (SUCCESS_CRITERIA tier note), not as an ECDLP improvement.
- **The reference is a closed form.** The ideal negation bound is a lower bound on any rho with the
  negation map. Measured plain rho at p = 16411, 5 runs, gave 0.80× `sqrt(pi r/2)`; 5 runs give a
  sanity check only (G3). The earlier study measured 1.004× over 108 runs.
- **w = 1 is the pessimistic end for IC.** A genus-4 Cantor addition costs hundreds of F_p operations,
  and pricing each native operation at 1 GAE overcharges IC. Linear algebra works mod r (56–60-bit),
  and those operations are priced the same as F_p operations, which is a convention and not a
  measurement.

## Predictions (frozen, PROTOCOL §7)

| # | prediction | outcome |
|---|---|---|
| P1 | x at p = 16411 vs ideal negation, w = 1, in [0.90, 1.10] | **holds**: 0.999 |
| P2 | x at p = 32771 in [0.60, 0.85], and D1 holds | **holds**: 0.769, below at all w |
| P3 | D2 slope in [−0.8, −0.3] | **holds**: −0.377 |
| P4 | non-baseline tuning, and x\* / x ≤ 0.95 at both primes | **holds**: cc 1.4; 0.877 and 0.832 |
| P5 | G1–G3 pass | **holds** |

## Gates and deviations

- **G1.** All 240 IC runs are certified and correct.
- **G2.** All 10 confirmation curves have h = 1.
- **G3.** Measured rho mean over `sqrt(pi r/2)` is 0.80, inside [0.6, 1.6].
- **D1.** The p = 32771 search was stopped at the 2 h limit. Residue 3 was re-scanned in chunks.
- **D2.** Extension blocks under the frozen rule:
  - 1801–2200: nothing found.
  - 2201–2600: found 2374, the fifth curve.
  - 2601–3000: stopped as unneeded.
- **Prefilter.** Seeds with reducible f have even #J and are skipped before point counting.
  `f_irreducible` is unit-tested against computed orders.

## Sources checked

- `research/genus4_followup_20261118`: the validated arm-x result, and finding F7, the per-curve split.
- `research/g2plus_leads_20261116/L03_base_rebalance/ASSESSMENT.md`: genus-3 base rebalance
  (cc\* = 1.0).
- research-kb is unavailable here (CLI not installed); the literature reference above is the
  standard double-large-prime analysis.
