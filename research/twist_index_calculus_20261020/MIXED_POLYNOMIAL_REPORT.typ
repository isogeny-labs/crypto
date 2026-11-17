#set page(paper: "a4", margin: 17mm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Mixed-component polynomial index calculus on a quartic twist

22 October 2026. Canonical quadratic twist at p=37, b0=2 over Fp⁴,
prime subgroup r=1,873,633, one public target Q=[271]G, seed zero.
Frozen protocol: QUARTIC_TWIST_LINE_PROTOCOL.md. Native records and
independent audits: MIXED_POLYNOMIAL_REPORT.md and MIXED_POLYNOMIAL_RUNS.md.
This PDF is the rendered results summary; the Markdown report contains
the complete evidence links, algebra and requirement table.

== Mechanism

The transported Frobenius and cubic automorphism preserve the L0=Fp and
L2=t²Fp abscissa lines. Their combined signed action has order 24 and
abscissa orbits of size 12. Thus 36 physical L0 plus 12 physical L2
factors give four log columns. Adding L2 raises physical relation yield;
orbit folding reduces the number of independent log unknowns. Transforming
one entire relation only gives a covariant row for the transformed
residual, so symmetry does not create independent rows at fixed base and
residual.

With one signed L2 point peeled, k=1 uses the quartic summation polynomial
S4 in the three symmetric L0 abscissae. Four verified k=1 relations from
eight frozen residuals have anchored rank four, solving three factor logs
and the target log. Flat and cubic-only encodings would have 48 and 16
anchored unknowns for the same physical base, so four rows are insufficient.

== Polynomial systems and pilot

For a=0, e1=x+y and e2=xy, the cubic equation at residual abscissa z is:

#align(center)[`S3 = z²(e1²−4e2) − z(2e1e2+4b) + e2²−4be1`.]

The k=2 arm explicitly scans all p² symmetric pairs after peeling two
signed L2 points. The k=1 and k=3 arms use
`S4 = Res_u(S3(x1,x2,u), S3(x3,z,u))`, interpolated and checked at fresh
field points. The quadratic–quadratic Sylvester resultant identity is
`(a2b0−a0b2)²−(a2b1−a1b2)(a1b0−a0b1)`.
Identity certificates IDC1h353e9e2256385dd8 and
IDC1hb4b71bf7bd6cba4e passed 32 sealed and 32 independent checks each;
these finite evaluations do not replace a formal model proof.

#image("mixed_polynomial_pilot.svg", width: 100%)

The first four frozen residuals contain two k=0, two k=1, one k=2,
zero k=3 and zero k=4 signed relations. An independent scalar-keyed
oracle enumerated all 1,873,633 subgroup logs. k=1,2,3 returned complete
pattern-specific sets. k=0 missed one relation; k=4 had a failed system
despite an empty reference set. The first generic k=2 Macaulay solver
failed on all 1,056 systems; a complete Fp² scan repaired that arm.
The degree-30 and degree-31 S5 retries on residual 2 still returned
“border unreachable at this degree.” The native S5 arm remains incomplete.

#pagebreak()
== One-target counted path

The producer reconstructs the curve, 48-factor base, twist eigenvalues,
four-column map and S4 interpolation; processes the frozen residual stream;
retains rank-raising rows; solves and verifies every factor log and the
target. It never calls the exact pair oracle or reads the known target
scalar during relation construction or matrix solving. Input fixture
creation is reported separately. The same-process strongest twist24 rho
recovers the same target.

#table(
  columns: (1.5fr, 0.6fr, 0.7fr, 1.4fr, 1.0fr),
  inset: 4pt,
  [Patterns], [Queries], [Rank], [Counted IC Fp M], [IC group ops],
  [k=1,2,3], [7], [4], [455,838,426], [8,123],
  [k=1,2], [7], [4], [154,272,224], [7,619],
  [k=1], [8], [4], [116,050,118], [3,936],
  [twist24 rho], [672 steps], [target], [348,941], [2,745],
)

#image("mixed_polynomial_cost.svg", width: 100%)

The k=1 path attempted 192 S4 systems, produced four distinct signed
relations, reached anchored rank four and recovered 271. The independent
auditor reconstructed all processed signed sets, orbit coefficients and
matrix, then verified every physical factor log, target and rho replay.
Final-source A/A records match in all non-timing fields. Pruning k=2,3
decreased counted IC field work 3.928-fold on this one target; this is
an adaptive observation, not a result for other curves or targets.

== Cost boundary and next checks

The k=1 IC uses 332.6 times rho's separately counted Fp multiplications
and 1.434 times its counted group operations. A combined S or wall-speed
ratio is not admitted: solver hash/tree and allocator CPU, peak bytes,
rho partition/table CPU, calibrated GAE and isolated intervals are missing.
No same-input msolve or Magma comparison was run. The result is therefore
outside priced leaderboard tables A–C and benchmarks/README.md. The
component panel links this report without promoting a speed claim.

Under a uniform-residual heuristic, k=1 yields O(1) hits per residual when
both line sizes are O(p) and r is O(p⁴), while the fold has O(p) columns.
Explicitly peeling O(p) signed L2 points per query gives O(p²)=O(r^(1/2))
field work if S4 solve cost and row independence stay controlled. Those
assumptions are unverified; this is no lower bound on other solvers.

Next: complete S5 across the tagged union, export identical systems to
msolve, price native hash/allocator and rho CPU and isolated GAE bands,
and run a frozen p=73/p=97 panel with independent replay. The related
problem record is problems/quartic-twist-polynomial-ic-pricing.md.
