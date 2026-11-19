# H116 chiral genus-three trace digit protocol

Protocol ID: `H116-CHIRAL-GENUS3-TRACE-DIGIT-001`

Filed after the signed-twist mod-two obstruction was measured and proved, and
before evaluating any chiral-cover trace. [pcc:unverified]

## Related-work and complexity boundary

Pila, *Frobenius maps of abelian varieties and finding roots of unity in
finite fields*, Math. Comp. 55 (1990), DOI
`10.1090/S0025-5718-1990-1035941-X`, gives the fixed-dimension theoretical
point-counting boundary used here.  As a concrete genus-three comparison,
Annamalai and Thirumalai, *Counting points on hyperelliptic curves of type
`y^2=x^(2g+1)+a*x^(g+1)+b*x`*, J. Finite Fields Appl. 2020, DOI
`10.1016/j.ffa.2020.101757`, give a polylogarithmic algorithm for a special
genus-three family.  Diem and Scholten, *Cover attacks for elliptic curves
with prime order*, arXiv:`2012.07173`, transfer extension-field ECDLP to a
genus-three Jacobian and then use index calculus; that is not the observable
or solver tested here.  Targeted repository and web searches found no exact
ECDLP digit decoder from the ordered chiral trace tuple below.  This is a
prior-art screen, not a novelty proof. [pcc:unverified]

The native experiment uses direct point counts only as a small-field oracle.
It makes no native timing claim.  The conditional algorithmic claim is that
an exact fixed-genus trace predicate would be polynomial in `log p` in the
Schoof--Pila model.

## Cover family and candidate algorithm

For `Q=[k]P`, `c in {1,-1}` and `m in {1,2}`, put `T=[m]Q` and

```text
f_c(x,y)=x+c*y,
C_(c,m)(Q): z^2=f_c(x,y)-f_c(T).
```

Because `c` is nonzero, `f_c-f_c(T)` has a pole of order three at `O`.
When its three finite zeros are simple, the double cover is smooth of genus
three.  With `u=f_c(T)`, the finite zero polynomial is

```text
c^2*x^3-x^2+(c^2*A+2*u)*x+c^2*B-u^2.
```

The trace is

```text
a_(c,m)(Q)=t_E-sum_(R != O) chi(f_c(R)-f_c(T)).
```

Use the ordered four-vector
`(a_(1,1),a_(-1,1),a_(1,2),a_(-1,2)) mod 3`.  Exhaust all 243 affine maps
from `F_3^4` to `F_3`.  A rule abstains if it uses a nonsmooth coordinate.
The target is the canonical digit `k mod 3` for every nonidentity target on
the five frozen prime-order curves.

If one total rule is exact and has a proved generic identity, recover the
digit `d`, form

```text
Q'=[3^(-1) mod r](Q-[d]P)=[floor(k/3)]P,
```

and repeat until the scalar is zero.  The natural scalar decreases at each
non-base step.  Thus an exact polynomial-time trace rule would give a
polynomial-time, hence subexponential, ECDLP algorithm.  A measured exact rule
without a generic identity is only a candidate.

## Why the prior obstruction does not apply

The substitution `R -> -R` gives the exact swap

```text
a_c(-T)=a_(-c)(T).
```

It does not force either ordered pair to be equal.  Consequently an
asymmetric affine rule may distinguish `Q` from `-Q`.  The experiment must
verify this swap and report how often the ordered residues actually differ.

## Frozen controls and statistic

Use every nonidentity target on the five frozen curves.  Require the cubic
discriminant above to be nonzero, the genus-three Weil bound
`a^2 <= 36*p`, and the exact point-count/trace identity for every admitted
cover.  Under `(x,y)->(s^2*x,s^3*y)`, use `c' = c/s` and level `s^2*u` for
`s=2,3`; the character sum and full trace must be unchanged.  Verify the
chiral swap at both multipliers for every target.  Require a planted label
vector to score 1,520/1,520.

Rank by the one-sided multiclass statistic `(3*M-C)^2/C` when `3*M>C`, with
matches `M` and coverage `C`.  In each of 4,096 deterministic within-curve
label shuffles, reselect over all 243 rules.  Retain total and covered exact
counts, all control counts, chirality distinctions, per-curve scores, and a
byte-identical replay.

A finite association only nominates an unchanged out-of-sample rule.  A null
rejects only this four-trace affine family. [pcc:unverified]
