# Optimized Jacobian.add source obligations

Read-only audit, 2026-11-01. Repository HEAD observed 2bf73d8b6b78918aa31c5ec1abfd16d221449f20; src/cryptanalysis/three_leads.rs SHA256 26e99b56dace8f848d055e8709bb989622e4caea65aec26d63e00805153dc3c6. No compile, experiment, job submission, repository edit, or cost measurement. This note contains algebraic derivations and proposed lemmas, not checked Lean proofs. Frozen 1,260-target scope and expired scientific authorization remain unchanged.

## Prior sources inspected

- AGENTS.md and .agents/skills/report-evidence/SKILL.md; newest dated shared log docs/session-logs/2026-10-31.md. This task's read-only repository boundary leaves the shared log untouched.
- packets/native-branch-composition/REPORT.md: independently reviewed nonmonic word division scope.
- packets/state-and-repair-closure/cantor-reduction/REPORT.md and reviews/cantor-reduction/REVIEW.md: existing C=0 branch and written Picard-class argument.
- packets/next-interface-derivations/pegcd/DERIVATION.md: generic fallback must retain noncoprime and inverse inputs.
- packets/fallback-geometric-invariant/REPORT.md; quintic-reduction-storage/REPORT.md; quintic-reduction-execution/REPORT.md: newest producer-checked generic fallback constructs valid stored output. The execution packet is present in the working tree; its publication state was not audited here.
- Ozbay Gurler and Hisil, Complete group law for genus 2 Jacobians on Jacobian coordinates (2024), https://link.springer.com/article/10.1007/s13389-024-00351-7, sections 2.1–2.3 inspected live. These distinguish polynomial Mumford conditions from divisor-class addition and require a nonsingular curve and characteristic not two for the geometric y^2=f model. No novelty claim.

All packet paths above are relative to research/pencil_lifecycle_20261030/integration/followup-continuation-v2/.

## Exact arithmetic scope

Keep prime p, 1<p<2^31, Field.n=1, diagnostic tables absent, active words canonical, normalized nonzero U inputs, and deg f=5 explicit. Geometric class claims additionally require odd characteristic and smoothness. Public Jacobian fields can be supplied arbitrarily: constructor checks at three_leads.rs:1139–1144 are not a universal input theorem. Existing constructor uses Field::new(p,1,0); extension-field and diagnostic-table dispatch are outside this audit. Physical allocation, counter headroom, compiler correspondence and array copies remain separate source interfaces.

## Branch map and immediate lemmas

1. Identity dispatch (1006–1011). From valid/canonical input b, returning b preserves it, and likewise a. For class addition, bind degree-zero normalized representation to the identity class. Retain source precedence when both degrees zero. Array padding/canonical struct equality is stronger than equality of the active decoded polynomials.

2. Fast 2+2 branch (1014–1043). Write Ua=X^2+qa X+ra, Ub=X^2+qb X+rb, Va=sa X+ta, Vb=sb X+tb. Define a=qa-qb, b=ra-rb, c=sb-sa, d=tb-ta, N=b^2-a*b*qb+a^2*rb, S1=c*b-a*d, S0=d*(b-a*qb)+a*c*rb. These names match aa,bb,cc,dd,norm,s1,s0 exactly. Guards N!=0 and S1!=0 are both needed for this shortcut; norm alone does not justify inverse_lead.

Suggested first lemma `quadratic_support_inverse_certificate`:
  Delta=Ua-Ub=aX+b; H=(b-a*qb)-aX;
  Delta*H = N-a^2*Ub.
Consequently N!=0 makes Ua and Ub coprime, including repeated roots within either support. Do not assume separable supports or distinct roots.

Suggested second lemma `quadratic_crt_slope_certificate`:
  Ub divides (cX+d)*H-(S0+S1*X).
Thus T=(S0+S1*X)/N satisfies T*Ua = Vb-Va mod Ub. V=Va+T*Ua satisfies both input residues; coprimality and input validity imply Ua*Ub divides f-V^2. This discharges exact division at1038 without assuming its quotient witness.

3. Batched inversion (1028–1035). Prove canonical decoded product N*S1 nonzero; model inv succeeds; ni=1/N and si=1/S1; all resulting words canonical. Source comparisons !=0 correspond to field nonzero only under canonicality. Reuse proved UInt64 modular arithmetic/inversion bridge, preserving its no-overflow bound rather than proving only an abstract field identity.

4. Exact leading coefficient (1036–1041). S1!=0 gives deg V=3 and leadingCoeff(V)=S1/N. Ua*Ub is monic degree4. Since deg f=5, Q=(f-V^2)/(Ua*Ub) has degree2 and leadingCoeff(Q)=-(S1/N)^2. Therefore inverse_lead=-(N/S1)^2 is nonzero and its product with leadingCoeff(Q) is1. Prove native pscale returns the same canonical normalized monic polynomial as pmonic(Q), and its last word is1. No monicity or zero-X4 hypothesis on f is needed for this degree/leading-coefficient argument; degree5 suffices. A sextic f would not justify this formula without a different leading-coefficient argument.

5. Reuse constructed reduction (1042 versus990–1002). U=Ua*Ub is already monic degree4 and V has degree3, so initial pmonic/prem in generic reduce leave their decoded values unchanged. Exact quotient from item4 and modeled prem(-V,outU) instantiate the quintic execution endpoint. Prove output U has length3 and output V length<3 with canonical/normal storage, residual divisibility, and the actual source-order calls. This supplies Divisor::from preconditions at950 and decoded Jacobian.valid; array-copy identity remains an explicit storage adapter.

6. Connect fast output to general fallback, rather than merely two valid outputs. N!=0 gives normalized first gcd D1=1 and second gcd D=1. Instantiate the six-word EEA endpoint and its normalization, then show the fallback numerator remainder agrees with the unique CRT interpolant V (degree<4). Generic fallback's U equals Ua*Ub; its reduction trace therefore agrees with the fast output. Need uniqueness of reduced polynomial remainder and canonical-normal-list decoding injectivity for exact stored equality; validity alone cannot establish equality. This route avoids reproducing class-addition reasoning independently in the optimized branch.

7. All other nonidentity cases (1045–1060). Existing constructed generic endpoint accepts arbitrary support gcds and both input residual divisibilities; dispatch needs to supply input-degree/normality/canonicality from stored Divisor fields and field configuration. Do not add coprimality to this branch: norm=0 includes common support, doubling and inverse cases. norm!=0,S1=0 is the existing C-zero exceptional branch (norm=-B,s1=C in prior notes), but generic endpoint should cover it without requiring a special-case theorem. Degree1+1 and mixed degree1+2 cases use this same source fallback; weighted complete-law files are a different implementation and do not replace this dispatch proof.

## Divisor validity versus class addition

The current generic endpoint and the proposed fast theorem yield monic U, short V, and U|(f-V^2). They do not identify the output with the sum in Pic^0. Needed class-level joints: normalized input ideal/divisor interpretation with multiplicities; general Cantor composition including shared supports and cancellation D^2; each reduction step changes the representative by the appropriate principal divisor with the minus sign on V; and field-of-definition/descent correspondence. Smoothness and odd characteristic belong here. Prior C-zero written class derivation is restricted and not a mechanized universal class theorem. Once generic class addition is proved, exact fast/generic output agreement transfers it to the shortcut.

## Recommended implementation order

Prove the two explicit polynomial certificates first, then native batched inverse and quotient leading coefficient. Compose the existing exact-division/remainder/reduction endpoint and add identity/fallback dispatch. In parallel, formalize generic divisor-class composition independently of source storage. Fixed reviewer controls should include N=0, N!=0/S1=0, repeated support within one coprime quadratic, noncanonical guard values, and a sextic leading-term counterexample to extending item4. These are proposed controls, not executed results.
