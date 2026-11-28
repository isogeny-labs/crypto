# Batched standard-subspace factor-base lifts

The opt-in `build_standard_subspace_factor_base_batch_lifts` candidate at
amended source commit `0c23166d62` inverts the ordered packed abscissae with the
existing zero-tolerant `Gf2::batch_inv`, then calls the same packed lift
formula and point lowering as the legacy builder. The baseline entry point
still selects individual inversions, so ecbench's current factor-base work
record remains attached to its original implementation. The frozen
[correctness protocol](PROTOCOL.md) is at commit `3febee7878`. Its
[Amendment 1](AMENDMENT-1.md) at commit `0c23166d62` replaces an unavailable
K17/d6 fixture with the existing K19/d7 fixture after the first test's
constructor failure.
[pcc:unverified]

The source-derived inversion-call counts for the five fixed bases are in
[inversion-counts.csv](inversion-counts.csv) and its [plot](inversion-counts.svg).
They count calls, not field multiplications, memory, or wall time. Batch
inversion adds products to replace repeated inversions; a complete cost
comparison must price both. The candidate also holds packed inputs, inverse
outputs and batch scratch in memory, each proportional to the subspace size;
the source permits dimensions through 20. [pcc:unverified]

The [diagram](flow.svg) shows the shared ordered lift and the remaining
accounting gate. Its editable source is [flow.dot](flow.dot); the plot source
is [render.py](render.py). A one-page [PDF](report.pdf) embeds both visuals.
The canonical IC leaderboard, progress timeline, and rootfinding paper were
checked for scope; this candidate changes no verified performance edge, so
their source and renderings stay as they were. [pcc:unverified]

## Checks

The amended focused release command was
`RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc /Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/cargo test --release --locked -j 2 --lib batched_ -- --nocapture`.
Its [stdout](test.stdout), [stderr](test.stderr), and [exit](test.exit)
record 28 passed, zero failed, including both new tests. The new tests compare
the candidate with the individual packed lift with the cache off and on, and
compare all public factor-base fields and ordered points on K5/d2, K7/d3,
K13/d5, K19/d7 and K23/d8. `rustfmt --edition 2021 --check` exited zero.
These are local producer checks pinned in claim `PCC-20261116-7748c7`, whose
standing is unverified; no independent verification is asserted. [pcc:unverified]

The first full release library-suite attempt used Rust 1.99.1 with four test
threads and a 600-second watchdog. It exited 124 after 3,374 output lines,
before the 6,569-test suite completed. A second attempt with a 1,800-second
watchdog also exited 124, after 3,939 output lines. Both runs passed the two
new batch tests and reported the same failure in the existing
`chained_s3_strategy_feeds_verified_relation_rank_and_target_solve` test.
The second run reached all 185 `koblitz_index_calculus::tests` entries: 184
reported `ok` or `ignored`, and that one reported `FAILED`. Its
[stdout](full2.stdout), [stderr](full2.stderr), and [exit](full2.exit) are
preserved. Neither watchdog run is a completed release suite. [pcc:unverified]

The failing test repeats in an isolated candidate-binary run
([stdout](isolate.stdout), [stderr](isolate.stderr)): its assertion expected
`capped.trials == 0` but found 16. A pre-candidate executable from the
existing `codex-rootfind-k13-admission-20261115` worktree fails the same
assertion in an isolated [baseline run](baseline.stdout). Its binary SHA-256,
worktree head and matching parent-source hash are in [CHECKS.json](CHECKS.json).
The executable's original build commit is not independently pinned, so this
supports a pre-existing failure classification without serving as an exact
parent-commit rebuild. The candidate change does not edit this test, its
default limits or its explicit Frobenius-base construction. [pcc:unverified]

The first release test is
retained as `test-first.*`: it exited 101 after 26 other matched-filter tests
passed, because both added tests called `KoblitzCurve::new(0, 17).unwrap()`
and that constructor returned `None`. The corrected K19 fixture has a fresh
source commit. The earlier long compile from that first source was interrupted
before an exit receipt; `test-interrupted.*` preserves its partial output and
is not a test result. [pcc:unverified]

Two earlier failed toolchain invocations
are retained as `test-rust193.*` and `test-rustup-arg.*`; neither compiled the
candidate. The first used Homebrew Rust 1.93.1, below the lockfile's minimum.
The second asked Homebrew Cargo for `+1.99.1`, which that binary does not
support. The successful attempt uses the installed Rust 1.99.1 Cargo and rustc
paths explicitly. [pcc:unverified]

## Scope and next gate

The ordered-output tests establish only the fixed correctness scope. The
release-library gate remains incomplete. Production integration needs a new
source and benchmark accounting
change under the reserved ecbench lane, followed by a source-frozen matched
L2 panel with A/A and independent replay. The K13 x86 admission probe in
the current PR remains the immediate comparison gate when its cost and
checkpoint prerequisites open. The broader performance obstruction remains
tracked in [root-finding complete-cost calibration](../../problems/rootfinding-complete-cost-calibration.md).
[pcc:unverified]
