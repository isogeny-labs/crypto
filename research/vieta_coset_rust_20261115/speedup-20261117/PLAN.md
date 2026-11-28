# Active goal: measured2–6× Vieta improvement

Target: matched full construction-plus-query elapsed cost against the existing
Euclid/Newton implementation, starting word62/m16384. Schoolbook, half-GCD and
lookup remain controls/comparators. Query-only counts/time stay separate.
No improved speed is claimed before a corrected prospective comparison.

First component: CPU profile the unchanged qualified worker on the fixed
conditioned target input003 from the previous published experiment, using
GNU gprofng software timer sampling at1ms. One newton and one half process,
45s cap each, CPU4, no retries, separate raw ownership. These are attribution
profiles, not throughput observations or evidence of speedup. The old study
and all of its frozen source/data/protocol hashes stay unchanged.

The official profile.sh probe failed because gawk is absent; perf/valgrind
are absent too. Native GNU gprofng is installed; its documented clock-sampling
mode is the materially different local fallback. No privileged profiling,
paid/cloud experiment or local S3 operation. Read raw native function tables;
no Python research/statistics. A symbolled release binary uses the exact old
worker/algorithm source, separate CARGO_TARGET_DIR and debug line tables.

Primary sources checked: FLINT v3.3.1 nmod_poly/hgcd.c and gr_poly/hgcd.c cached
primary sources; GNU gprofng collect-app/display-text help; existing
src/cryptanalysis/fp_backend.rs (Fp64/Fp64Arithmetic and their bounds/costs),
ecc2k130_guard.rs Montgomery construction. Previous direct observations at
immutable9c66263eb6 are historical input, not a new measured comparison.
KB MCP unavailable; status and exact/negative search wrapper queries all127.
Conductor MCP/CLI unavailable; owned worktree/branch isolates this scope.

Before any comparative experiment, native relation tests and a tiny actual
canonical seed/export fixture must pass, then freeze/publish source/binary,
workloads, controls, budgets, analysis and protocol. Use schema harness_drawn
with the exact deterministic rule, avoiding the previous explicit_list error.
Keep the goal active through failed candidates and publication blockers.
Full degree-bound general NF instantiation and signed standing stay unverified.
