# SKEPTIC review - idea 7, homogeneous AllSAT

Date 2026-11-14. Independent review, one core, about 15 minutes. Every number here is
`[pcc:unverified]`: a pilot-scale re-check, not a certified claim. Nothing in the producer's
files was edited. Scratch scripts:
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-07-homogeneous-allsat/indep.py`
and `probe.py` in the same directory.

## Verdict

The producer's verdict **kill-criterion-met is supported** for what was tested: Glucose4, XORs
cut to CNF, this encoding, family A (m+1 = 3, l = 4..7). It rests on family A alone. Family B
(m+1 = 4) does not independently support it: two sizes, 7 and 20 relations, and its sign depends
on a post-hoc relation-count correction (correct, but post hoc; see "Problems" 1).

## 1. Generator correctness (independent check)

My own field multiplication, brute-force curve point table, group law, sign-lifting and AllSAT
loop; the producer's code was used only for the modulus, the base vectors and the CNF builder
(the object under test). Cells: (k,l,n,seed) = (3,4,7,1), (3,4,7,2), (3,5,9,1), (4,3,7,1), (4,3,7,2).

| check | result |
|---|---|
| modulus irreducible, group order in Hasse bound, #E * P = O | pass |
| k=3: curve models of the homogeneous CNF = brute-force set of sorted triples with a sign choice summing to O | exact equality on 3 cells (1, 6, 3 relations) |
| k=4: same against brute force over all sorted 4-tuples | SAT finds no extra tuple; the 7 tuples per cell it misses are all of the form (a,a,c,c), i.e. P-P+Q-Q |
| every curve model lifts to points with a sign choice summing to O | pass |
| per-target SAT/UNSAT = brute-force decomposability, 16 random targets per cell | 80/80 agree |
| per-target model lifts to points summing to +-R | pass on every SAT target |
| planted targets (my own planting, 12 per cell) SAT with valid lift | 60/60 |

The chain S4 encoding is complete for nontrivial relations: with sorted x1<=x2<=x3<=x4 any
relation splits as (P1+-P2) = -+(P3+-P4), and the split fails only when P1+-P2 = O.

## 2. Reproduction

- Rerun command (`hallsat.py run 3 <scratch>/x.jsonl 1 4:7 16`): 18 rows, all counters
  (conflicts, decisions, propagations, restarts, model counts) identical to the matching rows of
  `raw_A_small.jsonl`; only wall time differs. 0.12 s.
- `results.json` rows equal the concatenated `raw_*.jsonl` rows (792 rows; no budget hit, no
  incomplete solve).
- Recomputed from the raw rows with my own aggregation: Q0 = 0.225, 0.138, 0.200, 0.178
  (k=3, l=4..7) and 0.300, 0.370 (k=4, l=3,4); slope of log2 Q0 = -0.048 bits/l over 4 sizes
  (k=3), +0.305 over 2 sizes (k=4). Matches the README.
- My independent count of trivial (a,a,a,a) models for k=4 gives the same 7 and 20 nontrivial
  relations.
- Not re-run: l=6, l=7 and family B measurement cells (time). Their numbers are checked only for
  internal consistency with the raw rows.

## 3. Fairness

Holds:
- Same base V for all three arms (one RNG stream per cell; H arms do not consume it), same CNF
  generator, same solver, lex symmetry breaking in every arm.
- Each unordered relation is counted once; SAT and UNSAT targets are reported separately and
  the per-target cost per relation includes the UNSAT targets (38-52% of targets).
- Twist models cannot occur in the per-target arm (a curve target forces curve summands), so
  the factor of about 2 lost by H0 to twist models is real, and H1 was run to remove it.
- All arms present in the raw data (H0, H1, PT) are reported.
- Final-UNSAT cost is small: conflicts up to the last model are 90-97% of the H0 total (my
  probe, k=3, l=4,5,6), so "run to UNSAT" does not explain the gap.
- Additional check from the raw rows: H0 conflicts per whole instance are 1.39, 1.61, 1.54,
  1.46 times (2^l x mean fresh per-target solve) at k=3, l=4..7. The homogeneous instance costs
  more than solving 2^l independent per-target instances with x3 fixed, which is direct evidence
  that learnt clauses are not being reused across relations.
- Per tuple: PT spends 0.34-0.43 conflicts per enumeration tuple, H0 1.44-1.77 (k=3). Both
  flat in l.

## Problems

1. **Family B sign depends on a post-hoc correction.** If the trivial (a,a,a,a) models are
   counted as relations, Q0 at k=4 becomes 1.54 (l=3) and 1.56 (l=4), above 1. Excluding them is
   right (P+P-P-P is a zero row) and the README discloses it, but it was applied after the run
   and it flips the family B result from "above 1" to "below 1". The headline "about 3x more at
   m+1=4" should carry that qualifier. Either way the two k=4 points are flat (1.54 to 1.56, or
   0.30 to 0.37), so the "grows with l" prediction is not supported there, but with 2 sizes
   no slope claim is admissible.
2. **The treatment encoding carries an avoidable handicap of about 1.45x.** `enc_s3` reifies
   x1*x2 into n fresh bits only when x3 is non-constant, i.e. only in the homogeneous arm. With
   the reification removed (same model sets), H0 total conflicts drop to 0.67, 0.69, 0.70 of the
   producer's at k=3, l=4,5,6 (my probe; 8, 8, 2 bases with different seeds). The ratio is flat,
   so Q0 would be about 0.2-0.3 instead of 0.14-0.23: still below 1 and still flat. The
   "4.4x to 7.2x" constant is therefore encoding-dependent and overstated by roughly 1.45x; the
   kill is not affected. Not checked at k=4.
3. **Kill criterion was concretised by the producer.** The idea said "kill: ratio flat in l or
   below 1". PROTOCOL.md turned this into "slope <= 0.15 bits/l in both families, or Q < 1 at
   the largest two l". The second clause alone is met by family A, and the observed values
   (Q about 0.2, slope -0.05) are far from both thresholds, so the choice of thresholds does
   not decide the outcome. Under the literal first clause ("in both families") the slope test
   would be undecidable because family B has two sizes.
4. **Protocol-before-results cannot be verified.** No git history. File times: PROTOCOL.md
   08:46:59, hallsat.py 08:49:31, first raw data 08:52:06. The protocol already names encoding
   details and a numeric null ("Q about 0.5 for H0"), so code and probably validation runs
   existed before it; that is allowed, but "written before any measured run" rests on the
   producer's word. Undisclosed deviation: the base seed in code is 1000*l + seed + 100000*k,
   the protocol says 1000*l + seed. Harmless.
5. **Sizes fell well short of the protocol** (A: l=4..7 of 4..10, 4 bases at l=7; B: l=3,4 of
   3..6). Disclosed. Family A has 4 sizes, enough for a slope; the l=7 point has 4 bases.
6. **Baselines.** The README table gives the linear-leaf count only for the per-target pipeline
   (1.6-1.7). `results.json` also holds the homogeneous leaf count (5.1-7.8 at k=3, 41-54 at
   k=4), which the README omits. No MITM arm was measured; for k=3 a pair-sum table lookup
   finds all relations in about N^2/2 additions (about 400 additions per relation at l=7 by my
   estimate, against 25,456 H0 conflicts). These omissions do not favour the treatment; both
   SAT arms lose to the non-SAT baselines by a margin growing at about 1.85 bits/l.
7. **Scope.** One solver, one encoding, fresh-solver baseline only, n tied to l. No XOR-aware
   solver, no incremental per-target control, no projected-enumeration variant, no homogeneous
   encoding with the linear leaf. The kill applies to "Glucose4 + CNF blocking-clause AllSAT",
   not to the idea under an XOR-aware solver. The producer says this.
8. **Statistics.** Bootstrap over 4-8 bases, 7-101 relations per cell. For family A every
   bootstrap interval of log2 Q0 lies below -1.3, so Q0 < 1 is robust there. For family B the
   intervals reach -0.30.

## 4. Does the verdict follow?

Yes, from family A: Q0 < 1 at all four l (0.23, 0.14, 0.20, 0.18), bootstrap intervals entirely
below 1, slope -0.05 bits/l over 4 sizes, and the pre-written clause "Q < 1 at the largest two l"
is met. Removing the encoding handicap I found moves Q0 to about 0.2-0.3 and leaves it flat.
Family B adds nothing reliable and should be reported as inconclusive on its own rather than as
a second confirmation.

Corrected verdict: **kill-criterion-met**, scoped to Glucose4/CNF, m+1 = 3.
