# ic.pipeline size=auto (2026-11-07)

`prime-abscissa:size=auto` sets B = 0.5·r^(1/3), the m = 2 family optimum.
Compared with the fixed B = 32 cell, MITM pair-table oracle, on six prime
curves from 2^17.7 to 2^28, local L0 container, counted GAE.

- Paired ops: 0.247 [0.207, 0.295], robust across the §5a band. At 2^28 the
  ratio is 0.113.
- Fitted α moves from 0.908 [0.828, 0.989] to 0.598 [0.564, 0.631], against
  2/3 in theory.
- Distance to matched rho.negation_v3: 12.8× [11.7, 14.0].

Validated (`VALIDATION.json`: independent validator report, scalar replay certificate, matched rho.negation_v3 with the §5a band); local L0, not L2.
The incumbent is the fixed-B frontier cell, not the hand-tuned per-size N of
`research/prime_base_oracle_tuning_20261029`. This is not an improvement over
the current IC baseline, and there is no leaderboard row.
Report: [REPORT.pdf](REPORT.pdf). Run and evidence:
`research/ecbench_ic_auto_base_20261107/`. Record:
`benchmarks/records/20261107T0640Z-ic-auto-base.json`.
