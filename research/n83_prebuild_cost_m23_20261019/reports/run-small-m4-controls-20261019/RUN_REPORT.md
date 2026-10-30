# N83 K2048/D12 m=4 constructed-control result

**Verified control result.** The `small-m4-v1` SSH card started at
2026-10-19 18:54:41 UTC. Both exact base imports passed. Each base's
four-summand constructed control returned a checked original-field model,
stored-point group relation and rank-one coefficient row. Separate native
general-field replay passed on Linux x86_64 and macOS arm64, including
changed-target, point and coefficient rejection. All six SSH control,
preflight and replay containers exited zero without OOM.

| base | columns / points | SAT variables / clauses / XOR rows | import / model / root / search (ms) | worker wall (ms) | cgroup peak (bytes) | checked rank |
| --- | --- | --- | --- | ---: | ---: | ---: |
| K2048 | 2,048 / 339,968 | 84,966 / 900,062 / 3,901 | 469.546 / 75.336 / 108.535 / 0.334 | 672.997 | 226,369,536 | 1 |
| D12 | 2,001 / 332,166 | 84,590 / 883,894 / 3,901 | 454.976 / 70.489 / 105.260 / 0.290 | 651.126 | 217,911,296 | 1 |

These are measured single-process observations, one control per base.
The seven x blocks, selectors and phase-zero choices are pinned in the
controls. Both returned with zero decisions/conflicts and three completed
XOR passes. The controls use different constructed targets, and the host
is shared L0; no timing ratio, base ranking or pipeline improvement is
admitted. Point counts and SAT model sizes are distinct obligations from
natural relation yield. The [figure](controls.svg) and [150 dpi render](controls.png)
show the measured control walls; [figure data](controls-data.json) names
the archived source receipts.

The card next runs the unpinned common public target in K1,D1,D2,K2 order,
with 1,790-second internal and 1,800-second external caps. At this report's
control boundary the first public arm had entered search and was writing
durable progress. The public card is **in progress**, and its final
statuses, natural relations and cumulative useful rank are not reported
as complete here. The control archive excludes those active runs.

## Reproduction and provenance

- [Frozen protocol](../../SMALL_M4_PROTOCOL.md), [host card](../../SMALL_M4_HOST_CARD.md), and [pre-execution progress-check addendum](../../SMALL_M4_HOST_CARD_V2.md).
- Worker commit `611aad60e4d9952d6239a3382a77184da3640eeb`; Linux worker SHA-256 `3048550afc4ad1bdad1e390f74802112dfc6fadf96048cbff823f8f65364b0c3`; Linux verifier SHA-256 `359968c67994521198c4b8b69f599b0f937bf080c2f0c26046346e021fc23b22`.
- Local arm64 verifier SHA-256 `614775f6297a139f33a9158c696b406a19162a767ebc2c1eaeffbaf7108b4bbf`; [K2048 replay](local-replay/K2048.json), [D12 replay](local-replay/D12.json). The temporary input cache from the earlier turn was absent; the same digest-pinned archives were fetched to SSD990-2 before these replays.
- [Completed control archive](../../small-m4-v1/controls/README.md), SHA-256 `076c60d6ba10127e9d07be2e6ccab5240f7dcc9356c763f2f96f5b86556d03ae`. Local manifest verification passed. A second GNU-tar/xz packing produced byte-identical output.
- EC2 m8i.8xlarge Spot, pinned CPU 31, one CPU quota, 4 GiB hard memory, zero swap, no network, read-only inputs and filesystem. Other research and migration activity made the host L0. Its CPU 31 shares a physical core with CPU 15. Container and host receipts are in the archive.

The original prebuilt/on-demand and full-cost requirements remain open
beyond the separately completed finite m=2/m=3 panel. No canonical
leaderboard or boundary row moves: these are constructed correctness
controls. The IC baseline, complete cold/reused cost, natural rank, linear
algebra, target verification, same-point rho and external solver comparison
remain separate measurement obligations. `candidate_id` and
`selected_best_total_runtime` stay null.
