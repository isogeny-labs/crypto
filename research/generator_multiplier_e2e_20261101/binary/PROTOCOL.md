# Random-binary multiplier-base end-to-end protocol

This prospective panel tests the strongest earlier generic result on a
registered random binary curve not used in the multiplier training scan:
eight unknown seed logs with the signed points `[m]P`,
`1 <= m <= 32`, sharing each seed's matrix column. Twelve held-out targets and
six measured rounds give 72 paired cells per arm.

`coordinate-subspace` is the coordinate-defined factor-base control required
for the table oracle. `plain-best` is the best plain-seed configuration in the
merged training study. `plain-same-seeds` removes only the multiplier closure.
`multiplier-aa` measures timing and position noise, while `rho-neg` anchors the
result against a generic ECDLP algorithm. All arms use the same native binary,
target stream, three-summand signed MITM oracle, trial cap and verification.

The configuration was fixed by interpolation between the merged degree-26 and
degree-28 training targets; this degree-27 curve and the targets in this file
were unseen by that scan. The primary metrics are `S` and total GAE.
The complete-cold outcome is `method_call_wall_ratio`, with
`process_wall_ratio` reported as the fork-to-reap practicality check.
Relation yield, relation-phase cost, rank work, memory and the complete pricing
band diagnose why a ratio moved but cannot replace it.

The candidate is a significant end-to-end improvement only if its 95%
counted-cost intervals against both `coordinate-subspace` and `plain-best`
lie below one with point ratios at most 0.50, and its L2 method-call interval
against `coordinate-subspace` lies below one with point ratio at most 0.70.
The candidate A/A method-call interval must contain one. A success establishes
the specialised two-sum signed-graph batching constant factor on this toy
random-binary pipeline. It does not establish a new ECDLP exponent, an
algebraic Weil-descent improvement, or a higher-degree extension-field
impossibility result.
