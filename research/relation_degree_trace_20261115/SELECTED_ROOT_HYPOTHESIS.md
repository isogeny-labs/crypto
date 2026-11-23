# Lazy selected-root construction

Preregistration before implementation and new correctness calls. The continuing
target is a 15x runtime improvement under COMPACT_SPEED_PROTOCOL.md, not a
conclusion from this development step [pcc:unverified]. That frozen protocol,
its controls, budgets, holdouts and acceptance thresholds remain in force.

## Sources and mechanism

KB unavailable: no KB MCP tools are loaded; `scripts/research-kb status`
reports no executable. Fallback review: projection.rs `augment_impl`,
quotient/compact.rs `solve`, cheap.rs pair recognition, and the compact
hypothesis/report. Reopened Isabelle2025-2's history/resolve_proof source:
https://isabelle.in.tum.de/website-Isabelle2025-2/dist/library/HOL/HOL/ISABELLE_HOME/src/HOL/Tools/groebner.ML.html
The exact identity representation remains the established acyclic construction.
This change schedules existing resultant/GCD kernels; no new algebraic result,
novelty, solving-degree reduction or degree-of-regularity result is claimed.

The current constructor computes every vertex projection and GCD, then selects
the highest-index vertex of degree 2..8. Search vertices in descending order
instead. For each candidate, compute its incident edge resultants in the same
recognized-pair order, orient and check each used identity, and fold its GCD in
the original order. Cache each canonical resultant by pair index, including
zero results, so fallback candidates never recompute an edge. Check the final
root identity before use. Reuse the recognized pair list for coordinates.

Retain the old all-vertices path as an explicit same-binary ablation. The
expanded constructor and F4 control keep their arithmetic and caps. Preserve
all compact certificate, containment, reduced-basis, S-pair and native boundary
checks, including final verification. Admission and failure conditions stay
unchanged. Rejection of disconnected graphs remains required.

## Falsifiable development checks

Use only existing seed-7 fixtures at n=3/4/6 over primes 2147483647 and
9007199254740881. Predict identical selected root polynomials AND original
cofactor rows, identical full compact certificates and complete reduced bases
between old and lazy paths. Compare the expanded basis too. Track the number
of canonical edge resultant constructions and vertex GCD folds; these are
structural diagnostics, not elapsed time or total field operations.

On the fixed six-cycle, predict two edge resultants and one vertex GCD fold,
versus six edge resultants and six vertex GCD folds in the old path. Each fold
may perform several Euclidean reductions; do not label folds as field work.
Check descending fallback by embedding an existing three-cycle in four
variables with the highest variable unused, and by adding a disconnected
higher-index recognized edge with too few projections. The projection helper
must choose the same eligible root as the old path; full solving must continue
to reject a disconnected/unused variable. Additional degenerate recognized
pairs may be constructed from existing small-field fixtures solely to test
zero-result caching and unsupported-degree fallback.

No fresh holdouts, benchmark, profiler or timing ratio is authorized by these
unit checks. Separately copied immutable release qualifiers and distinct build,
pre-call and post-call records are required. A runtime conclusion still needs
the S3 checkpoint and prescribed execution gates, with complete matched costs.
