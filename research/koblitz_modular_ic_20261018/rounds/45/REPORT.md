# R45: actual-source collectors through residue degree13

These are exact fixed-chart enumeration observations and conditional deck
proofs, not an independent evidence verdict or IC performance comparison.

| Attempt | Field | Actual residue degree | Rational source points | Closed points | Ten-point deck orbits |
|---|---:|---:|---:|---:|---:|
| control7-01 |128|7|420|60|6|
| collect9-01 |512|9|540|60|6|
| collect11-01 |2048|11|2860|260|26|
| collect13-01 |8192|13|7540|580|58|

Four native attempts and one Lean workflow pass; no rejected attempts in R45.
The degree7export is byte-identical to R39. R39donor polynomial/point files
match its sealed manifest. The composite9search found no lower-degree source
points on the retained chart. Every point passed all5source equations,8units,
Tate11order, coefficient descent, deck/Frobenius closure and local rank2.
The new native lambda candidate counts are276480,4055040,66027520; these
are detector inputs, not independent relations. All outputs and source/binary
hashes are indexed in results.json. No timing or speed comparison is made.

The new sizes supply6+26+58=90new complete ten-point deck orbits. Distinct
closed Frobenius classes within each orbit avoid immediate sign/scalar
stabilizer cancellation. They can still be principal, share the same Picard
image, or map to zero in the desired factor. None of these new classes has
been evaluated in Picard arithmetic or transferred to the selected target.

## Formal and method scope
Three named Lean statements prove mixed-zero from sign fixation, mixed-zero
from scalar two-step fixation, and the conjunction of their necessary
nonzero conditions. The generated definition equation brings the audit to4;
0failing/0trusted. No native arrays or global proper source are formalized.

Field storage is widened to u16coordinates/u32convolution. All products agree
with independent carryless polynomial reduction; nonzero inverses, no zero
divisors, basis-triple laws and all physical quadratic inverses are checked.
This replaces the old cubic exhaustive triple loop with a basis check plus
exhaustive product equality; R39point-set control is unchanged. Unused-loop
variable and audit-deprecation warnings remain in raw logs.

Prospective degrees7/9/11protocol was committed490a0e7c4; degree13amendment
6b9067245was committed before its affected data, after9/11outcomes. Same
4GiBvirtual/1200s native build/run caps; pinnedLean4.34.1/mathlibd13f23b,
-j1-M4096/1200s. All5attempts terminal. No agents, new EKSjobs, caps or claims.

## Corrected leads and unfinished construction
R44NEXT's proposed500/580screen was stale. R13already exhausted those
ordinary supports and proved projected zero by scalar-fiber averaging under
its written bridge. R44sign obstruction remains a separate rejection route.
No repeated500/580producer was launched.

The existing degree128representation has degree-compatible padding choices
P9+21Q+2R0, P11+22Q+R0 and P13+23Q (degQ5,degR0=7). Existing jet precision
covers those padding multiplicities, but new evaluation kernels and complete
space dimensions must be checked before calling these actual class inputs.
R46separately develops exact univariate root collection, requiring full
point-set equality before use. Its outcomes are not part of R45data.

Proper level77chosen-model map, arbitrary-divisor reverse action, full
quotient saturation/polarization, theta/node187 and genuine larger-ntransfer
remain open. Larger residue fields on fixed n5do not establish Koblitz-nor
attack scaling. Independent scientific replay/semantic review are pending.

See PROTOCOL.md, FORMAT.md, LITERATURE.md, results.json and NEXT.md.
