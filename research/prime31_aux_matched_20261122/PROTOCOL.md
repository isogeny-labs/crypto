# Matched auxiliary fibers on ordinary Prime31: proposed protocol

**Status: draft; no campaign execution or current canonical freeze.** This
document and `protocol.json` describe a bounded, prospective pilot. They do not
report Prime31 measurements. The planned flow is **54 IC cells plus one rho
diagnostic, zero attempted, zero completed, 55 blocked before submission**.
Development builds, fixed correctness tests, and preparation of the public
target are separate activities.

The implementation PR is [#4923](https://github.com/isogeny-labs/crypto/pull/4923).
The machine record uses `experiment.protocol/v1`, with protocol ID
`prime31-aux-matched-20261122-v1`. It remains `freeze.status = draft` until the
canonical checker, published source/input binding, and execution admission are
available. No field below authorizes bypassing those prerequisites.

## Question and scope

For one ordinary Prime31 target, determine what natural-query relations,
useful modular dependencies, and complete scalar recoveries occur for whole
ordinary auxiliary, supersingular j=1728, and Dickson fibers. Compare each
fiber with a plain base having exactly the same usable target-column count.
Within each whole fiber, compare dense membership with the retained quadratic
tower using the same production F4 solver and the same limits.

This is a feasibility and accounting pilot. It does not establish a speedup,
an asymptotic trend, the best construction seed, or superiority to an optimized
negation-aware rho implementation. Native counters remain scoped and partial.
Diagnostic elapsed time may describe an execution but cannot support a speed
ratio. A later timing comparison requires prospective L2/A/A admission and an
appropriate tuned rho control.

## Target and public input

Use the existing ordinary short-Weierstrass target

\[
p=2^{31}-1=2147483647,\qquad
E:y^2=x^3+847514846x+2087444724,
\]

with prime subgroup order `r = 2147446831` and
`G = (625037715, 2100627251)`. The implementation checks primality,
nonsingularity, subgroup membership, and the cofactor-one Hasse certificate;
independent affine arithmetic replays the points.

The public input prepared using fixture seed `20261122` is
`Q = (984837385, 1142774582)`. Its exact coordinates are recorded in `panel.json`
and `target-fixture.json` before any campaign execution. The generator's work
is recorded. The collector and oracle
receive only public curve parameters, `G`, `Q`, and `r`; they never receive the
fixture scalar. Every arm uses the same public `Q`.

## Registered cells

| Factor | Registered values |
| --- | --- |
| Auxiliary family | ordinary, supersingular j=1728, Dickson |
| Fiber depth | 6, 8 |
| Initial construction seed | 0, 1, 1093 |
| Signed relation arity | 4 |
| Arm within each family/depth/seed group | whole-fiber dense; whole-fiber tower; matched plain dense |
| Construction invocations | at most 8; invocation `i` uses initial seed plus `i` |
| Plain construction | seed 0, at most 4096 coordinate trials |
| Public query stream | seed 4188, at most 64 queries |
| Separate rho diagnostic | one production A=1 run, seed 4188, at most 1,000,000 steps |

The IC grid has `3 * 2 * 3 * 1 * 3 = 54` cells. The machine protocol names
each cell as a singleton arm, so its 55 arms and one execution per arm do not
imply 55 independent target instances. There is only one target. Identical plain
configurations can recur when references have equal usable cardinality; these
are correlated repeated configurations, not independent replicates. No
confidence interval or significance test is planned.

Run the deterministic panel order: family ordinary, supersingular, Dickson;
depth 6 then 8; seed 0, 1, 1093; dense, tower, plain; rho last. Since this order
is fixed, it cannot isolate temporal effects on elapsed time. Do not reorder
after seeing data to favor a setting.

## Membership and matching contracts

Preserve the complete coordinate fiber `V`, including coordinates that do not
lift to the target. Record the auxiliary curve, its order, kernel generator,
coset representative, rational maps, terminal value, complete fiber, and
independent polynomial certificate. Ordinary construction explicitly rejects
trace-zero auxiliaries and charges those attempts.

Let `c` be the number of usable signed target columns. The paired plain arm
constructs exactly `c` columns; it never truncates an auxiliary fiber. Record
the construction needed to determine that reference cardinality as
matching-design overhead. It remains part of the campaign's work and spending,
but is distinct from the plain algorithm's own construction. Every rejected
seed/build is retained with its available counters.

The dense/tower comparison has identical `V`, target lifts, target stream,
solver, and caps. The auxiliary/plain comparison matches usable columns but
also changes coordinate geometry, full membership cardinality, and setup. In
particular, a full auxiliary fiber can contain nonlifting coordinates while
the plain base contains only lifting coordinates. That comparison is declared
confounded in the variable ledger; it cannot isolate a single algebraic cause.

## Oracle and recovery contracts

Every query is formed as `R = alpha*G + beta*Q` before decomposition, using the
same rejection-sampled SplitMix64 coefficient stream for every arm. Never
select a query because its decomposition is already known, and never borrow a
generic discrete log to provide its coefficients. Preserve and tag `R = O`.

The sequential S3 system covers finite and identity intermediate-prefix
charts. Every accepted coordinate solution is lifted with signs and replayed
as a group relation. A first valid witness establishes a positive query; it
does not establish exhaustive extraction. A complete negative requires every
applicable chart and its extraction to finish. Timeout, degree, variable,
solution, input-term, and sign-lift caps produce unknown/censored outcomes.

For signed coefficient rows `A`, accumulate

\[
A\ell=\alpha+\beta d\pmod r.
\]

Track `rank(A)` separately from `rank([A|-beta])`. Retain repeated `A` rows:
different target coefficients can make a repeated row useful. A pinned `d`
must satisfy `[d]G=Q` in production arithmetic and independent affine replay
before the cell is labeled recovered. No rank-only success is allowed.

## Fixed caps and accounting

| Boundary | Cap or rule |
| --- | --- |
| Process | 120 s total external deadline: TERM at 115 s, KILL after 5 s grace; 2 affinity-bound CPUs, 4096 MiB |
| Campaign | serial execution, no retries, maximum USD 5 including build/idle/storage/transfer/failures |
| Collection | 64 public queries or 100 s, whichever stops first |
| Query | 1000 ms cooperative deadline shared across all charts |
| F4 | degree 512; at most 32 pairs per step |
| Extraction | 4096 coordinate solutions; 1,000,000 signed-lift attempts |
| System | 64 variables; 100,000 input terms |
| Solver stop | first independently accepted signed relation is sufficient |
| Native parallelism | `RAYON_NUM_THREADS=2`; one cell in flight |

The common degree cap is 512, above the degree-256 full dense membership input
at depth 8. Using degree 8 here would censor the dense arm at its input, before
a meaningful presentation comparison. External memory and process controls
remain necessary because F4 does not provide a strict native dense-memory
limit and cooperative deadlines are not hard real-time guarantees.

Record full process elapsed time, setup and matching phases, every failed
constructor attempt, every public query, extraction, signed replay, and rank
work. Native field/group/F4/root counters retain their named scopes. Never
sum partial counters as a complete cost model or omit a failed run from a
cost denominator. A killed cell keeps its flushed start records and external
termination receipt; absence of a terminal summary is explicit censoring.

The rho arm is the unchanged production A=1 reference. Its outcome is a
diagnostic public-target recovery check. It is not the tuned negation-aware
comparison needed for an end-to-end superiority claim.

## Admission, stopping, and reporting

Before submission, publish the exact source, lockfile, protocol, public input,
generated cell manifest, and execution command. Bind the executable/image to
that revision, upload the required source checkpoint, record one finite
original absolute campaign deadline, and obtain current account-budget and
reservation telemetry. The USD 5 lane ceiling does not establish available
headroom in shared daily/monthly budgets. Unknown headroom blocks launch.

Any mismatched fiber or point certificate, sign map, independent replay,
source/input hash, limit, or launch custody invalidates the affected sample
and stops further cells for diagnosis. Any change to caps, seeds, sample
selection, or analysis requires a prospective amendment. A cap stop is not a
mathematical negative and does not justify changing the next cell silently.

The report must retain all 55 planned outcomes, including cells never
submitted. Its initial flow is:

| State | IC cells | Rho cells | Total |
| --- | ---: | ---: | ---: |
| Planned | 54 | 1 | 55 |
| Attempted | 0 | 0 | 0 |
| Completed | 0 | 0 | 0 |
| Blocked before submission | 54 | 1 | 55 |

Use one `experiment.export/v1` record beside raw results when actual
executions exist, with hashes, source/image/job identities, resource and
budget receipts, and independent review. Do not manufacture successful run
rows for this initial blocked state. Subsequent flow updates must come from
retained receipts.

## Current protocol-validation limitation

The repository's protocol skill points to the canonical skill, schema, and
checker in `isogeny-labs/crypto-autoresearcher`. At observed revision
`808b08bcb0a53cb9ddfa7b6c4b4181d9e11d2445`, those three files are `VLT1`
binary envelopes in the existing checkout. Authenticated GitHub reads also
fail UTF-8 decoding. No decryption or access-control workaround was attempted.

The draft was structurally checked against the plaintext schema snapshot
already committed at
`research/pencil_large_prime_20261029/native_canonical_export/schemas/experiment-protocol.v1.json`
in crypto base `d080cca06199ced73d18acfa78c3256d350e6b77`. This is archived
schema validation only. It does not substitute for reading the current
canonical skill, running its semantic checker, or freezing the protocol.
Accordingly, the freeze hash, source commit, and freeze time remain null.

Once the canonical checkout is readable, validate this draft using its
`tools/experiment_protocol.py validate` command, address any drift, bind the
published source/input hashes and absolute deadline, and freeze it before
affected data. Current runtime access also lacks IsoLab submission
authentication, a source-checkpoint uploader, and live budget headroom. These
are execution blockers, not outcomes of the proposed mathematical experiment.
