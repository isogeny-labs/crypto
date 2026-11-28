# Bounded verified Krylov target witness

Goal is S^T w=e_k for rectangular Schur operator S, not recovery of every
seed coordinate. Only Sx and S^Tz callbacks are available to the finder.
No explicit matrix, reference solution or true target scalar is an input.

For each of3prescribed attempts choose nonzero row/column diagonals D,E and
a scalar projection u. Define A=E*S^T*D*S and b=E*e_k. Generate2n+2terms
u^T A^i b, then Berlekamp-Massey recurrence c0=1,c1,...,cd. Associated
polynomial is f(t)=t^d+c1*t^(d-1)+...+cd. If d=0orcd=0, retain a failed
attempt. Otherwise reconstruct y=-(1/cd)*sum_{j=0}^{d-1} c_(d-1-j)*A^j*b
with c0=1, redoing the Krylov traversal rather than storing its vectors.
If f(A)b=0this gives Ay=b. Form w=D*S*y and verify S^T w=e_k EXACTLY.
Accept only that identity; projection failure or a bad preconditioner cannot
create an incorrect accepted witness. Return unknown after3failedattempts.

The diagonal preconditioning is not asserted always successful, particularly
for singular/rank-deficient cases. No unweighted normal-equation equivalence
is assumed. Unchecked A^TA or scalar recurrence alone is not a certificate.
No finite-attempt success probability is claimed. The final exact witness
is valid regardless of why a projection succeeded, including rank deficiency.

Storage is O(n+r)field words beyond graph/operator and output witness:
diagonals, projection, recurrence/sequence and a constant number of vectors.
No n-by-n matrix or full Krylov-vector table is formed inside finder. Each
Aapplication uses one S and one S^T plus diagonal scaling; BMadds O(n²)
scalar arithmetic. All3attempts and final verification remain real costs.
Bounded success and asymptotic storage alone are not a10x result.

This is an implementation of classical verified Wiedemann ideas. Original
paper sectionII supplies recurrence-based inversion; singular/nonsquare
cases require care, motivating explicit certificate checks here:
https://www.enseignement.polytechnique.fr/informatique/profs/Francois.Morain/Master1/Crypto/projects/Wiedemann86.pdf
No novelty/priority claim. Group-level relation/Schur correctness is separately
qualified by peeling-operator. Unknown-action rings and production cost are
not covered by this known-field solver gate.
