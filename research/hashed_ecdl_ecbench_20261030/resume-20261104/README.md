# Native online-accounting correction — 2026-11-04

The native observer now starts the online interval before target subgroup
validation. Initial target-independent curve/configuration/base/index preparation
stays outside that interval; later base retries stay inside it. Source/selector/
width reductions defer the target multiplication until initial preparation is
ready. Partial collection validates its reusable base policies first and enters
the same target-validation boundary. Identity handling and every subgroup,
witness, certificate and scalar check remain. This is a producer implementation
assertion, PCC-20261104-9bb71e, standing unverified. [pcc:unverified]

The adapter retains the actual first online phase in record detail, along with
the source digests, exclusive clocks, counted group work and incomplete arithmetic
inventory. Input normalization and harness bookkeeping retain their cold/harness
scope. This correction supplies a native validation/search interval; it does not
make an unpriced inventory complete. [pcc:unverified]

## Development evidence

| Control | Producer observation | Scope |
| --- | --- | --- |
| Original ordering + same first-query-failure control | Fails: one target scalar multiplication remained in setup instead of zero | Original source at e84d46bb; expected regression |
| Corrected native package in crypto | 121 tests report success | Local native development suite |
| Corrected native package in cryptanalysis | 98 tests report success | Paired library development suite |
| Current finite-extension controls | 12 cases include boundary/certificate/RNG/work checks | Local controls; no scientific timings |
| Native format, strict all-target clippy, Rust 1.78 all-targets | Commands report success in both packages | Exact source and workspace manifests retained |

These observations are producer receipts, not independent signed verdicts.
The mirror statement is PCC-20261104-0f6851, standing unverified. Original and
corrected command logs are retained losslessly beside this report. [pcc:unverified]

The dedicated regression stops on the first oracle call and checks that setup
contains no target scalar call, target_validation contains that call, and the
entire cold clock equals reusable setup/base time plus the online clock. The
canonical online phases still sum exactly to the interval. Existing full
certificate/RNG/meter observation-off/on controls are retained. This is not an
old/new runtime or speedup comparison. Validation now occurs after preparation,
so a preparation limit can be returned before an invalid target is reached;
unknown remains distinct from invalidity and nonexistence. [pcc:unverified]

![Cold and online accounting boundaries](online-boundary.svg)

## Current-main registry integration

The combined-code library run exposed a separate driver integration omission:
`ic.hashed_native_v1` was absent from the exhaustive optimization policy registry.
The original binary reproduces the policy/registry mismatch. The adapter now has
an explicit diagnostic native-grid policy; backend/algorithm choices are knobs,
while trial/work/vector/bit/width/slice/attempt ceilings are resource envelopes.
Other methods keep their previous role classification. A focused regression
checks these roles, and the corrected production code passes strict clippy.
The first incomplete-call-site compile failure is retained with its retry.
These are producer development observations. [pcc:unverified]

PCC-20261104-d06cd6 has unverified standing. The current metadata-only plan
matches all 32 published workload IDs in order and every field of the original
frozen summary: curve identities, generator coordinates, subgroup orders, all nine
resolved arm identities/parameters, 32 workloads and 1,728 executions. It calls
no solver. The original spec and all prior frozen evidence stay unchanged.
`current-plan-summary.json` preserves the comparison input. [pcc:unverified]

## Preserved study and execution prerequisites

The four registered prime sizes, eight public targets, nine arms, seeds,
controls, one warmup and five measured rounds remain the requested 1,728-attempt
plan. Frozen v1/v2 are preserved byte-for-byte. A new pre-data amendment binds
current source and the corrected boundary, plus the current publication,
benchmark-record and finite cloud-cost rules. No scientific measurements are
substituted with local unit timings. [pcc:unverified]

Before any cloud build, obtain current account and lane headroom, including
charged/reserved costs, and enforce the original resource/deadline/retry bounds.
Before measurement, require actual fresh current-source S3 upload/readback and
an authenticated exact native image. Missing headroom stops launch; the user has
been asked for an existing connection/runner name. No IAM, pool, budget or trust
expansion is included. The actual study continues after those prerequisites.

Sources reviewed: existing native reduction/collector code and the pinned
[Bochum reduction](https://github.com/bochum-crypto/subexponential-ecdl/tree/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f),
[review request](https://github.com/isogeny-labs/crypto/pull/2984#discussion_r4426683962),
ECBench's exclusive online accounting contract, main's
[cloud controls](../../../taskq/deploy/eks-karpenter/COST_CONTROLS.md),
[sabotage-resistance contract](../../../docs/SABOTAGE_RESISTANCE.md) and
[benchmark-record contract](../../../docs/benchmark-records.md).

## Custody and sparse-checkout incidents

The published refresh dfa13276 preserves its two parent histories. Current main
2d746d05 merged cleanly as e84d46bb; its tree matches the dry-run tree. There was
no conflict-side fallback. The first sparse PCC append lacked the hidden tracked
main ledger; that uncommitted attempt was preserved separately, the exact main
prefix restored, and canonical appends then passed prefix checks. Missing old
benchmark metadata/raw/report files were restored from exact Git blobs, after
which the unmodified 32-record benchmark gate reports success. No historical
ledger/benchmark record was changed. [pcc:unverified]

Root compile attempts and any subsequent fixture failures are retained with
source identities. They are development checks, not scientific executions.
An archive-based original-source regression and a local reverse-patch copy both
reproduced the same omitted-scalar failure; the local copy's tracked blobs were
compared with the original Git object identities before adding the control.
The failed original ordering is retained, not counted as a passing test. [pcc:unverified]
