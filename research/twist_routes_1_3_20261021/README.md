# Twist routes 1–3: designs and bounded execution

[Current follow-up results and remaining requirements](reports/followup-20261023/RESULTS.md),
[PDF](reports/followup-20261023/RESULTS.pdf), and
[execution/raw publication status](execution-v2/README.md).
[Original execution report](reports/EXECUTION_RESULTS.md).
[Original experiment designs](EXPERIMENTS.md), [machine-readable status](experiments.json),
[dependency diagram](PROGRAM.svg), [Lean proof map](lean/README.md).

[Follow-up proposals E18–E29](proposals-20261022/README.md),
[proposal PDF](proposals-20261022/REPORT.pdf) and
[machine-readable designs](proposals-20261022/experiments.json) add twelve
blocked, unfunded experiments. They cover actual 3-gluing, coupled restrictions,
partial-relation elimination, independent divisor replay and solver/cost gaps.
That directory is the frozen design snapshot. Bounded local follow-up phases
have now run under separately committed protocols; the full twelve-experiment
campaign remains partial. A separate E30 incidence pilot is also recorded.
[Proposal validation and check logs](proposals-20261022/CHECKS.md) retain the
sandbox-blocked test attempt and its passing socket-enabled replay.

The 2026-10-21–22 native panels recorded 1,920 combined IC/rho executions:
1,704 verified logarithms, all identically replayed, and 216 empty-base cells.
Trace/cover/class-map controls, three bounded solver attempts, rational-height
searches and known positive pairing/isogeny controls are preserved. All three
eligible isogenies now have checked explicit duals. No calibrated IC speedup,
novel algorithm or NIST-size benefit is established.

Actual 3-torsion and explicit BHLS degree-three cover maps now pass native
and PARI checks on the three positive toy sources and public P-521. The toy
divisor transfers verify 24 distinct destination-subgroup recoveries back on
their sources; no NIST DLP is attempted. Polarized identification with the
certified torsion graphs and full costs remain open.

Five actual higher-prime toy torsion cases also replay. Coupled and helper
experiments retain identical projected rows to their matched rational
controls. E30's 270 finite-incidence cells produce no maximum-degree
reduction in the fixed grid. The 17-theorem Lean follow-up is a partial
concrete bridge. None of these results establishes a calibrated speedup.

The eighteen-experiment campaign remains partial: the result table retains
polarized gluing identification, higher-degree factorization, broader solver
grids, full accounting/calibration, formal map bridges and independent-host
replay as open requirements. The backlog cards remain blocked and unfunded.
E26–E28 have not run. The full raw bundle is prepared locally but its remote
store publication is blocked by missing AWS credentials. No continuing
background computation is implied by these records.

## Reproduction

Rust 1.90.0 builds the native runner in [runner/](runner/). Each versioned
protocol freezes its exact source, fixture inputs, argv, seeds, resource bounds
and stop rules. Use the pinned revision for historical replay. Fresh output
directories are required. The solver records include invalid and resource-limited
attempts, deterministic split raw archives and extraction/checksum instructions.

The native ecbench session and its full same-host audit are linked from the
result report. Native aggregation and equal 0/0.1/1 GAE sensitivity are in
twist-route-summary; charts read recorded data only.

The native `twist-browser-refresh` projects the frozen pre-study browser plus
the registered models and sealed new session into the current site index. Its
checks retain every historical yield and existing metadata value. Reproduce
its base with `git show ac09baf:docs/browser/data.json`; the source/hash receipts
are in `execution-v1/checks/catalog-browser-refresh-v2.json`. The legacy
`generated_by` label remains for the canonical site schema. Existing Node
catalog checks additionally replay declared-field cover identities; no Python
was used for the experiments, verification or catalog refresh.

```sh
cargo test --release --lib --locked
cargo test --release --locked --manifest-path research/twist_routes_1_3_20261021/runner/Cargo.toml
cargo build --release --locked --bin ecbench
cargo build --release --locked --manifest-path research/twist_routes_1_3_20261021/runner/Cargo.toml
target/release/ecbench verify --dir research/twist_routes_1_3_20261021/execution-v1/runs/ecbench-20261022T0012Z --replay-all --out FRESH_AUDIT.json --exit-code
bash research/twist_routes_1_3_20261021/build-current-results.sh
```

The earlier [E00 report](reports/RESULTS.md) and its frozen receipt retain their
original preflight scope. Current execution status is the linked result report.
