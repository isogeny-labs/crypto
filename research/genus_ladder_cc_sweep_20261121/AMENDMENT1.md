# Amendment 1 to the analysis plan (not the grid), 2026-11-21T19:41Z

Recorded before any run of this study; PROTOCOL.md and analyze_cc.py (frozen.sha256) are unchanged.
Trigger: the coordinator pointed to prior art, PR #4852, `research/g2plus_followups_20261117/F1_L03_base_constant_lab_instrument/`
(cc 0.5/1/1.5/2/3 without SGE at genus 3, p 8209-32771; best cc tracks the linear-algebra price kappa:
kappa=1 gives cc=2, kappa about 2 gives 1.5, kappa=9 gives 1; open thread: measure kappa at genus 3).
Additions, all analysis-side (script `analyze_amend1.py`, hashed in `frozen.sha256` when written):
1. RESULTS.md cites F1 as prior art and compares this study's optimum with F1's: same direction? does SGE(8,3),
   which cuts la_mul to about a third, move the optimum back toward cc=2?
2. Per cell and cc, report the measured ratio la_mul/rel_mul (SGE arm, merge work in la_mul) and the unmerged arm's,
   as the kappa-sensitivity F1 asked for.
3. Per cell, the best cc as a function of kappa (kappa in 1,2,3,4,6,9, I=10) from the recorded counts.
No prediction F1-F5 changes.
