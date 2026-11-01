# Prospective execution checklist

Machine protocol: [`protocol/protocol.json`](protocol/protocol.json).

Current state: draft and audit-blocked. No target-size row exists. Target data,
commit, freeze, checkpoint publication, image construction, and launch remain
disabled until every item below is green and an independent audit accepts the
packet.

This is the complete-cold one-target/Table A contract. Repeated one-target
calls are not a Table C batch. Shared 32-target preprocessing and an amortized
verdict require a separate versioned method and protocol.

## 1. Source and focused gates

The frozen `ic.koblitz_descent` entry must remain unchanged. The new
`ic.koblitz_descent_lp_v1` must pass one refreshed native receipt, from the
eventual source commit, covering all of the following:

- LP=0 delegates at the final `SolveReport` boundary and equals the frozen
  method after excluding identity and clocks: answer, relation scalars, column
  logs, phases, counters, unpriced list, factor-base identity, JSON, and replay;
- every LP candidate relation has exactly three summands, the two-summand Base
  branch is never emitted or admitted, and `aimed_lp_direct_two_sum == 0`;
- on the same deterministic n=13 base, unit, and aimed window, the nonzero-LP
  adapter has exactly the frozen collector's ordinary three-summand hit-trial
  census and probe scalars, and every adapted ordinary witness replays;
- toy recovery is under-ranked without graph elimination and succeeds only
  after at least one merged 2LP projected row;
- raw-to-projected sparse mapping solves and replays, while a mutated sparse
  coefficient is rejected fail-closed;
- the exported aimed window is the frozen collector's exact scan and only the
  named third summands are probed;
- global thread-local accounting owns group adds/doubles, canonicalisations,
  and Frobenius coordinate maps exactly once; LP-local accounting contributes
  only disjoint hash/table work and wrapper scalar-multiplication metadata;
- setup and relation wrapper scalar multiplications survive into final phase
  and total counts without duplicating their underlying adds/doubles.

The existing focused receipt is audit-invalidated. Its earlier PASS lines and
two infrastructure-only events remain provenance, not a launch gate.

## 2. Prospective identity and native analysis freeze

Before any target-size command, freeze and bind:

- one source commit in every arm, the focused receipt, and
  `.resume/RESUME.md`;
- exact bytes and SHA-256s for the protocol, specs, schemas, manifest, focused
  receipt, provider requests, native analyzer source, and native provider-
  preflight source;
- locked release `ecbench` and native-analyzer SHA-256s, Cargo.lock SHA-256,
  build flags, spec file/identity hashes, immutable image digest, and image
  receipt;
- a canonical `experiment.export/v1` contract and authenticated request, Job,
  Pod, attempt, terminal-state, and artifact receipts, including failed or
  refused attempts;
- `freeze.status = frozen`, canonical protocol hash, source commit,
  freeze/checkpoint receipt, and `filed_before_affected_data = true`.

Python is not an experiment analyzer, correctness oracle, replay path, result
generator, or export generator. The frozen Rust analyzer must reject duplicate
JSON keys, identity drift, missing attempts/artifacts, malformed exports,
incomplete rows, invalid replay, and provider-topology drift. A separate native
preflight must resolve all three specs and reproduce the real
`run-ecbench` first-reference/first-control plus `choose_aa`/`aa_pairs >= 5`
exit-43 gate.

Any unresolved identity, stale receipt, or non-native scientific path blocks
execution.

## 3. Provider and resource contract

Use `isolab_k8s_benchmark` on a pinned `c8a.2xlarge` Spot worker. The exact
request has the fixed instance/capacity node selector, equal four-CPU requests
and limits, equal 8-GiB memory requests and limits, `restartPolicy: Never`,
zero provider/scientific retries, 3,600 seconds per target child (600 seconds
for the control), and a distinct 7,800-second outer Job deadline. The outer
deadline covers the 6,600-second session estimate plus replay-all, canonical
export, and upload.

Before freeze, each request template must contain the exact argv and exact
payload bytes/SHA-256 (the payload is exactly the named spec file), a digest-
suffixed image reference whose suffix equals the separately bound digest,
provider-identity and Pod-spec hashes, and its cell-specific prefix under
`s3://setec-astronomy-artifacts/ecbench/icb-aimed-large-prime-n53-n61-20261101/`.

Frozen request files are the exact whitespace-free, key-sorted bytes produced
by `protocol_canonical_bytes`. `provider_identity_sha256` anchors a canonical
`icb.aimed2lp.provider_identity/v1` object with exactly `provider_tool`, cloud
account, region, cluster, namespace, service account, and admission-controller
identity. `pod_spec_sha256` is not an opaque supplied value: it is recomputed
from the canonical `icb.aimed2lp.pod_job_spec/v1` projection containing the
provider tool, Job name, digest-suffixed image and digest, exact argv, payload
hash/bytes, instance/capacity and exact node selector, all CPU/memory resource
fields, `restartPolicy`, child timeout, and outer deadline. The observed
provider and Pod/Job projection artifacts must be canonical bytes with those
exact shapes and hashes.

The native provider preflight must prove:

- n=53/n=61: first reference `rho-v2`, first control `aimed-aa`, and separate
  same-method `aimed` peer supplies the A/A calibration;
- n=13: first reference `aimed`, first control `aimed-lp0`, and later
  same-method `aimed-aa` supplies the A/A calibration;
- all three plans resolve, require L2, have at least five rounds and one
  warmup, and produce at least five A/A pairs under the real wrapper route.

Resource refusal or unavailable exact placement is `BLOCKED`; it never
authorizes a smaller domain, different host, altered target set, or longer
scientific cap.

## 4. Pre-target positive control

Run `specs/control-n13.json` in the frozen image at L2. It schedules two public
targets, five arms, one warmup, and five measured rounds: 60 executions total,
50 measured. Retain the complete session, failed rows, replay-all audit,
canonical export, provider attempt receipt, and artifact hashes.

The control passes only with:

- 50/50 verified measured rows and replay of every eligible row;
- ten operation-identical aimed/A-A pairs and ten exact aimed/LP=0 pairs;
- all twenty measured 2LP rows reporting partial activity, a projected row,
  at least one graph-merged row, zero projected-row verification failures, and
  `aimed_lp_direct_two_sum == 0`;
- one complete L2-bound session and a passing real wrapper topology gate.

A failed scientific control blocks the targets. Changing toy parameters
requires a pre-data versioned amendment, a new source/test receipt, and a new
freeze.

## 5. Target cells and custody

Only after the preceding gates pass may n=53 and n=61 launch concurrently,
subject to the frozen budget. Each cell is exactly 288 executions: six arms
times eight public targets times one warmup plus five measured rounds. Arm
order is the deterministic frozen shuffle.

For every attempt, including a failed or refused Job, retain:

- canonical request bytes/hash, request id, Job UID, Pod name/UID, node name,
  instance id/type/capacity, attempt number, exactly zero container restarts,
  exact requests/limits and node selector, image digest, argv, payload
  bytes/hash, provider-identity and Pod-spec hashes, timestamps, terminal
  reason, and provider status. The canonical lineage object itself is
  SHA-256 authenticated alongside request, terminal, and artifact listing;
- produced `session.json`, `host.json`, `plan.json`, `records.jsonl`, stdout/stderr,
  `job-exit.json`, replay-all audit, comparison receipts, canonical export,
  `SHA256SUMS`, uploaded object keys/versions/sizes/hashes, and an authenticated
  object listing. Each attempt names the exact provider-identity and Pod-spec
  artifacts; native analysis recomputes their local bytes and matches them to
  both the pre-data request anchors and the authenticated attempt fields;
- every scheduled outcome without filtering failures, exhaustion, caps,
  timeouts, lower-isolation rows, or incomplete attempts. Every uploaded
  object has a non-null object version and a second readback whose bytes and
  SHA-256 equal the locally authenticated artifact.

Every attempted envelope custodies the exact frozen request path and exactly
one each of `stdout.log`, `stderr.log`, `job-exit.json`, `SHA256SUMS`, the named
provider-identity artifact, and the named Pod/Job projection artifact. An
attempt that produced a session additionally custodians that session's
`session.json`, `host.json`, `plan.json`, `spec.json`, and `records.jsonl`.
A successful complete attempt must also custody replay-all, the per-cell
canonical export, and every decisive control/comparison receipt. For a partial
attempt those latter files are required if produced; a nonexistent PASS-only
control receipt is not fabricated. Every file consumed by native analysis must
have exactly one owning attempt and the same versioned readback-verified bytes.

The canonical export always contains exactly all 636 scheduled executions
(60 control plus 288 at each target size). `experiment.export/v1` requires each
scheduled row to carry one row-level scheduling attempt, so never-started
executions use `status: "cancelled"` with `attempts: 1`, null measurements,
null start/finish times, and an explicit unattempted exclusion reason.
`flow.attempted` is the execution-start count and remains zero for those rows;
a cell that never reached a provider request has `attempts: []` in its separate
attempt aggregate. Rows from blocked or
scientifically censored cells remain present but are excluded from inference
and may never be marked wall-valid. Attempt counts and backend/worker fields
come only from the authenticated provider lineage, never from record IDs.

Scientific exhaustion, a cap, or child timeout is retained once and produces
`CENSORED NON-IMPROVEMENT`; it is not a retry. An authenticated resource
refusal or infrastructure failure is `BLOCKED` and alone may use the one
frozen replacement attempt. `INVALID` is reserved for protocol, identity,
correctness, exact-equation, export, custody, or replay violations.
After a passing positive control, neither target may be `not_attempted`;
placement refusal is an authenticated `infrastructure_blocked` attempt. After
a non-passing control both targets must have empty attempt arrays and the exact
reason `positive control did not complete; target launch prohibited by
protocol`.

## 6. Frozen read-out

The independent units are the eight public targets per size. Five rounds are
repeated timings inside a target cluster, never `n = 40`. For each domain
factor and size, the native analyzer:

1. pairs every measured candidate row with the corresponding aimed row;
2. collapses repeats within each target and reports the ratio of complete-cold
   wall sums;
3. reports a target-cluster 95% interval;
4. enumerates all `2^8` target-level sign assignments for the preregistered
   contrast `candidate - 0.95 * aimed`, under the frozen independent,
   sign-exchangeable target-contrast assumption;
5. applies Holm to the four candidate-by-size wall tests.

The minimum one-sided p-value is 1/256. A configuration is `IMPROVEMENT` only
if its adjusted `p <= 0.05` and complete-cold wall-ratio upper bound is at or
below 0.95 at both sizes, after exact correctness, LP=0, A/A, L2, custody, and
replay gates. Two sizes do not support an exponent, cross-hardware claim, or
result above n=61.

The 0/0.1/1 GAE-per-native-operation band is mandatory but descriptive. The
zero-price value is explicitly a group-operation lower bound. If a named
native unit lacks a numeric count, affected nonzero weights are null and the
pricing verdict is `UNRANKABLE PRICING`; this neither upgrades nor erases the
finite-host wall verdict. Rho distances are descriptive only.

## 7. Method-governance scope

This is generic pair-table decomposition plus exact large-prime graph
elimination, not a transferred algebraic complexity result. Its counting
model assumes approximately uniform independent subgroup sums, correct
signed-orbit canonicalisation, a bounded disjoint residual domain, and useful
rank only from eliminated and independently verified rows. The repository's
residual-yield analysis is an adverse prior: reducing the residual domain
lowers entry and collision together, and the n=23 single-LP measurements were
1.9--4x the direct relation phase.

Shoup's generic-group lower bound remains applicable to a decomposition that
uses only generic group operations; no sub-square-root asymptotic ECDLP claim
is made. Gaudry--Thériault--Thomé and Diem motivate large-prime graph
elimination in different curve/class-group relation models. Their genus and
smoothness assumptions do not transfer a complexity claim to this Koblitz
elliptic-curve workload. Only the prospective finite-host result can decide
the engineering question here.
