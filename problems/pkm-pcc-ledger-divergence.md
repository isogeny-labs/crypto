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
