# Sparse affine substitution confirmation

[Result and visual report](../reports/sparse-compression-20261030/RESULTS.md) · [inspected report PDF](../reports/sparse-compression-20261030/RESULTS.pdf) · [frozen scientific protocol](PROTOCOL.md).

This directory preserves the original run, every failure, versioned corrections and independent checks. The fixed plan and source/runtime pins precede its original data; reproduction uses those revisions. All 612 records pass native counter checks, strict independent GP and full canonical protocol/export/file binding.

## Restore the raw records

The user-confirmed object store is `setec-astronomy-artifacts`, region `us-east-2`. Maintained operational metadata tooling restores the content-addressed archive and managed audit/analysis payloads; it does not recompute the research result. From the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v19
(cd research/twist_routes_1_3_20261021/execution-v19 && sha256sum -c runs.tar.xz.sha256 && tar -xJf runs.tar.xz)
```

Original archive: 9581956 bytes, SHA-256 `d78c4da409ce88afa2f252361988651a735e201e8947ebb207f238fc43e22296`. [Member inventory](RAW_MEMBERS.json) records all 10241 files. [Fresh remote/member/repack receipt](RAW_VERIFICATION.json) confirms each byte and deterministic repacking. Managed files are listed in adjacent `.artifacts.json` manifests; a sparse file is restored from that manifest. Verified temporary extraction/download duplicates may have been removed after the receipts; the original raw tree and archive remain. No original profile was rerun on resume.

## Audit and analysis

Use the pinned native harness with `audit PLAN.json runs/campaign-v1 FRESH_AUDIT`; keep all original output directories immutable. Mathematical replay uses the generated script under the frozen reference driver, byte pins, sole-marker/stderr policy and recorded GP version. Freeze new replay transport metadata and checkpoint before launching a fresh independent process. Source reconstruction builds from the recorded source/lock with Rust1.90.0; [measured executables](../execution-v19/reproduction/README.md) preserve the original binary bytes.

The exact one-shot native summary command was the pinned sparse summary binary with `summary execution-v19 execution-v19/audit-v1 execution-v19/analysis-v1` (all paths prefixed by the study). Full check: `python3 ../crypto-autoresearcher/tools/experiment_protocol.py check-export <wave>/experiment.export.json --protocol <wave>/protocol.json --verify-files`. The reference is `audit-v1/ADMISSION.json`.

The source-linked report gives exact counts, phase boundary, controls and exclusion rules. Setup is charged per target. Instruction counts are not clocks, cycles or calibrated GAE. No benchmark/leaderboard row is promoted by this diagnostic.
