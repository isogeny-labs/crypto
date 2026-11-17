# Exact scope

`NormalizedSquare.node`: rational w,r, r nonzero, and the explicit alternative
w=0 orw=r imply the actual identity w*w/r=w. Applying it to the Cauchy
node value needs the original puncture polynomial's nodal property.

`NormalizedSquare.valuation`: rational nonzero w,r and padicValRat2r=0 imply
padicValRat2(w*w/r)=2*padicValRat2w. The Cauchy-specific unit-normalizer
theorem is NOT proved by this declaration; its assumption is explicit. [pcc:unverified]

No mode bound, degree/recovery theorem, recursion or ECDLP correctness/cost
theorem is formalized here. These lemmas do not assume such a conclusion.

Executed: lake build exit0; actual staged-source repository proof gate PASS
with3 audited declarations (two named theorems and one generated instance
proof),0 failing and0 native-trusted; official import-inclusive
`lake env leanchecker --fresh --verbose NormalizedSquare` completed exit0.
The compiler printouts report only propext, Classical.choice and Quot.sound
for the named lemmas. The pre-staging FAIL_no_sources is retained as a failed
workflow attempt, not a proof. Signed standing remains pending. [pcc:unverified]

Pins: Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Standard foundations only,
no sorry, custom axiom, native_decide or kernel-check bypass in accepted proofs.
Claim records4e89a0 and94216f have standing unverified until a trusted signed
verdict exists. Local checks are producer evidence. [pcc:unverified]
