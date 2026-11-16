# Prospective independent component review, 2026-11-03

Frozen before candidate/proof access. Reviewer first_information proposed the recurrence direction and previously inspected the original sieve, but did not author candidate implementation or proof. This is a non-author review with disclosed advisory context, not blind derivation.

## Admission and bounds
Wait for root confirmation of exact published producer commit. Root approved ONE native wrapper120seconds total,2GiB address-space ceiling,one compilation job,offline; ONE Lean wrapper300seconds total,8GiB address-space,Lean -M3500 -j1 -s16384,OS10MiB stack. Include compilation/replay in those bounds; preserve failures, no automatic retries. No collector, scientific trajectory, cloud, container or runtime qualification invocation. Exact source/input/tool hashes must precede replay and be unchanged afterward.

## Source checks
Compare candidate against original point_sieve_candidates and exact extracted native function bodies, real meter and tracked_set; identify altered derives and Divisor wrapper explicitly. Inspect ordinary and exceptional branches, update-before-continue, active coefficient lengths versus padded coefficients, canonical input bounds, low-degree behavior and final unused update. Derive arithmetic and collection delta independently from actual source. Full SieveResult equality includes every scalar and ordered bucket vector.

## Reviewer controls fixed before source access
1. Branches where u(x)=0: nonzero slope, zero slope with q0 nonzero, and all-lambda q0 zero; construct algebraically valid complement fixtures when possible and otherwise document omitted unsupported fixture rather than altering baseline.
2. Continue correctness: points with u(x) nonzero but no rational y must still advance recurrence; use same exact ordered x scan.
3. Canonical wrap at small odd primes and at the stated upper native modulus bound; distinguish ring recurrence from canonical UInt64 refinement.
4. Active padding: leading zero coefficients and shorter active coefficient lengths must not introduce phantom operations or silently change baseline assertions.
5. Counter negative control: deliberately incorrect expected base arithmetic delta must fail; reject a changed ordered bucket/candidate result even when set cardinality agrees.
6. Initial/final x and p=2 arithmetic boundary only where supported; never claim characteristic-two full curve coverage from helper controls.

## Lean fidelity
Check ring recurrence and canonical-word add refinement at their actual statement scopes. Explicitly retain missing composition if no whole iterative UInt64 refinement is proved. Replay exact declarations/controls and named axiom inventory. Add simple reviewer-owned algebraic boundary controls only when they fit the same frozen single invocation; no new theorem scope inferred from successful compilation.

## Evidence
Preserve publication receipt, exact input manifest, commands, raw stdout/stderr, return codes, failures and final source readbacks. Report PDF/SVG/CSV/JSON at component scope and archive through repository artifact rules; no performance or whole-crate claim. No staging without root exclusive grant.
