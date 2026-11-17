# v6 qualification: passed (2026-11-06)

**Job:** `ic-line-v6-qual-20261106`, protocol IC-LINE-LAYOUT-E2E-FP28-V6-20261106 (frozen protocol_sha256 `12415faa…`, merged as f352040f06 in PR #3527).
- **Launch:** launched 2026-11-06T04:37:46Z by executor-mechanical from card CARD-IC-LINE-LAYOUT-V6-QUAL-01 (sha256 `2c4f70ca…`). CodeBuild was `isolab-experiment-launcher:2063184c-fe5d-46c4-8f4f-4b7d78f16a49`; the launcher reported `state: succeeded` at 04:41:32Z.
- **Pod and build:** pod `837109e0-7c65-4b86-bf76-12f9ea9c2d9a`, image `isolab/tasks@sha256:3c4ad8bf…`, source ea8e9ef7, ecbench sha256 `73362eed…`.
- **Results:** `s3://isolab-results-104186811093-us-east-2/runs/ic-line-v6-qual-20261106/837109e0-7c65-4b86-bf76-12f9ea9c2d9a/`

## Frozen checks (PROTOCOL-V6.md §5.1), run mechanically on the downloaded files

| Check | Result |
|---|---|
| `sha256sum -c SHA256SUMS` | OK; `binary.sha256` = `73362eed…` |
| `job-exit.json` exit_code 0, single pod | true (one `pod_uid`) |
| `audit.json`: ok, regraded, records bound, 48 records, all verified, every replay reproduced | true (48/48 verified) |
| `l2-gate.json`: accepted, 48 executions, no invalid outcomes, none below L2 | true (L2: 48) |
| min `time.solve_wall_ns` over the 48 records ≥ 5,000,000 | true |
| `session.json`: complete, 48 planned and written, elapsed × 576/48 ≤ 1,800,000 ms | true (elapsed 2,767 ms) |

All five `jq -e` commands exited 0.

## Sealed

No `ecbench compare` was run on the qualification. No per-arm wall statistic, ratio or record-level wall field was read; only the global minimum gate and the elapsed projection were evaluated, as §5.1 prescribes.
- `control-comparison.json` and `session/records.jsonl` are not copied here. They stay sealed until both wall sessions have written `job-exit.json`.
- Under invalidation rule 7, the qualification is not candidate wall evidence.

## Consequence

The scientific stage is unlocked: `request-session-1.json` and `request-session-2.json`, once each.

The files in `eks-results-v6/qualification/` are copies of the job's own gate files, plus the launcher result and the publication receipt (sha256 `5301437e…`).
