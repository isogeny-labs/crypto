#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= Shared interval setup for doubled rescue
T-1880 · Prime-field coefficient construction · 2026-11-07

The coefficient stream keeps its stride-specific identity. The interval-product engine can be shared because its polynomial and progression cache depend only on field, retained width and request cap.

#image("flow.svg", width: 100%)

*Checks.* Twenty larger-stride panels, 760 coefficient equalities, 120 rejected-key/borrow controls and 20 interleaved stream pairs pass. Shared ownership cleanup and bounded evaluation batches are checked.

#image("setups.svg", width: 100%)

*Width control.* Shared and separate equal-width engines both use 10,147 evaluation batches. Separate engines at their native widths use 9,953: retaining the smaller original width adds 194 batches. These are exact counts on the supplied panels, not timing measurements.

*Public recovery.* All 1,686 target/cap comparisons retain the fresh-engine rescue result, route and calls. Calls at caps 8,10,16,20,32,64 are 4,053; 3,278; 3,228; 3,224; 3,068; 2,881. Cap 8 leaves ten unresolved targets on each path; all other panels recover 281/281. Shared-engine counts are 0,260,260,260,205,0. All public shares are cold: m=5 and 2m=10 stay below the blocked-product threshold. Warm setup reuse is exercised by the larger-stride controls.

*Costs.* Rc adds a heap allocation for the original engine and reference-count operations; no allocation or peak-memory reduction is claimed. Smaller widths can add batches. All stream, source, quotient and verification work remains charged. This stays an optional adapter.

*Evidence.* validation01 passes first attempt; 98 native source hashes; inherited checks pass. PCC-20261107-2184b1 is an unsigned producer attestation; independent signed verdict pending. Original cohort unchanged; no cloud or timed workload. See DERIVATION.md and RESULTS.md.

*Sources and next.* Exact interval/context/doubled predecessor; official Rust Rc and RefCell documentation. KB unavailable. Next remove Rc ownership overhead or choose width before the first stream with an explicit doubling policy and matched controls.
