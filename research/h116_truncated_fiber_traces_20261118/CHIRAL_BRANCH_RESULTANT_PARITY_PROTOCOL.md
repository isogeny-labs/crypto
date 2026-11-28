# H116 chiral branch-resultant parity protocol

Protocol ID: `H116-CHIRAL-BRANCH-RESULTANT-PARITY-001`

Filed after individual branch-residual split characters were measured null and
before evaluating any pairwise branch resultant against scalar parity.
[pcc:unverified]

## Related-work and novelty boundary

For monic quadratics `H_i`, the resultant

```text
Res(H_i,H_j)=product_(alpha:H_i(alpha)=0) H_j(alpha)
```

is the norm of the cross-differences between the two unordered branch pairs.
Resultants, Sylvester determinants, and branch-divisor intersection are
standard.  The preceding experiment retained only each quadratic's own
discriminant; pairwise resultants measure mutual branch-divisor position and
are not determined by those individual split bits.

Targeted repository, ePrint, arXiv and journal searches found no prime-field
ECDLP scalar-parity decoder from characters of target-tethered branch-pair
resultants.  This is a prior-art screen, not a novelty proof.
[pcc:unverified]

## Observable and frozen family

Construct the eight residual quadratics `H_i` exactly as in protocol
`H116-CHIRAL-BRANCH-RESIDUAL-PARITY-001`, ordered by

```text
(m1,c+), (m1,c-), (m2,c+), (m2,c-),
(m3,c+), (m3,c-), (m4,c+), (m4,c-).
```

Order the 28 unordered pairs `(i,j)` lexicographically for `0<=i<j<8`.
For each pair, retain one bit according to whether `Res(H_i,H_j)` is a
nonzero square or nonsquare in `F_p`.  Abstain when either underlying branch
cubic is singular or when the resultant is zero (the branch pairs intersect).

Test every single resultant bit and every xor of two distinct resultant bits,
optionally complemented.  This gives

```text
2*(28 + binomial(28,2)) = 812
```

fixed affine Boolean rules predicting canonical scalar parity on the 1,520
nonidentity targets of the five frozen discovery curves.  No higher-order xor,
pair selection, or threshold is added after observing labels.

## Frozen controls

Retain every cubic reconstruction, residual/total discriminant, root-count,
chord-sum, and chiral-swap control from the parent experiment.  For every one
of the `1,520*28=42,560` pairwise resultants:

1. compute the closed formula

```text
Res(X^2+aX+b, X^2+cX+d)
  =(d-b)^2+(a-c)*(a*d-b*c);
```

2. independently compute the determinant of the full `4 x 4` Sylvester
   matrix by the 24-term Leibniz formula and require equality;
3. require symmetry under exchanging the two quadratics;
4. solve the linear difference of the quadratics (or handle equality) and
   require `Res=0` exactly when they have a common algebraic root;
5. report smooth/nonzero coverage and how often the ordered resultant tuple
   distinguishes `Q` from `-Q` under the exact chirality permutation.

Require a 1,520/1,520 planted scorer and byte-identical replay.

## Calibration and conditional algorithm

Rank by `(2*M-C)^2/C` when `2*M>C`.  In each of 4,096 deterministic
within-curve parity shuffles, reselect across all 812 rules.  Retain total and
covered exact counts, all controls, and per-curve scores.

A total exact rule with a generic proof would give the public binary recursion

```text
d=k mod 2,
Q'=[2^(-1) mod r](Q-[d]P)=[floor(k/2)]P.
```

All coordinates, fixed multiples, quadratic coefficients, resultants,
characters, and bounded rule evaluation cost polynomial time in `log p`.
Thus a proved generic rule would be polynomial-time and hence subexponential.
A finite exact fit is only a candidate.  A null closes only the frozen
one-/two-resultant affine family, not higher branch-divisor invariants or the
general ramification route. [pcc:unverified]
