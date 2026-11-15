# Frozen independent replay of the degree-four necessary conditions

15 November 2026. This replay checks the 59,697 branch assignments in `DEGREE4_ALL_PROTOCOL.md` using direct linear algebra instead of CRT reconstruction. It must be committed before execution. [pcc:unverified]

For each denominator and assignment, write `A=a0+a1 x+···+a4 x⁴`. The condition `A≡B·ej (mod Qj)` contributes two linear equations over `K` for each included quadratic factor. Build the `8×5` or `10×5` matrix of remainders of `1,x,...,x⁴` modulo each `Qj`; compute its left kernel once per factor set. For each assignment, form its right-hand side directly from `B·ej mod Qj`. It is consistent exactly when all left-kernel equations vanish. Solve any consistent system and record whether `a4≠0`. Count assignments, consistent degree-at-most-four systems and degree-exactly-four systems separately for the four denominator shapes. [pcc:unverified]

The replay uses the same exact curves, factorization and 2-torsion roots, which are necessary input data. It does not call `CRT_list`, read the producer's result, or use its candidate filtering function. A match of all four assignment counts and zero degree-exactly-four solutions would independently check the computational exclusion, while the mathematical completeness of the denominator classification remains a paper proof. [pcc:unverified]
