# 1. Certified domain and partial traces

Certified and partial-trace arms each decide50/66 queries; explicit decides38/66. The A/A arms have identical outcomes and eight conflict discrepancies. Partial traces do not add decided queries here. These are fixed-panel completion differences, not an isolated runtime or population result.

The frozen instrument internally replays every Found answer and checks Refuted against complete pair-table truth. Its raw format retains replay booleans, not successful point tuples: the post-run audit rechecks truth, but cannot independently replay those omitted tuples. The two later panels save actual points.

The accepted canonical artifact is export-final.json. Earlier export.json records an initial adapter metadata/schema attempt and remains pinned as historical evidence; raw rows/outcomes were unchanged. Automation stored the two raw JSONL files under the content hashes in .artifacts.json; identical local bytes were restored from committed source for validation.

Flow: planned264, attempted264, completed264; zero interruptions, exclusions or retries. No isolated clock/population inference. All claims are unsigned local evidence [pcc:unverified].

[PDF](report.pdf) · [recorded graph](outcomes.svg) · [plot data](plot-data.json)

Source data: research/trace_zero_experiments_20261109/rows.jsonl, summary.json, protocol.json and export-final.json. Source3ae8b21241256517a9c867b4e78fb25e02ff502e; frozen protocol16186e61242f683843585a932962141c6bf0c33a49997be0955fc9cc0d360af3. Common host receipt: research/trace_zero_comparisons_20261115/checks/host.txt. No independent signed review-run record. See the main study README for all reproduction commands and scope.
