# Repair the current Jacobian formula catalog

The main catalog check fails because its committed inventory describes 2,572
registry entries while the registry now contains 20,592. The native generator
now produces the complete current inventory using the same cost model and
200,000-iteration factor budget. This includes the exact author heads from
PRs 2669 and 2630 through ordinary merge commits, and builds on the cover
catalog repair in PR 2717.

All 2,572 old entries remain. Their route data, costs and classifications are
unchanged; one subgroup-provenance status changes from
`largest_prime_of_order` to the canonical `registry_representation`, retaining
the same order and bit count. All 18,020 added entries classify as `boundary`.
The 598 formula-advantage and 57 marginal classifications remain unchanged.
Modeled route count grows from 6,565 to 60,676; these are formula screens, not
executed transfers or measured improvements.

## Plain JSON and historical context

Two-space JSON exceeds GitHub's 100 MiB blob limit at 114,556,195 bytes. The
native serializer now uses ordinary one-space indentation, retaining field
order, every line break and every value. The resulting 94,146,832-byte plain
JSON is semantically identical to the full two-space preview. Existing
repository policy already keeps and permits this canonical text file in Git;
no compression, storage-policy override or label exception is introduced.

The dated transfer assessment retains its original negative finding about the
older formula inventory. That single background artifact now explicitly names
source commit `0c82a7dd1d1e6e3c847b49f8446d29d405beb0c5`, with its original
SHA-256 `eaf635a2a52cc7f595635bd9f967697de074cae38364bf7c032b254ff615aada`.
The resolver refuses a mutable ref, absent object or any attempt to pin current
implementation evidence. The schema and independent verifier enforce the same
scope. All other implementation files still bind current bytes. Only the
assessment's provenance section changes; its other 17 sections remain equal.

The report explains the historical reference and is rebuilt with the documented
XeLaTeX fallback and pypdf 6.0.0. Its 13 pages retain the 15,791-map cohort and
979 EC1 bindings in the text and PDF metadata. The first page was visually
checked. Existing dated DOT/SVG/PNG geometry remains current for that cohort.

## Source metadata and verification

The WeakCurves source and dist copy remain byte-identical. Their synthetic
GHS example now explicitly calls `0.122 [0.10, 0.16]` an arithmetic
M-equivalent cost ratio against plain rho, retaining the original study's
unpriced-hash and stronger-rho limits. No exact registry-curve verdict changes.
Only the two affected criteria checksum entries are updated.

The daily exporter reports exact committed catalog counts with neutral labels;
it does not treat coverage or modeled routes as attack-performance wins. The
full export completes in 2.08 seconds after Git objects are available. The
initial cold invocation took approximately 69 seconds while materializing
promised Git objects; that observation is retained rather than called a pass
of the one-minute steady-state check.

Local validation with Rust 1.99.1 passes all 28 Jacobian module tests, native
currentness, table equality, named queries, ranking/chart generation, and refusal
of an altered magic-number field. Six snapshot controls and full schema
validation, including wrong-path and mutable-ref refusals, pass. The ledger
generator is deterministic and the independent assessment verifier passes.
Raw successful, interrupted, unsuccessful and replacement checks, exact deltas
and source/binary digests are in [the evidence directory](ci-repair-jacobian-20261027/).

Conductor is absent in this runtime, so its documented absent-binary fallback
applies; edits use an isolated worktree. The earlier interrupted test build is
retained separately from the final 28-test pass. Fresh hosted validation with
the workflow's pinned Rust 1.98, current-head merge checks and review remain
required. No Sage experiment, solver benchmark or new performance claim is
created by this repair.
