# Follow-up to idea 8: powered run of target-parametric incremental SAT (m=3 PDP)

2026-11-14. Toy sizes, python-sat CaDiCaL 1.5.3, one core per process, shared
4-core container. Every number here is an unverified assertion of this session,
tagged [pcc:unverified]; no pcc claim was registered. Protocol (written before
the measured runs, hashes in `PREREG.sha256`): `PROTOCOL.md`. Raw per-target
rows for every arm: `results.json` (`raw_rows`, 1,080 rows) and
`raw_l*.jsonl`; printed tables: `analysis.txt`.

## Decision per the pre-registered rule

**INCONCLUSIVE.** [pcc:unverified] Primary metric: geometric mean of
(INC conflicts)/(BEST fresh-arm conflicts on the same target), UNSAT targets at
positions 30..59, l=5 (n=17), 24 targets: **0.793, 95% bootstrap interval
0.773 to 0.812**. The interval contains the threshold 0.8 (alive needs upper
< 0.8, kill needs lower > 0.8). The replicate on the clause-shuffled CNF
(INC2/BEST) gives 0.800 (0.783 to 0.817), also across the threshold.

The written prediction (ratio 0.85 to 1.15, no position slope) was wrong: the
ratio is below it and the position slope is below zero in every cell.

## What was run

Curve y^2+xy=x^3+1 over F_{2^n}, V = random l-dimensional subspace, cells
(l,n) = (3,11), (4,13), (5,17). Per cell 50 non-decomposable + 10 planted
decomposable targets, distinct, one seeded order, the same list for all arms.
Seeds: master 2026111408; V = master+1000 l; targets +1; clause shuffle +2.
Six arms, all CaDiCaL 1.5.3 default options, interleaved per target within a
process: INC, FRESH-A, FRESH-U, SPEC, FRESH-A2 (A/A arm: FRESH-A on the
clause-shuffled CNF), INC2 (INC on the shuffled CNF). BEST = per-target minimum
of FRESH-A, FRESH-U, SPEC. No other arm or solver was run. 3 cells x 60 targets
x 6 arms = 1,080 solves: 0 verdict mismatches against the ground truth, 0 lift
failures. [pcc:unverified]

Encoding fix: `pdp.py` `build2` adds the clause OR_j c_ij per summand, which
forbids x_i = 0. Validation at (3,11), `validation_l3.json`: exhaustive over
all 1,054 rational abscissae outside V, the CNF verdict equals brute-force
triple enumeration in parametric and constant form (0 mismatches; the unfixed
encoding disagrees on 9), every SAT model lifts to rational points whose signed
sum has abscissa x(R), 20 of 20 planted targets SAT and lifted. [pcc:unverified]

## Results (conflicts; UNSAT targets, positions 30..59) [pcc:unverified]

| l | n | UNSAT targets in set | INC/BEST GM (95%) | null reference FRESH-A2/BEST | INC/SPEC | INC/FRESH-A | INC2/BEST | A/A FRESH-A2/FRESH-A (95%), sd of ln |
|---|---|---|---|---|---|---|---|---|
| 3 (fails table control) | 11 | 25 | 0.590 (0.554..0.628) | 1.151 (1.096..1.212) | 0.529 | 0.551 | 0.587 | 1.074 (1.010..1.145), 0.163 |
| 4 | 13 | 25 | 0.747 (0.720..0.775) | 1.209 (1.171..1.248) | 0.747 | 0.622 | 0.762 | 1.007 (0.977..1.039), 0.081 |
| 5 | 17 | 24 | **0.793 (0.773..0.812)** | 1.189 (1.159..1.220) | 0.783 | 0.675 | 0.800 | 1.012 (0.992..1.034), 0.054 |

Median conflicts per UNSAT target (positions 1..59) and the non-SAT reference
counts per target (not run here; units differ from conflicts, no ratio formed):

| l | INC | FRESH-A | FRESH-U | SPEC | BEST | FRESH-A2 | INC2 | linear 2+1 scan, leaf solves 2^l | MITM 2^(2l)/2+2^l | enumeration 2^(3l)/6 |
|---|---|---|---|---|---|---|---|---|---|---|
| 3 | 126 | 223 | 316 | 212 | 202 | 224 | 125 | 8 | 40 | 85 |
| 4 | 974 | 1,482.5 | 1,528 | 1,253 | 1,253 | 1,497 | 996.5 | 16 | 144 | 683 |
| 5 | 6,639 | 9,602 | 8,755 | 8,170 | 8,170 | 9,706 | 6,784 | 32 | 544 | 5,461 |

INC needs a median of 6,639 conflicts per non-decomposable target at l=5; the
linear 2+1 scan baseline needs 32 leaf solves. From l=4 to l=5 the INC median
grows by a factor 6.8 (2.8 bits per unit of l, with n moving 13 to 17), the
scan reference by 2.

### Position (does reuse grow with targets solved?) [pcc:unverified]

OLS slope of ln(ratio) on position, UNSAT targets, positions 1..59.

| l | INC/BEST slope per target (95%) | factor per 10 targets (95%) | control FRESH-A2/BEST slope per target (95%) | INC2/BEST slope per target (95%) |
|---|---|---|---|---|
| 3 | -0.0033 (-0.0056..-0.0009) | 0.968 (0.945..0.991) | +0.0019 (-0.0005..+0.0045) | -0.0044 (-0.0074..-0.0013) |
| 4 | -0.0044 (-0.0064..-0.0025) | 0.957 (0.938..0.976) | +0.0006 (-0.0008..+0.0021) | -0.0049 (-0.0064..-0.0033) |
| 5 | -0.0032 (-0.0044..-0.0019) | 0.969 (0.957..0.981) | -0.0002 (-0.0012..+0.0009) | -0.0037 (-0.0050..-0.0025) |

The INC interval is entirely below 0 in every cell; the control slope is not
separated from 0. By the protocol's reading the ratio falls along the sequence,
by about 3% per 10 targets at l=5 over 59 targets. Where it levels off is not
measured. Consequence for the rule: the primary value depends on the sequence
length that was fixed in advance (positions 30..59). Post hoc, by thirds at
l=5 (positions 1-19, 20-39, 40-59): 0.884, 0.826, 0.783 (`posthoc.json`).

### Trend in l [pcc:unverified]

Primary statistic 0.590, 0.747, 0.793 at l = 3, 4, 5. ln GM difference l=5
minus l=4: +0.060 (0.047..0.072); l=5 minus l=3: +0.295 (0.256..0.335). Both
intervals are above 0: the ratio **rises** with l (less reuse at larger l) at
equal sequence length. l=3 fails the table-exhaustion control, so the l=4 to
l=5 step is the usable one. n moves with l (11, 13, 17).

### A/A noise floor [pcc:unverified]

Same formula, clause order shuffled, fresh solver: FRESH-A2/FRESH-A at l=5 is
1.012 (0.992..1.034) with per-target sd of ln ratio 0.054 (0.081 at l=4, 0.163
at l=3). A fresh arm that is not in the minimum scores 1.19 against BEST at l=5
(the bias of a minimum of three arms). INC/BEST = 0.793 is far outside both; the
measured effect is not noise. What is unresolved is only on which side of 0.8
it lies.

### Other readings

- BEST is SPEC on 20 of 24 primary targets at l=5 (25 of 25 at l=4).
- SAT (planted) targets, 10 per cell: INC/BEST 0.948 (0.398..2.401) at l=5,
  0.814 (0.282..2.013) at l=4; not resolved. [pcc:unverified]
- Post hoc, not in the protocol (`posthoc.py`): in propagations on the primary
  set INC/SPEC is 1.36 at l=5 (1.21 at l=4, 1.14 at l=3), INC/FRESH-A 0.65. The
  conflict ratio below 1 against SPEC does not carry over to propagations: INC
  works on the larger parametric CNF. Summed wall time on the primary set at
  l=5: INC 130.6 s, SPEC 89.6 s, FRESH-A 145.5 s, FRESH-U 116.2 s; the three
  l=5 processes ran concurrently on a loaded machine, so wall time is
  indicative only. [pcc:unverified]

## Table-exhaustion control

Targets used / exact number of decomposable abscissae outside V: l=3: 60 (10
decomposable) / 19, fails, flagged; l=4: 60 (10) / 231, i.e. 26% (4.3%);
l=5: 60 (10) / 2,679, i.e. 2.2% (0.4%). No target repeats in a sequence. The
l=4 cell is weaker on this control than the l=5 cell.

## Deviations from the protocol

1. Two of the three l=5 processes (INC,FRESH-A and FRESH-U,FRESH-A2) took
   10 min 57 s and 10 min 54 s, above the 10-minute limit, with the other agent
   loading the machine; they were left to finish. Three cores were in use for
   about 11 minutes. Conflict counts do not depend on timing.
2. Post hoc descriptives were added after the data (`posthoc.py`: propagations,
   decisions, wall time, thirds). They are labelled and carry no verdict.
3. Before the protocol was written: the l=3 validation and one 4-target
   calibration smoke at l=4 and l=5 on other targets (target-seed offset 99),
   stated in the protocol; the smoke rows are in the scratch directory only.
4. The primary set has 24 UNSAT targets (50 UNSAT targets per cell, 24 of them
   in positions 30..59), not 40.
5. Repository session log, knowledge-base lookup, Conductor task, benchmark
   records and research-visuals report were not done: the task restricted
   writes to this directory.

## Not covered

l >= 6 (one l=6 solve was 106 s in the pilot); m=4; a second V, curve or target
order; sequences longer than 60, so the level at which the position trend stops
is unknown; Glucose 4, MiniSat, CryptoMiniSat; an incremental solver with
learnt clauses deleted between targets; assumption cores; any comparison with
the linear 2+1 scan in common units; the cost of INC in propagations or time
as a pre-registered metric. There is no version control here, so "protocol
before data" rests on `PREREG.sha256` and file times only.

## Rerun

Smallest cell, one command (validation, six arms, comparison with the stored
rows, analysis; about 20 to 90 s, writes to the scratch directory or `$OUT`):

    ./rerun_smallest.sh

Full run: `python3 run.py 11 3 50 10 INC,FRESH-A,FRESH-U,SPEC,FRESH-A2,INC2 raw_l3.jsonl`,
the same with `13 4` and `raw_l4.jsonl`; l=5 as three processes
`python3 run.py 17 5 50 10 INC,FRESH-A raw_l5_p1.jsonl`,
`... FRESH-U,FRESH-A2 raw_l5_p2.jsonl`, `... SPEC,INC2 raw_l5_p3.jsonl`; then
`python3 analyze.py raw_l3.jsonl raw_l4.jsonl raw_l5_p1.jsonl raw_l5_p2.jsonl raw_l5_p3.jsonl > analysis.txt`
and `python3 posthoc.py`.
