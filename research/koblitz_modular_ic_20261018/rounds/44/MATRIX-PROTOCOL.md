# R44 finite-algebra inverse control, frozen before matrix-control data

Prepare another possible implementation repair while the sole current436
source producer runs. It does not replace or modify that live attempt.
A unit in a finite quotient has an invertible multiplication matrix over
its coefficient field. Build all columns in the exact kbase, solve against
1 by deterministic Gaussian elimination, recheck EVERY original matrix row
and the actual quotient product. All pivot inverses are coefficient-field
operations. Preserve complete quotient/branches, no root choice.

New correctness control: reuse the four accepted degree12algebras; compare
all32registered inverses with old Groebner/lift inverses, while retaining the
52direct/modular polynomial-value comparisons. One new inverse algorithm
control, not a replay of accepted point/Frobenius producers. Native4GiB/1200s.
No timing claim or comparison. Retain failure/cap and exact matrix/product
checks. A source producer may use the committed repair in a fresh attempt
ONLY if its prior affected producer is terminal; never duplicate a live one.
