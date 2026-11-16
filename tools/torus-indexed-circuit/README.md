# Redundant indexed membership/target compiler

`Augmented::member` and `Augmented::target` append unknown t bits and public
constant-product/range/borrow constraints to the unchanged original system.
Every original variable/equation is retained as a literal prefix. Public
role metadata is bound to the pinned original member layout.

The new crate is a compiler and a witness-extension qualification harness.
It invokes no discovery solver. `extension_candidate` takes known labels
only to check a supplied original witness; that API is not a query oracle.
`project_and_check` verifies the new assignment and its complete old prefix.

Read the study's indexed-circuit-v1/PROTOCOL.md, QUALIFICATION_SCOPE.md and
FROZEN.md first. Prime-field validity remains external; a/q have32bit caps.
The source-qualified controls include the ORIGINAL actual six-member system,
not a replacement tiny actual-field base. Native PCC-20261116-b3227f is
unsigned/unverified. Future edits require a new qualifier/version.

Development checks:

```sh
cargo test --release --manifest-path tools/torus-indexed-circuit/Cargo.toml
cargo fmt --manifest-path tools/torus-indexed-circuit/Cargo.toml -- --check
cargo clippy --release --all-targets --manifest-path tools/torus-indexed-circuit/Cargo.toml -- -D warnings
```

To reproduce full artifacts, use the qualified executable/NEW output
directory procedure in indexed-circuit-v1/REPRODUCE.md. The native executable
writes its source/binary hashes before controls, complete system/witness,
all comparison trace bytes and refusal/mutation results. Counts are not
complete cost or performance data. See NEXT.md before selecting a solver.
