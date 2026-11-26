# H2 phase-1 qualification

Producer execution/data claim **PCC-20261123-509e12**, UNVERIFIED.
Nine native Rust release controls pass; release build and strict all-target
Clippy with `--no-deps -- -D warnings` pass. Logs and exact source/executable
hashes are retained in validation/. [pcc:unverified]

The module exports the same polynomial representation as `f4_fp::Poly` and
orders every exported polynomial by descending grevlex, with monic leading
coefficient. A dedicated assertion checks this boundary. Current-main full
library compilation has separately recorded unrelated syntax/fixture errors;
this qualification compiles the exact new source through its standalone
manifest. It is not a claim that the full library or IC pipeline passed.

The first successful test log preceded the explicit export-order assertion;
that log is preserved. Source inspection caught the BTreeMap term order
before any saved-case probe. The qualified source corrects it and the final
nine-control run passes. No numerical fixture outcome was used for this fix.

Tests include a complete nonradical specialization, degree/dimension changes,
an inconsistent ideal versus a rank-drop specialization, invalid support and
arity, composite modulus, resource refusal, corrupt provenance, malformed
proof indices/exponents, and mutual-containment rejection of a changed ideal.
Exact certificates, not matching root lists or a fixed dimension, determine
acceptance. This remains producer evidence, without independent signed replay.

The native worker has been copied read-only to the fresh local run root in
worker.sha256. The saved-case construction protocol remains unconsumed until
its exclusive execution marker is written after this qualification is pushed.
PLAN.md and input.sha256 are immutable prospective subjects in claim
PCC-20261123-3ad7ad. The production default is unchanged; opt-in refusals must
fall back explicitly. This is not yet module-syzygy pruning or a cost win.
