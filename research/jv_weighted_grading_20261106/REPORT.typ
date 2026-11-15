#set page(margin: 2cm)
#set text(size: 10pt)
= Weighted grading for the k = 5 Joux–Vitse Edwards test: a loss
2026-11-06 · probe `weighted_probe` (`src/cryptanalysis/jv_quintic_edwards.rs`) · p = 271, seed 2, three random residuals

*Question.* Does the weighted grading that cut the D₄ route's solve also cut the 2-torsion Edwards route's per-residual decision cost?

*Answer.* No. It decides only at weighted degree 37, on a 28,981 × 26,618 Macaulay matrix costing 3.9·10¹⁰ F_p multiplications. That is about 670× more than `f4_fp`'s 5.85·10⁷. The unweighted Macaulay solve decides at degree 9 for 7.0·10⁷. Every figure is an unverified assertion.

#figure(image("fig_weighted.svg", width: 100%), caption: [Echelon multiplications against Macaulay degree; ★ marks the first decided degree. The dashed line is f4_fp's whole solve.])

#table(columns: 4,
  [solver], [decided at], [matrix], [F_p mults],
  [f4_fp grevlex], [inconsistent], [≤ 720 × 768], [5.85·10⁷],
  [Macaulay unweighted], [degree 9], [1,802 × 2,002], [7.0·10⁷],
  [Macaulay weighted (1,2,3,4,2)], [weighted 37], [28,981 × 26,618], [3.9·10¹⁰],
)

*Why.* The symmetrised equations are bounded by total degree 4 in the e_i, so their weighted degree ranges up to 16. The weighted grading fits them worse than the plain one.

*Precomputing the target-independent part.* Every trial lies in one fixed 105-monomial space, and its five combinations are generic. Trace replay already exploits the fixed shape, and no smaller system follows from it. Not built.

*Not claimed.* No change to C″, the leaderboard or benchmarks. One prime. A local probe.
