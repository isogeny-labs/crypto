# Independent source review of checked-phase native evidence

Reviewed 2026-10-29; source-identity read at 04:19 UTC. This is a bounded source review, not an
executed test result or a hardware/execution-admission decision.

No actionable lifecycle, projection, or strict-schema mismatch was found in the
two owner-supplied snapshots. The owner may continue its deterministic validation
and integration gates. This endorsement applies to the exact bytes below; it does
not certify a moving worktree, uninspected parent integration, all malformed-input
behavior, or whole-component storage bounds.

## Ownership and immutable review input

The consumer owner supplied stable files under Conductor T-780. They were copied
before review; neither repository source nor task ownership was changed. The
parent authorized writes only to this review directory. This reviewer did not
author either reviewed file and ran no compiler, Cargo command, native producer,
PMU call, generator, cloud action, or PR mutation.

| File | SHA256 |
| --- | --- |
| `source/consumer-native.rs` | `bad3d65a9b0744a3cbd97038e2c99c8fe79dd2d2b27289cdea19d78b3098a71d` |
| `source/consumer-schema.rs` | `b345a8415e427948e8a0caa3d8a9c0cac582bfbc16bf8e6498780679da53db31` |
| `source/producer-native-27679c4e.rs` | `8fba16ead93036cf26c7591d7535283932bab8b8ec4429fb67811eb41ef273f9` |
| `source/producer-types-27679c4e.rs` | `4ed8ae39c13f540b4257d71cde5bc3b6130800f95ccf8e1d848b7f5615a56247` |
| `source/producer-checked-27679c4e.rs` | `75084db97cd476e49ced93b752c10f9727dd885ed5371dd19718b1c7335b00bc` |
| `source/producer-collector-27679c4e.rs` | `69647f7955497e65becf039bed95a304dcd040b9e9a074a118690144f898fef4` |

Consumer originals are `research/ml_dataset_independent_validation_20261028/native/src/phase/{native,schema}.rs`
in `/home/ec2-user/crypto-ml-consumer-checked-phase-20261029`. They were supplied
before an owner compile/format pass and are identified by content hash rather than
claimed committed-source identity. The worktree HEAD observed after the review was
`9f4bb40b72036abf82fd557652fcf638752e696f`; this is not a statement that these two
snapshots are committed in that revision.

Producer input is immutable commit
`27679c4e59bc38e8c18e34dbc9687f9a2d5eb317`. Its four reviewed source paths have an
empty diff against frozen inventory pin
`9f1d54818cd791a25d9beeb74840fd0b94bf2de9`:

- `src/ml_dataset/telemetry/checked/native.rs`
- `src/ml_dataset/telemetry/checked/types.rs`
- `src/ml_dataset/telemetry/checked/mod.rs`
- `src/cryptanalysis/ecbench/isolation/checked.rs`

The consumer DESIGN, ACCEPTANCE, IMPLEMENTATION_FREEZE, and WIRE_INVENTORY were
read as the contract. The consumer's dedicated number-preserving protocol parser
was authored by this reviewer and is intentionally excluded from this independent
source endorsement.

## Source findings

No correction is requested from this snapshot review. The following source paths
support that conclusion and define its useful limits. Line references point to
the copied files, so later formatting cannot invalidate the evidence.

| Obligation | Consumer source | Independent comparison and result |
| --- | --- | --- |
| Fixed attrs and recorded byte order | `consumer-native.rs:21-35,130-134` | Every requested byte is reconstructed for the recorded little/big order; event config,112-byte attr,read-format7,leader/member97/96,pid0,cpu-1,group FD and flag8 are checked. Actual collector initialization at `producer-collector-27679c4e.rs:544-575` and checked request replay at `producer-native-27679c4e.rs:39-79` agree. |
| Unsupported backend | `consumer-native.rs:133-157` | Only little-endian Linux x86_64/AArch64 enters native preparation. Unsupported records require absent owner/current TIDs. No calls are consumed; exact remaining-state/lifecycle comparison at98-104 rejects invented calls or IDs. Big-endian AArch64 unsupported evidence remains admissible; impossible big-endian x86_64 is rejected. |
| Preparation/identity/placeholder | `consumer-native.rs:62-75,142-153` | Successful ID is a zero control marker with the actual u64 ID stored separately. ID zero is permitted; duplicate IDs within the open group trigger exactly the documented identity error. A leader preparation failure leaves a never-attempted cycles placeholder with group_fd-1/leader attrs. Actual collector604-632 matches. No new positivity rule is imposed on kernel IDs. |
| FD ownership and fallback | `consumer-native.rs:149-150,179-185` | Simultaneously open leader/member descriptors cannot alias. Cleanup is consumed before any fallback. Fresh per-attempt state permits reuse of a previously closed descriptor number. Only loss of the optional member after identified leader preparation and successful/no-needed cleanup requires a single fallback. Actual collector521-538,652-671 and producer replay366-380 agree. |
| Context/start/control sequence | `consumer-native.rs:109-115,158-181` | Both context records are derived from PID/TID inputs. Start failure suppresses reset/enable; finish failure suppresses disable/read and invalidates retained start. Reset failure short-circuits enable but still permits diagnostic finish read after the matching start call. Returned start success and retained attempt success remain distinct. Actual collector694-755,758-790 and checked replay319-350 agree. |
| Read/status/cleanup | `consumer-native.rs:77-104` | Read failure, any non32 read, identity mismatch, invalid tuple and unavailable statuses are preserved. Control-failed records can retain a valid raw tuple/assessment without a promoted observation. Every recorded derived field and consumed-call count must agree, so extra/reordered calls, invented raw values or missing cleanup fail. |
| Arithmetic and observations | `consumer-native.rs:5-18,188-205` | Ordered projection recomputes exact u128 floor scaling and checked u64 conversion. Overflow and zero running remain unavailable; zero raw with positive equal times remains a valid scoped zero. Clean lifecycle is a separate property from valid tuple status. One event's valid tuple cannot synthesize another event's missing count. Producer projection391-435 agrees. |
| Fixed claims | `consumer-native.rs:119-124` | Collector completeness, whole-worker/descendant coverage, hardware truth, platform admission and comparison-clock claims must remain false. No count aggregation or cost eligibility is produced here. |
| Strict owned wire inventory | `consumer-schema.rs:6-132` | All object structs deny unknown fields; nullable fields use explicit required deserialization; Phase is closed; CallResult is the external Ok/Err enum. Native/observation string-valued state fields are closed by exact semantic reconstruction/comparison, not accepted as arbitrary statuses. Inventory fields/types agree with pinned producer types. |
| Bounded list wiring | `consumer-schema.rs:54,75-77,86,92,117` | Observations/contexts/attempts/events/attrs/calls are routed through dedicated bounded deserializers. This review checked the wiring only; actual refusal-before-growth behavior belongs to the owner's io/integration tests and is not certified here. |

The native checker returns an independently reconstructed `(start_returned,
observations)` tuple. The parent caller must compare both fields with the recorded
metrics and perform the remaining phase binding/order/error-summary checks. The
producer comparison at `producer-checked-27679c4e.rs:235-274` illustrates that
obligation. Consumer parent integration was outside this review, so this packet
does not claim that obligation has passed.

## Deterministic cases to confirm in the owner gate

These are validation recommendations against the frozen acceptance matrix, not
claims that an uninspected test suite lacks them. All should use supplied/mock
records, with semantic tamper fixtures rehashed before rejection:

1. Valid group, valid zero raw, exact multiplex floor, overflow, zero-running,
   invalid times, read-ID mismatch,31/33-byte read, and read failure; keep
   instructions/cycles expectations independent.
2. Leader open/ID failure with the unattempted cycles placeholder; optional member
   open/ID/duplicate-ID failure followed by required fallback; successful fallback
   reusing a closed FD; cleanup failure suppressing fallback; reject omitted,
   gratuitous, third or post-work fallback and simultaneous FD alias.
3. Start context failure, finish context failure after a successful start,
   reset/enable/disable failure, and diagnostic tuple preserved under a control
   failure. Confirm that original start return true can coexist with final
   attempt.start_succeeded false after a finish context failure.
4. Supported LE x86_64 and AArch64, unsupported BE AArch64 with no TIDs/calls,
   and reject unsupported evidence with invented TIDs/native lifecycle. Include
   a valid zero event ID and duplicate-zero group failure.
5. Every required-null omission, unknown/duplicate/escaped-duplicate nested key,
   malformed or multikey CallResult, integer-as-float/type/overflow error, unknown
   status/operation, repeated_start, and first excess list item. Ensure the full
   audit path compares both derived native outputs against the metrics.

The source preserves unavailable/failure evidence rather than treating it as a
measured zero or silently dropping it. The general native collector can expose
NotStarted/RepeatedStart outside the fixed wrapper; the checked phase route
invokes start exactly once, so accepting such a lifecycle is not required here.
This is a fixed phase-policy validator, not a general collector-record importer.

## Remaining boundaries

Compilation, Clippy, deterministic tests, actual standalone audit/export dispatch,
retained-fixture interoperability, full32MiB admission accounting, S3 durability,
CI and delivery remain owner gates. No result in this packet proves PMU truth,
admitted execution cost, complete solver execution, or independent partial-snapshot
lineage. Nothing changes the USD200 total maximum, original deadline or cloud/job
ownership. Existing private provenance/counter/outcome labels remain restricted;
this review does not authorize public-feature changes.

An initial lookup for `telemetry/checked/verify.rs` returned path-not-found; the
actual immutable checked validator is `native.rs` plus `mod.rs`. No alternative
source or result was substituted. Repository snapshots and hashes listed above
were subsequently read successfully.
