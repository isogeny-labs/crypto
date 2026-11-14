# Skeptic review: idea 1, proof-size slope as a go/no-go gate

Reviewer: independent skeptic session, 2026-11-14. About 15 minutes, one core.
Every number here is an unverified reviewer assertion [pcc:unverified]; no pcc
claim, benchmark record or leaderboard row was made. I wrote only this file.
Scratch: `/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-01-proof-size-slope/`
(`indep.py`, `chk.py`, `rup.py`, `a2.jsonl`, `a3.jsonl`, `a4.jsonl`).

## Verdict

The producer's verdict, kill criterion met, follows from the pre-written
criterion. It holds on every subset of sizes I tried and survives the two
censoring corrections below. Its scope is narrow, as the producer says: it
kills "backward trimming of CDCL-found DRAT proofs exposes short refutations
on this pilot CNF", not "no short refutation exists".

- Reproduced: yes for l = 2, 3, 4 (CaDiCaL rows bit-identical); l = 5 not re-run.
- Verdict supported: yes. Corrected verdict: kill-criterion-met.

## 1. Instance generator (own code, n = 17, l = 2, all 12 targets)

`indep.py` uses its own field arithmetic (log tables, cross-checked against
carry-less multiply and reduce), its own curve group law, y found by
exhaustive search, and exhaustive search over all 2^17 values of X. It takes
only the CNF from the producer's `pdp.encode`.

| check | result |
|---|---|
| Field polynomial is irreducible (t has order 2^17 - 1) | pass |
| x(P+Q) satisfies S3(x(P), x(Q), x(P+Q)) = 0 on random points | pass |
| CNF model set projected on (x1,x2,x3), enumerated with MiniSat 2.2, equals my brute-force relaxed solution set | 12 of 12 |
| Every model's (x1,x2,x3,X) satisfies both S3 equations in my arithmetic | 12 of 12 |
| Row truth and `n_solutions_ordered` equal my brute force | 12 of 12 |
| The 8 UNSAT targets have no decomposition into rational points with x in V (all sign choices) | 8 of 8 |
| The 4 planted targets have a true rational decomposition | 4 of 4 |

Two findings, neither affecting the UNSAT rows:

- The chained encoding misses true decompositions whose intermediate point is
  at infinity. For the planted target x(R) = 1 there are 4 true ordered
  decompositions but only 3 relaxed solutions, and one true decomposition is
  not a model. The PROTOCOL.md statement "UNSAT in this relaxation implies
  non-decomposable" is therefore false in general. It holds for random targets
  with x(R) outside V, which is the case for every UNSAT row I checked.
- At l = 2 only x = 0 and x = 1 in V are x-coordinates of rational points, so
  the 4 planted SAT rows are 2 distinct targets, each twice (x(R) = 1 and
  76801). The l = 2 SAT cell has 2 independent rows, not 4.

Trimmer (`rup.py`): for the last l = 2 Glucose core, my own Python forward
checker found all 336 core lemmas RUP in order against the 855 core originals,
the empty clause follows, and the core originals are UNSAT under MiniSat 2.2.
I did not compare the trimmer with drat-trim, so whether a reference trimmer
would give smaller cores is unknown. That bias, if any, runs toward kill.

## 2. Re-runs

- Producer's command, (17, 2), both solvers: 24 rows, 0 mismatches against
  `results.json` on x(R), b, truth, conflicts, decisions, proof lemmas, T1 and
  T2 core, core literals, variables and clauses. Run time 0.6 s.
- (17, 3), CaDiCaL, T1 only: 8 of 8 rows identical in conflicts and T1 core.
- (17, 4), CaDiCaL, T1 only: the 6 stored rows are identical. The full cell of
  8 took 21.6 s.
- (17, 5): not re-run (one trim pass is reported at 100 to 170 s per target).

## 3. Fairness of the comparison

| item | finding |
|---|---|
| Same instances and seeds for both arms | Yes. T1 trims the proof of the same solve; seeds follow the stated formula in all 99 rows. |
| SAT and UNSAT mixed | No. Fits filter on truth = UNSAT; SAT rows are fitted separately. |
| At least 3 sizes | Yes: 4 sizes (l = 2..5), and 3 sizes for the l = 3..5 fit. |
| Strongest cheap baseline | The count baselines (scan 2^l, MITM 2^(2l), tuples 2^(3l)/6) are reported as references only. The kill threshold 1.7 is cleared by about 1.6 bits/l, so the baseline choice does not move the verdict. |
| Uncounted work | Core lemma count ignores lemma length. Core literals grow faster (3.79 bits/l), which makes the trimmed arm look worse, not better. Trim time (100 to 170 s against 17 to 35 s solve at l = 5) is not charged; it would also hurt the treatment. |
| Every arm reported | Yes, with gaps stated: Glucose stops at l = 4 (1 row), T2 has 1 row at l = 5, T3 covers l = 2, 3. Glucose SAT rows are in `results.json` but not in the README table. |
| Protocol before results | PROTOCOL.md mtime 07:55:27, first raw rows 08:05:59. The directory is untracked in git, so file times are the only evidence. |

Problems with the record:

1. The primary series was changed after a cost probe (n = 23, l = 3..7 became
   n = 17, l = 2..5) and PROTOCOL.md was not amended for it; only README.md
   records it. The probe rows were discarded, so I cannot confirm the choice
   was made before proof-size data existed. Adding l = 2 lowers the trimmed
   slope (3.27 against 3.55 without it), so the change favours "alive", not
   kill.
2. The l = 4 censoring is described as time censoring, but the two missing
   CaDiCaL rows (targets 4 and 6) take seconds. I ran them: 35,939 and 33,234
   conflicts, T1 cores 28,797 and 26,648. They are the two largest l = 4 rows.
   The likely cause is the Glucose run in the same process, not CaDiCaL cost.
   With the complete l = 4 cell the slopes are unchanged: conflicts 3.345,
   T1 core 3.275 (l = 2..5); 3.414 and 3.553 (l = 3..5).
3. The l = 5 censored row (target 0) is also the largest. Adding it with the
   producer's scratch figures (core 405,370; conflicts estimated from proof
   lemmas) gives conflicts 3.37, core 3.30, difference 0.07.
4. The headline range "core keeps 67-102% of the conflict count" is over cell
   geometric means. Per instance the range is 55% to 147%.
5. The pilot CNF is weaker than enumeration (conflict slope 3.34 against 3 for
   tuples), so the result says little about the repository's encodings.
6. One-step slopes are noisy at the small end: core lemmas gain 2.61, 3.61 and
   3.48 bits per step of l, conflicts 3.21, 3.25 and 3.58. The l = 2 cell is
   close to a constant-dominated regime.

## 4. Does the verdict follow from the kill criterion?

Criterion: trimmed slope within 0.3 bits/l of the conflict slope, and above
1.7 bits/l, on the primary series, with at least 3 sizes and no bootstrap
interval straddling a threshold. My recomputation (own script, paired
bootstrap within cells, 2000 draws):

| subset (series A, CaDiCaL, UNSAT) | rows | conflicts | T1 core | difference | paired 95% of difference | T1 core 95% |
|---|---|---|---|---|---|---|
| l = 2..5 | 29 | 3.34 | 3.27 | 0.07 | [-0.01, 0.15] | [3.14, 3.38] |
| l = 3..5 | 21 | 3.41 | 3.55 | -0.14 | [-0.24, -0.03] | [3.29, 3.78] |
| l = 2..4 | 22 | 3.23 | 3.08 | 0.15 | [0.01, 0.28] | [2.88, 3.26] |
| l = 4..5 | 13 | 3.58 | 3.48 | 0.09 | [-0.02, 0.20] | [3.30, 3.68] |
| l = 2..5, T2 (1 row at l = 5) | 23 | 3.25 | 3.13 | 0.12 | [0.02, 0.22] | [2.96, 3.27] |
| Glucose, l = 2..4 | 17 | 4.95 | 4.92 | 0.03 | [-0.01, 0.07] | [4.20, 5.74] |
| series B, l = 3..5 | 23 | 3.55 | 3.40 | 0.15 | [0.12, 0.18] | [3.20, 3.58] |

The producer's point estimates match mine. No interval reaches 0.3 for the
difference or 1.7 for the trimmed slope, so both kill conditions hold and the
"inconclusive" clause does not apply. The verdict is correct as stated.

What it does not settle (the producer says the same): trimming removes unused
lemmas from a proof the solver found and cannot find a different, shorter
proof. T3, the only real minimisation, ran at two sizes and gives no slope.
Whether the 2^l linear 2+1 certificate has a short clausal form was not
tested, and that is the question the idea actually asks.

## Not checked

- l = 5 rows, Glucose rows above l = 2, series B rows, T3 rows: not re-run.
- Generator checks at l >= 3 and at n = 11, 13: not done independently.
- No reference trimmer (drat-trim) or LRAT check.
- No research-kb, ecdb or session-log entry: writes were limited to this file.
