# What will establish that the degree is lower?

For Boolean generators f_i, the bounded matrix at D contains rows
`m f_i mod (x_j²−x_j)` with `deg(m)+deg(f_i) ≤ D`. Columns are squarefree
monomials, in a fixed order. A refutation is a vector a with `a M = 1`.
An independently replayed a proves the upper bound `D_ref ≤ D`.

At D−1 retain a dual vector v satisfying `M v = 0` and `v[1]=1`.
Then 1 is absent from that row span. Together, these establish
`D_ref = D` for those exact generators and that Boolean convention.
The verifier should multiply the named generators independently and check
both identities, rather than trust a producer's reported rank or degree.

The native tool implements this bounded refutation audit for small explicit
ANF systems. It also exhausts their Boolean assignments as a correctness
reference. It does not prove a degree bound for a factor base until an
exporter supplies the complete membership-plus-summation system, with all
selectors and exceptions represented. Exporters for the sixteen proposed
families are separate implementation obligations.

For satisfiable systems, retaining one found point is insufficient. Compare
the complete solution set with the truth table on complete small cells.
For a full Gröbner claim, retain a reduced basis G, original-generator
membership witnesses, the converse ideal-containment witnesses and reduction
of every required S-pair (including Boolean field relations). Fix the
monomial order. A solver's last input degree or returned basis degree does
not bound the degrees used to produce it.

For homogeneous regularity, state the homogenization, field equations,
saturation convention and invariant being measured. Obtain a Hilbert-series
or resolution certificate for that object. The Boolean refutation result
above is a different invariant. This round retains the user's regularity
goal and makes that additional proof obligation visible.

A proof about a sample of targets is about that sample. A proof about every
target of a tiny group is about that group. A bound for a degree-83 base
requires complete degree-83 coverage or a symbolic argument covering the
stated family, including all selectors. A branch cap cannot prove the latter.

Lean proof map, pending: finite-field and subgroup admission lemmas are
established references; new uniform degree theorems have not been asserted.
A future uniform theorem requires the exact polynomial family, a constructive
certificate-generation argument and the kernel-checked formalization of that
argument. Checking numeric inequalities alone would not discharge it.
