#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Evaluating modes without a coefficient scan
T-1904 · Recurrence audit and residue-section construction · 2026-11-08

A supplied quotient value v now admits direct evaluation of every mode: compute (X−1)^h modulo X^m−v and read its m components. This uses binary powering and never constructs the long mode coefficient vectors. The implementation reuses the modular-character-power principle already present in sign-history replay.

#image("flow.svg",width:100%)

*Exact identity.* In R=Fᵣ[Y]/g with monic nonconstant g, X^(jm+e)=v^j X^e in R[X]/(X^m−v). Consequently the coefficient of X^e in the reduced power is Fₑ(v). The quotient value can be a nonunit and g can have repeated factors. No inverse of v is needed. This is prime-field quotient arithmetic, not characteristic transport.

*Qualification.* Validation03 passes 1,008 dense-reference section equalities and 56 roots-of-unity projection equalities over F₉₉₈₂₄₄₃₅₃. Each large-field call uses 34 products in a seven-component scalar quotient algebra, for a mode-1 length of 71,303,168. These counts are not timings or base-field operation counts. Producer evidence awaits independent signed review.

*Why the block audit changed the plan.* The denominator-cleared triangular recurrence is valid: 448 quotient cases pass, including 168 nonunit cases. But a b-step block has denominator degree exactly mb; 384 degree checks pass. A degree-one square-root recurrence theorem cannot be used with m omitted. The explicit degree/number-of-blocks tradeoff is mb versus N/b. This audits a representation, not every possible recurrence algorithm.

*Costs and next.* The implemented dense section product uses m² coefficient products plus m(m−1)/2 wrap products in R. Binary powering uses bit-length(h)−1+popcount(h) such products. Quotient reduction, allocation/copies and the m·deg(g) output capacity remain. Integrate after candidate pruning, where g is small; initial candidate construction and source extraction remain separate costs.

*Failures and proofs.* Validation02 failed compilation on a moved fixture vector, then mistakenly ran the old binary; that output is explicitly rejected. Validation03 uses a fresh binary and stop-on-error and passes. Lean4.34.1 checks limited composition, normalization and wrap-substitution lemmas, not full quotient/runtime refinement. Sources, receipts and failure snapshots are retained.

*Sources.* Existing orbit.rs character powering and reference.rs pullback; Bostan/Gaudry/Schost, section6 Theorem14, author PDF at https://cs.uwaterloo.ca/~eschost/publications/pollard.pdf. KB unavailable; scoped exact-source fallback. No cloud or timing panel.
