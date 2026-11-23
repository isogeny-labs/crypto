# Frozen prediction for F_43 (written 2026-11-14 before any F_43 closure; see PROTOCOL.md "FROZEN PREDICTION")

Setting. p = 43, t = 1 plane scan: every (alpha, beta) in F_43^2, (a,b,t) = (alpha, beta, 1), k = 6, D = 4, same closure code. "Valid" = nonsingular,
nine S_3 coefficients nonzero, f(1) = 1+alpha+beta != 0. "Unit" = unit at k = 7 (unit_test.py). Generic dim = 605.
Condition inferred from F_19 (census, 3422 instances) and plane scans at p = 23, 29, 37 (training only):
  (A) 3t^2 + a = 0, i.e. alpha = -3 = 40: departs.
  (B) alpha, beta on one plane cubic Q_p(alpha,beta) = 0 (coefficients per prime, not lifted to Q): departs with dim in {599, 600, 602, 607}.
  (C) a third family, dim 794 (1 in V_4(6)) on unit instances (792/793 etc. on the same (alpha,beta) when not unit): not on any plane curve of degree <= 6 at p = 19..37.
  (D) the dimension of a unit instance depends on (a,b,t) only through (alpha,beta) = (a/t^2, b/t^3).

Predictions (each is falsifiable by the F_43 scan; none uses an F_43 closure):
 P1. Every valid point with alpha = 40 has dim6 in {600, 603} (never 605, never >= 786), whether or not it is unit; at most 2 of them have dim 600, at least 1.
 P2. Every valid unit point has dim6 in {599, 600, 602, 603, 605, 607, 794}; no other value.
 P3. Among valid unit points with alpha != 40, the points with dim in {599, 600, 602, 607} number between 20 and 42, and the degree-3 plane-curve fit to
     all valid points of those dims (any unit status) has nullity exactly 1 with no relation of degree <= 2; every valid point on that fitted cubic that is unit has dim in {599,600,602,607} (zero false negatives).
 P4. Valid points with dim >= 786 (class C, any unit status, alpha != 40) number between 28 and 46; the fraction of unit valid points that depart (dim != 605) lies in [0.12, 0.30].
 P5. Class C is not on a plane curve of degree <= 5 (nullity 0 for each degree 1..5).
 P6. (D): for 150 random valid-and-unit (a,b,t) over F_43 with (alpha,beta,1) unit, dim6 equals the plane dim6 of (alpha,beta): 150/150 agree.
Falsifiers: any of P1..P6 failing as stated. Result is reported as predicted/not per item.
