# PKM integration: divergent append-only PCC ledgers

- **Status:** open; report-only frozen-history conflict
- **Last updated:**2026-11-05

After normal publication to PR3452, new main d9c228db855c475da6ee11843654d6dca7bd97a3
and local20e84c4fb both extend the exact95-entry d30ledger. At11:48UTC the
main ledger has97entries and local has113; the common prefix is95 and neither
full ledger is a prefix of the other. Canonical `pcc ledger verify --base
origin/main` exits1. Its BROKEN/history-rewritten diagnostic describes failure
of that prefix condition; the snapshot shows divergent append histories,
not changed earlier95entries. [Snapshot and capture](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/validation/main-d30b9e0db-20261105/publication-20261105T1149Z/receipt.json).
Unsigned producer observation. [pcc:unverified]

AGENTS forbids selecting a side in frozen ledger/claim conflicts and forbids
rewriting the append-only ledger. No side selection, event rechaining, claim-body
rewrite, trust entry or prefix waiver is performed here. Official new local
observations may append after this dated snapshot; it remains immutable.

Next: canonical PCC maintainers must specify a provenance-preserving, officially
supported reconciliation contract for concurrent histories. Preserve every
claim body, event provenance and both Git parent histories; retain the actual
required prefix check. Review that route before any integration touches the
frozen ledger. A plain ordinary conflict resolution cannot claim this gate is
satisfied. Independently fix the inherited lint/report-path/runtime blockers,
review merged source against both parents, and obtain fresh affected validation.

## 2026-11-14 — orientation-anchor PR4118 occurrence

[PR4118](https://github.com/isogeny-labs/crypto/pull/4118) encounters the
same protected-history integration problem. Main
`e938a91494a3923f2ab0391d7ba54777a9b589e5` has1046 rows and feature
`56c5a20113a1d8506235c2ea8d3781d9b3d3e6b4` has1112 rows. Both preserve the
same exact1043-row original prefix, but their subsequent events diverge.
The canonical ledger is not a prefix of the feature. The read-only trial
conflicts in the PCC ledger and November14 journal; no source/evidence/claim
file conflict was observed. Exact hashes and counts are retained in
[conflict receipt](../research/prime_orientation_anchor_20261114/evidence/canonical-ledger-conflict-001.json)
and [trial](../research/prime_orientation_anchor_20261114/evidence/canonical-trial-002.log).
Unsigned producer observation [pcc:unverified].

Auto-merge was disabled. No side selection, event rechaining, claim rewrite,
new trust/signature, force push or prefix waiver is authorized or performed.
Protected conflict [reported on the PR](https://github.com/isogeny-labs/crypto/pull/4118#issuecomment-6875000168).
No Copilot conflict-resolution request was posted for the protected ledger.
The existing canonical-maintainer reconciliation contract remains the exit
condition; this occurrence does not create a weaker alternative.

The66-statement orientation source remains unchanged, with pinned gate66/0
and fresh replay exit0; its matrix base/rank obligations and separate broader
research objective remain open. Those unsigned component checks do not resolve
this integration conflict or imply a mathematical impossibility
[pcc:unverified].
