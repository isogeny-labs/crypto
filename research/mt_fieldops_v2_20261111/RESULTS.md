# IC4 vs the cheapest of four rho arms on fresh curves (v2, 2026-11-11)

Protocol `PROTOCOL.md` frozen in commit 6b0ac6f192 (2026-11-11T01:38:02Z) before any v2 data; IC parameters
`frozen_ic.json` (mu = 1.0, g = 2.0) carried over unchanged from the v1 tune set. 33 never-used curves
(`instances/`, 0 overlap with the 1031 curves in `research/`), seeds 11 and 12, 2112 runs, 0 errors, every target
of every run recovered and checked by counted scalar multiplication [pcc:unverified]. Bench record
`benchmarks/records/20261111T0142Z-mt-fieldops-ic4-v2.json`. Tables `results/holdout/analysis.txt`, summary
`results/holdout/summary.json`, figure `ratios.svg`/`ratios.png`, report `REPORT.pdf`.

The rho reference ("rhobest" in the tables; "strongest rho" below is shorthand for it) is, per configuration
and per (I/M, h), the cheapest of 12 runs of the **four pre-registered rho arms in this harness**: `rho`, `rho2`,
`rho2x`, `rhoseq`, each at kappa 0.5 / 0.7 / 1.0 (this favours rho). It is not ecbench's rho: this harness is a
separate counted u64 field layer (`examples/mt_fieldops.rs`), not the ecbench harness. The x-only sibling rho
`rho2x` is the cheapest arm in 51 of the 54 (cell, I/M, h) settings and `rho2` in the other 3 (24 b,
T = 16384, I = 30M) (validator count).

## Headline [pcc:unverified]

Cost per target in GAE (1 GAE = I + 2M + S); median over 22 paired configurations [bootstrap 95%].

| cell | IC4 / strongest rho at I = 3M, h = 0 / 0.1 / 1 | at I = 30M, h = 0 / 1 | §5a vs strongest rho (I = 3 / 10 / 30 M) | IC4 / bsgs2, I = 3M, h = 0 / 1 |
|---|---|---|---|---|
| 22 b, T = 1024 | 1.053 [1.042, 1.058] / 1.064 / 1.116 [1.098, 1.123] | 1.053 / 1.122 | dearer / dearer / dearer | 1.51 / 1.62 |
| 24 b, T = 1024 | 1.094 [1.086, 1.105] / 1.109 / 1.173 [1.164, 1.188] | 1.114 / 1.188 | dearer / dearer / dearer | 1.70 / 1.80 |
| 24 b, T = 4096 | 0.961 [0.957, 0.966] / 0.969 / 1.006 [1.000, 1.012] | 0.985 / 1.021 | unranked / unranked / unranked | 1.36 / 1.47 |
| 24 b, T = 16384 | 0.918 [0.916, 0.922] / 0.920 / 0.933 [0.930, 0.938] | 0.945 / 0.949 | cheaper / cheaper / cheaper | 1.20 / 1.31 |
| 25 b, T = 4096 | 0.972 [0.967, 0.977] / 0.981 / 1.022 [1.017, 1.030] | 0.997 / 1.038 | unranked / unranked / unranked | 1.44 / 1.53 |
| 25 b, T = 16384 | 0.908 [0.906, 0.911] / 0.912 / 0.928 [0.925, 0.932] | 0.936 / 0.944 | cheaper / cheaper / cheaper | 1.25 / 1.35 |
- **IC4 is cheaper than the strongest rho at T = 16384, in both group sizes, at every I/M and every h:
  0.91x to 0.95x as measured.** That is the row. Sensitivity (validator, `validation/VALIDATOR_REPORT.md`
  section 6.2, an estimate from the measured counters, not a new run): every rho arm pays a per-target start and
  restarts (Q plus two bank points; 2.5% to 3.2% of each rho run's cost) that IC4 and bsgs2 do not; with it
  removed IC4 is still cheaper at T = 16384 (0.932x to 0.978x, largest upper bound 0.982). Further rho
  overheads (phase-1 walk waste, restarts, sibling self-duplicates) were not quantified and can only lower rho,
  so the measured margin is an upper bound on IC4's lead within this rho family.
- At T = 4096 the order is not stable across the band (h = 1 ratios above 1 in every setting): unranked. With
  the rho start overhead removed it becomes dearer at I = 10M and 30M.
- At T = 1024 IC4 is dearer (1.05x to 1.19x).
- Against paired BSGS IC4 is dearer everywhere: 1.18x to 1.80x (distance remaining).
- Against the anchored batch rho that the earlier IC campaign was matched to, IC4 stays cheaper in most cells
  (`analysis.txt`, "vs rho"), as in v1; that comparison is superseded by the strongest-rho reference above.

## Shared verification constant

Every arm's counts include the counted double-and-add check of every recovered log, identical across the 16 runs
of a configuration: 31.6 (22 b) to 36.3 (25 b) GAE/target at I = 10M (for IC4 at 24 b, T = 16384, 34.7 of 63.8
GAE/target). It pulls every ratio toward 1. Without it (validator section 6.3): IC4/strongest rho 0.840 to 0.929
at T = 16384, 0.941 to 1.045 at T = 4096, 1.078 to 1.209 at T = 1024; IC4/bsgs2 1.49 to 2.04; all section-5a
verdicts unchanged.

## Phases

The per-phase breakdown (factor base, table, relations, solve, targets, verification for IC4; database, targets,
verification for the rho arms; baby table, targets, verification for bsgs2) is in each row's `phases` field of
`results/holdout/rows.jsonl` and is tabulated by the validator in `validation/VALIDATOR_REPORT.md` (section 6).

## Prediction vs outcome

Pre-registered: cheaper than the strongest rho at every T >= 4096 cell for every I/M; unranked or dearer at
T = 1024; dearer than bsgs2 everywhere. Outcome: holds at T = 16384 and T = 1024 and for bsgs2; **fails at
T = 4096** (unranked: the h = 1 ratio is 1.006 to 1.038). The v1 sensitivity runs had shown this cell as
cheaper; on fresh curves with the hash dedupe counted it is not.

## Where IC4 wins and why (interpretation, not measured)

Class of change: an algorithmic arithmetic change in a multi-target method (x-only pair sums sharing one
inversion, a progression shared by all targets), measured in counted field operations; not a relabelling or an
accounting change, and not a single-target result.

IC4's target trials need no y-coordinate and its trial progression W_k is computed once for all targets; the
strongest rho (`rho2x`) also gets two x-classes per inversion but must carry y along its walk. The fixed cost of
IC4 (factor base, table, relations) is amortized over T, so its advantage appears only at large T; at T = 1024 the
fixed cost dominates.

## Limits

Not the ecbench harness (a separate counted field layer, so the row is outside tables A-C); F_p additions and
subtractions are not counted in any arm; no transfer to ECC2K-130 / binary-field m = 83 is claimed or tested.
Toy groups (2^22 to 2^26); I, M, S counted, not timed; memory unbounded for every arm; local host (no
isolab-eks: this container has no AWS identity); one rho family (r-adding walks; no distinguished-point rho);
producer-written harness and analysis, so the row stands only after the independent validator's replay
certificate and report.

## Validation

Independent validator (`validation/VALIDATOR_REPORT.md`, `validation/replay-certificate.json` status verified,
`validation/validator-report.json` status pass; scripts in `validation/scripts/`): freeze order pass; instances
fresh and valid (no exact, isomorphic or same-j overlap with 30,652 earlier curves); all 2112 rows replayed with 0
mismatches on the measured binary, a scratch build of the frozen source and an instrumented build; 774,144
recovered logs independently checked, 0 failures, negative control flagged in all 6 arms; every median, interval
and verdict recomputed from `runs.json`. Disposition: validated with amendments (applied above). Claim
PCC-20261111-fa13c8 [pcc:unverified] until CI on main signs its verdict.
