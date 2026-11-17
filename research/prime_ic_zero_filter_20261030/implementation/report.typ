#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Structured prime-field IC: C1–C3 implementation controls
2026-10-30 · bounded native references; general goals remain partial

*C1 — Exact predicate.* A rational two-isogeny fibre constraint, canonical
rational coordinates and actual subgroup membership are combined with complete
point-summation charts. The special family requires rational two-torsion; it is
not a construction for arbitrary prime-order curves or all PKM auxiliary maps.

*C2 — Count-guided recovery.* Exact counter queries select interface bits, then
native arithmetic rechecks every signed summand and the target sum. Repetitions,
inverse pairs and intermediate identity sums are retained. Resource exhaustion
means unknown. The functional leaf still enumerates inputs, so this is not a
new grid-free complexity bound.

*C3 — Local message reuse.* A shared reduced decision diagram represents local
membership. One constructor exhausts coordinate encodings; the other solves
quadratic fibres, lifts both rational signs and checks subgroup membership.
The root constructor avoids the field scan. Setup and storage are charged.
It is not a compact contracted message for the full summation predicate.

#image("structure.svg", width: 100%)

*Checked scope.* Exact counts and witnesses agree with direct addition on all
three targets of the order-3 subgroup of y²=x³+1 over F5, at arities2 and3.
Two-point counts are2,1,1; three-point counts2,3,3. On y²=x³+x over F19,
order5, all signed base-pair/target predicates agree. Every single image fibre
in both fields compares the root-based message, exhaustive message and direct
membership, including poles, empty fibres and noncanonical coordinates.

*Limits and evidence.* The successful query controls use fresh500million-step
envelopes; the default100million is unchanged. Earlier compile errors and
resource failures remain in checks/. Graph edge counts above are structural
software diagnostics, not benchmark or cold-total costs. No unknown-target
solve, isolated timing panel, independent-host replay, Lean proof or new
mathematical result is asserted. Source hashes, raw checks, exact parameters,
reproduction commands and the open full-message obligation are in README.md.

*Sources.* PKM2016 sections3.1–3.4; Hexagon2610.00594v1,2610.00600v1 and
2610.00613v1; Kosters–Yeo1503.08001v3; existing prime-IC, isogeny-fibre and
torus studies. Cached primary texts were used; fresh web requests failed, and
the earlier2018 full-text gap remains. Global novelty and an IC improvement
are unestablished. Existing leaderboard/frontier graphs gain no qualified data.
