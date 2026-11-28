# Common cold-recovery accounting

2026-11-24 · Local development qualification · **Unsigned, unverified evidence**.

The new recovery path uses Isogeny Labs' existing relation consumer and accounts
for setup, every mask draw and query, incremental rank/linear algebra, and final
scalar replay. Three fresh-process arms recover the public F127 fixture and
reconcile their child and parent intervals. The cofactor arm retains five
unsupported target-in-base queries. This is an accounting implementation result;
no timing ratio or benchmark admission is claimed. [pcc:unverified]

## Recorded control

Frozen source **a4cd5ee0b3**. Executable SHA-256:

```text
f7fb325a7913246b80049cdaf066b635cbc1ff643c780f935872855741414b3b
```

The fixed fixture is \(E/\mathbb F_{127}:y^2=x^3+x+7\), order 109,
G=(1,3), Q=(11,98), selector depth 4, seed 781815, maximum 30 draws.
One run per arm; no warmup, repeated sampling or isolation admission.

| Arm | Draws / queries | Incomplete queries | Selected rows | Consumer rank | Scalar |
|:--|--:|--:|--:|--:|--:|
| Direct | 9 / 9 | 0 | 9 | 9 | 69 |
| Refined | 10 / 10 | 0 | 9 | 9 | 69 |
| Cofactor + msolve | 12 / 12 | 5 | 7 | 7 | 69 |

Values are recorded, not estimates. The base has eight unsigned points. Consumer
rank includes the target-scalar coordinate; it is not the historical base-only
rank. Recovery tests membership of that coordinate functional in the row space,
so full rank is unnecessary. All final scalar replays pass locally. [pcc:unverified]

![Recorded query and row counts; one fixed target/seed per arm. Source: evidence/cold_accounting_v1/graph_data.csv.](cold-accounting-counts.pdf){width=96%}

**Flow:** three arms planned, attempted and finished; none excluded or retried.
The wrapper and every child exit zero. All three parent receipts reconcile,
and all 37 records are retained. Five individual cofactor queries remain
undetermined, at draws 2, 3, 4, 8 and 11: the production builder rejects a target
already in its factor base. Zero exit and successful eventual recovery do not
turn those calls into complete relation sets. [pcc:unverified]

Direct/cofactor use distinct leaf x-coordinates, all different from the target x.
Refined additionally requires four distinct coarse Dickson images. Its narrower
query scope remains explicit. Direct alone constructs the pair index; neither
algebraic arm receives a fallback dictionary. The first sorted row is consumed
after each complete nonempty query. No known scalar enters this interface.

\newpage

## Accounting and development checks

![Accounting boundaries; parent forwarding and validation are outside its child interval.](cold-accounting.pdf){width=98%}

The child starts timing on entry to main. Disjoint phases cover input validation,
base and oracle setup, consumer setup, public-mask generation, the entire oracle
call, the consumer, final replay and record emission. The consumer interval
includes canonicalization, signed equality replay, duplicates, exact rank,
incremental linear algebra and its scalar check. Every skipped or failed draw
is retained. Phases plus explicit unattributed overhead equal pre-summary wall
time; summary serialization and teardown are captured by the parent instead.

The parent interval runs from spawn to observed exit, including up to 5 ms
polling latency. It excludes its own parsing, forwarding and receipt emission.
Algorithm state is fresh; OS caches are not flushed. Raw nanoseconds test this
instrumentation, and partial consumer operation counters are not a full
arithmetic price. No local value is promoted to a cost comparison.

The feature suite passes 23 tests and the default suite six, with a successful
release build. Controls exercise historical row reduction, solver failure
without fallback, receipt mutation, identity/duplicate draws, process-group
termination and truncated output. The 19 Direct/Refined query-history records
match the earlier frozen recovery in masks, targets, selected rows and relation
counts. These checks are producer-local, not independently signed. [pcc:unverified]

## Evidence, limits and next step

- [Protocol](COLD_ACCOUNTING.md), [reproduction](COLD_ACCOUNTING_REPRODUCE.md),
  [raw records](evidence/cold_accounting_v1/control.jsonl),
  [source identity](evidence/cold_accounting_v1/source_identity.json),
  [manifest](evidence/COLD_ACCOUNTING_SHA256SUMS).
- [Accounting source](https://github.com/isogeny-labs/crypto/blob/a4cd5ee0b3/research/smooth_p_plus_one_20261123/native/src/accounting.rs)
  binds the existing [production consumer](../prime31_four_summand_20261122/native/relation_adapter.rs).
  The [cofactor](COFACTOR_OUTPUT_RESULT.md), [oriented](ORIENTED_RESULT.md) and
  [initial literature/identity](REPORT.md) comparisons retain their earlier scope.
- PCC-20261124-212b0d covers tests/build; PCC-20261124-cc08b7 covers the control;
  PCC-20261124-69c854 covers recorded-history comparison. All remain unverified.
- The 2 GiB virtual-memory limit is a bound, not measured peak memory. Each
  child is limited to 120 seconds, each query to three seconds; the outer suite
  has 400 seconds plus five-second grace. No limits were raised.
- A later [source-layout repair](native/SOURCE_LAYOUT.md) restores a historical
  pinned source artifact and moves the current module byte-for-byte. The claim,
  gate, frozen control and executable remain unchanged. All 26 branch claims
  return not_covered, with no local verifier failures; all 23 feature tests pass
  after the move (PCC-20261124-5d70b3, unverified). [pcc:unverified]
- Root-library compilation and the unchanged historical benchmark-record gate
  limitation remain in [development checks](DEVELOPMENT_CHECKS.md). No EKS job
  was submitted. Current main does not package this driver plus msolve.
- Canonical `experiment.export/v1` schema/tooling could not be read from the
  moved crypto-autoresearcher repository (VLT1 transform/UTF-8 decoding errors).
  [development_flow.json](evidence/cold_accounting_v1/development_flow.json)
  is explicitly a local receipt, not a validated canonical export.

The original research requirement remains partial: output/rank and accounting
components exist, but no admitted complete-cost gain or new general mechanism
has been established. Next handle target-in-base residuals in a separately frozen
comparison applied equally to every arm, then register the exact workload for
admitted execution. Degree-32 cofactor and full-base oriented output remain open.
Prior figures, the scoreboard, timeline and leaderboard receive no new admitted
row; this component adds only its own count graph and accounting diagram.
