# Partial consumer implementation freeze

2026-10-29, before runtime implementation. T-783 owns only the standalone
native consumer and this research directory. Base is
`ee936f2f498bba8a0a4e5df69c218034d4e8a009`; the parent approved the complete
design manifest `8fbc2156c93a720f2378ccdc047368399cf0915ac719a3aa113f3529624c1558`.
The copied packet and `PARENT_FREEZE.json` govern semantics and acceptance.
This document fixes concrete module interfaces and conservative allocation
choices. It adds no producer, remote-run, cloud-job, spending or deadline authority.

## Files and ownership

All runtime paths below are relative to
`research/ml_dataset_independent_validation_20261028/native/`.

- Owner: `src/partial.rs`, `src/partial/{budget,io,chain,restore,report}.rs`,
  partial tests/fixtures, `src/main.rs`, `src/lib.rs`, and the narrow phase core
  extraction in `src/phase.rs`, `src/phase/{binding,budget,io,schema}.rs`.
- Proposed delegated writer, only after parent activation:
  `src/partial/schema.rs` and `src/partial/capture.rs`, with in-module tests.
  No other files, Cargo runs, staging, commits, ownership mutations or producer
  activity. Owner integrates and builds; parent independently reviews these
  authored modules. Root/source/runtime dependencies are unchanged.
- Existing `src/transport.rs` may receive only internal visibility/helper
  extraction necessary to reuse the existing process guard and canonical S3
  prefix validator. Its sealed fetch behavior remains unchanged.
- Session entries remain in this owned directory until the shared daily-log
  lease is available. No cross-chat contact is authorized.

## Concrete shared types and interfaces

`partial::schema` defines all wire records from `WIRE_AND_MODULES.md`, with
`Clone, Debug, PartialEq, Eq, Serialize, Deserialize` where applicable and
closed serde envelopes. Types use the concise inventory names: `Artifact`,
`ObjectRef`, `Binding`, `Anchor`, `Catalog`, `Receipt`, `SnapshotObject`,
`Manifest`, `Observation`, `Coverage`, `Delta`, `Head`, `Cursor`, `Segment`,
`Stop`, and `EventRecord`. `EventRecord.event` alone is `serde_json::Value`;
it is built only after the owner's duplicate/depth/allocation admission pass.
`Binding` may re-export the existing consumer-owned phase binding type after
its visibility is widened; serialized field order stays exactly frozen.
All fields are `pub(crate)` or public fields on crate-private records.

Schema helpers are `validate_artifact(&Artifact)`,
`validate_binding(&Binding)`, `validate_ref(&ObjectRef, cap: u64, cas: bool)`,
`validate_path(&str)`, `validate_remote(&str)`, and
`binding_digest(&Binding, &mut Budget) -> Result<String>`. No path helper
opens a file. Remote grammar accepts canonical S3 or a nonempty original
`file://` location for local-mirror identity; fetch separately requires S3.
The schema module does not allocate a collection before caller admission.

Source-backed clarification, 2026-10-29 before acceptance: `file://` requires
a nonempty absolute remainder beginning with `/`, matching pinned producer
`src/store/backend.rs:128–135`. Local mirror mode retains that original
identity but never follows the declared location. This narrows the shorthand
"nonempty" above to the producer's actual accepted grammar; no vector changes.
The same source-backed canonical identity rule rejects trailing-slash S3
aliases: the bare canonical bucket is valid, but neither `s3://bucket/` nor
`s3://bucket/prefix/` equals the producer backend's displayed identity. Inputs
are rejected rather than normalized. This is PC-02 from parent source review.

The parent ledger API in `partial::budget` is:

```rust
pub(crate) struct Budget { /* private checked counters */ }
impl Budget {
    pub(crate) fn charge(&mut self, bytes: usize, what: &str) -> anyhow::Result<()>;
    pub(crate) fn read(&mut self, bytes: u64) -> anyhow::Result<()>;
    pub(crate) fn written(&mut self, bytes: u64) -> anyhow::Result<()>;
    pub(crate) fn admit_object(&mut self, bytes: u64) -> anyhow::Result<()>;
}
```

`charge` reserves cumulative modeled allocation volume before allocation;
`read`/`written` count actual I/O and do not admit a logical reference twice.
`admit_object` adds exactly one declared logical reference and its bytes before
I/O. Metadata and artifact occurrences are both admitted. All counter overflow
fails explicitly. Constants expose the frozen limits; there is no CLI increase.

`partial::io::typed<T: DeserializeOwned>(raw: &[u8], &mut Budget) -> Result<T>`
performs duplicate/depth/node/string admission before the owned typed parse.
Its caller has already reserved raw bytes and decoder scratch and bound the
raw bytes to their admitted identity. `value` provides the same bounded parse
for arbitrary event bodies. These functions do not reset the parent budget.

The delegated capture module owns:

```rust
pub(crate) struct CaptureState { pub cursor: Cursor, pub stop: Option<Stop> }
pub(crate) trait CaptureReader {
    fn open(&mut self, artifact: &Artifact) -> anyhow::Result<Box<dyn std::io::Read>>;
}
pub(crate) fn genesis(binding_sha256: &str) -> Cursor;
pub(crate) fn admit_delta(delta: &Delta, prior: Option<&CaptureState>,
    binding: &Binding, budget: &mut Budget) -> anyhow::Result<CaptureState>;
pub(crate) fn replay(delta: &Delta, prior: Option<&CaptureState>, binding: &Binding,
    reader: &mut dyn CaptureReader, budget: &mut Budget) -> anyhow::Result<CaptureState>;
```

`admit_delta` validates every structural, path, count, range, head/cursor,
membership and sticky-stop condition possible without artifact reads. Its
returned state is only declared state for forward preflight, never byte proof.
`replay` repeats those checks and independently streams every segment/payload,
rehashes consumed bytes with exact-size-plus-one bounds, validates every raw LF
record and requires exact same-delta referenced membership. It records actual
read bytes through the parent ledger. `open` is descriptor rooted and returns
one already admitted regular artifact; no pathname, remote identity or cap may
be changed by the capture module. Capture does not admit logical objects again.

The five exact frozen hash-domain vectors are included at
`design/vectors/`. Delegated tests reference these exact files using paths
relative to the crate and additionally test independently expected rehashed
semantic errors. The owner supplies `Budget::new()` and a test-only
`Budget::with_limit(usize)` for early-refusal tests.

## Phase core extraction

The phase binding reader gains a typed descriptor/raw-context boundary whose
caller supplies the five actual admitted files through opened, rehashed reads.
The shared binding constructor computes canonical-plan commitment internally,
validates index/config/shard/source/protocol, and returns the existing Binding.
The sealed path retains its current external canonical-plan comparison. No
partial caller constructs a sealed manifest or invokes sealed audit.

The pure phase validation core accepts reconstructed Binding and typed parsed
start/end records plus exact input identities. The sealed projection keeps its
current validator label, key order, raw records and false partial-chain flag.
The partial wrapper labels the separate chain claim only after complete replay.
Every prefix and the final fresh audit are checked. Re-reading/revalidating a
prefix is allowed only with cumulative allocation and actual-read charges.

## Allocation and I/O implementation choices

The fixed parent cap is 256 MiB. Before opening paths or producing errors,
reserve a 128 KiB bookkeeping envelope, a 2 MiB bounded-report construction/
serialization allowance, and a single 32 MiB child allowance. The same phase
Budget is carried through the entire invocation, including final fresh audit.
Its actual used count is reported separately; the parent reserves its entire
maximum once, so child allocations cannot escape parent admission or cause a
per-ordinal reset. The 32 MiB is not added a second time as actual allocation.

Raw JSON buffers and decoder scratch reserve `4 * (raw_size + 1)` before any
read/decoder allocation. The structural pass adds 1024 bytes per node and
8 times decoded key/string length before the materialized parse. This covers
owned typed fields, duplicate-key sets, bounded container growth and retained
copies; repeated independent parses are charged again. Actual path/map/chain
copies outside those trees have explicit additional reservations. Collections
with capacity growth reserve the next capacity with checked arithmetic and
`try_reserve_exact` before insertion. Serialization and binding/hash formatting
reserve their own maximum before construction. No post-allocation-only gate.

The initial capture implementation uses one bounded line buffer per replay and
fresh admitted owned event trees, charged cumulatively for each line. It does
not claim a reusable JSON arena or reset spent reservations after dropping a
tree. This conservative choice is explicitly allowed by DESIGN and may refuse
earlier than producer maxima. R07 tests distinguish reused byte-buffer capacity
from cumulative owned parse charges. Context/phase rereads are also charged;
they may produce an explicit earlier resource refusal instead of weaker proof.

Linux roots are opened directory-FD-relative with `O_NOFOLLOW|O_CLOEXEC` at
every component; final files add `O_NONBLOCK` and post-open regular checks.
Destination children use exclusive creation. Exact-size stream copying and
parsing retain the opened identity and consume at most declared bytes plus one.
Final inventory is descriptor rooted, finite, exact and rehashed. Restores
reserve declared disk upper bounds before artifact transfer, keep the incomplete
marker through final fresh audit and fsync, and never adopt an existing target.
Remote children retain the existing bounded process group/time/output guard;
its polling download guard is explicitly not a filesystem quota.

## Validation and delivery

The frozen P01–P16, N01–N30 and R01–R14 requirements remain acceptance targets,
not coverage claims. Keep an executed-case ledger, raw failure logs and fixture
hashes. Source/fixture bytes are frozen before final acceptance. Required actual
CLI tests include the retained policy-only producer snapshot, independent
capture/payload chains and fresh restored-tree audit. Complete standalone
release tests and strict all-target Clippy run on settled source. Existing
40-cell and checked-cell exact-output comparisons cover core extraction.

Read-only independent review covers lineage/capture, descriptor I/O/admission,
shared core and CLI. Parent coordinates durability and later GitHub CI review.
No actual fresh remote invocation is authorized by this implementation freeze.

Sources consulted: copied pinned producer snapshot/capture/live code and prior
consumer phase/binding/protocol/transport code; the parent-reviewed primary
AWS GetObject/HeadObject and Linux open(2) references in `design/SOURCES.md`.
The repository's referenced `docs/CONDUCTOR.md` is absent from the base tree;
the live CLI help/check/claim and frozen task card supply coordination syntax.

## Additive review corrections before final acceptance

PC-CHAIN-01 binds metadata, artifacts, semantics and inventory to clones of one
opened root descriptor, including the final fresh audit. PC-CHAIN-02 flushes
each newly created intermediate directory and its containing directory before
descending. PC03 forces a single AWS CLI attempt and deterministic JSON output
on each partial HEAD/GET child; mock runners cannot claim actual remote reads.
No real remote invocation accompanies these changes.

Collection growth charges the entire newly requested logical capacity before
`try_reserve_exact`, retaining all previous reservations. Bounded geometric
growth avoids a fresh exact allocation for every appended member. This is an
additive conservative clarification of the cumulative allocation-volume model;
allocator metadata/rounding and RSS remain outside that explicitly named model.
The restricted write counter is named `written_restore_bytes_before_report`:
it counts accepted writes up to report construction, excluding the report's own
serialization/write. The final directory/marker durability gate still precedes
any successful command return. Test-only syscall fault points exercise refusal
and retained incomplete state; they are absent from production builds.
