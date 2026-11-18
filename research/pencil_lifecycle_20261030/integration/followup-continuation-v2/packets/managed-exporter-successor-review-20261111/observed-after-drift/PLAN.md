# Managed-stream retained-attempt exporter v2

## Implemented scope

This successor closes the two source-review blockers without modifying the
sealed v1 packet. It adds a new module and CLI, leaving the v1 schema and binary
available for historical replay.

- The exporter reads the exact frozen prepared-plan validation receipt whose
  SHA-256 is
  `1892db0ad5937bba74894c5be9f3b6f1e48dcf20ddb0284f043686a2f822ef1e`.
- It opens and hashes the seven actual plan files and accepts only the receipt's
  `plan-{16,20,24,28,32,36,40}.json` names and digests.
- The expected set comes from the plans, never from submitted attempts. The
  parser reconstructs 1,260 unique workloads, 22,680 primary slots, 18,900
  measured slots, 3,780 warmup slots, and 6,300 measured three-arm triplets.
- Every slot carries derived `sequence`, `round`, `warmup`, `arm`, `role`,
  workload, method, seed, plan, session, source, binary, and protocol identity.
- Every retry names its exact attempt-zero parent. A retry is retained but is
  never substituted for the frozen primary slot. Because the existing launch
  attempt is session-scoped, every supplied retry session must retain all
  3,240 rows of the same frozen plan; a one-row survivor cannot hide the other
  retry costs.
- Every FilePin is opened with `O_NOFOLLOW`, size checked, and SHA-256 checked.
  JSON is duplicate-key checked before typed or structural validation.
- Per-attempt evidence joins existing context, record seal, diagnostic
  snapshots/manifest, relation Header/Terminal stream, resource prefix/tail,
  native `ecbench.diagnostic_attachment/v2`, session, process-owner, and
  controller-custody schemas. Its embedded diagnostic prefix is checked where
  applicable; the resource prefix binds the attachment's exact bytes.
- Progress and embedded-prefix completeness are reported separately and remain
  diagnostic-only, matching the producer's nonfatal progress contract. They do
  not erase or promote the ordinary record, terminal relation stream, resource
  status, custody, or mapper result.
- Caller-supplied eligibility and raw-cost fields are absent from the v2 input
  schema and therefore rejected as unknown fields.
- Raw cost observations are projected from checked ordinary-record bytes. The
  finite 36-cell pair cost is emitted only by the existing
  `pencil-accounting-price-draft::analyzer`; any mapper error, blocker, missing
  counter, missing review, or partial cell set produces `null` and makes the
  candidate/reference pair ineligible.
- Structurally closed owner/controller custody is reported per arm but is not
  mislabeled as authentication. It does not suppress a valid counted-cost map;
  the output independently fixes `owner_authentication_verified:false` and
  `scientific_execution_admitted:false`.

The apply path is `integration.patch`. It requires the sealed predecessor
`pencil-managed-stream-integration-20261110/integration.patch` first, because
the v2 module consumes its managed progress/relation modules and deliberately
does not duplicate them.

## Actual-input dependency

The seven plan bytes were initially sparse-excluded, then hydrated from the
canonical artifact store into `prepared_native_plans/`. Their seven SHA-256
values match the committed receipt, totaling 5,201,676 bytes. A read-only
aggregate check found exactly 1,260 unique workloads, 22,680 unique
plan/sequence slots, 18,900 measured slots, 3,780 warmups, zero malformed
three-arm round groups, and zero malformed 18-slot workload groups. Runtime
validation still requires those exact files as FilePins; changing a plan or
deriving an expected set from submitted rows is rejected. The same run must
supply the actual per-session and per-attempt native evidence files. This
packet contains no fabricated plan, session, target, cost, or controller
receipt.

The preparation receipt's `binary_sha256` belongs to native plan generation.
It is retained in the receipt but is not treated as the later execution binary.
The execution binary is separately opened and hashed, then required to match
native session, context, diagnostic attachment, resource, and record bindings.

## Sources checked

- `research/pencil_large_prime_20261029/prepared_native_plans/result.json` —
  exact seven-plan membership and 1,260/22,680/18,900/3,780 dimensions.
- `research/pencil_large_prime_20261029/paired_export/src/plan.rs` — established
  record-to-plan identity fields and measured-round grouping.
- `research/pencil_large_prime_20261029/native_price/src/aggregate.rs` — exact
  full-cohort geometry and three-arm design.
- `research/pencil_large_prime_20261029/native_price/src/analyzer.rs` and
  `importer.rs` — registered nine-category mapper and null-on-blocker behavior.
- `src/cryptanalysis/ecbench/{diagnostics,diagnostic_resources,record,runner}.rs`
  — native context, `ecbench.diagnostic_attachment/v2`, embedded progress
  manifest, record seal, resource, and session schemas.
- `research/pencil_large_prime_20261029/src/relation_stream.rs` and the sealed
  `pencil_relation_sink.rs` — contiguous Header-to-Terminal stream and totals.
- `packets/retained-native-session-export/retained_export.rs.txt` — actual
  owner-process/controller/session custody layout.
- `pencil-managed-stream-review-20261111/REVIEW.md` — B1/B2 independent review
  findings that define this correction.

This is schema and implementation work, not a novelty claim, so no external
mathematical result is imported.

## Validation state

Source construction only. No compiler, dependency resolver, unit test, payload,
target collection, relation solve, source application, or Git index operation
was run. `retained_attempt_join_v2_controls.rs` supplies the adversarial test
source for the separately admitted validation step.
