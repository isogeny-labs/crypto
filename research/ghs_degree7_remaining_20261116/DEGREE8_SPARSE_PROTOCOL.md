# Degree-eight branch-supported cover screen

Frozen 16 November 2026 UTC before counted runs. This is a bounded lead
following the complete degree-seven exclusion on the twelve recorded
trace-`+22` carriers. It fixes `K=F_3[a]/(a^5+2a+1)`, source masks 68 and
84, and one carrier representative from each explicit `PGL₂(F_3)` orbit:
32280 and 35349. Source curves have 222 `K`-points. The target carrier
polynomials are exactly those in `CARRIER_ORBITS.json`.

For a degree-eight map normalized to take infinity to the elliptic origin,
the rational elliptic x-map `u=A/B` has `deg A=8` and
`deg B=8-e∞∈{1,3,5,7}`. The valuation parity argument in the degree-seven
theorem forces the rational branch factor `L=x` into `B` with odd exponent.
This screen covers eight branch-supported shapes:

`L, L³, L⁵, L⁷, LQi, L³Qi, L⁵Qi, LQi³`.

For the first four, all five quadratic branch factors are unpoled. Their
243 Galois-compatible elliptic 2-torsion assignments give one CRT residue
`A=(Bh mod M)`, `deg M=10`, per shape. For each of the last four, one `Qi`
is a pole, four remain unpoled, and `deg M=8`; every numerator is
`A=(Bh mod M)+λM`, `λ∈K×`. The exact square leading coefficient forces
`λ` to be a square, leaving 121 values. There are `4·5·3⁴·121=196,020`
logical high-degree candidates per carrier/source cell.

Ten fixed nonzero field points supply necessary square-value filters.
Every candidate that survives receives `deg A=8`, coprimality, exact
`f | B(A³+A²B+bB³)`, polynomial squarehood, and the full equation
`f C²=B(A³+A²B+bB³)`. All filter failures remain rejections within the
declared eight shapes; the search makes no claim about square factors at
nonbranch poles, multiple quadratic branch poles, or degrees 4–6 and 9–26.
The two Frobenius source orbits and two carrier coordinate orbits transport
any completed exact result within this finite census.

Prior work checked before this lead: the local odd-characteristic GHS
classification audit, [Howe's cover ceiling](https://arxiv.org/pdf/2110.04221),
and KB queries `degree eight genus five elliptic cover odd characteristic
GHS rational branch pole`, `hyperelliptic genus five degree 8 elliptic
subcover F3^5`, and `GHS degree 8 cover trace 22 rational Weierstrass pole`
at revision `9141e386254340a9bdba3036f2bb706a` (14 November, stale for
newer local work). No exact published formula was found in those bounded
queries; absence of a hit is not a publication-priority conclusion.
