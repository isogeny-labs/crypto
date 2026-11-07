# Closed fixed-difference Kummer walk

Frozen 2026-10-22 UTC before running candidate transitions. Inputs are the
21-model, 1,260-target source-verified cohort and checked fast-coordinate
constants/matrices from `research/kummer_coordinates_20261022/`.

For fixed signed G, state is `(K(D), K(D+G), a, b)` where `D=aP+bQ`.
Each step computes `K(2D)`, `K(2D+G)`, `K(2D+2G)` by three fast xADD calls
with known differences `K(0)`, `K(G)`, `K(0)`. A quotient-based hash chooses
`D'=2D` or `D'=2D+G`; the pair shifts accordingly. Coefficients update
`(a,b)->(2a,2b)` or `(2a+a_G,2b+b_G)` modulo the source subgroup order.
No Jacobian operation may enter ordinary transition arithmetic. Every
zero-output xADD must use the recorded two-torsion shift repair, and every
remaining exception must be retained as an explicit row.

First run source-label and quotient-transition checks on all 1,260 targets
for at least three steps each. If all gates pass, plug this walk into the
existing signed genus-two rank collector. Obtain signed divisors only at
relation candidates using checked Recover; preserve the bounded `D=0`
fallback counts. Run all 1,260 targets paired against same-source rho in
all 36 pricing cells, and publish raw, standardized summary, PDF and graph.
If a gate or cost run fails, keep exact failed rows and null unfinished cost
fields; no extrapolated candidate/rho ratio.
