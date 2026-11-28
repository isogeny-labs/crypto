# Implementation detail frozen before the Frobenius qualification

For each of the twelve already-fixed m2--7 target cells, use exactly four
input tuples: first m-1 entries are signed point0, last entry is point0,1,2,3.
These are proof witnesses, not a relation search. Replay their group sums
with GMP/OpenSSL and interpret the exact frozen group circuit on them.
Check a nonzero2x2 determinant of the two target-cross-product forms on
these four tuples. This directly certifies the lower-bound lemma's two
independent degree-e vanishing forms. No target or factor-base selection.

The circuit checker recomputes homogeneous degrees from the arithmetic DAG,
checks its only inputs are x_i,y_i,H, checks the formal target-line form
after cutting the three sum outputs, and verifies the pure-Y degree witness.
It does not expand high-m group polynomials or introduce free variables.
Line discriminants and the cubic no-rational2torsion Bezout certificate
are checked again. The universal group semantics use the same published
complete-law theorem and certified rational-point hypotheses.

This detail adds no invocation, resource allowance or target search to
FROBENIUS_PROTOCOL.md. Its source/input hashes still precede the sole180s
native invocation and conditional byte replay. No higher-m result yet.
