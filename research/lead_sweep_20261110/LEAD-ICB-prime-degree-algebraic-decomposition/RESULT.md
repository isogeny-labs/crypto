# RESULT: LEAD-ICB-prime-degree-algebraic-decomposition

## Outcome
- status: ran (single run of run.py, unmodified). exit=0, wall=40 s (run.status), run.py wall_s=40.3.
- run.stderr empty. run.py verdict field: "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): R6 >= 0.5 in every cell, and R6 exceeds the random-null R6 by > 0.2. Any cell with R6 < 0.5 (controls ok) falsifies. Inconclusive if: positive control < 0.8, null not separated, <5 unsolvable, |R6-R6_AA| > 0.25, or any unsound refutation.

Measured (unsolvable counts / refute rate at D=4,5,6):
- (13,5): Semaev 37 unsolvable, R4=R5=R6=1.0; A/A 36 unsolvable, 1.0/1.0/1.0; null 0 unsolvable.
- (11,5): Semaev 28 unsolvable, 1.0/1.0/1.0; A/A 26 unsolvable, 1.0/1.0/1.0; null 0 unsolvable.
- (13,6): Semaev 8 unsolvable (K=10), 1.0/1.0/1.0; A/A 6 unsolvable, 1.0/1.0/1.0; null 0 unsolvable.
- Unsound refutations of solvable instances: 0 in all arms.

Controls:
- Soundness: passed (0 unsound).
- A/A: passed (|R6 - R6_AA| = 0 in all cells).
- Null: FAILED to provide a comparison. All random same-shape systems (n equations, 2l variables) were solvable, so R6_null is undefined (no unsolvable instances) and the "exceeds null by > 0.2" clause cannot be evaluated.
- Positive control: FAILED. Overdetermined 8-var/24-quartic refuted fraction = 0.0 (< 0.8 required).

Verdict: inconclusive (run.py and my reading agree). Prediction's first clause (R6 >= 0.5) is met numerically, but the preregistered gate conditions invalidate the verdict.

## Reading
- R4 = 1.0 already at D=4, i.e. no degree growth was seen; D=5,6 add nothing. The statistic is saturated, so it has no resolving power on closure degree.
- Positive control returning 0.0 is a surprise. A 24-equation, 8-variable random system would most likely be solvable-free only if the random quartics have no common root; heavily overdetermined random systems are usually inconsistent, so refutation at D=6 should be easy. 0.0 suggests the control generator produces systems that are solvable (e.g. coefficient construction without a constant term, so the all-zero point is a root) or the detector fails on it. Likewise null systems being 100% solvable (40/40, 10/10) points to the same generator property (likely homogeneous or no constant). I did not investigate or modify run.py, per the rules. Consequently the null and positive controls are probably broken instruments, not evidence about the lead.
- Semaev instances are 70-95% unsolvable by brute force (n=13,l=5 gives 2^10 pairs vs 2^13 targets), so refuting them at degree 4 is unsurprising: with 13 degree-<=4 equations in 10 boolean variables the constant 1 lies in the span already. This is a counting effect of the 2l < n regime, not evidence of algebraic structure.
- The toy substitutes S_3 for the lead's S_{m+1} at m=4,5, so it does not test the lead's actual claim.

## Bearing on the sub-sqrt(p) question
none: toy sizes (n <= 13), failed positive and null controls, and a saturated statistic mean nothing about the decomposition degree at ECC2K-130/sect571 scale.

## Standing
[pcc:unverified], single local toy run, no independent replay.
