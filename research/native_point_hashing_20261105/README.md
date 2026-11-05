# Native point-table study instrument: development qualification

The [batch example](../../examples/native_point_hash_study.rs) and its
[pilot module](../../examples/native_point_hash_study/pilot.rs) supply the six
arms and orchestration of the first [planned study](../../backlog/XP-20261105-native-point-hashing/README.md).
The requested studies have **not run**. This directory contains development
qualification only. No scientific batch, cloud build or measurement job was
submitted. The original frozen 1,728-attempt panel is unchanged.

Current pilot software, fixed input recipe and development qualification are
described in [pilot-v1/PROTOCOL.md](pilot-v1/PROTOCOL.md) and
[pilot-v1/qualification.json](pilot-v1/qualification.json). The original
batch-only `qualification.json`, `source.sha256` and `development.tar.xz` below
remain historical evidence for commit `8c90746a23b2943a94f75558033958f31f91199e`;
check out that commit when replaying those older source hashes.

## Implemented scope

The input supplies a certified cofactor-one small prime curve, two public affine
hash bases, up to 256 public query points, an arm and explicit work limits.
No discrete-log labels enter the oracle. The example accepts field moduli up
to 65,535, subgroup orders up to 131,071 and selector thresholds up to 128.
These bounds cover the proposed 12/16-bit roster; they do not cover Certicom.
This is a two-point component instrument, not a full ECDLP solver.

| Arm | Implementation |
|---|---|
| `current`, `current_aa` | Original native `finite::prime_decompose`, with its default per-query cache and final-slot derivation |
| `prepared` | Reuse domain inverses and decoded selector points; retain arithmetic membership |
| `ordered` | Prepared points plus a BTreeSet of full canonical keys |
| `hashed` | Same full keys in a deterministically hashed HashSet |
| `fingerprint` | 64-bit fingerprint buckets, retaining full keys and exact equality |

All arms construct and retain the original ordered slot/column indexes. Prepared
selector vectors are reconstructed from those complete enumerations without
decoding the domain again. They preserve rejected selectors, selector order
and slot identity. The ordered/hash comparison uses the same full key type and
per-query key construction. Their additional lookup indexes are both charged
to preparation; matrix indexes remain present in both arms. No randomized table
iteration chooses witnesses.

Every witness also passes the original arithmetic factor-base predicate and
sum replay, including in indexed arms. Exhausted limits return `unknown` and
stop subsequent queries; they never become absence. Returned absence means
that the selected domains were completely exhausted.

## Replay scope and output boundaries

`native_point_hash_study run INPUT OUTPUT` writes a new batch record, preserving
an existing output. The output includes actual base cardinalities, preparation,
cumulative query prefixes 1/4/16/64/256, final diagnostics and all outcomes.
Preparation includes curve/base certification, complete matrix indexes and the
arm's reusable preparation. Native search and arithmetic witness replay are
inside the interval. JSON parsing/input binding and word replay are outside.
The final interval ends after the query loop and before independent word replay.

`native_point_hash_study check INPUT OUTPUT` checks input binding, complete base
cardinalities and the attempted witness/absence stream using the repository's
existing `PrimeCurve` word arithmetic. It counts the whole small curve, checks
the declared prime order/generator, enumerates the group from that generator,
rebuilds both hash bases, and derives the expected first witness in selector
order. It does not consult the prepared native lists. The wrapper is written
by this producer; reusing a separate arithmetic implementation is not an
independent full audit or a signed PCC verdict.

The checker explicitly does **not** authenticate work counters, clocks, L2
admission or PCC standing. Its JSON summary names that scope. Batch records
mark timing as unadmitted and pricing as incomplete. `counts` mode creates no
batch clocks. `clocks` mode exists for a future admitted native worker; it was
not exercised here. The caller must retain the separate word-check cost and
harness costs before any total-cost interpretation. A zero exit status is not
measurement admission.

## Development checks and source identity

The final producer invocation reports **8 passed controls**, strict release
clippy and changed-file format checks. Their exact commands, source hashes,
toolchain and raw-log hashes are in [qualification.json](qualification.json).
The six lossless raw logs are in [development.tar.xz](development.tar.xz), under
`development/`. To inspect them, extract that archive into a fresh directory;
the qualification's `log_member` and SHA-256 fields identify each original body.
The archive itself also has a pinned SHA-256. Packaging preceded the first
commit and preserves Cargo's exact final blank lines.
No native library or existing root library source was modified; the checks
target the new example and compile it against those libraries. The earlier
broad-library qualification remains the historical mixed outcome in PR #2984;
this directory makes no new whole-library pass claim. [pcc:unverified]

Controls use the fixed public `p=17, a=2, b=2, r=19, G=(5,1)` curve:

- All six arms give the same complete first-witness/absence stream over all
  19 public group points and pass word-arithmetic answer replay.
- Constant hashes force both full-key and fingerprint collisions without
  changing answers. Full keys distinguish signs and infinity.
- An empty base returns complete negatives; a search work limit remains
  unknown and leaves later queries unattempted.
- The replay rejects a corrupted absence and an input/base substitution.
- Both unequal slot orders and a tied threshold preserve slot/witness order.
- Input fields/coordinates, bounded reads and existing outputs are checked.

These are producer development observations with no signed PCC standing.
[pcc:unverified]

The initial six-control test log and the initial strict-clippy failure are
retained. Clippy rejected a direct modular-zero idiom; the final source uses
the existing `num_integer::Integer` predicate, with no lint suppression. The
initial source hash was not captured before that edit and the added controls;
do not treat the final manifest as an identity for the initial receipts.

## Remaining before the requested studies

The six-arm single-batch instrument now has a compact fixed 32-context recipe,
2,368-attempt schedule (the original 1,920 timed/warmup batches plus explicit
controls and counted passes), fresh-process execution, durable failure ledger
and paired diagnostic analysis. The [canonical draft](pilot-v1/protocol.json)
passes structural/cross-reference validation but remains unfrozen. The real CLI
development control completed 76 counts-only F17 children; no 12/16-bit solver
batch was run. Current receipts are in [pilot-v1](pilot-v1/qualification.json).
These are unregistered producer observations. [pcc:unverified]

Full field/hash/allocation inventory, external references, calibrated costs,
masking/canonical export and native host/resource admission remain pending.
The actual canonical protocol must be completed and frozen before data.
A digest-pinned image must include this example and earn native
L2/A/A admission; the generic deployed ECBench fixture cannot substitute.

The canonical-key/projective study, natural-reuse cache study, label-safe
deduplication study, distributed-rho study and fresh-target full native ECBench
confirmation remain pending. A batch test is not any of those studies.

The [retained launch-admission snapshot](../../backlog/XP-20261105-native-point-hashing/launch-readiness-20261105.json)
records absent AWS credentials, unknown budget headroom and the unavailable
main-branch checkpoint control path at its inspected revision. The current
runtime still exposes no cloud identity. An existing authenticated runner,
current account/lane headroom, exact-source S3 write/readback and native
image/execution admission are prerequisites. Do not use chat credentials,
increase quotas/trust, bypass the protected-ledger decision, run a local timing
substitute or modify the old frozen evidence. No scientific job has been queued.
