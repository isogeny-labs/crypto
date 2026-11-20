# Current-base development gate: all scoped checks pass

Source974f2e9cd46b000021cb20e6a78c3f54063f8882 is based on published feature
562f7075b4a6ad0a2c5de918c5d7e4c0a0d51d24 and its upstream SAT fix. Exact
example/proof hashes are recorded. No scientific input or binary changed.

All terminal exits are0:12Rust example tests (11query/core/reference and one
inverse-missing negative control), default Clippy, release certificate build,
the Lean gate (five query dependencies and one residual identity,0trusted
additions), and the separate native certificate checker. Clippy emits one
unchanged library warning; this is not a strict global lint result. The
certificate checker accepts IDC1h08a8b0da9bbd7a55 using32issuer points and
256additional points. The general finite-group counting bridge remains open.

The corrected invocation has no artificial virtual-memory limit, uses Cargo
one build job/one test thread pinned to CPU4, and Lean-j1. Raw first-attempt
allocation/thread failures are preserved in the sibling receipt. These are
correctness/static checks and carry no admitted timing measurement. The
required full library command has its own separately bounded receipt; scoped
success is not a full library or hosted-CI pass.
