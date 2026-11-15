# Idea 6 pilot: BDD bucket elimination for the PDP — result: kill criterion met

Pilot, toy sizes, one core, pure Python, 2026-11-14. Every number below is an
unverified pilot assertion. Protocol written before measuring: `PROTOCOL.md`.

## What was run

- `bddpdp.py`: self-contained F_{2^n} arithmetic, curve y^2+xy=x^3+1, Semaev S3,
  Weil descent to bit ANF, a hand-written ROBDD package (unique table, memoised
  and/or/xor/exists, no complement edges, no reordering) and bucket elimination.
- Validation (`python3 bddpdp.py validate`, passes): S3 roots equal x(P1+-P2) on
  n=7,11,13; BDD solution sets equal brute force for m=2 (l=3,4) and m=3 (l=2,3,
  including an exhaustive loop over all t in F_{2^n}); planted targets are SAT
  and a model lifts to curve points summing to the target; all 9 m=3 arms give
  the ground-truth answer.
- m=3 (split encoding S3(x1,x2,t)=0, S3(t,x3,xR)=0; 3l+n bit variables, 2n
  constraints): l=3,4,5,6 with n=11,13,17,19, 8 targets per cell (4 planted, 4
  random points), 3 variable orders x 3 elimination schedules, same targets,
  interleaved per target. l=7 (n=23) attempted.
- m=2 sanity cell (S3(x1,x2,xR)=0, 2l variables, n constraints): l=4..8,
  n=11,11,13,17,17, 8 targets per cell, 3 variable orders.
- 436 run rows, 364 completed, 72 blow-ups (4e6 created-node cap). Every
  completed run's SAT/UNSAT answer matches the brute-force ground truth. [pcc:unverified]

Baselines named by the protocol (counted, same instances): MITM memory
2^{ceil(m/2) l}, enumeration 2^{(m-1) l}, linear 2+1 scan 2^l. The brute-force
ground truth (a MITM over (x1,x2) against x3) was run on every target. The
untreated SAT arm was not re-run in this pilot.

## m=3 table (medians over 8 targets; node counts)

| order | schedule | l | n | max single constraint | initial pool | peak work BDD | peak live nodes | elim apply calls | linear scan 2^l | MITM memory 2^{2l} |
|---|---|---|---|---|---|---|---|---|---|---|
| xint_t | t_first | 3 | 11 | 238 | 1497 | 441 | 1805 | 11658 | 8 | 64 |
| xint_t | t_first | 4 | 13 | 1089 | 6850 | 1664 | 8222 | 65648 | 16 | 256 |
| xint_t | t_first | 5 | 17 | 4810 | 38826 | 13511 | 51138 | 506955 | 32 | 1024 |
| xint_t | t_first | 6 | 19 | 20909 | 176362 | 54393 | 228184 | 3534482 | 64 | 4096 |
| xint_t | x3_t_x12 | 6 | 19 | 20909 | 176362 | 54393 | 228184 | 2151356 | 64 | 4096 |
| xblk_t | t_first | 6 | 19 | 20909 | 176332 | 54393 | 228154 | 3155371 | 64 | 4096 |
| xint_t | x_first | 5 | 17 | 4810 | 38826 | 88093 | 113794 | 1901506 | 32 | 1024 |
| t_xint | t_first | 5 | 17 | 130717 | 409408 | 112119 | 519607 | 1566267 | 32 | 1024 |

Full per-cell tables for all arms: `analysis_tables.txt`; raw rows: `results.json`.

Fitted slopes, bits per l (least squares on log2 of the cell median): [pcc:unverified]

| order | schedule | sizes l | peak live | peak live / n | peak work | max single constraint | elim apply calls |
|---|---|---|---|---|---|---|---|
| xint_t | t_first | 3-6 | 2.36 | 2.08 | 2.39 | 2.15 | 2.77 |
| xint_t | x3_t_x12 | 3-6 | 2.36 | 2.08 | 2.39 | 2.15 | 2.65 |
| xblk_t | t_first | 3-6 | 2.36 | 2.08 | 2.39 | 2.15 | 2.74 |
| xblk_t | x3_t_x12 | 3-6 | 2.36 | 2.08 | 2.39 | 2.15 | 2.65 |
| xint_t | x_first | 3-5 (l=6 blow-up) | 2.99 | 2.68 | 3.45 | 2.17 | 3.50 |
| xblk_t | x_first | 3-5 (l=6 blow-up) | 2.99 | 2.68 | 3.45 | 2.17 | 3.52 |
| t_xint | t_first | 3-5 (l=6 blow-up) | 3.27 | 2.96 | 3.24 | 3.15 | 3.32 |
| t_xint | x3_t_x12 | 3-5 (l=6 blow-up) | 3.27 | 2.96 | 3.24 | 3.15 | 3.34 |
| t_xint | x_first | 3-4 only | - | - | - | - | - |

- Lowest m=3 peak-live slope over all 9 arms: 2.36 bits per l (l=3..6), against
  the MITM memory baseline slope 2 and the linear 2+1 scan baseline slope 1.
  Kill threshold was 1.8. [pcc:unverified]
- Decomposable and non-decomposable targets give the same slope (2.36 vs
  2.35-2.36 for the best arms; cell mix 5 sat / 3 unsat at l=3,4 because one
  random target happened to decompose, 4/4 at l=5,6). [pcc:unverified]
- Measured ratio of median peak live nodes to the MITM table size 2^{2l}:
  28, 32, 50, 56 at l=3,4,5,6 (best arm); to the linear scan count 2^l: 226,
  514, 1598, 3565. [pcc:unverified]
- The wall is in place before any elimination starts: a single side-A
  constraint (one bit of S3(x1,x2,t)) is already 238, 1089, 4810, 20909 nodes
  (slope 2.15) with x above t, and slope 3.15 with t above x. After reading the
  2l bits of (x1,x2) the residual is an affine function of t and essentially all
  2^{2l} of them are distinct; with t on top the residual quadratic forms in x
  are distinct per t (about 2^n = 2^{3l}). [pcc:unverified]
- Interleaved versus blocked x order differ by under 0.1% in node counts; the
  t-on-top order is about 10x larger at l=5. The `t_first` and `x3_t_x12`
  schedules give identical peaks (smallest-first conjunction makes them the
  same computation up to quantification order); they differ only in apply
  calls. The MITM-analogue schedule `x_first` is larger (it materialises the
  set U(t) of all pair sums). [pcc:unverified]
- Off-protocol extension (node cap raised from 4e6 to 24e6, order xint_t, 2
  targets, l=7, n=23): peak live 1.35e6, max single constraint 97360; fitted
  slope over l=3..7 is 2.39 (peak live), 2.16 (single constraint). Under the
  protocol cap l=7 is a blow-up during side-A construction. [pcc:unverified]

## m=2 sanity cell

| order | sizes l | peak live slope | peak live / n slope | peak work slope | max single constraint slope |
|---|---|---|---|---|---|
| blk | 4-8 | 1.29 | 1.10 | 1.43 | 1.03 |
| int | 4-8 | 1.43 | 1.24 | 1.65 | 1.04 |
| rev | 4-8 | 1.44 | 1.25 | 1.61 | 1.06 |

Lowest m=2 peak-live slope 1.29 bits per l against the MITM memory / scan
baseline slope 1 (threshold 0.8). Median peak live at l=8: 6530 (blk) against
2^l = 256. [pcc:unverified]

## Verdict against the pre-written kill criterion

Kill criterion met. The lowest m=3 peak-live slope is 2.36 bits per l (four
sizes, l=3..6), which is >= 1.8; it is above the MITM table slope (2) and far
above the linear 2+1 scan slope (1). Normalising by n (which grows with l in
this design) gives 2.08, still above the threshold. The m=2 sanity cell agrees
(1.29 >= 0.8). The prediction "peak node slope below the MITM memory slope" is
not supported at these sizes. [pcc:unverified]

## Deviations and limits

- The node cap counts nodes ever created in the manager (no garbage
  collection), not live nodes, so a "blow-up" is a cap on work, not a proof
  that the live size exceeds 4e6. Peak live itself is canonical per order.
- Peak live includes the initial pool of all 2n constraint BDDs (built
  eagerly). `peak_work` (largest intermediate result) is reported separately
  and has the same slope (2.39), so lazy constraint construction would not
  change the verdict; the single-constraint size alone (2.15) exceeds the
  threshold.
- n grows with l (smallest prime >= m l), so slopes contain a polylog factor;
  the "/ n" columns show its size.
- l=7 only off-protocol with 2 targets. m=3 cells at l=3,4 are 5 sat / 3 unsat.
- No plot or PDF (matplotlib is not installed in this container). No untreated
  SAT arm was re-run; the comparison is against counted baselines only.
- Time budget: about 40 minutes used, largest single run 49 s.

## What was not covered / what a full study would need

- Gate-level (Tseitin) or auxiliary-variable encodings in which no single
  constraint is large, with a tree-decomposition-driven elimination order; the
  direct encoding used here is dominated by single-constraint size. Known
  lower bounds for BDDs of field multiplication suggest this does not help,
  but it was not measured.
- ZDDs, complement edges, dynamic variable reordering (sifting), and-exists
  relational product, generalized-cofactor (restrict) simplification of side A
  by T(t).
- A direct S4 encoding without t, m=4, l > 7, random (non-polynomial-basis)
  subspaces V, curves with a != 0.
- A compiled BDD package (CUDD/Sylvan or Rust) to reach l=8..10, 8+ targets at
  every size, bootstrap intervals on the slope, and an untreated SAT arm on the
  same instances.

## Reproduce

```
python3 bddpdp.py validate
python3 bddpdp.py run --m 3 --l 3 --targets 8 --out raw_m3.json     # smallest cell, < 2 s
python3 bddpdp.py run --m 3 --l 3 4 5 6 --targets 8 --out raw_m3.json
python3 bddpdp.py run --m 2 --l 4 5 6 7 8 --targets 8 --out raw_m2.json
python3 bddpdp.py run --m 3 --l 7 --targets 4 --orders xint_t xblk_t --out raw_m3_l7.json
python3 bddpdp.py run --m 3 --l 7 --targets 2 --orders xint_t --scheds t_first x3_t_x12 --node-cap 24000000 --out raw_m3_l7_cap24M.json
python3 analyze.py > analysis_tables.txt
```
