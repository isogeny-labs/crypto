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
   x-only folded keys into the left table).
2. **Word-blocked Bloom filter** — five bits a word in front of every
   folded left lookup (lever-3 style from the 100× plan).

| | |
|---|---|
| oracle id | `rootfind-kmitm:m=3..12` |
| code | `src/cryptanalysis/ic_framework/rootfind_kmitm.rs` |
| wall bench | `examples/rootfind_kmitm_wall.rs` |
| host | shared VM; `[pcc:unverified]` |

## Wall (24-bit prime, \|F_x\| = 12, 32 random targets)

From `wall.txt` / `results.json` (Bloom + batch-inv on right≥2):

| m | left/right | kmitm build | kmitm probe | notes |
|---|---|---:|---:|---|
| 3 | 2/1 | 12 µs | 0.99 µs | batch-inv + Bloom |
| 4 | 3/1 | 55 µs | 0.96 µs | batch-inv + Bloom |
| 5 | 4/1 | 0.27 ms | 0.93 µs | batch-inv + Bloom |
| 6 | 5/1 | 0.91 ms | 0.92 µs | batch-inv + Bloom |
| 8 | 7/1 | 4.5 ms | 1.0 µs | batch-inv + Bloom |
| 10 | 9/1 | 5.7 ms | 1.1 µs | batch-inv + Bloom |
| 12 | 9/3 | 5.7 ms | **27 µs** | batch-inv over 1760 right sums + Bloom |

Vs the prior probe-favoring + right=1-only batch-inv on this host: right=1
probes ~1.0–1.8 µs → ~0.9–1.1 µs; **m=12 probe ~220 µs → ~27 µs (~8×)**.

Planted-sum recovery tests pass for m = 3..6, 10, 12.

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
