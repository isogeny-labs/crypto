# Bounded native SAT PDP CLI correctness fixture

Prepared 2026-10-30 from the SAT worktree after contract `f71728b66558`.
This file is a prospective software correctness plan, not an executed receipt,
a scientific experiment, a performance comparison, or a generated training
corpus. Verified cloud dataset cells remain zero. No solver, build, or test was
executed while preparing it. Freeze the final source, this plan, the runnable
fixture protocol, and driver before execution. The replay module inspected here
has SHA-256 `19f180408a0e3b1114cb510386741d02d5ea951acc4e9f6b3fcc9a047f9c21db`.

## Prerequisites and ownership

1. The parent reconciles and terminates the existing validation pipeline before
   reusing its target directory. No concurrent compilation or competing native
   solver execution is authorized by this plan.
2. Coordinate the reserved `solvers.rs` change with T887. The prepared
   `solvers-integration-proposed.patch` must be reviewed, applied by its owner,
   and committed before building this fixture. Until then newly generated SAT
   cells do not retain the required GateMap and cannot qualify.
3. Compile the final committed source with its full 40-character
   `CRYPTO_BUILD_GIT_COMMIT` and `CRYPTO_BUILD_GIT_DIRTY=false`. Retain the exact
   binary bytes/hash and build receipt. The generator rejects missing/dirty
   compiled provenance. Do not reuse an older binary with the same filename.
4. Author and freeze a **software-correctness** protocol before generation.
   The CLI requires the `experiment.protocol/v1` container even for this use.
   It must bind this table, resource limits, stopping/acceptance rules, source,
   binary, and non-scientific scope; preserve the separate canonical body hash
   and complete protocol-file SHA-256. `freeze.status` must be `frozen`,
   `filed_before_affected_data` true, and `frozen_at` nonempty. The canonical body
   excludes `freeze`. The plan takes the **complete file hash**, not the body
   hash. Do not relabel a post-run document as a pre-run freeze.
5. Create an unused two-digit attempt namespace, e.g. `sat-cli-01`. Preserve
   all earlier attempt directories and receipts. Local execution uses no
   `--publish`, remote, budget grant, or IAM changes. Archival uses the parent's
   already authorized artifact workflow only after local outputs exist.

## Finite candidate population and fixed order

One CLI plan has `n=[7]`, `m=[2,3]`, families `[koblitz,binary]`, solver `[sat]`,
`curve_seeds=1`, `generators=1`, `keys=2`, seed `20261030`, factor-base size `8`,
degree bound `3`, work limit `10000`, and paired twists. Curve seed and generator
index are both zero. `--curve-seeds 1` is a **count** selecting index zero; it is
not a request for curve seed one. The same distinction applies to generators
and keys. No target, scalar, branching polarity, variable priority, domain,
or identity pattern is chosen after observing solver behavior.

The mixed-radix `DatasetPlan::cell` ordering makes these exact indices:

| Pair order | Family | n | m | Key index | Curve cell | Twist cell |
|---:|---|---:|---:|---:|---:|---:|
| 1 | koblitz | 7 | 2 | 0 | 0 | 1 |
| 2 | koblitz | 7 | 3 | 0 | 2 | 3 |
| 3 | koblitz | 7 | 2 | 1 | 4 | 5 |
| 4 | koblitz | 7 | 3 | 1 | 6 | 7 |
| 5 | binary | 7 | 2 | 0 | 8 | 9 |
| 6 | binary | 7 | 3 | 0 | 10 | 11 |
| 7 | binary | 7 | 2 | 1 | 12 | 13 |
| 8 | binary | 7 | 3 | 1 | 14 | 15 |

Run both members of a pair before assessing that pair. Stop at the first pair
passing all selection gates below. Retain all previous attempts, including
zero-row, unsupported, capped, unknown, and solver-error outcomes. List unrun
indices as `not_run_after_first_qualified_pair`, never as successful or failed.
If no pair qualifies, report the integration gate missing and stop. No seed,
factor-base, arity, or key extension is implicit; a larger search requires a
new prospective bounded plan. This first-qualified selection is only for
finding a regression fixture and cannot estimate success rates or performance.

The source supports this grid: dataset constructors admit n3..61 and m2..15;
the native binary S3 builder admits n2..127 and m2..15 subject to its cap.
Its explicit upper bound is `n*(2*m-1)+4*n*(m-1)+3*n*n*(m-1)`, giving **196**
variables for n7,m2 and **385** for n7,m3, below the adapter's 100000-variable
cap. These are allocation bounds, not solver runtime predictions. Arity two
has one identity pattern; arity three has two. Across all 16 cells, at most
24 native pattern solves can occur. The adapter may stop a cell earlier after
a checked group lift, an unknown result, deadline, or capture cap.

## Exact native command shape

The following are prospective commands, not execution records. `binary`,
`attempt`, `protocol`, and `protocol_file_sha256` must resolve to the frozen
absolute paths/hash retained by the parent driver. The example uses a single
plan and separate output roots because each root binds its shard identity.

```sh
"$binary" dataset plan --out "$attempt/plan.json" \
  --n 7 --m 2,3 --families koblitz,binary --solvers sat \
  --curve-seeds 1 --generators 1 --keys 2 --seed 20261030 \
  --factor-base-size 8 --degree-bound 3 --work-limit 10000 \
  --protocol-sha256 "$protocol_file_sha256"

"$binary" dataset preflight --plan "$attempt/plan.json" --protocol "$protocol"

# Repeat in table order with cell=0,...,15, stopping only at pair boundaries.
"$binary" dataset generate --plan "$attempt/plan.json" --protocol "$protocol" \
  --out "$attempt/candidate-$cell" --max-cases 1 \
  --shard-index "$cell" --shard-count 16 \
  --seconds-per-case 30 --memory-mib 2048 --trace-mib 64

"$binary" dataset verify --out "$attempt/candidate-$cell"
"$binary" dataset status --out "$attempt/candidate-$cell"
"$binary" dataset export --out "$attempt/candidate-$cell" \
  --destination "$attempt/export-$cell"
```

`twists=true` is mandatory in plan/v1 and set by `dataset plan`; there is no
`--twist` switch. The exact plan is 16 cells. Check its parsed fields and every
manifest's `config` against the table, not against an assumed directory order.
The plan file SHA-256 and the native typed-plan binding SHA-256 differ: retain
both, with preflight's binding and each `run.json`.

For the selected curve/twist only, repeat each `dataset export` command against
the **same destination**, comparing its stdout receipt and complete exported
inventory. Also export from the same immutable source into a fresh retry
folder if needed to check source-independent output reconstruction. Choose
one retry mode in the frozen driver; the bounded command count below assumes
same-destination retry. Do not promise byte-identical independent solver reruns:
the adapter has a wall deadline. Determinism here means re-export of identical
retained input bytes.

## Costs, limits, and what is actually measured

| Layer | Bound / accounting rule |
|---|---|
| Candidate selection | At most 16 cells, serial, fixed table; at most 24 pattern solves. |
| Worker | 30-second supervisor deadline per cell, 2048MiB Linux `RLIMIT_AS`, worker Rayon threads fixed to 1 by generator. |
| Native SAT adapter | One 10-second deadline shared across patterns; conflict budget 10000 shared across patterns; group-lift call budget 10000 per cell; one model per pattern. Construction/other phases remain subject to outer supervisor. |
| Native capture | 100000 events and 16MiB native-event stream across a cell's patterns; map at most16MiB; map/formula collection at most64MiB in the proposed adapter. |
| Whole worker output | `--trace-mib 64` covers cell files before manifest seal. It is polled every100ms and checked on exit; overshoot is possible and retained. It is not an exact storage quota. |
| Replay | Shared10million charged literal visits/parity-word operations; 100000 raw events/16MiB; 4096 decisions; 64MiB derived payload; separate terminal censor record4096bytes. |
| Replay memory | Shared64MiB declared logical source/state/pending payload with conservative expansion reservations. Allocator overhead, process RSS, and native-regeneration temporaries are outside this count. |
| Source admission | Formula/map aggregate64MiB, each raw map16MiB, variables at most100000; source role/encoding/hash binding before replay. |
| CLI count | One plan and preflight; at most16 generate,16 verify,16 status,16 first export, and2 selected immutable export retries. Corruption controls below add at most4 export invocations. |
| Driver | One serial driver, wall ceiling1800seconds; command ceilings generate60s, verify30s, status10s, export60s. Preserve partial output and terminate child process groups on driver timeout. No overlapping replacement driver. |
| Driver storage | Prospective admission ceiling3GiB across this attempt's raw cells/exports/logs; measure before/after every command and stop before another command when exceeded. This is not a kernel quota or guarantee against one-command overshoot. Preserve observed final bytes. |

Keep command exit codes, start/end timestamps, process/resource receipts,
stdout/stderr, all phase starts and completed phase telemetry, interruption
records, source/binary hashes, and actual file counts/bytes for **every** attempt.
Do not discard failures when totaling CPU, wall time, bytes or verifier work.
Retain native `word_ops`, conflicts, decisions, propagations, restarts, clause
visits, literal visits, analysis visits, and parity row/pass counters as named
algorithm counters. They are not retired CPU instruction counts.

Retain phase-scoped PMU raw values, time-enabled/time-running, inheritance,
multiplexing and unavailable reasons. `retired_instructions_exact_in_scope`
exists only for usable unmultiplexed readings. Whole-worker retired instructions
remain null in the supervisor record; report phase scope and exclusions rather
than summing phase values into an asserted full solve instruction count. Retain
CPU usage and peak RSS with their native units; RSS is a high-water mark, not an
additive phase metric. All clocks are diagnostics with
`clock_valid_for_comparison=false`. This fixture makes no speed claim.

## Selection and acceptance checks

A pair qualifies only after all these checks succeed:

1. Both source cells pass native `dataset verify`. All manifest-bound artifacts
   pass SHA-256/byte checks, including each `sat-pattern-P.gate-map.json`,
   `sat-pattern-P.xor.cnf`, and `sat-native-events.jsonl`. Gate maps have role
   `native-binary-s3-gate-map` and encoding `json`. Formula role is
   `complete-original-cnf-xor`, encoding `dimacs-xor`. Neither an older cell
   without a map nor a raw solver-only trace satisfies admission.
2. Both members have at least one exported occurrence with
   `record.kind=sat_decision` and `admission_status=verified_decision`; at least
   one verified decision therefore belongs to the twist. A zero-row successful
   command does not qualify. Require the declared strict allowlistv7 and four
   input filenames, including `sat-features.jsonl`; require the corresponding
   `restricted/sat-records.jsonl` artifact and all manifest counts to reconcile.
3. At least one member contains the following **same-pattern** chain, joined
   through restricted source identity and event sequence:
   `verified decision before -> native Learned -> matching Backjump -> verified decision after`.
   Both decision/assignment pairs must have been admitted by the exporter.
   There must be no censored suffix before the latter decision. Check the
   learned clause ID and backjump level against raw events, and require the
   canonical learned clause to be present in the latter context's active CNF.
   Because replay cannot reach the later admitted decision without checking
   that Learned event, this establishes bounded local entailment checking;
   merely finding the words `learned` and `backjump` in a raw log does not.
   Retain an acceptance witness with cell, pattern, four event sequence IDs,
   two feature IDs, learned literals/ID, backjump level, source hashes and
   summary work. This is replay-backed local certification, not an independent
   standalone LRAT proof file or full native execution proof.
4. Every restricted verified decision joins exactly to the expected public
   `feature_id`, split and split component. Recompute each feature ID from the
   canonical public `inputs`; no chosen literal, action priority/activity,
   event ID, outcome, scalar, future work or native cost enters those inputs.
   XOR masks appear as variable-index lists rather than wide JSON bitmasks.
   All unknown records have null feature joins. Reconcile unique public rows
   separately from occurrences; deduplication can reduce public line counts.
5. Each summary matches emitted verified/censored counts and shared budget
   totals. `native_execution_trace_verified`,
   `native_instruction_count_verified`, and `unsat_proof_checked` stay false.
   A native SAT verdict or a PDP relation is not an ECDLP scalar recovery.
   Record relation/group-lift success separately when the ordinary verifier
   confirms it; do not require or invent such a result just to pass this
   causal-context software fixture.
6. The selected pair's immutable re-export receipts/inventories agree. Static,
   prime-prefix and algebraic feature bytes for the same retained inputs remain
   unchanged across retry. Their equivalence to pre-SAT exporter behavior is a
   separate regression check on pinned existing fixtures, not inferred from
   this new SAT pair.
7. Hash the complete original source-cell inventory before and after exports
   and corruption checks; it must be unchanged. Retain all rejected and partial
   output directories and failure receipts, including exporter scratch files
   where available. Fail closed on an unexpected hard replay/binding error;
   do not skip a software defect as merely an unqualified candidate.

## Four bounded corruption/censor controls

Run these only on independent copies of the selected rich cell, never its
original evidence. Keep original and mutated inventories and exact mutation
recipes. Negative copies are software test inputs, not dataset samples.

- Change a GateMap semantic value while updating the copied outer artifact
  hash/size references consistently. Export must reject source substitution
  during regeneration; rejection solely at the outer hash gate is a distinct,
  weaker control and does not satisfy this semantic check.
- Change one retained Assignment's reason/literal to a non-unit or forged XOR
  explanation, maintaining valid JSON and updated copied artifact hashes.
  Export must hard-reject the inconsistent preceding state.
- Truncate immediately after a Decision line and before its Assignment. The
  complete earlier contexts must retain identical feature IDs/bytes; that
  unpaired decision must never acquire a feature. Require a restricted
  truncated unknown-suffix record. Adjust the copied manifest references to
  make this a semantic truncation test rather than only a digest failure.
- Append a well-formed unmatched dynamic clause at a point after an already
  verified context (maintain contiguous sequence, truncate the remaining
  original suffix, and update copied artifact references). Earlier context
  bytes must remain identical; later admission stops with a typed unsupported
  suffix. Do not reinterpret the injected clause as an original axiom.

The central stored-input verifier may reject an altered copy before semantic
export replay. Record that exact gate honestly. If that prevents one intended
semantic CLI control, keep the rejection as custody coverage and rely on the
specific scoped replay unit test for the semantic behavior; mark the semantic
CLI control missing rather than claiming it ran. All four calls are bounded
by the command limits above. No automatic retry with another mutation.

## Retention and terminal receipt

Write a machine-readable driver receipt listing source/binary/protocol/plan
hashes, every attempted index and its status/cost/artifacts, unrun indices,
selected pair or null, transition witness or null, all acceptance/control
results, every command exit status, and exact limitations. A driver trap must
write this receipt and archive partial outputs on failure or timeout. S3
archival retains the whole attempt under a separate SAT validation index, with
fresh object readback and hash verification; it does not change
`verified_cloud_dataset_cells=0`. Never overwrite a failed attempt to make a
later run appear clean. The parent commits the design, driver, source revision,
small receipts and artifact index using the existing ownership rules.

## Source pointers and current evidence boundary

- `src/ml_dataset/mod.rs`: `PlanArgs`, `GenerateArgs`, `DatasetPlan::validate`,
  `DatasetPlan::cell`, `generate`, `verify_protocol`; exact flags, axis order,
  worker limits, typed plan binding, protocol-file versus body hashes.
- `src/cryptanalysis/binary_semaev_chain_sat.rs`:
  `chain_s3_max_variables`, `finite_domain_clause_count`, `build_with_gate_map`;
  admitted arities, variable bound, opt-in native encoding semantics.
- `src/ml_dataset/solvers.rs`: `sat_solve`, `SatEventLog::append`,
  `native_sat_trace_caps_preserve_an_exact_accepted_prefix`; the retained test
  checks bounded event-prefix custody using a synthetic SolveBegin event. It
  does not establish native PDP decisions or the learned/backjump CLI gate.
- `src/ml_dataset/sat_prefix.rs`: `replay`, `rup`, the learned unit/nonunit
  controls, and `native_mixed_solver_stream_matches_independent_boolean_replay`.
  The latter is an ordinary small CNF/XOR fixture, not a PDP CLI fixture.
- `src/ml_dataset/export.rs`: SAT source admission, restricted joins,
  `sat_feature_occurrences`, `unique_sat_feature_rows`, `sat_outcomes`.
- `src/ml_dataset/telemetry.rs`: phase PMU and process-resource scope.
- `research/ml_dataset_algebraic_targets_20261030/cli-fixture/run.sh`: prior
  immutable re-export/hash/join checks; a reference pattern only, not evidence
  that this SAT acceptance gate has run.

At preparation time the runtime is uncompiled, the shared adapter proposal is
unapplied, and no candidate is known to qualify. The bounded selection can
legitimately end with a missing native PDP learning-transition fixture.
