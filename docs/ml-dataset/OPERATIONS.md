# Dataset campaign operations

The native entry point is `crypto dataset`. Read [the design](DESIGN.md) before
using the data for a learning experiment. Each CLI invocation has a finite cell
budget. A planned cell, a saved known-answer instance, a solved point decomposition
and an independently recovered discrete logarithm are different records.

## Planning

`dataset plan` constructs a mixed-radix index without materializing the corpus.
Its default cross-product has 24 field sizes, 14 arities, three curve families,
two twist choices, 16 curve seeds, eight generators, 64 target keys and ten
methods: **165,150,720 method cells**. This is a derived planned count, not a
count of completed solutions. Every curve/target configuration has both twist
choices. The prime field at n=61 is exactly 2305843009213693951.

Freeze an `experiment.protocol/v1` record with the shared research protocol
checker and commit it before generation. The plan binds the SHA-256 of the
complete protocol file. That file hash differs from the protocol's canonical
body hash in `freeze.protocol_sha256`; retain both.

```sh
crypto dataset plan --out campaign-plan.json --protocol-sha256 FILE_SHA256
crypto dataset capabilities
```

For a smaller frozen pilot, set axes explicitly:

```sh
crypto dataset plan --out pilot-plan.json --n 23 --m 2,4,15 \
  --families koblitz,binary,prime \
  --solvers bsgs,enumerate,root,sat,f4,f5,f6,weil,pkm,double-large-prime \
  --curve-seeds 1 --generators 2 --keys 2 --factor-base-size 4 \
  --degree-bound 3 --work-limit 20000 --protocol-sha256 FILE_SHA256
```

This pilot has 720 method cells. Unsupported method/parameter combinations
remain visible in results. A budget exhaustion is censored evidence, not UNSAT.
Natural random targets are kept separate from deliberately planted test
fixtures; passing a planted unit test does not make a natural pilot PDP solved.

## Execution and recovery

Build the binary from a committed source snapshot with
`CRYPTO_BUILD_GIT_COMMIT` set to that commit and `CRYPTO_BUILD_GIT_DIRTY=false`.
The generator refuses missing or dirty provenance. An output directory is
bound to the exact binary, plan, source, resource limits and shard.

```sh
crypto dataset generate --plan pilot-plan.json --protocol protocol.json \
  --out /work/dataset --max-cases 36 --shard-index 0 --shard-count 20 \
  --seconds-per-case 30 --memory-mib 2048 --trace-mib 64 --publish \
  --remote s3://setec-astronomy-artifacts/store
crypto dataset verify --out /work/dataset
crypto dataset status --out /work/dataset
```

Use a digest-pinned image on the repository's EKS Spot lab. Assign disjoint
shards before submission, record every Job identity, and inspect the existing
Job and S3 receipts before retrying a submission. Worker-local process leases
do not provide a distributed cloud lock. The ordinary throughput Job does not
establish CRIU recovery, isolated benchmark clocks or comparative speed claims.

Reissuing the same generation command skips verified completed cells and retries
their publication when needed. A partial cell requires explicit reconciliation:

```sh
crypto dataset reconcile --out /work/dataset --cell CELL_INDEX \
  --reason "Original worker terminated; external Job and owner reconciled"
```

Reconciliation checks local Linux processes and preserves the partial directory
under an immutable attempt name before allowing another attempt. It cannot
prove that a worker on another host has terminated. Retain all interrupted
attempts when moving a campaign to another host.

## Artifact and measurement contract

Each sealed cell includes protocol, plan, run binding, exact public curve and
target, restricted scalar/provenance, factor base, backend artifacts, process
logs, phase telemetry and a manifest binding every emitted file by length and
SHA-256. Save raw witnesses and seeds as restricted labels, never model inputs.

Publication uses the content-addressed store and a namespace derived from the
complete run binding. Uploads are verified by downloading and hashing every
object, including the final completion receipt. `status` checks the recorded
acknowledgement; `publish` performs a fresh remote check. No receipt is evidence
that the underlying algorithm completed unless its outcome and replay say so.
Objects larger than 4 GiB must be chunked; this implementation rejects them
because the existing multipart store path does not guarantee immutability.

Hardware counters use the existing native PMU instrumentation for construction,
known-answer replay, factor-base replay and solver phases. Save raw counts,
scaled estimates, time enabled/running, inheritance and errors separately.
An unavailable PMU produces a null instruction label with its reason. Counts
cover user-space work in the calling thread and newly spawned descendants when
inheritance succeeds. They exclude process startup, publication and unrelated
preexisting threads. Multiplexed counts are estimates, not exact instruction
labels. Interrupted phases retain their start record but may lack a final PMU
read. Wall times on shared workers are diagnostic.

Trace byte caps apply to worker output before manifest sealing. Manifest and
publication metadata are accounted separately. The supervisor preserves partial
files after a killed call; complete native algorithm internals still require
explicit trace hooks. Capability records name those gaps. Exact first-fall and
last-fall degrees remain null until a checker establishes their stated algebraic
definition and all required closure/rank conditions.

## Before training

Use public equations, factor-base coordinates and admissible prefix states as
features. Keep answers, future pivots, recovered logs, construction seeds and
future conflict clauses in the target/restricted partition. Group equivalent
presentations, signs, Frobenius orbits, generators, twists and related cases in
the same holdout component. The initial conservative split groups by field size;
cross-size embeddings need an additional component merge before training.

Validate public export, split disjointness and leakage before a training run.
Retain all capped, unsupported and failed rows. Fit success probability and
resource predictors with censoring, and measure learned SAT or elimination
policies against frozen classical baselines with held-out instances. A neural
prediction about hidden algebraic structure becomes a mathematical result only
after independent symbolic or formal verification.
