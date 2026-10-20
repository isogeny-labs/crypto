# XOR cache v2: completed execution record

**Final status: COMPLETE_BOUNDED.** The existing authorized SSH card ran from 2026-10-20 01:16:34 UTC through 02:00:10 UTC. Migration reconciliation did not start or stop a card.

- All 16 natural attempts (K2048/D12 × public targets 0/1 × A1/B1/B2/A2) reached 10,000 conflicts with `UNKNOWN_search_cap`. No natural relation returned. Capped outcomes do not establish zero yield.
- In each cell, the two cache-off repetitions match each other and the two cache-on repetitions match each other. Decisions, conflicts, restarts, XOR passes, row operations, and row scans match across settings.
- Cache-on repivot visits + cached skips exactly equal cache-off repivot visits in all four cells. This is a work-counter diagnostic only; shared-host elapsed times are not compared.
- Four constructed controls and four independent general-field replays passed. The default solver remains cache-off. Natural useful rank, complete IC cost, and base selection remain unresolved.

Run report: [RUN_REPORT.md](../reports/run-20261020T0116Z/RUN_REPORT.md). The graph and exact cell data are adjacent. Full receipts: [archive](../runs/xor-cache-v2-final.tar.xz) and [per-file manifest](../runs/xor-cache-v2-final.files.sha256). Archive SHA-256: `cf3310c7cb597c144b30a427683ed41d37cf4e2ec2425945d259a1173e776abd`; manifest SHA-256: `55a87e7e0ca6561c48d1ad558ec8b3d8ba385e4cb4c26a6b6188688732eea4ee`.

The 01:24:11 observation from the previous handoff is historical; this completed card and receipt set supersede it. Frozen [protocol](../XOR_CACHE_PROTOCOL.md), [host card](../XOR_CACHE_HOST_CARD.md), and [driver continuation](../XOR_CACHE_DRIVER_CONTINUATION.md) remain governing records.
