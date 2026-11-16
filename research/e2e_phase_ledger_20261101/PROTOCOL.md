# Frozen design: complete cold E2E phase ledger

The machine-readable authority is `protocol.json`.

## Question

On Koblitz E0 at `n=13`, how do current F4, F5, F6-IC and root-finding PDP
backends change the complete cold resource vector when relation collection is
followed by a full-rank, independently checked relation solve?

## Fixed design

- Two shards, four independently generated public targets per shard.
- One warmup and six measured rounds; alternating arm order.
- One native `ecbench` binary and one EKS `c8a.2xlarge` Spot host per shard.
- L2 isolation is mandatory for wall-time claims.
- `koblitz-standard-subspace:dimension=5`, m=3, degree 3, node budget 8192,
  and 200000 relation trials for the common backend comparison.
- `incremental-gauss-full-rank-checked` for every IC arm.
- m=2 root-finding uses dimension 4 and is a labelled structural rank-ceiling
  diagnostic, not a one-factor comparison with m=3. A pre-freeze smoke run on
  a discarded target stream plateaued at rank 10 of 11 under the full-rank
  rule; censoring is the expected admissible outcome.
- A strong signed-Frobenius rho reference and an identical A/A rho control are
  present in every shard.

## Primary evidence

The primary measurement is the complete counted `bitop/v1` resource vector,
including its phase split and coverage flags. A scalar ranking is admissible
only where `work.complete` is true for both records. The 0/0.1/1 GAE band is
reported beside it. Cold wall time is secondary and comparative only within a
single L2 shard, with A/A spread shown.

No native F4/F5/F6 counter is equated to another backend's native counter.
Solver calls, degrees, rows, columns, nonzeros, rank, XORs, searches, peak
bytes, relation trials, accepted relations, matrix work, and verification
outcomes remain separate columns.

## Censoring and stopping

Each child has a fixed 90-second cap. Timeouts and failures are retained and
never retried for algorithmic reasons. Infrastructure-only retry is allowed
once and must produce a new session identity. A shard is invalid for wall
comparison if any counted record is below L2, fails replay, lacks its matching
target/round peer, or uses a different binary.

Instrument smoke used target seed `202611010130`. The registered target and
measurement seeds were rotated afterward and before freeze; smoke records are
not study data or admissible evidence.

## Decision rule

A backend is called the scoped winner only if all its recoveries verify, its
complete `bitop/v1` total is lower than inherited F4 on the paired targets,
the ordering survives the declared unpriced-work band, and its same-session
L2 cold-wall interval beats inherited F4 beyond the rho A/A spread. Otherwise
the result is a Pareto/resource-vector report or inconclusive. No extrapolation
to ECC2K-130/131 is permitted.
