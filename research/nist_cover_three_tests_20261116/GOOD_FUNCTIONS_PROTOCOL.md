# Kernel3 symmetric good-function construction

2026-11-16. Fixed input: the actual recorded kernel3 candidate produced in kernel3-resume-v1, whose conditional differential scalar is square (unsigned PCC-20261116-ba8a27). Build Tian arXiv:2007.03172 section4.2.3's eight good functions before attempting the divisor-class lift. Freeze this contract before its function/rank data. KB unavailable; the primary section and existing native target APIs were reviewed.

Use the pinned Riemann quartic, descent matrix and fixed-window target point receipt. Transform every recorded Fp point to the original Riemann coordinates and require the curve equation. Retain finite-chart exclusions. Require at least64 available points; do not resample the coordinate window after a failure.

With infinity2=(1:m2:0), use the basis B={1,x,x^2,y,y^2,xy,x*(y-m2*x)^2,y*(y-m2*x)^2} of H0(6*infinity1+4*infinity2). Its cubic binary forms have order at least2 at infinity2. The pole divisor has degree10 and Riemann--Roch dimension8; degree<=3 plane forms have no quartic relation. This is the theoretical basis/dimension argument, not inferred from a rank sample.

Index the120 symmetric triple products by nondecreasing triples from0..7, summing distinct permutations. The omitted repeated-permutation multiplicities are nonzero scalars in the declared characteristic and do not change the span. At diagonal triples(P,P,Q), build constraints with at most512 attempts: P index=i modulo N, Q index=(17*i+1+floor(i/N)) modulo N; skip P=Q. Stop only on rank112. Require exactly eight kernel vectors. The fixed balanced schedule avoids holding one point fixed, which cannot supply sufficient rank.

Check every kernel vector on128 heldout diagonal pairs, drawn with P=(11*i+7) modulo N and Q=(37*i+19) modulo N, skipping equality within a fixed512-attempt bound. These are heldout correctness checks, not a statistical independence claim. Combine rank112 with Tian's dimension/containment argument; sampled vanishing alone is not a proof of global vanishing.

Require rank8 among generic triple evaluations divided by omega squared, where omega=det(1,x,y). Use at most64 triples with indices(i,i+17,i+41) modulo N, rejecting repeated/collinear supports. Retain the selected indices, all attempted/excluded counts and exact interpolation values. Preserve every coefficient of the eight functions and m2, along with input/source hashes. Reject a changed constant coefficient, which destroys diagonal vanishing.

No rank threshold, support window or function basis may be relaxed after data. A guard failure is retained and diagnosed. This component constructs target functions only; Kummer-coordinate normalization, solving the lift, source-map/kernel/Frobenius/dual checks and restricted-base source recovery remain open. All assertions are producer-unverified; no timing or speedup claim.
