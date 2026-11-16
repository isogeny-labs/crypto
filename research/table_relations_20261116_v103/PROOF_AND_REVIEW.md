# Exact low-degree vanishing relations, necessary-only contract

For a finite allowed set A of bit strings and all squarefree monomials of
degree at most d, let E[a,m]=m(a). A coefficient vector c is in ker E exactly
when its Boolean polynomial vanishes on every allowed tuple. Highest-pivot
row elimination preserves row span. Assign each nonpivot coefficient in turn
to one and solve pivot coefficients in ascending order: each pivot row has
only lower columns. This gives dim ker E independent vectors and hence the
entire degree-bounded vanishing space. Every returned basis vector is checked
on every original allowed row, not only the elimination basis.

Substituting actual coefficient bits as affine forms in the same 76 global
parameters preserves necessity and does not increase Boolean degree. Reduce
repeated variables by theta^2=theta. No independent local q or coefficient
selector is added. All original roots in the already scoped affine family
survive. The zero set of these low-degree cuts can strictly contain the table;
the exact nilpotency/table predicate, full prefix support, all divisor charts,
ghost branches and original variable recovery are still required for the
original 339-variable all-roots contract. This worker returns none of those.

Controls exhaust every polynomial coefficient vector for tiny Boolean spaces,
using an independent direct monomial truth table. Empty and full allowed sets
are included. Odd prime and extension controls use literal finite-field root
products; bit encoding does not authorize reducing NIST residues modulo an
unrelated prime or assuming a proper prime-field subspace.

Fourteen binary residue fields are fixed from C102, q<=32. Features through
degree three at width20 total1351, below the frozen2048 cap. The largest
evaluation matrix has11781 rows of1351 bits; memory is bounded by that matrix,
row basis and explicit relation vectors, comfortably below4GiB. All setup,
table generation, derivation, exact positive verification and serialization
count against78-second worker demotion and90-second supervisor limits. A late
or partial result cannot carry an original exact count/completion claim.

Review: the constant monomial is bit0 in feature coordinates, not confused
with the constant coordinate bit in a coefficient code. Bit string monomials
are squarefree; nullspace reconstruction order is ascending for highest-pivot
rows. The brute oracle checks the whole kernel, not only positive membership.
No false exclusion is permitted. Ranks/cut counts and single timings do not
establish a completed solve or a comparative speed ratio. Interpolation,
evaluation codes and low-degree annihilators are established methods; the
novelty of a complete new rootfinding reduction remains unestablished.
