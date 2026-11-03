# L9 PROTOCOL — what are the 124 extra round-3 falls of Semaev arm A (n=31, s=2)?
Written 2026-11-02 before any L9 run. No security claims.

## Objects
- Generator: N1 `src/lib.py system(n,k,s)` (k=(n+1)/2, N=2k Boolean vars; vars 0..k-1 = a-bits, k..2k-1 = b-bits).
  Arm A = components of the K-valued F(a,b) = (ab)^2 + z·ab + z^2(a^2+b^2) + a6, a,b in a random k-dim F2-subspace V of K=F_{2^n}
  (from MP-comm_alg-1 `semaev_parts`). C1 = same affine part, bilinear part L(W_j W_l) for a second random subspace W.
- Engine: `src/macdump.c` = L1 `macclose.c` verbatim + (argv1) stop after round R, (argv2) dump after every round the
  pivot rows of W_4 ∩ P_{<=3} (packed bits) and the low-monomial list. Pivot set under macclose's fixed column order is
  canonical for a subspace, so "new leading monomials in round 3" = piv(W^(3)_low) \ piv(W^(2)_low) is canonical.
- The "124" = dim W^(3)_low − dim W^(2)_low for A n31 s2 (2945−2821; L1 log). Round 3 = x_v · (round-2 new low rows).

## Pre-registered predictions
- P0 (sanity): macdump reproduces the L1 n31 A s2 round log (1022, 2821, 2945) and its pivot dump spans have exactly those dims.
- P1: all 124 new leading monomials have degree 3 (log already shows bydeg[3] +124); prediction: their bidegrees are mixed
  (2,1)/(1,2) only, none pure a^3 or b^3.
- P2 (a<->b symmetry): U = W^(3)_low / W^(2)_low is stable under the swap a_j<->b_j; the 124 split 62/62 between (2,1),(1,2).
- P3 (count): 124 = 4n. Prediction: every A draw at n=31 (UNSAT s3, s6, and a SAT draw s0) shows +124 in round 3 when
  round 2 ends at 2821; at n=29 and n=27 the A-specific structural part of round 3 is 4n (116, 108), measured as the
  dimension of round-3 falls of A that are NOT present in a matched control, or by explicit membership of the identified family.
- P4 (mechanism): the falls need K-coherence of bilinear and affine parts (same subspace and same K-structure); the swap
  symmetry is NOT needed. Variants (same V,z,a6): A_V1V2 (a in V1, b in V2, coherent) keeps falls; A_Lrand (random F2-linear
  map in place of L in bilinear part) loses them; A_noaff (affine part 0) — no prediction (exploratory).
- Kill criterion: if U is not explained by any tested family, report the narrowest residue (bidegree, block, dimension).

## Budget
<= 4 concurrent single-thread processes, each run stopped at round 3 (<~5 CPU-min each at n=31). Dumps in $TMPDIR; only
compressed summaries kept under raw/ (<200 MB total).
