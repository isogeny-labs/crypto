# `rootfind-kmitm` — probe-favoring MITM + batch-inv + Bloom + threads (2026-11-19)

**Decision: engineering.** New oracle id beside `ksum` / `rootfind-subtract`.
Default split stores a table of **left** = `min(9, m−1)` sums and probes
with every **right** = `m−left` sum `S` via `R − S`, rejecting overlapping
summands. For `m ≤ 10` that makes `right = 1`. Right sums with `right ≥ 2`
are materialised once at prepare. Pass `split=balanced` for the classic
`⌈m/2⌉ / ⌊m/2⌋` tradeoff when the left 9-sum table would be too large.

On primes with siblings:

1. **Batch inversion** — `right = 1` batches over the factor base;
   `right ≥ 2` batches over the precomputed right-half sums (Montgomery
   x-only keys into the left table).
2. **Word-blocked Bloom + mont→witness map** — Bloom and lookup stay in
   Montgomery abscissae (no `from_mont` on the miss path).
3. **Hot-path hygiene** — precomputed FB Montgomery coords, in-place
   batch-inv (no per-probe alloc), inline right indices.
4. **`decompose_lanes`** — PLAN lever 2 scalar: up to four targets share
   one batch-inv; field work is interleaved across the lane.
5. **`decompose_threaded`** — PLAN lever 4: `Arc`-shared prepared tables,
   `fork_worker`, `std::thread::scope` over independent targets.

| | |
|---|---|
| oracle id | `rootfind-kmitm:m=3..12` |
| code | `src/cryptanalysis/ic_framework/rootfind_kmitm.rs` |
| wall bench | `examples/rootfind_kmitm_wall.rs` |
| host | shared VM; `[pcc:unverified]` |

## Wall (24-bit prime, \|F_x\| = 12, 32 random targets)

Same-host baseline → after hot-path + lanes (see `results.json`):

| m | left/right | baseline probe | now probe | lanes4 | vs baseline (single / lanes4) |
|---|---|---:|---:|---:|---|
| 3 | 2/1 | 0.97 µs | 0.70 µs | 0.49 µs | **1.39× / 1.99×** |
| 4 | 3/1 | 0.94 µs | 0.69 µs | 0.48 µs | **1.36× / 1.96×** |
| 5 | 4/1 | 0.94 µs | 0.66 µs | 0.45 µs | **1.43× / 2.08×** |
| 6 | 5/1 | 0.92 µs | 0.68 µs | 0.46 µs | **1.36× / 2.00×** |
| 8 | 7/1 | 1.03 µs | 0.75 µs | 0.60 µs | **1.37× / 1.70×** |
| 10 | 9/1 | 1.12 µs | 0.81 µs | 0.69 µs | **1.38× / 1.62×** |
| 12 | 9/3 | 26.5 µs | 23.0 µs | 23.0 µs | **1.15× / 1.15×** |

m=12 remains bound by ~1760 right-sum field ops a target; lanes amortize
the inversion but not the arithmetic.

### PLAN lever 4 — threads over targets (m=12)

`right = 1` probes (~0.7 µs) stay spawn-bound: extra threads lose at this
`|F_x|` / target count. The win is m=12 (~23 µs/target). Median of five
wall reps at 64 targets (`m12_thread_reps.txt` / `results.json`):

| threads | median ns/tgt | vs thr1 |
|---|---:|---:|
| 1 | 21.97 µs | 1.00× |
| 2 | 15.92 µs | **1.38×** |
| 4 | 7.83 µs | **2.81×** |

Planted-sum recovery tests pass for m = 3..6, 10, 12; lanes and
`decompose_threaded` recover a planted m=4 sum.

## How to read the numbers

- Prefer `ksum` for m≤9 when memory allows and a single hash lookup beats
  an `|F|` pass.
- Large A: left=`min(9,m−1)` grows as `C(A,L)·2^{L−1}`; use
  `split=balanced` when that table does not fit.
- Counted `S` is the MITM's own additions. No security / sub-rho claim.
- Remaining PLAN lever: AVX/IFMA lanes (and table-build batching) stay in
  the #4708 prototype until parity/replay are wired.

## Reproduce

```bash
cargo test --release --lib rootfind_kmitm
cargo run --release --example rootfind_kmitm_wall -- \
  --bits 24 --size 12 --targets 32
```
