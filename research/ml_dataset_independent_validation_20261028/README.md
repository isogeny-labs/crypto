# Independent native ML dataset consumer

`native/` implements the supported saved-dataset consumer in Rust. It checks
native producer artifacts, restores one published cell from S3 and exports an
allowlisted static view for solver-routing and cost models. Its independent
arithmetic does not link the producer's field or curve code. It does not
search for logarithms or generate campaign instances.

The earlier Python consumer and its receipts are retained as historical
artifacts. They are superseded for execution; see [NATIVE_MIGRATION.md](NATIVE_MIGRATION.md).
The producer remains separately owned in `codex/ml-ecdlp-dataset-20261028`.
This companion does not modify its working tree or launch compute workers.

Explicit factor-base policy datasets have a versioned companion contract:
see [POLICY_COMPATIBILITY.md](POLICY_COMPATIBILITY.md). The native consumer
accepts strict plan/v2 and factor-base/v2 bindings and emits its own static
routing view/v2. This is not conformance to the producer's v4 training export.

Sealed cells with the exact checked-phase policy also receive an independent
recorded-contract check. It reconstructs the attempt binding, verifies protocol
freeze hashes with number-preserving canonicalization, checks the finite native
syscall lifecycle, and recomputes each recorded counter projection. The new
`labels.checked_phase_contract` result remains restricted; public feature bytes
and legacy v1 labels are unchanged. Policy-only, unavailable, operation-error and
start-only prefixes remain explicit. Successful checks do not establish hardware
truth, complete execution, whole-worker counts or admitted training cost.

This component uses a conservative32MiB cumulative modeled-storage allowance,
in addition to strict file/array bounds. `consumer_resource_limit` is a consumer
refusal, not a producer outcome. Partial snapshots and their lineage are not
accepted as sealed cells. See the
[frozen contract](../ml_dataset_consumer_checked_phase_20261029/IMPLEMENTATION_FREEZE.md)
and [validation record](../ml_dataset_consumer_checked_phase_20261029/VALIDATION.md)
for exact scope, evidence and remaining interoperability limits.

## Native commands

```sh
cargo build --release --locked --manifest-path native/Cargo.toml
native/target/release/dataset-consumer fetch --receipt completion.json \
  --expected-sha256 TRUSTED_COMPLETION_SHA256 --out /new/restored-run
native/target/release/dataset-consumer audit --run /new/restored-run \
  --expected-run-sha256 TRUSTED_RUN_SHA256 --arithmetic \
  --native-verifier /trusted/native/crypto
native/target/release/dataset-consumer export --run /new/restored-run \
  --arithmetic --native-verifier /trusted/native/crypto --out /new/routing-view.jsonl
cargo test --release --locked --manifest-path native/Cargo.toml
```

`audit` checks artifact sizes and SHA-256 digests, canonical paths, unique JSON
keys, the frozen plan and mixed-radix cell index, run identity, shard membership,
public/config bindings, sampling provenance and outcome/certificate consistency.
Use an isolated restored directory. The consumer detects changed input bytes;
it is not a filesystem locking protocol against concurrent writers.

`export` requires producer replay unless `--allow-unreplayed-preview` is explicit.
It retains failed, unsupported, interrupted and censored outcomes, with null
features when public inputs are absent. It refuses to overwrite an output file.
Producer replay is separately identified and its binary digest is checked before
and after invocation. It does not count as an independently implemented solver.

## Independent positive certificates

`--arithmetic` checks scalar equations `k * G = target` and PDP sums with exactly
the configured number of summands. It validates field definitions, nonsingular
curve equations and point encodings, including exceptional addition cases.
PDP points must belong to the declared signed factor base; PKM's alternate
Kummer domain is checked separately, without adding unlisted points. Complete
twist metadata is checked against the underlying curve and twist parameter.

The code uses independent affine arithmetic over prime fields and binary
polynomial fields. Prime validation uses twelve Miller–Rabin bases within the
bounded word range; binary irreducibility uses Rabin's criterion. These checks
reuse established mathematics:
[SEC 1 v2, §§2.1–2.2](https://www.secg.org/sec1-v2.pdf),
[Sorenson–Webster, Theorem 1.1](https://arxiv.org/pdf/1509.00864), and
[Rabin's criterion in the FLINT author's documentation](https://fredrikj.net/fdoc/html/algorithms/polynomials/irreducibility.html).

The arithmetic module receives public inputs and the positive certificate,
never the private witness. The surrounding provenance audit separately reads
the witness's sampling metadata. A checked scalar equation establishes neither
the declared subgroup order nor how the scalar was found. No group cardinality,
solver independence, UNSAT, exact cost, exhaustive enumeration or fall-degree
claim is verified by this module.

Each certificate's digest and scoped results appear under
`labels.positive_certificate_arithmetic`; an empty certificate list has checked
count zero. The broad `mathematics_independently_verified` and
`labels.independently_verified` flags remain false. Positive checks do not turn
unsupported, timed-out or negative cells into verified mathematical results.

## Independent distinct-root certificates

The native consumer also checks recognized prime/root/m=2 and binary/root/m=2
certificates with `--arithmetic`. Its separate polynomial implementation computes
`monic gcd(f, X^q-X)` and compares it with the product of the declared distinct
linear factors. This checks the complete root set of each completed nonzero
polynomial without invoking the producer root finder. The zero polynomial has
an explicit all-field result; only the saved finite factor-base domain is used
for group-lift checks. Censored calls never establish absence of roots.

The checker binds curve, field, twist, target, configured budget, ordered signed
factor base, S3 coefficients, query order, finite-domain lift prefix and outcome.
Binary certificates additionally bind hashes of the complete public/base/config
objects. Prime v1 certificates have no equivalent full-object hash; their
mathematical fields are bound, but arbitrary annotations are outside that claim.
Results appear only in `labels.root_certificate_arithmetic`; static feature
schemas are unchanged. A positive root-set label is per supplied completed call,
and does not certify complete execution, every intermediate event identity,
instruction cost, factor-base construction, subgroup order, UNSAT or fall degree.
Historical root artifacts remain readable with explicit unverified status.
See [ROOT_CERTIFICATE_DESIGN.md](ROOT_CERTIFICATE_DESIGN.md) for the bounded
prime/binary contracts and primary mathematical sources, and
[ROOT_VALIDATION.md](ROOT_VALIDATION.md) for the 51-cell sealed acceptance,
retained failures, source reviews and verified S3 readback.

## Feature and measurement contract

Only `features` is suitable as a model input. It contains typed public curve and
field descriptions, generator/target, factor-base points/actions/orbits, solver
and work limits. Seeds, complete manifests/configs, witnesses, generation traces,
solver traces and terminal answers are excluded. IDs, labels and split metadata
remain separate. This is the narrow `ml-dataset.static-routing-view/v1` view,
not full conformance to the broader `crypto-ml-v1` training design.

Phase telemetry is label-only. Preserve raw counts, scaled estimates, counter
availability, enabled/running times, unfinished phases, exclusions and source
hashes. Integer scaling and status consistency are checked independently;
missing readings remain null. Exact and multiplexed estimated counts remain
separate. These checks do not calibrate a PMU or establish whole-worker cost:
`whole_worker_instruction_count` stays null and no phase totals are invented.

Whole field sizes are assigned deterministically to 80/10/10 hash buckets.
Every generator, key, twist and family at a fixed n stays together. Small panels
can have uneven or empty partitions. Linked isogeny/subfield components across
sizes are not identified, so this split alone cannot establish cross-size
learning claims. Adaptive policies still need a separate prefix-only trace
view with decision-time features.

## S3 restoration and durability

`fetch` requires a completion receipt and a SHA-256 anchor from a trusted source.
It permits only `setec-astronomy-artifacts`, checks canonical content-addressed
keys and declared download limits (20 GiB, 100,000 references), then checks HEAD
size, ETag-bound GET and every downloaded hash. The restored artifact set must
exactly match the manifest. Failed restores remain visibly incomplete; existing
destinations are never overwritten. AWS CLI is used only for S3 transport.

Byte integrity does not imply mathematical validity or retention guarantees.
This tool does not configure versioning, Object Lock or lifecycle. A controller
software-fixture test does not establish EKS worker permissions. Production
publication must pass a fresh worker-role write/read/hash preflight and publish
completion receipts only after every referenced object is durable.

## Validation and open work

Native tests cover fixed prime/binary field and group vectors, tampered public
certificates, exceptional points, fields, twists and PKM domains; provenance,
PMU labels and feature isolation; duplicate keys, path traversal, symlinks and
changing bytes; and bounded S3/producer command handling. They are software
fixtures, not campaign draws. Historical 19-test and 25-test Python receipts
remain unchanged, with separate evidence for the Rust replacement.

The deployed-worker native-producer-to-S3-to-independent-consumer replay remains
outstanding. The separate controller archive roundtrip in
[POLICY_COMPATIBILITY.md](POLICY_COMPATIBILITY.md) checks retained software
fixtures. As of the 2026-10-28 06:47 UTC inspection, the dated canonical
catalog had no completed cells. Pilot 002 failed on S3 HEAD permission before
publication. Broad solver/curve/twist coverage, instruction accounting, factor
base controls, large-scale generation and scientific label acceptance remain
unfinished. This consumer does not resolve those producer requirements.

The campaign budget is **$100–$200 total through tomorrow**, with a hard planning
ceiling of $200. PR #2751 records a conservative $188.3888 reservation, leaving
$11.6112 unallocated. Those are planning reservations, not verified billing or
account-wide enforcement. No extra retry or compute worker is authorized by
this companion's implementation.

The completed native validation is recorded in
[evidence/native-20261028/validation.json](evidence/native-20261028/validation.json):
79 release tests, strict Clippy and formatting passed. The live S3 software
fixture restored 13 artifact references (11 unique objects, 4,153 declared
bytes), verified all hashes, and independently checked one positive certificate.
The receipt, source hashes and raw test outputs are preserved beside the report.
This establishes the controller storage path and software checks only.

The PR #2757 delivery follow-up adds a regression for missing instruction
enabled time alongside a complete cycle tuple. The native validator rejects
that incomplete instruction tuple before accepting any exact label. After
merging main `189ac727fcc4475b42755d4f1c15f97a4a102070`, 80 native release
tests (73 library and 7 integration), strict Clippy, formatting and the release
build passed. The new source hashes and logs are separate in
`evidence/review-followup-20261028/`; the earlier receipts remain unchanged.
These local checks do not establish current-head remote CI or review approval.
