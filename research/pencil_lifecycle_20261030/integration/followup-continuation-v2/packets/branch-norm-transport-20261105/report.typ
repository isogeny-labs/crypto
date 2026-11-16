#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Actual branch evaluator and norm transport
[pcc:unverified] Five prospective Lean declarations construct the missing actual branch transport and support-stalk order sum. No compiler, test, axiom audit or trial was run. Predecessor afccdb7d86 remains unchanged.
#image("transport.svg",width:100%)
== Construction
The branch evaluator E is proved equal to rho composed with the affine polynomial evaluator. Here rho is the actual affine localization map followed by branchRingEquiv.symm. Polynomial ring-hom extensionality reduces compatibility to constants (the algebra tower identity) and X (the projectiveX definition). The already-derived affine quotient relation then maps to E(f)=Y².
Mapping n²-u1u2U=b²f through E and substituting the curve relation proves the conjugate-product/support identity. At a simple branch with nonzero b and a support root, the predecessor proves both conjugate orders1. Applying additive valuation multiplicativity therefore gives the support-stalk sum2. The source wrapper derives its polynomial norm premise from coefficientData and the two input-support divisibilities; it does not assume the desired local valuation identity.
== Deliverables and exact limits
Declarations: eval_eq_rho_affine; curve_eval; stalk_norm; support_stalk_order_two; source_support_stalk_order_two. No new axioms or proof holes. Their elaboration and imported predecessor proof remain unverified. input-bindings.json pins source modules and inspected APIs, not a complete import closure.
The remaining base bridge factors nonzero g=(X-a)^m t with t(a) nonzero, proves E(t) a unit, and uses order(E(X-a))=2 to obtain order(E(g))=2m. This packet does not implement that bridge. The base norm multiplicity at a support root is1; curve order2 does not double Mumford multiplicity. Infinity and global divisors remain separate.
Sources: exact quotient/localization definitions, branchRingEquiv, Mathlib algebra-tower and polynomial-evaluator APIs; predecessor source pins and Stacks00PD/0EXQ references are in DERIVATION.md. Exact declared outputs: result.json and declarations.csv. Diagram is editable SVG. Machine validation and independent source review remain required.
