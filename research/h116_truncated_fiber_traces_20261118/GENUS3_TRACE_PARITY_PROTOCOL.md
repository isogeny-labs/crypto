# H116 genus-three cover trace parity protocol

Protocol ID: `H116-GENUS3-TRACE-PARITY-001`

Filed after the selected division-torsor cocycle was proved constant and before
evaluating any cover trace. [pcc:unverified]

## Complexity and related-work boundary

Pila, *Frobenius maps of abelian varieties and finding roots of unity in
finite fields*, Math. Comp. 55 (1990), DOI
`10.1090/S0025-5718-1990-1035941-X`, extends Schoof's method to fixed-
dimension abelian varieties.  Thus the zeta function, and in particular the
Frobenius trace, of an explicitly supplied fixed-genus curve is computable in
time polynomial in `log p` in that theoretical model.  The native experiment
uses direct point counts only as a small-field oracle; it makes no timing
claim.  Targeted repository, Crossref and OpenAlex searches found fixed-genus
point counting and cyclic-cover work, but no exact ECDLP decoder matching the
trace-bit family below.  This is a prior-art screen, not a novelty proof.
[pcc:unverified]

## Cover family and candidate algorithm

For `Q=[k]P` and `1<=m<=8`, put `T_m=[m]Q=(x_m,y_m)` and form the quadratic
cover of `E`

```text
C_m(Q): z^2 = y - y_m.
```

The function `y-y_m` has a pole of order three at `O`.  When
`x^3+A*x+B-y_m^2` is squarefree, the cover is smooth of genus three.  Its
point count and trace are

```text
#C_m(F_p) = #E(F_p) + sum_(R in E(F_p), R != O) chi(y(R)-y_m),
a_m(Q) = p+1-#C_m(F_p).
```

The eight candidate bits are `a_m(Q) mod 2`.  Test every affine Boolean rule:
the xor of any subset of the eight bits, optionally complemented.  There are
512 rules.  A nonsquarefree member abstains for every rule that uses it.  A
total exact parity rule could be iterated by the ordinary public halving
recursion, giving a polynomial-time ECDLP algorithm when combined with fixed-
genus point counting.

## Controls

Use all 1,520 nonidentity targets on the five frozen prime-order curves.  For
every admitted cover require the genus-three Weil bound `a_m^2 <= 36p`.
Recompute trace parity after the coordinate rescalings
`(x,y)->(u^2x,u^3y)`, `u=2,3`; a nonsquare `u` twists the cover and negates
the character sum, which must leave parity unchanged.  A planted feature must
be decoded exactly.

Rank by the same one-sided exact statistic `(2M-C)^2/C` when `2M>C`, with
coverage `C`.  In each of 4,096 deterministic within-curve label shuffles,
reselect over all 512 rules.  Retain exact and exact-on-covered counts,
smoothness coverage, per-curve scores, all control counts and a byte-identical
replay.

A finite association only nominates an unchanged OOS rule.  It is not a
genericity proof; a null rejects only this eight-cover affine-bit family.
[pcc:unverified]
