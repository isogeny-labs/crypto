# R89: transport all 32 normalized RR generators

Use the R50 accepted `rr-pid50-01` basis, `originalLift50`, and
degree-16 relative source equation. For each of the 32 polynomial
module generators, substitute the actual R85/R86 forward diamond
map, clear the common pole bound by powers of `h=z+V` and the
R86 registered denominator, and solve for the image in the PID
basis with a full module lift. Repeat with the R88 inverse using
`k=p+q+p²` and `G=p k²+p+k²`. Retain both denominator and
module-lift witnesses. A failure must identify the exact
generator and obstruction; do not infer normal-chart gluing from
the rational field inverse alone. The 132-section finite-u RR
intersection is an independent control route.
