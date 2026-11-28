# H116 chiral branch-residual parity protocol

Protocol ID: `H116-CHIRAL-BRANCH-RESIDUAL-PARITY-001`

Filed after complete Prym factor-degree transport was measured null and before
evaluating any residual-branch character against scalar parity.
[pcc:unverified]

## Related-work and novelty boundary

The chord-and-tangent law identifies three intersections of a line with a
Weierstrass cubic, and Riemann--Hurwitz identifies the ramification divisor of
the target-tethered double cover.  Quadratic characters and branch-divisor
factorization are standard.  The earlier H116 signed-Kummer experiment used
products of `y-y(A_j)` at fixed anchors; this experiment instead removes the
known target intersection from its own chiral branch cubic and measures the
field of definition of the other two ramification points.

Targeted repository, ePrint, arXiv and journal searches found no prime-field
ECDLP scalar-parity decoder from the ordered residual-branch characters below.
This is a prior-art screen, not a novelty proof. [pcc:unverified]

## Ramification observable

On `E: y^2=x^3+a*x+b`, let `T=[m]Q=(x,y)` and choose `c in {+1,-1}`.  The line

```text
x+c*y = level,       level=x+c*y
```

meets `E` in the roots of

```text
G(X)=X^3-X^2+(a+2*level)X+(b-level^2).
```

The known root `x` divides `G`.  Retain the residual quadratic

```text
H(X)=X^2+(x-1)X+(a+2*level+x^2-x)
```

and its discriminant

```text
Delta_c(T)=-3*x^2-6*x+1-4*a-8*c*y.
```

The feature bit is one when `Delta_c(T)` is a nonzero square and zero when it
is a nonsquare.  Abstain if `Delta_c(T)=0` or if `H(x)=0`; these are exactly
the repeated-residual and tangent-at-the-known-point singularities detected by

```text
disc(G)=H(x)^2*disc(H).
```

Use multipliers `m=1,2,3,4` and order features as

```text
(m1,c+), (m1,c-), (m2,c+), (m2,c-),
(m3,c+), (m3,c-), (m4,c+), (m4,c-).
```

Test all 256 xors of feature subsets, optionally complemented: 512 affine
Boolean rules predicting canonical scalar parity on the 1,520 nonidentity
targets of the five frozen discovery curves.

## Frozen controls

For every target, multiplier and chirality:

1. multiply `(X-x)H(X)` and require exact recovery of `G(X)`;
2. require the quadratic discriminant to equal the displayed closed formula;
3. independently evaluate the general cubic-discriminant formula and require
   `disc(G)=H(x)^2 disc(H)`;
4. enumerate every base-field root of `H` and require root count `0,1,2` for
   nonsquare, zero, and nonzero-square discriminants;
5. when smooth and split, lift both roots to their line points on `E` and
   require `T+R+S=O`;
6. require exact chiral negation swap: the data for `(T,c)` equals that for
   `(-T,-c)`.

Report smooth/singular counts, rational chord-sum checks, and how often the
ordered tuple distinguishes `Q` from `-Q`.  Require a 1,520/1,520 planted
scorer and byte-identical replay.

## Calibration and conditional algorithm

Rank by `(2*M-C)^2/C` when `2*M>C`.  In each of 4,096 deterministic
within-curve parity shuffles, reselect across all 512 rules.  Retain total and
covered exact counts, all controls, and per-curve scores.

A total exact rule with a generic proof would give a public binary recursion:

```text
d=k mod 2,
Q'=[2^(-1) mod r](Q-[d]P)=[floor(k/2)]P.
```

The residual discriminants, field characters, fixed scalar multiples, and
512-rule evaluation all cost polynomial time in `log p`, so a proved generic
rule would be polynomial-time and hence subexponential.  A finite exact fit is
only a candidate.  A null closes only this four-multiple affine Boolean family
of branch-residual split characters. [pcc:unverified]
