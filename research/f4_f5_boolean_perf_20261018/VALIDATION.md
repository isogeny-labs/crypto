# Validation and execution status

Baseline: `1e2bf0715a2a927afc92e6730d6e65b59f15ab9e`. Source/binary digests
are retained in `records/`. The first measured driver remains frozen as
`records/driver-snapshot.rs`; the generalized solver comparison uses the final
versioned example. Its field and solver modules are native Rust.

## Solver and analyzer checks

`cargo test --offline --locked --release --manifest-path
tools/generalized-rootfinder/Cargo.toml` passes **10 independent tests**, plus
the binary and documentation test targets. The standalone Cargo build passes.
CLI fixtures return complete certificates: two Boolean roots and eight
separator tuples. Pocklington tampering, reducible moduli, inseparable/repeated
roots, literal rational poles, truncated output, search budgets and all original
separator coordinates have dedicated controls. GF(2^83) extraction uses wide
field elements; the prime certificate control proves the 127-bit Mersenne prime.

The native runtime admission/bootstrap analyzer passes three controls: matching
qualified samples, contention/missing samples, and rejection of changed outputs.
An all-missing local-runtime control reports inconclusive rather than a gain.

The final standalone sources also pass `cargo fmt --check` and
`cargo clippy --release --all-targets -- -D warnings`. The first Clippy run
reported seven style lints; the corrected run and all ten repeated solver
tests pass. Post-profile changes replace integer divisibility spelling and
annotate the indexed Gaussian loop; the exact profiled field/solver sources
remain in `records/profiled-source/` with their original hashes in
`records/final-source.sha256`. Final publication hashes are recorded separately.

## Repository library checks

The first `KIC_GF2_WRITE_TABLE_ONCE=1 cargo test --release --lib` ran in the
restricted sandbox and ended with signal 15 (exit 143) before a suite summary.
It recorded socket-permission failures and is retained as an interrupted run;
the source of that termination is unknown. All 38 `pollard_collab` tests passed
unchanged in a serial host rerun with loopback access.

The full host rerun used `KIC_GF2_WRITE_TABLE_ONCE=1 RAYON_NUM_THREADS=1
timeout 1200 cargo test --locked --offline --release --lib -- --test-threads=8`,
through the native `isolated_bench busy` lock. It completed with **4,396 passed,
one failed, 118 ignored**. The failure was the unchanged
`cryptanalysis::jv_cover::tests::nagao_finds_planted_six_sums_and_agrees_with_the_oracle`:
its second trial exhausted its internal time budget while the suite was
contended. A fresh serial host rerun of that exact test passed unchanged
(all six trials; **one passed**). Thus all 4,397 nonignored library tests are
covered by passing executions; this is not a claim that the parallel invocation
was wholly green. The initial failure and successful rerun are both preserved.

The shared GF(2) tests, including poisoned reused storage and partial final
tables, passed in that full run with the candidate enabled. The matrix-F5
row-space checks passed as well. Default-path focused checks passed all ten
shared-kernel tests. The touched example's Cargo test target also passed
(zero declared tests; compilation).

## Measurements and unavailable comparisons

The native analyzer revalidates each successful row's outputs/accounting,
complete root sets or decomposition verdicts, and merged-worker Callgrind
profiles. Five A/A samples remain controls; five alternating A/B pairs provide
the primary matrix comparison. Fresh seeds and the n17 scaling pair agree.

Both n19 matrix arms returned `None` under the existing matrix admission limits,
in both configurations; the driver recorded exit 101 on unwrap. These are
failed/capped comparisons, not measurements of a speedup. The generic F4-basis
n23 planted run also recorded exit 101 after its 50-second profiling budget.
All five failures are retained. No negative decision is inferred from them.

The local native isolation preflight refused the busy host. Every local elapsed
time in a Callgrind record is diagnostic only. The dedicated clean-CI workflow
retains all A/A and A/B attempts, requires exact outputs and empty residual user
threads on the reserved core, and applies an exact paired bootstrap in Rust.
Its qualification is a separate artifact, not presumed successful here.

Magma, msolve, FGb, Singular and PolyBoRi were not available locally. Their
matched external comparisons, native Arm64/GPU results, and the old hard-family
20-case all-roots/8x gate remain unresolved. No scientific gate is weakened.

The leaderboard component panel and its twins copy native prepared values.
IC/rho progress, curve identity data and tables A-C retain their existing values.
The report PDF and every graph are generated from the same recorded JSON, and
copies in `benchmarks/` use that same rendering source.
