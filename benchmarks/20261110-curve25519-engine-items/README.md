# 20261110-curve25519-engine-items

Copies of the figure and report of `research/curve25519_engine_items_20261110/` (two engine options
on the Curve25519-family miniatures, k = 20 and 24, single-target ecbench: (A) `wwalk cv=0`, the
target-in-walk arm without per-collision recomputation; (B) `rho.negation_chained`, a chained
jump set-up for the rho-neg reference). Not a `benchmarks/README.md` row: no validation gate was
passed and the arm is not decomposition-based index calculus. (A) measured 0.879x and 0.949x the
current arm; (B) measured 1.53x and 2.74x rho-neg and is rejected as a reference. Records:
`benchmarks/records/20261110T0357Z-curve25519-engine-items-k20.json`,
`benchmarks/records/20261110T0357Z-curve25519-engine-items-k24.json` [pcc:unverified].

**Validated copies (2026-11-10).** An independent validator passed the study with errata
(`research/curve25519_engine_items_20261110/validator-report.json`). The record-pinned
`report.pdf`, `engine_items.png` and `engine_items.svg` above are unchanged. The corrected report is
`report-validated.pdf` and the corrected figure `engine_items-validated.{png,svg}`: erratum E1 (the
measured cv=0 code kept the sign bit in a second, uncounted hash map) was fixed and (A) re-measured
on 16 fresh sessions under PROTOCOL.md Addendum 1. On the fixed code twcj-cv0 / twcj measured 0.879
[0.876, 0.881] at k = 20 and 0.950 [0.948, 0.951] at k = 24 (w = 0), A1 to A4 pass, and the
distance remaining twcj-cv0 / rho-neg measured 1.217 and 1.259 at w = 0. Records:
`benchmarks/records/20261110T0542Z-curve25519-engine-items-a1-k20.json`,
`benchmarks/records/20261110T0542Z-curve25519-engine-items-a1-k24.json` [pcc:unverified]. Still not a
`benchmarks/README.md` row: no `VALIDATION.json` gate entry was filed.
