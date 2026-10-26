# Functional acceptance protocol

Scope: validate generation, bounded diagnostics, archive integrity, and storage
round trips. These are software acceptance checks; no total-cost winner or
large-scale performance improvement is selected.

Inputs are the committed JSON files in `examples/`, fixed seeds, and their
content hashes in the resulting manifests and receipts. The group-law tests use
exhaustive rational-point censuses in fields of sizes 16, 32, and 101. Additional
construction tests use the 83-degree binary field and secp256k1.

Success requires: on-curve and subgroup checks for all points; exact deterministic
regeneration; every retained relation witness re-adds to the queried point; a
separate direct pair enumeration agrees for arity 2; explicit statuses survive
state and degree caps; tampering is rejected; conditional-upload duplicate and
checksum-conflict tests pass; and a live fresh-directory fetch reproduces each
published archive when the requested bucket allows access.

Stop on any arithmetic mismatch, corrupt object, mismatched identity, unsupported
input, or exhausted explicit budget. Preserve failed construction rows and access
errors. Elapsed times in receipts are unqualified implementation diagnostics,
including artifact verification where named, not benchmark comparisons. The
repository-wide release-library test is a separate required publication gate.

The sample candidate comparison checks CLI compatibility and missing-value
behavior. Its small fixture panel is not a scientific selection experiment.
Before a substantive campaign, freeze discovery and held-out panels, candidate
families, equality of base-size policy, solver encoding and limits, cost model,
and acceptance criteria as described in the design document.
