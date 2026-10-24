# Paired check of 04b's arms against 04's recorded IC-2LP arm: genus-3 IC-2LP on the same instances, seeds and counted unit (launch card)

Status: **draft**. Three things block a valid pairing and none is yet resolved: the two studies generate instances by different rules, they use different counted units, and the tool that runs 04b's arms has not been checked against 04's recorded rows.

Question: on one set of instances, seeds and counted unit, do 04b's arms (the rebuilt IC-2LP baseline and its A1 and A2 variants, and rho) reproduce the exponent and per-size constant of 04's recorded IC-2LP arm? Does the departure from 4/3 carry over from the original baseline to the rebuild?

## Sources (every number cited)

- [research/ic_improve_20261014/04-genus3-2lp/README.md](../../research/ic_improve_20261014/04-genus3-2lp/README.md) (04, the original): IC-2LP exponent q^1.385 [1.379, 1.391] over 2^10 to 2^18 (9 sizes, 3 curves each); IC-basic q^1.910; paired difference -0.525 [-0.529, -0.521]; paired ratio 2LP/basic 0.620 [0.604, 0.638] at 2^10 and 0.036 [0.035, 0.036] at 2^18; IC-2LP fit 2^14 to 2^20 is 1.425 [1.422, 1.428]; local exponents rise from 1.25 to 1.46 [1.45, 1.48]; linear algebra q^1.519 [1.512, 1.525]; relation phase only q^1.295 [1.291, 1.299]; row weight from 7.3 to 25.3 per column. Item 7's IC-2LP exponent is q^1.30 at b = 8 to 13, the gate-0 positive control (01 README: "the 1.30 and 1.65 of item 7 become 1.385 and 1.910").
- [research/ic_improve_20261014/04-genus3-2lp/PROTOCOL.md](../../research/ic_improve_20261014/04-genus3-2lp/PROTOCOL.md): instances q = next prime above 2^b + 1 with seed 20261014 * 1000 + 10 b + curve (item 7's formula; b <= 15 curves 0 to 2 are item 7's instances); test curves 0, 1, 2 (curve 3 optional); arms IC-basic, IC-2LP, IC-1LP (kappa1 = 2), A/A (seed + 1), rho (r = 64, negation map). Unit: cost = M_q + ceil(log2 q) I_q + c_ell M_ell, c_ell = 1 primary and 9 sensitivity. Budget: 3 GB per job (prlimit --as), single-threaded, 60 min per job. Decision rule: ADVANCES / REGRESSES / MATCHES on the 95% CI of the exponent difference (at least 4 common sizes); constant level from the paired ratio at the largest common size; settling question with the upper-range slope CI against 4/3 and the trend CI; controls A/A and gate 0.
- 04's recorded per-curve rows: research/ic_improve_20261014/04-genus3-2lp/results/run-20261014/ (for example b10_c0_all.jsonl, b11_c1_all.jsonl; summary.json).
- [research/ic_improve_20261014/04b-genus3-2lp-rebalance/README.md](../../research/ic_improve_20261014/04b-genus3-2lp-rebalance/README.md) (04b, the rebuild): instances q = largest prime below 2^k with seed 20261016, k = 10 to 20, 7 curves per size (test curves 1 to 4, training 5 to 7); unit S = total / sqrt(ell), counting F_q multiplications plus multiplications mod ell; arms BL (kappa = 2, F = ceil(2 q^{2/3})), A1 (kappa tuned per size from {0.5, 0.7, 1, 1.4, 2}), A2 (structured-Gaussian-style pruning and Markowitz merges before Lanczos), rho. Fits: rebuilt baseline q^1.383 [1.382, 1.384] over 2^14 to 2^20 (from the problem record); A1 1.373 [1.372, 1.374]; A2 1.419 [1.417, 1.421]; relation phase 1.332; linear algebra 1.447; row weight 7.8 at 2^14 to 14.4 at 2^20.
- [problems/ic-improve-04-genus3-2lp-study-missing.md](../../problems/ic-improve-04-genus3-2lp-study-missing.md): the original's artefacts are now on main (first unblock met). The second unblock condition is this matched comparison, with design differences named.

## Conflicts this card must resolve before any pairing

1. **Instance rule.** 04 uses q = next prime above 2^b + 1, seed 20261014 * 1000 + 10 b + curve. 04b uses the largest prime below 2^k with seed 20261016. These are different instances. 04b's tool must be run with 04's rule (or 04's instances must be regenerated under 04b's rule and 04 re-run, which this card does not do).
2. **Counted unit.** 04's cost is M_q + ceil(log2 q) I_q + c_ell M_ell (c_ell = 1). 04b's unit is S = total / sqrt(ell). Per instance, S and cost differ by a factor depending on ell; an exponent fit on S and on cost can differ. The pairing converts both to 04's unit, and the conversion is stated in the protocol.
3. **Design differences.** 04b's rebuild has row weight 4.9 to 14.4 and linear-algebra exponent 1.447; 04 has row weight 7 to 25 and 1.52 (04 README; 04b README). The problem record names the differences to be named: large-prime elimination order, stop margin, factor-base order, Lanczos variant.
4. **Quoted figures.** The request quoted "q^1.30" and "0.62 to 0.65x whole-solve". 1.30 is item 7's IC-2LP exponent (04 README: the gate-0 positive control), not 04's relation-phase exponent, which is 1.295 [1.291, 1.299]. No "0.62 to 0.65x whole-solve" line appears in 04's README; the nearest figures are the paired 2LP/basic ratio 0.620 [0.604, 0.638] at 2^10 and the linear-algebra share 0.65 at 2^20 (a share, not a ratio). The decision rule therefore uses the paired per-size ratio table, and the quoted range is re-read from the paired result, not used as an input.
5. **Sizes.** 04's common range for the exponent is 2^10 to 2^18 (IC-basic only reaches 2^18); 04b fits 2^14 to 2^20. The pairing uses the sizes both studies ran, at least four, and reports the 2^19 and 2^20 IC-2LP points separately.

## Gate 0 (before any new arm)

The tool that runs 04b's arms reproduces at least one of 04's recorded rows (one size and curve, IC-2LP, under 04's instance rule): identical instance, k, steps, counters and cost. Failure stops the item; the same rule as 04's gate 0.

## Decision rule

Taken from 04's protocol, applied to 04b's arms against 04's recorded IC-2LP arm, on the paired instances:

- Exponent level: ADVANCES if the 95% CI of Delta (04b arm minus 04 IC-2LP) lies below 0; REGRESSES above 0; MATCHES if it covers 0. Requires at least 4 common sizes.
- Constant level: the paired ratio at each common size; decided at the largest common size.
- TRADES if the two levels point in opposite directions.
- Settling question: "settles at 4/3" if the upper-range slope CI covers 4/3 and the trend CI covers 0; "keeps rising" otherwise.
- The departure from 4/3 carries over to the rebuild if 04b's arms and 04's recorded arm agree in exponent (MATCHES) on the paired instances; if they disagree, A1 and A2 are re-run on 04's tool, as the problem record requires.

The quoted 04 figures are cross-checks. The paired result is the input.

## Resources

Single-threaded, 3 GB and 60 minutes per job, as 04's budget. 04's own jobs reached 40 minutes wall at 2^18 on a shared host. Only 04b's arms are run; 04's recorded arm is read from its records.

## Open inputs

1. The instance rule: which of the two rules the paired check uses, and whether 04b's tool implements it.
2. The conversion between cost and S, stated in the protocol.
3. Gate 0 result (not yet run; no solver run was made for this card).
4. The sizes both studies ran at the same q (by the chosen rule).
