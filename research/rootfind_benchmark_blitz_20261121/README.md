# Rootfind benchmark blitz (2026-11-21)

**Decision: engineering.** `[pcc:unverified]`. Broad wall-time matrix for
prime-field rootfind oracles — not a security / sub-rho claim.

| | |
|---|---|
| host | 4 vCPU shared VM with `avx512ifma` (see `host.txt`) |
| oracle wall | `examples/rootfind_floor_wall.rs`, `examples/rootfind_kmitm_wall.rs` |
| planted-log e2e | `examples/rootfind_pipeline_e2e.rs` (FB→prepare→collect→Gauss→verify) |
| machine scoreboard | `results.json` |
| driver | `run_matrix.sh` |

Every planted-log solve in the e2e cells below verified (`unverified=0`).
High-m / large-`|Fₓ|` cells that refuse prepare (packing limit or OOM guard)
are recorded as `SKIP`, not as failures.

## Scoreboard — planted-log e2e wall (median)

Baseline = `rootfind-subtract:m=3`. Floor keeps the same search / charged `S`.

| cell | bits | \|Fₓ\| | subtract | floor | floor× | kmitm:m=4 |
|---|---:|---:|---:|---:|---:|---:|
| e2e_b20_a14_full | 20 | 14 | 4.0 ms | **2.4 ms** | **1.62×** | **3.49×** (1.1 ms) |
| e2e_b20_a64_full | 20 | 64 | 0.69 ms | **0.50 ms** | **1.40×** | 0.06× (prep-bound) |
| e2e_b24_a64_full | 24 | 64 | 8.3 ms | **4.6 ms** | **1.81×** | 0.63× |
| e2e_b24_a128_full | 24 | 128 | 3.8 ms | **2.3 ms** | **1.64×** | 0.02× (prep-bound) |
| e2e_b24_a256_breakthrough | 24 | 256 | 3.8 ms | **3.3 ms** | **1.16×** | — |
| e2e_b28_a64_full | 28 | 64 | 112 ms | **61 ms** | **1.85×** | **5.97×** (19 ms) |
| e2e_b28_a128_breakthrough | 28 | 128 | 54 ms | **28 ms** | **1.96×** | — |
| e2e_b28_a512_breakthrough | 28 | 512 | 21.5 ms | **16.4 ms** | **1.31×** | — |

Recipe (engineering): small `|Fₓ|` → prefer `kmitm:m=4` when prepare is cheap;
same-search / large `|Fₓ|` → `floor:m=3`; cold `kmitm:m≥6` is prepare-bound.

## Scoreboard — floor vs subtract oracle probe (m=3)

`probe` = subtract ns/target ÷ floor ns/target (higher is better for floor).

| bits | A=64 | A=128 | A=256 | A=512 | A=1024 |
|---:|---:|---:|---:|---:|---:|
| 20 | 1.86× | 1.40× | 0.90× | 0.95× | 0.98× |
| 24 | 1.98× | 2.03× | 1.74× | 1.09× | 1.00× |
| 28 | 1.92× | 2.05× | 2.12× | 2.04× | 1.35× |
| 32 | 1.93× | 2.02× | 2.13× | 2.22× | **2.25×** |

m=2 cells also in `floor_wall_matrix.txt` / `results.json` (typically ~1.6–2.0×
probe). Note: on 20-bit fields a few m=2 cells show small charged-add counter
drift while hits still match; recorded as `charged_adds_match=false`.

## Scoreboard — kmitm oracle wall (lanes + threads)

See `kmitm_wall_*.txt`. Highlights (probe ns/target; thr speedups vs thr1):

- Small A=14: full m=3..12 + lanes4 + thr1/2/4 (bits 20/24/28).
- A=64: m=3..6 measured; m≥8 skipped by OOM guard (left-table estimate).
- A=128/256: m=3..4 (and m=5 at A=128); higher m skipped.
- Lanes4 often **1.1–2.4×** vs scalar probe; thr4 up to ~**2×** when work is
  large enough (spawn overhead dominates tiny probes).

## Harness changes landed with this blitz

- `rootfind_floor_wall`: multi-bit / multi-m / prepare timing; soft charged-add check.
- `rootfind_kmitm_wall`: `--skip-ksum`, OOM guard, soft prepare refuse.
- `rootfind_pipeline_e2e`: soft-skip arms that refuse prepare at large `|Fₓ|`.

## Reproduce

```bash
export CARGO_TARGET_DIR=target-kmitm-fast
cargo +1.98.0 build --release \
  --example rootfind_floor_wall \
  --example rootfind_kmitm_wall \
  --example rootfind_pipeline_e2e
./research/rootfind_benchmark_blitz_20261121/run_matrix.sh
```
