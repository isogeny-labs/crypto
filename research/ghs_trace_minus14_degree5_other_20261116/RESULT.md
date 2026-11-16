# Trace -14: degree-five obstruction for the full recorded equation slice

16 November 2026. The corrected degree-four and now complete degree-five screens exclude `K`-morphisms of degrees one through five from each of the twelve recorded trace -14 genus-five carrier equations to each of the twenty recorded trace -14 normal-form elliptic equations over `K=F3[a]/(a⁵+2a+1)`. This is a **finite, computer-assisted theorem for 240 exact carrier/source equation pairs** [pcc:unverified]. It is not a classification of every genus-five curve in the isogeny class and does not produce an evaluable GHS transfer.

## Exact model and orbit coverage

Each carrier is `Y_m/F3: y²=f_m(x)`, with `f_m=x¹¹+Σ_(i=0)^10 digit₃(m,i)xⁱ`. Exact [PGL₂(F3) identities](CARRIER_ORBITS.json) split the twelve monic odd-degree masks into two six-model coordinate orbits:

```text
9735:  9735 44227 64725 90937 124586 144548
17371: 17371 21624 92153 100118 156795 159256
```

For matrix `(a,b;c,d)` in the receipt, the identity is `f_target(x)=(cx+d)¹² f_rep((ax+b)/(cx+d))`, with `ad−bc≠0`; `y_target=(cx+d)⁶ y_rep` gives the curve isomorphism. The orbit producer enumerated all 24 projective linear classes; [audit.py](audit.py) independently reimplemented their polynomial transformations over `F3` and checked both six-model orbits. Two output-only JSON failures and the preserved intermediate receipt are documented in [AMENDMENT_ORBIT_OUTPUT.md](AMENDMENT_ORBIT_OUTPUT.md); the final producer exited zero.

Each source is `E_b/K: v²=u³+u²+Σ_(i=0)^4 digit₃(b,i)aⁱ`, with `#E_b(K)=258=6·43`. The twenty masks form four five-element Frobenius orbits, represented by `11,202,38,80`; their exact mask arithmetic is checked in [TraceMinus14Orbits.lean](../ghs_trace_minus14_isogenous_degree4_20261116/TraceMinus14Orbits.lean). Frobenius conjugation preserves each carrier, map degree, and the denominator search.

The degree-five receipts cover 38 representative cells: two carrier-orbit representatives for source 11, and all twelve carriers for each of sources 202, 38 and 80. Each cell represented 14,446,377 logical assignments in the six complete normalized denominator types `1`, `Q_i`, `(x-c)²`, `Q_iQ_j`, `Q_i(x-c)²`, and `H²`. Across the 38 cells this is **548,962,326 logical assignments**. All rank-deficient quadratic-square systems were enumerated explicitly. Every candidate was checked against `f C²=B(A³+A²B+bB³)`.

The source-bound [aggregate audit](AUDIT.json) checks all 38 cell receipts, their source hashes and exhaustive counters, the source census, and both orbit transformations. It found **54 retained degree-five numerators and zero exact identities**. Independent coefficient replays give an explicit nonsquare obstruction for each retained numerator: [mask 11](SQUARE_WITNESS_11.json), [mask 202](../ghs_trace_minus14_degree5_isogenous_20261116/SQUARE_WITNESS.json), and [masks 38/80](SQUARE_WITNESS.json). The replays reconstruct exact divisibility and either exhibit a nonsquare leading coefficient or a nonzero lower-coefficient remainder after forcing a square root. [DegreeFiveClosure.lean](DegreeFiveClosure.lean) checks the carrier partition and count arithmetic; it does not formalize PGL₂ geometry or the Sage elimination.

## Resulting bound and remaining attack question

The prior geometric argument excludes degrees one through three for these recorded carriers. The corrected [degree-four census](../ghs_trace_minus14_isogenous_degree4_20261116/RESULT.md) covers all twenty source equations; the degree-five receipts above cover every source and carrier orbit. [Howe's theorem](https://arxiv.org/abs/2110.04221) supplies a `K`-morphism of degree at most 33 in this ordinary isogeny class, whose fundamental discriminant is `−776`. Therefore the **minimum `K`-cover degree lies in `[6,33]` for each of these 240 exact equation pairs**. The finite lower bound has two numerator algorithms at degree four; at degree five the exhaustive interpolation has one implementation plus an independent final-square replay. The PGL₂ coordinate identities have a separate Python reimplementation.

Because `33<43`, any Howe-supplied map has injective pullback on the order-43 source subgroup by `φ_*φ^*=[deg φ]`. The theorem gives no coordinate formula for that map, and its pullback lands in `J(Y_m)(K)` rather than providing the needed evaluable image in `J(Y_m)(F3)`. There is consequently no complete trace -14 GHS decomposition or same-target rho measurement here.

[Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805) classifies its specified odd-characteristic direct-cover construction; [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) already describes isogeny-assisted cover attacks. The new finding is a bounded, corrected degree obstruction on a recorded slice. It does **not** establish an unreported vulnerable curve family, a new vulnerable exact curve, or a new attack method. Higher cover degrees, other carrier models and explicit small-field transfer remain open.
