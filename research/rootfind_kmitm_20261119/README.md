# `rootfind-kmitm` — probe-favoring MITM + batch-inv right=1 (2026-11-19)

**Decision: engineering.** New oracle id beside `ksum` / `rootfind-subtract`.
Default split stores a table of **left** = `min(9, m−1)` sums and probes
with every **right** = `m−left` sum `S` via `R − S`, rejecting overlapping
summands. For `m ≤ 10` that makes `right = 1`. Right sums with `right ≥ 2`
are materialised once at prepare. Pass `split=balanced` for the classic
`⌈m/2⌉ / ⌊m/2⌋` tradeoff when the left 9-sum table would be too large.

On primes with siblings, the **right=1** probe uses a Montgomery
**batch inversion** over the factor base and folded-key lookups into the
left table (lever-1 style from `rootfind-floor`) — one inversion a target
instead of a full group subtraction per base point.

| | |
|---|---|
| oracle id | `rootfind-kmitm:m=3..12` |
| code | `src/cryptanalysis/ic_framework/rootfind_kmitm.rs` |
| wall bench | `examples/rootfind_kmitm_wall.rs` |
| host | shared VM; `[pcc:unverified]` |

## Wall (24-bit prime, \|F_x\| = 12, 32 random targets) — probe split + batch-inv

From `wall.txt` / `results.json`:

| m | left/right | kmitm build | kmitm probe | notes |
|---|---|---:|---:|---|
| 3 | 2/1 | 12 µs | 1.1 µs | batch-inv |
| 4 | 3/1 | 55 µs | 1.4 µs | batch-inv |
| 5 | 4/1 | 0.26 ms | 1.0 µs | batch-inv |
| 6 | 5/1 | 0.86 ms | 1.2 µs | batch-inv |
| 8 | 7/1 | 4.1 ms | 1.4 µs | batch-inv |
| 10 | 9/1 | 5.3 ms | 1.8 µs | batch-inv |
| 12 | 9/3 | 5.4 ms | 220 µs | precomputed right |

Vs the first balanced half-split on this host (same A): probe was
~31 µs–12 ms; probe-favoring alone dropped that to ~1.7–195 µs; batch-inv
trims another ~1.2–1.5× on right=1 at this small A (larger A amortises the
single inversion over more base points).

Planted-sum recovery tests pass for m = 3..6, 10, 12.

## How to read the numbers

- Prefer `ksum` for m≤9 when memory allows and a single hash lookup beats
  an `|F|` pass.
- Large A: left=`min(9,m−1)` grows as `C(A,L)·2^{L−1}`; use
  `split=balanced` when that table does not fit.
- Counted `S` is the MITM's own additions. No security / sub-rho claim.

## Reproduce

```bash
cargo test --release --lib rootfind_kmitm
cargo run --release --example rootfind_kmitm_wall -- \
  --bits 24 --size 12 --targets 32
```
