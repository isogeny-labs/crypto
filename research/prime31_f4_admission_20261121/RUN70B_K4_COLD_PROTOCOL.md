# Prime31 Run70B — cold K=4 public-certificate reference cost (preregistration)

Date 2026-11-22 UTC. Scope: **GENERIC negative control only**, not a non-generic index calculus algorithm. Source must be checked into draft PR #4901 before native measurements.

Run70 exact-source original Rust passed twice at K=1/4/16: the first four publicly verified signed triple certificates per target made all 32 targets rank-visible after 2,528,933 actual RHS-bearing kG shifts, versus 5,378,231 for K=1, with no further improvement at K=16. However run70 charged **16** certificates and the *entire K=1 walk*, so its 2.12–2.38 seconds do not measure a K=4 solver.

## Frozen direct cost experiment

Clone the previously checked original-Rust \`prime31_run70_multi_witness.rs\` into an isolated new example, preserving \`include!("prime31_run69_genuine_partial_rank.rs")\`, the original crate's \`Arithmetic, Base, PRIME31\`, B677, 88 verified free rows, signed DSU and exact quotient RREF. Only change the certificate finder to stop at exactly 4 *distinct* sorted signed triples per public target; do not first enumerate 16. Use 32 public unknown-index Q from original frozen seed 0x68B677A1D1; hidden scalars are post-solve checks only.

Build the full explicitly **GENERIC** 916,469-key signed-pair table, collect all *actual* kG shifts and authentic signed-pair RHS rows, recompute quotient target visibility every 32 hits, stop at the first point all 32 Q's have one rank-visible certificate. Exact replay gate: 2,528,933 actual shifts, 1,088 signed-pair hits, 546 independent pair rows, all 32 target scalars verified by original Rust [x]G=Q and by separate same-process original-Rust BSGS. Any mismatch is failure, not a rewritten result. Full charge includes base/table setup, every public target certificate leaf (positive or negative), row-space checks, target verification, harness, and reference BSGS; measure wall times and source hashes on two native CI executions, 41 original library tests.

A favorable K4 vs K1 query count is already established; the new question is whether **K4-only total time** is below the matched (not optimized) BSGS reference and whether target-certificate enumeration overhead erases the rank savings. It is not permissible to call this a non-generic computational victory even if generic K4 wins. No index-calculus-frontier/optimized rho comparison is available yet; these remain required for any qualifying solver result.

Previous work and owners: run69 authentic RHS and run70 multi-readout (PR #4901), #4692 registered RR9/F4 blind gate, #4822 RR9/Krylov, #4812 Horner/F3, #4809 S3 roots, #4816 small-field controls; Amadori–Pintore–Sala DOI 10.1016/j.ffa.2018.01.009 predates the prime-field one-relation strategy. Research-KB MCP/local Rust unavailable; exact-source GitHub review and source-specific CI are the fallbacks.

Status: **preregistered only**; no K4-only wall-clock measurement yet.


## Source-specific measured close (2026-11-22 UTC)

The originally frozen experiment **PASSed twice**. Exact original-Rust source blob \`94a83b21ef32593bc76a67ef15bf831b010d5a42\`, commit \`083dc8caf2b673fdbe4a4c59212990ca41143280\`: [Actions #45066979236](https://github.com/isogeny-labs/crypto/actions/runs/45066979236) job 137345217885 and [Actions #45066973351](https://github.com/isogeny-labs/crypto/actions/runs/45066973351) job 137345200888 both **SUCCESS** with 41/41 original Rust library tests, 32/32 complete target logs each and exact frozen K4 relation counts. The *actual cold K4 only* end-to-end cost was 1086.016 / 1708.381 ms versus same-process BSGS 150.769 / 235.548 ms, respectively, **7.20×/7.25× slower**. New precise [report](RUN70B_K4_COLD_RESULT.md) and [frozen results](RUN70B_RESULTS.json). Existing one-cohort and generic-oracle qualifications remain.
