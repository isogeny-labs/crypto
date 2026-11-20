# Rootfind pipeline e2e — planted-log IC wall (2026-11-20)

**Decision: engineering.** `[pcc:unverified]`. Complete planted-log index
calculus (factor base → prepare → relation collection → Gauss → verify
`[d]G = Q`), oracle swapped, everything else fixed. Not the oracle-only
microbench in `research/rootfind_kmitm_20261119/`.

| | |
|---|---|
| harness | `examples/rootfind_pipeline_e2e.rs` |
| instance (small) | 20-bit prime, `|Fₓ| = 14` |
| host | shared VM (see `host.txt`); has `avx512ifma` |

Every reported solve verified. Fail-closed. No security / sub-rho claim.
Charged `S` for `rootfind-floor:m=3` matches `rootfind-subtract:m=3` on
the same seeds (same search).

## Breakthrough: batched prime PairTable prepare (PLAN lever 5)

**Change.** `rootfind-floor:m=3` and `rootfind-subtract:m=3` build the
negation-folded pair table with
`PairTable::build_negation_folded_prime_batch` (one inversion per abscissa
row) instead of serial `build_negation_folded`.

Evidence: `breakthrough_batch_b28_a512.txt`,
`breakthrough_batch_b24_a256.txt`, `e2e_after_batch_table.txt`.
Parity: `cargo test --release --lib rootfind_floor` (verdicts + charged
adds vs subtract).

### Large-|Fₓ| cold pipeline (median of 5)

| \|Fₓ\| | bits | arm | wall | vs subtract (same builder) | vs pre-batch subtract |
|---:|---:|---|---:|---:|---:|
| 512 | 28 | subtract:m=3/walk | 21.3 ms | 1.00× | — |
| 512 | 28 | **floor:m=3/walk** | **16.3 ms** | **1.30×** | **~1.56×** (was 25.5 ms) |
| 256 | 24 | subtract:m=3/walk | 3.9 ms | 1.00× | — |
| 256 | 24 | **floor:m=3/walk** | **3.3 ms** | **1.17×** | — |

Prior floor breakthrough (serial PairTable, A=512): wall 20.6 ms / 1.24×.
After batched prepare: **16.3 ms / 1.30×** (floor setup ~6.9 ms).

## Prior breakthrough: floor prepare no longer doubles PairTable work

`rootfind-floor:m=3` used to rebuild every pair into a Montgomery FastMap
after PairTable build (~2× prepare). Fixed to Bloom + PairTable lookup.
See `breakthrough_prepare_b28_a512_v2.txt`.

## Cold single-DLP (median of 5, after batch table)

Baseline = `rootfind-subtract:m=3`. Source: `e2e_after_batch_table.txt`.

| arm | median wall | vs subtract wall | notes |
|---|---:|---:|---|
| subtract:m=3 | 4.0 ms | 1.00× | baseline |
| floor:m=3 | 2.4 ms | **~1.6×** | same `S` |
| kmitm:m=4 | 1.1 ms | **~3.5×** | different search; lower `S` |
| kmitm:m=12 | ~60 ms | 0.07× | prepare-bound |

## Recipe (engineering)

| regime | prefer | why |
|---|---|---|
| small \|Fₓ\| (~14) | `rootfind-kmitm:m=4` | ~3.5× e2e wall vs subtract:m=3 |
| same search as subtract, any \|Fₓ\| measured | `rootfind-floor:m=3` | 1.17–1.6× e2e; identical `S` |
| large \|Fₓ\| + kmitm:m=4 | avoid cold | prepare-bound |

## Reproduce

```bash
cargo test --release --lib rootfind_floor
cargo run --release --example rootfind_pipeline_e2e -- \
  --bits 20 --size 14 --repeats 5 --multi 8
cargo run --release --example rootfind_pipeline_e2e -- \
  --bits 28 --size 512 --repeats 5 --panel breakthrough \
  --breakthrough-focus prepare
```
