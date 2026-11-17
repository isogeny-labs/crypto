# Reduced polynomial validity under affine normalization

2026-10-31, T-1140. Producer formal proof packet; independent source-fidelity review pending.

For a field K and a!=0, T_n(p)=a^(-n)p(aX+b) preserves polynomial degree, including the zero polynomial, and natural degree. If u is monic, T_deg(u)(u) is monic. Combining these facts with the preceding divisibility theorem preserves the explicit predicate: monic u, natDegree u<=2, degree v<degree u, and u|(f-v²).

The strict ordinate bound uses degree in WithBot(N), with degree(0)=bottom. This includes the identity (1,0), without a false natDegree(0)<0 requirement. The proof applies for every field with nonzero scale; no characteristic assumption is added here.

Seven main declarations and eleven fixed controls are checked, followed by separate named axiom readback. Controls include the exact rational quadratic substitution T_2((X+1)^2)=(X+2)^2 for a=2,b=3; the unscaled polynomial has leading coefficient4 and is rejected as monic. The identity and (X,1) on f=X^5+1 are valid and transport correctly. Equal support/ordinate degrees and nonmonic2X are rejected.

Source binding: three_leads.rs:945-980 slices up through d and vp strictly below d; valid checks d<=2, the leading slot1, and empty remainder of f-v² modulo u. The historical genus2_model_gate.rs:14-56 scales the support by a^d and ordinate by a^3 and retains stored d. The algebraic predicate matches these intended polynomial conditions. This packet does not prove the literal vector loop, trim/remainder implementation, machine-field arithmetic, global projective extension or divisor-class equality. Those remain separate bindings.

Attempt1 found unavailable imports/lemma names. Attempt2 exposed Lean module-root lookup after adding only the missing compiled modules. Attempt3 passed with a private union containing the three pinned cached modules and inherited symlinks. Three source modules and their copied cache files have custody records; the final launcher hashes the full dependency union before and after each check. No cache build or download was performed. The first two control attempts failed on mapped rational constants and a lemma name; controls3 passed. All failed receipts are retained. Existing compiled dependencies remain in the trust boundary; no independent binary rechecker or independent review is claimed.

Primary sources: Mathlib Polynomial Degree/Lemmas and Degree/Operations official documentation, checked against pinned source. https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Degree/Lemmas.html . No novelty claim. No scientific execution, timing, enumeration or cost ratio. Campaign limits remain unchanged. The diagram represents logical dependencies, not measured performance.
