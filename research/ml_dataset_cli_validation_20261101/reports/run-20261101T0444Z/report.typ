#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Dataset CLI: retained 46-case attempt
*Recorded outcome: all 46 cases have terminal receipts; final causal-data acceptance failed.*

This is a local software validation record, not a production corpus or a timing comparison. The fixed attempt started on 2026-11-01 at 04:44:40.144974Z and stopped at 04:47:05.408755Z. Derived elapsed time: 145.263781 seconds, from the two driver timestamps. CPU time is not aggregated here. Source: clean e0a8650be05962af95d1ca82082443b68ed784f5. Frozen protocol commit: 4d27bc2ce4b1c92ac0b398b49a0d6427b530aaf9.

#image("outcomes.svg", width: 100%)
Figure: counts read from five central verifier outputs for fixture ml-dataset-cli46-e0a865-final008-20261101. Data and exact command labels are in data.json. The 92 planned producer/replay observations concern the same 46 cases.

== Checks and failure
All five verifiers report artifact hashes checked and known-answer replay performed. Native Koblitz: 4 verified. Legacy: 6 verified. Four factor-base policies: 20 verified, 4 unsupported. Prime domain panel: 5 verified, 3 partial. Prime causal panel: 3 verified, 1 budget exhausted. These are recorded solver outcomes, not a passed final acceptance gate.

Wrong-recipe verify and restore controls both exited 1 with “factor-base recipe or control seed differs from frozen config.” Applicable publication/restore commands exercised a local file backend; S3 custody of this evidence is a separate archive operation.

The final gate required nonempty prime prefix and algebraic channels. Both had zero public rows; static features had 4 rows and restricted prefix records had 8. Each root input had degrees 4,4,2,2,2. The returned basis was exactly the three quadratic input constraints, with zero elimination steps and two pairs above the frozen degree bound of 3. The independent ideal-equality check withheld admission. The frozen failure was retained without retry or resampling.

== Evidence and next implementation
Exact seeds, parameters, limits and control definitions remain in protocol.json and the panel plans/recipes in the sealed fixture. The driver execution command and individual argv arrays remain in command receipts. No limits were increased. Before this attempt, the unfiltered library suite passed: 6003 passed, 0 failed, 125 ignored; the real release CLI build and embedded-source preflight also passed.

The source diagnosis is in cli46-causal-source-diagnosis.md and channel-diagnosis/DIAGNOSIS.md. The next repair must preserve original constraints in the same generating set reported to certificate checking and specialized recursively. It must retain the raw bounded-solver report and existing work limits.

A separate failure-summary command-label collision was discovered after termination. Any corrected reconciliation is derived evidence and does not replace the original failure export or change acceptance. The original USD 200 production ceiling and expired deadline remain unchanged. No cloud compute was launched for this reconciliation.
