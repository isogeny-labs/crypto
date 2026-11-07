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


## 2026-11-07 — Same obstruction in S56 consumer PR3647

The source curve catalog is merged in cryptanalysis PR1431, but crypto
PR3647 cannot integrate current main while preserving the append-only
prefix rule. The [read-only S56 snapshot](../docs/curves/s56/ledger-fork-20261107.json)
binds branch head ee2b1eee and main e374755b and their complete ledger
hashes. Both extend the same 401-line prefix through sequence400; their
sequence401 events differ: branch PCC-20261107-d43ebf versus main
PCC-20261107-286789. Neither complete ledger is a prefix of the other.
These are producer metadata observations, not signed execution or mathematical
admission. [pcc:unverified]

No ledger event, claim body, trust entry or verdict is rewritten, selected
or rechained. Independent CI repairs (a test-helper Clippy diagnostic and
a versioned historical-registry input binding) can proceed on the published
branch without choosing a ledger. Ordinary journal conflict resolution is
held with the protected ledger conflict rather than used to bypass it.
[pcc:unverified]

The next action remains with the canonical PCC maintainers: specify and
review an officially supported concurrent-history reconciliation that
retains both event and claim histories and satisfies the actual required
prefix check. Exit requires that contract plus successful native ledger
checks against the actual main and final-head CI; completing this report
or repairing the unrelated CI failures does not meet that exit condition.
[pcc:unverified]
