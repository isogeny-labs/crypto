# Explicit-one-worker guard confirmation

Run 20261019-040338-boolean-native-serial-gu-6ec5e9. Fresh same-binary confirmation after the initial four-worker regression. Five baseline A/A and five serial0/1 pairs per cell; each one-worker round additionally measures the legacy kernel with all three switches off. Exact input, raw/canonical output, rank, XOR and profile counts match. No host or earlier sample is pooled.

Host: Intel(R) Xeon(R) Platinum 8488C; rustc 1.99.1 (58cf8f9c3 2026-10-13); c7i.2xlarge on-demand; reserved cores/SMT siblings: one=3,7
four=3,7,2,6. The refined guard caches the explicit RAYON_NUM_THREADS=1 configuration and checks the actual current pool before selecting serial work. With an explicit four-worker configuration, both switch states skip the registry query.

All four multiworker controls are neutral under the frozen gate.

All eight one-worker incremental cells qualify faster. Primary n23 ratios are1.0802 F4 and1.0730 F5. Direct same-host legacy to candidate ratios are1.1141 and1.1209, with CIs[1.0980,1.1297] and[1.0608,1.1402]. All12cells have five eligible pairs; eleven rejected attempts remain archived. These ratios do not reprice the earlier complete-solver table.

Incremental pairs fix single-write/vector switches1 in both arms. Direct legacy ratios compare that round's all-off kernel with the candidate in the same binary, rather than multiplying previous gains. Admission, first eligible of three attempts, exact3125-resample CI and max(2%, twice A/A noise) are unchanged. Every slower, neutral or missing outcome remains in serial-guard-summary.json.

The switch remains opt-in and requires explicit one-worker configuration; nested multiworker pools keep parallel iteration. Four worker/switch configurations pass the existing GF2 reference tests before native timing. These are matrix calls, not complete recursive roots or decomposition. The eighteen-arm cost table retains its earlier source and serial-clear-off flags. Source: SERIAL_GUARD_PROTOCOL.md, run-serial-guard.sh, analyze-serial-guard.rs and runs/serial-guard/ in the raw archive. The prior slower control is preserved in its own report.

## Incremental pairs and four-worker controls

|Arm|n|Seed|Workers|Before ms|After ms|Ratio|Decision|
|---|---|---|---|---|---|---|---|
|matrix-f4|17|230|1|6.1083|5.8542|1.0434|faster|
|matrix-f4|23|230|1|53.4171|49.6467|1.0802|faster|
|matrix-f4|23|231|1|53.4533|49.8884|1.0695|faster|
|matrix-f4|23|232|1|53.7325|49.1784|1.0913|faster|
|matrix-f5|17|230|1|6.5614|6.3594|1.0276|faster|
|matrix-f5|23|230|1|56.6003|52.8025|1.0730|faster|
|matrix-f5|23|231|1|56.1721|52.0773|1.0721|faster|
|matrix-f5|23|232|1|56.9634|52.8105|1.0853|faster|
|matrix-f4|17|230|4|8.1520|8.1383|1.0093|neutral|
|matrix-f4|23|230|4|58.7065|58.0531|1.0180|neutral|
|matrix-f5|17|230|4|8.3863|8.4727|0.9863|neutral|
|matrix-f5|23|230|4|57.1523|58.0442|0.9974|neutral|

## Direct legacy to candidate; one worker

|Arm|n|Seed|Legacy ms|Candidate ms|Ratio|Decision|
|---|---|---|---|---|---|---|
|matrix-f4|17|230|6.3206|5.8542|1.0813|faster|
|matrix-f4|23|230|54.9114|49.6467|1.1141|faster|
|matrix-f4|23|231|54.8660|49.8884|1.1005|faster|
|matrix-f4|23|232|55.7328|49.1784|1.1320|faster|
|matrix-f5|17|230|6.8030|6.3594|1.0691|faster|
|matrix-f5|23|230|59.2979|52.8025|1.1209|faster|
|matrix-f5|23|231|57.9081|52.0773|1.1170|faster|
|matrix-f5|23|232|58.6563|52.8105|1.1057|faster|
