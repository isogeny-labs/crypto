# Trace -14: all recorded source equations and carriers exclude degree four

16 November 2026. The corrected degree-four cover screen now includes all twenty trace -14 normal-form source equations in the recorded `K=F3[a]/(a^5+2a+1)` census and all twelve recorded monic odd-degree genus-five carrier equations over `F3`. Four source-orbit representatives (`b` masks 11, 202, 38, 80) required 48 exact cells. Every cell completed with zero degree-four numerator candidates in all four normalized denominator classes. Frobenius conjugation extends this finite exclusion to **240 exact carrier/source equation pairs**. This is a computer-assisted bound on a recorded model slice [pcc:unverified], not a claim about all genus-five curves or all GHS transfers.

## Exact objects and coverage

For carrier mask `m`, `Y_m/F3` is `y²=f_m(x)`, where `f_m=x^11+Σ_(i=0)^10 digit_3(m,i)x^i`. The twelve masks are `9735,17371,21624,44227,64725,90937,92153,100118,124586,144548,156795,159256`. For source mask `b`, `E_b/K` is `v²=u³+u²+Σ_(i=0)^4 digit_3(b,i)a^i`, with `#E_b(K)=258=6·43` and trace -14. The twenty source masks form four five-element Frobenius orbits:

| Representative | Complete orbit | Degree-four cells |
|---:|---|---:|
| 11 | 11, 17, 71, 147, 238 | 12 |
| 202 | 202, 81, 117, 47, 187 | 12 |
| 38 | 38, 172, 223, 53, 241 | 12 |
| 80 | 80, 153, 190, 156, 217 | 12 |

The first twelve cells are the corrected [mask-11 census](../ghs_trace_minus14_carrier_degree4_20261116/RESULT.md); the remaining 36 receipts are in this directory. Every carrier polynomial is over `F3`, so `a↦a³` changes only the source equation and the map coefficients. It preserves the normalized degree and denominator class. The source-orbit arithmetic was independently recomputed in Python in [audit.py](audit.py) and checked in [TraceMinus14Orbits.lean](TraceMinus14Orbits.lean).

For a map translated so the rational point at infinity goes to the elliptic origin, write `u=A/B`, `v=Cy/B²`, and `deg A=4`. The previous parity argument leaves exactly `B=L`, `L³`, `L(x-c)²` with nonbranch rational `c`, and `LQ_i`, where `L` is the carrier's unique rational branch factor and the `Q_i` are its five quadratic factors over `K`. Each of 48 representative cells enumerated 59,697 assignments, for **2,865,456 assignments** in all. CRT reconstruction gave zero degree-four numerator candidates. A separately frozen [direct linear-system replay](direct_linear_replay.sage) tested the same 48 cells by left-kernel consistency and solving for five numerator coefficients. It found 11,952 consistent numerators of degree at most four, **all of degree below four**. Its cumulative cell CPU time was 230.395 seconds. The [aggregate audit](AUDIT.json) binds both algorithms' source hashes, census coverage, per-shape counts and zero degree-four fields. The two methods share the field and branch-root computations but use different numerator reconstruction algorithms. The primary CRT screen consumed 100.895 cumulative CPU seconds; these are search costs rather than attack costs.

The exact rational degree-two isogeny from `E_11` has a codomain isomorphic to `E_202`. Its [source-bound witness](ISOGENY.json) records the rational functions, codomain isomorphism, an order-43 point, and its nonzero order-43 image; it also checks the scalar-seven homomorphism relation. The first producer attempt reached those mathematical assertions but failed JSON serialization; [AMENDMENT.md](AMENDMENT.md) records the output-only correction. This is a subgroup-preserving *elliptic isogeny*. It is not a map into a small-field Jacobian.

## Degree bound and vulnerability boundary

The earlier geometric result excludes degrees one through three for all recorded genus-five carriers. The corrected screens exclude degree four for the 240 exact pairs above. [Howe's theorem](https://arxiv.org/abs/2110.04221) supplies a `K`-defined map of degree at most 33 in this ordinary isogeny class, under the verified fundamental discriminant `−776`. Consequently the minimum degree for each recorded pair lies in **`[5,33]`**, subject to the stated geometric argument and finite Sage screen. Since `33<43`, Howe's map has an injective pullback on the selected order-43 source subgroup by push-pull. That existence statement does not give coordinate formulas or an evaluable transfer to `J(Y_m)(F3)`.

The original exact pair `(m,b)=(9735,11)` retains its stronger separately screened interval `[8,33]`; degrees five through seven have not been excluded for the other 239 pairs. There is no complete trace -14 GHS decomposition or same-target rho comparison in this result. [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805) covers its specified direct odd-characteristic construction, and [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) already describes isogeny-assisted cover attacks. The new contribution here is the corrected, broader *finite degree obstruction*; no new vulnerable family, exact vulnerable curve, or attack method is established.
