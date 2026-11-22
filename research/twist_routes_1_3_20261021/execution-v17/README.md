# Complete cold costs and matched known-equation compression

[Result and visual report](../reports/complete-cold-compression-20261029/RESULTS.md) · [inspected report PDF](../reports/complete-cold-compression-20261029/RESULTS.pdf) · [frozen scientific protocol](PROTOCOL.md).

This directory preserves the original run, every failure, versioned corrections and independent checks. The fixed plan and source/runtime pins precede its original data; reproduction uses those revisions. This is a descriptive finite diagnostic; consult the report for its exact admission scope.

## Restore the raw records

The user-confirmed object store is `setec-astronomy-artifacts`, region `us-east-2`. Maintained operational metadata tooling restores the content-addressed archive and managed audit/analysis payloads; it does not recompute the research result. From the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v17
(cd research/twist_routes_1_3_20261021/execution-v17 && sha256sum -c runs.tar.xz.sha256 && tar -xJf runs.tar.xz)
```

Original archive: 68786960 bytes, SHA-256 `da8e6df41f965a431ec2b647977c0eceead7eef169e0fa4f2f528cfb44d1b416`. [Member inventory](RAW_MEMBERS.json) records all 85850 files. [Fresh remote/member/repack receipt](RAW_VERIFICATION.json) confirms each byte and deterministic repacking. Managed files are listed in adjacent `.artifacts.json` manifests; a sparse file is restored from that manifest. Verified temporary extraction/download duplicates may have been removed after the receipts; the original raw tree and archive remain. No original profile was rerun on resume.

## Audit and analysis

Use the pinned native harness with `audit PLAN.json runs/campaign-v1 FRESH_AUDIT`; keep all original output directories immutable. Mathematical replay uses the generated script under the frozen reference driver, byte pins, sole-marker/stderr policy and recorded GP version. Freeze new replay transport metadata and checkpoint before launching a fresh independent process. Source reconstruction builds from the recorded source/lock with Rust1.90.0; [measured executables](../execution-v19/reproduction/README.md) preserve the original binary bytes.

Both original strict GP stack-warning rejections and unchanged-script larger-initial-stack successors are retained in audit-v1/audit-v2. The original reporting stop and the versioned pre-query-failure guard correction remain. The exact flat plan completed, but canonical rectangular schedule/analysis-source checks do not pass; all findings remain. Analysis-v2 reports successful matched pairs and retains all failures. These historical protocol gaps are not fixed retrospectively.

The source-linked report gives exact counts, phase boundary, controls and exclusion rules. Setup is charged per target. Instruction counts are not clocks, cycles or calibrated GAE. No benchmark/leaderboard row is promoted by this diagnostic.
