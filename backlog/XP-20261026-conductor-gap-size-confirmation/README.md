# Broader conductor gaps and a precise crater comparison

Status: **blocked, not running**. A native prospective allocation planner and
auditor now exist, but the measurement executor and independent measurement
verifier remain unimplemented. No paid resource, lab lease or reward is created.

The [2026-10-27 preflight](../../research/conductor_gap_preflight_20261027/PROTOCOL.md)
and `examples/conductor_gap_confirmation.rs` provide `plan`, `audit` and
`preflight` commands. They check 128 classes, 4,096 distinct class/logarithm
pairs and 32,768 prospective primary invocations with separate seeds and a
fixed block permutation. There is no solver-execution command; `preflight`
reports the remaining gates and exits 3. `card.json`'s `run.argv` is still the
intended future measurement command, which the current example rejects.
Allocation validity does not make this card ready or freeze confirmation.

The [design study](../../research/conductor_gap_size_ladder_20261026/README.md)
prepares parameter classes at four native-size rungs and upper structural
sizes, and retains the failed D=-19 restricted-family search. Prime
parameters do not supply exact endpoint models or target transport.

The [protocol](../../research/conductor_gap_size_ladder_20261026/PROTOCOL.md)
sets the proposed objective: all four rung intervals inside [0.95,1.05],
with nominal 99% simultaneous coverage. This is a fixed-cohort generic-rho
cost comparison. It cannot establish equivalence for every class, an
endomorphism-sensitive algebraic method, or production-size security.

Before ready promotion, certify all selected models, orders, conductor
strata, map/dual relations and transported G/Q; implement the native driver,
checkpoint/resume and independent full replay; and freeze an executable
pilot protocol. The full disjoint pilot is 32,768 independent one-target
solves plus separate replays. The proposed confirmation is 1,024,000 solves
plus replays/controls, subject to the pilot variance, coverage and feasibility
checks. Neither count is a guaranteed minimum. A resource estimate cannot
be inferred from parameter-search time or the generic operation model.

The ninety-minute local guard is retained as a stop limit, not a promise
that this workload fits. A guard stop leaves the pilot partial. Freeze any
changed envelope and count before execution; do not reduce a rung silently
or sample until equivalence passes. Independent classes, fresh targets and
solver seeds are distinct replication units. Report a difference, a null
result or an inconclusive result according to the frozen rule.
