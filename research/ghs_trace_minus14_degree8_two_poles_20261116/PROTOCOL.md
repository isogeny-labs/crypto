# Trace −14 degree-eight two-branch-pole screen

Frozen source: `screen.sage`, SHA-256 `b06a004a8654cab2686c1870a311af31abef4083e24289753f86230606e0d75e`.

Field: K=F3[a]/(a^5+2a+1). Test each pair of carrier masks 9735, 17371 and source masks 11, 202, 38, 80. Source E_b: v²=u³+u²+b, #E_b(K)=258. Carrier Y_f: y²=f(x), deg f=11. Its K-factorization is one monic linear L and five monic irreducible quadratics Qi.

Search exact degree-eight morphisms u=A/B, v=yC/B², where fC²=B(A³+A²B+bB³), gcd(A,B)=1. Denominators B=L Qi Qj and B=L³ Qi Qj, all 10 unordered quadratic-pole pairs. For each, CRT enumerates all three roots of e modulo each unselected quadratic, yielding 27 branch assignments. A is the corresponding residue of Bh modulo M=f/(L Qi Qj), plus M(λx²+μx+ν), with nonzero square λ and μ,ν in K. All logical 3,858,261,660 numerator candidates per cell are represented by bitsets over ν; 20 nonbranch K-points filter candidates. Every survivor undergoes direct gcd, divisibility, polynomial-square, and full-identity checks. Point-filter equivalence is checked directly for 540 assignments times 20 points per cell.

This is one implementation and two explicit denominator shapes. Zero hits exclude those shapes only, not all degree-eight covers, subgroup correspondences, or GHS vulnerability. A hit would require independent identity replay, subgroup map construction and checking, prior-work comparison, and complete same-target attack accounting before any vulnerability claim.

Run: `sage screen.sage FMASK BMASK > cell_FMASK_BMASK.json 2> cell_FMASK_BMASK.stderr` from this directory. Record source hash, counts, elapsed CPU and wall time, and raw stderr for all eight cells. Do not change this source after starting the counted run; make a new version if correction is needed.
