# ecdl-tensor-ic

A toy, end-to-end implementation of index calculus for the elliptic-curve
discrete logarithm problem in which relations are found by **Boolean tensor
contraction**. It is a native Rust port of the Go prototype `ecdltensor`
(carrier branch `research/ecdl-tensor-go-source-20261029`, commit 1c29f66).
It follows the four-step algorithm of the paper that solves ECDL in expected
time 2^O(b/(log b)^(1/4)), conditional on Theorem 2 of [Ope26]: index calculus
with randomly hashed factor bases.

`PORTING.md` maps every Go identifier to its Rust counterpart and records the
design choices of the port. The related crate `crates/hashed-ecdl-tensor` is a
separate implementation of the Bochum hashed-factor-base reduction; this crate
does not share code with it.

## What is implemented

| Step | Code | Notes |
|---|---|---|
| Prime-order curves over F_p | `curve.rs` | Random b-bit curves found by naive point counting (b ≤ 26) |
| 1. Factor-base selection | `relation.rs` `new_factor_bases`, `new_affine_factor_bases`, `new_windowed_factor_bases` | m random hashed membership predicates, each with \|F_i\| ≈ (2r)^(1/m). There is no notion of smoothness. The default hash is keyed SHA-256. The gadget path uses a random affine hash, A ∈ F_i iff (α_i x + β_i y + γ_i) mod p < U_i, because that is cheap to check with low-degree gadgets. A coordinate window can be added: A ∈ F_i iff x(A) < p>>k as well (see below). |
| 2. Relation finding | `relation.rs` `relation_network`, `tensor.rs` | `[u]P+[v]Q = A_1+…+A_m` is encoded as a Boolean tensor network with (2m−1)(2b+1) edges. |
| Counting → search | `tensor.rs` `find_witness` | Self-reduction fixes one bit at a time, so any **counting** solver works. |
| 3. Linear algebra | `linalg.rs` | Left-kernel vector mod r by sparse Gaussian elimination (`left_kernel_vector_sparse`, used by the solver); `left_kernel_vector` is the dense reference |
| 4. Extraction | `solver.rs` | ξ = −U/V, always checked against `[ξ]P == Q`; an unverified answer is never returned. |

Relation search has two elimination orders. The default contracts the
network forward, which costs |F_1| x ... x |F_m|. `Solver::mitm` instead
contracts it from both ends toward the middle (`relation::mitm_find_relation`):
it enumerates the first half of the factor bases into a table keyed by the
partial sum and looks up the complement from the second half. That is about
2 sqrt(2r) point additions per attempt instead of 2r — the same network and the
same set of solutions, only a different elimination order, returning one
witness rather than a count. `ecdldemo -mitm` selects it.

With meet-in-the-middle, relation search is also parallel: the solver draws a
batch of targets in order, evaluates them on `Solver::threads` workers
(`ecdldemo -threads N`, 0 = one per core), and keeps the hits in target order,
so the outcome does not depend on the thread count. Only the scalar draws stay
sequential. While an `opcount` tally is active the search stays on one thread,
because the tally is thread-local.

Counters sit behind the `Counter` trait: `SparseContractor` (exact,
vertex-by-vertex, sums out each edge once no later vertex uses it),
`BruteForceCounter` (reference) and `DFSCounter` (depth-first along a hinted
schedule, needed for gadget networks). An [Ope26] solver would be another
`Counter`.

The low-degree gadgets (`gadgets.rs`, `pointgadget.rs`) give O(b) edges and
maximum degree O(b^(3/4)): `mul_check` (two-stage four-step NTT), `lin_check`,
`range_check`, `is_zero`, `select`, `zero_if`, branchless `complete_add`,
`validity_check`, `membership_check`, and fan-out copy vertices. **Hints only
steer `DFSCounter`; the count is defined by the vertices alone.** The gadget
tests run in exhaustive mode, where every hint enumerates every bit pattern it
writes, and check that exactly one assignment survives and that it is correct.

### Coordinate window

`new_windowed_factor_bases(c, m, rng, k)` adds a compact coordinate constraint,
F_i = {A : x(A) < p>>k and hash_i(A) < U_i}, and scales U_i by 2^k so |F_i|
stays near (2r)^(1/m); if the window alone would be smaller than that, k is
lowered until it fits. `k = 0` is exactly the old behaviour, and
`new_affine_factor_bases` delegates to it, so the default path is unchanged.

The window fixes the high bits of x, so the sets no longer have to be built by
enumerating the group:

- `Curve::window_points(x_bound)` enumerates the points with x < x_bound by
  taking a modular square root of x^3 + ax + b per abscissa (Tonelli–Shanks in
  `field::sqrt_mod`), so its cost is O(x_bound) square roots, not O(|G|).
- `FactorBases::window_candidates(c, i)` is F_i from the window alone, and
  `list` uses `list_window`, which does the same for the union of windows.
- `membership_check` adds one `range_check` on x, a block-level compare chain
  of degree O(t), so the gadget network keeps its O(b) edges and O(b^(3/4))
  degree.
- The gadget relation network takes its hint candidates from the window, so a
  relation search never touches a point outside it.
- The window does not need gadgets. The solver applies it whenever
  `window_bits > 0`, and the meet-in-the-middle path uses it too; it is a
  constraint on the affine hash, not on how relations are encoded.

Listing cost at b = 24 (p = 8388617, m = 2), measured: k = 0 takes 859 ms and
evaluates a hash per group point; k = 8 takes 12 ms and holds the same |F_i|.

`ecdlscale` prints the same table as the Go command, byte for byte:

|     b | mul edges/b | add edges/b | point edges/b | max degree | degree / b^(3/4) |
|------:|------------:|------------:|--------------:|-----------:|-----------------:|
|    64 |        35.2 |       218.2 |         217.1 |        352 |             15.6 |
|   256 |        31.4 |       198.4 |         195.9 |       1248 |             19.5 |
|  1024 |        29.1 |       186.8 |         183.1 |       2304 |             12.7 |
|  4096 |        27.7 |       180.2 |         175.6 |       8768 |             17.1 |
| 16384 |        27.0 |       176.5 |         171.3 |      17024 |             11.8 |

The modulus there is p = 2^(b−1)+1, which is not prime; only the structure is
measured, and networks are built but never evaluated.

## Run

```sh
cargo test --release --locked
cargo clippy --locked --all-targets -- -D warnings
cargo run --release --bin ecdlscale
cargo run --release --bin ecdldemo -- -bits 14 -m 2
cargo run --release --bin ecdldemo -- -bits 8 -m 2 -gadgets
```

From the repository root the same tools are part of the `ic` research CLI,
with the global `--json` and `--out` report options, and the library is
re-exported as `crypto_lib::cryptanalysis::tensor_ic`:

```sh
cargo run --release --bin ic -- tensor scale                  # same table as ecdlscale
cargo run --release --bin ic -- tensor scale --bits 64,256 --json
cargo run --release --bin ic -- tensor demo --bits 12 --m 2 --seed 1
cargo run --release --bin ic -- tensor demo --bits 8 --gadgets
```

All 19 Go tests are ported (`tests/ecdl_test.rs`, `tests/gadget_test.rs`;
each carries a `// Go: TestName` marker), with the same primes, curves,
loop bounds, exhaustive modes and assertions.

## Differences from the Go code

- Randomness is `rand::rngs::StdRng` seeded with the Go seeds. Go's
  `math/rand` streams are not reproduced, so random curves, factor bases and
  sampled test inputs differ; deterministic structure (parameters, edge
  counts, degrees) is identical.
- Go's `ProbablyPrime(30)` (Miller–Rabin plus Baillie–PSW) is a Miller–Rabin
  test on 30 fixed prime bases here. It selects the same NTT primes Q at every
  size checked (b = 4 … 16384).
- Two Go defects are fixed (review findings on PR #2895; regression tests
  in `tests/regression_test.rs`): `new_factor_bases` redraws its SHA-256
  keys while any F_i is empty, as the affine version already does (an empty
  F_i made the solver's relation loop run forever), and `SupportVertex::new`
  drops duplicate rows (they made `SparseContractor` and `DFSCounter`
  disagree with `BruteForceCounter`).
- `sqrt_mod` returns `Option<u64>` (`None` plays Go's `ok == false`), and the
  window is `x_bound: Vec<u64>` plus `window_bits: usize` on `FactorBases`,
  `window_bits: usize` on `Solver`, and `--window` on `ecdldemo`. Go's
  `BenchmarkFactorBaseBuild` becomes a deterministic test
  (`window_build_avoids_whole_group_listing`) that counts membership hashes
  through `opcount` instead of timing.
- `Vertex` is an enum and `Counter` a trait; `Var` is `Rc<RefCell<…>>` with
  pointer identity. `Solver::solve_with_stats` returns partial `Stats` on
  error as Go does; `Builder::hint_nil` stands for Go's nil hint generator.

## What this does *not* show

- **There is no subexponential speedup here.** The paper's bound depends on
  the [Ope26] contraction algorithm, which is not implemented. Every counter
  in this crate is exponential; the prototype costs about r per relation,
  which is worse than Pollard rho.
- **The default network still reads whole points** (point-addition vertices
  of degree 3(2b+1)). The low-degree version is the gadget network
  (`-gadgets`).
- **The gadgets are our own construction**, including the NTT
  decomposition and the affine factor-base hash. They follow the paper's
  one-line description, not its (unavailable) encodings.
- **O(b) edges does not give a speedup on its own.** The running time comes
  entirely from the contraction solver.
- **Factor bases are listed by enumerating the group**, a toy shortcut.
- It runs at toy sizes only and cannot attack deployed curves such as P-256,
  P-521 or Ed25519.
