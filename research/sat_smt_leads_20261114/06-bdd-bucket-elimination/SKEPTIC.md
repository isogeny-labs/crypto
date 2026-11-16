# Skeptic review: idea 6, BDD bucket elimination for the PDP

Date: 2026-11-14. Independent review, one core, about 15 minutes. Every number
here is an unverified reviewer assertion ([pcc:unverified]). Scratch scripts:
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-06-bdd-bucket-elimination/`
(`indep.py`, `chk2.py`, `chk3.py`, `rerun_m3_l3.json`).

## Outcome

- The kill criterion, as pre-written, is met: lowest m=3 peak-live slope 2.36
  over four sizes, threshold 1.8. The stated verdict follows from the protocol.
- The verdict is weaker than the headline suggests. The slope 2.36 depends on
  the protocol's choice to grow n with l (n = 11, 13, 17, 19). With n held
  fixed, the same best arm gives a peak-live slope of about 1.45, which is
  below the 1.8 threshold. See "Fixed-n check" below.
- The kill still holds in substance: at every size the BDD peak is 28x to 175x
  the MITM table, and a single side-A constraint has slope 1.94 to 2.1 at
  fixed n. The claim "above the MITM slope of 2" is not established; "no
  better than a table" is.

## 1. Instance generator and answers (independent check)

Own code, no producer functions: F_{2^11} with log tables, all curve points by
brute force over y, own addition law (associativity and group order 2116
checked), factor base x in {0..7} (13 points, x=6 has none).

For the 8 targets of the m=3, l=3 cell (xR taken from `results.json`):

| check | result |
|---|---|
| Planted targets decompose into three factor-base points | 4 of 4 |
| Point-level answer equals producer `truth_sat` | 8 of 8 |
| Algebraic solution count (exhaustive over all t) equals `truth_solutions` | 8 of 8 (6 or 0) |
| Algebraic solutions that do not lift to points | 0 |
| All 72 result rows' `sat` equals my point-level answer | yes |
| Producer BDD solution sets (3 orders) equal my point-level sets | 24 of 24 |
| `planted_x` is in my point-level set | 4 of 4 |

The generator and the ground truth are correct on this cell. I did not check
larger cells independently.

One oddity: target 2 (random) and target 6 (planted) have the same xR = 435
(R and -R). The l=3 cell therefore has 7 distinct instances, 4 distinct
decomposable ones. Every decomposable target in the study has exactly one
decomposition (6 ordered triples).

## 2. Re-run

`python3 bddpdp.py validate` passes (2 s). The rerun command for m=3, l=3
produced 72 rows; every field except wall times is identical to the matching
rows in `results.json` (0 differences).

## 3. Fairness

| item | finding |
|---|---|
| Same instances and seeds | Yes. Seeds match the protocol formula; baselines are counted, so no instance mismatch is possible. |
| Strongest cheap baseline | MITM memory 2^{2l} is the criterion's baseline and is used. The 2^l linear scan is a time count, not memory; the "peak nodes / 2^l" ratios in the README compare unlike quantities. A 2^l-memory MITM (table on x3, enumerate pairs) exists and would make the BDD look worse, not better. |
| Uncounted work | None that helps the treatment. Peak live includes the eagerly built pool; `peak_work` is given separately. Side-A build cost is reported separately from elimination calls. |
| At least 3 sizes | Yes for the four best arms (l=3..6). Three arms have 3 sizes, one has 2 and is not fitted. |
| Decomposable vs not | Reported separately; slopes agree (2.36 vs 2.35). Mix is 5/3 at l=3,4 and 4/4 at l=5,6. Not a concern. |
| All arms reported | Yes: 9 m=3 arms and 3 m=2 arms, including blow-ups and the off-protocol l=7 run, which is labelled as such. |
| Protocol before results | Cannot be confirmed. No git history. File times: `PROTOCOL.md` 08:46:33, `bddpdp.py` 08:48:19, first raw output 08:48:56. A 30 kB script existed in some form before the protocol's final save, so development runs before the protocol are possible. The protocol's cap (4e6) and "l=6 attempted" read as informed by such runs. The criterion itself (1.8) comes from the task text, so tuning of the threshold is not a concern. |
| n grows with l | This is the main weakness; see below. |

## Fixed-n check (reviewer, producer's code, order xint_t)

The protocol sets n = smallest prime >= 3l, with uneven steps (+2, +4, +2).
Local slopes in the producer's data swing with those steps (2.19, 2.64, 2.16).

Producer's medians, slope after dividing by n^k:

| metric | k=0 | k=1 | k=2 |
|---|---|---|---|
| peak live | 2.36 | 2.08 | 1.81 |
| peak work | 2.39 | 2.11 | 1.84 |
| max single constraint | 2.15 | 1.88 | 1.60 |

The pool is 2n constraints, each with an n-dependent tail, so n^2 is a
defensible normalisation for peak live; it lands on the threshold.

Holding n fixed (2 planted + 2 random targets per new cell, schedule
`x3_t_x12`; the largest l in each row is the producer's own cell):

| n | sizes l | peak live slope | peak work slope | single constraint slope |
|---|---|---|---|---|
| 19 | 3-6 | 1.45 | 0.97 | 1.94 |
| 17 | 3-5 | 1.47 | 0.94 | 2.11 |
| 17, schedule `x_first` | 3-5 | 2.05 | 2.29 | 2.11 |

Medians at n=19: peak live 11208, 28719, 79765, 228184 for l=3..6, against
MITM tables of 64, 256, 1024, 4096.

Reading: at fixed n the best arm's slope is below 1.8, so a fixed-n protocol
would not have fired the kill criterion on slope. The low slope comes from a
large l-independent overhead (conjoining parity constraints over the n bits of
t costs roughly 2^{n/2} at small l), not from beating the table: the ratio to
the table is 175x at l=3 and 56x at l=6. The single side-A constraint is 5 to
6 times 2^{2l} at every l, and every arm holds it in memory, so peak live is
bounded below by a table-sized object in this encoding. I did not run l > 6
at fixed n, so where the fixed-n slope settles is not measured.

## 4. Does the verdict follow

Yes under the pre-written criterion and design: 2.36 >= 1.8 on four sizes, and
blow-up did not prevent three sizes. Corrected verdict: kill-criterion-met.

Qualifications the headline should carry:

- The fitted 2.36 is not a clean exponent. It includes a polylog factor from
  growing n; the excess over the MITM slope of 2 is not established.
- The margin over 1.8 disappears under n^2 normalisation (1.81) and reverses
  at fixed n (1.45). The kill rests on absolute size and on the
  single-constraint floor, not on the slope margin.
- The kill is for one encoding (direct ANF, one BDD per output bit, split
  through t). Orders with t interleaved among x bits, restrict-by-T, and
  gate-level encodings were not run. A rough count suggests restrict-by-T with
  t on top is still about 2^{2l} per conjunct, but that is an estimate, not a
  measurement.
- l=6 completes for only 4 of 9 arms under the protocol cap; the cap counts
  nodes created, not live nodes.
- m=2 sanity cell uses n = 11, 11, 13, 17, 17, so its slopes carry the same
  confound. It does not decide the verdict.

## Process notes

- No session-log entry was written: this review was restricted to this one
  file. The orchestrator should log it.
- The producer's files were not modified (imports ran with `python3 -B`).
