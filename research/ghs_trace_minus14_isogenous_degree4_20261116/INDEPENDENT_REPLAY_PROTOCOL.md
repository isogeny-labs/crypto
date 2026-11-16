# Pre-run direct-linear replay

The CRT screens found zero degree-four numerators in 48 source/carrier
representative cells. To check their decisive necessary condition with a
different algorithm, `direct_linear_replay.sage` builds the coefficient
matrix for a degree-at-most-four numerator at every unpoled quadratic
branch factor. For each of the same four denominator classes and every
two-torsion root assignment, it checks whether the branch-value vector is
in the matrix column space by a left-kernel test, then solves for the five
numerator coefficients. It records both the count of degree-at-most-four
solutions and the count with nonzero fourth-degree coefficient.

Freeze this source before running all 48 pairs `(bmask,fmask)`, with
`bmask∈{11,202,38,80}` and the twelve carrier masks in the main protocol.
Each pair must cover 59,697 assignments. An independent zero count for the
fourth-degree coefficient corroborates the CRT exclusion; it does not
formalize the geometric denominator classification or establish an attack.
