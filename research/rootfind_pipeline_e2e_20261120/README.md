# Rootfind pipeline e2e — planted-log IC wall (2026-11-20)

**Decision: engineering.** `[pcc:unverified]`. Complete planted-log index
calculus (factor base → prepare → relation collection → Gauss → verify
`[d]G = Q`), oracle swapped, everything else fixed. Not the oracle-only
microbench in `research/rootfind_kmitm_20261119/`.

| | |
|---|---|
| harness | `examples/rootfind_pipeline_e2e.rs` |
| instance | 20-bit prime, `r = 1 046 999`, `|Fₓ| = 14`, 3 seeds |
| host | shared VM |

Every reported solve verified. Fail-closed. No security / sub-rho claim.

## Cold single-DLP (median of 3)

Baseline = `rootfind-subtract:m=3`.

| arm | median wall | decomp | vs subtract wall | notes |
|---|---:|---:|---:|---|
| subtract:m=3 | 4.1 ms | 4.0 ms | 1.00× | baseline |
| floor:m=3 | 2.5 ms | 2.5 ms | **1.61×** | same `S` (lever 1) |
| floor:m=2 | 12.7 ms | 12.7 ms | 0.32× | vs m=3 baseline; **~2.0× vs subtract:m=2** |
| kmitm:m=4 | 1.1 ms | 1.0 ms | **3.54×** | different search; lower `S` |
| kmitm:m=6 | 3.3 ms | 0.2 ms | 1.23× | prepare starts to show |
| kmitm:m=10 | 64 ms | 0.2 ms | 0.06× | prepare-bound |
| kmitm:m=12 | 59 ms | 1.2 ms | 0.07× | prepare-bound |
| kmitm:m=12 thr=4 | 60 ms | 1.4 ms | 0.07× | cold single-DLP; setup dominates |

## Amortized (16 planted logs, cold pipeline each)

| arm | per-DLP wall | vs subtract:m=3 |
|---|---:|---:|
| subtract:m=3 | 3.4 ms | 1.00× |
| floor:m=3 | 2.1 ms | **1.63×** |
| kmitm:m=4 | 1.2 ms | **2.87×** |
| kmitm:m=12 | 58.8 ms | 0.06× (re-prepares) |

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

## Reproduce

```bash
cargo run --release --example rootfind_pipeline_e2e -- \
  --bits 20 --size 14 --repeats 3 --multi 16
```
