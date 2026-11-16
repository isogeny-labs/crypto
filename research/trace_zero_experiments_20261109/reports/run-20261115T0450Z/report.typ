#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 8.5pt)
= 1. Certified domain and partial traces
2026-11-15 · crypto PR3744 · TZP-20261109-finite-domain-partial-trace · unsigned local evidence

#image("outcomes.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 3pt,
 [Arm], [Found], [Refuted], [UNKNOWN], [SAT conflicts],
 [explicit], [28], [10], [28], [6506],
[certified], [36], [14], [16], [6876],
[partial-trace], [36], [14], [16], [5384],
[certified-aa], [36], [14], [16], [6871],
)

Certified and partial-trace arms each decide50/66 queries; explicit decides38/66. The A/A arms have identical outcomes and eight conflict discrepancies. Partial traces do not add decided queries here. These are fixed-panel completion differences, not an isolated runtime or population result.

The frozen instrument internally replays every Found answer and checks Refuted against complete pair-table truth. Its raw format retains replay booleans, not successful point tuples: the post-run audit rechecks truth, but cannot independently replay those omitted tuples. The two later panels save actual points.

The accepted canonical artifact is export-final.json. Earlier export.json records an initial adapter metadata/schema attempt and remains pinned as historical evidence; raw rows/outcomes were unchanged. Automation stored the two raw JSONL files under the content hashes in .artifacts.json; identical local bytes were restored from committed source for validation.

*Flow:* planned 264, attempted 264, completed 264, not-completed 0, excluded 0, interrupted 0. Every UNKNOWN remains in the denominator. Zero retries and zero paid resources. 33 common known-answer instances: n7/9/83/131,d4/5, planted/cancellation/two full-width uniform-scalar targets, plus the n5 signed-base UNSAT control.

*Controls and scope:* the preregistered small-field positive pilot gate passes; two paired repeats are the same targets, not independent population samples. Per 33-query repeat counts: explicit: 19/19 decisions; certified: 25/25 decisions; partial-trace: 25/25 decisions; certified-aa: 25/25 decisions. A/A outcome discrepancy is zero. Summed conflicts include censored queries and are not comparable successful-work costs. No clock, scaling or challenge-recovery claim.

*Execution:* one CPU, 2GiB address-space ceiling, 600second total cap per panel including its pilot/freeze. Each oracle uses 2000 cumulative SAT conflicts and a 750ms safety fuse. Shared local host; no L2 isolation. The user's explicit local EKS/S3 exception applies. All source, input, caps and analysis were frozen before primary rows.

*Identity:* source 3ae8b21241256517a9c867b4e78fb25e02ff502e; protocol 16186e61242f683843585a932962141c6bf0c33a49997be0955fc9cc0d360af3. Binary 15c4d1b193999153c6ddc6ec94959167c8f4d016be197b55b26d6993001b1126. Raw rows, native summary, pilot, exit status, accepted export, file checks and host receipt are linked by README.md. Canonical export/file checks pass; no independent signed review-run record exists. Known inverse/halving and tau norm algebra are prior work, not new theorems.
