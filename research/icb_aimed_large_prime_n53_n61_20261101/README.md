# Aimed m=3 plus exact 2LP at n=53 and n=61

Status: **prospective draft, audit-blocked; zero n=53/n=61 rows have run.**
Source, protocol, native analysis/export, provider preflight, identities, and
refreshed tests are not frozen. No target command is permitted until the
machine protocol and every gate in [`PROTOCOL.md`](PROTOCOL.md) are accepted by
an independent audit.

The narrow question is whether exact double-large-prime graph elimination can
reduce the complete cold cost of the validated Table A one-target aimed `m=3`
pipeline on `K_0/GF(2^53)` and `K_0/GF(2^61)`. Within a size, the factor-base
recipe, aimed window, sparse log solver, walked descent, public targets,
verification, matched rho comparator, and cold accounting boundary stay fixed.

This study does not construct a 32-target Table C batch by repeating the
one-target method. Shared preprocessing and an amortized verdict are out of
scope and need a separate versioned method/protocol.

## Candidate method

`ic.koblitz_descent_lp_v1` is a new identity; the frozen
`ic.koblitz_descent` entry is not modified. For an active LP arm, it probes the
same aimed three-summand window, uses Base membership only to keep a bounded
residual domain disjoint, and never emits a direct two-summand Base relation.
The exact frozen `PairSumTable` scan runs first. Only a probe with no ordinary
hit enters a partials-only LP pass, so the graph cannot pre-empt an ordinary
witness or mutate on a probe the frozen stream would consume. The exact named
third summands visited by the incremental second pass are included in
`summands_scanned` and exposed separately as
`lp_partial_scan_trials` and `lp_partial_summands_scanned`. Exact graph-closed
rows are mapped into the frozen projected columns, and both raw and projected
group equations are verified before sparse ingestion.

The prospective treatment is the full versioned adapter composition. A
deterministic focused gate must first prove parity with the frozen collector's
ordinary aimed three-sum opportunity census and probe scalars; that gate
removes a known direct-search confound but does not relabel graph elimination
alone as the measured treatment.

`large_primes=0` delegates at the final `SolveReport` boundary. The old
focused PASS receipt predates audit repairs and is invalidated. A refreshed
source-bound receipt must cover sparse mutated-row rejection, final LP0 report
equivalence, three-summand/direct-two-sum invariants, merged-row recovery,
accounting ownership, resolved plans, provider topology, native analysis, and
canonical export.

## Frozen design after approval

Each target cell uses eight public targets, one warmup, five measured rounds,
deterministic per-target/round arm shuffling, L2 isolation, and six arms:

| Arm | Purpose |
|---|---|
| `rho-v2` | matched signed-Frobenius rho, descriptive distance |
| `aimed` | frozen rule-v8 aimed baseline |
| `aimed-aa` | same-method position/noise calibration |
| `aimed-lp0` | new identity with exact frozen-path delegation |
| `aimed-lp2-c05` | exact 2LP, residual-domain factor 0.5 |
| `aimed-lp2-c1` | exact 2LP, residual-domain factor 1.0 |

The complete-cold boundary includes factor-base and pair-table construction,
LP-domain/table construction, collection, graph elimination, sparse log solve,
target descent, and verification. Complete paired wall time on one immutable
binary and homogeneous `c8a.2xlarge` L2 placement is the primary finite-host
metric. The 0/0.1/1 counted-cost band is descriptive: zero price is a GAE
lower bound, while missing native counts make affected positive weights and
the complete-pricing verdict `UNRANKABLE PRICING`.

The provider request is prospectively fixed at four exclusive vCPUs, 8 GiB,
3,600 seconds per target child (600 for the control), and a distinct
7,800-second outer Job deadline. The latter covers the estimated 6,600-second
session plus replay, native export, hashing, and upload. Requests and limits
are equal, the instance/capacity selector is fixed, restarts and scientific
retries are zero, and the image reference is digest-suffixed. Failed/refused
attempts and versioned readback-verified uploaded objects remain part of the
evidence. Each attempt explicitly names the provider-identity and Pod-spec
artifacts whose recomputed hashes must match their pre-data request anchors.
The Job/Pod/node/instance lineage object also has its own canonical SHA-256,
bound with the request, terminal object, and artifact listing.
The request payload is exactly the named spec file, so its byte count and
SHA-256 are independently recomputed rather than accepted as metadata.
Frozen request files use exact protocol-canonical bytes. The provider identity
artifact has a fixed semantic identity shape, while the Pod/Job projection is
deterministically reconstructed from the frozen image, argv, payload,
placement, resources, restart policy, and both deadlines; native preflight and
analysis reject a supplied Pod hash that does not equal that reconstruction.
Every attempt retains the frozen request, stdout/stderr, terminal JSON,
`SHA256SUMS`, and both projections. Produced sessions retain their five core
files; a successful complete session additionally requires replay, export, and
all decisive control/comparison receipts in the same versioned,
readback-verified custody listing.

## Controls and inference

The n=13 positive control has two targets, five arms, one warmup, and five
measured rounds: 60 executions, 50 measured. It must yield ten exact A/A
pairs, ten exact LP0 pairs, twenty active 2LP rows each containing a merged
relation, no direct two-sum, no LP verification failure, and complete
replay-all/custody receipts.

For n=53/n=61 the real wrapper sees `rho-v2` as first reference and
`aimed-aa` as first control; `aimed` supplies its same-method A/A peer. For the
control it sees `aimed` then `aimed-lp0`, with `aimed-aa` later. A native
preflight must reproduce that exact exit-43 topology from resolved plans.

Inference uses eight independent target clusters per size; the five rounds are
repeated timings. The native analyzer reports target-cluster intervals and an
exact `2^8` sign-flip test of `candidate - 0.95 * aimed`, then Holm-adjusts the
four factor-by-size wall tests. One factor is an `IMPROVEMENT` only when its
upper wall-ratio bound is at most 0.95 and adjusted `p <= 0.05` at both sizes,
after every exact gate. This is a finite-host result, not an exponent or
cross-hardware claim.

Scientific exhaustion/cap/timeout is retained as
`CENSORED NON-IMPROVEMENT`; authenticated resource/infrastructure refusal is
`BLOCKED`; `INVALID` is reserved for protocol, identity, custody, correctness,
export, or replay violations.

The canonical export has exactly 636 rows, one for every scheduled control or
target execution. Never-started placeholders use `status: "cancelled"`,
`attempts: 1` as required by `experiment.export/v1`, null measurements/times,
and an explicit unattempted reason. They do not increment the independent
`flow.attempted` execution-start count, and a cell with no provider request has
an empty attempt-aggregate array. The export keeps partial blocked/censored
records excluded and wall-invalid; provider request,
Job, Pod, node, instance, attempt, and restart lineage supplies the row
attempt/backend fields.

## Evidence and limits

PR #2850 measured a different compact-orbit `ic.pipeline` component. Its best
stage ratios, 0.604 at n=53 and 0.559 at n=61, came from two rounds with no
warmup and are not pooled with this study.

This candidate is generic pair-table decomposition plus exact large-prime
graph elimination. Its counting model assumes approximately uniform
independent subgroup sums, correct signed-orbit canonicalisation, a bounded
disjoint residual domain, and useful rank only after exact residual elimination
and row verification. The repository's adverse residual-yield result found
single-LP relation phases 1.9--4x the direct method at n=23 and explains why
this experiment may fail.

Shoup's generic-group lower bound rules out treating this generic composition
as a new asymptotic ECDLP attack. Gaudry--Thériault--Thomé and Diem supply
large-prime graph precedents in different genus/class-group models; their
complexity theorems do not transfer to this Koblitz elliptic-curve workload.

The machine-readable authority is
[`protocol/protocol.json`](protocol/protocol.json). Result and custody shapes
are defined by [`RESULT_SCHEMA.json`](RESULT_SCHEMA.json) and
[`RECEIPT_SCHEMA.json`](RECEIPT_SCHEMA.json). Until those, the native analyzer,
the source receipt, and image/checkpoint identities are frozen and audited,
this directory is a design packet, not empirical evidence.
