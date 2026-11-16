#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(leading: 0.6em)
= Historical data and current workflow source
Implementation checkpoint · unsigned data821559 · 2026-11-16

#image("data-view.svg", width: 100%)

The remaining required PCC failure is a missing whole archival input after a genuine artifact-store migration. Data2af6d7 retains both complete failures. Repair02 adds that exact archive to both existing loaders, preserving the original verifier, keyring, claims and signing restrictions. Required CI remains pending. #text("[pcc:unverified]")

A frozen unsigned data claim also pins the previous workflow file. Its authentic old bytes and the actual new Git source are distinct. The helper holds the entire current file, exposes canonical historical bytes only during data-attestation replay, and restores the current regular file before source tests or ledger publication. The executing workflow was already loaded from the actual Git source; historical byte checks do not establish current execution. #text("[pcc:unverified]")

Data821559 retains 12 metadata controls and an actual replay of the entire old b55 claim through the unchanged verifier. It exits 1 against current bytes, 2 (unsigned not_covered) during the historical data view, and 1 again after exact restoration. The original 22 tests, 62 canaries, eight earlier address controls, actionlint and workflow wiring pass at their stated scope. These are data/process observations, not signed scientific verification. #text("[pcc:unverified]")

The helper is limited to one workflow address and frozen statement/data attestations. Conflicting users, non-data claims, changed bytes, modified current sources and unrelated cleanup targets are refused. All original/current files, complete fixtures, receipts and failures remain. Safe source/address custody is uploaded on every CI outcome. Canonical scientific graphs are unchanged. #text("[pcc:unverified]")

*Next:* publish normally, inspect actual required CI and its complete metadata artifact, and merge only after all required checks/review. Scientific NEW28, shared-main ledger reconciliation and historical LP/EKS work remain separate. No ECDLP advantage or signed scientific standing is established. #text("[pcc:unverified]")

#text(size: 8pt)[Sources: PCC_ARCHIVE_AND_SOURCE_ADDRESS_PROTOCOL_02.md; PCC_HISTORICAL_WORKFLOW_ADDRESS_02.json; evidence/pcc-archive-and-historical-source-controls-02; original pcc.py; genuine bot532 and old workflow SHA e3b1a7fa….]
