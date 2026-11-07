# rho v3 ladder table (2026-11-07)

`rho.negation_v3` / `rho.plain_v3`: the tuned rho search on a jump table
built from shared ladders, two additions a jump, k ≈ √(F/2) jumps. Counted
GAE on six prime curves from 2^17.7 to 2^28, local L0 container.

- Paired ops v3 / v2: 0.864 [0.832, 0.898]; the order holds across the
  §5a band.
- plain_v3 / plain: 0.939 [0.901, 0.979].
- rho.negation_v3 total S / √(π/4): 1.30 at 2^17.7, 1.04 at 2^24 and 2^28.
  The search phase is 1.00–1.03 (±0.04) at 2^24 and above.

Unvalidated: no L2 run, no validator report; not a leaderboard row.
Report: [REPORT.pdf](REPORT.pdf). Run and evidence:
`research/ecbench_rho_v3_ladder_20261107/`. Record:
`benchmarks/records/20261107T0639Z-rho-v3-ladder-fixed.json`.
