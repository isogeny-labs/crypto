# Trace −14 degree-eight mixed-pole screens

The frozen main screen completed all eight source/carrier representative cells and found zero exact degree-eight maps in six denominator shapes. A separately frozen companion screen found zero exact maps in the seventh shape `L³(x-c)⁴`. The earlier frozen `L⁵(x-c)²` screen supplies the eighth shape needed to close this mixed-pole signature set under the rational branch-point exchange. These finite negative results do not exclude other degree-eight covers or establish a GHS vulnerability.

## Exact curves and receipts

Use K=F3[a]/(a⁵+2a+1). The source equations are `E_b: v²=u³+u²+b`, with b masks 11 (`a²+2`), 202 (`2a⁴+a³+a²+a+1`), 38 (`a³+a²+2`), and 80 (`2a³+2a²+2a+2`); each has K-order 258. Carrier models are `Y_f: y²=f(x)` over F3 with masks 9735 and 17371. In each model `f=L∏Qi`, with one K-linear L and five K-irreducible quadratics Qi.

The main source (`screen.sage`, SHA-256 `5f6a832d6a72346b34f11b6c9cca40ad81f6e9f2fb85100986f1c6cb097c50b4`) was frozen in commit `274c873c759d`. It searched `L(x-c)²`, `L(x-c)⁴`, `L(x-c)⁶`, `LQi(x-c)²`, `LQi(x-c)⁴`, and `L³Qi(x-c)²` for every nonbranch c and all applicable Qi. Each cell represents 35,754,048 logical numerator candidates. The eight cells together represent 286,032,384 candidates; 366,879 passed the point filter. None passed polynomial squarehood. Aggregate measured CPU time was 1,537.03 s. The `audit.py` receipt check passed for all eight cells.

The companion source (`swap_closure.sage`, SHA-256 `b5b8729e78d98ce4967b3f52a3e3a607dc32069546b715dbd4029d6d8f0a53ed`) was frozen in commit `bc3794372a07`. It searched `L³(x-c)⁴`: 58,806 logical candidates per cell, 470,448 across eight. It retained 1,802 degree-eight numerators and found zero exact maps. Aggregate measured CPU time was 6.13 s. The `audit_swap.py` receipt check passed.

All 16 Sage processes exited zero. Both auditors check source hashes, fields, masks, schemas, candidate counts, and per-cell hit counts. The Lean file `DegreeEightMixedPoleCounts.lean` compiled and checks candidate arithmetic plus closure of eight abstract shape signatures under the branch-swap exponent rule. It does not prove the Sage eliminations or the geometric coordinate-change rule.

## Coordinate-orbit scope

The exact carrier orbit receipt in `../ghs_trace_minus14_degree5_other_20261116/CARRIER_ORBITS.json` exhibits twelve monic degree-eleven models in two PGL2(F3) orbits. A coordinate change retaining a monic odd-degree model either fixes the rational point at infinity or exchanges it with the sole finite rational branch point. Affine changes preserve the factor-degree and exponent signature. Under the exchange, a degree-eight denominator `B=L^r D` of degree b becomes, up to nonzero scaling, `L^(8-b) D'`, with the nonbranch factors of D carried to factors of the same degrees and exponents. The eight signatures

`LH², LH⁴, LH⁶, LQiH², LQiH⁴, L³QiH², L³H⁴, L⁵H²`

are closed under that rule. `L⁵H²` was checked in the prior `../ghs_trace_minus14_degree8_quadratic_20261116/` frozen screen. Subject to the stated coordinate transformation, the union excludes these eight shape signatures for all 240 recorded exact equation pairs. The direct receipt evidence is eight representative cells per shape arm; it is not 240 independent runs. This orbit statement applies to the union, not to an isolated non-closed shape.

## Claim boundary and prior work

The complete degree-one-through-seven census gives a computer-assisted minimum-cover interval [8,33] for the 240 pairs. This mixed-pole union removes only eight named degree-eight signatures; [8,33] remains the full interval. There is no explicit trace −14 cover map from these screens, no selected-subgroup norm receipt, and no measured complete GHS/rho cost for this family. The [odd-characteristic direct GHS classification](https://eprint.iacr.org/2015/805.pdf), [Howe's cover bound](https://arxiv.org/abs/2110.04221), and [Joux–Vitse isogeny-assisted transfer](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) are prior art. Exact-equation search found no published match for these masks, but absence of a search result does not establish publication priority or vulnerability.

A distinct next lead is a nonhyperelliptic genus-five carrier in the same F3 isogeny class. LMFDB records the class with polynomial `1+14T⁵+243T¹⁰` as `5.3.a_a_a_a_o`, with a Jacobian and geometric elliptic splitting after degree five. This class record does not supply a nonhyperelliptic curve equation or a transfer map. An explicit carrier, its L-polynomial, and a costed selected-subgroup transfer are required before treating that route as a new family.
