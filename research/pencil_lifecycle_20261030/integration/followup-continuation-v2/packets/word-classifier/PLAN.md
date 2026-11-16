# Exact same-support word classifier connection

Frozen target before new proof: for prime p with 5<=p<2^31, canonical four cross-scaled words s1,t1,s2,t2<p, literal UInt64 neg=((p-(b%p))%p) and sub=((a+neg b)%p), inverse-first/equal-second/zero-sub-third classifier equals existing classifySameU22 after casting UInt64.toNat into ZMod p. Preserve exact branch order; stop before delegated double/mixed payload. Counter transitions are not in this value-level classifier and require separate eager-prefix headroom. Multiplication/cross-scaling composition remains explicit preceding bridge; do not claim complete Rust operational semantics.

Reconciled old NatClassifier attempts: two missing Field(ZMod p) errors. Existing pinned Mathlib.Algebra.Field.ZMod defines exactly the needed instance and imports Field.Basic and ZMod.Basic. Use its pinned existing objects only, with hash inventory; no install/download. Literature: prior complete-law paper provenance retained; local UInt64 core arithmetic lemmas and Mathlib field instance are direct primary implementation sources. No new addition-law/novelty claim.

Controls: noncanonical equality, overflow, and inverse-first all-zero overlap. Native compile limit120s each. Fresh readback. New proof producer-only pending independent review.
