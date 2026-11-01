# Exactness of the Markowitz-priority elimination

The candidate receives the same accepted relation rows as the existing
point-driven pencil collector until either method stops. Each row is checked
against the Jacobian relation identity before insertion. The only new rule
chooses which live factor-base column and incident row to pivot next.

A degree-one factor-base column occurs in only one live row. Any linear
combination that cancels that column has coefficient zero on its sole row,
so removing that row cannot remove a target-solving dependency from the
current relation set. The solver repeats this reasoning as the incidence
matrix changes. For every remaining pivot, its coefficient is nonzero in
the prime field `F_r`. Multiplying the pivot row by its inverse and
subtracting multiples from incident rows are invertible elementary row
operations. The Markowitz score and tie-break only reorder these valid
operations; they do not change the accepted relation identities.

The lazy priority heap is checked against the current incidence and row
lengths when an entry is popped. Every column touched by a removed or
updated row receives a refreshed entry. Thus the chosen tuple is current
among live columns, up to duplicate entries that represent the same tuple.
An extracted row with no factor-base coefficient yields a scalar candidate
only if its target coefficient is nonzero. The binary accepts that scalar
only after direct multiplication on the archived source curve matches both
the target and planted scalar. The same-process rho scalar is checked by a
fresh source multiplication, and an independent process replay checks all
non-wall fields.

These arguments establish algebraic soundness and exact preservation of a
dependency under peeling and elimination. They do not establish that the
fixed relation stream is sufficient before the matrix, memory or wall cap;
those are measured outcomes with null candidate ratios on failure.
