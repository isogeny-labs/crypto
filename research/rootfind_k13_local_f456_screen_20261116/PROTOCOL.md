# Fresh-target K13 F4/F5/F6 local screen

Status: frozen before data. This is a separate engineering diagnostic. The ten-arm EKS L2 protocol in `../rootfind_f456_batch_combined_20261115/` retains its original seed, sample count, host, controls, and stopping rule.

## Question and test boundary

On registered Koblitz E0 over F_(2^13), subgroup order 2003, which current complete-cold phases dominate the rootfinding and Groebner IC collectors on the **same** fresh public targets? This screen checks whether the batched m=3 collector warrants a code optimization or whether F4/F5/F6 remains the faster current-source route. It cannot establish a scaling exponent, population performance, or an L2 speed claim.

## Frozen specification

`SPEC-K13-SCREEN.json` SHA-256 is `5c6b17ddf935d3e9ba153e0617a8b40522e6bbd79fc346932a830ddcb91e94ee`; canonical ecbench spec ID is `ECS1h326f1843d687`. The source parent is `88112c5a930009c883059c1c2e3c6c92c4b23f18`; bind the actual run commit, binary hash, host, isolation and dirty state in the session. Public target seed `202611160511` and algorithm seed `202611160512` are distinct from the frozen EKS and previous local screens.

There are 2 target workloads × 8 arms × (1 warmup + 2 measured rounds) = 48 executions, alternating order, local L0, each with a 120-second child timeout. Run one epoch only; do not automatically retry or increase a bound. All IC arms use `binary-standard-subspace:dimension=5`, `max_trials=200000`, and the exact oracle strings in the spec. The arms are strong rho, matrix-F4, matrix-F5, inherited-F4, F6-IC, scalar rootfinding m=3, batched rootfinding m=3, and an independent batched A/A duplicate. F4/F5/F6 use degree 3 and node budget 8192. No changed parameter may be pooled with these data.

## Measures and controls

Primary descriptive measure: per-execution complete-cold `time.process_wall_ns`, including setup, collection, rank/LA, target descent, and answer verification. Compute batched/F4, batched/F5, batched/inherited-F4, batched/F6, batched/scalar and batched/A/A ratios **only** on exact target-round pairs where both records have verified answers and complete process walls. Report pair count and omitted pairs, and retain every failure and timeout. Report arm wall medians only for valid measured executions, with denominators. Secondary measures: phase wall medians, work-counter completeness and bitop caveats, method-call wall, peak RSS, relation trials, verified relations, independent rank if present. Do not divide by zero rank or infer a missing phase cost.

`ecbench verify --replay-all` checks each deterministic measured answer and `[d]G=Q`; preserve replay problems and failed records. The A/A ratio indicates local noise. A local ratio does not qualify a quantitative speed claim under the progress-fidelity rule; a claim would need matched isolated baseline/candidate evidence and independent validation. Preserve the frozen EKS same-host L2 panel as the confirmatory decision.

## Outcome actions

- If batched m=3 spends most of its complete wall in pair-table construction or lookup and a Groebner arm is faster on valid pairs, prototype a separate abscissa-indexed table implementation and hold out a new seed for its evaluation. Include setup and memory.
- If rootfinding is faster on valid pairs, inspect the phase and rank counters, then retain the frozen EKS panel for isolated confirmation and independent replay.
- If any arm times out or records are incomplete, report that scope as censored and repair the specific producer problem before another protocol. A bounded timeout is not mathematical evidence about the engine.
- If the A/A ratio is comparable to an observed candidate difference, treat the difference as unresolved and use an isolated host under a separate frozen run.

## Execution and custody

Before launch: commit and push this spec/protocol/analyzer, build release ecbench from the exact committed source, validate `ecbench plan`, create an open `bench.record/v1`, and push a complete S3 resume checkpoint. Use `--cpus none --allow-busy --wait` with the shared lock. Store raw session/records/work/replay, stdout/stderr, binary/source/environment hashes and failed rows. Close the benchmark record as measured, censored, or invalid according to the evidence; upload raw bytes and produce a source-linked report, SVG, and inspected PDF. Do not enter this L0 screen in the IC leaderboard.

## Sources and prior scope

The binary section of `papers/rootfinding-decomposition/rootfinding_decomposition.tex` compares root finding and matrix-F4 under older factor-base/degree settings and shows why setup can dominate at small n. `research/f4_f5_f6_cross_family_20261011/ANALYSIS.json` is historical source/host evidence. `research/rootfind_dense_binary_pilot_20261116/PROTOCOL.md` is the immediately preceding five-arm L0 design on different targets. The current ten-arm spec is the parameter source for this screen. None of these is pooled into this new measurement.
