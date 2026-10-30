# Proposed standalone implementation boundaries and freeze sequence

This is design only. Parent review precedes implementation. The intended isolated consumer branch is stacked on PR #2816 at exact commit `67629e36fc45775ad2166f80f0e7b9c0eab1d144`; moving PR heads are not implicit permission to rebase or change this baseline. The producer runtime is a separate pending pin.

The implementation can stay entirely within the existing standalone consumer crate and dedicated consumer documentation. Suggested source boundaries:

| Module/API sketch | Responsibility and boundary |
|---|---|
| `phase.rs::audit_sealed(root, members, cell_context, admission) -> Result<Option<CheckedPhaseAudit>>` | Identify reserved paths/roles; return None only for the actual legacy/no-checked case. Establish exact descriptor membership, call independent binding and phase validators, build restricted results. Wrong/missing checked evidence is an error, never None. |
| `phase/schema.rs` | Independently owned strict phase/native/policy wire types, required-null wrappers, closed enums, bounded lists and strings. No producer imports or permissive nested Value. Exact nested field inventory is pinned to final owner source before coding. |
| `phase/binding.rs::bind(...) -> BoundAttempt` | Actual file-byte hashes; existing independent canonical plan/index reuse; v2-only attempt/run/config/protocol freeze validation. Return the complete attempt-binding identity and source descriptors. Do not conflate protocol file/body or raw/canonical plan hashes. |
| `phase/binding.rs::protocol_body_digest(raw, admission)` | Dedicated unique-key, number-token-preserving reader and independent pinned canonicalization. Exclude exactly the top-level freeze. Stream the digest; account for raw/string/number/key-sort scratch and retained nodes before allocation. Do not call producer functions or alter legacy Value semantics. |
| `phase/native.rs::validate_collection(record, phase_pid, policy, admission) -> DerivedProjection` | Independent finite-state checking of supplied calls/context/FD/ID/attrs/fallback and arithmetic. Return the derived observations, started state and lifecycle status for exact comparison. Never open counters or invoke a producer verifier. |
| `phase.rs::validate_context(bound, finite_records, admission)` | Four-phase dependency closure and pair checks, policy/byte identity, strict usage/error claims, missing-end state, independent native projection comparisons. Pure finite evidence validation, reusable by a future partial adapter. |
| `phase/io.rs` / `phase/budget.rs`, or small private equivalents | Ordinary contained files, exact per-type byte bounds, hash-bound reads/EOF, bounded parsing and pre-allocation modeled-storage reservations. Keep the 32MiB proposal local to new component work; no global cap increase. |
| `phase/tests.rs`, `native/tests/checked_phase.rs` | Hand-derived fixtures, exact canonical vectors, pre-allocation refusal, semantic rehashing and actual standalone audit/export paths. External retained producer fixtures exercise interoperability only after final owner source pin. |
| Small `audit.rs`/`lib.rs` integration | Register module, preserve v1 dispatch/output, attach new v2 restricted summary; no feature expansion, no generic/root arithmetic refactor. |

The new phase result needs explicit states, not a single broad verified flag: policy-only; checked durable start with no end; checked complete recorded phase; valid recorded unavailable counter; and validation/resource error. `CheckedPhaseAudit` summarizes actual supplied and checked records. It cannot assert complete execution, hardware truth, cost eligibility or mathematical solver conclusions.

The module admission ledger charges all allocations introduced by this component: file buffers, independent parsed protocol numbers, canonical scratch, cloned authoritative values/descriptors, typed wire structures, projection and retained output. Existing `plan_info`, manifest/member Value allocations and cross-cell audit retention are outside this component's new claim. A copy into the new component is inside the ledger. Reserve capacity before allocation and release only when the corresponding new object is dropped; keep charges for retained output through handoff.

Native contract reconciliation must retain the owner's latest correction: native syscall support is only little-endian 64-bit Linux x86_64/AArch64; unsupported big-endian records can declare request byte order but have no native calls/TIDs. Open descriptor uniqueness is simultaneous, not global over attempts, so a fresh fallback can reuse a closed descriptor number. Event ID zero handling follows the final exact native code; no positive-ID rule is introduced by this design.

## Exact canonicalization fixtures

`protocol-canonical-vectors.json` contains 14 positive and 11 negative authored cases, with literal expected canonical body UTF-8, full protocol bytes, body SHA-256 and full file SHA-256. Its expected strings are specified independently; hashes were computed over those strings. It is not a consumer test result or a producer replay.

Positive cases include: field/whitespace order; integer -0 versus float -0.0; 1 versus 1.0 versus 1e0; exponent boundaries at -4/-5 and 15/16; shortest finite float forms including subnormal/max finite; integers beyond u64; Unicode keys/values and escaped key spelling; preserved array order; nested sorted objects; newline/slash strings; and preserving nested freeze while removing only the top-level freeze.

Negative cases include: duplicate and escaped duplicate keys; nested duplicates; non-JSON NaN; overflow to a nonfinite float; draft or filed-after-data freeze; blank frozen timestamp; missing freeze; body changed with an otherwise correct updated file hash; trailing JSON. Bind each fixture's full file hash into its plan in integration tests so checksums alone cannot decide the semantic cases.

The protocol digest remains the repository's pinned Python-compatible contract. These vectors deliberately distinguish it from generic serde serialization or JCS; neither is an acceptable replacement. The owner must review the vectors and native field inventory before the design commit is frozen.

## Sequence for parent review

1. Parent reviews DESIGN, ACCEPTANCE, MODULE_BOUNDARIES and the exact vector file; resolve any changes to the proposed 32MiB component admission model and native wire inventory.
2. Existing consumer owner coordinates a narrow implementation task/claim in an isolated branch from exact PR #2816 source. Commit this design and vectors before runtime edits. This planning agent claims no implementation ownership.
3. Reconcile the final corrected producer phase and native source pins and policy digest. If schema/contract changes, explicitly amend the design; do not infer compatibility from a moving checkout.
4. Only after parent implementation authorization, add independent code and deterministic tests, then use the standalone build/test coordination. Retain exact immutable executable/source/input evidence and existing software delivery gates.

Sealed-cell v2 acceptance is the first increment. Independent partial-snapshot lineage/restore support remains an incomplete requirement of the larger task, requiring a later adapter and its own finite chain tests. Nothing in this staged scope marks that requirement complete or authorizes a new cloud or hardware run.
