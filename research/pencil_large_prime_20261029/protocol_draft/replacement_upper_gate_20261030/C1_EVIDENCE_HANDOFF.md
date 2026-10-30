# C1 evidence packaging handoff

This is orchestration documentation for `pencil-p186379-20261030-c1`, controller build `9a658f4d-b903-4083-9a77-b04c217cb6a0`. It records no result, analysis run, scientific replay or new execution. Parent owns cloud operations and final admission. Read actual immutable c1 artifacts before constructing any input below.

## Verified identities

Paths below are relative to `/home/ec2-user/crypto-pencil-signed-graph-20261029` unless absolute. Hashes were read from current local files; verify again before use.

| Artifact | SHA-256 |
|---|---|
| `research/pencil_large_prime_20261029/protocol_draft/replacement_upper_gate_20261030/protocol/replacement-frozen-protocol.json` full file | `4b1290eb3802af06238d70e83d6cdf049d85f483dc709ba89e40d9f73563166e` |
| Protocol canonical body, removing freeze | `18d5ea7d08097c6b1c952d979a71be5d05efa82902bee1bc1470b031345f37c9` |
| Adjacent `replacement-finite-coverage-review.json` | `1d18cad8da81aea06105b7022fc9ff26565ea4eb62a3e116fa925c54a8f2855f` |
| Exact prepared upper-gate plan | `481d795e6208cfb3ca648c463de2d7b232adfb7d28bebc341afa4f58e119dfcf` |
| Existing upper-gate prepared receipt | `a02f83518a76caa290aa78dbe4fba1aa4728eb85d97b519c46610f514b740d17` |
| Scientific ecbench binary | `a1c8ca89748a22b8a07ea00fd3983061be0d8197d5cae49fca883dcf5dd50eab` |
| `research/pencil_large_prime_20261029/native_price/target/release/analyze_cohort` | `e086ebeb7f261983af647284307a768e522b1f5921eabfb0146706f7eede360d` |
| `/tmp/pencil-unknown-runtime-target/release/pencil-native-canonical-export` | `b904a0da30851284da6301d4c97117cf427d6e3d32ea0ff242d90a928367bf7e` |
| New packaging adapter source identity | `5d14e1a9ea20e56c016cf80a12c85ac37d6b4653a6caf53530da69f9a78bc4e3` |
| `/tmp/native-observation-report-target/release/pencil-native-observation-report` | `55f58566fcac0de84b5eab5abbc3399122c6f3fc0c022399a875bf3004da7abf` |

The scientific instrument source remains `543a54b4b1c3be7052872af3ffe393a44c69a3c3`. Do not confuse artifact-child commit metadata with that source identity. The new packaging executable supports explicit unknown runtime; the older study `target/release` exporter does not.

## Inputs that must come from c1

1. Retrieve immutable versioned computation/fresh-epoch archives and manifests under parent ownership, verify downloaded hashes, and retain exact raw files. Preserve source session state; do not relabel an unfinished session completed. Retain durable bucket/key/version/hash locators so another host can recover artifacts. Keep computation and fresh epochs separate; never splice their rows to make a complete session.
2. Construct a new `pencil-price-cohort-input/v1` manifest using the exact upper plan and existing prepared receipt, new c1 session ID and records binding, unchanged scientific binary and native analysis provenance, and the new replacement coverage review. `protocol_sha256` is the **canonical body** `18d5...`, not the full-file hash. Preserve the exact scope `gate-p186379-target0/v1` with one workload,18 executions and5 pairs. Use b1 `terminal-partial/native-analysis/analysis-manifest.json` as a shape example only.
3. Construct explicit `pencil-execution-history/v1` for observed c1 rows only. Its protocol hash is also `18d5...`. Each row needs exact record_id/session_id/seq, known attempts/resumed, checkpoint when applicable, and immutable evidence bindings. Derive counts from actual controller/generation/record evidence; do not copy b1's attempts1/resumed0. Checkpoint writes are not restores. Unknown attempt/restore history blocks packaging; it cannot be converted to zero.
4. Bind resources in `analysis-manifest.separate_resources` before analysis. Canonical packaging verifies that sidecar equals the analysis's resource observation. A report-only sidecar is insufficient for canonical attachment. Preserve controller/allocated-instance intervals separately from active scientific execution. Unmeasured active duration stays null. Retain prior b1 and earlier costs separately, without refund or zero-cost inference.
5. Package request schema is `pencil-canonical-package-input/v1`. Bind protocol by its **full-file hash** `4b129...`, native_analysis, analysis_script, canonical_tool, execution_history and export_id. Preserve the existing14-file native analysis source inventory, unchanged pair/cohort binary bindings and build receipt in analysis_provenance. The scientific analyzer and mapping did not change. Do not copy b1 record/history/output bindings.

## Commands prepared from current CLI source

The capitalized paths are required future c1 input/output paths, not existing artifacts. Run only after actual data ingestion and the applicable parent authorization. The commands were inspected, not executed for this handoff. Use new output paths and capture every exit/stdout/stderr separately.

```bash
cd /home/ec2-user/crypto-pencil-signed-graph-20261029

research/pencil_large_prime_20261029/native_price/target/release/analyze_cohort \
  C1_ANALYSIS_MANIFEST.json --p186379-target0 > C1_NATIVE_ANALYSIS.json

/tmp/pencil-unknown-runtime-target/release/pencil-native-canonical-export \
  C1_PACKAGE_REQUEST.json --out C1_EXPORT.json

/tmp/pencil-unknown-runtime-target/release/pencil-native-canonical-export \
  check-export C1_EXPORT.json \
  --protocol research/pencil_large_prime_20261029/protocol_draft/replacement_upper_gate_20261030/protocol/replacement-frozen-protocol.json

/tmp/native-observation-report-target/release/pencil-native-observation-report \
  C1_REPORT_REQUEST.json NEW_REPORT_DIRECTORY
```

CLI correction: the canonical packager's `--out` names a new JSON **file**, not a directory; its implementation calls `write_exclusive` on that exact path. `check-export` reads that same file. The analyzer writes JSON to stdout, while the observation renderer takes a new output **directory**. These output contracts were rechecked against each current `main.rs`; this documentation correction changes no evidence or scientific execution.

Native package/check recomputes the finite accounting from retained records using linked reviewed analysis code. This is not independent mathematical/source-count solver replay and does not satisfy the gate's required15 measured replays by itself. This handoff executes neither type of replay.

## Neutral output and incompleteness

Retain all18 planned slots,15 measured slots,3 warmups,5 pairs and36 finite cells even if few rows exist. Missing rows are absent observations, not fabricated failures, attempts or zero-cost measurements. Incomplete evidence may produce nonzero package/check exits while retaining useful diagnostic exports; record exact findings and exits rather than weakening checks. All clocks remain unqualified, including a potentially complete fresh epoch.

The renderer request `pencil-native-observation-report-request/v1` binds native analysis, title, exact c1 run_id, scope, scientific source identity, sidecar evidence and resources. Produce standardized JSON, exact-fraction CSV, coverage SVG, editable LaTeX and PDF. A separate finite-model ratio SVG appears only when at least two ratio cells are available; missing ratios remain null, not zero. Float plotting is a visual projection only; exact fractions remain authoritative. Inspect final PDF pages and SVG rendering, retain hashes and independent review, and do not infer a scientific verdict from timeout or missing data.

Completion of this gate still does not itself authorize the remaining cohort. Preserve the frozen requirements for complete gate records, all15 measured independent non-wall replays, independent review and separate prospective cohort admission.
