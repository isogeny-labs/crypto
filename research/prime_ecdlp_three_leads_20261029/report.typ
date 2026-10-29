#set page(paper: "a4", margin: 20mm)
#set text(size: 10pt)
#set heading(numbering: "1.")
= Prime-field ECDLP: three leads
Development and formalization follow-up · 29 October 2026

This work adds native development paths and checked structural proofs.
No measured scientific panel or ECDLP speedup is established here.

#image("status.svg", width: 100%)

== Symmetric PKM
The new native successor accepts finite curve targets, constructs the symmetric
system, visits F4 solutions, factors their root polynomials, lifts points and
verifies signed sums. Construction and extraction are separate intervals.
Unrestricted field branching is retained in the counters. Repeated abscissae
remain outside this representation; arithmetic counters are partial.

The original construction comes from crypto PR2650 at
`adde64fdc22d51dc180fe0b9f30a2d28adf992fe`. The historical instruments and records
are preserved. The follow-up is an oracle implementation, not a complete
factor-base logarithm and target-descent pipeline.

== Five-sum replay
The exact prior source is PR2852 at
`42e460293269d149f4ead6929476fcc993babfea`. The frozen raw sessions are store-backed.
The ordinary unauthenticated object request returned HTTP 403. No independent
replay, recomputed performance result or validation admission is claimed.

== Hecke target interface
The native control consumes certified known-log points and an arbitrary public
target, checks labels and n-torsion, tries bounded generic shifts,
and verifies any recovered scalar. It separates setup and online ledgers.
No new Hecke relation constructor or efficient natural target attachment is
supplied. The D79 generalized-curve importer and coordinate certificates remain
an explicit next step. The development fixture is not a D79 performance study.

#pagebreak()
= Lean proof boundary
Lean 4.34.1, Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Eight named structural theorems in `lean/ThreeLeads.lean`:

+ A finite distinct-root factor polynomial divides $X^d-1$ exactly when every
  proposed root has $d$th power one.
+ Repeated squaring gives $x^(2^t)$ in a commutative monoid.
+ Certified table labels and an injective scalar encoding make a hit correct.
+ Valid scalar-row combinations reproduce their combined right-hand side.
+ A natural target attachment with nonzero target coefficient yields recovery.
+ A kernel perturbation preserves all rows while changing a nonzero coordinate.
+ A router with $s$ shifts and a known-log set of size $b$ covers at most $s b$
  targets. Include the identity if it is recognized implicitly.
+ The table-family exponent $k/(2k-1)$ exceeds $1/2$ for integer $k>=1$.

`lake build` succeeded. The consumer proof gate passed with zero trusted
exceptions. A separate `lake env leanchecker ThreeLeads` completed with exit 0.
Transitive axiom output names only `propext`, `Classical.choice`, and `Quot.sound`.
See the exact receipts in `validation/` and the full proof map in `README.md`.

These are standard structural implications, not a claimed new complexity theorem.
The scalar-to-point and natural-attachment premises are explicit. No theorem
connects Rust execution to the formal objects, bounds PKM solving degree, proves
fast Hecke target attachment, or establishes sub-square-root prime-field ECDLP.

= Execution impediments and resumption
The checkpoint gate refused experiment launch. Its upload transport has no
configured AWS profile or checkpoint token in this environment. The Isolab
skill requires preserving that gate. GitHub access alone does not supply the
checkpoint or artifact-store credential. No gate was relaxed and no scientific
panel was silently replaced by local development tests.

`NEXT.md` preserves the three execution contracts: exact-source five-sum replay,
complete-cost PKM comparison with an external solver, and fresh arbitrary-target
Hecke tests with matched generic controls. `PROBLEM.md` names each missing bridge
and its exit condition. Development validation outcomes are retained separately.
The eight focused native tests and strict lint pass. The full library check
was capped at 40 minutes: 5,232 passed, zero failed, 123 ignored, one unchanged
isogeny-walk case unfinished. This is not a full-suite pass. Large raw logs are
not uploaded; committed excerpts and hashes identify them.

= Sources
PKM primary text: https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf .
Internal sources: crypto PR2632, PR2650, PR2852; cryptanalysis PR1274, PR1278,
PR1281; `src/cryptanalysis/f4_fp.rs`; and the pinned Mathlib dependency.
No canonical performance/curve graph is changed because no new measured cost,
curve, isogeny or admitted comparison is established.
