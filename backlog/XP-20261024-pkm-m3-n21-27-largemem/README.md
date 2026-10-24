# PKM m = 3 at N = 21, 24, 27 with msolve -t 16 on 256 to 512 GB (launch card, conditional)

Status: **draft, conditional**. Run only if workstream E does not already decide H0 (see "Activation gate"). Two systems are not yet generated (N = 27 and the second N = 21 target's full run, see Inputs).

Question: at m = 3, does the Kummer-tower F4 solving degree stay at 7 at N = 21, 24 and 27, or does it rise to 8 or more? The N = 27 requirement is the H1a threshold (L > 10, which is N >= 27 at m = 3, per backlog/XP-20261015-pkm-m3-n21-m4-n20/card.json decision_value).

## Sources (every number cited)

- [research/pkm_m3_n21_msolve_20261018/RESULTS.md](../../research/pkm_m3_n21_msolve_20261018/RESULTS.md):
  - Control m3_t5 at N = 15 finishes at max degree 7 with a one-element basis (`agrees with item 3's committed D = 7`).
  - Target m3_t7_x221026520 at N = 21: the first run stops at the wall (15,458 s) inside the 6th degree-7 round, at 15.3 GB peak. Degree-7 rounds add 2,354, 4,269, 5,421, 5,043, 5,502 basis elements; matrices grow from 98k x 101k to 384k x 528k.
  - Amendment A1 with `-t 16` (run 20261018T2208Z): control m3_t5 at 16 threads takes 245.9 s with the same answer. For the N = 21 target, degree 7 ends without the unit ideal, then msolve selects 398,409 degree-8 pairs and exits 139 inside that round at 99.6 GiB, under a 108.8 GB cap, before the round reports new elements. Reading: inconclusive (memory stop).
  - Stated requirement: the N >= 27 requirement for H1a is unchanged by these runs.
- [backlog/XP-20261015-pkm-m3-n21-m4-n20/card.json](../XP-20261015-pkm-m3-n21-m4-n20/card.json): the seed card for the same question (blocked; RAM 133 to 146 GB projected for m = 3, N = 21, derived in research/pkm_dickson_torus_20261015/README.md section 5, not run). That card's m = 4 half is answered (D >= 8 at N = 20).
- research/pkm_m3_n21_msolve_20261018/run_mt.sh (the `-t $THREADS` variant of run.sh): the command is `nice prlimit --as=$CAP -- timeout $WALL msolve -t $THREADS -v 2 -f $SYS/$name.ms -o runs-mt/$name.out`.

## Activation gate

Run only if research/pkm_m3_specialise_20261024/ (workstream E: msolve on the m = 3, N = 21 system with one or two tower variables fixed, 16 GB and 2 h per run) does not already decide H0. Workstream E is not on origin/main at writing. If E reaches degree 8 at a smaller N, this card's N = 21 run is not needed for H0; the N = 24 and 27 runs are still the H1a question and remain conditional on E's reading.

## Resources

- 256 GB host for the N = 21 target 1 run (RESULTS.md Amendment A1 next step names a host with 256 GB or more); 512 GB for N = 24 and 27 (the seed card's 133 to 146 GB projection for N = 21 is derived and was not measured; the N = 21 degree-8 round reached 99.6 GiB under a 108.8 GB cap and exited there, so the degree-8 requirement is above 99.6 GiB and the 133 to 146 GB projection is not contradicted by any measurement).
- msolve -t 16 (16 threads), one run per target at a time.
- Wall: not estimated from the records read. The N = 21 run had reached 4.3 h inside degree 7 at one thread, and the -t 16 run's wall before exit is not in RESULTS.md. Per-round times are in reports/run-20261018T2208Z-mt/ (not read for this card); the estimate is null until those are read.

## Decision rule (fixed now)

Per system, the outcome class is one of:

- **Finished at degree 7** (unit ideal reached, no degree-8 round): H0 reading at this N (D = 7).
- **Productive degree-8 round** (new elements reported in a degree-8 round before the end): D >= 8 at this N. This is the H1 negative reading for the m = 3 kill test (PKM card: D >= 8 at N = 21, >= 9 at N = 24).
- **Memory or wall stop**: inconclusive. No reading is drawn from it.

Two targets per cell where the budget allows; a single inconclusive target is not averaged into a verdict.

## Inputs (pinned vs open)

| input | status |
|:--|:--|
| systems m3_t7_x221026520 (N = 21, target 0), m3_t7_x704078485 (N = 21, target 1), m3_t8_x1227422598 (N = 24, target 0) | pinned by name in RESULTS.md; `.ms` files under research/pkm_m3_n21_msolve_20261018/ (not read here) |
| N = 27 system | **open**: not generated in the sources read |
| msolve | pinned: v0.6.5 per RESULTS.md (the control reproduces the committed D = 7) |
| workstream E outcome | **open**: research/pkm_m3_specialise_20261024/ not on origin/main |
| hosts | **open**: a 256 GB and a 512 GB host are not named |

## Open inputs

1. Whether workstream E decides H0.
2. The N = 27 system (generation method and seed).
3. The hosts and their owner.
4. The per-round wall times (reports/run-20261018T2208Z-mt/).
