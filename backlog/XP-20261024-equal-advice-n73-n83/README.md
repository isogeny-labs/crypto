# Equal-advice preprocessing rho vs compact-orbit IC at n = 73 and n = 83 (launch card)

Status: **draft**. Every input the repository can pin is pinned below. The card stays draft until a named idle host exists and its own PROTOCOL.md is frozen (see "Open inputs").

Source protocol (frozen in the repository): [research/equal_advice_wide_20261018/PROTOCOL.md](../../research/equal_advice_wide_20261018/PROTOCOL.md). Instrument and command: [run.sh](../../research/equal_advice_wide_20261018/run.sh) and [README.md](../../research/equal_advice_wide_20261018/README.md). Origin: [research/ic_leads_review_20261018/README.md](../../research/ic_leads_review_20261018/README.md) section 2.1 item 1 and section 3.2. Review item: REVIEW.md section 5.2, M1 (in the review workspace /Volumes/SSD990-2/ic-regime-review-20261024, not in this repository).

## Question

On the two wide Koblitz-type rungs n = 73 (K = 600) and n = 83 (K = 3240), at equal advice bytes, is compact-orbit index calculus (IC) or Bernstein-Lange preprocessing rho (BL) faster per target? Rung verdict: `BL_WINS_AT_EQUAL_ADVICE`, `IC_WINS_AT_EQUAL_ADVICE` or `UNRESOLVED`.

## Decision value

The result decides whether any online N83 number may be quoted and whether the IC online advantage over rho survives once the IC's own advice is charged. If BL wins at tier A (the IC-favourable budget) on both rungs, the leaderboard entry for the IC online win is edited (the protocol makes this a recommendation only). If IC wins at tier B (the generous budget) on both rungs, the IC claim survives equal advice at both sizes. Anything else is UNRESOLVED and no claim moves. Until this runs, REVIEW.md M1 says no online N83 number should be quoted.

## Inputs pinned from the repository

| input | pin |
|:--|:--|
| protocol | research/equal_advice_wide_20261018/PROTOCOL.md (frozen; blob recorded in card.json at launch) |
| driver | research/equal_advice_wide_20261018/run.sh (`run.sh n73 n83`) |
| code | examples/ic_equal_advice_wide.rs, examples/ic_equal_advice_claims.rs, examples/koblitz_orbit_dlp_fast.rs (on origin/main 5cb553810e at writing) |
| n73 IC candidate | experiments/koblitz-single-target-n73-20261003/candidate-manifest.json |
| n83 IC candidate | research/n83_factor_base_panel_results_20261014/candidate_K3240_root21.json, expected base hash 54847e372e3d68a2509bb16227aef70e09ceb9948d9e73149b6abc21355a2c0e |
| n83 IC environment | run.sh rung_config (KIC_ROOT_ONLY21=1, KIC_INDEXED5_FIXED30=1, KIC_SYMMETRIC_S3=1, ...) |

Curve identities (PROTOCOL.md "Rungs"): n73 `EC1N73Ckb1h25afb0fafbdf`, r = 86,020,738,150,056,119; n83 `EC1N83Ckb1a1haacb69ee9f37`, r = 8,569,786,107,849,059.

## Memory and time (PROTOCOL.md "Memory, time and hosts")

| | n = 73 | n = 83 |
|:--|:--|:--|
| resident at once | IC about 5.6 GB + BL about 5.8 GB | IC 13.9 GiB + BL 15.4 GB |
| minimum RAM (preflight rejects below) | 24 GiB | 48 GiB |
| address-space cap per process | 16 GiB | 32 GiB |
| IC setup | rank stage 18,398 s single-thread (5.1 h), 5.63 GB | 580 to 694 s with 4 rank threads |
| BL table walks (projected) | 40 to 100 core-hours (3.6e11 steps) | 27 to 67 core-hours (2.4e11 steps) |
| session wall, projected, 32 threads | 3 to 5 h | 2 to 3.5 h |

All of these are the protocol's projections, not measurements. The wide-step cost (0.4 to 1.0 us) has not been measured; the n = 61 check measured only the narrow backend (0.2 to 0.3 us).

ESTIMATE for the card header, derived from the rows above: about 73 to 173 core-hours and about 9 h wall at the upper end. The header uses the upper values.

## Host requirement

- At least 48 GiB RAM, 32 hardware threads, at least 2 GiB free disk (run.sh preflight thresholds).
- The host must be otherwise idle during the online passes (PROTOCOL.md). The script does not establish CPU isolation; the run is isolation level L0 unless an operator with an isolab receipt sets ISOLATION_LEVEL.
- **This Mac at load 70 is inadmissible.** PLAN.md records host load 60 to 70 from other sessions, so online millisecond timings from this host cannot support any verdict.

## Decision rule (PROTOCOL.md "Statistic and decision rule")

Per (run, level): ratio_i = BL_online_ms_i / IC_online_ms_i over the 64 targets; median with a 10,000-resample target bootstrap 95% interval. Tier A `BL_FASTER_AT_TIER_A` iff in both runs the upper end is below 1. Tier B `IC_FASTER_AT_TIER_B` iff in both runs the lower end is above 1. Rung verdict: `BL_WINS_AT_EQUAL_ADVICE` if tier A holds; else `IC_WINS_AT_EQUAL_ADVICE` if tier B holds; else `UNRESOLVED`. Failed controls or a replay failure override both. The rungs are judged separately.

Prediction recorded before the run (a derivation): BL wins on both rungs, median ratios near 0.01 to 0.1.

## Shards

Two shards, one per rung (`run.sh n73`, `run.sh n83`), each in its own process group on the same host. Not pooled.

## Open inputs (why this is a draft)

1. No idle host is named. None in the record meets the requirement at idle load.
2. The card's PROTOCOL.md is a pointer, not frozen. It must be frozen with the host named.
3. The wide-step cost is unmeasured; the BL projection depends on it.
4. The claim check will report FAIL on independent validation, isolation level and the replay pointer until a second host replays it (PROTOCOL.md "Limits"). The run is accounting and audit, not an improvement claim.
5. Funding and the lab space are the owner's call (card.json).
