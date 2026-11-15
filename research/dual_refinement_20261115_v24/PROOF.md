# Iterated common-dual refinement (prepared; not executed)

Let a finite-field affine matrix pencil be M(u)x=b(u), with outer domain u=p+B t. Form the common left annihilator N of the constant matrix M(p) and every direction matrix M(B_i). For each lambda in N, every feasible pair satisfies

lambda b(p) + sum_i t_i lambda b(B_i) = 0.

Solve these affine linear conditions in t. Inconsistency certifies that the entire outer cell has no feasible pair. Otherwise substitute a particular solution and a basis for the homogeneous kernel into p+B t, and recompute the common annihilator on this smaller domain. Every substitution preserves all feasible u by induction; it does not assert that every retained u is feasible. The absence of a strict dimension reduction is a fixed point of this filter only. Each strict step drops dimension, so there are at most dim(B) strict steps and one final test.

This argument works in every characteristic. The prepared implementation specializes to binary packed arithmetic and the exposed fixed norm-span pencils. It neither assumes squarefreeness nor divides by a coefficient. It is not an implementation for odd fields, a complete norm search, or a complete original polynomial-root solver. Characteristic-specific matrix assembly still requires its own proof.

For m equations, n inner variables and d outer coordinates, a dense implementation computes a left kernel of at most (d+1)n rows and m columns, plus affine elimination in d variables; storage is polynomial in those sizes. Outer point enumeration in this diagnostic is limited to d<=9 (512 points) and is not a proposed full-width search. Production use should return affine cells rather than enumerate their points.

Falsifiable next prediction: on the six-bit actual pencil the dimension path may be 6 to 4 to 3, recovering exactly eight feasible directions. Failure means the fixed point remains an overapproximation; it requires a changed guarded split or higher-order certificate. No such path has yet been measured. Tiny exhaustive affine-domain controls and comparison with independent recorded truth sets must precede use.
