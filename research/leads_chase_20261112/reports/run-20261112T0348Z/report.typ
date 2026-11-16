#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= J2 run report: factor-base constant of the N4v2 plane-quartic DL1 solver

*Run* 2026-11-12, 03:48–05:20 UTC, local container (4 vCPU, 15 GB, oversubscribed by concurrent jobs; no isolation tier earned; counted M-equivalents, not wall time, are the measure). Commit of the protocol: `dccf37b6` (grid), amendment `0056aced` (held-out, frozen before its units). Binary `v2.rs` (research/quartic_n4v2_20261020) built with rustc 1.97.0, SHA-256 `183810cf…79bb1`. Bench record `benchmarks/records/20261112T0348Z-j2-percolation-fb-size.json`. All values `[pcc:unverified]`: one machine, no independent replay.

*Command per unit:* `v2 ic <q> <seed> D 3.0 <c> 4 4` (arms D, DL1, DL; c_S = 3.0, γ = 4, block 4 as frozen in PARAMS.md); three `v2 calib <q> <i>` per size; cost = Σ over phases and over every attempt of mul + r_inv·inv + r_lmul·lmul with the median calibration (PARAMS.md rule); band: plus w·GAE·(hash + cmp), w ∈ {0, 0.1, 1}.

*Requested and completed:* grid 54 units (c ∈ {1.40, 1.50, 1.60, 1.75, 2.00} × k ∈ {16, 18, 20} × seeds 1–3, and c ∈ {1.60, 1.75, 2.00} × k = 22 × seeds 1–3), 54 completed; held-out 30 units (c ∈ {1.60, 1.75, 2.00} × k ∈ {21, 22} × seeds 4–8), 30 completed after a rerun of 26 that first ran with the binary missing (empty output, kept aside). A/A repeat (k = 16, seed 1, c = 1.75): all counters identical.

#figure(image("../../figures/j2_threshold_cost.svg", width: 100%), caption: [Grid, seeds 1–3. Left: first-attempt success. Right: charged DL1 M-eq relative to the same seed at c = 2.00 (derived), mean with min–max over seeds; shaded box: the preregistered interval at c = 1.75.])

#table(columns: 4, [*k (q)*], [*success at 1.40/1.50/1.60/1.75/2.00*], [*ratio at c = 1.60*], [*ratio at c = 1.75*],
  [16 (65,479)], [0/3, 0/3, 3/3, 3/3, 3/3], [0.87–1.12], [0.91–1.00],
  [18 (262,111)], [0/3, 0/3, 3/3, 3/3, 3/3], [0.72–0.80], [0.79–0.85],
  [20 (1,048,507)], [0/3, 0/3, 3/3, 3/3, 3/3], [0.67–0.69], [0.78],
  [21 (2,097,091) held out], [–, –, 5/5, 5/5, 5/5], [0.66–0.69], [0.77],
  [22 (4,194,271) grid + held out], [–, –, 8/8, 8/8, 8/8], [0.65–0.67], [0.77])

*Outcomes against the protocol.* Threshold between c = 1.50 and 1.60 at every k (predicted c_50 ∈ [1.40, 1.75]): holds. Success at c = 1.75: 100 %. Cost ratio at c = 1.75 in [0.70, 0.85] at every k: falsified at k = 16, holds at k ≥ 18. The post-hoc arm c = 1.60 was frozen as a held-out test and confirmed at k = 21, 22: ratio 0.655–0.685 (w = 0), per-seed order 1.60 < 1.75 < 2.00 at w = 0, 0.1, 1. At k = 16 the order fails at w = 0 and 0.1.

*Scope.* Engineering change of one parameter of a frozen pipeline, on the weak GHS quartic family only. Not validated (no replay certificate or validator report), so no leaderboard or benchmarks/README row. Raw records: `research/leads_chase_20261112/j2/units/`, `j2/heldout/`.
