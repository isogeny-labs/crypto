# Tet offensive: sparse linear algebra (slug `crypto-sparse-la`)

Scope: `src/cryptanalysis/koblitz_sparse_la.rs`, and the symmetric-difference
merge in `src/cryptanalysis/pc_degree_harness.rs` (kernel
`gf2_la/sparse_rank_refute_*`). WALL-only: every fingerprint below is equal
base vs candidate.

Base commit: `aaf15e6bdc`. Host: shared 4-vCPU VM, Callgrind Ir (1 thread) is
the primary evidence; wall is "local, unisolated, [pcc:unverified]", load
average 1.2-1.6. Binaries: base perfbench sha256
`8434743a55acb2dcc0032cba6ae130919210e11d015d2d7f96f886943d7254c5`, final
candidate `af423b8c85cb82720196175a74c0bf5ab484f363ea7dae891175866eca2e5b64`.

## Commits and measurements (Ir, base -> final candidate, fingerprints equal)

| kernel | base Ir | cand Ir | note |
|---|---|---|---|
| relation/block_wiedemann_r45_n768_w13 | 490.63M | 469.89M (-4.2%) | fp 7135e476... equal |
| relation/sparse_solve_r45_c4800_w3 | 566.49M | 506.35M (-10.6%) | fp 529569b5... equal |
| gf2_la/sparse_high_macaulay_k5m3d5 | 232.03M | 232.03M | unchanged, fp equal |
| gf2_la/sparse_finish_macaulay_k7m3d5 | 156.45M | 156.44M | unchanged, fp equal |
| gf2_la/sparse_rank_refute_macaulay_k5m3d5 | 393.49M | 461.86M (+17%) | wall 1.28x faster, fp equal |
| block_lanczos_solve (scratch example, 512x512, 9/row, 3 seeds, mod 2^45 prime) | 1001.99M | 646.18M (-35.5%, 1.55x) | result hash 9ac1bee82bf471bf equal |

1. `koblitz_sparse_la: unchecked CSR gathers, division-free transpose product`
   - `LazyLanes::add_sparse` is `unsafe`, unchecked gathers; invariants asserted once
     (`from_rows` column range, `row_lazy` x length). Alone: BW -1.0%, sparse_solve -8%
     (perfbench Ir 485.7M / 521.5M).
   - `CsrMatrix::mul_block_transpose` (block Lanczos normal operator) now runs the
     lazy-lane row kernel on a cached CSR transpose instead of a `u128 %` per entry and
     lane. Lanczos Ir 1002.0M -> 829.6M with this plus the gathers.
2. `lazy-lane block_gram and block_axpy_small`: Lanczos 829.6M -> 646.2M.
   `block_axpy_small` was 45% and `block_gram` 25% of Lanczos Ir.
3. `flat approximant-basis columns`: `BasisColumn.f` is one `Vec`; allocation-free
   lambda shift. About -2.4% on block_wiedemann, -1% on sparse_solve.
4. `pc_degree_harness: branch-free symdiff`: wall (perfbench median_ns, 3 interleaved
   rounds): base 98.3/100.1/94.9 ms vs cand 77.7/72.3/75.0 ms.

## Identity and tests
- perfbench fingerprints equal for all five kernels above (incl. report fields).
- Lanczos result hash equal (scratch example, not committed).
- Module tests: `koblitz_sparse_la` has 34 passing tests (2 ignored) in a standalone
  crate that includes the unmodified-path file via `#[path]` (the full lib test build
  needs research fixtures not in this sparse worktree). Run after commit 3, before
  commit 4 (which only touches `pc_degree_harness`). Wide<L> widths exercise the
  transpose and axpy changes.
- Not run: the full-lib `cargo test` (not buildable in this worktree).

## Reverted ideas
- Branch-free merge in `sparse_macaulay::xor_sorted`: Ir +8.6%, wall 30.3 -> 39-44 ms on
  `sparse_high_macaulay_k5m3d5` (1.3x slower; merge runs are predictable there). Reverted.
- Reducing the approximant-basis per-element cost: `fixed_sub_from` is already 2 folds;
  a 3m-range variant saves one instruction and overflows for m >= 2^62. Not done.

## Deferred method changes (would alter counters/outputs, not done)
- Block Lanczos iteration count / block size policy; Wiedemann sequence length margin.
- Fast (half-GCD / PM-Basis) approximant basis: same generator, but changes `products`-type
  report accounting only if lengths change; larger restructuring, not attempted.
- AVX-512 IFMA lazy lanes for the SpMV (valgrind cannot measure).

## Host notes
Disk was at 1.5-3.4 GB free; the worktree is sparse (docs/, research/ omitted) and
the large build dirs were deleted at the end. SpMV inner loop (about 50% of BW Ir) is at its
mul/add/adc floor (4 lanes x 3 instructions per entry).
