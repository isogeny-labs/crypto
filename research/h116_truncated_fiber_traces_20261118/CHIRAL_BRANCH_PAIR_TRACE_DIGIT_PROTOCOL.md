# H116 chiral branch-pair genus-one trace-digit protocol

Protocol ID: `H116-CHIRAL-BRANCH-PAIR-TRACE-DIGIT-001`

Filed after pairwise branch-resultant characters were measured null and before
evaluating any auxiliary genus-one trace against a scalar digit.
[pcc:unverified]

## Related-work and novelty boundary

For two separable residual quadratics with no common root, the double cover

```text
C_ij: Z^2=H_i(X)H_j(X)
```

is a smooth genus-one quartic with two rational points at infinity.  Its
elliptic isomorphism class retains the cross-ratio geometry of the four branch
points.  Quartic models, character-sum point counts, birational conversion to
Weierstrass form, and Schoof-style polynomial-time point counting are
standard.  The preceding panels retained only individual discriminants and
pairwise resultants, not this genus-one Frobenius trace.

Targeted repository, ePrint, arXiv and journal searches found no prime-field
ECDLP scalar-digit decoder based on these target-tethered branch-pair traces.
This is a prior-art screen, not a novelty proof. [pcc:unverified]

## Frozen observable and rule family

Use the eight residual quadratics ordered as

```text
0=(m1,c+), 1=(m1,c-), 2=(m2,c+), 3=(m2,c-),
4=(m3,c+), 5=(m3,c-), 6=(m4,c+), 7=(m4,c-).
```

Construct exactly four genus-one quartics, in this order:

```text
H_0*H_2, H_1*H_3, H_4*H_6, H_5*H_7.
```

Thus point negation swaps features `0<->1` and `2<->3` rather than collapsing
the ordered tuple.  Abstain when either branch cubic is singular or the two
quadratics have zero resultant.  Otherwise retain the genus-one Frobenius
trace modulo three.

Test all affine maps from the four residues to `F_3`: every coefficient vector
in `F_3^4` and every offset in `F_3`, for `3^5=243` rules.  The target is the
canonical digit `k mod 3` on the 1,520 nonidentity targets of the five frozen
discovery curves.

## Frozen controls

Retain every cubic reconstruction, discriminant, residual root-count,
chord-sum, chiral-swap, and pairwise resultant control from the parent panels.
For each admitted genus-one quartic:

1. multiply the two monic quadratics to an explicit monic quartic;
2. at every `x in F_p`, require factored and expanded evaluations to agree;
3. precompute the exact number of square roots of every field element and use
   it to count affine points directly;
4. independently sum the quadratic character and require
   `#C_affine=p+sum_x chi(H_i(x)H_j(x))`;
5. add exactly two rational points at infinity and require
   `trace=p+1-#C=-1-sum_x chi(...)`;
6. require the elliptic Hasse bound `trace^2<=4p`;
7. report how often the ordered four-trace tuple distinguishes `Q` from `-Q`.

Require the 1,520/1,520 planted scorer and byte-identical replay.

## Calibration and conditional algorithm

Rank rules by `(3*M-C)^2/C` when `3*M>C`.  In each of 4,096 deterministic
within-curve digit shuffles, reselect across all 243 rules.  Retain total and
covered exact counts, all controls, and per-curve scores.

A total exact rule with a generic proof would give a public ternary recursion:

```text
d=k mod 3,
Q'=[3^(-1) mod r](Q-[d]P)=[floor(k/3)]P.
```

The frozen subgroup orders are prime and different from three.  For the
generic complexity claim, convert each smooth quartic to an elliptic model and
use polynomial-time finite-field point counting; the native exhaustive
character sum is only the small-curve validation oracle.  The remaining fixed
arithmetic and rule evaluation are polynomial in `log p`.  Therefore a proved
generic rule would be polynomial-time and hence subexponential.  A finite
exact fit is only a candidate.  A null closes only this four-trace affine
family. [pcc:unverified]
