# Next construction: mixed degree shared-support cancellation

Status: source/literature-derived construction plan, not a theorem or measured result. No build, native run, cohort or repository edit.

## Sources checked first

- Repository AGENTS.md literature-first and scope rules; latest materialized dated session log2026-10-30 (read only; root owns publication/logging).
- Ozbay Gurler–Hisil, Complete group law for genus2 Jacobians on Jacobian coordinates, DOI10.1007/s13389-024-00351-7, sections2,3.3,4.2; https://link.springer.com/article/10.1007/s13389-024-00351-7 . Primary text reopened. It treats mixed-degree support extraction/cancellation and weighted formulas; smoothness and odd characteristic belong to geometric group-law interpretation. Characteristic5 is permitted for a nonsingular model already in the stated shape.
- Retained exact source `/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs`: dispatcher606–624, generic mixed232–292, complete mixed828–878. Source common path references equation17, while primary paper separates secondary composition17 from generic disjoint18; bind expressions, not equation-number shorthand.
- Existing proof graph: canonical project `research/pencil_lifecycle_20261030/integration/lean/same-support/` modules WordAdd11Validity,WordDoubling,WordErrorExclusion,InvariantBranches; prior `integration/four-lead-inventory/law-remaining-obligations.md` and `/var/tmp/pencil-five-leads-invariants-20261030/REPORT.md`.

## Selected smallest useful slice

`add_projective` next dispatches degree-one+degree-two into `add12_weighted_complete(...).expect`. Select its K=0 shared-support prelude and cancellation return (lines835–866). Prove source-faithful exclusion of `coincident_y_mismatch`, and validity/nonzero weights for returned `p4` on the preceding cancellation guard. This extends one real output branch and establishes reusable support extraction; generic K!=0, tripling and positive double-plus-other remain distinct subsequent obligations.

For degree-one a=(X,Y,Z,W), degree-two b=(Q,R,S,T,Zb,Wb), normalize x=X/Z², y=Y/(Z⁵W), q=Q/Zb²,r=R/Zb⁴,s=S/(Zb³Wb),t=T/(Zb⁵Wb). Let u(U)=U²+qU+r,v(U)=sU+t. Assume field K, four weights nonzero, (U-x)|(f-y²) and u|(f-v²). No coefficient residual identities are premises.

Derive exact guard scales from the literal source:
- Kraw=Z⁴ Zb⁴ W * u(x). Thus Kraw=0 iff u(x)=0.
- L=Y*(Zb⁵Wb), RY=(S*(Z³*(X*Zb²))+T*Z⁵)*W.
- With D=Z⁵ Zb⁵ W Wb nonzero, L=D*y and RY=D*v(x).
- From input divisibilities and u(x)=0 derive y²=v(x)², hence L=-RY or L=RY. After inverse/cancellation guard failure, the actual mismatch None is excluded.

Define source p4 exactly: X4=-(Q*Z²)-X*Zb²; Y4=S*(Z³*X4)+T*Z⁵; Z4=Z*Zb; W4=Wb. Derive x4=-q-x and y4=v(x4). Because u(x)=0, factor u=(U-x)(U-x4), including repeated-root case. Then (U-x4)|(f-y4²); Z4,W4 are nonzero. Under Kraw=0 and L=-RY, actual source returns this p4 before mismatch/tripling checks. Cancellation needs no inverse of x-x4 or y: repeated support and zero ordinate must remain allowed in the algebraic statement.

## Exact acceptance target

1. Field theorem deriving scaled guards, ordinate dichotomy and p4 normalized validity from original valid-input predicates.
2. Literal UInt64 prelude/p4 expression model with source parentheses, using existing FieldExpr primitives. Prime5<=p<2³¹; all10 input coordinates canonical; four nonzero word weights; Field.n1,tablesNone and normal execution/counter headroom external. This path uses only mul/sq/add/sub/neg and literals0, so it needs no new exponent-loop or literal5 bridge. Canonical outputs and equality/negation reflection discharge actual guards.
3. Conditional word theorem: zero K plus cancellation guard returns canonical valid WordD1 with nonzero word weights. Companion theorem excludes mismatch under valid inputs. Separate kernel/axiom readback and independent source review.
4. Controls: nonunit weighted distinct support cancellation; repeated-support cancellation; y=0 precedence; invalid degree-one ordinate causing mismatch; zero-weight failure; p5 canonical boundary. Freeze exact fixtures before controls. Do not require a nonsingular curve for the abstract polynomial theorem; explicitly label algebraic fixtures.

## Next dependency after this slice

Generic mixed K!=0 returns z'=K*Zb*Z*Wb,w'=1. Nonvanishing is immediate from the guard; normalized numerator congruence and output divisibility remain. Its complete proof then supports positive shared-support double-plus-other using the reviewed doubling result and x4!=x to establish the secondary support denominator. Tripling retains its separate y-dependent guard/formula obligation. No whole-dispatcher or class-sum conclusion follows from cancellation alone.
