# Errata (after the validator report, `benchmarks/20261118-genus4-prime-order/validator-report.md`)

PROTOCOL.md stays frozen. The numbers in `analysis.json` are unchanged; the validator recomputed all
187 cells exactly.

- **E1 (F2, "fully charged" overstates it).** The IC walk's per-step partition hash and two
  coefficient additions are not counted. Charging them at w = 1 moves the ratios as follows:
  - x\* at p = 16411: from 0.8759 to 0.8848, still below the ideal negation bound;
  - x at p = 16411: from 0.9987 to 1.0040, an arm already reported as not below.
  So the counts are "every emitted native operation charged". Uncounted walk bookkeeping is
  disclosed and does not change any verdict.
- **E2 (F4, exponent and novelty wording).**
  - **Exponents.** "IC cost grows as p^1.62 / p^1.54" is a two-point slope between p = 16411 and
    p = 32771, not an exponent fit.
  - **Novelty.** "What is new here" is an unchecked novelty claim, because research-kb was
    unavailable. Read it as "this lab's implementation measured at these sizes", nothing more.
- **E3 (F5, linear-algebra share).** The "56–60%" figure is linear algebra over the total w = 1
  cost. Over native work it is 56.2–60.6%.
- **E4 (F7, slope interval).** The D2 slope interval reused one bootstrap seed at both primes, so
  the two bootstraps were not independent. With independent streams:
  - x: [−0.3847, −0.3699];
  - x\*: [−0.4612, −0.4463].
  The point estimates are unchanged.
- **E5 (F1, logging order).** D2 was logged after its data. It records the extension the frozen
  rule already required. Its one real deviation, stopping the 2601–3000 block, cannot change the
  five smallest seeds.
- **Notes (F6, F8, F9).**
  - cc\* = 1.4 lies at the edge of the tuning grid.
  - No calibrated w was measured; the band 0 / 0.1 / 1 is the convention.
  - The bench record is still open.
- **Margin.** The §5a order for x\* holds up to 1.14 GAE per native operation at p = 16411 and up
  to 1.56 at p = 32771. Pricing mod-r operations like F_p operations is not material: x\* would
  reach the bound only if a mod-r operation cost more than 1.25 / 1.93 GAE. One Jacobian
  operation costs about 394 F_p multiplications.
