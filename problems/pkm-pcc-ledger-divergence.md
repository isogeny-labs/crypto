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


## 2026-11-15 production binary-subtraction continuation

Fetched main `30e066b0928150bcefa6c09bfe948830b175bba7` and the baseline
continuation branch have a semantic common prefix of 1,088 entries; main has
1,091 entries and the working continuation has 1,092. The official prefix check
fails at entry 1,088. Both histories and claim bodies remain unchanged.
[Current receipt](../research/binary_subtract_frontier_20261115/evidence/ledger-prefix-check.txt).
No conflict-side selection, rechaining or waiver was performed. This also
blocks crypto PR 4344's production continuation from being merged under the
existing gate. The paired cryptanalysis ledger still passes its prefix check.
Producer observation. [pcc:unverified]





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





## 2026-11-16: external PR4378 merge audit

The read-only [round-one merge audit](../research/prime_field_transfer_goal_20261116/receipts/round-02/round01-external-merge-audit.json)
finds external merge3f50ddffe7504818c1fd44dfbf362d54e3f3348c preserves its
incoming main parent's1184-row canonical prefix but not the previously
published PR parent's1182-row prefix; their common prefix has1179 rows.
Both parents' full normalized journal blocks and multiplicities survive.
PR4378 merged at66d1727b4ab2086a2855578fd5b0942954d5664a,02:30:36Z.
The reporting session neither performed nor repairs this frozen-ledger
resolution. These are unsigned provenance observations. [pcc:unverified]

The successor local-lifting component publishes from a fresh main with
unchanged claim files. Its *unpublished* local ledger snapshot is retained,
and new registrations use the existing append tool after the current main
prefix. That normal publication check does not retroactively preserve the
older published prefix or resolve this canonical reconciliation problem.

## 2026-11-16: external PR4388 merge audit

The read-only [round-two merge audit](../research/prime_field_transfer_goal_20261116/receipts/round-03/round02-external-merge-audit.json)
finds merged head619cb13e5af20fdb6fee5ed7904fb7fbc92b8bc4 retains all18
frozen source/proof/report pins, five claim bodies, the old PNG artifact
manifest and both parent journal-block multisets. The old published parent's
1193-row ledger is not a prefix (common1188); incoming-main's1190-row
prefix survives in the1195-row merged ledger. PR4388 merged at
6ebf026274395b5de50ddbc8d0523a72ba016a7b,03:21:51Z. This authoring
session neither performed nor repairs that external frozen-history
resolution. The closed PR's monitor is stopped. [pcc:unverified]

## 2026-11-16: PR4399 current-base fork before round06 publication

The read-only [baseline diagnostic](../research/prime_field_transfer_goal_20261116/receipts/round-06/ledger-baseline-diagnostic-01.json)
finds current main with1197 rows, published dfe88f759 with1233 and the round06
snapshot with1250. Main and both lane versions share1195 rows, then fork:
sequence1195 is PCC-20261116-942215 on main versus PCC-20261116-aeb339
on the published lane. The full published1233-row ledger is a byte prefix
of round06; its new observations were appended with the existing tool.
The current1197-row base is not a prefix of either lane snapshot. This blocker
already exists at dfe88f759 and is not repaired by round06. Diagnostic claim
is in receipts/round-06/ledger-baseline-claim-01.txt, unsigned. [pcc:unverified]

The earlier historical1195-row main-prefix preservation statement does not
establish preservation of today's1197-row base. No canonical event, frozen
claim, prefix gate or trust policy is changed to resolve this fork. Both Git
histories and the exact diagnostic remain available. A later new claim can
append after the dated snapshot without changing its observed counts.
Required action remains a canonical provenance-preserving reconciliation
contract; a normal conflict edit cannot preserve both different linear prefixes.
