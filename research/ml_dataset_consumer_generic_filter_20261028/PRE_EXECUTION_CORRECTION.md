# Pre-execution wire correction

The first frozen config files accidentally included a descriptive schema field.
Production CaseConfig has no schema member. Removed only that field and refreshed
the config hashes before any fixture generation. All 40 mathematical parameters,
seeds, indices, limits, policies and family/twist choices are unchanged. The first
freeze remains in commit1202bf89359. No result informed this correction.

Each one-cell supervisor wrapper declares 8 curve seeds, 2 generators, 3 keys,
and both twists, but selects only shard94 or95 out of96. Mixed-radix decoding
therefore yields exactly curve_seed7/generator1/key2 and the frozen twist.
max-cases1 bounds execution; the other95 planned indices are not executed.
All40 wrappers and protocol bytes are frozen before generation.

The first preflight rejected the protocol for a missing formal freeze record.
No generate command or worker ran. Retained preflight stdout/stderr and old input
hashes under validation/preflight-missing-freeze. Added the required frozen_at,
filed_before_affected_data and canonical-body SHA256 record, then rebound all40
plans/specs. No mathematical config, sample or resource cap changed. The failed
preflight is not a software cell attempt or a retry of a generated cell.
