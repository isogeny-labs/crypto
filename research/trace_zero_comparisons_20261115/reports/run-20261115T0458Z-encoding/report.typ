#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 8.5pt)
= 2. Matched relation encodings
2026-11-15 · crypto PR3744 · TZC-20261115-encoding · unsigned local evidence

#image("outcomes.svg", width: 100%)

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
