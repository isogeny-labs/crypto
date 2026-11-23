# Concrete next construction: actual-quintic nonbranch norm order

This is a read-only proposal, not a theorem acceptance or execution admission. Exact inspected sources and saved logs are pinned in sources.json. The earlier four-lead inventory is superseded only where later evidence closes an obligation. No matched cohort is rerun.

## Existing geometry and evidence

The actual weighted Proj already has its affine and infinity charts, properness over Spec K (WeightedProjBase.weightedProj_proper_over_K), global smoothness over a perfect field with characteristic not two and squarefree quintic (WeightedProjSmoothFinal.smooth), and the actual infinity stalk DVR (WeightedProjInfinityDVR.projective_origin_dvr). Their current source hashes agree with the later source/build narrative in WEIGHTED_QUOTIENT_COMPONENTS.md; that narrative records direct pinned Lean4.34.1/Mathlib d13f23b builds and standard axioms. This pass did not locate separate per-module raw logs or a distinct-author fidelity review for those three later geometry components. Do not silently upgrade narrative evidence to independent replay.

Later directly inspected logs establish additional saved producer evidence: cross-chart-inverse-lean.log prints cross and gluedToProj_isIso, so the old conditional global chart-isomorphism gap is closed at that source's stated arbitrary-field scope. source-actual-quintic-branch-lean.log prints both actual-quintic branch-order specializations, including the separable corollary. nonbranch-dvr-lean.log and derivative-point-dvr-lean.log preserve the rational point DVR developments. These are saved kernel/axiom outputs with build success described in adjacent notes, not fresh checks or an independent source-fidelity review by this agent.

The old NONBRANCH_DVR.md sentence saying the derivative alternative is unformalized is superseded by DERIVATIVE_POINT_DVR.md. Similarly, older claims that global smoothness or the glued-to-Proj isomorphism is absent are superseded. WeightedPic0Bridge still constructs a stalk isomorphism, not a Picard group. RuntimeSemantics.PrincipalClassInterface still assumes the principal-divisor premise.

## Exact local theorem to construct first

Work over Field K. Let F be WeightedProjSmoothPoints.quintic a3 a2 a1 a0; assume F.Separable, (2:K) !=0, a rational point (x0,y0) with y0^2=fAt ... x0 and y0!=0. Let (b,alpha,c,q,r) be Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2, require b!=0 and the two actual-quintic Mumford divisibilities. Use the existing source-style n and residual support U, with u1=X^2+q1*X+r1, u2=X^2+q2*X+r2 and U=c^2*X^2+q*X+r.

Additional branch-selection premise: n.eval x0 = b*y0. This selects the zero of the intended factor, not merely a zero of its norm. It is necessary: norm zero alone can select the conjugate factor. For input-support corollaries, derive this premise from the input ordinate and interpolation congruences; do not assume the desired valuation or principal divisor.

Use the real type R := WeightedProjNonbranchZeroOrder.ProjPointRing a3 a2 a1 a0 x0 y0 hcurve. Its domain comes from Common22SeparableAffineDomain.projective_point_domain_of_separable; its DVR comes from WeightedProjNonbranchDVR.projective_point_dvr with 2*y0!=0 derived from h2 and hy.

Define rho : AffineRing ->+* R as the composition of algebraMap into Localization.AtPrime(pointIdeal ...) and (pointRingEquiv ...).symm.toRingHom. Define iota : K ->+* R through the existing affine constant map followed by rho; X_R := rho(xbar ...); Y_R := pointY .... Define N := n.eval2 iota X_R, B := iota b and each support image by the same eval2. There must be one checked evaluation map, not unrelated abstract N,B,u1,u2,U inputs.

Derive:
1. Nonzero b and 2 map to units (field units transported through iota).
2. N-B*Y_R is a nonunit: show its affine preimage lies in pointIdeal=ker pointEval using n(x0)=b*y0, then use localization at that prime and the actual stalk equivalence.
3. Y_R^2 equals F evaluated through iota/X_R, from the affine curve quotient relation and WeightedProjQuinticHead's actual coefficient/evaluation facts.
4. Transport Common22SourceHead.source_curve_eq through this polynomial evaluation map to obtain (N-B*Y_R)*(N+B*Y_R)=eval(u1)*eval(u2)*eval(U). Do not supply this norm identity as an extra conclusion-shaped premise.
5. Apply the existing point_norm_order, now with its domain, DVR, units, nonunit and norm arguments all constructed. Conclude ord_R(N-B*Y_R)=ord_R(eval u1)+ord_R(eval u2)+ord_R(eval U). This supplies source-specific nonbranch multiplicity allocation without a new axiomatized class interface.

Exact intended imports: WeightedProjNonbranchDVR, WeightedProjNonbranchZeroOrder, WeightedProjQuinticHead, Common22SourceHead (and its coefficient/support imports). Inspect existing affine evaluation lemmas before adding duplicates. Common22CoordinateRing.source_style_norm_factorization is an alternative intermediate, but its AdjoinRoot presentation requires a checked map to the same affine ring; direct evaluation of source_curve_eq avoids inventing that equivalence.

## Global class construction that still follows

The local theorem is one prerequisite. Construct a single rational-function field for the actual integral smooth projective curve and compatible local embeddings. Glue the local numerator to g=n(x)-b*y. Prove all finite zero orders, absence of other zeros, and infinity pole order6 when c!=0 versus5 when c=0; the existing abstract fraction-pole theorem must be instantiated with that same field and place. Rational K-points alone do not exhaust closed points over finite K: use all closed points with residue degrees, or an algebraic-closure development with explicit descent. Include repeated support and branch multiplicities.

Then define concrete effective Mumford divisors, a finite-support principal divisor of g, and the genuine quotient/class map. Establish div(g)=D1+D2+Dres-(4+deg Dres)*infinity, identify the normalized source output with the negative residual class using the cleared ordinate congruence, and instantiate residual_represents_input. An arbitrary AddCommGroup quotient with an assumed principal predicate would not close this step.

Finally preserve source order: add22_weighted first handles same-u inverse/equal/mixed paths, then shared-support composition, then c-zero degree-one and common degree-two. The shared-support branch performs doubling followed by two sequential add_point_to_result calls. A common-branch class theorem cannot be promoted to all branches without output validity/nonzero weights and both sequential additions. Existing same-support source proofs should be reused rather than reproved.

## Proposed finite proof validation (not approved or run)

First perform a dependency-custody readback of the retained Lean4.34.1/Mathlib d13f23b closure; some historical commands use migrated /dev/shm paths, which cannot be assumed live. Propose one120-second ordinary construction check after exact source/runner freeze, 8GiB address space, Lean -M3500 -j1 -s16384 and OS10MiB stack;110-second work cutoff plus cleanup inside120. If that closure cannot fit, report the missing dependency/build envelope before compiling. No silent retry.

Controls should include a nonbranch point satisfying the selected numerator zero, the opposite sign where norm zero does not imply numerator nonunit, b=0 and characteristic2 exclusions, and repeated support valuation additivity. Named axiom readback must reject sorryAx and missing/duplicate entries; retain exact failures. An independent author must inspect source fidelity and the global/local distinction after producer publication. Historical1260 target rows, rho controls and cost ratios remain unchanged.
