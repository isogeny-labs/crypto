# SKEPTIC review — Idea 9, solution-rich regime (2026-11-14)

Independent review, one core, about 15 minutes. Every number here is
`[pcc:unverified]` pilot data. Scratch (scripts and rerun rows):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-09-solution-rich-regime/`
(`indep.py`, `heldout.py`, `rows_n7_rerun.jsonl`). No producer file was edited.

## Conclusion

The stated verdict, **kill-criterion-met**, follows from the pre-written
criterion and I could not break it. On fresh targets the result is stronger
than reported: the one cell below enumeration (n=13, l=5, 0.549x) does not
survive out of sample. The verdict covers this chain encoding with free t at
n <= 13 only, as the producer says.

## 1. Instance generator and encoding (own code, n=7, all four l)

My own F_{2^7} arithmetic, point enumeration from the curve equation, and
group law (checked: group order 142, associativity, closure, order * P = O on
300 samples). The producer's `Enc` was used only as the object under test.

| Check | Result |
|---|---|
| Seed reproduces the stored target x(R), and it is a rational x | 96/96 |
| Producer `nsol`, `k3`, `k2` equal my brute force over all point triples | 96/96 each |
| Decomposable with t finite equals decomposable in full (no chain-only loss) | 96/96 |
| Glucose SAT/UNSAT equals brute-force decomposability | 96/96 |
| CNF with all coordinate bits fixed is SAT iff some t satisfies both S3 equations (my arithmetic), l=2,3, 12 targets | 3456/3456 |
| My own planted targets (l=2,3,4): SAT and model lifts | 59/60 |
| First model does not lift | 1 measured (l=3, target 21), 1 planted; both (x1,x2,x3) = (1,1,7) |

The non-lifting model is the twist class the producer described (x1 = x2 = 1,
t = 0). Both targets are truly decomposable, so SAT/UNSAT stays correct; the
producer does not count such a row as a relation, which is slightly against
the SAT arm (22 relations instead of 23 at n=7, l=3).

## 2. Rerun of the smallest cell

`python3 run.py 7 24 <scratch>/rows_n7_rerun.jsonl 1 2 3 4` took 1.1 s.
456 rows, 0 differ from `rows_n7.jsonl` and from the n=7 rows in
`results.json` (all fields except wall time). `results.json` rows equal the
three jsonl files (932 rows). My own recomputation of every cell from the raw
rows reproduces the README table: per-relation costs, both baselines, best
budget and B, 15 above / 2 below for streamlining, and the ratios (11.7x to
313x against the linear scan, 2.6x to 17.7x and 0.549x against enumeration).

n=11 and n=13 were not rerun (about 45 minutes); they were recomputed from
rows and tested on fresh targets below.

## 3. Fairness

- Same instances for all arms: confirmed per cell (same target ids and x(R)
  in every arm). Baselines come from the same targets' ground truth.
- Baseline strength: the linear 2+1 scan with early exit is the right cheap
  baseline. It is not tuned upward; scanning only the x3 that lift to a point
  would roughly halve it. MITM is stated but not tabulated; it would not
  change the direction.
- Selection favours the SAT side, not the baseline: best arm is a minimum
  over six arms, and B is chosen on the scored targets. The kill is met anyway.
- Decomposable and non-decomposable targets are pooled the same way in every
  arm (total cost over all targets / relations).
- Every arm in the rows is reported, including CaDiCaL and the budget check.
- Slopes use 3 values of l at n=7 and n=11, none at n=13, so cross-n scaling
  rests on two sizes. The verdict does not depend on slopes.

Held-out test of the budget arm (mine; fresh seeds offset by 500000, exact-cut
cost recovered from real `conf_budget` runs, Glucose being prefix-consistent):

| Cell | Fresh targets | Producer's B: cost/rel | Best B in sweep: cost/rel | Best / linear scan | Best / enumeration |
|---|---|---|---|---|---|
| n=13, l=5 | 400 | B=128: 0 relations | B=512: 50,879 | 1,209x | 41x |
| n=11, l=6 | 300 | B=64: 9,588 | B=1024: 5,859 | 2,157x | 118x |
| n=11, l=5 | 300 | B=128: 5,429 | B=128: 5,429 | 436x | 17.6x |
| n=7, l=3 | 400 | B=4: 74.2 | B=8: 52.2 | 29x | 7.4x |
| n=7, l=4 | 400 | B=8: 42.7 | B=4: 35.7 | 26x | 8.8x |

With B chosen on the first half and scored on the second, the ratios to the
linear scan are 1,315x, 2,996x, 686x, 25x and 26x. So the producer's in-sample
budget figures (1,008 at n=13, l=5; 821 at n=11, l=6; 1,997 at n=11, l=5) are
low by 3x to 50x, and the 0.549x cell is a one-relation artefact. The "best
SAT / linear scan" range of 11.7x to 313x is therefore optimistic for SAT;
held out it is about 25x to 3,000x.

## 4. Problems

1. Protocol-before-results cannot be confirmed. All files carry copy-time
   mtimes (09:15 to 09:21; `rows_n11.jsonl` took about 25 minutes but is dated
   5 minutes after `PROTOCOL.md`), and there is no git history. The kill
   criterion matches the idea text given to the workflow, so this does not
   move the verdict.
2. Amendment 1 was written after conflict counts had been seen on probe runs
   (n=13 and n=17), and it shrank the study from n in {13,17,19} to {7,11,13}.
   The kill criterion was not changed.
3. Measured runs began before validation had passed (producer's own
   deviation). My independent check at n=7 finds the encoding correct.
4. Coverage is thin: 7 relation-producing cells, n=13 has one (8 targets,
   3 decomposable, 3 censored runs), n=11 l=6 has 13 of 16 targets, and
   n=13 l=6,7 from Amendment 1 were not run. The SAT-optimal l is at the sweep
   edge in both n, so P1 is untested rather than confirmed.
5. Budget arm is in-sample and idealised; see the held-out table. The README
   says so, but the headline ratios still use it.
6. Streamlining used one random draw per target, so the two sub-1 ratios
   (0.74x, 0.50x) are single-draw noise candidates.
7. The primary criterion is close to unbeatable in raw units at high density:
   the linear scan needs 1.4 to 2.7 leaf solves per relation at l >= n/2,
   while the median SAT run needs 32 (n=7) to 14,309 (n=11) conflicts. A
   conflict and a GF(2) leaf solve are compared one for one.
8. Scope: one weak encoding (refutation cost grows about 0.7 to 0.8 bits per
   bit of n at fixed l), no t-free or XOR-aware arm. The kill applies to this
   encoding and these sizes, not to SAT in the solution-rich regime generally.

## 5. Does the verdict follow?

Yes. K-lin: no cell has any SAT arm below the linear-scan count; the smallest
margin is 11.7x in sample and about 25x held out. K-enum: the single cell
below enumeration is not reproduced on 400 fresh targets (41x above), and the
protocol already ruled that below-enum without below-lin counts as killed.
P2 is not supported. P1 is not bracketed.
