# R13 Table A tournament lacks an isolated host

- **Status:** `investigating`
- **Kind:** resource limit and access limit; empirical timing blocked
- **Affected work:** [R13 single-target protocol](../research/ic_tool_program/rounds/R13-prefetch-gate/PROTOCOL.md), Table A cold IC pricing
- **Next action owner:** R13 experiment operator and AWS account administrator for a dedicated on-demand host
- **Last updated:** 2026-10-26

## Problem

The frozen R13 cold, single-target prefetch-gate comparison cannot yet produce qualified timed A/A and ABAB records because the shared Xeon 6975P-C host is busy, a dedicated Spot replacement was reclaimed, and on-demand launch is denied in the research account. The three primary Koblitz sizes, eleven M1 sizes, 24 holdouts, five-round schedule, acceptance intervals, matched rho and scalar checks remain fixed in the protocol. Without an isolated A/A and the full paired schedule, R13 cannot select a candidate or promote a Table A row.

## Evidence and uncertainty

- **Observed:** On shared `m8i-flex.8xlarge` `i-03d5e15a586275e77`, the first A/A launch refused CPU 2 because its SMT sibling CPU 18 was not reserved. The corrected `ICPROG_BENCH_CPUS=2,18` launch first waited on `/tmp/crypto-bench.lock` held by another validation job. Its next launch produced zero timed `*.price.json` records through attempt 14; `runs/aa/refusals.log` records machine-busy preflight values from 2.86 to 6.71 other-process CPU seconds in a two-second window against a 0.20-second limit. The log and runner output are under the R13 run tree on the worker and in the checkpoint artifact store.
- **Observed:** A dedicated same-feature `m8i-flex.4xlarge` Spot instance `i-06ef80569878fa0d5` launched in `us-east-2a` and was reclaimed at 2026-10-26 20:50:27 UTC during the baseline build. A smaller same-feature `m8i-flex.2xlarge` Spot instance `i-02003f9ff691df036` launched there at 21:09:09 UTC, passed initial hardware checks, and was service-terminated at 21:15:46 UTC while both arm builds were in progress. Neither host produced a completed arm binary or timed sample. Other attempted Spot placements returned insufficient capacity or `MaxSpotInstanceCountExceeded` at the research account's standard-vCPU quota. The research account denied on-demand launch under `SpotOnlyEC2Launch`. The other configured AWS account reports zero standard EC2 vCPU quota for both Spot and on-demand in the checked regions.
- **Observed:** The host-class check, 24/24 holdout regeneration, corrected 90/90 native pin, and 114 native `icprog` tests passed on the shared worker. The release library suite had one inherited `ecbench` zero-weight legacy-ID assertion failure among 4,862 executed tests; this does not supply an R13 timing result.
- **Derived or proved:** None. These are observed infrastructure and test states, not an algorithmic result.
- **Suspected explanation:** Competing CPU activity explains the isolation refusals; this is directly testable by repeating the same preflight after the other jobs stop.
- **Missing capability:** A quiet, source-bound host of the declared CPU feature class for the full schedule, or an AWS policy/quota change permitting a dedicated on-demand replacement.
- **What this evidence does not establish:** It gives no R13 cold-cost ratio, no verdict on the prefetch gate, and no Table A row. Spot reclamation and refusals do not imply that the frozen experiment cannot succeed on a qualified host.

## What would unblock it

- [ ] Obtain a host with AVX512F, PCLMULQDQ, VPCLMULQDQ and GFNI, sufficient memory and disk, both SMT siblings reserved, an uncontended `isolated_bench` preflight and stable custody for raw records throughout the declared schedule.
- [ ] Rebuild both frozen arms with matching toolchain and lockfile; record source and binary digests, pass host-class, 24 holdout and 90 pin checks, then complete five-round A/A and suite/holdout ABAB with clean isolation receipts and verified scalars. Follow the protocol's conditional extension rule.

These conditions permit a decision for the measured CPU subclass. Any Table A promotion still requires the matching Emerald Rapids host-class reproduction and `VALIDATION.json` artifacts, including independent replay, raw-data validator and matched rho reference.

## Next discriminating step

- **Action or proof obligation:** Let the current shared-host A/A preflight finish its bounded attempt series. If contention persists, obtain a dedicated on-demand host after the research account policy permits it, or a genuinely quiet reserved window on the existing host.
- **Inputs and controls:** Frozen R13 binaries, suite and holdouts; byte-identical A/A baseline; matched same-target rho and scalar certificates; host-class and SMT checks.
- **Execution limits and stop condition:** The native A/A runner caps each first-row isolation preflight at 60 attempts with 15-second delays. A refused preflight contributes no timed sample. A Spot replacement retains its stated host budget and must not pool times with another host.
- **Outcomes and next actions:** A clean A/A permits the paired schedule; continued refusals leave the timing unresolved and trigger host change; any completed but failed control or scalar check stops candidate promotion and preserves the raw record.
- **Recheck trigger:** Current A/A completion, a quiet shared-host window, or an AWS policy/quota change.

## Resolution and history

- 2026-10-26: Recorded the shared-host refusals, two build-phase Spot reclamations and denied on-demand replacement. The frozen candidate and inputs are intact. **Unresolved.**
