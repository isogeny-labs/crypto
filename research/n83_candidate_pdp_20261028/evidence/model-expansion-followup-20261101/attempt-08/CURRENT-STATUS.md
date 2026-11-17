# Current status and dataset map

Attempt 08 completed after the preflight snapshot. Its original `README.md` and `preflight.json` are retained as pre-run records; the preflight's `new_labels_exist: false` describes the state before production and is not a current run status.

Protocol v5 (SHA-256 `3a041db64c7d6d70aa99200fe46500770998de5faffaf2acb7a2e366c855eb57`) generated and verified 144 inputs for spike35, spike36 and spike37. The top-form producer completed all 144 certificates, and fresh-process replay verified all 144. The ten-base model in `model/` uses those 144 records together with the preserved 336-system spike27–spike33 panel.

The combined model has 480 systems, 340 finite interval-midpoint proxy labels and 140 positive-dimensional rows without finite labels. Grouped held-out-base ridge won 0/10 folds; MAE was 2.3598021339571256 versus 0.0 for the matched-configuration baseline. The preregistered exploratory-lead condition is false. These are top-form quotient interval proxies, not exact FFD, LFD, affine solving degree or exact degree of regularity.

`DATASET-INDEX.json` lists every file by relative path, role, byte length, SHA-256 and custody source. The 144 ignored `system.sing` payloads are bound by per-directory `.artifacts.json` manifests; `tools/artifacts/artifacts.py verify --remote` reported `144 hydrated, 0 not pulled, 0 problems`. Tracked metadata, summaries, certificates, model rows and predictions are in Git. Four Python bytecode files are classified as reproducible local caches and are not scientific inputs or outputs. No files were removed.

The rendered ten-base report remains at `research/n83_candidate_pdp_20261028/reports/run-20261101T1848Z-ten-base-model/report.md` with its PDF, SVG and plotted data. The next useful measurement for factor-base ranking is a separately frozen exact FFD/LFD study on the retained PDP systems; the current model does not answer that question.


The tracked attempt-08 data and ten-base report were archived to S3 from Git commit `8111a5c6e7b9`; the archive readback was byte-identical and SHA-256 verified. See `S3-CUSTODY.json` for its immutable S3 URI, digest, contents, the separately stored 144 system payload references, and the r38/r39 fixed factor-base S3 prefixes.
