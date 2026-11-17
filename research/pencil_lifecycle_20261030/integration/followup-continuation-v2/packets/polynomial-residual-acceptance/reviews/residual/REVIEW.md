# Independent residual and acceptance review

Reviewer /root/monic_review, 2026-11-01. Prospective plan/source-derived obligations recorded before reading Residual, ListGuards and Acceptance. Parent supplied claim/module locations; earlier helper proofs had been independently reviewed. No sibling verdict accessed. Source reference three_leads.rs captured locally; producer inputs bound by hashes.

## Findings

Residual.neg_correct faithfully models a nontrimming pointwise negation. slots_correct models max-length zero-padded addition recursively; add_correct applies trim without changing the polynomial. sub_correct composes addition with negation; residual_correct composes the already reviewed product with subtraction to obtain F-V^2 and canonical output. The primitive bounded-word premises are preserved. Empty inputs and cancellation/trailing zeros are handled explicitly. The recursive list model does not assert that it is an operational Rust allocation model.

ListGuards derives polynomial natDegree length-1 from last stored word one, then monicity. shape_valid uses u.length=d+1 and v.length<=d to establish degree(v)<degree(u), including d=0 where v must be empty. acceptance_iff_divisibility states both directions under residual-length fuel bound. accepted_valid only needs actual successful acceptance and therefore correctly omits a sufficient-fuel premise. complete_valid_test retains d<=2 and last-word-one on the RHS along with Valid: it does not claim equivalence to algebraic validity for arbitrary list encodings. No conclusion is assumed circularly. No mathematical defect found in this explicit model composition.

Native source: psub is padd(a,pneg(b)); padded addition, negation and product order match the model at the value level. Jacobian.valid has degree/top guards and residual prem empty. Native up slices degree+1 coefficients, vp slices degree coefficients then trims, which motivates the explicit shape assumptions. Canonicality, prime modulus, p<2^31 and prime-field/no-table interpretation remain external premises; native valid does not establish them. The proof does not yet show fixed-array extraction/slicing correspondence, usize capacity arithmetic, allocation/counter behavior, or compiled-program equivalence. Fidelity: faithful_scoped for the list/word model; partial for full native acceptance.

## Fresh checks

Residual, ListGuards and Acceptance rebuilt in a fresh isolated snapshot. ResidualControls/ResidualReadback and AcceptanceControls/AcceptanceReadback were copied after producer stability confirmation and rebuilt successfully. Eleven main declarations and twenty producer controls have 31 passing named axiom readbacks. Six reviewer controls also passed in IndependentControls2, giving 37 passing named readbacks restricted to propext, Classical.choice and Quot.sound.

Reviewer controls cover exact-square cancellation, unequal subtraction lengths, degree-zero identity, degree guard rejection, trailing-zero divisor guard rejection and an acceptance case with malformed v length. The latter accepts d=0,u=[1],v=[1],f=[1] because residual is zero; it demonstrates why the external v.length<=d shape premise cannot be omitted when deriving algebraic Valid. Acceptance alone does not enforce all encoding obligations.

Retained failure: IndependentControls initially used bare decide, which could not reduce the well-founded slots recursion. Failed log includes sorryAx for failed declarations and is NOT passing evidence. IndependentControls2 uses explicit defining equations before kernel decide; all six corrected controls and their axiom readbacks pass. Producer inputs were not modified.

## Independence and workflow

Completed: separate reviewer; source-first prospective obligations; independent semantic inspection; fresh exact new-module rebuilds; independent boundary controls; axiom readback; model/native scope distinction; stable local copies and hashes. Reused imported compiled helpers and cached Mathlib retain their trust boundary. Missing: precommitted plan, immutable published inputs at dispatch, clean rebuild of all dependencies, independent kernel checker, multi-reviewer independence-checker gate and coordinator publication. This packet does not assert full workflow completion or official scientific promotion. No scientific run or repository mutation occurred.

Primary references inspected: three_leads.rs padd/pneg/psub/pmul, Divisor up/vp and Jacobian.valid; Residual/ListGuards/Acceptance; earlier reviewed arithmetic, product and monic remainder bridges. These use standard polynomial subtraction, convolution and degree/divisibility reasoning without a novelty claim.
