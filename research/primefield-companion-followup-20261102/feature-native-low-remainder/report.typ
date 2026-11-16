#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Low-product Euclidean remainders
T-1834 · 2026-11-07 · General nonmonic division

General Euclidean division now computes only the low coefficients needed for its remainder. The existing reversal/Newton kernel still supplies the exact quotient, which remains available for half-GCD row updates. Constant divisors return a zero remainder without forming a residual product.

#image("flow.svg", width: 100%)

*Proof.* Let a,b have degrees m≥d and let k=m−d+1. The quotient reversal is rev(a) times the inverse of rev(b), modulo X^k. Thus the highest k coefficients of a−bq vanish, so the remainder has degree below d. It suffices to compute a modulo X^d minus the short product of q and b modulo X^d. Tests explicitly reconstruct the omitted high coefficients. The production path relies on exactness of the unchanged quotient kernel.

*Accounting.* For nonzero a with m≥d>0, the previous q b result has m+1 coefficients; the short product returns at most d. It avoids the high output coefficients and full subtraction vector. Quotient/reversal/inverse arrays and recursive multiplication temporaries remain. The exact schoolbook count is the number of retained coefficient pairs with i+j below d; general recursive costs remain input-dependent. No total peak-memory or timing ratio is asserted.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Exact software check], [Count], [Coverage],
 [General division pairs], [2,592], [Same q,r; reconstruction; high cancellation],
 [Lower / equal / higher products], [1,792 / 800 / 0], [Fixed panel; inversions unchanged],
 [Constant divisors / zero remainders], [288 / 680], [Boundary and exact-divisibility paths],
 [Schoolbook pair controls], [1,220], [Exact retained-pair count],
 [Zero-divisor / empty-dividend controls], [32 / 4], [Errors and zero result preserved],
 [Complete resultants], [432], [All exercise half-GCD; 47 zero results],
 [Additional recoveries], [617], [5,929 source calls; zero unresolved])

*Retained controls.* The 288 matrix-update checks, 8,320 singleton cases, 1,108 source points and 24 windows pass. The inherited 63 public comparisons retain 511 calls. Final normal build pins seven native sources with audits disabled. The old full-resultant count assertion isolated only matrix changes; this successor audits division counts separately while retaining local matrix savings and full-resultant output/inversion checks.

*Evidence and scope.* Original 4,563-case cohort unchanged. No cloud, timing comparison, external benchmark or full Cargo rerun. Prime and source subgroup assumptions remain. PCC-20261107-b0cd9b is unsigned producer evidence. See DERIVATION.md and the validation, recovery and normal-build receipts for exact inputs, commands and hashes.

*Sources and next work.* FLINT documents Newton quotient reversal and short-product remainders; the prior monic implementation supplied a local model. KB unavailable; no novelty claim. Next price a short-quotient division path against Newton reversal, including leading inversion and crossover costs. Historical reports and canonical graphs remain unchanged.
