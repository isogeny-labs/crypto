# `rootfind-kmitm` — probe-favoring MITM + batch-inv + Bloom (2026-11-19)

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
| 3 | 2/1 | 0.97 µs | 0.67 µs | 0.51 µs | **1.45× / 1.91×** |
| 4 | 3/1 | 0.94 µs | 0.68 µs | 0.45 µs | **1.38× / 2.08×** |
| 5 | 4/1 | 0.94 µs | 0.68 µs | 0.45 µs | **1.38× / 2.08×** |
| 6 | 5/1 | 0.92 µs | 0.64 µs | 0.44 µs | **1.45× / 2.10×** |
| 8 | 7/1 | 1.03 µs | 0.74 µs | 0.54 µs | **1.40× / 1.91×** |
| 10 | 9/1 | 1.12 µs | 0.77 µs | 0.66 µs | **1.46× / 1.69×** |
| 12 | 9/3 | 26.5 µs | 23.2 µs | 22.6 µs | **1.14× / 1.18×** |

m=12 remains bound by ~1760 right-sum field ops a target; lanes amortize
the inversion but not the arithmetic. Threads (PLAN lever 4) and AVX/IFMA
lanes stay open.

Planted-sum recovery tests pass for m = 3..6, 10, 12; lanes recover a
planted m=4 sum.

## How to read the numbers

- Prefer `ksum` for m≤9 when memory allows and a single hash lookup beats
  an `|F|` pass.
- Large A: left=`min(9,m−1)` grows as `C(A,L)·2^{L−1}`; use
  `split=balanced` when that table does not fit.
- Counted `S` is the MITM's own additions. No security / sub-rho claim.
- Remaining PLAN levers (AVX lanes, threads, table-build batching) stay in
  the #4708 prototype until parity/replay are wired.

## Reproduce

```bash
cargo test --release --lib rootfind_kmitm
cargo run --release --example rootfind_kmitm_wall -- \
  --bits 24 --size 12 --targets 32
```
