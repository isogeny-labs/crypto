# Paired diagnostic export

Standalone Rust projection of one `ecbench.record/v1` JSONL session. This is
analysis tooling, not an experiment launcher or a validation certificate.

```
cargo run --release --manifest-path research/pencil_large_prime_20261029/paired_export/Cargo.toml -- records.jsonl candidate rho rho_AA rho pairs.json pairs.csv
```

Candidate and reference labels must differ, and the two A/A labels must differ.
The reference can also serve as one A/A side: the example above needs only three
executions per target/round, reusing the same rho record for both comparisons.
Outputs must not already exist. Measured rows
are paired by workload identity, absolute round and exact decimal algorithm
seed; a seed mismatch within a workload/round, duplicate arm or mixed session
is an error. Warmups and other arms are counted and ignored. Missing partners,
failures and exhausted/capped executions remain visible, with null ratios.
Without `--doc PLAN.json`, a target wholly absent from the input cannot be
discovered. With a plan, expected measured workload/round pairs are emitted even
when all their records are absent; missing execution sequence numbers also
include absent warmups. No new target is sampled.

JSON schema is `pencil-paired-diagnostics/v1`. CSV is one row per comparison,
with complex observations encoded as quoted JSON cells and literal `null` for
unknown values. Exact integer timers, seeds and phase maps remain in JSON;
floating diagnostic ratios are derived observations. The SHA-256 binds the
exact input bytes. Rows retain record IDs, method configurations, outcomes,
cap/failure details, phase maps and incomplete cost observations.

Cold diagnostics divide `time.method_call_wall_ns` only when both rows report
verified outcomes, independent target checks and L2/L3 isolation. No fallback
to the narrower `solve_wall_ns` is permitted. Parent audit clocks remain
separate unqualified observations; max RSS is an independent process-memory
observation. Neither is added to child timers. Missing memory stays null.

Online diagnostics require checked integer phase sums equal to the online
window and an explicitly exported `time.online_clock_diagnostics` object with
`error:null`. Absence is unknown and yields null online ratios and
`online_error_not_exported`, while retaining phase data. Producer errors also
block the ratio. A clean error observation alone does not validate a run.

All qualified ratios remain null pending independent review bound to protocol,
source, binary and records. Optional plan mode computes the explicit empirical
A/A rule below; a passing calculation alone does not certify the evidence. It also does not validate record seals, independent
replay, workload equivalence beyond recorded IDs, L2 grading, protocol adherence
or the clock boundary's isolation coverage. Those require a separate reviewer.
There are no aggregate rankings, statistical conclusions or full-price GAE
ratios; lower-bound cost blocks remain labeled observations.

Sources inspected: `src/cryptanalysis/ecbench/{record,methods,runner}.rs`,
`docs/ecbench/README.md`, and `../COST_MODEL_DRAFT.json`. Tests use explicit
field projections with the actual record names, not fabricated sealed records.

## Optional immutable plan and empirical calibration

Append `--doc PLAN.json` to the command. The file must be an `ecbench.plan/v1`
with `spec_id`, `arms`, `workloads` and `executions` as emitted by ecbench.
Each supplied record, including warmups and other arms, must match its plan
sequence number, full workload and resolved method objects, role, arm, round,
warmup flag and algorithm seed. Duplicate sequence numbers and plan identities
are rejected. The export records the exact plan SHA-256 and its declared spec
identity; reconstructing the spec hash and validating the plan's authenticity
remain independent-review obligations.

`target_summaries` retains all expected measured rounds and raw per-round ratios
(null for ineligible observations), plus observed median/minimum/maximum and
counts. Per-arm raw times and their summaries are separate from ratio summaries.
No confidence interval or ranking is inferred from these summaries.

The `operational-tolerance/v1` rule is computed separately for cold and online:
exactly five expected measured A/A pairs must all be eligible, use identical
resolved methods, and have positive clock values. Each **AA_FIRST / AA_SECOND**
ratio must lie in the inclusive interval **[0.90, 1.10]**, checked using integer
cross-products. Thus `candidate rho rho_AA rho` calibrates rho_AA/rho. This is an
empirical operational tolerance, not a confidence interval or superiority test.
Per-target `calibration_passed` and reasons are also attached to pair rows and
CSV. Qualified ratios stay null even after calibration passes. The protocol
must freeze this rule before affected data; this tool does not freeze a protocol.

Backward-compatible mode without `--doc` remains diagnostic-only and does not
compute calibration because the expected round population is unknown. Tests
exercise absent entire workloads, empty records with a plan, absolute measured
rounds after warmup, identity mismatches, exact tolerance endpoints, and
independent cold/online failures using synthetic field projections.
