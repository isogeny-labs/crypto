# AUDIT MP-lattices-3 (2026-10-31)

verdict: partially_valid. Outcome `refutes` stands, scoped to: accounting, triangular LM-diagonal, l<=2, box <=2l.

## Re-run (reran=true)
- scratch/audit_MP-lattices-3/run.py (copy of run.py), 212 s CPU. All 23 RESULTS.json rows reproduce exactly (delta, dim).
- numpy matmul RuntimeWarnings (divide by zero / overflow / invalid in E@b) are the known macOS Accelerate numpy-2.0 spurious warnings. Checked E@b vs (E*b).sum(1): finite, max diff 3.6e-15. No effect on results.

## Code vs protocol
- S3 polynomial is the correct Semaev S3 for y^2=x^3+Ax+B. Joint system S3(x1,x2,u),S3(x3,x4,v),S3(u,v,z) is the correct S5 resultant system.
- Weight w=k-e.b with sum w>0 is the correct Coppersmith det<p^{l*dim} condition. Closure edges go from m to its non-leading terms. Term orders are valid monomial orders (weighted + lex).
- Max-weight closure via min-cut is correct. No randomness or seeds. Constants A=7,B=11,z=5 affect only coefficients, not LM-diagonal accounting.
- Controls are real: dense univariate d=2,3,5 and BD 0.25 match the closed forms l/(dl+d-1) and BD. S3 block 1/18,1/10,0.11905 match W10 JM-basic.
- Dense univariate rows live under key `sanity_and_dense_controls` (A0_univ_dense). The `results[]` A0_univ rows (d=3,5) use sparse f and are wrong (e.g. d=3 l=1 0.25 vs dense 0.2; d=5 0.1667 vs 0.1111). They are labelled superseded only in README, not in RESULTS.json. Anyone reading `results[]` alone gets wrong controls.
- l=4 S3 block 0.12962 exceeds W10 exact optimum 0.128889. Not an error (box 2l vs W10 box differs) but "0.6% gap" is not a like-for-like comparison.

## Weaknesses (none flips the outcome)
1. Shift rule is "largest feasible k per monomial", not the optimum over k. W10 optimises (M,k) jointly. For one polynomial this coincides; for 3 polynomials it need not. Audit test: restricting allowed generators to {g1},{g3},{g1,g2},{g1,g3},{g1,g2,g3} (lp2 variant) gives delta=0 at l=1 and (for {g1},{g3}) at l=2. So k-rule is not hiding a positive lattice at these l.
2. A3 "multiplicity s=2l" is box-limited. Box u,v<=2l is exactly the degree of g^l, so large u,v multipliers are mostly infeasible (at l=1, A1=A2=A3 by construction). Audit test: u,v box 4 at l=1 and 6 at l=2 (partial) still gives 0.
3. Sanity check (u,v<=p^delta reproduces S3 block) only shows g3 alone is reproduced (dim 9, 25 identical to A0_S3block); it does not exercise g1/g2. It is a weak positive control for the joint machinery. No sweep of the u,v bound between ~0.06 and 0.25.
4. S5 comparison uses W10 values (l=2..4 verified in run.md: 0.002455, 0.004804, 0.006334). l=1 0.000010 not checked by me. At l=1 both are effectively 0, so "A3 <= S5" is a tie at l=1 and a loss at l=2.
5. Limited l: l=3,4 not run, while S5 is still rising (0.0048, 0.0063). Heuristic info bound 4*delta+2<3 (delta<0.25) does not exclude delta~0.17-0.2, so asymptotic closure is not shown.
6. Not run: Howell true-det (arm C), qflll (E), isolation (D). Stated honestly. Cuso-style cancelling combinations and non-triangular sublattices not covered (W10 also lists these as uncovered).

## Overclaims in README
- "Full-size internal nodes strictly worse than eliminating them" is shown only for l<=2 under triangular accounting; not a general statement.
- "class 3 can be closed at accounting level for l<=2" is fine; anything beyond l<=2 is not supported.
- Title "REFUTES" should read: refutes the pre-registered prediction (A3 > S5 by >10% at l=2 or 3) at l=2 (l=3 untested), accounting level.

## Corrected outcome
refutes (prediction_if_false met at l=1,2: A3=0 <= S5). Scope: accounting, triangular, box<=2l (spot-checked to u,v box 6), l<=2, no non-triangular/Howell arm. Not a closure of the S5 joint-lattice idea in general.

Audit artifacts: scratch/audit_MP-lattices-3/{lp2.py,audit_var.py,RESULTS.json,rerun_out.txt}
