# Chained transfer measurement admission and append-only ledger synchronization

- **Status:** open
- **Kind:** access/tool limit, measurement gap, and PR synchronization gap
- **Affected work:** [PR #3877](https://github.com/isogeny-labs/crypto/pull/3877),
  [queue #3904](https://github.com/isogeny-labs/crypto/issues/3904),
  [CT05–CT07](../research/chained_jacobian_transfers_20261110/QUEUE.md)
- **Next action owner:** continuing research session; ledger policy reconciliation requires the repository's accepted procedure
- **Last updated:** 2026-11-11

## Problem

The fixed F101 native transfer can be developed and checked locally, but a
controlled solving-cost comparison is not admitted without S3 checkpoint
transport and live IsoLab budget/capacity visibility; the legacy PR branch
also has a PCC ledger fork relative to current main. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** `scripts/checkpoint.sh gate` reports no AWS profile,
  `CHECKPOINT_UPLOAD_TOKEN` or token file. No new experiment/benchmark launched.
  The native tests and Lean development builds are distinct local development
  work. `crypto-autoresearcher` is vault locked without its KEY binding;
  research-KB MCP/CLI and ecdb credentials are unavailable. [pcc:unverified]
- **Observed:** `pcc ledger verify --base origin/main` fails. At the recorded
  comparison there are 569 common rows, 584 base rows and 573 head rows;
  the first new claims on the two sides differ at row 569. No entries were
  rewritten. A Copilot base synchronization request was posted to #3877,
  explicitly prohibiting picking one ledger history or rewriting hash links.
  Additional local claims after this comparison are retained in the branch.
  [pcc:unverified]
- **Observed:** the global bench-record gate names missing historical raw
  records after artifact manifests and referenced benchmark entry directories
  were materialized. No new benchmark record or leaderboard row is introduced
  by this work. Historical #3877 ecbench audit failures identify unrelated
  archived records as well. The exact global checks remain unsatisfied.
  [pcc:unverified]
- **Derived or proved:** none of these access or synchronization failures is
  a mathematical obstruction to a DLP transfer. The fixture has a genuine
  Richelot hop, a nonzero endpoint, zero projection controls and a complete
  finite-fixture shortcut comparison; all remain producer assertions until
  their actual PCC standing permits more. [pcc:unverified]
- **Missing capability:** admitted external experiment transport/accounting,
  vault access for lead-atlas writes, an accepted append-only ledger
  reconciliation, and signed independent verification.
- **Observed full-library failure:** completed root attempt04 has6184 pass,
  13 fail,133 ignored (PCC-20261111-8ffeb3). Restoring exact tracked fixture
  blobs fixes6 failing tests. Five still require an untracked archived
  targetwalk records.jsonl; two require absent container time-namespace
  metadata. Raw failure and targeted reruns are retained in the study evidence.
  The full run remains failed; no raw archive or kernel metadata was fabricated.
  [pcc:unverified]
- **What this does not establish:** a speed advantage, its absence for all
  algorithms, security, or nonexistence of cross-characteristic computation.

## What would unblock it

- [ ] A successful checkpoint gate using configured credentials/token, with
  actual S3 readback where required by the repository contract.
- [ ] Live remaining daily/monthly IsoLab allowance and pool/quota visibility,
  then a finite frozen request under the EKS skill; no quota or identity changes.
- [ ] A ledger synchronization that retains both histories under an explicitly
  accepted repository procedure and passes the base-prefix/hash-link check.
- [ ] Final-head applicable CI/review gates and signed PCC standings; a local
  self-check or unsignatured verdict does not discharge these.

These conditions admit a comparison and PR integration. They do not themselves
establish an algorithmic gain. The comparison must include the direct elliptic
shortcut τ φ [2] discovered on all 102 source points. [pcc:unverified]

## Next discriminating step

- **Action:** reconcile the pending base synchronization request, preserve the
  current source commits, and recheck access before CT05. Continue CT09 native
  generalization/proof work if these external gates remain blocked.
- **Inputs and controls:** exact fixed source/target equations and subgroup in
  the report; direct elliptic, full Jacobian and collapsed-route arms; preserve
  historical Python and all failed receipts.
- **Limits:** no paid build/job until the existing budget and checkpoint gates
  admit it. No ledger rewrite, force push, protection bypass or plaintext vault
  write. A pending check is not a pass.
- **Outcomes:** admitted access permits frozen benchmark execution; denied or
  unknown access retains CT05 as blocked while other authorized work continues.

## Evidence links

- [Current report and proof map](../research/chained_jacobian_transfers_20261110/REPORT_20261111.md)
- [Source review receipt](../research/chained_jacobian_transfers_20261110/SOURCES_20261111.md)
- [Original broader prime-cover backend gap](prime-cover-balanced-jacobian-and-induced-maps-missing.md)
