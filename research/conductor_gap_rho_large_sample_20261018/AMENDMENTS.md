# Amendments (after freezing, before any data)

1. 2026-10-18 07:09–08:50 UTC. The first launch of `run_sessions.sh` passed each
   session an explicit CPU (`--cpus 25` … `31`). In that mode this ecbench build
   exited with status 0 before taking its lock or creating the session directory
   (only the SMT-sibling warning was printed; no session, record or lock file was
   written). No data was produced or seen. The runner now pins each session
   externally with `taskset -c <cpu>` and passes `--cpus none` (L0, operation
   counts unaffected); nothing in the specs, seeds, arms, targets or analysis
   changed. `FROZEN.sha256.v1` is the original freeze; `FROZEN.sha256` is the
   re-freeze after this one-line runner change.

2. 2026-10-18 08:52–09:05 UTC. The second launch ran for 13 minutes; only record
   *counts* were looked at, no S value. Two throughput facts forced a change:
   ecbench spends ≈0.3 s of process overhead per execution on this loaded host,
   so the single ℓ = 8209 session (6 curves × 4096 targets × 3 arms, twice with the
   warm-up round) would have taken ≈12 h serially; and the warm-up round doubles
   every session for a wall-time quantity this study never reads. Changes:
   (i) `measurement.warmup` 1 → 0 in every spec (operation counts are unaffected;
   wall time is not a result here); (ii) the ℓ = 8209 session is split into four
   shard sessions `s1a`–`s1d` of 1024 shared targets each with distinct target
   seeds (2026101801, 03, 04, 05), the same 4096 targets per curve in total, run
   in parallel; `analyze.py` reports each shard and the pooled family. Nothing
   else changed (curves, arms, measurement seed, unit, analysis rule). The
   interrupted sessions (l1: 11 records, l2: 16, s2: 702, all in their warm-up
   or first round) are kept unmodified in `sessions/interrupted-20261018T0852Z/`
   and are not used. `FROZEN.sha256.v2` is the freeze before this amendment.

3. 2026-10-18 11:10 UTC, after the frozen analysis of tiers S, M and 21 of the 24
   tier-L pairs had been seen (l4 was still running). Tiers S and M are at 1 within
   1 % and 4 %. Tier L pooled over 21 pairs sits at 0.92 [0.85, 0.999], 16 targets
   per curve; no single pair survives Bonferroni, and one of the eight isomorphic
   twin controls in the same tier is further from 1 (1.59 [1.20, 2.18]). Under the
   frozen rule this is not a rejection. As a post-hoc, pre-registered replication
   (not part of the frozen analysis, reported separately), the four tier-L specs
   are repeated verbatim with fresh target seeds 2026101835–38 as sessions
   `l5r`–`l8r`: same 24 pairs, same 8 twins, 16 new shared targets per curve, same
   arm, same measurement seed. Prediction under H0: the replication's pooled
   floor/crater interval contains 1 and the combined 32-target interval moves toward 1.
   The frozen specs l1–l4 are byte-identical to the freeze (checked by sha256).

4. 2026-10-18 12:45 UTC, after the replication (item 3) completed. Replication
   pooled floor/crater: 0.920 [0.855, 0.991], reproducing the frozen 0.917
   [0.852, 0.987]; but its eight crater-twin controls pooled to 0.891
   [0.788, 1.002], the same size of shift with no conductor change, and the
   per-pair intervals that exclude 1 are driven by high-S crater draws that also
   depress the twin ratios of the same pairs (pairs 0 and 5). The tier-L shift is
   therefore not separable from representation noise at this n. Pre-registered
   twin round, sessions `t1`–`t8` (3 pairs each, all 24 pairs): crater, crater twin,
   floor, floor twin, with u = 5 and fresh target seeds 2026101841–48, 16 shared
   targets. Predictions: if the shift is a property of the crater *representation*
   (all 24 craters are the same j = −884736 model family), then
   crater-twin/crater < 1 while floor-twin/floor ≈ 1 and floor-twin/crater-twin ≈ 1;
   if it is a property of the *conductor*, floor-twin/crater-twin ≈ 0.92 as well;
   if it is noise, all three pooled intervals contain 1. Reported separately from
   the frozen analysis; nothing in the frozen sessions changes.
