# Rigorous boundary-rank bounds

Date: 2026-11-01
Status: architecture-level bounds proved and finite certificates implemented;
hashed-ECDLP family lower bound open; no E1 production data

## Result

Let a width-`w` Boolean boundary tensor be flattened at a split with `a` row
bits into

```text
M in Z^(2^a x 2^(w-a)).
```

All ranks below are over `Q`. The following bounds are now kernel checked:

```text
0 <= rank(M) <= min(2^a, 2^(w-a)).                         (1)

det(M[rows, columns]) != 0 for a k x k minor
    => k <= rank(M).                                      (2)

M = L R, with L having d columns and R having d rows
    => rank(M) <= d.                                      (3)
```

Therefore a nonzero `k × k` minor proves `k <= d` for every exact linear
message that factors this boundary. For a balanced split, a nonzero minor of
order `2^floor(w/2)` meets (1), proves full flattening rank, and rules out any
linear separator message smaller than the complete smaller-side table.

This is a rigorous lower bound for one precisely named architecture. It is not
a lower bound for arbitrary ECDLP algorithms.

## Family and work corollary

For a family indexed by `n`, suppose all three explicit hypotheses hold:

1. a certified family lemma gives `2^e(n) <= rank(M_n)`;
2. every admitted exact contraction exposes a linear separator message of
   dimension `d(n)` with `rank(M_n) <= d(n)`;
3. the implementation performs at least one work unit per message coordinate,
   so `d(n) <= W(n)`.

Then

```text
2^e(n) <= W(n).
```

If there is a positive integer `c` with `c n <= e(n)`, then
`2^(c n) <= W(n)`. Hence that architecture is exponential and, in particular,
not subexponential in `n`.

The formal theorem uses natural-number functions and states these hypotheses
directly. It does not smuggle in a cost model. At present hypothesis 1 is not
proved for the production hashed prime-field family, and hypotheses 2–3 must
be established for each proposed contraction implementation. The exact open
obligation is tracked in
[`problems/hashed-ecdl-boundary-rank-family-lower-bound.md`](../../problems/hashed-ecdl-boundary-rank-family-lower-bound.md).

## Recomputable finite certificates

`tensor-census` now emits a certificate for both the canonical split and the
minimum-rank admitted balanced split:

- the ordered row-side boundary positions;
- selected matrix row and column indices;
- the exact signed decimal determinant.

The verifier reconstructs the flattening directly from the exact boundary
tensor, extracts only the declared minor, and recomputes its determinant with
fraction-free Bareiss elimination. It does not call or trust the rank routine.
A determinant mismatch, duplicate/out-of-range index, singular minor, or
nonexact division fails closed. A valid order-`k` certificate proves only the
lower bound (2); maximality comes from matching the universal upper bound or
from the separately tested exact-rank computation.

The deterministic property control covers 1,152 integer matrices of sizes
`1 × 1` through `6 × 6`: the fraction-free rank agrees with independent
rational Gaussian elimination, and every discovered rank-profile minor has a
recomputed nonzero determinant. A corruption test changes the recorded
determinant and is rejected.

No production certificate is reported here. The E1 protocol is still draft
and the production semantic cut generator is not complete.

## Formal proof map

Source: [`lean/TensorBoundaryBounds.lean`](lean/TensorBoundaryBounds.lean)

| Claim | Lean theorem | Proof ingredients |
| --- | --- | --- |
| (1), general matrix | `matrixRankUpperBound` | Mathlib row/column cardinality rank bounds |
| (1), Boolean dimensions | `booleanFlatteningRankUpperBound` | `Fintype.card (Fin n) = n` |
| (3) | `exactLinearSeparatorRankBound` | rank of a product is at most the left rank; left rank is at most its column count |
| (2) | `nonzeroMinorRankLowerBound` | nonzero determinant gives full minor rank; submatrix rank does not exceed matrix rank |
| Combined finite bound | `nonzeroMinorForcesSeparatorDimension` | transitivity of (2) and (3) |
| Pointwise family bound | `exponentialRankForcesExponentialMessage` | transitivity of the two exposed hypotheses |
| Linear-exponent message bound | `linearExponentForcesExponentialMessage` | monotonicity of powers plus the pointwise theorem |
| Work bound | `linearExponentForcesExponentialWork` | explicit `message <= work` hypothesis |

The project pins Lean `v4.35.0`, `aburan28/crypto-lean` revision
`148f5dc389b341401e06d76abe7783c4ccf42f2e`, and through it Mathlib revision
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`. There are no `sorry`
declarations or project axioms. Build, proof-gate, and axiom-audit receipts are
recorded in [`VALIDATION.md`](VALIDATION.md).

## What these bounds do not cover

- nonlinear or data-dependent compression;
- randomized or approximate contraction;
- a different tensor/Boolean encoding with different cuts;
- algorithms that avoid materializing or processing the separator message;
- arbitrary coordinate-aware, algebraic, or generic ECDLP algorithms;
- quantum algorithms.

Thus even a full-rank theorem for this family would refute the registered
linear-message index-calculus mechanism, not “subexponential ECDLP” in full
generality.

## Sources checked before implementation

- The E1 protocol and implementation receipts in this directory.
- Akshima et al., *A Subexponential Time Algorithm for Elliptic Curve
  Discrete Logarithm*, Bochum v0.2 at the pinned repository revision used by
  this project. Its claimed asymptotic reduction remains conditional on the
  external contraction theorem.
- Markov and Shi, *Simulating quantum computation by contracting tensor
  networks*, SIAM J. Comput. 38(3), 2008,
  <https://doi.org/10.1137/050644756>. Proposition 3.6 and §4 relate ordinary
  contraction work to maximum intermediate tensor width and treewidth.
- Orús, *A Practical Introduction to Tensor Networks*, Annals of Physics 349,
  2014, <https://doi.org/10.1016/j.aop.2014.06.013>. Equation (10) expresses a
  cut as a sum over boundary indices and bounds the resulting Schmidt rank by
  the product of the cut bond dimensions.

The literature supplies the general contraction framework. The contribution
of this slice is the exact statement/certificate bridge for this repository's
integer boundary matrices, not a novelty claim for rank factorization.

## Requirement-to-evidence map

| Requested requirement | Acceptance evidence | Status |
| --- | --- | --- |
| Establish rigorous finite-instance rank lower bounds | Recomputable nonzero-minor certificates checked against the exact BTNIR tensor | Implemented and locally tested; no production certificate yet |
| Prove the separator-message implication | Readable proof and kernel-checked Lean theorem | Proved |
| Prove the asymptotic cost bridge | Conditional Lean theorem with rank, separator and work hypotheses exposed | Proved conditionally |
| Bound the actual hashed prime-field family | Uniform nonzero-minor or equivalent rank theorem for every member of the frozen family | Open |
| Disprove every subexponential prime-field ECDLP algorithm | Lower bound covering arbitrary algorithms | Not established; outside this method's scope |
