# `rootfind-kmitm` — probe-favoring MITM for m = 3..12 (2026-11-19)

**Decision: engineering.** New oracle id beside `ksum` / `rootfind-subtract`.
Default split stores a table of **left** = `min(9, m−1)` sums and probes
with every **right** = `m−left` sum `S` via `R − S`, rejecting overlapping
summands. For `m ≤ 10` that makes `right = 1` (one pass over the factor
base). Right sums with `right ≥ 2` are materialised once at prepare.
Pass `split=balanced` for the classic `⌈m/2⌉ / ⌊m/2⌋` tradeoff when the
left 9-sum table would be too large.

| | |
|---|---|
| oracle id | `rootfind-kmitm:m=3..12` |
| code | `src/cryptanalysis/ic_framework/rootfind_kmitm.rs` |
| wall bench | `examples/rootfind_kmitm_wall.rs` |
| host | shared VM; `[pcc:unverified]` |

## Why this exists

- Full [`KSumTable`](`ksum`) packs indices into one `u64` and **stops at
  m = 9**. `m = 10..12` had no root-finding table path.
- Balanced half-splits cut **build** hard but leave probe enumerating
  `⌊m/2⌋`-sums every target (orders of magnitude slower than full `ksum`
  hash lookup when that table fits).
- Probe-favoring puts almost all work in the left table so probes become
  `O(|F|)` for `m ≤ 10` and `O(C(A,3)·4)` for `m = 12`.

## Wall (24-bit prime, \|F_x\| = 12, 32 random targets) — probe split

From `wall.txt` / `results.json`:

| m | left/right | kmitm build | kmitm probe | prior balanced probe |
|---|---|---:|---:|---:|
| 3 | 2/1 | 12 µs | 1.7 µs | ~1.8 µs |
| 4 | 3/1 | 58 µs | 1.9 µs | ~31 µs (**~17×**) |
| 5 | 4/1 | 0.27 ms | 1.5 µs | ~34 µs (**~23×**) |
| 6 | 5/1 | 0.96 ms | 1.7 µs | ~309 µs (**~183×**) |
| 8 | 7/1 | 4.2 ms | 1.8 µs | ~1.4 ms (**~808×**) |
| 10 | 9/1 | 6.0 ms | 2.1 µs | ~5.0 ms (**~2390×**) |
| 12 | 9/3 | 5.4 ms | 195 µs | ~12.4 ms (**~63×**) |

Planted-sum recovery tests pass for m = 3..6, 10, 12; full `ksum`
refuses m = 10.

## How to read the numbers

- **Probe wall:** default split is for many-target / probe-dominated
  workloads. Prefer `ksum` for m≤9 when memory allows and a single hash
  lookup beats an `|F|` pass.
- **Large A:** left=`min(9,m−1)` grows as `C(A,L)·2^{L−1}`; use
  `split=balanced` when that table does not fit.
- Counted `S` is the MITM's own additions (not comparable 1:1 to full
  `ksum`'s `S`). No security / sub-rho claim.

## Reproduce

```bash
cargo test --release --lib rootfind_kmitm
cargo run --release --example rootfind_kmitm_wall -- \
  --bits 24 --size 12 --targets 32
```
