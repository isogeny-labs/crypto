# Trace −14: complete degree-seven cover screen across source and carrier orbits

16 November 2026. **Result:** all eight source/carrier orbit representatives have zero exact degree-seven `K`-morphisms under the full finite denominator partition. Seven cells are new in this continuation; the E11/Y9735 cell has an earlier complete degree-seven receipt. Together with the prior degree-one-through-six exclusions and Howe's degree ceiling, the **computer-assisted minimum `K`-cover degree lies in `[8,33]` for all 240 recorded exact equation pairs**. This is a degree obstruction, not a new vulnerable curve family or a full attack result.

## Exact equations and scope

Let `K=F3[a]/(a^5+2a+1)`. Every source has `E_b: v²=u³+u²+b`, order `258=2·3·43` over K. Four five-element Frobenius orbits have representatives `b= a²+2` (mask 11), `2a⁴+a³+a²+a+1` (202), `a³+a²+2` (38), and `2a³+2a²+2a+2` (80). The 12 recorded genus-five carriers form two `PGL₂(F3)` coordinate orbits, represented by

```text
Y_9735:  y²=x¹¹+x⁸+x⁷+x⁶+x⁵+x²+2x
Y_17371: y²=x¹¹+2x⁸+x⁷+2x⁶+2x⁵+x⁴+x³+x²+1.
```

The [preceding audit](../ghs_trace_minus14_degree6_orbits_20261116/AUDIT.json) verifies the source/carrier orbit inventories and the degree-six result. The orbit identities preserve map degree, so eight representative cells cover 20×12 exact equation pairs. This is a statement about those normal-form equations and genus-five carrier models, not all curves of this trace or arbitrary Jacobian correspondences.

## Degree-seven search and audit

After translating the carrier's rational point at infinity to the elliptic origin, write `u=A/B`, `deg A=7`, `gcd(A,B)=1`, and `v=yC/B²`. The full identity is `f C²=B(A³+A²B+bB³)`. Valuation parity gives the ten `(quadratic branch-pole count, square-part degree)` pairs checked in the [Lean closure](DegreeSevenClosure.lean); the concrete denominator types are partitioned among six source-bound arms. All final candidates were checked against the exact polynomial identity.

| Arm | Denominator scope | Seven new cells | Exact maps |
|---|---|---:|---:|
| [quadratic-square](PROTOCOL.md) | `H²`, `QiH²`, squarefree quadratic `H` | 7 | 0 |
| [sparse](SPARSE_PROTOCOL.md) | `1`, `Qi`, `Qi³`, `QiQj` | 7 | 0 |
| [remaining](REMAINING_PROTOCOL.md): linear square | `H²,H⁴,H⁶,QiH²,QiH⁴`, linear `H` | 7 | 0 |
| remaining: cubic square | `H²`, monic cubic `H` | 7 | 0 |
| remaining: two poles plus linear square | `QiQjH²`, linear `H` | 7 | 0 |
| remaining: three quadratic poles | `QiQjQk` | 7 | 0 |

All 42 new JSON/stderr receipts are in this directory. The [source-bound audit](AUDIT.json) verifies the frozen hashes, masks, schemas, exact orders, completion counters, empty hit lists, and no censored cubic systems. The quadratic-square arm records 37,946,232 logical parameter assignments per cell; the two-pole arm records 7,906,140 logical numerator candidates; the three-pole bitset arm represents 643,043,610 logical numerator candidates per cell. These are different search counters and must not be summed as a common operation count. The scripts use one Sage elimination implementation per arm, adapted from the earlier fixed-pair screen. The structural audit is independent of receipt generation but is not an independent algebra-system elimination. For the three-pole and two-pole arms, point/bitset filters are necessary conditions followed by exact identity tests; the carrier-dependent evaluation points avoid branch roots.

`lean DegreeSevenClosure.lean` exited zero. Lean checks the ten arithmetic pole-budget pairs, eight-cell indexing, 42-receipt count, and the conditional `[8,33]` implication. It does **not** prove the geometric denominator classification, PGL₂ identities, elimination, or Howe's theorem. The earlier exact-pair [geometric and finite search report](../ghs_trace_minus14_20261116/RESULT.md) records its own independent Magma geometric replay but only one elimination implementation. Accordingly the new all-pair interval is explicitly **computer assisted**, with independent elimination replay still desirable before treating the finite exclusion as a fully replicated theorem.

## Transfer, cost, and prior-work boundary

At extension degree 11 over K, the source group order is `174449211009144402077135526=2·3·43·617·3323·329787202203776417`, with the last 59-bit factor prime. [Howe's theorem 4.17](https://arxiv.org/abs/2110.04221) gives a `K`-cover of degree at most 33 in this ordinary isogeny class. Its pullback is injective on that prime subgroup because `33<r`, but no coordinate formula or nonzero descent norm into the subfield Jacobian is available. There is no trace −14 decomposition run or full same-target GHS/rho cost. The receipt wall/CPU times measure cover searching, not an attack.

[Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf) classifies the direct odd-characteristic `(2,…,2)` descent construction for prime degrees through seven. [Tian–Yu–Li–Wang 2015](https://doi.org/10.1007/978-3-319-17533-1_12) gives explicit odd-characteristic cover models and maps, including cubic-extension cases. [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) already combines isogeny-assisted covers and decomposition. A targeted exact-equation search did not identify these equations, but that does not establish global priority. The present result supplies a stronger finite degree bound; it establishes neither a new eligible vulnerable family nor a new attack method. The next constructive window is degree 8–33 or a non-cover correspondence, followed by an evaluated subgroup norm and complete measured comparison against matched rho.
