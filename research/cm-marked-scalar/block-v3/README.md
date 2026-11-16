# Block folding V3: five-curve counted result and refused timing

The single frozen125-row untimed diagnostic completed, with every original-mu point output checked. It records all25 configurations on P-192 primary and P-224/P-256/P-384/P-521. Native summary validation passes, and a fresh archive extraction produces a byte-identical native summary. These are group-call/planner event counts, not native latency.

| Curve | Selected direct D+A | Previous cached D+A | Block-folded D+A | Folded binary D+A | Candidate planner transitions |
| --- | ---: | ---: | ---: | ---: | ---: |
| P-192 | 228 | 232 | 231 | 229 | 671 |
| P-224 | 260 | 266 | 265 | 260 | 843 |
| P-256 | 300 | 303 | 303 | 300 | 926 |
| P-384 | 449 | 452 | 452 | 450 | 1554 |
| P-521 | 606 | 613 | 612 | 606 | 2153 |

The candidate saves one doubling relative to the same previous bank/schedule on P-192/P-224/P-521, and no group call on P-256/P-384. It still exceeds the count-selected direct D+A on every curve. These are the fixed default candidate/binary rules; the selected direct artifact is the previous optimized count plan, with its runtime loading/parsing included if timing executes. All six ordinary window widths, API references and controls remain in count-summary.json. No result was selected by native timing.

Complete candidate vectors (D,A,N,logical table entries; physical bank objects; normalized odd-table entries; DP transitions) are P-192(190,41,16,6;10;4;671), P-224(220,45,18,10;14;8;843), P-256(251,52,25,10;14;8;926), P-384(379,73,27,18;23;16;1554), P-521(520,92,34,18;21;16;2153). D/A/N include actual prefix/table construction and the loop. The DP counter counts offered transitions, not every CPU instruction or heap allocation. Folded-block counts include zero-column blocks and must not be read as operations saved.

All planning, label generation, allocation, point-bank setup, normalization and destruction are inside the implemented full cold clock. The five phases and nested bank/loop subphases are validated. However, this attempt collected no native function times: calibration admission returned3 before the worker, recording6.3 other CPU-seconds over the two-second preflight (ceiling0.2), with CPU/memory PSI0. No calibration rows file, held-out launch or retry. The true CPU cost of planning/setup, A/A noise and an e2e speed ratio remain unmeasured. Count savings do not establish an e2e gain.

## Publication and resources

PR3674 was updated before execution. Source/qualification3b7fe627df895dd2fbef4d2bbebee6c5bda75016 and freeze f43454319f8dd6c885fcfcde756ae73f1b698d92 were verified hosted before data. Canonical protocol body digest72609205fba2fa0c5f16387ade18663276b538dadb7042013d329d444ce40860. The new implementation follows the user's explicit request; the older refused attempt is unchanged. One diagnostic, one calibration/conditional held-out launch,120seconds/2GiB/thread1 per invocation; no unchanged retry, cloud job, cap/role/budget change, generator or controller restart.

## Custody and replay

The original attempt-001 tree stays locally ignored and unchanged. Its exact archive copy is runs/attempt-001. runs.tar.xz is5244bytes, SHA256a0eda0859cc46f8821d403d2361581f54735c1250c1e5572b5f9a9f59fc7a124. runs-manifest.sha256 binds every raw member. Verify the basename archive checksum, extract into a fresh directory, then verify the member manifest against that directory. Deterministic repacking reproduces the archive. Fresh native --count-summary replay is byte-identical; this replays analysis, not the125 point executions or a timing attempt.

The native software qualification and its preserved first warning/intermediate snapshots are in qualification/block-build-v1 through v4. No root Cargo test result is claimed; no Rust implementation changed. KB tools/wrapper unavailable as before; the existing construction/cold contracts and Bernstein–Lange introduction/sections3–4 were checked. No novelty, global-chain optimality or expanded global-quotient claim.
