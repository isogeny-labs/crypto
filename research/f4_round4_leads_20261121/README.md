# F4 speed-up round 4: six leads, 2026-11-21

**Standing.** `[pcc:unverified]`. One shared 4-vCPU VM, isolation L0, no PMU. PR #4859.

## Result

Pre-registered (`CONFIRM_PROTOCOL.md`, frozen at `b5795d4061`), on `jv_cover --exp dlp`, p = 53:

- `--stop hilbert:16 --trace` on this branch against `--stop hilbert:16` before it:
  - wall **1.231×** (9.97 → 8.14 s, 10/10 rounds) and CPU 1.235×;
  - counted S about 1.32× (9.62 G → 7.24–7.30 G oracle multiplications);
  - 31 relations and a correct logarithm in every run.
- Full F4: 1.060× median, 8/10 rounds. That fails the pre-registered 9/10 sign rule, so it is not significant.
- Counted operations under `--trace` with 4 threads **vary with scheduling** (0.9 %). The protocol predicted them deterministic; that prediction failed.

Report: `reports/run-20261121T0500Z/` (REPORT.pdf, confirm.svg, instructions.svg). Record: `benchmarks/records/20261121T0437Z-jv-cover-f4-round4-confirm.json`.

## Phase 1: the six leads, measured and designed in parallel

| lead | verdict | key measurement | estimated gain (estimate) |
|:--|:--|:--|:--|
| L1 per-shape elimination templates | **pursue** | Hilbert-mode F4 is 1.93 G instructions over 104 tests, of which 61.8 % (inclusive) is symbolic preprocessing. The top shape covers ≤ 85 %; about 8 shapes cover ~95 %. | 50–60 % fewer F4 instructions in Hilbert mode |
| L2 reducer-search index | pursue-small | The scan is 3.3 % of F4 instructions (ccov) and 0.7 % (jv_quintic). Divisibility masks do not beat today's one-word probe. | ~2–3 % of F4 (AVX2 4-wide probe) |
| L3 narrow small-prime arithmetic | pursue-small | 110 M instructions go to u64→u32 Barrett narrowing, and add_lazy has fixed per-call overhead. | ~11–14 % of F4 instructions |
| L4 large-system allocation | **drop** (code) | The faults come from allocators returning memory to the OS, not from large buffers. jemalloc `lg_extent_max_active_fit:10` with decay off brings jv_quintic faults from 1.37 M to 0.24 M, equal to glibc. jv_cover has ~830 faults per run. | none on jv_cover |
| L5 remaining per-term allocations | pursue-small | 408,630 unpack calls; byte-wise pack/unpack; Pair lcm Vec; no pre-sized emitted Poly. | ~5 % of F4 (cheap set); the full `Poly` change does not pay for 56 files |
| L6 zero reductions / early abort | pursue-small | 0/104 systems have 1 in the ideal, so an early "no root" abort gains ~0. A signature (F5-style) criterion's upper bound is 54.5 % of reduced rows. | F5-style criterion: 1.1–1.2× C_cov plausible, upper bound 1.33× |

Per-lead details: `phase1/leads.json`, `phase1/L*/`.

## Phase 2: what was built (callgrind F4 instructions, 104 Nagao tests)

| step | change | effect |
|:--|:--|:--|
| 1 | trace through `f4_hilbert_guarded`; `jv_cover` refuses `--trace` without a stop rule; recording runs charged | Hilbert 1.096 G → Hilbert+trace 0.979 G; counted S 392.3 → 298.1 |
| 3 | u32 accumulators (p ≤ 4097), byte pack/unpack, u64 divisor scan, pre-size (≈ 0) | full 1.138 → 0.998 G; Hilbert 1.096 → 0.966 G; output and counted ops byte-identical |
| 2 | frozen replay steps, a superset template widened by support union and rebuilt by integer reachability (`F4_FP_FREEZE=0` turns it off) | 0.859 → 0.774 G (−9.8 %); identical output and counted ops |
| 4 | signature criterion | deferred: branch traces target the diverged replays (about 47 % of counted F4, an estimate) more directly |

Lead 4 had no code change. jemalloc stays per consumer (round 3).

## Open

- Branch traces: record the continuation at a replay divergence.
- Deterministic trace recording under parallel batches.
- Hilbert as `jv_cover`'s default: an owner decision, not made.
