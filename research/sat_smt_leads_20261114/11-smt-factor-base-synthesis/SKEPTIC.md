# Skeptic review: idea 11, solver-driven synthesis of the factor base

Reviewed 2026-11-14, one core, about 15 minutes. Everything here is `[pcc:unverified]`:
a reviewer's re-computation, not a ledger claim. Scratch scripts:
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-11-smt-factor-base-synthesis/indep.py`
and `an.py` in the same directory.

## Verdict

- Reproduced: yes, for the two smallest SAT3 cells and for every table figure I recomputed.
- Stated verdict "kill-criterion-met": weaker than stated. Corrected: **inconclusive**.
- The measurements are sound. The label is not what the pre-written criteria say.
  Kill 1 is not met (the producer says so). Kill 2 as written in `PROTOCOL.md` requires
  S/A inside the A/B range "in every solve arm"; the L21 candidate arm separates by a
  factor of 1000 at l=7, so Kill 2 is not met either. The task's own kill wording names
  "a simple degree-2 linearisation decision rate" as a solve cost, and that rate is
  0.986 (S) against 0.000 (A, B) at l=7.
- What the data do support: no base better than the polynomial span was found, the
  polynomial span is the base the linear 2+1 baseline already uses, so there is no
  improvement over the current baseline. That is the protocol's pre-registered third
  outcome ("known mechanism, nothing new to synthesise"), not a kill.
- The SAT arms cannot carry a "no difference" conclusion: the A/A floor is a factor
  1.6 to 1.9 with 4 bases per arm, and only two sizes (l=3, 4) contrast S with random V.

## 1. Instance generator (independent check)

Own code: table-based F_{2^n} multiplication (checked against shift-and-reduce), curve
points found by exhaustive (x, y) search, own addition law. None of the producer's field,
curve, lift or brute-force functions were used. The producer's `cnf_m3` was used only as
the object under test.

All 192 rows of cells (l,n) = (2,7) and (3,11) in `raw_sat3.jsonl`:

| check | result |
|---|---|
| factor-base size equals the recorded `npoints` | 192 / 192 |
| recorded `dec` equals my triple enumeration (P1+P2 not in {O, T}) | 192 / 192 |
| planted targets decomposable | 96 / 96 |
| targets decomposable only through P1+P2 = 2-torsion point | 0 |
| SAT models (re-solved) decode to x in V, lift to points summing to +-R | 111 / 111 |
| UNSAT answers agree with brute force | 81 / 81 |
| CaDiCaL conflict count equals the recorded value | 192 / 192 |

The t = 0 exclusion is sound. With t in F_{2^n} forced, a model using non-liftable
(twist) x-coordinates can only arise through 2-torsion, so excluding t = 0 removes all
spurious models. In these two cells it changed no ground-truth label.

Not checked independently: cell (4,13) and larger, the m=2 CNF, the L21 leaf solver
beyond its `dec` output, and the defect numbers (def2, def3, hill-climbs, exhaustive
counts). dim V.V = 2l-1 for the polynomial span is immediate (products are c^2 t^k,
k = 0..2l-2).

## 2. Rerun

`python3 validate.py` ran in 39 s and printed the figures the README quotes: 23 instances
agree, 8 decomposable, subfield product dimension 3 and 5, probe (4,13) 94443 conflicts,
probe (5,17) 368481 conflicts in 27.2 s.

The rerun command does not regenerate any row of `results.json`. I therefore re-solved the
two smallest cells myself (table above): conflict counts match exactly. `results.json`
rows are identical to the three raw files (1056 rows). My recomputation of geometric
means, S/A, A/B and the SAT3 slopes (3.87 / 4.58 / 3.93) matches the README.

Running the rerun command created a `__pycache__` directory; I removed it.

## 3. Fairness

Holds:
- Same seeds, same 4 shared random targets for all 12 bases of a cell; all
  non-decomposable rows come from those shared targets (0 planted rows in any
  non-decomposable table). Decomposable and non-decomposable are reported separately.
- Same CNF rule in every arm, bases interleaved S, A, B, A/A arm present.
- The linear 2+1 scan baseline is included and nothing is uncounted on the treatment
  side (computing dim V.V is negligible). No L21 leaf was truncated at the 2^14 cap.
- File times are consistent with protocol first: `PROTOCOL.md` 09:24:52, `analyze.py`
  09:29:25, first cell finished 09:29:44. The protocol does not mention the (2,7) cell or
  the t = 0 exclusion, which fits it predating both. There is no git history, so file
  times are the only evidence.

Problems:
1. Verdict label does not follow from the criteria (see Verdict).
2. SAT3 is underpowered. Non-decomposable l=4: S/A 0.646, A/B 1.643, S/(A∪B) 0.85,
   exact permutation test on per-base means p = 0.50. Decomposable l=4: S/A 0.41,
   A/B 1.92, S/(A∪B) 0.53, p = 0.16. A true effect of up to about 2x in either direction
   would not be seen. "No difference beyond noise" is correct; "no difference" is not shown.
3. The SAT3 slope uses three sizes only by adding the degenerate cell (2,7), where every
   V has dim V.V = 3. Disclosed by the producer. Two informative sizes do not make a slope.
4. "Decomposable rows show the same picture" is loose. SAT2 decomposable: S is slower,
   S/A 1.49 at l=7 and 1.39 at l=8 (S/(A∪B) 1.50 and 1.72, p = 0.16 and 0.07). SAT3
   decomposable l=3: S/A 1.55 with A/B 0.97. None is significant, but the direction is
   not uniform and the headline quotes non-decomposable only.
5. `run_solve.py` was last modified in the second the first cell finished, so the added
   cell and the resampling rule were post-protocol edits made just before the run.
   Both are disclosed; neither changes a conclusion.
6. Factor-base size varies a lot between bases and is not controlled: at (4,13) the B
   bases have 11, 21, 7, 9 points against 19, 17, 13, 15 for S. This drives the leaf count
   and part of the A/B spread (L21 leaves A/B = 1.67 there).
7. The SAT arm is not competitive with the baseline in any arm: about 60000 conflicts at
   l=4 against about 16 linear leaves. The S-versus-random SAT contrast therefore says
   nothing about beating the linear 2+1 baseline; it only compares two weak arms.
8. The S formula is smaller (w = 7 against 9 to 10 at l=4, about 650 against 710
   variables). Conflicts are not cost-equivalent across arms. Propagations and CPU give
   the same reading (S/A 0.59 and 0.57 non-decomposable at l=4, A/B 1.65 and 1.71), so
   this does not change the conclusion.
9. README says "933 instances" twice; the files hold 1056 rows (768 with a SAT run).
10. Scope gaps, all disclosed: no z3/CEGIS search, no coordinate change on x, defect
    defined on V.V and the m=2 system instead of the descended S4, exhaustive search only
    at l=3, one curve, one solver. The claim "nothing below 2l-1 exists" rests on the
    cited Bachoc-Serra-Zemor bound for prime n, not on the search arm.

## 4. Does the verdict follow?

No, not as labelled. Under the pre-written criteria:

- Kill 1 (best defect inside the random top 5%): not met. Polynomial span is below the
  random minimum for l >= 4.
- Kill 2 (no solve-cost difference beyond the A/A floor in every arm): not met. L21
  candidates and the decision rate separate far beyond the floor (permutation p = 0.002,
  the smallest attainable with 4 against 8 bases, at every l >= 4).

So structure exists and it pays, but only through the mechanism the baseline already
exploits, and the best base found is the baseline's own. Against the success rule (a
measured improvement over the current index-calculus baseline) the result is zero
improvement. Whether a solver-driven search could find a base that helps beyond dim V.V
(for example on a degree-3 defect of the chained m=3 system) was not tested, and the SAT
arms are too noisy to rule out a 2x effect. Hence inconclusive, leaning negative.
