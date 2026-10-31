#set page(margin: 2cm)
#set text(size: 10pt)
= A11 pipeline pricing: feasibility report, 2026-10-19
*Outcome.* No ecbench method builds or solves a PKM-tower summation system, so no S was measured for either arm and no speedup is claimed. Full evidence with file and line references: `FEASIBILITY.md` in the parent directory.

== What the pipeline does today
The `prime-kummer` base (`ic_framework/prime_tower.rs`) only lists the curve points whose abscissa is in $g mu_(2^t)$. Every prime-curve oracle wired in `solve_ic_prime` (`ecbench/methods.rs`) decides a relation by group-law additions. The F4 tower engines (`f4_fp_tower.rs`, `f4_tower_generic.rs`) are reached only from diagnostic examples that price no pipeline.

== What would have to change
A new prime-curve decomposition oracle that builds the tower system per target, solves it with the tower F4 engine, lifts and verifies the witness, and charges F4 field multiplications in the counted unit; then A11 as a presentation option on it. Estimate: 1.5 to 3 engineer-weeks.

== Recorded values (existing stage diagnostic, not S)
Per-system F4 field multiplications at m = 3 on p1 = 786433, from `analysis-amendment4.txt`. A11 is lower at t = 2, 3, 4; its fitted growth per bit is steeper (8.769 against 7.468).

#figure(image("graph.svg", width: 80%), caption: [log2 field_ops per F4 system, A0 against A11. Values in `data.csv`.])

#table(columns: 3, [design], [t], [field_ops],
[A0],[2],[508104],[A0],[3],[238512767],[A0],[4],[15920614779],
[A11],[2],[4663],[A11],[3],[4788601],[A11],[4],[855071386])
