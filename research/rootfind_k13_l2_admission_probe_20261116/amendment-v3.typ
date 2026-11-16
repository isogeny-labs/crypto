#set page(paper: "us-letter", margin: (x: 0.72in, y: 0.67in))
#set text(font: "New Computer Modern", size: 9pt)
#set heading(numbering: none)

= K13 admission probe: Amendment 2

*Before-data coverage correction.* The native EKS benchmark path rejects any
record below L2. The frozen ten-arm K13 panel includes rho and five
rootfinding arms. The v2 probe screened only batch m3, its A/A copy and
F6-IC; it could pass while an omitted panel arm made the full job fail
admission. No v1 or v2 probe has run. Their files remain unchanged.

#figure(image("admission_v3.svg", width: 60%), caption: [The v3 screen covers every non-Gröbner arm before the full comparison.])

*Intended v3 cell.* `SPEC-K13-ADMISSION-v3.json` copies all six non-Gröbner
arm objects exactly from the frozen panel: rho-strong (the actual reference),
scalar rootfinding m3/m2, batch m3/m2 and the batch A/A control. It preserves
all eight Koblitz E0 n13 public targets, factor-base dimension five and
algorithm seed. One warmup plus five measured rounds gives 288 processes,
240 measured. The static source/spec replay in `VALIDATION-v3.json` checks
exact arm and workload identity, the native launcher's static admission
predicates and execution count. It is not a native run.

*Decision boundary.* Every record, including warmups, must have a scalar
satisfying `[d]G=Q`, complete clocks and phases, L2/L3 grade and solve wall
at least 1 ms. A v3 pass permits the unchanged 560-execution panel only
after its own gates. The four Gröbner arms and any solver speed comparison
remain for that panel. The correct EKS account currently exceeds its daily
and monthly limits, and another task holds the checkpoint path; no new Job
was submitted.
