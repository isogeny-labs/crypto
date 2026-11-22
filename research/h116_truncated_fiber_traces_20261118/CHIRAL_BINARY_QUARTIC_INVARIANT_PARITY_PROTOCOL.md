# H116 chiral binary-quartic invariant parity protocol

Protocol ID: `H116-CHIRAL-BINARY-QUARTIC-INVARIANT-PARITY-001`

Filed after branch-pair genus-one Frobenius traces were measured null and
before evaluating any binary-quartic invariant character against scalar
parity. [pcc:unverified]

## Related-work and novelty boundary

For a binary quartic

```text
f(X,Z)=a*X^4+b*X^3*Z+c*X^2*Z^2+d*X*Z^3+e*Z^4,
```

the classical relative invariants are

```text
I=12*a*e-3*b*d+c^2,
J=72*a*c*e+9*b*c*d-27*a*d^2-27*b^2*e-2*c^3,
4*I^3-J^2=27*disc(f).
```

Binary-quartic invariant theory and its relation to the Jacobian `j`-invariant
are standard.  The prior panel retained only the finite-field isogeny class
through Frobenius trace; `I,J` retain isomorphism/cross-ratio geometry not
determined by that trace.  Their relative `GL_2` weights are even, so their
quadratic characters are stable under the corresponding nonzero determinant
powers; the implementation additionally uses a fixed canonical monic model.

Targeted repository, ePrint, arXiv and journal searches found no prime-field
ECDLP scalar-parity decoder from these target-tethered `I,J` character tuples.
This is a prior-art screen, not a novelty proof. [pcc:unverified]

## Frozen observable and rule family

Use the same four smooth branch-pair quartics as protocol
`H116-CHIRAL-BRANCH-PAIR-TRACE-DIGIT-001`, in order

```text
H_0*H_2, H_1*H_3, H_4*H_6, H_5*H_7.
```

For each quartic, retain the quadratic character of nonzero `I` followed by
the quadratic character of nonzero `J`.  Abstain on a selected bit if the
quartic is singular or the corresponding invariant is zero.  The eight-bit
order is therefore

```text
(model0,I), (model0,J), (model1,I), (model1,J),
(model2,I), (model2,J), (model3,I), (model3,J).
```

Point negation swaps model `0<->1` and `2<->3`.  Test all 256 xors of feature
subsets, optionally complemented: 512 affine Boolean rules predicting
canonical scalar parity on the 1,520 nonidentity targets of the five frozen
discovery curves.

## Frozen controls

Retain every inherited branch-cubic, residual-quadratic, resultant, quartic
evaluation, square-root census, genus-one point-count and Hasse control.  For
each admitted quartic:

1. compute `I,J` directly from its five expanded coefficients;
2. compute its discriminant independently as
   `disc(H_i)*disc(H_j)*Res(H_i,H_j)^2`;
3. require the exact finite-field identity `4*I^3-J^2=27*disc(f)`;
4. report the nonzero `I,J` feature census and how often the ordered tuple
   distinguishes `Q` from `-Q`.

Require a 1,520/1,520 planted scorer and byte-identical replay.

## Calibration and conditional algorithm

Rank by `(2*M-C)^2/C` when `2*M>C`.  In each of 4,096 deterministic
within-curve parity shuffles, reselect across all 512 rules.  Retain total and
covered exact counts, all controls, and per-curve scores.

A total exact rule with a generic proof would give the public binary recursion

```text
d=k mod 2,
Q'=[2^(-1) mod r](Q-[d]P)=[floor(k/2)]P.
```

The fixed multiples, branch quadratics, quartic coefficients, `I,J`, quadratic
characters and bounded rule evaluation all cost polynomial time in `log p`.
Thus a proved generic rule would be polynomial-time and hence subexponential.
A finite exact fit is only a candidate.  A null closes only this eight-bit
affine character family. [pcc:unverified]
