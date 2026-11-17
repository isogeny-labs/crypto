#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Phase 7 norm proof reconciliation
2026-11-05 · T-1588 · Preservation and read-only archive recovery

== Recorded outcome
The preserved native receipt reports passed: Controls exited 0; ExpectedFailure exited 1; both children were reaped; cleanup errors were empty. All three outer phases exited 0. Total recorded wall time, derived by summing the phases, was 284.045741254 seconds.

#image("phases.svg", width: 100%)

The frozen runner checks 14 named axiom readbacks and three malformed-evidence rejection controls before reporting pass. All nine frozen source hashes match. Both custody receipts report 16,736 files and 3,111,892,318 bytes with the same manifest hash. These are preserved historical outcomes, not fresh theorem executions.

== Artifact availability
The raw phase 7 stdout, parsed axiom JSON and telemetry directory are absent after migration. Individual axiom lines and the expected-failure diagnostic were not independently re-read. The passed native receipt is retained without replacing missing raw evidence with a new run.

Two predecessor archives were downloaded read-only and verified by full SHA-256 and byte count. They recovered the phase 4 Nonbranch.olean (416,112 bytes), both custody manifests, and the v3 dependency closure. Exact URIs, member names, hashes and private locations are in ../recovery-20261105.json. Original temporary paths and historical receipts were not overwritten. Cache contents themselves have not been revalidated.

== Independent review remains prospective
All ten frozen review-source hashes match. The revised runner masks signals during child registration and requires a specific single unsolved False rejection. Its twenty independent fixture declarations and ten producer declarations remain unexecuted by the reviewer. Source fidelity is provisional; no final review verdict is issued.

The mathematical scope is an actual rational nonbranch point stalk with explicit branch selection and nonzero scalar and ordinate. Global divisors, infinity and arbitrary residue fields remain separate obligations.

== Evidence and next dependency
Sources: phase7-sequence-receipt.json, lean-phase7-receipt.json, both custody-phase7 receipts and phase7-recipe.json in the parent directory; recovery-20261105.json; the adjacent actual-nonbranch-norm-independent-review freeze, plan, replay and preparation records. Full hashes and source-pin checks are in result.json. CSV and editable SVG retain the displayed recorded values.

Preserve prior failures and unexecuted proposals. Recover phase 7 raw outputs from another migration artifact if available; otherwise retain the explicit gap. Fresh independent replay requires prospective release and valid resource limits. No compiler or scientific run was launched for this reconciliation. Canonical graphs and research ledgers are unchanged.
