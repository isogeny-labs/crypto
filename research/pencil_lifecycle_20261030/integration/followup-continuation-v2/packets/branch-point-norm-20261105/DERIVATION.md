# Simple branch-point local norm and multiplicity

[pcc:unverified] Source construction and hand derivation, not a compiler result. The existing repository already supplies the actual rational branch stalk DVR, ord(y)=1, ord(x-a)=2, and a source-specific numerator-order-one theorem. This packet reuses those results at source level and adds the paired-conjugate/product construction and a derivative obstruction for overlapping supports. No compiler, test or trial was run.

## Objects and exact normalization

Let K be a field of characteristic different from 2, f=X^5+a3 X^3+a2 X^2+a1 X+a0 separable, and P=(a,0), where f(a)=0. Let R be the actual weighted-Proj branch stalk ProjBranchRing, identified with the affine point localization by branchRingEquiv. Write vP for its normalized additive DVR order, valued in Nat-infinity on R, with vP(y)=1. The base order va on K[X] at X-a is normalized by va(X-a)=1. They are different valuations.

Factor f=(X-a)q. Simplicity gives q(a)=f'(a)!=0, so q is a unit in R. The curve relation y²=(x-a)q makes x-a a unit times y², hence vP(x-a)=2. For any nonzero polynomial h=(X-a)^m h0 with h0(a)!=0, its germ has vP(h(x))=2m=2va(h). This is the ramification index 2. Stacks 00PD explains the normalized unit/uniformizer decomposition; 0EXQ defines the ramification index by precisely this change of uniformizer. Residue degree is 1 at this rational point.

## Local norm calculation

Set g-=n(x)-b y and g+=n(x)+b y with b!=0. If n(a)=0, write n=(X-a)m. Thus n(x)=y² q^-1 m(x) and g-=y(y q^-1 m(x)-b), g+=y(y q^-1 m(x)+b). The parenthesized residues are -b and b, so both are units: vP(g-)=vP(g+)=1 and vP(g-g+)=2. This proves the germs are nonzero and their extended orders finite; conversion to Nat is then legitimate. If n(a)!=0, both factors have nonzero residue and order0. Characteristic!=2 gives the usual distinct conjugation in the quadratic function-field interpretation. The elementary simple-branch source lemma itself needs only a field and simple root; it does not claim separability of that field extension in characteristic2.

Under the polynomial identity H=n²-b²f=u1u2U, the local norm factorization gives

  vP(u1(x))+vP(u2(x))+vP(U(x))=2vP(g-).

At a support root the values are 2 and 1 respectively. On the BASE instead,

  va(u1)+va(u2)+va(U)=va(H)=vP(g-)=1.

The factor 2 in the curve stalk does not double the Mumford polynomial root multiplicity. One may also see the base multiplicity directly: H(a)=0 and H'(a)=2n(a)n'(a)-b²f'(a)=-b²f'(a)!=0. Exactly one support polynomial has a simple root at a; the other two have nonzero value. Repeated branch roots or two support factors sharing this branch root contradict these generic premises.

## Concrete nonvacuity and naive-formula countermodel

Over Q take f=X^5-X, a=0, u1=X²-X, u2=X²+1, and both input ordinate polynomials zero. The input divisibilities hold because f=X(X-1)(X+1)(X²+1). In the actual source coefficientData with (q1,r1,s1,t1,q2,r2,s2,t2)=(-1,0,0,0,0,1,0,0), direct substitution gives (b,alpha,c,q,r)=(-2,0,0,-4,-4). Thus n=0, U=-4(X+1) and n²-u1u2U=4f=b²f. All values here are hand algebra, not experiment output.

The polynomial f is separable: a common root of f and f'=5X^4-1 would satisfy 5f-Xf'=-4X=0, hence X=0, where f'=-1. At P=(0,0), g-=2y and g+=-2y each have order1. The support germs have orders (2,0,0). The naive nonbranch equality would say 1=2 and is false. Base polynomial multiplicities are (1,0,0), which correctly sum to1. This fixture satisfies the actual source determinant guard b=-2!=0 and two quadratic input-support divisibilities, so it is not merely an unrelated abstract norm example.

## Prospective Lean source and exact remaining bridge

BranchConjugateOrders.lean.txt contains three no-hole prospective declarations:

- actual_branch_conjugate_orders reuses projective_eval_factor and projective_branch_x_factor_zero_order for b and -b, then AddValuation.map_mul. It proves both orders1, product order2, non-top and Nat versions on the actual branch stalk. No desired valuation identity is assumed.
- numerator_root_from_norm derives n(a)=0 from a support root using the actual quintic and the existing branch_norm_forces_numerator_factor.
- two_support_roots_force_b_zero differentiates the polynomial norm identity. Two support roots force n(a)=0 and then b²f'(a)=0, so simplicity forces b=0. Permuting supports gives the other pairs on paper. This is the mathematical exceptional-case condition, not a verified Rust dispatcher correspondence.

Separability supplies hsimple via WeightedProjQuinticHead.derivativeAt_ne_zero_of_separable. Existing source_norm_identity supplies the polynomial identity for coefficientData. Source-specific input values and local branch objects are already in the cited modules; none are replaced by arbitrary valuations or assumed principal divisors.

The new source does NOT yet formalize the support-germ sum equation. The concrete remaining transport lemma is: for E=eval₂RingHom(projectiveCoeffHom)(projectiveX), prove E(f)=projectiveY² using quotient_affine_relation, the affine localization algebraMap and branchRingEquiv.symm, then map the polynomial identity to obtain (E(n)-B*Y)(E(n)+B*Y)=E(u1)E(u2)E(U). Unlike the nonbranch source, these branch evaluator and ProjPointRing evaluator definitions have different stalk presentations; no definitional equality is assumed. Apply additive multiplicativity after that bridge. Formalizing vP(E(h))=2va(h) also needs the explicit residual-unit criterion and a finite polynomial-root factorization; its paper proof above fixes normalization and hypotheses. These are precise unimplemented Lean obligations, not premises smuggled into the new theorem.

Infinity orders, nonrational closed points and residue-degree descent, finite support and assembly in the actual global principal divisor remain separate constructions. The output here is the rational simple-branch local statement and classifier, with no global divisor asserted.

## Sources and status

Primary local files and hashes are in sources.json. Sources checked (KB): research-kb MCP unavailable in the exposed catalog; scripts/research-kb missing in the sparse worktree. Known primary paths and the finite-order independent review were used. No novelty or exhaustive absence claim follows. Primary references: https://stacks.math.columbia.edu/tag/00PD and https://stacks.math.columbia.edu/tag/0EXQ. The existing source declarations have not been recompiled here; the new declarations require exact import binding, compilation, named axiom readback and independent source review. Proposed controls should retain b=0, nonsimple f and the naive 1=2 countermodel, but none were executed in this task.
