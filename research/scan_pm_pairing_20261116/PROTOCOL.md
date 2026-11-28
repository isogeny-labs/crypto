# Protocol: ±P pairing in the linear 2 + 1 scan (`lin21-ht`)

Frozen 2026-11-16 before any measured row. Local pilot on a shared 4-vCPU
container; nothing here can become a benchmark record, a leaderboard row or a
`benchmarks/README.md` row, whatever the outcome. Every number that follows
from this protocol is `[pcc:unverified]`.

## 1. Question

Does pairing the two base points `±P₁` above one abscissa, with a shared
inversion, reduce the cost of the repository's own scan arm `lin21-ht`
(`examples/ic_algebraic_floor/linear21.rs`, `Linear21::decide`,
`Variant::HalfTrace`) at commit `8d039abd94`?

## 2. What the existing code does (read before any run)

- `linear21.rs:445-479`: per block of 256 base points, `FastCurve::add_many(r,
  &negated[lo..hi])` (full affine sums, one batched inversion of `x_R + x_i`),
  then `Gf2::batch_inv` on the 256 abscissae. Both signs of every abscissa are
  visited as separate leaves; the two equal denominators of `±P₁` are both in
  the batch. No pairing in this arm.
- The library already has the pairing of the first inversion, unused by this
  arm: `FastCurve::add_many_lazy_paired_x` (`src/cryptanalysis/koblitz_fast.rs:454`)
  and the eight-lane `add_many_lazy_soa_paired` (`:583`), used by
  `ic_boundary.rs:8743`, `koblitz_index_calculus.rs:7909` and
  `src/bin/ecbench/pdp3_koblitz.rs:732`. Neither shares anything with the
  second inversion (`1/x(Q)`), which only `lin21-ht` needs.

So the answer to "already present" is: in the library, yes for the addition's
denominator; in `lin21-ht`, no. The pilot therefore runs.

## 3. Arms (one binary, same process, same targets)

| arm | t stage | leaf |
|---|---|---|
| `A1`, `A2` | `linear21.rs`, byte-identical copy of the repository file (sha256 below) | the repository's |
| `BC` | the same calls (`add_many`, `batch_inv`) | this study's copy of the leaf (`linear21_pm.rs`) |
| `PM` (`lin21-ht-pm`, primary) | ±P pairing, ONE batched inversion of `e·(x·x_R + √b)` per pair; `t₂ = t₁ + x·x_R/e²`, `1/t₁ = t₂(e/g)²`, `1/t₂ = t₁(e/g)²`; 11 mul + 4 sqr per pair after the batch | as `BC` |
| `LZ` (secondary) | library `add_many_lazy`, then `batch_inv` on the abscissae | as `BC` |
| `LP` (secondary) | library `add_many_lazy_soa_paired` (existing pairing), then `batch_inv` on the abscissae | as `BC` |

`A1`/`A2` is the A/A control: the baseline twice, interleaved with the others.
`BC` exists so that `BC/PM` isolates the t stage from any effect of the leaf
having been copied. `PM`, `LZ`, `LP` compute `y(Q)` only when a verification
needs it. Scan order, block size (256 leaves), kernel cap and skip rules are
the repository's in every arm.

## 4. Cells, targets, seeds, runs

- Cells (`a:n:d`, the 2026-11-13 study's, n ≥ 23): `1:23:8, 1:29:10, 0:31:11,
  0:37:13, 0:41:14, 1:47:16, 0:53:18, 1:59:20`. Koblitz curve and standard
  subspace base built by the repository's own constructors.
- Targets per cell: the first 64 non-decomposable `[k]G` of the seeded stream,
  up to 8 naturally decomposable ones (0 at n = 59), 8 planted
  `P_i + P_j + P_k`. Measured seed `20261116`. Development used seed `7` only
  (cells 13:5, 17:6, 19:7, 23:8, 37:13, 47:16; 16 to 24 "no" targets).
- Timing: decision mode (stop at the first witness), each arm timed per target
  with `Instant`, arm order rotated by target and repetition, 3 repetitions
  (2 at n = 59), per-target minimum. The t stage alone is timed the same way.
- Processes (`run.sh`): cells 23 to 47 in one process and 53 in another, in
  parallel; then n = 59 as two shards (32 + 32 "no", 4 + 4 planted) in
  parallel. Each process is capped at 590 s; rows completed before a cap are
  kept and the shortfall is reported. Another agent uses the same machine.
- One measured run. No rerun unless a process fails to produce rows; a rerun
  and its reason would be reported with both outputs.

## 5. Metrics

Over the "no" class (a full scan in every arm):

- **Counted (primary):** field operations per target `M + S + (n + 8)·I`
  (multiplications, squarings, inversions; one counter scheme, a const-generic
  build of the same code; `BC` and `PM` instrumented, `LZ`/`LP` t stages
  modelled at 3 products per batched element + 9 and one inversion per batch).
  Ratio `BC / arm`. `BC`'s repository-level counters must equal `A1`'s on every
  target.
- **Counted, elimination priced (reported beside it):** `8M + 8S + 8(n+8)I +
  T + W`, `T` = byte-table lookups, `W` = word operations of the `F₂`
  elimination (4 per pivot step), system build (3 per column) and kernel
  enumeration: the price vector of `research/scan_floor_leaf_20261116/`. The
  rule is evaluated on both; the verdict of record is the field-operation one
  because the task names it. If the two differ, both are stated.
- **Wall (secondary):** `Σ A1 / Σ arm` with a paired bootstrap 95% interval
  (2000 resamples over targets). A/A band: the interval of `Σ A1 / Σ A2`,
  widened symmetrically about 1 (`hi = max(hi, 1/lo)`). "Outside the band in
  the reducing direction": the lower end of `A1/arm` exceeds the band's `hi`.

## 6. Decision rule (per arm; `PM` is the arm of record, `LP` is reported)

With the three largest n (47, 53, 59):

- **holds** if `BC/arm` counted ≥ 1.05 at all three AND wall is outside the
  A/A band in the reducing direction at all three;
- **fails** if `BC/arm` counted < 1.02 at n = 59, or wall is inside the band at
  all three;
- **inconclusive** otherwise.

## 7. Amdahl ceiling, stated before measuring

Stage breakdown of the baseline from the development seed (seed 7, 16 "no"
targets per cell), `[pcc:unverified]`:

| n | field ops/leaf | t stage, field ops | t stage, elimination priced | t stage, wall (t stage alone / whole `A1`) | A1 ns/leaf |
|--:|--:|--:|--:|--:|--:|
| 23 | 24.8 | 37.7% | 22.4% | 11.3% | 247 |
| 37 | 33.4 | 28.2% | 13.6% | 4.0% | 394 |
| 47 | 33.0 | 28.8% | 11.7% | 3.5% | 473 |

Ceilings for any change confined to the t stage, at n = 47: 1.40 in field
operations, 1.13 with the elimination priced, 1.036 in wall time. The wall
share is far below the counted share because the repository's t stage runs
inside the library (inlined `pclmulqdq` products, four interleaved inversion
chains) at about 16 ns per leaf, while the leaf's products are calls from the
example crate and its elimination is not a field operation.

Development-seed readings, for the record: `BC/PM` field operations 1.054 to
1.061, priced 1.022 to 1.035; `BC/LP` (model) 1.080 to 1.095; whole-leaf wall
`A1/PM` 0.999 and 1.019 at n = 37 and 47. Expectation: the wall condition is
not met for `PM`.

## 8. Gates (all arms, every measured target)

1. Verdict and first witness equal across `A1, A2, BC, PM, LZ, LP`; the witness adds up to `R`.
2. Full decomposition set (all-witness mode) of `PM` equals `BC`'s on every
   target; of `LZ` and `LP` for d ≤ 16 (at d ≥ 18 they are checked by gate 1 only).
3. For d ≤ 11 (n = 23, 29, 31) the set equals exhaustive pair enumeration
   (`Lin::exhaustive`: all `(i, j)`, third point by table lookup, no linear algebra).
4. `BC`'s counters `trials, consistent, splits, gauss_xors, field_muls,
   field_invs, verify_adds` equal the repository arm's; `PM`'s leaf counters
   equal `BC`'s.
5. All planted targets decided "yes".

A failed gate voids the cell for the arm concerned.

## 9. Frozen inputs

| file | sha256 |
|---|---|
| `main.rs` | `167a0b0e594184db8c957613aff5f1742c89e2088c9c7ac8dd1f92b010385a70` |
| `linear21_pm.rs` | `02377359b8ae3adbf67bda37157d7de804293fdd4720f30bd1b9e03ffe6ef1ff` |
| `linear21.rs` (= `examples/ic_algebraic_floor/linear21.rs`) | `0554f40f74d8bba367c09766079d7dd61925ad5d5eb349ad8e4099302068bb22` |
| `build.sh` | `ff78891c7102380f46058d9eed7fa71a91c73d12246d59803d0a70f531d37627` |
| `run.sh` | `b2b4a39452fb36af6de921a52efe0a28a7aae988cc33df347ab118fb157930b4` |
| binary `scan_pm` | `0022edce8f273589ee7de8ac5ad1eabd6bd957f197e0caa2bd664268dba9f177` |
| `libcrypto_lib-42993a31e73be122.rlib` (cargo `--release`, HEAD `8d039abd94`) | `594f6a40aef4a52dcb007108f6eb958e93f451afb62d508656fd04587b567c0f` |

`rustc 1.97.0`; Intel Xeon 2.10 GHz with `pclmulqdq`, `avx512f`, `vpclmulqdq`.
The repository example `ic_algebraic_floor` built at this commit (11 min 09 s).

## 10. Deviations from the brief, declared now

- The harness is compiled with `rustc` against the library `rlib` that
  `cargo build --release --example ic_algebraic_floor` produced, not as a
  scratch crate with a path dependency: the second route would compile the
  library a second time on a shared machine. No repository file is changed.
- Three arms were added to the three asked for (`BC`, `LZ`, `LP`), because the
  library's existing pairing and lazy addition are the comparators the earlier
  pilot lacked.
- The rule is evaluated on two counted metrics (§5).

## 11. Not covered, not claimed

No end-to-end relation or ECDLP cost, no matched rho reference, no validator
report, no isolab host, no benchmark record. Other curves, even n, `b ≠ 1`,
n > 59, and a leaf moved into the library are outside this pilot.
`research-kb` and the lead atlas are not installed in this container and were
not queried; no novelty statement is made.
