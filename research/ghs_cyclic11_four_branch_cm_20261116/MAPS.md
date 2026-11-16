# Exact quotient maps and subgroup boundary

16 November 2026. The following maps are defined over `k=F5`. They are
carrier-side maps. The source elliptic-to-quotient isogeny is currently
known only by its isogeny class. Claim standing: unverified.

| Case | Rational cover equation | Involution | Quotient coordinates | Prime subgroup |
|---|---|---|---|---:|
| trace 2 | `C2: y^11=x(x-1)/(x-2)` | `tau(x,y)=(2/x,1/y)` | `u=x+2/x`, `v=y+1/y` | 71 |
| trace 24 | `C24: y^11=x((x-1)/(x-2))^9` | `tau(x,y)=(2/x,1/y)` | `u=x+2/x`, `v=y+1/y` | 47 |

The integer numerator identity for `C2` is certified by
`IDC1ha61ebaac2a79b72e`. For `C24`,
`IDC1h01d6f75e07ec20c7` certifies the relevant integer numerator
identity and direct rational substitution gives `f(2/x)f(x)=2^-8`;
`2^-8=1` in F5. The certificates are evaluated tier, and the
specialization uses exact integer reduction modulo five. The involution
fixes `u,v`. [pcc:unverified]

The map `pi:C→Q` is the morphism to the smooth projective curve with
function field `k(u,v)`. It has degree two: `x` satisfies
`x²-u*x+2=0`, while the order-11 deck action gives eleven distinct
generic conjugates of `v` over `k(u)`. Thus `[k(C):k(u)]=22` and
`[k(u,v):k(u)]=11`. The two geometric fixed base points satisfy
`x²=2`; on each 11-point fiber, an involution of an odd-size torsor has
one fixed point. Riemann–Hurwitz with `g(C)=10` and two fixed points
gives `g(Q)=5`. This function-field and genus derivation is on paper;
the trace-2 quotient has also been independently replayed as the
hyperelliptic curve below. The trace-24 smooth model remains a paper
derivation. [pcc:unverified]

For `C2`, set `D=D_11(v,1)` and
`P=v^5+v^4+v^3+2v^2+3v+1` in `F5[v]`. The exact quotient has the
smooth model

`H2: W²=(v-2)(D-1)`, with `W=(2u-D-2)/P`.

Here `D-2=(v-2)P²` modulo five, and the right side of `H2` is a
squarefree degree-12 polynomial. The quotient invariant relation and
square-factor relation have evaluated identity records `IDC_Q2.json`
and `IDC_H2.json`; `hyperell_replay.gp` independently verifies the
factorization, squarefreeness, and Weil polynomial
`X^10-2X^5+3125`. The rational formula for `W` extends to a morphism
on the smooth projective models; the displayed denominator only
restricts the affine chart. [pcc:unverified]

Pullback and norm give homomorphisms

`pi*: JQ(k) → JC(k)` and `pi_*: JC(k) → JQ(k)`

with `pi_* pi* = [2]` on `JQ(k)`, the standard finite-flat degree
formula for Picard groups. For either odd prime `r=71,47`, if
`D∈JQ(k)[r]` and `pi*(D)=0`, then `2D=0`; hence `D=0`. Therefore
`pi*` is injective on the exact order-`r` subgroup, and the subgroup
survives in `JC(k)`. The source group orders and corresponding Jacobian
orders are `3124` and `3102`, so such prime-order subgroups exist by
Cauchy's theorem. For trace 2, the Jacobian order follows from an
independent hyperelliptic Weil-polynomial replay; trace 24 still uses
the quotient point-count derivation in `RESULT.md`.
[pcc:unverified]

The remaining arrow is

`E(F3125) = Res_(K/k) E(k) --[explicit isogeny needed]--> JQ(k)`.

Tate's theorem gives an isogeny at the isogeny-class level because the
two degree-ten Weil polynomials match, but it does not give the degree,
an algorithm to evaluate it on points, or its action on the chosen
order-71 or order-47 subgroup. The displayed `pi*` does not close that
gap. [pcc:unverified]

There is a stronger **existence** result for trace 24. For
`F_t(X)=X^10-tX^5+3125`, the polynomial discriminant is
`disc(F_t)=5^30(t²-12500)^5`. At `t=24`, the residual factor is
`-11924=-4·11·271`, which is nonzero modulo `47` (remainder `14`).
Because the characteristic polynomial is irreducible and `47` is prime
to its discriminant, the `47`-adic Tate modules of
`Res_(K/k) E24` and `JQ24` are rank-one free modules over the same
finite étale order `Z_47[X]/(F_24)`. An isomorphism of these modules
commuting with Frobenius lifts by integral Tate to an actual
`k`-isogeny whose degree is prime to `47`. Its restriction to any
order-47 subgroup is injective. This upgrades trace 24 from an
abstract isogeny-class match to a **nonzero subgroup-preserving map
whose existence is proved**, conditional on the quotient Weil
polynomial derivation. It still supplies no formula or evaluation
algorithm. At `t=2`, `-12496=-16·11·71`, so this discriminant criterion
does not decide preservation of the order-71 subgroup. The arithmetic
remainders are checked in `FourBranchFinite.lean`; the discriminant
formula was independently checked by PARI/GP `poldisc`.
[pcc:unverified]

Primary references: [Tate 1966](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf)
for isogeny classes, and [Saito's Picard functor notes](https://www.ms.u-tokyo.ac.jp/~t-saito/talk/eepr.pdf)
for `pi_* pi*=[deg pi]`. The integral lattice step uses
[Poonen–Rybakov, Lemma 2.2(iii)](https://math.mit.edu/~poonen/papers/tate_modules.pdf).
The exact equations and native/PARI count
receipts are in `RESULT.md`.
