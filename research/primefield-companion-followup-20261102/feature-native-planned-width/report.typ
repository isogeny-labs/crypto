#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= Public interval-width planning
T-1884 · Prime-field coefficient construction · 2026-11-07

The optional policy chooses a bounded interval width from 2m before the first stride-m stream. It uses only public eligibility and the prescribed opposite radii. The same borrowed engine serves both streams; no initialized cache is resized.

#image("flow.svg", width: 100%)

*Exact checks.* Twenty scalar-stream panels, 760 coefficient equalities, four invalid-plan controls and twelve recovery-wrapper checks pass. All 2,248 public target/cap comparisons preserve scalar outcomes, routes and source calls.

#image("batches.svg", width: 100%)

*Scalar-stream costs.* Planned sharing reduces combined batches from 10,147 to 9,782 and query trees from 809 to 659. Retained capacity sums rise from 454 to 532 slots. The separate no-rescue snapshots have 5,040 versus 4,869 batches and 454 versus 359 capacity slots: planned progression setup occurs in 11 panels instead of 16. Per-panel rows are retained; these sums are not peak memory.

*Public warm-cache control.* E/F₂₅₇: y²=x³+x+7, P=(1,3), r=281. At m=35, caps 64 and 128 each recover 281 targets in 9,486 source calls with no successful doubled-phase rescue. Both widths use 4,655 batches. Planning changes convolutions/query trees from 490/4,165 to 980/3,675; maximum final retained capacity rises from 22 to 24 slots. A changed evaluation mix is not a priced arithmetic gain.

*Retained small-order panels.* The six m=5 panels keep their source totals and routes; only cap 8 leaves ten unresolved targets. These short products create no retained interval cache. The original cohort is unchanged.

*Evidence and limits.* validation01 passes first attempt with 102 source hashes. PCC-20261107-fb6d9b is an unsigned producer attestation; independent signed verdict pending. No cloud or timing run. Source grids, coefficients, tails, inversions, copies and final verification remain charged. The policy stays optional.

*Sources and next.* Exact interval/progression/borrowed predecessor and FLINT polynomial allocation/evaluation conventions; KB unavailable. Next count primitive field work and temporary allocations, since batches, convolutions and query trees have different costs. DERIVATION.md and RESULTS.md provide details.
