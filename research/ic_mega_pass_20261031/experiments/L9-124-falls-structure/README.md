# L9: the 124 extra round-3 falls of Semaev arm A at n=31

**Answer.** Write a_i = a^(2^i) and b_i = b^(2^i). These are K-valued F2-linear forms in the a- and b-bits, with K = F_{2^n}.
The 124 falls are the trace components Tr(θ·Q) of **4 K-valued relations Q**. Each Q has bit-degree 3 and uses **4 consecutive
Frobenius twists**, with indices t..t+3. The 4 relations are independent modulo the round-2 relations, and each Frobenius orbit
contributes n F2-dimensions. So the count is **4n**: 124 at n=31.
- Their cubic pivots, in K-RREF with cubic monomials first, are a0·a1·b3, a0·a2·b3, a0·a3·b1 and a0·a3·b2.
- No relation fits in 3 consecutive twists. Window {0,1,2} explains 0 of the 124; window {0,1,2,3} explains all 124.

The falls need the bilinear part and the affine part to be K-semilinear in the **same** Frobenius coordinates (a,b):
- Replacing L(x) = x²+zx in the bilinear part by a random F2-linear map gives 0 falls.
- Replacing the affine part by random vectors gives 0 falls.
- Neither the a↔b symmetry nor the trace map is the cause. With a ∈ V1 and b ∈ V2 (no swap symmetry) A still gets 124. With
  affine z²(a+b) instead of z²(a+b)² it also gets 124.

This is a Frobenius/linearized-polynomial ("small Frobenius window") syzygy effect. A rank-n tensor (C1) without coherent
K-structure does not have it. novelty_unverified: no literature search was done. It looks like the known Weil-descent
first-fall-degree phenomena. No security claim.

## Count formula, checked on 4 values of n
| n | plateau after round 2 = n(3N−5) | A round-3 falls | explained by window {0..3} | C1 round-3 falls |
|---|---|---|---|---|
| 29 | 2465 | 198 (s0, s1, s2) | **116 = 4·29** (82 unexplained, see below) | 261 (degree 4 full) |
| 31 | 2821 | **124** (s0 SAT; s2, s3, s6 UNSAT) | **124** | 0 |
| 33 | 3201 | **132** | **132** | 0 |
| 35 | 3605 | **140** | **140** | not run |
| 27 | 2133 | 1233 | 431 | 1341 (degree 4 full; structure is swamped) |

Rank bookkeeping against C1 (n=31 and n=33): A has a degree-4 leading rank smaller by 8n (248 and 264) and a total rank smaller
by 4n. So 8n extra degree-4 cancellations occur: 4n become falls and 4n become zero syzygies.

At n=29 the 4 relations appear as well (same K-RREF supports, K-dim 37→41). They sit beside 82 other falls that occur because
the degree-4 rank is near full (27352 of 27405). Those 82 are not classified. At n=27 count-forced falls dominate, so 4n cannot
be separated there.

## Pre-registration scorecard
- **P0 held.** The round log reproduces L1 exactly: 1022 / 2821 / 2945, with degree-4 rank 34224.
- **P1 partly held.**
  - All 124 new leading monomials have degree 3 and none is pure a³ or b³.
  - The split is (2,1) = 93 and (1,2) = 31, which depends on the monomial order.
- **P2.**
  - Held: W2 and W3 are swap-stable.
  - Failed: the 62/62 split.
  - Failed: "symmetry needed".
- **P3 held.**
  - All 4 tested n=31 seeds (SAT and UNSAT) give exactly 124 falls.
  - The 4n structural part was checked at n=29, 31, 33 and 35.
  - The n=27 check was not separable.
- **P4 held** for A_V1V2 (keeps 124) and A_Lrand (0).
  - A_randaff (0) and A_lin (124) are additional evidence.
  - A_noaff, A_ab and A_sq change the round-2 plateau, so they are not comparable.

## Method
- `src/macdump.c` is L1's macclose with two additions: a stop after round R, and dumps of the low pivot rows after each round.
  Pivot sets under the fixed order are canonical, so "new leading monomials" is well defined.
- `src/krel.py` computes the F2-span T of Tr(x^t · a^e b^f) over K-monomials in a Frobenius window. It then measures
  dim(W2 + (W3∩T)) − dim W2.
- `src/kker.py` and `src/newrel.py` solve for K-valued relations, meaning K-polynomials whose every trace component lies in W_r,
  and give their K-RREF.
- Relations were spot-checked by direct membership (`src/chk.py`).
- A bug was fixed during the run: the non-reduced echelon reduction was not linear. All numbers above come from the corrected,
  fully reduced normal form.

## Unexplained residue and blockers
- I did not derive the closed symbolic form of the 4 relations, as functions of z and a6. I also did not write out the explicit
  degree-4 syzygy that produces them. They are pinned only numerically, by support and window.
- The 82 non-window falls at n=29 are unclassified.
- Only seed s0 was run at n=33 and n=35.
- Dumps (~10 MB) were kept in $TMPDIR, not here. Raw run logs are in `raw/q*.jsonl.gz` and analysis outputs in `analysis/`.
