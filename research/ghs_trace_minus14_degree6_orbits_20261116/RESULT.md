# Trace −14: degree-six cover search across both carrier and all source orbits

16 November 2026. **Status:** finite negative cover result. The eight representative source–carrier cells have zero exact degree-six `K`-morphisms in the frozen search. With the earlier degree-one-through-five exclusions and Howe's ceiling, the minimum `K`-cover degree is in **[7,33] for each of the 240 recorded exact equation pairs**. The E11/9735 pair has the stronger earlier [8,33] fixed-pair interval. This does not establish a new vulnerable family or a new attack method.

## Exact curves and orbit reduction

Let `K=F3[a]/(a^5+2a+1)`. All sources have equation `E_b: v²=u³+u²+b`, with `#E_b(K)=258=2·3·43`, and the four source Frobenius-orbit representatives are:

| mask | `b` |
|---:|---|
| 11 | `a²+2` |
| 202 | `2a⁴+a³+a²+a+1` |
| 38 | `a³+a²+2` |
| 80 | `2a³+2a²+2a+2` |

The two carrier `PGL₂(F3)`-orbit representatives are `Y_f: y²=f(x)`:

| mask | `f(x)` |
|---:|---|
| 9735 | `x¹¹+x⁸+x⁷+x⁶+x⁵+x²+2x` |
| 17371 | `x¹¹+2x⁸+x⁷+2x⁶+2x⁵+x⁴+x³+x²+1` |

The prior [source-orbit audit](../ghs_trace_minus14_isogenous_degree4_20261116/AUDIT.json) covers 20 normal-form source masks in four orbits; the [carrier-orbit audit](../ghs_trace_minus14_degree5_other_20261116/CARRIER_ORBITS.json) covers 12 carrier masks in two orbits. The coordinate identities preserve map degree. The 4×2 representative cells therefore cover 20×12 exact equation pairs, subject to those independently recorded orbit identities.

## Complete degree-six finite search

The [frozen protocol](PROTOCOL.md) and [source](screen.sage) search the ten denominator shapes for `u=A/B` and check the full identity `f C²=B(A³+A²B+bB³)`; the source SHA256 is `85d92e98849801cb35570e91c6a00755488b91090a4f1eaf8971082e9165250d`. One earlier E11/9735 receipt is reused. The seven new JSON receipts and stderr timings are retained in this directory. The [audit](AUDIT.json) verifies all eight source hashes, masks, order/trace fields, ten shape counts, fallback reporting, and zero exact identities.

| Carrier mask | E11 | E202 | E38 | E80 |
|---:|---:|---:|---:|---:|
| 9735 | 0 (earlier receipt) | 0 | 0 | 0 |
| 17371 | 0 | 0 | 0 | 0 |

Each cell reports 14,506,074 branch assignments, totaling 116,048,592 across eight cells. The seven new wall times were 54.34–58.98 seconds per cell on this host, with concurrent runs; these are producer times, not attack costs. The elimination uses one Sage implementation per new cell. The prior E11/9735 screen also has one implementation. There is no independent algebra-system replay of the degree-six exclusion, and no claim about arbitrary correspondences.

The [checked Lean closure](DegreeSixClosure.lean) proves representative index/count arithmetic and the conditional `[7,33]` implication. `lean DegreeSixClosure.lean` exited zero. It does **not** formalize the geometric pole classification, elimination, source/carrier coordinate identities, or Howe's theorem. The earlier degree-five and source-orbit Lean files were rechecked with exit zero.

## Subgroup and attack implications

For the selected degree-11 extension of `K`, recurrence from trace `−14` gives `#E(F_{3^55}) = 174449211009144402077135526 = 2·3·43·617·3323·329787202203776417`; Sage verified that the final 59-bit factor is prime. A degree at most 33 map supplied abstractly by [Howe's theorem 4.17](https://arxiv.org/abs/2110.04221) would have degree coprime to that factor, so pullback to the Jacobian over `F_{3^55}` is injective on the selected prime subgroup. No coordinate formula for such a map, nonzero descent norm into `J(Y)(F_{3^11})`, decomposition run, or same-target GHS/rho measurement is present. The 59-bit factor and the producer timings cannot be read as attack cost or speedup.

## Prior work and novelty boundary

[Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf) classifies the direct odd-characteristic `(2,…,2)` descent construction for prime extension degrees 2, 3, 5, and 7. [Howe 2022](https://arxiv.org/abs/2110.04221) provides the cover existence mechanism and degree ceiling used here. [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) already combines isogeny-assisted covers with decomposition. The present exact equations and finite obstruction may be new as recorded computations; they establish neither a new eligible attack family nor a new attack method. Exact-equation priority remains incompletely checked against the global literature. The next construction target is an explicit degree 7–33 morphism or a different correspondence with a proven nonzero norm on the 59-bit subgroup, followed by a complete attack-versus-rho cost on the same source subgroup.
