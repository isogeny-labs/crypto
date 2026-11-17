# Native symmetric-oracle transfer: design, not a runnable protocol

Status: proposed implementation and controlled comparison. No execution has
been commissioned, frozen, dispatched or reported by this file. It does not
activate an existing draft card or claim that a native adapter exists.

## Question 1 — hold the base fixed, vary only the oracle

Implement a general-F₂-subspace symmetric m=2 adapter. Compare it with the
unchanged direct/F4 oracle and the exact subtraction/pair reference on the
same signed inventory. Preserve source/build/binary hashes and identical
field, exact curve representation, subgroup, targets and stopping limits.

The matrix's consistency and nullity are intermediate observations. The
endpoint is verified successful decomposition with complete work charged,
not just rejection work, linear-algebra rank or number of algebraic roots.

Correctness admission before measurement must cover:

- Every enumerated affine solution satisfies the original linear system.
- Original coordinates satisfy the standard S3 equation; field values alone
  do not bypass original-inventory or group checks.
- A splitting root must lie in V and the original point inventory; keeping
  only roots of an enlarged hull changes the factor base and is forbidden.
- Both signs, repeated points/roots, zero abscissae, identity targets and
  inconsistent systems have explicit checked semantics.
- Independent native point-pair enumeration on the admitted bounded fixture
  agrees with completed verdicts. A budget stop remains unresolved.
- Positive controls are annihilated by the recorded prime subgroup order.
  A whole-curve control does not establish subgroup-qualified operation.

## Question 2 — hold the oracle fixed, vary only base selection

Only after Question1 admission, compare high-excess, median-excess and random
full-rank subspaces. Match actual signed/projected columns, not just intended
dimension or x-count. If a size-matched block does not exist, report it.
Keep product-space structure, subgroup support and target-conditioned nullity
as separate data. Never assume that matching cardinality makes coverage or
rank identical.

Use fresh public target workloads shared across arms, never derived from
knowing point logs in the factor base. Constructed positives are a separate
correctness arm. Training selects the base; disjoint holdout cannot select
it or increase training rank. Freeze exact IDs, seeds, target stream and
decision rules before collecting comparative data.

## Work ledger — no missing category is free

| Phase | Required charges |
| --- | --- |
| Setup and selection | Candidate construction, failed candidates, product-space computation, inventory, matrix templates, lookup/pair tables |
| Target preparation | Public target queries, canonicalization, encoding and actual matrix assembly |
| Linear solve or direct/F4 | Every row/word operation and field operation, including inconsistent attempts and fallback |
| Symmetric search | Particular/kernel representation, every affine candidate visited, including unsplittable candidates |
| Splitting and lifting | Field roots/Artin–Schreier work, original-membership tests, signs, repeated-root cases, group/subgroup checks |
| Useful relations | Verification, deduplication, dependent rows, projected-column operations, incremental rank |
| Records and recovery | Explicitly distinguish instrumentation/logging from scientific work, include it where the endpoint requires it, preserve interrupted epochs |

Export native classes symmetrically and price the complete 0/0.1/1 band.
Do not add several counters for one operation or compare a fully charged arm
with an unpriced incumbent. Separate cold setup from online work and name
exactly how many targets reuse setup.

## Controls, stopping and verdicts

- Positive control: verified original-inventory subgroup decomposition.
- Negative control: matched random-subspace population, with the same oracle,
  field, inventory-size policy and budgets.
- A/A: independent nuisance draws with the same method, not a second label
  on the exact same deterministic run.
- Fixed finite resource envelope; cap affine nullity/candidates, matrix cells,
  time and memory. Do not increase a cap after seeing a failed cell.
- Interleave arms; freeze the analysis/selection rules and mask arm labels
  when drawing comparative conclusions. Declare effect size and sample-size
  rationale before measurement.
- Count all timeouts, low-rank arms and failures. A failed positive control
  invalidates the measurement; a cap hit is inconclusive, not a refutation.

An improvement requires verified answers and lower complete stage cost on
matched fresh targets, with useful independent relations and the full pricing
band disclosed. A degree decrease, extra coverage or faster rejection alone
is not an improvement. Zero independent rank has no finite cost-per-rank
estimate. Smaller verified improvements still count; no rho win is required
for a stage result.

Before any wall-speed claim, use the fastest available matched external
implementation on the exact same exported system, host and resource envelope.
The external reference obligation does not disappear because a different
oracle avoids Gröbner reduction.

## Launch prerequisites — currently unfulfilled

1. Implement and validate the adapter on the final source, including affected
   native tests and required release-library checks before its source push.
2. Freeze a new executable protocol/export contract with enforceable budgets,
   full accounting, controls, known exact model identities and original stops.
3. Establish a supported execution/image route under current isolab-eks and
   an actually ready identity. A default ecbench fixture is not this workload.
   Do not broaden access, launch a paid host or reuse chat credentials.
4. Complete and upload a current resume checkpoint through the canonical gate.
   Prior successful b83 custody checkpoint is not this future experiment's gate.
5. After a real run, preserve every raw record, verify independent replay,
   render its own report and follow actual artifact/PR admission gates.

This design does not claim to answer the separate m=3/4 closure transfer,
certified regularity,83/130-bit operation or an end-to-end ECDLP advantage.
