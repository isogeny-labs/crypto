# Degree-five cover search for trace -14 isogeny neighbor

Frozen 16 November 2026 before execution. The recorded trace -14 source
`E_11/K: v²=u³+u²+a²+2`, `K=F3[a]/(a^5+2a+1)`, has a verified rational
degree-two isogeny to the normal-form source
`E_202: v²=u³+u²+2a⁴+a³+a²+a+1`. Both have order 258 and an order-43
subgroup. The complete corrected degree-four census excludes a map to
E_202 from every one of the twelve recorded monic odd-degree trace -14
genus-five carriers. The source `screen.sage` adapts the earlier complete
degree-five denominator algorithm to E_202 and those carriers.

For each exact carrier mask `m`, let `f_m=x^11+Σ digit_3(m,i)x^i` and let
`L` be its unique rational branch factor. The normalized degree-five
denominators are `1`, `Q_i`, `(x-c)²`, `Q_iQ_j`, `Q_i(x-c)²`, and `H²` for
monic quadratic `H` coprime to `f_m`, with `c` away from the rational
branch root. Enumerate all compatible elliptic two-torsion assignments at
unpoled branch factors, reconstruct `A` by CRT, solve the quadratic-square
coefficient system including rank-deficient fallbacks, and check every
survivor against `f_m C²=B(A³+A²B+bB³)`. The declared logical coverage is
14,446,377 assignments per carrier. The first cell is mask 9735; if it
completes soundly, continue through the other eleven masks.

A hit is a proposed evaluable cover only. It needs independent equation
replay, subgroup transfer into the intended small-field Jacobian, descent
and complete same-target rho comparison before a vulnerability claim.

Prior-work gate: [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf)
classifies its specified odd-characteristic direct covering type;
[Howe 2022](https://arxiv.org/pdf/2110.04221) gives abstract bounded-degree
cover existence; [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf)
already covers isogeny-assisted transfer in principle. The exact E_202
parameter and carrier masks were checked against the local research KB;
no matching formula was found, which is not a publication-priority result.
