# Active-column matched control for natural EC graph partition

Date: 2026-11-25, America/Los_Angeles. Conductor crypto task `T-60`. The first run and raw files in [`EC_GRAPH_SOLVER_PROTOCOL.md`](EC_GRAPH_SOLVER_PROTOCOL.md) remain unchanged. The current S3 checkpoint gate reports missing `.resume/RESUME.md`; the user explicitly authorized local bypass and this remains a small public local run.

Immutable input: `S4_MATCHED_V2_STDOUT.txt`, SHA-256 `3accec218c07a2acd2fb433270fa8724db6aee57fef85d0e29e3eb3de16c1f7f`. V2 source and separately copied qualifier SHA-256 `1535d941928d4e420f757abe6009f9a434f43973e737a8d40d49540b7f9983d0`. The only algorithmic addition to V1 is a **plain elimination after dropping zero base columns**, with a separately charged scan of all `rows×16` base cells. It uses the same row order, active-column order, exact modular elimination routine, row witnesses, graph partition and full target-column arm. Rank equality across all arms is a mandatory guard. No full target logarithm is pinned because the relation matrices are rank deficient.

Compare active-column plain versus component partition to isolate the graph split from zero-column pruning. Report multiply/add, inversions, pivot scans, row updates, and setup cell scans for both. The graph arm also pays union-find/grouping overhead not represented by the scan count. A component benefit is admitted only if its elimination savings exceed extra setup in compatible work units; mixed operation categories cannot be added as a wall-time speedup. Frozen prediction: the one-component z=320 case should show no structural split gain beyond row/column ordering; exact operation counts are descriptive. `[pcc:unverified]`

Run once with `timeout 60 gp -q < research/prime_field_100_ideas_20261123/EC_GRAPH_SOLVER_V2_QUALIFIER.gp`, capture stdout/stderr/exit separately, require empty stderr, zero exit and final sentinel. Preserve failure; no silent retry. No performance/security claim.

## One executed V2 control

The qualifier rechecked all 57 emitted signed EC point witnesses, matched rank across all arms and reached its sentinel. [Raw stdout](EC_GRAPH_SOLVER_V2.stdout.txt) SHA-256 `587fe3385da92ab41d4bcbc89cb8a792ea5d8a1b12f059294f1280998a77ba3e`; [stderr](EC_GRAPH_SOLVER_V2.stderr.txt) is empty SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; [exit](EC_GRAPH_SOLVER_V2.exit.txt) contains `0`, SHA-256 `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa`.

| Target | Active plain elimination | Component partition | Extra graph split effect after pruning | Setup scans, both arms |
|---|---|---|---|---:|
| z=0; 24 rows, 10 active columns, 2 components | 192 multiply/add; 4 inversions; 212 pivot scans; rank 4 | 108 multiply/add; 4 inversions; 106 pivot scans; rank 4 | saves 84 multiply/add and 106 pivot scans | 384 cell scans |
| z=320; 33 rows, 12 active columns, 1 component | 333 multiply/add; 6 inversions; 348 pivot scans; rank 6 | identical 333/6/348; rank 6 | zero | 528 cell scans |

Thus the earlier z=320 plain-versus-partition decrease was entirely zero-column pruning, not graph splitting. The z=0 base-only matrix has a genuine charged-elimination reduction from component grouping. The graph setup performs the same all-cell scan as a simple active-column filter **plus** union-find and grouping work; that additional work is not timed or counted in compatible field-operation units. Consequently V2 does not establish a full-cost speedup. Both full matrices including target have the same ranks as their base-only matrices (4 and 6), but neither covers 16 factor-base logarithms. The common target column can reconnect components, so this base-only improvement is not a full target-solve result. `[pcc:unverified]`
