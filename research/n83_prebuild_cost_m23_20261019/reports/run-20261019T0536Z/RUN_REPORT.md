# N83 native m=2/m=3 finite-query run, Spot L0

**Run ID:** `run-20261019T0536Z`. **UTC interval:** 2026-10-19 05:36:13–05:45:03. **Host:** EC2 Spot `i-01f85a6c905181498`, x86_64, 32 logical CPUs, shared with unrelated research jobs. Each worker container was pinned to CPU 31 with 4,294,967,296 bytes hard memory, memory-plus-swap equal to memory (zero swap), no network, a read-only root and read-only inputs. This is L0 timing evidence. Every arm's actual Docker configuration, peak and exit is in its raw directory.

## Frozen inputs and execution

Primary `icv1-f2m83-tm6151469093347-debefd74`, subgroup order `2417851639230796216685689`, cofactor 4. The U archive BLAKE3 is `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`; D is `e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682`. Each has 16,627 columns and 2,760,082 distinct signed-Frobenius point rows. Public corpus SHA-256 is `2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5`; the validation sidecar did not enter worker containers. Native source commit `a147d3abf4d8e47a7677a61794a1ab8003b2c307`, x86_64 worker SHA-256 `2859910318cc514e21aa6136d3263fd5daa2c8d1642fc6668448a76943d906c8`, Docker runner SHA-256 `7ef1ea404378e9f2d84e263ee8e46ce366d55b88d65327afbf9d84bd81bce40d`. The frozen command was `sh run-card-spot-l0.sh` after its six SHA-256 input checks. [Protocol](../../PROTOCOL.md), [host addendum](../../SPOT_L0_ADDENDUM.md), [run card](../../run-card-spot-l0.sh), [raw receipt tree](../../runs/spot-l0-v1/runs/), [raw SHA-256 manifest](../../runs/spot-l0-v1/runs.sha256). The transferred raw package SHA-256 was `9bd8a0171be460be5aabbde9ca7b4151d91483c523792a042b8946c93f79d322`.

Before natural probes, both D setup paths produced point-set BLAKE3 `8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0` and labeled-row BLAKE3 `805d1cbc65cac0a74ebf49315b8c17b0d5229bdceba5725ba4349fb1cc6c9e9c`. Constructed controls U/D at m=2/3 each had rank one and rejected an altered target and row. All four [separate general-field replays](control-replay/) passed on the host receipts. These controls are constructed, so their relations are excluded from natural yield.

## Recorded outcomes

| m | base | public-query arms | completed complement attempts | returned relation rows | useful rank | worker process wall mean, observed range (s) |
| ---: | :--- | ---: | ---: | ---: | ---: | :--- |
| 2 | U | 9 | 24,840,738 | 0 | 0 | 9.475 [9.415, 9.579] |
| 2 | D | 9 | 24,840,738 | 0 | 0 | 9.615 [9.537, 9.712] |
| 3 | U | 9 | 9,437,184 | 0 | 0 | 8.402 [8.336, 8.479] |
| 3 | D | 9 | 9,437,184 | 0 | 0 | 8.526 [8.452, 8.664] |

All 36 natural worker receipts say `COMPLETE`; all 36 separate replay receipts say `PASS`; all 78 Docker arms, including two setup and four controls, exited zero. Each m=2 arm exhaustively checked every first point for its exact target and base; each m=3 arm checked the frozen `2^20` sampled pairs. Target 0 has two A/A runs per base and m; targets 1–7 have one. Across U and D, the observed finite attempts are 49,681,476 at m=2 and 18,874,368 at m=3. No relation row was returned by these policies, so no natural rank increment or relation-specific group equation exists to replay. This is a finite observation, not an estimate of yield outside the tested event and budgets.

The D setup workers measured 3.622364206 s for prebuilt import/index and 28.688622769 s for in-memory regeneration/index (one arm each). Their point-set and labeled-row digests matched. The setup comparison is a single-sample L0 stage diagnostic and excludes one-time archive construction, full-object replay, rank accumulation, linear algebra and target recovery. [Figure source](setup-data.json), [SVG](setup.svg), [150 dpi PNG](setup.png).

## Censoring and remaining work

There were no capped or failed worker arms in this finite panel. The m=4 natural SAT comparison is a separate frozen experiment. This run does not qualify a complete cold index-calculus cost or a winner against the primary-N83 IC incumbent or matched same-point rho. The smaller retained bases need separate identified arms. The paired D prebuilt/on-demand per-query comparison is frozen in [its own protocol](../../PAIRED_MODE_PROTOCOL.md) and must not pool timing samples with this source revision.
