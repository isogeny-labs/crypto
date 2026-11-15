# Point-table pilot: software contract, not launch admission

Status: canonical protocol **draft**. No 12/16-bit oracle batch or timing
measurement has been taken. Input generation and development controls are
preparation, not study outcomes. The original frozen native feasibility panel
and all earlier evidence remain unchanged.

Current canonical draft: [protocol-v3.json](protocol-v3.json). The
[2026-11-08 user budget exception](budget-override-20261108.json) removes
account/lane headroom gating for this first pilot within a USD 10 total cap.
The old [v1 draft](protocol.json) is retained unchanged. All measurement,
authentication, source/image, resource and review requirements continue.

## Requested experiment and current software

The question is whether prepared points, exact membership indexes and hash
tables reduce useful relation-search costs after their construction is charged.
This first pilot retains the original native final-slot derivation baseline.
It does not use a quadratic pair scan as that baseline.

| Requirement | Software path | Remaining admission work |
|---|---|---|
| Fixed public inputs independent of arm | `make-plan`, compact context recipe, query/control hashes | Commit and checkpoint the exact generator/recipe source |
| Complete attempt flow | Deterministic 2,368-row schedule and append-only start/finish ledger | Native worker/image and durable S3 output integration |
| Balanced nuisance conditions | Fresh child per batch; randomized context/arm blocks, paired reversals | Native L2/host/rusage qualification and A/A timing admission |
| Exact relation answers | Original arithmetic witness replay plus word-group reconstruction | Independent audit and signed PCC standing |
| Paired analysis without pseudoreplication | One counted batch per arm/base context; base-seed clusters | Masked canonical export and final preregistration |
| Complete costs | Native preparation/query/replay counters and raw clocks | Full inventory/calibration, checker/harness/resource costs and external references |

## Fixed pilot recipe

Use the registered 12- and 16-bit prime/cofactor-one curves, thresholds 32 and
128 and base seeds 0 through 7: **32 contexts**. Each context supplies the same
256 uniform public subgroup query points to all six arms. Separate SHA-256
domains derive base and query seeds. Rejection sampling avoids modular bias.
The native prime sampler chooses the common affine-hash prime; its RNG then
draws both affine seeds. Parameter sampling is supplied component-input
preparation, not a silently priced phase of a full ECDLP solve.

The compact JSON recipe stores all curve/hash parameters and hashes the query
and control vectors. It also binds the full expanded input hashes and schedule.
`check-plan` reconstructs these deterministically and rejects changed contexts,
queries, order, counts or duplicate identities. Generating this recipe invokes
no native decomposition oracle and records no study clocks.

Keep actual base cardinalities, empty bases and failed/exhausted attempts. A
context is never resampled to create a desired base or yield.

## Enumerated attempts

| Stage | Attempts | Purpose |
|---|---:|---|
| Context controls | 192 | One counts-only control batch per context/arm |
| Forced collisions | 64 | Constant hash, both hash-index arms, ordinary fixed queries |
| Counted diagnostics | 192 | One counts-only ordinary batch per context/arm |
| Warmup | 384 | Two rounds x 32 contexts x six arms |
| Measured | 1,536 | Eight rounds x 32 contexts x six arms |
| Total | **2,368** | No automatic retries or restores |

The original proposed **1,920 timed/warmup arm-batches** are preserved.
Additional counts/control attempts, previously left unenumerated in the draft,
are now explicit. Preflight runtime calibration, image builds and external
references need separate costed qualification; they are not hidden in this
attempt total or claimed to have run.

Word arithmetic constructs each context's controls before measurement: the
first available pair sum is positive; the first point in the generator walk
missing from the complete pair-sum set is negative. An empty base has only a
negative control; a full sumset has only a positive control. Preserve and report
these mathematical exceptions rather than resampling. All six native arms
receive identical control queries. Constant hashes retain full point equality.

Controls, collision checks and counted batches precede the timing schedule.
Within each warmup/measured round, shuffle context blocks and then all six arms.
Reverse the whole order in the second round of each pair. Each invocation uses
a fresh process per child. Counts/control children use the counts-only
accounting binary; clocks use the ordinary timing binary, which has the default
allocator and plain keys. Both release binaries share the same driver/source
and lockfile, and have separate SHA-256 identities in run.json. No table is
shared across child batches. The accounting binary refuses clocks.

## Execution and failure ledger

The executor requires a new output directory and one original absolute Unix
deadline, no later than 1,800 seconds from submission. It starts one child at a
time, binds inputs and outputs by SHA-256, retains stdout/stderr and never
overwrites an earlier run. A durable start precedes each child; a durable
finish records its exit/timeout/replay status and artifact hashes.

Any failed child, exhausted resource limit, invalid replay or timeout stops the
whole schedule. Later attempts remain unattempted. An interrupted start remains
in flight. There is no automatic resume, retry, new name or deadline extension.
The external native worker must still enforce the absolute pod/job deadline,
signal cleanup and durable artifact custody; this CLI is not that admission
layer and does not submit to AWS.

The analyzer checks ledger order/identity and artifact hashes before reading
records. A partial schedule reports its full flow with no complete-case
contrasts. It never silently discards a failed attempt.

## Analysis and limits

The v3 analyzer reports **21 count diagnostics**: native root/add/meter events,
eight key/hash/lookup events, and ten observed allocation-request/layout counters.
Snapshots cover preparation and cumulative query prefixes 1/4/16/64/256. The
System wrapper observes only the active thread's cold interval, ending before
teardown. Reporter vectors are preallocated outside it; parse/record serialization
and the separate word check are outside. Allocation optimization may elide or
combine requests. These counters are neither semantic allocation guarantees nor
usable/live bytes, allocator arenas or RSS. Internal table probe groups and peak
live bytes are explicitly null; full-key equality is not a probe-group count.
Hash feeds retain raw key behavior and are toolchain-specific.

For each size/threshold cell, pair all five candidates with Current and report
the three direct adjacent contrasts Prepared/Ordered, Ordered/Hashed and
Hashed/Fingerprint. Use one counted record per independent base seed, not queries
or timing repeats as independent observations. Missing metrics leave clusters
unavailable and forbid a complete-case ratio. Zero baseline totals forbid a
ratio; complete absolute totals remain available. The existing native
`ecbench::stats::linearised_ratio` supplies descriptive ratios and log-scale
Student-t intervals where defined. This is up to 3360 descriptive cells, not a
powered or multiplicity-controlled search. None of these diagnostics prices a
whole method.

Intervals are descriptive and pointwise. They establish neither power nor
multiplicity-controlled selection. Zero or undefined denominators stay
undefined. No winner, equivalence, runtime speedup or fastest-tool claim is
promoted from this pilot analysis. **Timing comparisons are refused** until the
native L2/host/resource and canonical export/masking integrations exist.
[pcc:unverified]

The canonical record passes structural and cross-reference checks as a draft
using the pinned readable historical validator. Current canonical plaintext
access is unresolved; this is not current launch admission. Its
source/binary admission, runtime estimate, power and freeze fields remain
unready. Passing that validator does not establish launch readiness. Freeze
the completed canonical record and analysis before the first affected data;
the existing frozen protocol in the older native study is not changed.

## Invocation contract

The native example provides `make-plan OUT`, `check-plan PLAN`,
`execute-plan PLAN NEW_DIR ABSOLUTE_UNIX_DEADLINE [ACCOUNTING_BINARY]` and
`analyze-plan PLAN RUN_DIR OUT`, alongside the existing `run`/`check` batch
commands. `make-development-plan OUT` supplies only the fixed F17 lifecycle
control profile, whose child batches always have clocks disabled. It cannot
substitute for the 12/16-bit pilot. The analyzer reports positive/negative
control coverage and the empty-base/full-sumset exceptions explicitly.
These are the software interfaces, not instructions to run a local
timing substitute. A cloud submission requires the maintained authenticated
native image/worker path and exact-source S3 checkpoint/readback, within the
scoped budget exception and retained limits. No such submission was made.

The current card supplies ACCOUNTING_BINARY explicitly. Omitting it retains
ordinary counts with allocation fields unknown; it does not satisfy the v3
accounting contract. Run headers and analyses use v2 schemas, as do batch records.
The native `qualify-development-accounting DEVELOPMENT_PLAN PLAIN_RUN ACCOUNTED_RUN
[HISTORICAL_V1_RUN]` command compares complete F17 development artifact streams,
including original v1 native counters/answers when supplied. It is a producer
software check, not authentication of measurement counters or scientific data.
