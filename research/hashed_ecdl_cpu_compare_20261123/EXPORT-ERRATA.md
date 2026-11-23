# Original protocol/export seed-table mismatch

This is a post-run disclosure. The accepted pre-run protocol
`protocol.frozen.json`, its hash `5bcd3100…`, all 360 raw calls, original native
analysis and lossless `export-v1/export.json` remain unchanged. The original
protocol-bound canonical check **fails with 324 findings**. Every finding is a
measured cell outside the protocol-derived seed table. [pcc:unverified]

The captured canonical checker's `seeds_table` derives numeric instance IDs
`0..11`, measured replicates `0..8`, and cycles the two `explicit_list` seeds
across every measured row. The published native driver, fixed configuration and
human protocol instead identify a target by `case-seed`, use collector rounds
`1..9` for measurements, and keep one target/oracle seed fixed for each target.
Those choices were present before data; the structured `explicit_list` field
does not encode them correctly. The pre-run schema/readiness checks missed that
semantic inconsistency. [pcc:unverified]

Offsetting replicates or renaming instances cannot fix the fixed-versus-cycled
seed conflict. We retain actual seed fields. No new seed, target, replicate,
measurement, accepted criterion or pre-run timestamp is manufactured.

Two distinct records accompany the same immutable export:

- `canonical-check.stdout`: original protocol-bound failure, 324 findings.
- `schema-flow-file-check.stdout`: canonical `validate --verify-files` without
  a protocol argument, zero findings. Its scope is JSON schema, unique cells,
  internal flow and referenced file identities. It does not check seed binding
  or accept the original protocol/export pairing. [pcc:unverified]

The native adapter independently retains all 24 raw CSV tokens and original
status JSON, checks the actual case/seed/round mapping, and requires the full
native completion/point-control/hash census and observed supervisor exit. The
native observation remains 360 completed calls on 12 targets, with the recorded
controls. Protocol-bound canonical acceptance remains failed. Neither fact
replaces the other. The producer records are unsigned. [pcc:unverified]

`protocol.next-draft.json` is a **future-only, unadopted draft**. It selects
`harness_drawn` and describes the deterministic frozen backend rule, which can
represent fixed named target seeds. Its freeze block is draft/null. It does
not amend the original run retrospectively, authorize another attempt, or
provide prospective standing for existing data. Before future measurements,
exercise the actual export identity/seed mapping using explicitly synthetic
metadata, then review, freeze, publish and checkpoint that future protocol.

Unsigned metadata statement: `PCC-20261123-169bed`. Canonical checker:
`/workspace/classical-ecdlp-validation-ready/tools/experiment_protocol.py`,
`seeds_table` and `lint_export`; original main data and export stay under
`/tmp/ecdlp-cpu-comparison-20261123/`. S3 remains excluded.
