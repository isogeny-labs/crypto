# M31 dual target-functional rank-one regularization

Status: algebraic derivation and deterministic fixed-input reference [pcc:unverified]. No native solve, new relation collection, timing or speedup claim.

Scope notice #5122 comment 7136452052; parent 2b97fe822304080d69ef3762d26ef439e93abb73. #5076 owns extension admission; #5120 owns native integration. KB/Conductor/local checkout and worker tools unavailable; one coordinator, zero workers. Bounded fallback current AGENTS/ACTIVE, main and branch dated logs, catalog/SOURCES, prior matching/Gram/Lanczos reports and current pq_wiedemann.rs blob b1712946b81487bf5c4c45fdb3e969e9edb70aec (already has weighted Gram operator). Open PR searches "incremental compatibility" and "rank-one Gram" produced no relevant competing scope; not an absence proof. Literature: Kaltofen–Saunders 1991 DOI 10.1007/3-540-54522-0_93, sparse rank/solve preconditioning; Thomé 2016 https://arxiv.org/abs/1604.02277, homogeneous block Lanczos and storage/breakdown context. The following direct rank-one argument is not a novelty claim.

## Algebra

Let D be the m-by-n reduced point matrix with source weights as columns, c the reduced target functional, W a diagonal row weight, A=D^T W D, over the prime subgroup field. The two mutually exclusive exact certificates are D^T u=c (no target-only witness) and Dv=0, c^T v=1 (target functional nonzero on point kernel). Keep the original tape, other public functional and group checks in the real pipeline. No target recovery is claimed here.

For nonzero lambda consider B=A+lambda*c*c^T and solve B y=lambda*c. Put h=c^T y. Then A y=lambda*(1-h)*c.

* If h != 1, u=W D y / (lambda*(1-h)) is a negative certificate, even without any rank-preservation assumption; verify D^T u=c and lift.
* If h=1, A y=0. Only accept a positive certificate after checking D y=0 and c^T y=1, lifting source weights and checking original equations. Equality ker(A)=ker(D) is sufficient to imply the point-kernel condition but never substitute it for the check. If Gram has spurious nullspace, refuse and use existing fallback.
* Breakdown or unsolved system is neither outcome. c=0 can return the trivial negative certificate without a solve.

Assume rank(A)=rank(D)=k. If c is outside im(A)=im(D^T), the equation forces h=1; there exists v in ker(A) with c^T v=1 by duality, and Bv=lambda*c. Moreover ker(B)=ker(A) intersect ker(c^T): if Bv=0, c outside im(A) forces c^T v=0 then Av=0; converse immediate. Thus rank(B)=k+1. In the important **one-dimensional kernel case k=n-1**, B is nonsingular and every solution is a normalized positive witness. For larger kernels B is still singular, though the system is consistent; do not assume scalar Lanczos works.

If c belongs to im(A), choose z with Az=c and s=c^T z (independent of the chosen solution by symmetry). B=A(I+lambda*z*c^T). For 1+lambda*s nonzero the right factor is invertible and y=lambda*z/(1+lambda*s) solves the system. h=lambda*s/(1+lambda*s) cannot be 1. If 1+lambda*s=0, multiply By=lambda*c by z^T: (1+lambda*s)c^T y=lambda*s=-1, contradiction. At most one nonzero lambda is exceptional; two distinct predetermined nonzero choices suffice for this algebraic consistency property, not for Krylov nonbreakdown. For rank-deficient negative inputs B remains singular, so retain exact fallback. This argument adds no unproved generic nonsingularity assumption.

## Fixed-data reference

Study m31-dual-target-rank-one-v1; deliberately reused frozen public data, no new PRNG, master/replicate/child seeds, independent sample, randomized timing, or changed historical schedule. M31 p=2147483647, a=847514846, b=2087444724, group/subgroup r=2147446831, h=1, G=(625037715,2100627251), public Q=(976979610,1228979761), historical seed 0x7033315f6d340101. Arithmetic below is over F_r, not F_p. Producer binary/rejection trace gaps remain; not a qualified general reproducible A/B.

Exact input pins checked:
- 577d6c2c1c4a527604d965e6fb62c44df7902f36:research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json, blob 7fd27b9888058399d8814cef373d373243e6a1e5.
- b1af35f3822160bd6e57cf2d1458f4ab0b8d2c10:M31_DUAL_NONFILL_PIVOTS_20261124.json (same directory as this report), blob 72f2a2240994e3436ab8e5a1910a31cf1949bdcf.
- 62a96998536d060ad0fa0a6005571816207148ac:M31_DUAL_LANCZOS_20261124.json, blob 087e171304a14bc9fcd2faae953820e1bfedf697.

Deterministic BigInt reference, not Rust: construct signed point maps from relation_rows and c from each row label's query b; for each of 303 candidate.tape pivots calculate -other/pivot, check exact map, substitute into all point rows and c, clear pivot row and check recorded point-nonzero delta. Sort live source IDs and compare frozen reference. Remove empty point rows; apply A as D^T(Dx), with W=I. Read frozen gram_solution z and check Az=c. Compute s=c^T z, lambda=1, delta=1+s and y=z/delta, h=c^T y. Check shifted residual By=c and D^T(Dy/(1-h))=c, compare normalized Dy to Dz. Finally check all original signed rows against frozen lifted_point_multipliers and corresponding query b. These are exact equalities, no tolerance. No shifted Krylov execution; no positive experimental control or new target recovery.

| Quantity | Exact result |
|---|---:|
| Pivot expression/delta checks | 303/303 |
| D rows / columns / nonzeros | 363 / 342 / 2034 |
| c nonzeros | 337 |
| s | 1114467698 |
| lambda / delta | 1 / 1114467699 |
| h | 172015757 |
| Unique exceptional lambda on this capture | 2118574033 |
| Old Gram and shifted residual checks | 342/342 each |
| Normalized Dy equals Dz | 363/363 |
| Original negative certificate equations | 645/645 |

All values modulo r. Rank-one application need not materialize c*c^T: one sparse dot, one scalar product by lambda, one sparse axpy; at most 675 extra multiplications and about 674 additions at current c support 337, plus common Gram operation. O(nnz(c)) extra arithmetic and O(nnz(c)) representation; matrix dimension unchanged. Relative to 4068 Gram coefficient contributions this is extra work, not a win on the negative full-rank capture. Ordinary checked Gram Lanczos remains the primary negative path. Consider rank-one only for positive-certificate handling at rank transition; native benchmark must charge all extra operations and breakdown/fallback, and first qualify an independently constructed positive control on the actual admitted M31 extension. The rank-one argument does not imply faster collection or complete solving.

No new randomness, paid compute, native CI dispatch, merge or leaderboard changes. Native work and extension stay with #5120/#5076. No native timing, phase elapsed/CPU/RSS unavailable; branch-only append, canonical-main synchronization and independent PCC pending. Retain mandatory experiment-seeds/v1, M31, parallel-worker/resource, publication/no-merge/stop and impact requirements.

Impact score: 4/10 — Derived a rank-one symmetric formulation that is nonsingular at an exact one-dimensional useful-kernel transition, and exactly checked the negative branch on the existing M31 capture. Positive extension execution, native costs and complete solver advantage remain open.
