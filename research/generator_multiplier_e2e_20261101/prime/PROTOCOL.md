# Prime CM multiplier-base end-to-end protocol

This prospective panel asks whether the previously observed relation-count
advantage survives a complete cold `ecbench` method call.  The candidate is
`cm-multiplier:size=2,norm=16`: two unknown seed logs, with known signed
endomorphism multiples sharing each seed's matrix column.  The fixed curve is
the registered j=0 subgroup used by the accounting tests; twelve held-out
targets and six measured rounds give 72 paired cells per arm.

The controls separate the proposed mechanism.  `coordinate-glv` is the
conventional automorphism-folded coordinate base. `plain-equal-points` has
approximately the candidate's table-vertex scale but one unknown per seed.
`plain-same-seeds` keeps the candidate's two seed columns and removes their
multiplier closure. `multiplier-aa` repeats the candidate to measure position
and timing noise. `rho-neg` is the generic ECDLP reference, not an expected
victim of the claimed constant-factor improvement. Every target contains a
planted scalar verified independently by the parent process.

Parameters were selected from the merged 2026-10-23 training study and its
scaling rule, before these target seeds were generated. No held-out row may be
used to change a base size, norm, target count, metric or threshold. The
primary counted result is the candidate/control ratio in `S` and total GAE.
The requested end-to-end result is the L2-gated
`method_call_wall_ratio`; `process_wall_ratio` is the broader fork-to-reap
practicality check. Relation trials, relation-phase GAE, matrix rank/rows,
memory and the pricing band are diagnostics, not substitute speedups.

The win criterion is deliberately two-part: the candidate must have a 95%
interval below one and a point ratio at most 0.75 against both
`coordinate-glv` and `plain-equal-points` in counted cost, and its complete
cold method-call ratio against `coordinate-glv` must have a 95% upper bound
below one with point ratio at most 0.80. The candidate A/A method-call interval
must contain one. Failure licenses only the statement that the effect did not
survive this panel. Even success is a constant-factor result for this native
two-sum pipeline at a toy size; it is not an exponent change and does not
disprove higher-degree extension-field decomposition algorithms.
