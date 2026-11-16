# A direct-cover separation and a metacyclic CM obstruction

16 November 2026. This note checks two leads against primary papers. It is a
mathematical scope audit, not an attack-cost result. The shared dated session
log remains reserved by Conductor T-6720, so this scoped note records the
sources and decisions until that log can be updated. Claim standing:
[pcc:unverified].

## The trace-24 source is outside the classified degree-five direct cover

Let `K=F5[z]/(z^5+4z+1)` and

`E24/K: Y^2=X^3+2X+2+z+2z^3`.

The independently replayed order is `#E24(K)=3102=2*3*11*47`, hence
`#E24(K)=2 mod 4`. The source equation, count and quotient data are in
[`../ghs_cyclic11_four_branch_cm_20261116/RESULT.md`](../ghs_cyclic11_four_branch_cm_20261116/RESULT.md).

[Iijima--Momose--Chao 2013, Section 7 classification list](https://eprint.iacr.org/2013/487.pdf)
classifies odd-characteristic direct `(2,...,2)` covers under its
isogeny condition. Its degree-five elliptic row has

`C0/K: y^2=(x-alpha)(x-alpha^q)(x-alpha^(q^2))(x-alpha^(q^3))`,

with `alpha in K\F_q`, and the carrier genus is five. All four branch
points are `K`-rational. Taking one as origin shows that the associated
elliptic curve has full `K`-rational 2-torsion, so its group order is
divisible by four. A finite-field isogeny preserves group order. Thus
`E24` cannot be isogenous over `K` to any member of this classified
degree-five, genus-five direct-cover row. The assertion is limited to
the paper's `(2,...,2)` cover construction and its isogeny condition;
it does not rule out other correspondences. [pcc:unverified]

This separation strengthens the reason to seek an evaluable
`Res_(K/F5) E24 -> J(Q24)` map: such a map would cross a boundary of
the published *direct-cover* classification. It would not by itself
establish global publication priority, a new geometric family or a
faster attack. The previously proved degree-prime-to-47 isogeny
existence remains nonconstructive. [pcc:unverified]

## A distinct three-branch metacyclic family fails the ordinary-source test

Let `ell=6n+1` be prime, with odd `n>1` and `gcd(n,3)=1`; choose
`k mod ell` of order three. For prime `p` with `p!=3,ell`, consider
the smooth three-branch Kummer curve

`Y: y^ell=x(1-x)^k` over `F_q`, where `q=p^s`.

Write `h=(k^2+k+1)/ell`. The order-three map

`tau(x,y)=(1/(1-x), (-1)^k*y^k/(1-x)^h)`

gives the quotient `Q=Y/<tau>` of genus `n`. This is the established
metacyclic/Fermat quotient geometry of
[Carocca--Lange--Rodriguez, Theorem 1 and Corollary 3.9](https://arxiv.org/abs/0906.4185),
also treated by [Tafazolian](https://arxiv.org/abs/2611.08176).
Assume `ord_ell(q)=n`. The same fixed-orbit argument as for the
dihedral family yields `#Q(F_(q^m))=q^m+1` for `1<=m<n` and

`P_Q(X)=X^(2n)-tX^n+q^n`,
`t=(q^n+1-#Y(F_(q^n)))/(3n)`.

This sparse polynomial is an arithmetic deduction from the published
geometry; its global priority has not been established.
[pcc:unverified]

The apparent elliptic-source lead has a CM obstruction. The same
Carocca--Lange--Rodriguez paper, Proposition 3.10, proves the quotient
CM type `Phi` primitive in the cyclic CM field
`M=Q(zeta_ell)^(<k>)` of degree `2n`. The image of `p` in
`Gal(M/Q)` has order `n` or `2n`, because the image of `q=p^s`
has order `n`. [Blake, Theorem 1.2](https://arxiv.org/abs/1311.5454)
computes the Newton slopes from the decomposition subgroup `D` of
`p`. If `|D|=n`, it has two cosets. Their CM-type intersection sizes
are `a` and `n-a`; `a` cannot be `0` or `n`, since either case would
make the primitive `Phi` induced from the imaginary quadratic
subfield fixed by `D`. The slopes are therefore `a/n` and
`1-a/n`. Odd `n` makes both different from `1/2`, so `J(Q)` is
neither ordinary nor supersingular. It cannot be isogenous to the
Weil restriction of *any* elliptic curve over `F_(q^n)`, whose slopes
are only `0,1` or `1/2`. If `|D|=2n`, the sole slope is `1/2`:
ordinary elliptic sources are still excluded, but a supersingular
elliptic source needs a separate test. In either case `p` divides
`t`. This no-go is for this CM family and order condition, not for
arbitrary metacyclic covers. [pcc:unverified]

There is an exact check at `ell=31,n=5,k=5,p=2`. In characteristic
two the same cover and order-three quotient are tame because `2` is
prime to `3*31`; `ord_31(2)=5`. In `F32`, a nonzero 31st power is
exactly `1`. The equation `x(1-x)^5=1` becomes
`x^6+x^5+x^2+x+1=0`. This degree-six polynomial has no linear
factor over `F2`, hence no degree-five factor and no root in `F32`.
Thus `#Y(F32)=3`, the two twisted fixed-point counts are each 33,
and `#Q(F32)=23`. Together with the first four point counts this
gives `P_Q(X)=X^10-2X^5+32`, whose 2-adic slopes are `1/5,4/5`.
This independently illustrates the obstruction without treating a
sparse Weil polynomial as proof of an elliptic source.
[pcc:unverified]

## Evidence boundary and next action

The shared KB was queried first at revision
`9141e386254340a9bdba3036f2bb706a` (last refresh 14 November
2026). Sources checked (KB):
`source:8f4f7a19982f0723d69d7d648dff47dd`
(`crypto-autoresearcher:knowledge/frontiers/ecdlp/index-calculus/KR-IC-5ea2f8.yaml`)
points to the odd-characteristic cover classification; the original
2013 paper above supplies the actual scoped theorem. The KB is
older than this 16 November result. The metacyclic CM proof was
checked against the two original papers cited above, not against a
search snippet. [pcc:unverified]

The next constructive test remains the order-47 source-to-quotient
map for the trace-24 four-branch case. The metacyclic three-branch
family with `|D|=n` supplies no elliptic source, while `|D|=2n`
leaves only a supersingular possibility; neither should be promoted
as an ordinary GHS family. [pcc:unverified]
