# Saved-record cover input audit

2026-11-13, frozen before the new native validator executes. Exact correctness
check of two saved records, not a new solver, timing or scaling experiment.

Question: what are the distinct rational-base occupancies in the two completed
round5 records, including the unreported reference count? Do the saved tower,
cover and target inputs satisfy the stated arithmetic constraints?

Inputs: crypto@cc986a93b, the frozen round5 m4t4-none.jsonl and
m4t4-cubic.jsonl. SHA-256 respectively
88f5ad4b1d972c3d78f2d64cad5934788ff93e7f203c44528c833a185ac7ba7e and
f9eeef35b50b7dd5e0f282f1c700e1d3f762f21e8cc0c31f074ae761e31bf895.
Generator/source snapshot3466f3be9e; actual binary digest recorded by the
producer923a1c95d29a48aac30608fdf2c05692c1860231d880ececf2d7aec102ca8bc1.
The validator does not execute that binary or replay its F4 computations.

Complete enumeration of exactly16 Kummer abscissas for each fixed record:
verify the31-bit prime by bounded trial division, curve nonsingularity,
primitive16th root and distinct coset points. Apply the recorded polynomial
cover (or identity) and independently evaluate its image cardinality and
nonzero quadratic-residue filter, matching the producer's exclusion of y=0.
Check the target abscissa's residue status. Retain every point and residue.

Acceptance: all arithmetic guards pass; the cubic image cardinality16 and
on-curve count8 match its saved fields. The reference count is unknown before
this audit and is reported whatever it is. Different curve/target draws remain
explicit. No scalar, subgroup order, relation yield or F4-basis correctness is
inferred from these fields. Recorded solver counts are retained as recorded.

Controls: field17 has square residues2 and1 and nonsquare5; root1 fails exact
order16; altered input digest or unsupported field/depth fails before audit.
Inputs are not regenerated or selectively replaced.

Resources: local bounded native correctness check, two saved rows, each16
points; p<=2^32-1 bounds primality trial division. No solver call, benchmark,
paid job or timing output. Monthly isolab budget remains exhausted. This does
not authorize an experiment rerun or circumvent the standing run protocol.

Sources checked: full original protocol/amendments, both launch scripts,
completed rows/progress, frozen cover-map and summation code, earlier auxiliary
cover review. KB MCP/wrapper unavailable; three named GitHub fallback queries
for auxiliary covers, cover-map auxiliary variables and cover-map lift prior
work. No novelty or repository-wide absence claim. A separate graph encoding
is only a proposed standard representation change, not a demonstrated gain.
