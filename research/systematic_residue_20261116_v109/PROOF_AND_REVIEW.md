# Same-family exact block coordinates and lazy affine graph import

The coordinate proof is C108 SYSTEMATIC_RESIDUE_NEXT.md. E maps all76 original
bits, including the three global polynomial lifts, to the first72 observation
bits. E*R=I,E*K=0 and rank[R K]=76 certify theta=R*(y+b)+K*z is bijective.
Every leading guard and old q bit is evaluated in recovered theta, never
identified with a new kernel bit by position. Full-field H and source/U
recovery also use original theta. No new independent local selector exists.

Graph import computes complete affine forms in new primary bits through every
old affine XOR/NOT subgraph. It emits gates lazily only where nonlinear use
or output requires them. Constants, equal affine factors and complements obey
exact Boolean identities. Every other gate is imported in topological order;
induction proves every output equals the old output on recovered theta.
Output-ancestor pruning is exact. New input coordinates expose first-seven
coefficient blocks; conservative dependency unions explicitly certify each
block uses only its own4s input positions. Other residues retain all coupling.

Tiny circuits enumerate every assignment and compare outputs with an
independent old-circuit evaluator, verifying unique inverse coordinates.
Actual controls check E/R/K,256inverse samples, all14 numeric nilpotency
predicates and original lift/infinity guards. These supplement the algebraic
proof rather than replacing it with samples. Random variable ordering does
not establish novelty or speedup; systematic coding and affine import are
known methods used here in a new hard-family representation.

Only this fixed necessary affine family is covered; original339-variable
chart/ghost/source/U completeness is not claimed. Source and input hashes,
one core,4GiB,90-second outer supervision and78-second worker demotion remain.
A new native search includes fresh coordinate/graph construction, original
model back-mapping, support verification and reporting. UNKNOWN remains
inconclusive and engine UNSAT needs independent certification; no timeout or
smaller graph is a speed ratio. Requested prime-field problems require their
own valid parameter maps, not an assumed F2 prefix domain.
