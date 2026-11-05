# Instrument control and disjoint resource sizing

[Result and visual report](../reports/cold-sizing-20261029/RESULTS.md) · [inspected report PDF](../reports/cold-sizing-20261029/RESULTS.pdf) · [frozen scientific protocol](PROTOCOL.md).

This directory preserves the original run, every failure, versioned corrections and independent checks. The fixed plan and source/runtime pins precede its original data; reproduction uses those revisions. This is a descriptive finite diagnostic; consult the report for its exact admission scope.

## Restore the raw records

The user-confirmed object store is `setec-astronomy-artifacts`, region `us-east-2`. Maintained operational metadata tooling restores the content-addressed archive and managed audit/analysis payloads; it does not recompute the research result. From the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v16
(cd research/twist_routes_1_3_20261021/execution-v16 && sha256sum -c runs.tar.xz.sha256 && tar -xJf runs.tar.xz)
```

Original archive: 3417044 bytes, SHA-256 `d102b9660e94a9e72cfa063818cc753b4e74a0353417afcea33dc2f104da8ad1`. [Member inventory](RAW_MEMBERS.json) records all 3188 files. [Fresh remote/member/repack receipt](RAW_VERIFICATION.json) confirms each byte and deterministic repacking. Managed files are listed in adjacent `.artifacts.json` manifests; a sparse file is restored from that manifest. Verified temporary extraction/download duplicates may have been removed after the receipts; the original raw tree and archive remain. No original profile was rerun on resume.

The source-linked report gives exact counts, phase boundary, controls and exclusion rules. Setup is charged per target. Instruction counts are not clocks, cycles or calibrated GAE. No benchmark/leaderboard row is promoted by this diagnostic.
