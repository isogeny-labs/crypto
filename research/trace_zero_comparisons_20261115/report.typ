#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 8.5pt)
= 1. Certified domain and partial traces
2026-11-15 · crypto PR3744 · TZP-20261109-finite-domain-partial-trace · unsigned local evidence

#image("../trace_zero_experiments_20261109/reports/run-20261115T0450Z/outcomes.svg", width: 100%)

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

#pagebreak()
= 2. Matched relation encodings
2026-11-15 · crypto PR3744 · TZC-20261115-encoding · unsigned local evidence

#image("reports/run-20261115T0458Z-encoding/outcomes.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 3pt,
 [Arm], [Found], [Refuted], [UNKNOWN], [SAT conflicts],
 [certified], [36], [14], [16], [6847],
[constants], [36], [14], [16], [21365],
[implicit], [36], [12], [18], [20958],
[full-point], [28], [6], [32], [14439],
[constants-aa], [36], [14], [16], [21501],
)

Certified and constants each decide50/66; implicit48/66; full-point34/66. All five arms have zero invalid decisions. The constants A/A arms have identical outcomes and thirteen conflict discrepancies. Allocation shrinks without a completion gain on this panel.

At n131, certified/constants/implicit each give4Found,4Refuted,8UNKNOWN out of16; full-point gives16UNKNOWN. Maximum allocated variables at n131 are104669 /4217 /6572 /106110 respectively. The d4 base has only two signed points. All n131 d5 cases, with eight base points, remain UNKNOWN. Thus the successful wide relations here are singleton-x controls.

The implicit arm receives a linear basis, not an enumerated point table. Full-point chains retain complete generic/double/opposite/identity branches.172 saved original point tuples replay in the post-run integrity audit. Native total group additions+doublings are374 /390 /410 /168 for the four distinct methods; failed/capped queries remain included.

*Flow:* planned 330, attempted 330, completed 330, not-completed 0, excluded 0, interrupted 0. Every UNKNOWN remains in the denominator. Zero retries and zero paid resources. 33 common known-answer instances: n7/9/83/131,d4/5, planted/cancellation/two full-width uniform-scalar targets, plus the n5 signed-base UNSAT control.

*Controls and scope:* the preregistered small-field positive pilot gate passes; two paired repeats are the same targets, not independent population samples. Per 33-query repeat counts: certified: 25/25 decisions; constants: 25/25 decisions; implicit: 24/24 decisions; full-point: 17/17 decisions; constants-aa: 25/25 decisions. A/A outcome discrepancy is zero. Summed conflicts include censored queries and are not comparable successful-work costs. No clock, scaling or challenge-recovery claim.

*Execution:* one CPU, 2GiB address-space ceiling, 600second total cap per panel including its pilot/freeze. Each oracle uses 2000 cumulative SAT conflicts and a 750ms safety fuse. Shared local host; no L2 isolation. The user's explicit local EKS/S3 exception applies. All source, input, caps and analysis were frozen before primary rows.

*Identity:* source 64296a66a8f69fe2c6d79c5e7969dc5d09b328da; protocol 02c280306ed527e8a741fafb506cb031501be867552202f4c4365173bd45805a. Binary 7689cd7efd4451b197dcffbfdfab3773e613280e1b5e196b93152c36d6f23c43. Raw rows, native summary, pilot, exit status, accepted export, file checks and host receipt are linked by README.md. Canonical export/file checks pass; no independent signed review-run record exists. Known inverse/halving and tau norm algebra are prior work, not new theorems.

#pagebreak()
= 3. Reversible tau presentations
2026-11-15 · crypto PR3744 · TZC-20261115-tau · unsigned local evidence

#image("reports/run-20261115T0502Z-tau/outcomes.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 3pt,
 [Arm], [Found], [Refuted], [UNKNOWN], [SAT conflicts],
 [direct], [36], [14], [16], [21475],
[tau-image], [34], [14], [18], [11936],
[tau-preimage], [36], [14], [16], [17388],
[direct-aa], [36], [14], [16], [21605],
)

Direct and tau-preimage each decide50/66; tau-image48/66. The A/A arms agree on outcomes with fourteen conflict discrepancies. All successful mapped answers are transported back;142 saved original point tuples replay in the post-run audit. No completion advantage is observed for these transported finite-domain presentations.

At n131 all three distinct presentations give4Found,4Refuted,8UNKNOWN. Direct maximum allocation is4217 variables; both transported presentations reach61728. Across all66 rows, native group additions+doublings are390 direct,4888 image and31238 preimage, including setup and replay. These counts exclude field/ANF construction and SAT work and are not runtime ratios.

Exploratory post-run profile: for the eight-point n131 d5 base,126 x-coordinate bits are common constants directly, versus17 after either transport. This is a structural observation, not a preregistered causal conclusion. No unenumerated lifted-delta map or assertion that the transported x set is linear is supplied.

*Flow:* planned 264, attempted 264, completed 264, not-completed 0, excluded 0, interrupted 0. Every UNKNOWN remains in the denominator. Zero retries and zero paid resources. 33 common known-answer instances: n7/9/83/131,d4/5, planted/cancellation/two full-width uniform-scalar targets, plus the n5 signed-base UNSAT control.

*Controls and scope:* the preregistered small-field positive pilot gate passes; two paired repeats are the same targets, not independent population samples. Per 33-query repeat counts: direct: 25/25 decisions; tau-image: 24/24 decisions; tau-preimage: 25/25 decisions; direct-aa: 25/25 decisions. A/A outcome discrepancy is zero. Summed conflicts include censored queries and are not comparable successful-work costs. No clock, scaling or challenge-recovery claim.

*Execution:* one CPU, 2GiB address-space ceiling, 600second total cap per panel including its pilot/freeze. Each oracle uses 2000 cumulative SAT conflicts and a 750ms safety fuse. Shared local host; no L2 isolation. The user's explicit local EKS/S3 exception applies. All source, input, caps and analysis were frozen before primary rows.

*Identity:* source 3c54f99045404f2b056085cc38a23b79ea691d40; protocol 3462390eea9a1434d5616810e40a965dbf50f3261c41352ef17c715aaeb8c29d. Binary 7689cd7efd4451b197dcffbfdfab3773e613280e1b5e196b93152c36d6f23c43. Raw rows, native summary, pilot, exit status, accepted export, file checks and host receipt are linked by README.md. Canonical export/file checks pass; no independent signed review-run record exists. Known inverse/halving and tau norm algebra are prior work, not new theorems.
