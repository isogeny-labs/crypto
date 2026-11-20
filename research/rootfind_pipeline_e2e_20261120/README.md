# Rootfind pipeline e2e — planted-log IC wall (2026-11-20)

**Decision: engineering.** `[pcc:unverified]`. Complete planted-log index
calculus (factor base → prepare → relation collection → Gauss → verify
`[d]G = Q`), oracle swapped, everything else fixed. Not the oracle-only
microbench in `research/rootfind_kmitm_20261119/`.

| | |
|---|---|
| harness | `examples/rootfind_pipeline_e2e.rs` |
| instance (small) | 20-bit prime, `r = 1 046 999`, `|Fₓ| = 14` |
| host | shared VM (see `host.txt`) |

Every reported solve verified. Fail-closed. No security / sub-rho claim.
Charged `S` for `rootfind-floor:m=3` matches `rootfind-subtract:m=3` on
the same seeds (same search).

## Breakthrough: floor prepare no longer doubles PairTable work

**Bug.** `rootfind-floor:m=3` built `PairTable::build_negation_folded`, then
**re-added every pair** to fill a Montgomery `pair_map`, so prepare wall was
~2× subtract while decomp was already ~2× faster — e2e wall **lost** at
`|Fₓ| ≥ 256`.

**Fix.** One PairTable build; Bloom over `to_mont(x)` of folded keys; on a
Bloom pass, `from_mont` → `lookup_folded` on the table (no second FastMap).

Evidence: `breakthrough_prepare_b28_a512_v2.txt`,
`breakthrough_prepare_b24_a256.txt`, `e2e_after_floor_prepare.txt`.

### Large-|Fₓ| cold pipeline (median of 5, prepare-focus panel)

Baseline = `subtract:m=3/walk`. All solves verified; `S` identical floor↔subtract.

| \|Fₓ\| | bits | arm | setup | decomp | wall | vs subtract |
|---:|---:|---|---:|---:|---:|---:|
| 512 | 28 | subtract:m=3/walk | 9.5 ms | 12.9 ms | 25.5 ms | 1.00× |
| 512 | 28 | **floor:m=3/walk** | 11.1 ms | 6.5 ms | **20.6 ms** | **1.24×** |
| 256 | 24 | subtract:m=3/walk | 2.2 ms | 1.7 ms | 4.9 ms | 1.00× |
| 256 | 24 | **floor:m=3/walk** | 2.5 ms | 1.0 ms | **4.4 ms** | **1.12×** |

Before the fix (same host, A=512): floor setup **26 ms** / wall **39 ms**
(0.71× vs subtract). After: setup **11 ms** / wall **20.6 ms** (**1.24×**).

IFMA / `walk-prime-batch` arms do not beat floor+walk here (see older
`breakthrough_b28_a512.txt`); kmitm:m=4 remains prepare-bound at large A.

## Cold single-DLP (median of 5, after prepare fix)

Baseline = `rootfind-subtract:m=3`. Source: `e2e_after_floor_prepare.txt`.

| arm | median wall | vs subtract wall | notes |
|---|---:|---:|---|
| subtract:m=3 | 4.0 ms | 1.00× | baseline |
| floor:m=3 | 2.5 ms | **1.61×** | same `S` |
| floor:m=2 | 12.6 ms | 0.31× | vs m=3; ~2× vs subtract:m=2 |
| kmitm:m=4 | 1.1 ms | **3.51×** | different search; lower `S` |
| kmitm:m=6 | 3.4 ms | 1.17× | prepare starts to show |
| kmitm:m=10 | 57 ms | 0.07× | prepare-bound |
| kmitm:m=12 | 58 ms | 0.07× | prepare-bound |
| kmitm:m=12 thr=4 | 59 ms | 0.07× | cold single-DLP; setup dominates |

## Amortized (16 planted logs, cold pipeline each)

From the same run (`e2e_after_floor_prepare.txt` JSON `gains` /
`amortized_multi_dlp`).

| arm | per-DLP wall | vs subtract:m=3 |
|---|---:|---:|
| subtract:m=3 | ~3–4 ms | 1.00× |
| floor:m=3 | ~2 ms | **~1.6×** |
| kmitm:m=4 | ~1 ms | **~3×** |
| kmitm:m=12 | ~58 ms | prepare-bound (re-prepares) |

## Shared-prepare m=12 threaded collect (16 DLPs)

One prepare (~58 ms), then 16 collects with `decompose_threaded`:

| threads | collect total | per-decomp | vs thr1 |
|---|---:|---:|---:|
| 1 | 19.8 ms | 1.1 ms | 1.00× |
| 2 | 27.3 ms | 1.6 ms | 0.72× |
| 4 | 23.5 ms | 1.3 ms | 0.87× |

At this size the relation phase is ~1 ms/DLP — thread spawn loses.
Oracle-wall m=12 with heavy right=3 probes (see
`research/rootfind_kmitm_20261119/`) is where lever 4 wins (~2.8× at 4
threads).

## Recipe (engineering)

| regime | prefer | why |
|---|---|---|
| small \|Fₓ\| (~14) | `rootfind-kmitm:m=4` | ~3.5× e2e wall vs subtract:m=3 |
| same search as subtract, any \|Fₓ\| measured | `rootfind-floor:m=3` | 1.1–1.6× e2e; identical `S` |
| large \|Fₓ\| + kmitm:m=4 | avoid cold | prepare-bound |

## Reproduce

```bash
# Small-A full panel
cargo run --release --example rootfind_pipeline_e2e -- \
  --bits 20 --size 14 --repeats 5 --multi 16

# Large-A prepare-focus (floor vs subtract)
cargo run --release --example rootfind_pipeline_e2e -- \
  --bits 28 --size 512 --repeats 5 --panel breakthrough \
  --breakthrough-focus prepare
```
