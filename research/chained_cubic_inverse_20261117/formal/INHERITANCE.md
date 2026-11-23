# Conditional inverse theorem inheritance

The actual Lean4.35.0 primary source is retained in ../primary-inputs/Field.lean;
its pinned compiler/import hashes and full first output are in the input manifest
and first-frozen-15B-success-02/. KB/theorem MCP tools are unavailable in this
runtime; the wrapper previously returned127. No novelty assertion is made.
[pcc:unverified]

| Target | Existing declaration reused | New checked composition |
| --- | --- | --- |
| norm_cubic | standard CommRing/Grind normalization | specialize the written norm at b=0 |
| inverse_by_square | Lean.Grind.Field.inv_mul; Field.mul_inv_cancel; Semiring.mul_assoc; Semiring.one_mul | reassociate a*(a*a) inverse for nonzero a |
| cubic_inverse_pair | the two preceding lemmas; AddCommGroup.neg_zero; Semiring.zero_mul; Prod.ext | equality of the two inverse pairs |

All three print only propext, Classical.choice and Quot.sound. The checked
standard Field is a hypothesis: there is no instantiated Rust CubicField or
Picard theorem. The local pin records the same4.35.0 actually used by the direct
first proof call, so the repository-wide main-only proof gate selects that
version. The pin was added for CI context after the unchanged proof passed;
it is not claimed as a member of the earlier31-file execution lock.
[pcc:unverified]
