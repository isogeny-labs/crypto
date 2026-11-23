# Exact scope and pins

`AnchorBudget.penalty` is the list sum with coefficients 1,2,... .
`AnchorBudget.Feasible g ms` says every head multiplicity is at least g
plus the weighted suffix penalty, recursively. No ECDLP or valuation
statement is hidden inside this definition.

- `penalty_ge_sum`: for EVERY natural-number list, its weighted penalty
  dominates its sum.
- `mass_bound`: for EVERY g and feasible list ms,
  2^(length ms)*g <= sum(ms)+g.
- `fibonacci_bound`: for EVERY g and feasible list ms of length n,
  F_(2n)*g <= sum(ms), AND F_(2n+1)*g <= penalty(ms)+g.
  This is the main sharp necessary budget inequality, with a simultaneous
  induction that proves the structural implication from every suffix
  constraint. It is not a finite arithmetic lookup certificate.
  The length is the number of required levels: a when requiring all packet
  levels, but u for the source's retained levels. Substituting a for u in
  a claimed obstruction to that actual recursion would be unjustified. [pcc:unverified]

The written rational anchor-valuation bridge, node preservation, polynomial
degree, optimal-schedule equality construction, Fibonacci asymptotics,
moment identity, chart integrality and all global algorithm statements are
NOT Lean-proved here. The native finite checks do not discharge those
unformalized general claims. [pcc:unverified]

Pinned Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Transitive pins are in
lake-manifest.json. Standard foundations only in the successful build;
no custom axioms, sorry or native evaluator are permitted as accepted proofs.

Build001 failed on an incorrectly resolved local package symlink. Build002
passed the first two declarations. Build003 failed on redundant tactics
after a congruence step had already solved its goal; its inferred sorryAx
diagnostic is FAILED evidence, not an accepted proof. Its source is retained
as attempt-003.lean.txt. Build004, actual gate, and fresh replay statuses
are reported in VERIFICATION.md after their executions finish.
 [pcc:unverified]