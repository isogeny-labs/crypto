# Published compact native records

raw-rows.csv is the exact24-column worker CSV in schedule.tsv order,with one explicit header added. Nativeu/replicate0/warmupfalse are preserved. Each schedule line identifies the actual collector arm,case,seed andround; round0iswarmup. No field is converted from JSON to evade its data-size policy.

summary.csv andlatency.svg are bytecopies of the original frozen native analysis outputs. point-audit-progress.tsv is the original control progress stream. Full certificates,status JSON,event log,lossless export and referenced payload stay local with checks/main-result/local-data-receipt.json. Original protocol/export canonical acceptance fails324seed-table findings; see EXPORT-ERRATA.md.

These compact records do not supply a remote full replay or authenticated execution attestation. [pcc:unverified]
