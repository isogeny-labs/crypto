#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#set table(stroke: 0.4pt + gray)
#show heading: set block(above: 1em, below: 0.5em)
= T20: n = 73, K = 600 Koblitz index calculus, cold end to end, old versus landed wide producer

Class: *engineering* (same algorithm, same probes, same relations; native speed only). Run 2026-10-19/20 on one 4 vCPU Xeon 2.80 GHz KVM guest, 15 GiB, no PMU; IC pinned to cores 1 to 3 and rho to core 1 through `tools/isolated_bench.py` (41 isolated runs, 0 flagged contended). Branch `t20-n73-k600-rerun` on the head of PR \#2156. Protocol frozen before timing in `protocol.json` (commit c15653376). All numbers below are measured (M) or derived from the records by `examples/t20_paired_analysis.rs` (D); sources in `runs/` and `reports/analysis.json`.

== Baseline and arms

Baseline row: the landed single-target n = 73, K = 600 row (`experiments/koblitz-single-target-n73-20261003`: frozen base `base_n73_K600.jsonl`, frozen public target, rank seed 7; its recorded R1 rank-stage wall is 18,398 s sequential on another host and is a historical anchor only, since the parallel rank used here draws different per-column scalars). Arms, same binary, same instances:
- *O* (old): `KIC_LAZY_CANON=0 KIC_PREFETCH=0`, serial index build, `KIC_RANK_THREADS=3`.
- *N* (landed): defaults (lazy canonical key, Bloom-line prefetch) plus `KIC_INDEX_THREADS=3 KIC_INDEX_BATCH_INV=1`, `KIC_RANK_THREADS=3`.
Order: N1, O1, N2 (N1 against N2 is the A/A control; O1 is bracketed by them).

== Result (primary: cold end-to-end wall, every phase, process start to exit)

#table(columns: (auto, auto, auto, auto, auto, auto, auto), align: right,
[arm], [wall s], [cpu s], [index build s], [rank stage s], [base load s], [target s],
[O1 old], [9304.7], [24853], [92.4], [9207.7], [3.62], [0.29],
[N1 landed], [4622.0], [12331], [60.1], [4557.4], [3.71], [0.16],
[N2 landed (A/A)], [4559.4], [12206], [63.2], [4491.7], [3.57], [0.17])

#table(columns: (auto, auto, auto, auto), align: right,
[comparison (D)], [end-to-end wall], [rank-stage wall], [rank-stage cpu (column sum)],
[O1 / mean(N1,N2)], [*2.027* (2.014 to 2.037)], [2.035 (2.023 to 2.045)], [2.031 (2.024 to 2.037)],
[O1 / N1], [2.013 (2.000 to 2.026)], [2.020 (2.007 to 2.033)], [2.018 (2.010 to 2.026)],
[O1 / N2], [2.041 (2.023 to 2.052)], [2.050 (2.033 to 2.061)], [2.043 (2.036 to 2.050)],
[A/A N1 / N2], [1.014 (1.005 to 1.020)], [1.015 (1.006 to 1.022)], [1.012 (1.008 to 1.016)])

Intervals: 95% percentile bootstrap, 10,000 resamples over the 600 rank columns stratified by thread class (column mod 3), the same resampled columns in both arms, rank wall = maximum over thread strata. They describe sensitivity to the heavy-tailed column costs, not run-to-run noise; run-to-run noise is the A/A spread (1.4% on wall). There is one O run.

*Kill condition* (frozen: end-to-end ratio below 1.1 means the component gains do not reach the row): *not triggered*; 2.027 is above 1.1 and the interval lower bound (2.014) is above both 1.0 and the A/A spread. The component gains do reach the row: the rank stage is 99% of the wall in both arms, and its per-probe saving carries through (scan 0.458, rank cpu 0.49).

*Counted S is unchanged.* The per-column probe and attempt counts are identical in all three arms (600 columns), as are the target relation (457,561 probes), the recovered scalar and the relation row; mean 70.26 M probes per relation. So the ratio of counted operations old/new is 1.000 at every weight of the section 5a band. The gain is native time per counted operation, reported as wall time (secondary under section 8), not as a lower S.

== Verification, controls
- All three IC arms: 600 relations, rank 600, target solved, `group_verified`, recovered scalar 12345678901234567 equals the validation-only sidecar; independent replay with `examples/t20_replay.rs` (own GF(2^73) arithmetic) passes on all three rows (`reports/replay.jsonl`). All 20 rho draws `verified=true`.
- A/A: N1/N2 = 1.014 (above). Positive control (harness): scan of 20,000 states at the real base, 3 interleaved rounds, new/old = 0.458 (new 284/311/318 ns per probe, old 649/675/673), matching the component note's 0.38 to 0.5. Positive control (estimator): N1 columns scaled by 1.10 against N2 gives rank cpu 1.113 (1.109 to 1.118).

== Phase breakdown
Rank stage 9207.7 s to about 4524 s (2.03x); index build 92.4 s to about 61.7 s (1.50x; 3 threads with batch inversion here, system time about 35 to 58 s from page faults of a 5.7 to 5.9 GB process); base load, basis, linear algebra and target total under 4 s in every arm; process start and teardown outside `process_total` 0.4 to 0.7 s. Peak RSS 5.7 GB (O1), 5.9 GB (N1).

== Matched rho and distance remaining
20 rho draws (`koblitz_rho_fixture 73 0 signed_frobenius 1 packed`, one worker, walk seeds 910000001 to 910000020, same frozen target, same host): wall mean 173.3 s (bootstrap 95% of the mean 127.5 to 227.4), median 143.5, range 32.3 to 545.9; cpu mean 172.0 s; walk steps mean 26.9 M (20.0 to 34.9 M; ideal 30.4 M).

#table(columns: (auto, auto, auto), align: right,
[IC / rho (interval over rho draws only)], [wall (IC 3 cores, rho 1 core)], [cpu seconds (matched resource)],
[old producer], [53.7 (40.7 to 73.2)], [144.5 (109.7 to 195.7)],
[landed producer], [*26.5* (20.1 to 35.7)], [*71.3* (54.3 to 96.9)])

The distance to rho is about halved (wall and cpu) but IC remains 26x (wall) and 71x (cpu) above this host's rho. For reference, the 2026-10-05 frozen pair (rho ledger 390 s, IC R1 rank 18,398 s) is 47x; the rho draws here are shorter on average (26.9 M steps against 70.7 M in that single ledger draw).

*Section 5a band* (GAE per native operation w in 0, 0.1, 1; S = priced + w times native, divided by sqrt(r), r = 8.6e16; IC native = 3 per probe, derived: root solve, canonical key, Bloom query; IC group additions are not emitted by the example, so IC S is a lower bound; rho native = canonicalisations, Frobenius maps, partition hashes, table inserts and queries, negations examined, mean of 20 draws):

#table(columns: (auto, auto, auto, auto, auto), align: right,
[w], [IC S (lower bound)], [rho S], [IC / rho], [old / new IC],
[0], [0], [0.092], [undefined], [1.000],
[0.1], [43.1], [1.459], [29.6], [1.000],
[1], [431.2], [13.76], [31.3], [1.000])

The IC-versus-rho order changes inside the band (at w = 0 the IC lower bound prices nothing), so by the section 5a rule the two arms are *unresolved at w = 0 and rho is ahead for w >= 0.1*; no cross-method win is claimed. The old-versus-new comparison is identical at every weight.

== Deviations and limits (all disclosed)
- First launch aborted before any timed IC data when `isolated_bench` refused a start because this session's own polling used 0.31 CPU s in its 2 s settle window; threshold relaxed from 0.2 to 0.5 CPU and refused starts retried (orchestration only; `run_t20.sh`).
- The container was restarted several times; the driver was restarted each time and skips finished steps. N2 was killed at about 20 min and rerun from scratch (the partial stderr is in `runs/aborted/`). The scan controls and N1, O1, and the final N2 each ran once uninterrupted.
- Rho draws 2 to 12 were lost to a cleanup mistake of mine (a zero-size stderr filter removed each draw's records) and regenerated by the same frozen command after the IC runs; draw 1 (before N1) was kept. Rho draws 2 to 20 therefore do not bracket the IC runs; the draw seeds are fixed, so step counts are those of the same walks, only the timings are new.
- One O run (cost about 2.6 h); the paired interval is conditional on the columns and the A/A spread (1.4%) bounds run-to-run noise; no second O run.
- IC used 3 cores, rho 1 core: wall ratios are not matched-resource; the cpu-second ratios are. Wall time is secondary evidence (AGENTS.md section 8); no PMU, so no IPC. Hardware class: Linux x86-64 Xeon 2.80 GHz guest only.
- The IC example does not emit group additions or per-kind native counters; IC S is a lower bound.
- Not claimed: no S improvement, no advance, no rho win, nothing about n other than 73 (the engineering note predicted 1.3 to 2x for n = 69 to 83; only n = 73 is measured here).

== Next
T21 (basis-aware root solve, modulus-specialised reduction, closed-form Koblitz order) needs new code and builds that would contend with timed runs; it is written up as the next card in `NEXT_T21.md` with its kill condition, not run.

#figure(image("reports/t20_e2e.svg", width: 100%), caption: [T20 recorded values: (a) wall by phase, (b) paired ratios with 95% intervals against the 1.0 and 1.1 lines, (c) 20 rho draws against the IC wall times. Source `reports/analysis.json`, `runs/`; plotted by `make_graph.py`.])
