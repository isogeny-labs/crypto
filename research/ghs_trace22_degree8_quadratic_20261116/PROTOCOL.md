# Frozen degree-eight quadratic-square cover cell

16 November 2026, before execution. This study tests the open denominator
`B=x H^2`, where `H=x^2+u x+v` is monic and squarefree, coprime to the branch
polynomial, for a degree-eight map from a trace-22 genus-five carrier to an
elliptic source over `K=F3[a]/(a^5+2a+1)`. The first cell is carrier mask 32280
and elliptic b-mask 68. If the first run is sound, the same frozen code may be
run on the other representative cells `(32280,84)`, `(35349,68)`, `(35349,84)`.

The carrier is `Y:y^2=f(x)`, with `f=x^11+sum_(i=0)^10 digit_3(fmask,i)x^i`.
The target is `E_b:v^2=u^3+u^2+b`, where
`b=sum_(i=0)^4 digit_3(bmask,i)a^i`. Its order is 222 over K. The carrier
branch polynomial factors as `x Q_1...Q_5` with distinct irreducible
quadratics. For each of the `3^5=243` Galois-compatible assignments of the
three elliptic two-torsion x-values at each `Q_i`, CRT gives the unique
residue `h` modulo `M=Q_1...Q_5`.

With `B=xH^2`, the numerator forced by the branch constraints is
`A=(Bh) mod M`. Since `deg M=10`, a degree-eight map requires the x^9
coefficient of A to vanish and the x^8 coefficient to be nonzero. The first
condition is a polynomial in `(u,v)`; specialize each of the 243 `u` values,
solve the resulting polynomial over K for every `v`, and enumerate all K if
the polynomial vanishes identically. For every candidate, check `H` shape,
`deg A=8`, `gcd(A,B)=1`, and the exact identity

`f C^2=B(A^3+A^2 B+b B^3)`.

Point-square tests may reject candidates before polynomial squarehood, since
the right-hand quotient must evaluate to a square at every point where f is
nonzero. They cannot reject a true identity. Save every exact hit with A, B,
C and all counter fields; zero hits is a conclusion for this denominator shape
and these exact cells only. A hit still requires an independently checked
subgroup image and a full same-target GHS/rho comparison before a vulnerability
claim. The earlier complete degree-seven exclusion and other degree-eight
partial screens remain separate.

Prior-work gate: [Iijima-Momose-Chao 2015](https://eprint.iacr.org/2015/805.pdf)
classifies its direct `(2,...,2)` covering type, and [Howe 2022](https://arxiv.org/pdf/2110.04221)
gives a cover-existence ceiling 26 for this fixed carrier/source class. Neither
source supplies the present explicit degree-eight identity. Local screens in
`../ghs_degree7_remaining_20261116/` already exclude degree seven on one
fixed carrier and named degree-eight shapes; they leave this general quadratic
square part open. A missing exact-equation web hit is not publication priority.
