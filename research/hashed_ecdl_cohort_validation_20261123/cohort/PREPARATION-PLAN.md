# Prospective expanded-cohort preparation, before code or data

This plan is prospective. It defines source and harness qualification for a
new cohort; no new target generation or solver run has occurred. Classical,
native Rust/C, local only; S3 remains excluded. [pcc:unverified]

## Question and fixed intended design

On six already admitted curve domains, compare the recorded staged-numerator
parent with its reverse-pass candidate using36additional public targets.
The intended target/oracle seeds are303,404,505,606,707,808, each on cases0..5.
These form a deterministic bounded fixture census, not independent uniform
samples. They are disjoint from old seeds101/202; actual coordinate collisions
must be checked before acceptance rather than assumed absent.

The intended new main schedule has432cells:3logical arms,36targets,4rounds.
Round0 is a warmup; rounds1..3 are measured. Armsa/c use one identical parent
binary; armb uses the candidate. Every measured arm occupies each of three
positions once per target. No old timing/solver data is pooled. No statistical
significance, ranked timing, asymptotic exponent or population extrapolation
is a decision criterion. Exact certificates/counters and descriptive per-target
clock ranges/medians are the intended observations.

Preserve m8,tau=2ceil(r^(1/8))+2,trials16,work20M,columns16384,batch64,
the original six curve/generator tuples, the target x-search/even-y encoding,
and both solver libraries byte-for-byte. Only a common worker input whitelist
and an explicitly separated target-only mode may change before solver calls.
Use a separate source copy; never edit the old frozen source or data.

## Literature and existing contracts checked

Read the original scale.rs, CPU/tensor Cargo manifests, panel-v2 scheduling,
raw-row/status/analyzer controls, point-audit-v2, lossless export adapter and
future-only protocol.next-draft. The old adapters hard-code12targets,
360cells,9measured repeats,seeds101/202 and old protocol/source hashes.
The next draft still describes those12targets; its harness_drawn mapping is
unadopted. Do not substitute it for the new protocol or retrospectively fix
the324original seed-table findings. [pcc:unverified]

The source/primary mathematical review in the parent README applies to the
unchanged curve admission and point arithmetic. This is harness expansion,
not a new mechanism or novelty search. KB MCP remains absent and Conductor
is absent from PATH; retain the earlier wrapper127 receipts and primary-source
fallback, with no reservation or novelty clearance asserted. [pcc:unverified]

## Qualification stages and exact gates

1. Snapshot new parent/candidate source copies and prove their solver/source
   differences equal the recorded sole CPUlib difference. Add the identical
   new scale worker to both; preserve old workers. Target-only mode must return
   before oracle construction or discrete-log recovery and emit public headers.
2. Extend collector/analyzer/auditor together to the six new seeds and4rounds.
   Derive every count from the fixed seed list. Preserve raw native armu,
   replicate0,warmupfalse separately from logical identifiers. Parameterize
   process CPU pin from the actual allowed host set; do not silently assume
   CPU4. Graph height must include every per-target summary row.
3. Compile with pinned Rust1.99.1/C11-O2, locked offline dependencies and one
   build worker. Copy qualifiers read-only before executing them. Run relevant
   release library tests and collector/auditor tests on copied binaries.
4. Check target-only outputs for all48case/seed headers (old12 plus new36):
   parent=candidate=an independent OpenSSL-derived x-search/even-y target;
   old12 must also equal the archived public headers. Require every new target
   nonidentity/on-curve and full prime-order admission, and no within-domain
   coordinate duplicate among all8seeds. No recovered scalar is an input.
5. Exercise schedule uniqueness/Latin positions, missing/incomplete flow,
   wrong native identity, unknown seed, duplicate identity, invalid clock,
   inconsistent exit status, certificate/row disagreement and point-binding
   controls. Later solver preflight must retain wrong-scalar and consistent
   target-substitution rejections. Preserve every failure; fresh paths only.
6. Replace the old export's hard-coded counts/source/protocol bindings
   prospectively. Positive synthetic export plus seed/identity substitutions
   must pass/refuse under the actual native schema/seed-table checker before
   adopting harness_drawn or freezing the new protocol. No fabricated attempts.
7. Only after qualification: finalize/publish the protocol and source identities,
   capture a local checkpoint, then separately authorize execution through the
   existing standing local user instruction. No new permission request is
   needed; this plan itself is not a claim that the final protocol is frozen.

The intended main resource limits remain1500s driver,120s finalization and
120s point-audit,120s worker/60s replay, unchanged15032385536-byte sampled
whole-cgroup stop and one solver worker. Preserve the distinction between the
1800s planning envelope and actual independent phase watchdogs. Stop on the
first failed call, retain all planned/unattempted cells, and do not retry or
resume the dataset. A timeout is an inconclusive run, not a mathematical result.

Preparation phases each have600s wall cap and the same physical stop, one
compile/test worker. Target-only point checks each have60s. Record distinct
build, pre-call and post-call manifests; never replace a pre-call manifest.
Check disk headroom before builds, retain failures and frozen evidence, and
avoid changing release/codegen settings to conceal resource problems.

## Acceptance and interpretation

Preparation passes only when every actual gate above is met and exact source,
binary, protocol and input identities are recorded. Components can be recorded
as passed while the overall preparation remains incomplete. A new main result
requires432complete cells, exact per-target certificate and structural-counter
equality across arms/rounds,432Native replays,432OpenSSL positives and864
intended negative point-control rejections. Missing phases/costs stay unknown.
Compare each point with its matched parent baseline; any clock observation
remains descriptive at shared-host L0. No performance leaderboard promotion
follows from these preparation checks.
