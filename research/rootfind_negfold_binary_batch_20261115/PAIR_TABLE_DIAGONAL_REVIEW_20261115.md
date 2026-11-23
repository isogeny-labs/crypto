# K13 pair-table diagonal peel: source review

2026-11-15 PST. Scope: candidate `rootfind-negfold-batch` source revision
under [Amendment 7](AMENDMENT-7.md). This is a source and correctness record;
no scientific timing or outcome row has been produced for this revision.

## Mechanism

`PairTable::build_negation_folded_binary_batch` enumerates abscissa groups
from the current row onward. Its first group has the same abscissa as the
left summand. `FastCurve::add_many_lazy` asks the SIMD routine to process the
whole row; an exceptional sum makes that routine decline, and the entire
row takes the scalar fallback. The new code completes the diagonal group
first, then presents the off-diagonal groups as adjacent signed pairs to
`FastCurve::add_many_lazy_soa_paired` on x86 when their layout permits. A
declined call uses `add_many_lazy_paired_x`; the portable route also uses
that fallback. The x-only SIMD output is converted to lazy points in the
original order. The table never reads their unfinished ordinates.

The route preserves the scalar pair enumeration, first-witness insertion,
and abstract addition count. Diagonal sums use full group addition because
they are exceptional; each off-diagonal batch counts the same number of
attempted sums. The existing kernel handles its own runtime CPU and field
checks and declines degenerate inputs without appending or counting.

## Checks and limits

- ARM release scalar-versus-batch table and m=2/m=3 decomposition parity:
  passed `RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/wt/codex-rootfind-f456-20261115/target cargo test --release --lib binary_batch_table_matches_scalar_table_and_witnesses`
  (one test passed; K13 and K17 fixtures, first witnesses, insert counts,
  abstract additions, and 200 targets per fixture at m=2 and m=3). The toolchain emitted
  a debug-info stripping warning but exited successfully.
- x86 macOS cross-target `ecbench` compile: passed
  `RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/wt/codex-rootfind-f456-20261115/target cargo check --target x86_64-apple-darwin --bin ecbench`
  with exit status 0. This type-checks the paired call and conversion but
  does not execute the kernel.
- AVX-512 execution and complete cold wall comparison: pending an admitted
  x86 study run. Neither source inspection nor cross-compilation establishes
  a wall advantage.

The [editable graph](pair-table-diagonal.dot), [rendered SVG](pair-table-diagonal.svg),
and [PDF](PAIR_TABLE_DIAGONAL_REVIEW_20261115.pdf) show the route and open
measurement boundary. The original six-arm spec and earlier receipts remain
bound to their own revisions. A new publication receipt is required before
this source is measured as the study candidate.

Internal provenance: `src/cryptanalysis/ic_boundary.rs` table builder;
`src/cryptanalysis/koblitz_fast.rs` paired and fallback arithmetic;
`research/rootfind_negfold_binary_batch_20261115/AMENDMENT-7.md`.
