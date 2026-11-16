# Trace -14 degree-four screen on the rational 2-isogeny neighbor

Frozen 16 November 2026 before the counted runs. The previous corrected
twelve-carrier census tested source `E_11: v²=u³+u²+a²+2` over
`K=F3[a]/(a^5+2a+1)` and found no degree-four map. Its source has order
258 and one rational degree-two isogeny. Sage gives codomain coefficients
`[0,1,0,a³+a²+2a+2,2a⁴+a³+a+2]`; translating its x-coordinate by its
linear coefficient gives the isomorphic normal-form source

`E_202: v²=u³+u²+(2a⁴+a³+a²+a+1)`.

Both source equations have 258 K-points and trace -14. This is an isogeny
neighbor, not a new parameter family. Search the same twelve recorded
monic odd-degree trace -14 carriers from the prior census against E_202.
The exact list and carrier encoding are in
`../ghs_trace_minus14_carrier_degree4_20261116/PROTOCOL.md`.

Use the corrected all-carrier denominator partition `B=L`, `L³`,
`L(x-c)²`, and `LQ_i`, with `L` the carrier's actual rational branch
factor. Enumerate all compatible 2-torsion residues at each unpoled `Q_i`
and reconstruct every possible degree-four numerator by CRT. Test every
survivor against `f C²=B(A³+A²B+bB³)`. Expected assignments per carrier:
59,697. A positive map must be checked independently and composed with
the explicit degree-two isogeny before any subgroup or attack-cost claim.

Prior work: [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf)
already treats isogeny-assisted cover attacks. The odd-characteristic direct
classification [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf)
and [Howe 2022](https://arxiv.org/pdf/2110.04221) were reviewed in the
parent protocols. An exact new equation would still need a specific
publication search and a full same-subgroup GHS/rho comparison.
