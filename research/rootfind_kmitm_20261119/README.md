# `rootfind-kmitm` — meet-in-the-middle for m = 3..12 (2026-11-19)

**Decision: engineering.** New oracle id beside `ksum` / `rootfind-subtract`.
Stores a table of **left** = `⌈m/2⌉` sums and probes with every **right** =
`⌊m/2⌋` sum `S` via `R − S`, rejecting overlapping summands.

| | |
|---|---|
| oracle id | `rootfind-kmitm:m=3..12` |
| code | `src/cryptanalysis/ic_framework/rootfind_kmitm.rs` |
| wall bench | `examples/rootfind_kmitm_wall.rs` |
| host | shared VM; `[pcc:unverified]` |

## Why this exists

- Full [`KSumTable`](`ksum`) packs indices into one `u64` and **stops at
  m = 9**. `m = 10..12` had no root-finding table path.
- Full `m`-sum build is ~`2^{m−1}·C(A,m)` additions. MITM build is
  ~`2^{L−1}·C(A,L)` with `L = ⌈m/2⌉` — drastic once `m ≥ 4`.

## Wall / build (24-bit prime, \|F_x\| = 12, 32 random targets)

From `wall.txt`:

| m | left/right | kmitm build | ksum build | build speedup | kmitm probe | ksum probe |
|---|---|---:|---:|---:|---:|---:|
| 3 | 2/1 | 12 µs | 87 µs | **7×** | 1.8 µs | 17 ns |
| 4 | 2/2 | 6 µs | 413 µs | **64×** | 31 µs | 7 ns |
| 5 | 3/2 | 52 µs | 1.2 ms | **24×** | 34 µs | 11 ns |
| 6 | 3/3 | 45 µs | 3.5 ms | **76×** | 309 µs | 28 ns |
| 8 | 4/4 | 312 µs | 8.6 ms | **28×** | 1.4 ms | 15 ns |
| 10 | 5/5 | 0.87 ms | n/a | — | 5.0 ms | n/a |
| 12 | 6/6 | 2.5 ms | n/a | — | 12.4 ms | n/a |

Planted-sum recovery tests pass for m = 3..6 and m = 10; full `ksum`
refuses m = 10.

## How to read the numbers

- **Build / memory:** kmitm wins hard (and is the only option for m>9).
- **Per-target probe on misses:** full `ksum` is one hash lookup; kmitm
  enumerates the right half, so probe wall is higher when the full table
  fits. Prefer `ksum` for many targets at m≤9 when memory allows; prefer
  `rootfind-kmitm` for high m, tight memory, or build-dominated workloads.
- Counted `S` is the MITM's own additions (not comparable 1:1 to full
  `ksum`'s `S`). No security / sub-rho claim.

## Reproduce

```bash
cargo test --release --lib rootfind_kmitm
cargo run --release --example rootfind_kmitm_wall -- \
  --bits 24 --size 12 --targets 32
```
