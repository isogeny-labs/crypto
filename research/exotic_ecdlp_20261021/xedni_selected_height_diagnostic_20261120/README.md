# Selected rational xedni curves have rank at least two

The three rational curves selected by the frozen two-point xedni lift census each have a recorded Mordell–Weil rank lower bound of at least two. The two-point rank-gap premise `d<r` in the checked [varying-family height obstruction](../xedni_mazur_torsion_20261120/README.md) therefore does not hold for this selection policy on its accepted 20-pair panel. All three rational torsion groups have order one, and the twelve distinct selected points have canonical heights between 2.6364 and 4.7782 in Sage's normalization.

| Curve | Rational model `y²=x³+Ax+B` | Accepted pairs | Selected points | Sage torsion order | Rank evidence |
| --- | --- | ---: | ---: | ---: | --- |
| C0 | `A=−1129`, `B=−13679` | 4 | 4 | 1 | Simon `(3,3)`; proof-enabled PARI and mwrank both return 3 |
| C1 | `A=−847`, `B=8035` | 8 | 4 | 1 | Simon `(3,3)`; proof-enabled PARI and mwrank both return 3 |
| C2 | `A=−9478/31`, `B=64516/31` | 8 | 4 | 1 | Simon `(3,3)` under four known-point inputs; proof-enabled PARI reports lower 2, upper 3 at default and effort 3; mwrank censored at 90 s |

The C2 rank methods differ in how far they certify the third independent point. The common conclusion needed here is the lower bound `d≥2`; the 90-second mwrank cap and both PARI errors remain result rows. Simon's `(3,3)` interval is stable with no known points, one, two, and all four selected points (two inverse pairs). Its returned point list contains two points, so this packet does not promote the C2 third rank beyond the method-specific report.

## Height and torsion evidence

The [height result](height-result.json) includes every distinct selected point, both Sage and PARI canonical-height calculations at 128-bit precision, all 20 accepted-pair parallelogram residuals, and each curve's rational torsion order. The two height implementations agree to the displayed precision; the largest parallelogram residual is below `1e−36`. These point heights provide an observed upper bound for the twelve selected points. They do not provide a minimum nonzero height for the full Mordell–Weil lattices or a uniform bound across curves selected at other field sizes.

An [independent integer point recount](torsion-result.json) and [prime-exclusion certificate](torsion-certificate.json) corroborate the trivial torsion calls. On all three curves the first good field prime is 3 and the reduced group has order 7, so a rational torsion prime can only be 3 or 7. For each candidate, the certificate gives another good prime with a group order indivisible by it:

| Curve | Exclude 3 using | Exclude 7 using |
| --- | --- | --- |
| C0 | `#E(F₇)=4` | `#E(F₅)=9` |
| C1 | `#E(F₅)=10` | `#E(F₅)=10` |
| C2 | `#E(F₅)=7` | `#E(F₁₃)=15` |

The arithmetic implication uses [Silverman, *The Arithmetic of Elliptic Curves*, Proposition VII.3.1(b)](https://www.pdmi.ras.ru/~lowdimma/BSD/Silverman-Arithmetic_of_EC.pdf): at a good prime `p`, reduction is injective on torsion of order prime to `p`. The audited point counts and this theorem exclude every torsion prime. The reduction theorem has not been formalized in the Lean modules in this PR.

## Frozen method and claim boundary

The [protocol](protocol.json) fixes the compressed parent census by SHA-256, extracts all 20 accepted rows into exactly three curves and twelve distinct points, and specifies both canonical-height implementations, Simon two-descent rank bounds, and a 90-second cap for each rank arm. The [rank proof amendment](proof-protocol.json), [C2 effort amendment](effort-protocol.json), [Simon input perturbation](perturb-protocol.json), and [torsion certificate protocol](torsion-certificate-protocol.json) were written before their corresponding follow-ups. The accepted Sage launcher runtime is retained in [sage-runtime-info.json](sage-runtime-info.json). The [independent audit](audit-result.json) replays source selection, exact finite-field orders, point-height consistency, and every completed, error, and timeout status.

This is a finite selected-curve arithmetic diagnostic on the existing `p=47`, two-point law. It does not estimate a uniform rank or height distribution for the general xedni curve-selection rule. A next falsifier should freeze a larger-field selection policy with `r` exceeding the selected curve rank, then measure its curve-construction cost, rank interval, torsion, point heights, and admission rate under one target-independent rule. The all-sample theorem also requires a uniform **lower** bound on nonzero free height across that family; the present finite panel supplies no such bound.

No new Lean theorem is claimed in this packet. Its rank conclusions are Sage method results, the finite-field counts are exact independently replayed arithmetic, and the torsion inference uses Silverman's reduction theorem as an explicit external input. The previously checked Lean obstruction remains conditional on its stated group, reduction, rank, norm, and coefficient-budget hypotheses.
