# Trace −14: first degree-eight cover construction screens

16 November 2026. The first degree permitted by the preceding complete degree-seven screen is eight. Three frozen degree-eight denominator shapes were searched across all eight source/carrier orbit representatives. All 24 source-bound receipts have **zero exact cover identities**. This is a shape-specific null result; the overall minimum-cover interval remains `[8,33]` for the 240 recorded equation pairs, subject to the prior computer-assisted finite and geometric evidence.

## Exact objects and shapes

Use `K=F3[a]/(a^5+2a+1)`, `E_b: v²=u³+u²+b`, and `Y_f/F3: y²=f(x)`. The four source masks `11,202,38,80` and two carrier masks `9735,17371` represent the 20 source and 12 carrier equations in the [degree-seven census](../ghs_trace_minus14_degree7_orbits_20261116/RESULT.md). Each source has 258 K-points. Let L be the unique linear branch factor of f over K and let H be monic and coprime to f. The exact identity for a map is `f C²=B(A³+A²B+bB³)` with `u=A/B`, `deg A=8`, and `gcd(A,B)=1`.

| Frozen protocol | Denominator B | Parameter coverage per cell | Final exact maps across eight cells |
|---|---|---:|---:|
| [quadratic](PROTOCOL.md) | `L H²`, squarefree quadratic H | 243 branch assignments × 243 first-coefficient specializations | 0 |
| [L-cubed quadratic](L3_PROTOCOL.md) | `L³ H²`, squarefree quadratic H | same | 0 |
| [L-fifth linear](L5_PROTOCOL.md) | `L⁵(x-c)²`, `L(c)≠0` | 243 branch assignments × 242 values of c | 0 |

The [audit](AUDIT.json) verifies all source hashes, complete final progress records, exact masks/order, and zero hit lists. [DegreeEightCounts.lean](DegreeEightCounts.lean) checked the finite cell and parameter counts with Lean exit zero; it does not formalize the elimination. The first two arms enumerate every root of the degree-eight x⁹ coefficient equation, including zero-polynomial fallbacks; the third enumerates c directly. Necessary point-square tests precede exact divisibility and polynomial squarehood. Across eight cells, the `L H²` arm had 579 final divisible candidates and `L³ H²` had 581; every one failed polynomial squarehood. The `L⁵(x-c)²` arm had two divisible candidates, both nonsquare. These counts are search-stage counts, not DLP operation counts. The elimination and square tests each use one Sage implementation; the receipt audit is structural and does not independently replay the algebra.

## What remains open

This search does not rule out other degree-eight denominators, including `L H²` with cubic H, mixed quadratic branch poles, or more general correspondences. It provides no coordinate map, evaluated subfield norm, decomposition run, or same-target GHS/rho measurement. The exact 59-bit subgroup identified in the prior trace −14 report remains a transfer target if a map is constructed. Neither an attack speedup nor a new vulnerable family follows from these negative screens.

For prior work, [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf) treats direct odd-characteristic `(2,…,2)` descent, [Tian–Yu–Li–Wang 2015](https://doi.org/10.1007/978-3-319-17533-1_12) gives explicit cover models in adjacent cases, [Howe 2022](https://arxiv.org/abs/2110.04221) supplies the cover-degree ceiling used here, and [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) establishes isogeny-assisted cover-and-decomposition attacks. Exact-equation priority remains unproven by targeted searches. The next constructive target is another degree-eight denominator shape or an explicit correspondence into the genus-five Jacobian, followed by subgroup and full cost verification.
