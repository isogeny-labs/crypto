# Disjoint cohort enrollment contract

This module implements the run-free preparation design recorded in
`../protocol_draft/full_cohort_continuation_20261030/DISJOINT_ENROLLMENT.md`.
It does not freeze a protocol, admit prior observations, authorize execution,
or qualify timing.

The request binds the pinned `ecbench` binary and build information, the seven
original cohort specifications and plans, the existing p739 gate specification
and plan, and explicit protocol-lineage artifacts by path and SHA-256. The
request is the only source of file locations. The output directory must not
exist.

The request schema is `pencil-disjoint-enrollment-request/v1`. It contains:

- `instrument`: `binary` and `build_info` file bindings plus
  `source_revision`;
- `original_specs` and `original_plans`: seven file bindings each, logically
  named `cohort-{16,20,24,28,32,36,40}.json`;
- `gate_spec` and `gate_plan`: file bindings logically named
  `gate-p739.json`;
- `remainder`: the fixed source/output names and declared remainder label; and
- `protocol_lineage`: one or more caller-supplied file bindings. Each file
  binding has exactly `name`, `path`, and lowercase `sha256` fields. Relative
  paths resolve from the request file's directory.

The approved instrument, build information, original seven specifications and
plans, and p739 gate hashes are compiled into this narrow route. A request may
select their locations but cannot substitute a different baseline by changing
its own hash declarations.

The derived cohort-16 remainder changes exactly two parts of the original
cohort-16 specification: it removes the one curve object that is byte-for-byte
equal as JSON to the gate's sole p739 curve, and it replaces the declared label
with the request's remainder label. Every other JSON value and key is retained.
The gate must itself equal cohort 16 after retaining only that exact curve and
changing only its label.

The plan-only output must use schema `ecbench.plan/v1`. The validator compares
the original seven plans with the enrolled set consisting of the p739 gate,
the derived remainder, and the unchanged other six plans. It requires exactly
1,260 unique workloads and 22,680 unique slots: 18,900 measured and 3,780
warmup executions. Each slot is keyed by workload ID, warmup flag, round, and
arm name. Complete workload objects, complete resolved arm and method objects,
exact unsigned 64-bit algorithm seeds, and the arm order within every workload
round must agree. Only plan specification identity, workload array index, and
execution sequence may differ. Duplicate, missing, overlapping, malformed, or
unknown-schema content is rejected.

The command emits a normal native-preparation receipt, a separate
`pencil-disjoint-enrollment/v1` certificate, and hash-bound workload and slot
mappings that retain both original and enrolled indices and sequence numbers.
The p186379 target-0 gate remains a separately charged prerequisite and is not
part of this partition. No solver, cohort, cost, timing, memory, or cloud work
is performed by this route.
