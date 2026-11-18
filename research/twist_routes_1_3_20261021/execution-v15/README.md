# Fixed input generation and independent reference correction

[Result and visual report](../reports/cold-inputs-20261029/RESULTS.md) · [inspected report PDF](../reports/cold-inputs-20261029/RESULTS.pdf) · [frozen scientific protocol](PROTOCOL.md).

This directory preserves the original run, every failure, versioned corrections and independent checks. The fixed plan and source/runtime pins precede its original data; reproduction uses those revisions. This is a descriptive finite diagnostic; consult the report for its exact admission scope.

## Restore the raw records

The user-confirmed object store is `setec-astronomy-artifacts`, region `us-east-2`. Maintained operational metadata tooling restores the content-addressed archive and managed audit/analysis payloads; it does not recompute the research result. From the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v15
(cd research/twist_routes_1_3_20261021/execution-v15 && sha256sum -c runs.tar.xz.sha256 && tar -xJf runs.tar.xz)
```

Original archive: 9776 bytes, SHA-256 `d34f16bc190d146a5474b7511e5b8f29b6d554007e60064bc268ac90cde6fc8f`. [Member inventory](RAW_MEMBERS.json) records all 15 files. [Fresh remote/member/repack receipt](RAW_VERIFICATION.json) confirms each byte and deterministic repacking. Managed files are listed in adjacent `.artifacts.json` manifests; a sparse file is restored from that manifest. Verified temporary extraction/download duplicates may have been removed after the receipts; the original raw tree and archive remain. No original profile was rerun on resume.

The source-linked report gives exact counts, phase boundary, controls and exclusion rules. Setup is charged per target. Instruction counts are not clocks, cycles or calibrated GAE. No benchmark/leaderboard row is promoted by this diagnostic.
