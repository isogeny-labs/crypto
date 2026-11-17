= 3. Reversible tau presentations
2026-11-15 · crypto PR3744 · TZC-20261115-tau · unsigned local evidence

#image("outcomes.svg", width: 100%)

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
