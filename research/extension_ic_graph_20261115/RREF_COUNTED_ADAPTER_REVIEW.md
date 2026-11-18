# Exact modular arithmetic for the incremental relation solver

Selected after29's actual public-target recovery and explicit post-call
coverage finding. The original raw, copied qualifier and14738-byte summary
stay unchanged. This patch corrects accounting, not solver arithmetic or a
claimed performance gain. [pcc:unverified]

Sources checked(KB): exact/negative RREFcounter queries unavailable,
/tmp/ic-rref-accounting-kb-search{1,2}.log. Read residual_walk.rs local
mul_mod/sub_mod/FixedFactor and the complete RelationSystem::insert_by,
its ops definition and small/wide equivalence controls; read jv_bitops
classification/batch hooks and authoritative bitop/v1 formulas. No new
pricing rule or novelty claim is introduced. [pcc:unverified]

The generic solver remains unchanged. A CountedRelationSystem adapter
delegates each insertion and reads its legacy product-count delta. Every
forward/backward elimination product has one modular subtraction, including
the RHS. An independent pivot normalization has columns+1products and no
modular subtraction. Thus products=delta and additions=delta-(columns+1)
on an independent insertion, or delta on a dependent insertion. The existing
inv_mod already charges its inversion; the adapter never adds it again.
[pcc:unverified]

The new hook_muls classifies products by the same modulus/owner/phase rules
as hook_mul; it adds no field-product surrogate to native integer events.
The public adapter is used by the2LP full path; unrelated generic callers
of raw RelationSystem remain outside this coverage change. Values, row
selection, RNG consumption and legacy product counts stay the same.
[pcc:unverified]

Meaningful controls are11products/5subtractions/2inversions for a solved
two-equation hand fixture, small/wide prime equivalence against the existing
solver (including the non-Shoup path above2^63), dependent/zero insertions,
and field/scalar/other-modulus batch classification. Native coefficient
scans, fixed-factor quotient/setup, allocation/copy/control work remain
explicitly uncounted. Complete costs and comparative flags stay null/false.
[pcc:unverified]

The replay entrypoint loads the immutable29affine certificate matrix before
counting, inserts it in the same order and stops when the same protected
target is determined. It must retain rank398 and7711970legacyproducts,
charge those products once, charge the exact subtraction count, and retain
the already-hooked inversion count. This is a recorded-matrix counter
validation without new relation search/descent or a new fullDLP trial.
Never compose its price with an old pipeline subtotal as a new end-to-end
measurement. A separate prospective protocol and source publication must
precede its actual call. Native adapter tests pass3/3; sieve tests25pass with
2existing ignored; sampler2/2 and public-input1/1 pass. Library/twoexample
warnings-denied Clippy, production examples, standalone qualifier13checks,
qualifier build/Clippy pass. Five root-source hashes remained unchanged
through testing. Saved-matrix scientific replay is still pending.
[pcc:unverified]
