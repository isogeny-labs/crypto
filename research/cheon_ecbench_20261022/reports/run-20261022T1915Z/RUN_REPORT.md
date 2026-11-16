# Run report: Cheon ecbench sessions, 2026-10-22T19:15Z

This report is descriptive only. The interpretation is in [`../../REPORT.md`](../../REPORT.md).

## What ran

The three specs below were run at isolation level L0, recording counts only.

| session | spec id | session id | arms | curves |
|:--|:--|:--|:--|:--|
| minus | `ECS1h6ad1dae8d8c6` | `ECBS1h293023c203b3` | `rho.negation`, `cheon.pminus1_comb` | 7 (32–56 bits) |
| minus-naive | `ECS1hfc8dda230071` | `ECBS1h6323501e31ef` | `rho.negation`, `cheon.pminus1_naive` | 6 (32–52 bits) |
| plus | `ECS1h3c46fa8c4fe9` | `ECBS1ha367083f3e51` | `rho.negation`, `cheon.pplus1_comb` | 7 (32–56 bits) |

- **Specs and records.** The specs are `../../spec-minus.json`,
  `../../spec-minus-naive.json` and `../../spec-plus.json`. The raw records are
  in `../../sessions/<session>/`.
- **Workloads.** Each curve has 8 planted targets. Indices 0–5 are the
  workloads of PR #2422.
- **Binary.** The sessions record `git_commit: null` because the binary was
  copied out of the build tree. Its sha256 is
  `0750e00f676c7bd5e225c8edd9ebb07e9b017a45f5e7bf1a2072ae009573f6e5`, built from
  the sources at commit `2881fd08e`.
- **Host.** Intel Xeon at 2.10 GHz with 4 cores, shared with other jobs. No
  wall time is admitted.

## Completion

| session | runs requested | runs completed | verified | failures / timeouts |
|:--|--:|--:|--:|:--|
| minus | 112 | 112 | 112 | none |
| minus-naive | 96 | 96 | 96 | none |
| plus | 112 | 112 | 112 | none |

`ecbench verify --replay-all` replayed 320 of 320 runs identically, with 0
problems. The audit receipts and their sha256 values:

| receipt | sha256 |
|:--|:--|
| `sessions/minus-20261022.audit.json` | `b66ef51eaee1aad251c7485a650a4feb23d93c8aab419d2f90134579ce60ea03` |
| `sessions/minus-naive-20261022.audit.json` | `86418d16d871c2ce6ce4141a349782cbfdcb3a63120fdddf0381cd17c2801a3d` |
| `sessions/plus-20261022.audit.json` | `74dbf4e2d313a38a24bb5404d0ec35a79403535a07785224ab51e3a9c027404c` |

The receipts come from the same host class. There is no independent-runner
receipt.

## Measured values

- **Unit:** `ecbench.gae` group operations, with every phase charged. These are
  measured counts.
- **Per-run oracle columns:** `oracle_queries` and `oracle_server_group_ops`.
  These are also measured.
- **Per curve and arm:** mean, min, max, S = mean/√n (derived) and the paired
  ratio to rho with a 95 % bootstrap interval (derived) are in Table 1 of
  [`../../results/results.md`](../../results/results.md).

![Measured operations against group order, all arms; sessions ECBS1h293023c203b3, ECBS1h6323501e31ef and ECBS1ha367083f3e51; 8 targets per point](../../results/fig1_scaling_measured.svg)

The graph's data is in [`../../results/results.json`](../../results/results.json).
