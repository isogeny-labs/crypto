# Uniform-yield implementation addendum: compile-only

2026-10-26. Extends the frozen UNIFORM_YIELD_20261025_PROTOCOL.md without
changing its population, sampler, criterion, controls or resource caps. This
addendum is prepared before ANY scientific execution. The S3 checkpoint upload
question remains unanswered and the documented token remains absent. No gate,
upload, experiment, verifier, corruption path or new proof has been executed.

## Versioned producer and independent audit

- Preserve native/uniform-yield.cpp byte-for-byte, including its October25
  preflight INPUTS.sha256. New native/uniform-yield-v2.cpp adds the protocol's
  explicit source/twist nondominated policy sets over expected initial rank
  gain, mean degree and expected collection calls, for original/reduced modes.
  It binds its own new source path in the result index. No new study input or
  numerical result is emitted during compilation.
- New native/uniform-yield-audit/ inherits the previously independent Rust
  field/group/polynomial and semantic witness routines, never C++ arithmetic.
  The state path uses a breadth-first reachable-space closure followed by a
  descending-rank rational solution, separate from the producer's recursive
  GMP implementation. Rust exact fractions normalize arbitrary-precision
  BigInt numerators/denominators by its own Euclidean gcd.
- Audit code is intended to rebuild all actual point/sign/root/query/solver
  certificates, base-log solves, source recoveries, states/transitions/rational
  values, 210 policy pairs and all source/twist frontiers. It checks exact file
  membership and rehashed manifest bytes, writing a fresh receipt only after
  all checks. This behavior is NOT established by a successful build.
- Corruption code covers row, degree, witness, arm/target, transition, rational,
  query/log/scalar, source and frontier alterations, rehashing affected pins.
  A missing-failed-query control returns an explicit null if none exists.
  None of these controls has been run; do not report rejection counts.

## Actual validation and remaining launch conditions

The producer and ordinary auditor build pass. `cargo test --release --locked
--offline --no-run` also compiles the auditor/backend test harness; it executes
NO tests. These are compiler checks only. Existing unchanged root-library and
metadata checks are ordinary implementation validation and cannot be described
as study executions or independent mathematical verification.

The Lean recurrence/probabilistic interpretation and exact study runner remain
unimplemented. Expected-call recurrence code is not a universal expectation
proof. The queued explicit F13 dual remains unexecuted. Native commands remain
capped600seconds, iteration3600seconds, two existing build workers; no paid
resources or upload/credential reuse. Old results/proofs/17identities and all
registry/catalog/benchmark inputs remain frozen.

After explicit checkpoint-only authorization and token configuration, refresh
the resume checkpoint with the unmodified required gate, finish the faithful
proof/runner, freeze compiled sources and run the whole preregistered contract.
Before any interpretation, independently replay all payloads/controls and
retain all policies, strongest-source comparisons, failed shifts and losses.
No uniform yield, degree, speed, regularity or hardness result is claimed now.
