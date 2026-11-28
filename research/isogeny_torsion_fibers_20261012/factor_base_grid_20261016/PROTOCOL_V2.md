# Factor-base grid accounting rerun

Frozen 2026-10-16 before RUN_V2. [PROTOCOL_V1.md](PROTOCOL_V1.md)
continues to define the exact curves, 8/16/32/64 columns, public inputs,
four targets, three rounds, eight arms, trial streams, 300-second child cap,
512 MiB cap where supported, exhaustive pair gate, and matched controls.
RUN_V1 is retained unchanged. Inspection of its frozen runner during the
first panel found a ledger defect: relation search, row insertion and scalar
recovery shared one `query_cpu_ns`, and the χ fallback scan counter was not
serialized on solve children. This is an accounting defect, not a changed
oracle or a failed mathematical check. RUN_V1's group counts and answer
replays remain independently auditable, but its phase table is incomplete.

RUN_V2 repeats the complete 384-child panel from the same frozen inputs,
base-prefix rule, seed stream, cap and arm order, with no algorithm changes.
The modified runner records disjoint `relation_cpu_ns`, `row_cpu_ns`,
`recovery_cpu_ns`, `setup_cpu_ns`, `verify_cpu_ns` and `other_cpu_ns`.
Here relation CPU is query CPU minus separately timed row insertion and
recovery. The checker requires
`relation + row + recovery = query` and
`setup + query + verify + other = total` for every completed child.
The χ `exhaustive_fallback_evaluations` counter is serialized even when
zero. Existing setup/query/verify field and group counters remain; row
and recovery code uses integer arithmetic only. Their modular work is
still not converted into a portable group-operation price.

The run command is `run_panel_v2.sh RUN_V2 RUNNER CHECKER`, using release
Rust binaries built from the committed runner and checker. The launcher
snapshots both sources and their binary/input SHA-256 hashes before freeze,
then preserves per-child exits, stdout, stderr and replay. Any nonzero
child or checker exit remains evidence and blocks a complete comparison.
The V2 summary is a descriptive counted-group and shared-host CPU table.
It may be compared with V1 only if all deterministic scalars, relation
witnesses, ranks, trials, and counted group operations match; timing may
differ. Run `RUNNER audit-grid-v2 RUN_V1 RUN_V2` and require its versioned
audit to match both plans, every exhaustive pair set, all 384 scalar and
relation records, group and field counters, and every V2 CPU phase sum.
Portable `S`
remains unset until square-root internals, polynomial/table operations,
allocations, startup, I/O and independent-host calibration are priced.
An isolated wall claim still requires L2 admission under V1's rule.
