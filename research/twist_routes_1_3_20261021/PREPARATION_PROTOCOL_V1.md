# Execution fixture preparation, version 1

This preparation supplements the 18-experiment design. It does not relax the
acceptance criteria or turn a diagnostic into a pipeline result. Commit this
protocol and its native source before running `prepare`.

Inputs: the registered structural fixtures and even/odd-composite controls
already named in EXPERIMENTS.md, plus eight generated A=-3 ordinary prime-order
curves, two at each field bit length 8, 10, 12 and 14. Each ladder search uses
SplitMix64 initialized with 20261021 + 100*bits + replicate. Pick p in the
specified bit interval and b in 1..p, use the next odd prime, and reject singular,
nonprime-order, supersingular and duplicate candidates. Preserve every rejected
candidate. Limit each search to 10000 candidates; exhaustion is a failed setup.

Recompute all small-field group orders by exact enumeration. The first sorted
nonidentity point is the generator on prime-order fixtures; composite/even cases
are structural controls only. Independently count the standard twist and require
the two orders to sum to 2(p+1). Freeze the least nonsquare d. Register the exact
prime, quadratic and applicable cubic/sextic extension models, and preserve the
tower/flat basis conversion generator. Verify the identity implementation against
the existing registered structural models before writing additions. No wall-time,
performance, cross-repository EC1, or group-isomorphism claim follows from setup.

Commands from repository root (Rust 1.90.0):

```sh
cargo build --release --locked --manifest-path research/twist_routes_1_3_20261021/runner/Cargo.toml
research/twist_routes_1_3_20261021/runner/target/release/twist-route-experiments prepare NEW_DIRECTORY
```

Preserve source/protocol/lockfile hashes, source commit, compiler and command
outputs. The emitted manifest and registry additions must be committed before
curve/solver execution. Setup is correctness-only L0; subsequent source changes
and run protocols are versioned amendments. All original route obligations remain
open until their own checks and comparisons complete.
