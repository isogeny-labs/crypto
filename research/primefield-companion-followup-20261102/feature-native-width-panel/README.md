# Matched interval-width comparison driver

This driver connects the qualified interval counter to the fixed20 stride/cap cases. It preserves five policies: original shared width, planned shared width, separate native-width engines, an identical repeat, and original width with exactly one additional scalar multiplication per phase. Arm order rotates by case id; each arm starts with a fresh context. The doubled phase retains the initial cache only for shared arms. The initial phase remains separately visible if no rescue is used.

The independent native reference expands (X-1)^((r-1)/2) by repeated multiplication, extracts the three residue classes and reverses each delivery stream. It uses additions/subtractions only and neither the interval engine nor its binomial-inversion recurrence. It is a correctness fixture outside the measured region, not a runtime extractor. Every arm's delivered coefficient vector is compared to it before emission. The original4563-case cohort and public-target panels remain separate.

## Data and checks

`--panel` would emit exactly200 JSON lines:20 cases x5 arms x2 incremental phases. The constants are four(r,m) pairs281/35,1009/63,2017/63,2521/315 crossed with caps1,2,7,16,64. `analyze.py` rejects wrong row order/count/keys, boolean integers, negative/overflow counters, allocation failures, invalid coefficients, mismatched vectors, unequal A/A and a positive control other than exactly +1 scalar product. It retains signed deltas separately by phase/dimension. It does not implement elliptic or polynomial arithmetic and cannot validate a set of mutually forged coefficient vectors by itself.

`export.py` binds raw rows to a successful producer receipt, frozen protocol, source bindings, binary/commit and an admission URI/digest. It emits canonical experiment.export/v1 with100 execution cells: both incremental phases are named counter dimensions within one cell, not independent replicates. Failed/partial attempts remain raw receipts; they cannot become successful exports. Admission references and producer receipts are consistency evidence, not authenticated independent review. External admission and independent evidence review remain required.

Counting remains interval-engine-only. Caller allocation, dense-reference construction, group/source operations, coefficient arithmetic outside products and output formatting are excluded. The ten dimensions retain their predecessor definitions; no weighted total, peak-memory or timing claim follows.

## Software qualification

`python3 generate.py` writes two reversible entry-point adaptations; it leaves the prior instrument unchanged. `python3 validate.py FRESH_DIR` audits them, compiles with overflow checking, executes only `--self-test` on the disjoint r41,m5,cap2 fixture, then runs synthetic parser mutations. The software-test limit is90s build,30s native,30s parser. It does not execute the production20-case panel.

Use `/home/ec2-user/primefield-ecdlp-research-20261028/.venv/bin/python test_export.py` for synthetic export/refusal tests and that same interpreter with `../feature-native-resultant-protocol/authority/tools/experiment_protocol.py validate protocol.draft.json`. System python's jsonschema import lacks Draft202012Validator in this environment; the existing venv passes the canonical schema tool.

The temporary synthetic export tests intentionally use mock hashes, a mock freeze and a synthetic admission URI; they are deleted afterward and are never research evidence. Native and parser outputs are retained under validation01; export-test02 is the source-binding-enhanced adapter check.

## Execution status and next step

**No production comparison has run. protocol.draft.json is validated but not frozen.** Production binary/commit binding, runtime sizing preflight, and an admitted execution environment/budget remain unfilled. The development binary is not silently promoted to a production build. The proposed future ceiling is one120s attempt on1 CPU/1GiB, concurrency1, maximumUSD1 including overhead, withUSD0 reserved; this is not authorization to submit. No cloud action is taken by any script here.

After those bindings are available, freeze and commit the protocol before collecting rows. Publish the exact source/configuration and execution command before a cloud run. Preserve source setup and all failures. Compare initial and doubled deltas without summing overlapping inverse/product events. The public warm m35 recovery panel needs its own source-cost protocol.

Sources checked: work-accounting counter and source-bound receipt, planned-width panel, canonical protocol/export authority already pinned in resultant-protocol. KB unavailable; source fallback explicit. No novelty claim for polynomial expansion or instrumentation.
