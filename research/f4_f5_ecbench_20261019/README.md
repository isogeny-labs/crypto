# F4/F5 matrix construction, recursive roots and full ecbench recovery

This continuation measures the original16/22-variable Boolean fixtures and
complete cold one-target DLP recovery. Baselinea78cc8906 includes the previous
three qualified GF2 switches. Final source9bed104b0a adds cached direct parity
packing, active layouts/top-degree/linear-tail policies, a bounded quadratic
Gray tail after an algebraic root reduction, and specialized column lookup.
All policies stay explicit and opt-in. Owner Windows checkpoint fixde185b4ded and later lockfile, Cairn and cover
assessment fixes throughf236a8c2aa are preserved by merges; the timing source contract stays9bed104b0a.

Read [the results report](REPORT.md) ([PDF](REPORT.pdf)),
[the native summary](native-summary.json), [the ecbench summary](ecbench-summary.json)
and [raw evidence/replay](RAW_EVIDENCE.md). Per-run reports live in `reports/`;
rendered copies are in `benchmarks/20261019-f4-f5-recursive-ecbench/`.
Native analysis and statistics are Rust; Python only formats recorded data.

## Native complete roots and decomposition

All32 final matched changes pass the native improvement gate; all72 method/
input cells have five admitted samples. The four matrix and two full-table
four-worker controls are neutral. All40 rejected attempts are retained.

For n23 planted complete roots, paired F4 falls365.4568 →32.4994ms
(baseline/candidate11.2209, CI95[11.1618,11.5292]); F5 falls371.0485 →
32.2656ms (11.5008, CI95[10.8881,11.7209]). First-witness decomposition
improves3.3003/3.3690 for F4/F5. These are separate tasks and samples.

In the separate final eighteen-arm panel, n23 planted complete-root medians
are F4/F5/inherited/typed-generalized/generic-GB:
32.3955/32.3086/41.0426/1.5138/683.4834ms. First-witness F4/F5/F6/typed/
Semaev-batched/full-table medians are7.9602/7.9927/7.6039/1.5980/46.2176/
134.0259ms, with cold setup charged. These Frobenius-invariant bases differ
from the full-DLP bases below; no cross-panel ratio is formed.

## Complete cold ecbench recovery

Final source9bed104b0a; medians in milliseconds on one dedicated host.
Each method has40 verified measured executions per curve. L2 columns show
which clocks are admitted; all other observations remain in the raw records.

| Method | n17 solve ms | n17 process ms | n17 L2 | n23 solve ms | n23 process ms | n23 L2 |
|---|---:|---:|---:|---:|---:|---:|
| f4-baseline | 91.3372 | 119.0197 | 40/40 | 2592.6712 | 2619.9191 | 40/40 |
| f4-tail | 51.3415 | 79.3053 | 40/40 | 1636.2769 | 1666.0858 | 40/40 |
| f5-baseline | 95.9779 | 124.2733 | 40/40 | 2764.2312 | 2793.9766 | 40/40 |
| f5-tail | 55.9904 | 84.5148 | 39/40 | 1806.6459 | 1836.5007 | 40/40 |
| f6-native | 113.0123 | 141.3677 | 40/40 | 3952.1317 | 3981.7135 | 40/40 |
| generalized-typed | 91.9735 | 118.9833 | 40/40 | 5946.1759 | 5972.4103 | 40/40 |
| semaev-batched | 0.8323 | 27.9561 | 33/40 | 20.5768 | 47.8789 | 39/40 |
| semaev-full-table | 0.8427 | 26.4699 | 34/40 | 19.9445 | 46.3640 | 39/40 |
| rho-strong | 2.6819 | 25.1376 | 40/40 | 2.7805 | 26.6863 | 39/40 |

F4 candidate/baseline paired solve ratios:0.5623 at n17 and0.6309 at n23;
F5:0.5822 and0.6532. The native gates admit all four comparisons outside
A/A noise. The n17 F5 interval uses39 admitted pairs; the other three use40.
All528 executions per curve verify, including88 warmups; all440 measured
replays reproduce. Partial S prices and rho ratios are diagnostics, not a
fully priced cost ranking. Whole-process medians are supplementary clocks.

The original complete-root and first-witness native panel is a separate
workload and is documented in its own report and summary.

## Boundaries and controls

Complete roots, first-witness decomposition, complete DLP cold solve, and
whole ecbench child process are different intervals. Component fixtures use
the original Frobenius-invariant bases. Full DLP specs use standard polynomial
subspaces: dimension5 at n17,6 at n23. Methods within each panel receive
matching inputs; results across panels/hosts are never pooled or multiplied.

Native comparisons have five matched rounds, A/A controls, whole-core
reservation, output audits and retained retries. The eighteen-arm panel
includes F4/F5, inherited F4/F6 paths, scalar/wide/generalized Boolean roots,
generic polynomial-system solving and both Semaev table backends. All72
method/input cells remain, including missing/failed cells. Paired F4/F5
changes also include two fresh planted holdouts and four-worker controls.

Each full ecbench panel has11 arms,8 independent public targets,5 measured
rounds and1 warmup. Every method starts cold. Native harness admission and
full sealed-session replay are required. Both solve and process clocks are
retained; partly priced S and unpriced-counter sensitivity are diagnostic.
A lower-bound S ratio does not establish a total-cost ranking or a rho win.

The initial0acfc55b7f and lookup-only71c4c99d85 runs precede the corrected adaptive schedule; their reports and
summaries stay separate from the final runs. The auxiliary Callgrind batches
measure instructions only. Local smoke/diagnostic wall values are unadmitted.

## Reproduce

Extract and verify `runs.tar.xz` as described in [RAW_EVIDENCE.md](RAW_EVIDENCE.md),
then replay the Rust analysis with `replay-analysis.sh`. `render.py` renders
the recorded summaries. `run-native-adaptive.sh` and `run-ecbench-adaptive.sh` are
the frozen thin shell runners. Their exact source/spec/binary/host manifests
and every attempt are in the archive. `PROTOCOL.md`, `ECBENCH_PROTOCOL.md`,
`PROFILE_PROTOCOL.md` and `LOOKUP_PROTOCOL.md` precede their measurements.

For one current solver invocation:

```sh
RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 \
KIC_GF2_SERIAL_CLEAR=1 KIC_ROOTFIND_BATCH_SETUP=1 \
cargo run --release --locked --example f4_f5_boolean_perf -- \
  --arm solve-f4 --n 23 --seed 230 --random 0 --matrix-runtime adaptive-quadratic-tail
```

For ecbench use the identity-bound parameters in `ecbench-adaptive-n*.json`; leave
`KIC_*` overrides unset, because replay custody intentionally refuses them.

## Remaining measured bottleneck

The final n23 planted F4 call still spends instrumented medians10.8679ms
building matrices and16.8551ms reducing them (separate, nonadditive phase
observations in the native report). On the identical complete-root fixture,
typed generalized roots take1.5138ms in the eighteen-arm panel. In full DLP,
target0/round1 still makes135075 F4 solver calls,135058 of them infeasible,
with only17 quadratic tails. Repeated matrix construction/refutation is the
next target. Reusing coefficient blocks across target queries is a proposed
next experiment; it is not implemented or counted as another gain here.

## Limits

The n19 fixture has36 variables and remains an existing matrix-admission
refusal. These two admitted sizes do not establish scaling, external-fastest,
Arm64/GPU, full-cost/rho or the original hard all-roots/8x qualification.
Magma/msolve/FGb/Singular/PolyBoRi matched external runtime comparisons were
unavailable in the original panel and remain unrun here. Tables A–C,
`docs/index-calculus-scoreboard.html` and the rho progress graph retain their
existing qualified values; this result has a separately labeled component/
native-wall section on the regenerated leaderboard and archived-session
entries in the lab browser. No ranked complete-price curve row is promoted.

[EXECUTION.md](EXECUTION.md) retains execution status, failed validation
commands, source history and CI limitations. Requested comparison completion
is tracked in [VALIDATION.md](VALIDATION.md).

The first all-width prepared schedule improved the original complete-root
calls but regressed full DLP runtime on the smaller standard-subspace systems.
That regression is retained in its own two initial-version reports. The final
`adaptive-prepared` / `adaptive-quadratic-tail` policies try cheap early degrees
below16 original variables and retain the original wider schedule. Legacy
`prepared` / `quadratic-tail` names preserve their old behavior for replay.
Full Semaev-table setup additionally stays on the caller under an explicit
one-worker resource contract; other worker settings keep the parallel path.

The two pre-version L0 screen sessions are replayed with their exact guarded
source through `replay-frozen-screen.sh`; all other sessions use the current
versioned implementations. A screen verifies16 records (8 warmups) and
reproduces8 measured records. Full curve sessions verify528 records, including
88 warmups, and replay440 measured records. Warmups are excluded by the native
replay-all mode. None of these correctness counts is a native-time claim.
