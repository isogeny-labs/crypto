# Unified factor-base sweep: implementation contract

Requested behavior: one native launch command searches structured factor bases,
evaluates successful decomposition separately from rejection, preserves every
candidate and derivation, resumes interrupted work, and streams CADO-NFS-style
line logs with rates and provisional predictions.

The executable is `factor-base-sweep`. Its native workers reuse the registered
curve constructors, factor-base inventory/FB1 protocol and degree-bounded F4.
A versioned plan declares curves, constructor recipes, seed, train/holdout probes
and hard per-candidate resource budgets. The default public-fixture preset covers
small binary and prime fields; unsupported sizes/formulations fail explicitly.
No wide-curve support or mathematical regularity certificate is inferred.

Each candidate retains its exact ordered signed point inventory, FB1, recipe,
field/model/subgroup facts, order commitment and content hashes. Trial records
bind the exact polynomial system and solver parameters, target, verdict,
witness verification, observed productive solving degree and matrix resources.
A completed F4 solve and a refutation are different outcomes. Degree caps,
timeouts and truncated bases cannot certify regularity or infeasibility.

Random probes use matched seeded public known-answer group targets. The solver
receives the target point, not its scalar. A reported relation is checked by
actual point addition and its signed inventory columns. Training selection and
holdout validation remain distinct; refutations cannot count as successful
relations. Certified degree of regularity stays null unless accompanied by a
separate certificate, which this implementation does not claim to produce.

Progress includes candidate/stage/trial IDs, F4 degree and matrix events,
heartbeat elapsed time, verified relation rate, rejection/exhaustion rates,
and provisional remaining-work estimates. Comparisons require matching curve,
targets, solver and limits; raw wall rates are operational telemetry, not an
accepted speedup. Solver-stage observations never establish an end-to-end gain.
Incomplete native pricing keeps a total-cost winner unresolved. The interface
reports the best observed successful solving degree as an algebraic diagnostic,
with coverage, base size and cost alongside, not as a speed verdict.

Run manifests and completed artifacts are immutable. Resumption verifies their
hashes and configuration/binary identity, skips verified completed work, and
retains interrupted/failed attempts. Human lines and JSON events are flushed
incrementally; an active solver emits heartbeats. No earlier evidence is edited.

Acceptance validation: deterministic inventories and identities; independent
point-witness checks; positive, negative, capped and timeout outcomes kept
separate; observer parity with the unchanged solver; actual CLI interruption,
resume, corruption and configuration-mismatch cases; native full release library
and affected binaries/integration tests. Functional fixtures are not a research
campaign or a speed comparison. The historical #1898/#1975 artifacts and #1963
card remain unchanged. No S3 operation is part of implementation.
