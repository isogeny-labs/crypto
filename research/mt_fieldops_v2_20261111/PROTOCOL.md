# Multi-target IC4 vs the strongest rho, fresh instances (protocol v2, frozen before any data)

Confirmatory rerun of `research/mt_fieldops_20261107/` (PR #3624). The validator of that panel
(`../mt_fieldops_20261107/results/holdout/VALIDATOR_REPORT.md`) found the matched anchored rho weak and the
`rho2` control handicapped, and used two scratch controls (`rhoseq`, `rho2x`) that cut IC4's wins from 17/18
to 14/18 and left it ahead of the x-only sibling rho only at T = 16384. Here those controls are pre-registered
arms and every number comes from never-used curves.

## Question

Is IC4 cheaper per target than **the strongest rho in this harness**, i.e. the cheapest of four rho arms per
configuration, in field operations, across the section-5a band, on fresh curves?

## Arms (one binary: `examples/mt_fieldops.rs` at the commit that adds this file)

- `ic4`: unchanged design; parameters frozen from the earlier disjoint tune set, `frozen_ic.json`
  (mu = 1.0, g = 2.0), not re-tuned. One change: the phase-1 relation dedupe lookup is now counted as a hash
  operation (the validator found it uncounted; it raises IC's cost).
- `rho` (anchored batch rho, r-adding walks), `rho2` (sibling R - A in full, +2M + S), `rho2x` (x-only sibling,
  +M + S, sibling recomputed in full on a hit, counted), `rhoseq` (phase-1 database = {G, 2G, ..., NG}, no walk
  waste; phase 2 as `rho`); each with kappa in {0.5, 0.7, 1.0}.
- `bsgs2`: sum-set baby table, paired giants, mf in {0.7, 1.0, 1.4}. Reported, not part of the rho reference.
- `rhobest` (analysis only): per configuration and per (I/M, h), the cheapest of the 12 rho runs (4 arms x 3
  kappa). This favours rho and is the reference for the headline.

## Instances (fresh)

`instances/`: 33 Weierstrass prime-order curves, `gen_instances.py --families W --bits 22,24,25 --per 11
--seed mtfo-v2-20261111` (idx 0..10), checked disjoint from all 1031 curves in `research/` (0 overlap).
Seeds 11, 12 (never used before). Cells (22, 1024), (24, 1024), (24, 4096), (24, 16384), (25, 4096),
(25, 16384): 22 paired configurations per cell, 16 runs per configuration, 2112 runs. Every target of every run
must verify; failures are reported, not dropped.

## Cost unit and decision rule (unchanged from v1)

Cost per target in GAE = [(c_I I + M + S)/(c_I + 3) + h (H + L)] / T with c_I in {3, 10, 30} and
h in {0, 0.1, 1}. Per (cell, c_I): IC4 is **cheaper** than X if the bootstrap 95% upper bound of the median
per-configuration ratio IC4/X is below 1 at all three h; **dearer** if the lower bound is above 1 at all three;
otherwise **unranked**.

Pre-registered predictions (from the v1 validator's sensitivity runs): vs `rhobest`, IC4 cheaper at every
T >= 4096 cell for every c_I; unranked or dearer at T = 1024. vs `bsgs2`, dearer everywhere.

Headline for the leaderboard row: the set of (cell, c_I) where IC4 is cheaper than `rhobest`; the distance
remaining is reported as IC4/rhobest and IC4/bsgs2, not as pass/fail.

## Limits known in advance

Toy groups (2^22 to 2^26); counted, not timed; memory unbounded for every arm; local host, not isolab-eks
(this container has no AWS identity); producer-written harness, so an independent validator replays it.
