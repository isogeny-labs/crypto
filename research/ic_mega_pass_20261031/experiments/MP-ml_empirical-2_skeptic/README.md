# MP-ml_empirical-2 skeptic falsifier: is there a SAT/UNSAT signal in the degree-<=3 Macaulay data?

Instances: `ic corpus` Weil-descended Semaev S4 (m=3) systems, Koblitz a=1, n in {13,15,17},
l in {3,4,5}, certified SAT/UNSAT labels, seed 7.  `macaulay_screen.py <dir> <dmax> <out>` builds the
F_2 Macaulay matrix of each .anf at degrees 2..dmax (columns = squarefree monomials, high degree =
high bit, leading-bit elimination) and records rank, HF, dim(W_d cap B_{<=k}) for k = d-1, 2, 1, and
whether 1 in W_d.  `analyze.py` gives the Mann-Whitney AUC of each statistic with a 200-draw
label-permutation null.

Result: at l = 4, 5 every degree-<=3 statistic (and degree 4 at l=4 on a 12-instance subset) is an
exact constant across SAT and UNSAT; AUC = 0.5 exactly; selectivity 1 at any recall.  At l = 3 the
rank/HF are constant and only the fall dimension varies, one-sidedly and only on SAT instances whose
planted witness has repeated base points; SAT instances with three distinct points sit at the UNSAT
value.  Separation appears at D* (n=13: degree 5) or D*-1 (n=17: degree 4).  Numbers in RESULTS.json.
Toy scale, no security claim.
