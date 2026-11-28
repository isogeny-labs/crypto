# Exact affine quotient and conservative Boolean closure

Contract: preserve exactly the satisfying assignments to the same 76 primary
parameters of C101's guarded necessary circuit. This is not the original
339-variable all-roots contract. No field, source, lift or exceptional branch
is removed. A native timeout or UNSAT assertion supplies no original completion.

Replace each AND output by a fresh Boolean atom. Every other wire becomes an
affine form in the primary and AND atoms, by induction on circuit order. Retain
each equation z=a*b. Output-zero conditions are affine equations. Highest-pivot
Gaussian elimination preserves their solution set; after choosing all free
atoms, ascending pivot substitution gives exactly one full assignment.

In the quotient, infer linear equations only from these Boolean identities:
0*a=0, 1*a=a, a*a=a, a*(a+1)=0, and a*b=1 implies a=b=1. Each inserted row is
necessary. Conversely all original AND equations remain enforced after
substitution, except syntactic identities that hold for every free assignment.
Thus even if the 16-round propagation cap is reached, there is an exact
bijection between reduced satisfying assignments and original circuit models.
The circuit's functional evaluation then gives uniqueness over each primary
assignment. Restoring every original field variable remains separate work.

The implementation uses integer bitsets, with a preflight limit of two million
affine support terms before solver formulation. Dense fill or a cap is an
inconclusive experiment, not a discarded branch. Atoms and products scale
linearly in the gate count before substitution; actual affine support is
measured rather than assumed sparse. Solver setup and preprocessing both count
against the same 45-second deadline as C101; process reporting has an outer
90-second supervisor limit and 78-second worker demotion boundary.

Review: substitution leaves no pivot atoms in solver expressions; constants
occupy bit zero; reconstructed primary bits start at bit one. Contradictory
linear rows are retained as bad=true. Independent small exhaustive truth tables
check projection, completeness and unique recovery, including constants and
contradictions. Every actual solver model is rechecked in the original gate
circuit and numeric field predicates. Whole-H blocking is restricted to proved
unsupported H; a supported H pauses for original recovery. No native UNSAT
certificate is independently accepted. No baseline comparison is claimed.

Novelty boundary: affine substitution, Boolean idempotence and XOR-aware SAT
are established methods. This cycle tests their effect on this specific
shared-parameter local-nilpotency system. It makes no claim to invent Gaussian
elimination, SAT/XOR solving, or a novel general root-finding algorithm.
