# Pinned source map and minimal ownership proposal

All consumer line references below are at `67629e36fc45775ad2166f80f0e7b9c0eab1d144`. Producer contract references are at `e9de83e5c6cc1ac2eb9633227b53d3ce3a568413`. The corrected phase runtime is intentionally not assumed from a moving checkout.

| Location | Current behavior | Smallest proposed change |
|---|---|---|
| `native/src/audit.rs:231` plan_info | File-hash/schema plan/run/protocol binding; ordered canonical plan digest. | Reuse current logic. Add v2-only binding verification; do not silently call current behavior complete AttemptBinding verification. |
| `native/src/audit.rs:325` validate_metrics | Legacy event tuple/scaling/status validation. | Leave unchanged behind v1 dispatch. |
| `native/src/audit.rs:436` phase_measurements | Role-based scan, exact v1 start/end schemas, start-only preservation. | Dispatch on reserved path/role/policy plus strict version. Keep old output unchanged; send v2 to independent module. |
| `native/src/audit.rs:495` audit_cell | Manifest/config/provenance/member hashes, positive/root/filter checks. | Pass exact authoritative descriptors/bytes to phase module; attach restricted v2 contract summary. No change to root/filter arithmetic. |
| `native/src/audit.rs:795` audit | Audits sealed cells and lists incomplete directories. | Count only actual v2 checks if a summary is added. Continue explicit incomplete/non-audited status; no hidden partial adapter. |
| `native/src/audit.rs:922` static_features | Public curve/base/static-config allowlist. | No additions. Require byte-equivalence under private evidence mutations. |
| `native/src/audit.rs:1067` export_rows | Separate restricted labels; atomic create-if-absent export. | Add v2-only restricted contract record. Preserve existing replay-or-preview gate and feature-policy versions. |
| `native/src/common.rs:15,99,181,219` | Ordinary files, unique keys, hash-bound parsing, 64MiB general cap. | Reuse integrity helpers; new per-type cap/pre-allocation semantics must occur before generic allocation. Avoid global parser semantic changes. |
| `native/src/large_prime/json.rs:1–350` | Hash-bound streaming schema routing and counted admission before Value expansion. | Reuse design/testing patterns. No generic-filter edits or borrowing its domain constants are required. |
| `native/src/transport.rs:224` | Sealed publication/v1 restore; exact manifest member set. | No transport change for sealed support. Verify preservation with existing mock/local fixture infrastructure. Partial-snapshot/v2 remains unsupported here. |
| `native/src/main.rs` | audit/export/fetch/certificate; explicit producer-replay or preview export gate. | Help may name checked evidence; no new partial/cloud CLI necessary. |
| Proposed `native/src/phase.rs` and `phase/{schema,binding,native,tests}.rs` | Absent. | Independently owned strict wire types, finite phase/native evidence checker, independent binding/freeze canonicalization, bounded reader and tests. Names are implementation proposals. |
| `native/tests/support/mod.rs`, `integration.rs`, retained fixture test patterns | Synthetic sealed fixtures, member rehashing, feature isolation, CLI tests. | Add checked fixtures and semantic rehash tests without rerunning a producer campaign. |
| `native/Cargo.toml` | Standalone workspace; Serde derive/preserve_order, SHA-256; no producer crate. | Preserve independence. Any number-token support must be narrow and tested; do not casually enable a global number representation that changes existing Unique parsing. |

Producer `src/ml_dataset/capture.rs:36–131` is the referenced AttemptBinding wire/byte contract. Producer `mod.rs:1282–1322,1380–1507` defines protocol freeze and canonical number behavior. These are specifications to reproduce independently, not code to import. The normative phase policy and complete status grammar live in the frozen phase DESIGN and exact policy file. Native collector DESIGN supplies ABI, exclusion and lifecycle background.

Current owner checks are retained in `owners.json`. Only the external planning directory was written. A proposed next implementation claim is restricted to the standalone consumer crate, its own consumer-phase research documentation and the small existing consumer README/update surfaces selected by the owner. It excludes all producer phase/native files, snapshots, generic-filter implementation, cloud state, corpus generation and shared journals unless separately coordinated.
