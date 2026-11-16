# Exactness of the degree-peeling relation solve

All linear algebra is over the archived prime subgroup order `r`. A
relation row has factor-base coefficients `f_i`, a target coefficient
`b`, and a known generator coefficient `-a`. Before insertion, the native
collector checks the three rational curve points, their square roots,
the complementary divisor, and the Jacobian identity. The transfer
checks ensure that the accepted row represents a true identity in the
same subgroup as the source generator and target.

For one elimination attempt, let `A` be the matrix of factor-base columns.
If a column occurs in exactly one live row `j`, any row combination `v`
with `v A = 0` must have `v_j = 0`, because that column's sole nonzero
entry has no other term to cancel it. Row `j` can therefore be removed
without removing a target-solving dependency. Repeating this argument
proves the degree-one peel is exact for the current relation set. The
implementation reconstructs that set at every walk, so a discarded row
can participate in a later attempt after more relations arrive.

In the remaining core, the solver selects a live factor-base column and
a nonzero pivot row, divides by its pivot coefficient, and eliminates the
column from every other incident row. These are invertible elementary row
operations over `F_r`. It updates the incidence index after every row
change. Thus any produced factor-base-free row is a consequence of the
accepted group identities. If its target coefficient is nonzero, the
known coefficient divided by it is a scalar candidate. The candidate is
accepted only after direct source multiplication verifies both the target
point and archived planted scalar. A factor-base-free row with zero target
coefficient and nonzero known coefficient would contradict the checked
group identities and is asserted impossible.

This proves the algebraic validity of a recovered scalar and that peeling
cannot remove a dependency from a fixed relation set. It does not prove
that the relation stream is sufficient, that a configured matrix or wall
cap will be reached only after a solution, or that the implementation has
the lowest possible cost. Those are measured separately, with failed
attempts and null cost ratios preserved.
