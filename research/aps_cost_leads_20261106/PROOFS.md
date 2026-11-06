# Mathematical argument and proof map

All assertions here retain producer standing [pcc:unverified].

## Disjoint retained-base frontier

Let nonempty disjoint coordinate partitions satisfy `O_i subset N_i`.
For each tuple in `product N_i` outside `product O_i`, let `j` be its first
coordinate not in `O_j`. It belongs to exactly one cell

```
O_0 × ... × O_(j-1) × (N_j \ O_j) × N_(j+1) × ... × N_(m-1).
```

Existence is the first-failure argument; uniqueness follows because two
different first-failure indices would require the earlier coordinate both
inside and outside its old partition. Thus the cells are disjoint and their
union is exactly the difference of the products. Intersecting with the same
relation equation preserves this identity. Sorting/deduplicating coordinate
sets is sound here because the coordinate partitions themselves are disjoint.

This does **not** itself authorize omitting the old region. The driver must
first finish the old traversal, without timeout, early stop or an informative
witness. It retains that transcript. The independent reader checks the
coverage predecessor before subtracting the old region from its point oracle.

| Statement | Declaration / evidence | Exact scope and open bridge |
| --- | --- | --- |
| First failure exists in a finite tuple | `APSLeads.first_failure` | Arbitrary predicates on the finite index range |
| Product difference equals frontier union | `APSLeads.frontier_cover` | `old i -> new i` for each coordinate; membership predicates instantiated at a fixed tuple |
| Frontier index is unique | `APSLeads.frontier_unique` | Two first-failure regions of the same tuple have the same index |
| Actual retained-base solver omits no new solution | Native exact-set and producer/reader tests | Lean does not verify the Rust encoder, complete F4 traversal or sampler; these refinements remain open |

The standalone file is `lean/Frontier.lean`, using only `Std` from Lean
4.34.1. It has no Mathlib dependency. Its three `#print axioms` outputs
are inspected in development receipts. The first two use `propext`,
`Classical.choice`, `Quot.sound`; uniqueness uses `propext`, `Quot.sound`.
No `sorry`, custom research axiom or native kernel bypass is used in this
source. Independent signed proof standing is not supplied by a producer
build [pcc:unverified].

## Nonhorizontal line chart and exceptions

On `y^2=x^3+ax+b` in odd characteristic, substituting `y=t x+c` gives

```
x^3 - t^2 x^2 + (a-2tc)x + b-c^2.
```

For `t != 0`, set `U=t^2`, `V=c/t`. Its elementary coefficients are
`(U, a-2UV, UV^2-b)`. Conversely, if its cubic has three distinct allowed
abscissae, all with nonzero curve ordinates, then at each such root
`y_i^2 = U(x_i+V)^2`. An `x_i+V=0` would force `y_i=0`, which is refused.
Therefore `U` is a nonzero square, `t` exists, and choosing `c=tV` recovers
the line. The two square roots give global sign orientations of the same
coordinate set. Recovery still checks actual signed point sums and nonzero B.

Horizontal lines `t=0` fall outside this chart. Their cubic coefficients
are `(0,a,c^2-b)` and must be searched separately. Omitting them without
the existing orbit-and-label certificate loses valid informative relations.
The explicit public horizontal development control exercises this exception.

The previous identity wrapper issued `IDC1h9e131e5cd24a395f` for the algebraic
coefficient substitution and `IDC1ha14548fc2b5ce5c3` for the scalar frontier
counting identity in the previous study. They retain their original native
evaluation scope and unsigned attestation standing. The actual curve/divisor
lifting argument above is a readable proof, not a full Lean theorem. Missing
bridges are finite-field square roots, divisor-root semantics, curve/group
relation correspondence and refinement of the Rust equations [pcc:unverified].

## Checked trace replay

Native replay constructs basis polynomials through recorded arithmetic
operations on the current input. Input reductions and Buchberger S-pairs
then check the candidate against the current specialization. Trace keys
alone never authorize reuse. Any mismatch, failed reduction, resource cap
or deadline refuses certification and charges ordinary F4 fallback (or
retains timeout). This does not make an unchecked cached basis a certificate.
The implementation's derivation-to-ideal and Buchberger reasoning are not
formalized in Lean here; exact-set and poisoned-specialization controls
remain software checks with producer standing [pcc:unverified].
