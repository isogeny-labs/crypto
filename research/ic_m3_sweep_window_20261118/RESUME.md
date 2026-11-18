# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv

## Goal
Lower the counted cost S of the m = 3 rootfind-subtract index-calculus oracle with a rotating sweep
window (Lead 1 of research/ic_m3_cost_leads_20261115/LEADS.md). Draft PR #4739.

## Lead / hypothesis
A decomposition T = P_i + P_j + P_k has three chances to be found, so sweeping only a rotating
fraction f = 1/N of the base per trial and then taking the next target should find relations
per counted addition faster. Predicted S ratio at a = 4096, f = 1/8: 0.43-0.57 of the full sweep.

## Spec
research/ic_m3_sweep_window_20261118/specs/sweep.json (PROTOCOL.md frozen in c7535aafb1). Curves,
targets and target seed of research/ic_gpu_rootfind_la_20261115/specs/rf3la.json; arms sweep in
{1, 1/2, 1/4, 1/8, 1/16} at a in {2048, 4096}, echelon LA, rho.negation + A/A; 2 rounds; L0.

## State
Code (sweep=1/N in RootfindSubtractOracle, plugins.rs) and test rootfind_subtract_sweep_window
committed and passing (16/16 rootfind lib tests). Next: benchrec init, then
`benchrec run -- target/release/ecbench run --spec .../specs/sweep.json --out <scratch>/sweep/session --cpus none --allow-busy`.

## Next steps
1. Run the session under benchrec; `ecbench verify --replay-all`; `ecbench band` (default and --no-price-table-work).
2. analyze.py: replication of the f = 1 arm against round 5, S ratios, rows, row-yield check, band.
3. REPORT.md, figure, PDF; validator review; tick docs/ACTIVE.md; mark PR #4739 ready.

## Open blockers
benchrec close is blocked repo-wide by main's malformed benchmarks/records/20261117T0334Z-hec-multi-production-cold.json
(exclusion needs the user). No S3 upload transport in this container for checkpoints.
