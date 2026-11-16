# Skeptic review of lead E (scan certificate as a checked proof)

2026-11-15, independent session. Everything below is `[pcc:unverified]`: a local review at toy
sizes, one niced process, about 1.5 CPU minutes. No producer file was edited. My scripts and logs
are outside the repository, in
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-E/`
(`skeptic.py`, `trim.py`, `skeptic.log`, `rerun.log`, `e_f13_2.jsonl`).

## Verdict

**Weaker than stated, but the core result holds.**

- The `xproof` refutations are sound refutations of the original CNF. I found no way for a
  definition to constrain existing variables.
- The smallest cell reproduces exactly, and the 1.39 slope recomputes from the raw rows.
- "Short proofs exist" follows only from the rule as restated in amendment 1, and only with size
  counted in clauses. In literals the slope is 1.48 with an interval that crosses the 1.5
  threshold.
- By the rule in the original protocol text (scan-guided CDCL proofs, no new variables) the
  decision is "not shown".
- Whether amendment 1 was written before the measured data cannot be established from file times.

## 1. Soundness of the extension definitions

**Reading the generator.** `XProof.define` always allocates `nv + 1` and records `(d, a, b)` with
`a`, `b` already in use. Definitions are the only clauses added after `__init__`. So the
generator cannot emit a definition that constrains old variables.

**Weakness in the producer's check.** `check_defs` is given `nv0` and the CNF clause count by the
generator itself. It checks the definition block but never checks that the "CNF" part is the
instance. In my tamper test, an extra unit clause placed in the CNF part with the count bumped
by one is accepted by `check_defs`. The producer's pipeline does not do this, but the check
would not catch it.

**Independent checker** (`my_check_formula` in `skeptic.py`). It accepts a formula only if:

- its prefix equals `System.render("dual")` (ORs, gates, rails);
- everything after is a rail XOR, a 4-clause group `d = a xor b` with `d` equal to the largest
  variable seen so far plus one and `a != b < d`, or a unit clause;
- the unit clauses, expanded through the definitions down to base variables, equal the multiset
  of `System.rows` with their right-hand sides.

This covers both the prefix-chain variables inside the producer's CNF and the proof's extension
variables.

| check | (13,2) t0 | (17,2) t0 | (17,3) t0 | (13,4) t0 |
|---|---|---|---|---|
| my formula/definition check | ok, 1 315 defs | ok, 2 355 | ok, 4 801 | ok, 6 401 |
| every proof variable defined | yes | yes | yes | yes |
| `rup_dep` on the raw proof | VERIFIED | VERIFIED | VERIFIED | VERIFIED |
| my forward RUP (MiniSat propagation, every lemma in order) | pass, 16 681 lemmas | pass, 31 721 | pass, 73 526 | pass, 116 620 |
| Glucose 4 on the CNF without definitions | UNSAT | UNSAT | UNSAT | UNSAT |

Trimmed proofs (first-pass core) also pass my forward check for (13,2), (17,3) and (17,5)
target 0, both against the full formula and against the core formula alone.

Not checked by me: the other 44 `xproof` proofs. They have the producer's `rup_dep` verdict and
`check_defs` only.

**Tamper tests** on the (13,2) formula: a smuggled unit, a "definition" of an existing variable,
`d = a xor b xor 1`, a variable defined twice, an extra row. My checker rejects all five.
`check_defs` rejects all five with the honest clause count, and accepts the extra row when the
count is bumped (the weakness above).

**Adversarial test with decomposable targets.** 103 targets (planted, plus random ones that
happen to decompose) across (13,2), (11,3), (17,3), (17,4), (13,3):

- `xproof.measure` returned `NO_PROOF` / engine verdict SAT on 103 of 103.
- A forged proof (the partial lemma stream plus an empty clause) was rejected by `rup_dep` on
  103 of 103, and by my forward check on the 30 I ran it on.
- A second forgery (all leaf clauses plus the empty clause, no reasoning) was rejected on 103
  of 103.
- Not completed: replaying a non-decomposable target's proof on a planted target's CNF. The
  variable counts differ, so all 10 attempts were skipped.

## 2. Ground truth and instance fidelity (own field and curve arithmetic)

I wrote separate field multiplication, a table-based quadratic solver and point addition, using
the `poly`, `B`, `xR`, `yR` stored in the rows.

- All 49 `xproof` targets lie on the curve, have x(R) outside V, have no u = 0 event, and are
  not a sum of three signed base points. This used a pair table, plus full triple enumeration
  for l <= 3.
- One of them, (13,4) target 0, is a sum of **two** base points. It is non-decomposable for
  exactly three summands only. This does not affect any reported number.
- Per-leaf agreement between a SAT solver and my enumeration, on both the `xproof` CNF and the
  CDCL arms' CNF:

| cell | targets (measured / random / planted) | leaves per CNF | mismatches | models found | models that lift |
|---|---|---|---|---|---|
| (13,2) | 6 / 30 / 20 | 224 | 0 | 39 | 39 |
| (11,3) | 6 / 20 / 20 | 368 | 0 | 46 | 46 |
| (17,3) | 6 / 6 / 8 | 160 | 0 | 14 | 14 |

A model "lifts" when its (x1, x2, x3) are x-coordinates of rational points in V \ {0} whose
signed sum is R. The CNF being refuted is the real decomposition instance at these sizes.

## 3. Reproduction and slope

**Rerun.** The producer's command for (13,2) gave 30 rows identical to `results.json` in every
field except wall times.

**Slope**, recomputed from `raw/*.jsonl` (least squares on per-target log2, bootstrap over
targets):

| series | metric | slope | interval | per-step increments |
|---|---|---|---|---|
| n = 17, l = 2..5 (6 targets each) | clauses (lemmas + definition clauses) | **1.393** | 1.356..1.431 | 1.34, 1.46, 1.36 |
| n = 17, l = 2..5 | lemmas only | 1.400 | 1.361..1.439 | |
| n = 17, l = 2..5 | **literals** (lemma literals + 3 per definition clause) | **1.483** | 1.444..1.523 | 1.44, 1.55, 1.44 |
| n = 17, l = 2..5 | lemma literals only | 1.507 | 1.467..1.549 | |
| n = 17, l = 2..5 | raw (untrimmed) clauses | 1.336 | 1.299..1.373 | |
| n = 17, l = 3..5 | clauses | 1.409 | 1.389..1.430 | |
| n = 17, l = 2..6 (l = 6 is one target) | clauses | 1.391 | 1.358..1.426 | last step 1.37 |
| n = 13, l = 2..4 (post hoc) | clauses | 1.429 | 1.394..1.464 | 1.29, 1.57 |
| n = 13, l = 2..4 (post hoc) | literals | 1.554 | 1.519..1.588 | |

Is 1.39 a fair "bits per l"?

- The unit is stated (clauses) and the number is right in that unit.
- It is metric-dependent. In literals the n = 17 slope is 1.48 and its interval crosses 1.5; in
  lemma literals it is 1.51; at n = 13 it is 1.55. The README says literal counts are reported,
  but none are given for `xproof`, and the margin against the 1.5 threshold disappears there.
- It rests on four sizes at one n, 6 targets each from one curve per cell. The l = 6 point is a
  single target and adds nothing to the fit. The intervals reflect target spread only, not
  curve or n.
- Size per leaf grows by about 0.4 bits per l (4 783 to 10 664 clauses per leaf from l = 2 to
  5) with no sign of flattening. The increments are steady at about 1.35 to 1.45, not
  approaching 1.
- The secondary series (n growing with l) is 2.30. The README does say so.
- In absolute size the certificate is 50 to 100 times larger than the CDCL proofs at l = 2 and
  only reaches parity at (17,4). "Short" means a scaling statement at fixed n, nothing more.

Minor: the reported `final` size is the core marked during the re-check of the trimmed proof,
not the trimmed proof that was checked as given. It is 0.1 to 0.5 % smaller, e.g. 9 641 against
9 653 at (13,2). The effect on the slope is negligible, but it departs from "size of a proof the
checker verified as given".

## 4. Protocol timing, the rule, and the sharing statistic

**Amendment 1.** `PROTOCOL.md` has a single modification time, 07:54:03Z, which is the time of
amendment 2. The first measured rows (`raw/p17_2.jsonl`, `p17_3.jsonl`) are from 07:42 to 07:43.
So file times cannot show that amendment 1's text predates the data. What they do show:

- `xproof.py` (07:41:50) and `run.py` (07:42:22) were last saved before the first row file
  (07:42:26), so the arm existed before measurement.
- The gap is 36 seconds, and `xproof.py` was debugged on the seed-999 probe (`dbg.xp` in the
  producer's scratch, 07:41:13).
- The amendment's prediction for the new arm (1.0 to 1.4) was met at 1.39, at the edge.

Treat "before any measured run" as unverified.

**Amendment 2** states it was written after data. Its timing is consistent with the file times.

**Does "short proofs exist" follow from the pre-written rule?**

- The original rule covers the scan-guided CDCL arms. There the outcome is "not shown" (two
  complete sizes at n = 17; about 3 bits per l at n = 13), and the original prediction of 1.0
  to 1.5 for those arms failed.
- The positive decision exists only because amendment 1 added a second proof system and applied
  the same threshold to it. Under that restated rule, in clauses, it is met: 1.39, upper end
  1.43, 24 of 24 verified.
- Fair wording: "a checked extended-resolution certificate with clause-count slope 1.39 at
  n = 17 exists; rule met as amended, in clauses; not met by the original arms; marginal in
  literals".

**Sharing.**

- It is computed on trimmed proofs: the T2 proof in `run.py`, and the smallest trimmed
  candidate in `min3.py`, with dependencies taken from that proof's own check.
- The dependencies are the antecedents of one conflict analysis under the checker's core-first
  propagation order. The statistic is therefore checker-dependent and is not a minimum.
- The rule cell was moved from l = 4 to l = 3 by amendment 2, after data. At the post hoc
  (13,4) cell the median of medians is 5.5, above the threshold of 4. "No cross-leaf sharing
  shown" depends on that move. The README reports both values, which is the honest reading:
  undecided.

## Problems, in order of weight

1. The positive decision comes from an arm and a rule added by amendment, and the amendment's
   pre-data timing is not evidenced.
2. The slope depends on the size unit: 1.39 in clauses, 1.48 in literals with the interval
   crossing 1.5.
3. One fixed n, four sizes, one curve per cell. Per-leaf cost grows steadily with l, and the
   series with n growing has slope 2.30.
4. `check_defs` trusts generator-supplied boundaries. Soundness holds for the proofs I checked
   independently (4 raw, 3 trimmed), not by the producer's check alone.
5. 44 of 49 `xproof` proofs rest on the in-repo checker only.
6. The sharing decision depends on a rule cell moved after data, and the two cells disagree.
7. The reported trimmed size is not the size of the proof checked as given (under 0.5 %).
8. One measured target is decomposable with two summands.
9. The cross-target replay test was not completed.
