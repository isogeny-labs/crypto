# Independent C0 curve-point review

Reviewer /root/monic_review,2026-11-01. Source-first quotient/root/ordinate derivation recorded in review-plan.md before Output/Quotient/CurvePoint body access. Exact source add22_weighted C0 lines1000-1023 and normalize lines534-548 in projective_law_gate.rs. Parent supplied normalized curve assumptions and l=v1-a*u1, not producer quotient proof.

## Independent derivation and findings

A monic quadratic product has degree4 and next coefficient q1+q2. Interpolant l=v1-a*u1 has degree<=2 with top coefficient -a. Thus f-l^2 has degree5, coefficient5=1 and coefficient4=-a^2 when f is monic quintic with coefficient4=0. Product divisibility yields a quotient; degree forces degree1. Comparing coefficients5 and4 gives quotient X-(q1+q2+a^2). This was independently derived prospectively and matches the producer proof.

quotient_formula does NOT assume the quotient formula: it obtains a witness from divisibility, derives its degree, expands the degree-one polynomial, compares actual coefficients and reconstructs the expression. output_on_curve evaluates that identity at its derived root x, yielding f(x)=l(x)^2; output ordinate is -l(x). czero_output discharges product divisibility using prior explicit coprimality from bnum!=0 and common interpolation from cnum=0, with individual input divisibility. No circular point-on-curve premise. Degree and coefficient hypotheses are explicit. a=0 is admitted without a hidden leading-interpolant nonzero condition. No smoothness or divisor-class equivalence is claimed by this point equation.

## Native normalized and weighted boundary

At all input weights one, native B=bb and A=aa, a=A/B. Source raw X=(q1+q2)B^2+A^2, Z=B, Wout=1. Native D1 normalize decodes x=X/Z^2 and y=Y/Z^5 (NOT Z^3). The reviewer added normalized_source_ordinate and freshly proved the literal source Y/B^5 equals a*(x^2+q1*x+r1)-(s1*x+t1)=-l(x), assuming B!=0. This is a new source-coordinate identity, not merely the squared curve equation.

For arbitrary input weights, the prospective calculation uses Z=z1*z2, W=w1*w2, BW=bb*W, q_i=qic/Z^2,r_i=ric/Z^4,v11=s1c/(W*Z^3),v10=t1c/(W*Z^5),a=aa/(BW*Z). The raw formulas then have output denominators (BW*Z)^2 and (BW*Z)^5. This weighted expansion is recorded as a derivation, not a new machine-checked full weighted source theorem. Exact scaling of A/B/C from native common coordinates, nonzero B/W/Z, branch guards, word arithmetic, normalization and compiled execution remain separate obligations.

## Fresh verification

Output, Quotient, CurvePoint, PointControls and PointReadback passed fresh isolated builds: twelve main declarations, ten producer controls and twenty-two named readbacks. Five reviewer declarations passed: the general normalized source-ordinate identity and four fixtures covering a=0, a!=0, missing coefficient4 hypothesis and the ordinate sign. Twenty-seven passing readbacks use only standard propext, Classical.choice and Quot.sound. No reviewer failures occurred. Producer failures remain unchanged in the producer packet.

## Scope and workflow

Mathematical fidelity faithful_scoped. The quotient is derived, the output point equation is proved, and normalized source ordinate matches with explicit denominator premise. Native full weighted/word/compiled fidelity partial. Earlier imported interpolation/coprimality helper bodies were inspected but not all rebuilt in this review; copied compiled dependencies and cached Mathlib retain their trust boundary. Source-first local plan, separate reviewer and fresh named checks completed. Plan not precommitted, dispatch publication not established, independent-kernel/full dependency and multi-reviewer gates absent. Publication and official decisions remain coordinator obligations. No scientific execution or repository mutation.

Primary references inspected: native add22_weighted and normalize, new three modules, prior CZero common_interpolant/support_coprime/product_divides_residual, and producer controls. Standard polynomial division coefficient comparison supplies the derivation, with no novelty claim.
