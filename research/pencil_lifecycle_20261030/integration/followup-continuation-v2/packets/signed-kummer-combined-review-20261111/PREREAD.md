# Signed Kummer combined runner source-review preread

Recorded 2026-11-11T00:15:00Z after the initial file inventory and before the
final release-suitability judgment. This is not a blind preregistration: the
reviewer already knew that the packet declared 49 staged inputs and retained a
null generated lock. The attacks below were fixed before the final detailed
inspection.

## Producer claim under review

The source packet intends to stage an exact offline Rust workspace, compile
three test executables from one generated lock, and run a bounded qualification
consisting of:

- 28 fixed signed-producer cases and seven fixed Labels cases;
- 11 actual p739 driver calls (original, disabled, observed, and eight injected
  callback faults);
- forced-fallback and zero-vector probes; and
- four adapter protocol negatives.

The packet does not claim that any of those calls have yet run. Admission is
false and the generated lock, publication revision, deadline, and release
receipts are null.

## Prior

The predecessor driver and lock reviews justify the intended algorithms and
panel shapes, but a newly assembled source packet is more likely to fail at a
staging, package-selection, or result-binding joint than at the already
reviewed arithmetic. Prior probability of at least one release-blocking
integration defect: moderate.

## Load-bearing joints and attacks

1. **Closed staging.** Every Rust module named by a staged crate must be in the
   49-file copy set or be a generated, hash-bound file. Attack: follow every
   `mod`/`#[path]` edge from each selected target and look for a file that is
   merely mentioned in a review binding but never copied.
2. **Cargo target identity.** Each of the three commands must select exactly
   one intended executable from a workspace that Cargo can actually resolve.
   Attack: compare `--manifest-path`, `-p`, target kind/name, nested
   `[workspace]` declarations, path dependencies, and the executable filter.
3. **Outcome/source binding.** A passing row must bind the code revision and
   staged source bytes that produced it, not reuse a semantic success boolean
   as a provenance assertion. Attack: mutate both result rows consistently
   while leaving trace identity inconsistent with the published revision or
   staged source hash.
4. **Panel completeness.** Acceptance must retain all 28+7+11+2+4 planned
   checks and reject missing, duplicate, or reordered rows. Attack: delete one
   panel row, one fault trace, or one negative test and inspect whether the
   wrapper can still pass.
5. **Shared resource bounds.** Materialization and all three compiles must
   share the 1,800-second build allowance; all executable calls must share the
   600-second execution allowance. Attack: look for a phase-local timeout that
   resets the overall 2,400-second bound or a subprocess that escapes the
   captured process group.
6. **Custody and cleanup.** Private source, staged driver source, executables,
   donor inputs, owner permit, and final cgroup must be checked both before and
   after use. Attack: replace a file after selection or leave a child in the
   cgroup after the launcher records terminal state.

## Review boundaries

No compiler, Cargo metadata, tests, lock generation, solver, experiment, or
cloud action is authorized by this review. Static package-resolution concerns
therefore remain concerns until corrected by construction or checked later by
the separately authorized release workflow.
