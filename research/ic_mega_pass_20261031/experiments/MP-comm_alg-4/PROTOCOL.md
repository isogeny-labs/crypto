# MP-comm_alg-4 falsifier protocol (pre-registered 2026-10-31, before any data)

Hypothesis tested (reduced per skeptics): for the chained m=3 Weil-descended binary system
  S3(x1,x2,y)=0, S3(y,x3,xR)=0 over F_{2^n},  x_i in V_i (dim d_i), y in V_y (dim dy),
the solving degree is governed by the SMALLEST block min(d1,d2,d3,dy) ("block law"),
not by the total unknown count N=d1+d2+d3+dy ("count law"). The n-flatness explanation claim is DROPPED
(it follows from linearity of Frobenius in the y bits). The multihomogeneous framing is prior art
(Faugere-Safey El Din-Spaenlehauer 2011, Spaenlehauer 2012, Petit-Quisquater 2012, Salizzoni 2304.13485).
This toy CANNOT support any growth law f(l,m) at n=131 (l~44); it can only support/kill
"degree tracks the minimum block at fixed field and equation count".

## Fixed parameters
- Field F_{2^13}, modulus x^13+x^4+x^3+x+1 (checked irreducible in code). 2 F_{2^13} equations -> 26 Boolean equations + N field equations v^2+v.
- Curve y^2+xy=x^3+x^2+a6, S3(X1,X2,X3)=(X1X2+X1X3+X2X3)^2+X1X2X3+a6; xR a valid abscissa (Tr(xR+1+a6/xR^2)=0).
- Each block an independent uniformly random subspace of F_2^13 of the stated dim.
- Expected #solutions 2^(N-26) <= 1/2 in every cell (mostly unsat); sat status recorded from msolve GB.
- 4 draws per cell, seed = 1000*cell_index + draw. Null: per instance, random_matched (ffd_semaev_core semantics:
  same #eqs, same per-equation monomial-degree multiset, same N), plus field equations. Delta = D_chain - D_null.
- Metric D = max F4 round degree reported by `msolve -g 2 -v 2` (char 2, DRL), comparative only.
  Exact cross-check: Boolean XL refutation degree D* (min D with 1 in span{m*f : deg<=D}) for N<=14 unsat instances.
- Timeout 60 s per msolve run; a cell whose pilot exceeds it is dropped and reported.

## Cells (d1,d2,d3,dy)
y-series l=3: (3,3,3,13),(3,3,3,6),(3,3,3,5),(3,3,3,3),(3,3,3,2)
y-series l=4: (4,4,4,13),(4,4,4,7),(4,4,4,6),(4,4,4,4),(4,4,4,3)
C1 (equal N, different min): (3,3,3,5)N14 min3 vs (4,4,4,2)N14 min2 vs (2,4,4,4)N14 min2;
                             (4,4,4,6)N18 min4 vs (5,5,5,3)N18 min3
C2 (equal min, different N): (3,3,3,3)N12 vs (3,3,3,6)N15 vs (3,3,3,13)N22; (4,4,4,4)N16 vs (4,4,4,7)N19 vs (4,4,4,13)N25

## Predictions (median over draws)
- prediction_if_true (block law): C1: D(min larger) - D(min smaller) >= 1 at equal N; C2: D flat (spread 0) as N grows at fixed min;
  y-series flat for dy >= l, drop >= 1 at dy < l. Null arm does NOT show the C2 flatness (Delta changes with N).
- prediction_if_false (count law): C1 pairs equal D (within 0); C2 D rises with N tracking the null (Delta ~ constant);
  or y-series drops already at dy=l+2 / l+3.
- Decision: supports if C1 and C2 both go block-law direction in >= 3/4 of the contrasts; refutes if C2 rises with N
  in >= 3/4 of contrasts together with C1 ties; else inconclusive.
- FSS biseries heuristic (not derived for Boolean, trilinear case): bilinear part S3(y,x3,xR) gives D <= min(dy,d3)+1+(Boolean shift).
  Recorded as heuristic only.

## Pilot (step f), recorded before the full run
- (3,3,3,3): chain D=2 (msolve), XL D*=3; null D=4, D*=4; 0.03 s/run.
- (4,4,4,13) seed 5000: chain D=4 in 2.7 s; NULL timed out at 60 s.
- Deviation rule applied: null arm run only for N<=19; cells with N>=20 report chain only. No cell dropped.
