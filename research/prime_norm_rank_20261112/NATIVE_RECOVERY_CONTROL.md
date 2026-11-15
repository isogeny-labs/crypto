# Native finite regression contract

Local implementation correctness control only. No attack solver, timing panel,
scientific scaling or paid experiment. Source/protocol committed before new
Rust execution; not a claim of external preregistration.

Reuse exactly the existing NormNonconstantControl over F17:
F=normA=[16,12,8,1], H=[8,1], f=[2,2,0,1], U=[12,6,11,16], V=[4,6,1],
q=0,y=6,alpha=1,beta=6; coefficient lists are ascending degree.
Positive monic G has roots[0,3,5,6,10,13]. Negative certificate control G'
has roots[0,3,5,6,7,10], replacing13 by7. Both have six distinct rational
nonzero-y abscissas on the same curve, include q=0 and are monic degree6.
G' is wrong for this fixed norm, not asserted globally unsatisfiable.

Build actual6-dimensional product monomial families and branch ell, the
quotient moment from polynomial remainder modulo F, and actual5-dimensional
reduction. Compare explicit entry formula with literal T^transpose*S*T.
Check F/H monicity/degrees, Bezout identity and branch, independent positive
root-product versus polynomial norm, and all listed point lifts.

Use Gaussian elimination for rank and an independent Leibniz determinant
for determinant/adjugate. Include a nonsymmetric2-by2 adjugate control to
catch accidental transposition. Check both products with the adjugate,
actual radical coordinates and scaled-column factorization.

Predictions before execution: reduced rank4, determinant0 and nonzero diagonal
adjugate. Positive cofactor pairings all zero. Negative certificate pairings
not all zero, exactly matching the nonzero scale times actual coefficient
residuals. The same S and its rank are used in both cases. One invocation
contains two deterministic controls, not two independent physical runs.
No subgroup/DLP solution, generic relation yield or asymptotic gain follows.

Freeze implementation before producing a write-once JSON artifact. Preserve
failures, actual exit receipts and source/input/output hashes. No original
proof source/output or existing benchmark is modified [pcc:unverified].
