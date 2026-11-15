# Public branch-coordinate proof target

Pure local proof development, 2026-11-13. No solver or timing experiment.
Continue the explicit public basis plan in faithful-report/NEXT_CORANK.md.

For ell_0=1, define T(z)=(-sum_j ell_(j+1)z_j,z). Tail deletion is its
inverse on ker ell. Attach ell to the actual upper/lower polynomial families
and prove its first coefficient is1 from the pinned product monomial basis.
Define the restricted matrix by actual congruence, transfer arbitrary-vector
pairing, and transfer the existing branch-radical theorem to an exact reduced
kernel statement. A numeric rank, witness oracle or unspecified branch basis
must not replace this attachment. Corank/adjugate and efficient moment finding
are distinct further obligations.

Sources checked (KB unavailable): prior NormMatrixFaithful, NormMomentMatrix,
NormBranchRadical, faithful-report/NEXT_CORANK.md; pinned Mathlib
RingTheory/Polynomial/DegreeLT.lean basis_val/basisProd_castAdd and Fin
sum/cons APIs. This is standard linear algebra, no novelty claim. Current
admission is unobserved, not a freshly established exhausted balance.
