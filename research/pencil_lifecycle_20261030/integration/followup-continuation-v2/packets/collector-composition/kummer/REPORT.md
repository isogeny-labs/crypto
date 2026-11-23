# Kummer original-G transitions and signed-pair uniqueness

2026-11-01. Read-only source construction; no scientific runs or target enumeration. Source snapshot: followup-continuation-v2/packets/kummer-graph-integration/src/pruned.rs, walk_impl_uncached.rs and recover_boundary_impl.rs. This is the preserved graph-integration candidate, not an assertion about a deployed runtime.

## Derived and checked component

SignedPair.unique_signed_lift proves: in an additive commutative group, if pi(x)=pi(y) exactly when y=x or y=-x, and G+G is nonzero, then simultaneous equalities pi(D)=pi(E) and pi(G+D)=pi(G+E) imply E=D. filter_unique specializes this to candidates passing the same two projection filters. zero_anchor_ambiguity proves that opposite distinct signs cannot be distinguished when the anchor is zero. Three declarations and three controls pass, with six separate named axiom readbacks.

The proof is directly tied to pruned.rs delta_lift/boundary_lift candidate filters. It establishes uniqueness conditional on the geometric fiber theorem, not that the coordinate map has those fibers, that candidates exist, or that generic Recover returns a matching divisor. It is distinct from the older fixed-offset orbit or scalar-label recurrences. Primary literature CCS sections 2 and 6 supplies quotient/recovery context: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf . Its Recover interface excludes two-torsion inputs; the present conditional uniqueness statement requires a non-two-torsion anchor but permits a two-torsion candidate. No formula-extension claim follows.

## Actual adjacent-pair invariant

Let D=aG+bQ in the intended Jacobian, with subgroup order r annihilating G and Q. Inputs must satisfy k~pi(D), kp~pi(D+G), kg~pi(G), k0~pi(0), all represented by nonzero projective vectors on the intended Kummer, with valid model constants. branch returns c in {0,1}. The required successor is k'~pi(2D+cG), kp'~pi(2D+(c+1)G), with labels ((2a+c) mod r,2b mod r). These geometric equalities are separate from the scalar recurrence. Label u64 arithmetic and annihilation premises must also be established for actual r.

## Finite transition obligations from step/advance

1. c=0: xadd_repaired(k,k,k0) must represent 2D; xadd_repaired(k,kp,kg) must represent 2D+G.
2. c=1: the mixed call must represent 2D+G; xadd_repaired(kp,kp,k0) must represent 2D+2G.
3. Plain xadd returns nonzero: prove its vector is on the correct geometric sum, not merely nonzero.
4. Repair action selected: prove each linear action L is translation by rational two-torsion T on the quotient, that the transformed difference is valid, and applying L again restores the desired sum. Also prove returned L(z) is nonzero; xadd_repaired does not separately test it after applying L.
5. Selected pair fails: signed_fallback reconstructs D' from labels, checks native validity and projects D',D'+G. Its correctness needs Jacobian addition/scalar/projection correspondence and the label invariant. It does not repair a wrong nonzero xadd result.
6. validate_transition=true compares native reconstructed expected projections and validity. With false, Some pairs are returned without this guard; its correctness therefore depends on obligations 1-4.

## Exact unresolved geometric equations

Write H for the source Hadamard transform and t for its four nonzero constants. For valid X=pi(P),Y=pi(Q),Z=pi(P-Q), source xadd computes W=H((H(X)_i H(Y)_i/t_i)_i), F_i=W_i^2 product_{j!=i} Z_j. Required identities are F_i pi(P+Q)_j - F_j pi(P+Q)_i=0 for all i,j, together with F!=0 on each claimed ordinary domain. For the doubling calls P=Q and Z=pi(0), prove this separately: the paper's abstract xADD interface explicitly states P!=Q, so the source's use at equal inputs needs its own formula-domain argument or xDBL bridge.

For each repair L_T require L_T pi(P)~pi(P+T), 2T=0, and L_T^2 projectively identity on the valid locus. Then transformed differential addition has operands P+T,Q and difference P-Q+T, and the second action removes T. Nonzero output and geometric-domain hypotheses are essential. The source same comparator checks cross-products and accepts a zero vector against anything, so it cannot supply these nonzero premises.

For generic signed recover, assume anchor degree2, ordinary general coordinates q/plus/minus, and nonzero D,Delta,h1 as computed in recover_boundary_impl.rs lines 251 onward. Let U=X^2-q[1]X+q[2], V=q_b1 X+q_b0 returned by those formulas. The concrete missing identities are U | f-V^2, project_any(U,V)~kq, and project_any(anchor+(U,V))~kplus. Those three obligations plus the fiber theorem and G+G!=0 give the intended signed divisor by unique_signed_lift. Scalar labels do not establish these identities.

## Finite signed-lift branch obligations

- Opposite neighbour: xadd_repaired(kq,kG,kplus) must yield pi(D-G), since kplus represents D+G and differential addition is sign-ambiguous. Show the selected sign interpretation explicitly before feeding Recover.
- All three ordinary conversions succeed: Recover checks special_anchor, D_zero, Delta_zero, b1_coefficient_zero, or returns computed. A computed output is not revalidated in this wrapper; formula soundness must supply the missing checks.
- D_zero: exceptional_lift enumeration must include the true candidate and its filters must be sound; failure remains None.
- Delta_zero: delta_lift enumerates at most four candidates, checks native validity and both projections, and returns only when one matches. Conditional uniqueness is now proved; candidate completeness and source projection fibers remain open.
- Failed conversion: boundary_lift checks zero vector/singular map, ordinary-versus-boundary chart, identity chart, or the degree-one conic q1*q3=q2^2. Identity checks neighbour projection. Degree one enumerates square-root signs and checks validity and both projections. Prove completeness by chart and retain nonsquare/ambiguous failures.
- special_anchor or h1=0 with ordinary conversion has no additional repair here. No branch may be silently counted as successful closure.

## Verification and next construction

The fresh Lean checks use existing copied Product/Mathlib dependency binaries as a disclosed trust boundary. First proof attempt lacked an available abel tactic and over-simplified a control; corrected explicit additive identities passed. One shell invocation used the wrong Lean root directory and failed before checking; that log is retained. No independent semantic reviewer has reviewed this new theorem packet yet.

Most concrete next construction: bind project_any/general_homogeneous as inverse projective maps with nonzero denominators on the chosen theta model, then prove the quotient-fiber property for valid Mumford divisors. That turns candidate-filter uniqueness from an interface theorem into a source-specific signed recovery result. The doubling specialization above is a separate immediate geometric obligation for transition closure. No new performance result is claimed.
