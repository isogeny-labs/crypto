# `rootfind-floor` production oracle (2026-11-19)

**Decision: engineering.** Lands lever 1 of
`research/rootfind_baseline_100x_20261117/PLAN.md` as a new oracle id
beside `rootfind-subtract`. Same search, same charged group additions,
same witnesses; cheaper field work (x-only Montgomery candidates) and a
word-blocked Bloom filter in front of the exact abscissa map.

| | |
|---|---|
| oracle id | `rootfind-floor:m=2\|3` |
| code | `src/cryptanalysis/ic_framework/rootfind_floor.rs` |
| wall bench | `examples/rootfind_floor_wall.rs` |
| host | shared 4-vCPU VM; `[pcc:unverified]` |
| Conductor | unavailable in this environment (`conductor` not on PATH) |

## What changed

1. **Shared denominators** — already in `rootfind-subtract`; kept.
2. **Abscissa only, Montgomery-resident** — form `x(R ∓ F_i)` in
   Montgomery form; finish `y` only on a filter hit.
3. **Word-blocked Bloom filter** — 16 bits a key, five bits a word, in
   front of the exact map (miss-dominated workloads).

`rootfind-subtract` is unchanged (ecbench-extend: never change what a
registered method counts).

## Correctness

Unit tests
`floor_matches_subtract_verdicts_and_charged_adds_m2` and
`floor_matches_subtract_verdicts_m3` require identical verdicts, identical
witnesses, and identical charged `ops.adds` against `rootfind-subtract` on
synthetic prime curves. Zero mismatches.

## Wall time (32-bit prime, 2048 targets, m=2)

From `wall.txt` (`[pcc:unverified]`):

| A | subtract ns/pt | floor ns/pt | speedup |
|---|---:|---:|---:|
| 64 | 18.9 | 9.8 | **1.92×** |
| 256 | 17.3 | 8.4 | **2.07×** |
| 1024 | 17.0 | 8.0 | **2.12×** |

Charged additions matched in every cell. Hits matched (0/0/1). This is
inside the PLAN's measured 2.2–3.1× band against the best existing CPU arm
(prototype was 10–14 ns/pt; production oracle carries witness recovery and
ledger bookkeeping).

## What this does / does not claim

- **Does:** constant-factor wall and field-work improvement of the
  decomposition oracle; fail-closed parity with `rootfind-subtract`.
- **Does not:** change counted `S`; close the IC-vs-rho exponent gap; any
  deployed-curve or key-recovery claim. Multi-target amortization stays
  secondary.

## Reproduce

```bash
cargo test --release --lib rootfind_floor
cargo run --release --example rootfind_floor_wall -- \
  --bits 32 --sizes 64,256,1024 --targets 2048
```
