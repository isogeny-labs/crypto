# SKEPTIC review — idea 5, conflict-directed polynomial lemmas

Reviewer: independent skeptic pass, 2026-11-14. One core, about 12 minutes.
Every number here is an unverified assertion `[pcc:unverified]`. The producer's
files were not edited; all re-runs were done on a copy under
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-05-conflict-directed-poly-lemmas/`.

## Outcome

The verdict **kill-criterion-met** follows from the pre-written criterion, clause
(a), for the implementation that was actually run (Glucose 4, CNF-cut XORs).
It survives completing the truncated l=4 cell: random/directed k=8 is 1.27 over
all 8 instances (bootstrap 95% 0.86..2.02), below the required 2.0.

Two of the headline's supporting figures are weaker than stated once the cell
is complete:

| figure at l=4, k=8 | producer (truncated cell) | this review (8 instances) |
|---|---|---|
| random/directed conflicts (A3/A2) | 1.24 (0.71..2.51), n=5 | 1.27 (0.86..2.02), n=8, idx 6 censored |
| untreated/directed conflicts (A0/A2) | 0.39 (0.22..0.66), n=6 | 0.56 (0.29..1.05), n=8 |
| directed slope, all targets, log2 median | 3.76 | 3.36 |
| untreated slope, all targets | 2.52 | 2.52 |

So "directed used about 2.6x the untreated conflicts" becomes about 1.8x with an
interval that reaches 1.0. The directed arm is still not better than untreated
and its slope is still higher, so the conclusion does not change.

## 1. Instance generator — checked, correct

Own field arithmetic (shift-and-reduce multiply, log/antilog tables, own
half-angle solve and affine group law); only `make_instance` was used, to obtain
b, the basis and R.

- x^13+x^4+x^3+x+1 is irreducible (x^(2^13) = x, 13 prime).
- All 22 measured instances (l=2,3,4): R is on the curve, the basis is
  independent, and every planted target's x-coordinates lie in V and lift to
  points with P1+P2+P3 = R.
- Brute force over all rational points with x in V (2-torsion point x=0
  included), all sign choices: SAT/UNSAT agrees with the solver status of every
  stored row, and with the producer's `bf_count` (my count 12 = their 6 index
  triples x 2 global signs). No mismatch in 22 instances.
- Fresh A0 solves of the 4 SAT instances at l=2: the model's x1,x2,x3, decoded
  by my own code, lift to points whose sum has x-coordinate x(R). 4/4.

Not checked independently: lemma soundness (the producer checks planted
solutions only; UNSAT statuses of lemma arms agree with brute force, which is
indirect support), and model lifting at l=3 and l=4.

## 2. Re-run — reproduced exactly

`python3 run.py 2` (6.8 s) and `python3 run.py 3` (62 s) on the copy: all 96
rows match the stored rows field for field (status, conflicts, decisions,
propagations, rounds, lemma counts, generator word-XORs). l=4 was not re-run in
full; the missing l=4 runs were added instead (next section).

## 3. Fairness

**Same instances and seeds:** yes. Each arm builds a fresh solver on the same
instance; the solver is deterministic. Proof logging is on only in A2, but A2
equals A0 conflict-for-conflict whenever no lemma is added, so logging does not
perturb the search.

**Truncated deciding cell — completed here.** The criterion says "all instances
of the largest l cell"; the producer had 5 of 8 for A3/A2. I ran the missing
k=8 runs with the producer's `run_arm`, B=9500, 150 s cap:

| l=4 idx | truth | A0 | A2 k=8 | A3 k=8 | A3/A2 |
|---|---|---|---|---|---|
| 6 | UNSAT | 117139 (stored) | 862349 (stored) | TIMEOUT at 1380649 | > 1.60 |
| 3 | SAT | 32863 | 42242 | 28768 | 0.68 |
| 7 | UNSAT | 472664 | 143751 | 305737 | 2.13 |

idx 6 A3 is censored, so 1.27 is a lower bound. For the geometric mean to reach
2.0 the idx 6 random-support run would need about 5e7 conflicts (60x the
directed run). That is not formally excluded. Without idx 6 the ratio is 1.23
(0.79..2.02), n=7.

**Stopping rule.** The run stopped at the wall-clock budget in a fixed,
pre-declared instance order. I found no sign that the stop was chosen on the
data, and the completed cell gives the same answer.

**Clause (b) is misreported.** The protocol's clause (b) is "directed slope
within 0.25 bits/l of untreated". 3.76 (or 3.36) against 2.52 is not within
0.25; it is higher. The README says the criterion is met "on both clauses".
Read literally only clause (a) is met. The "alive" condition (slope lower by
at least 0.25) fails either way, so this is a wording error, not a change of
verdict.

**Slope.** Three sizes, as required, but weak: at l=2 five of eight instances
finish before any lemma is added, so the treatment is a no-op there, and the
slope is carried by the l=4 cell. Slopes move by 0.4 bits/l when two instances
are added (3.76 to 3.36), so differences of that size between arms are not
resolved. Per-instance ratios range from 0.14 to 4.4.

**SAT/UNSAT mixing.** In every cell planted = SAT and random = UNSAT, without
exception, so "all targets" is a fixed 4+4 mix at l=2,3 but 3+3 or 3+2 at l=4
in the producer's data. The producer reports the split tables. The split does
not rescue the idea: with the cell complete, A3/A2 k=8 at l=4 is 1.5 on the
four SAT instances and about 1.1 on the four UNSAT ones.

**Baselines.** The reference counts (enumeration 2^(3l), MITM, linear 2+1 scan
2^l) are stated and every SAT arm is far above them: 76090 median conflicts
against 4096 triples at l=4. The untreated baseline is a weak solver
configuration (no native XOR), which cannot favour a kill verdict on clause
(a), since that clause compares two lemma arms.

**Uncounted work.** Lemma generation is counted (word XORs) and reported beside
conflicts, not folded into the metric. A3 may run the generator up to five
times per round; that is counted. Brute-force validation and calibration are
outside the metric, correctly.

**Protocol before results.** File times are consistent with the claim:
`pdp5.py` 08:28, `PROTOCOL.md` and `run.py` both 08:33:50, first measured log
finished 08:34:59. The addendum was written after calibration on idx 900-902
and it discloses that the k=12 pair was added because of one calibration
instance. Calibration output (`calibration.json`) is not in the directory, so
the calibration figures in the addendum cannot be checked.

**Arms reported.** All six arms that ran are in the tables. The k=12 pair
points the other way from the hypothesis (A3/A2 = 0.38 at l=4, n=5) and is
reported.

## 4. Does the verdict follow?

Yes, for clause (a), on the point estimate the protocol specifies. Limits on
what the kill covers:

1. It is a kill of this implementation, not of the idea as written. The idea
   text asks for pycryptosat with XOR clauses; the run used Glucose 4 with
   lemmas of mean length 30 cut into arity-4 CNF pieces. Long XORs in CNF are
   known to propagate poorly, so a native-XOR solver could behave differently.
   The producer states this deviation.
2. The conflict signal is variable frequency in a round's learnt clauses, with
   rounds overrunning B because Glucose checks the budget only at restarts.
3. One n, one encoding, one solver seed, 8 instances per cell. The upper
   interval bound (2.02) touches the threshold.

## Not done in this review

- l=4 k=12 runs for idx 3, 6, 7 and the A1 runs for idx 3, 7.
- A noise-floor (A/A) arm, for example clause-order shuffles of A0.
- Repository session log, research-kb lookup and Conductor task: this review
  was restricted to writing this one file.

## Files

- Review scratch (not in the repository):
  `/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-05-conflict-directed-poly-lemmas/indep.py`
  (independent checks),
  `/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-05-conflict-directed-poly-lemmas/complete.py`
  and
  `/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-05-conflict-directed-poly-lemmas/complete_l4.jsonl`
  (the seven added l=4 runs).
