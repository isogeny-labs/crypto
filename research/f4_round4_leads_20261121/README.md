# F4 speed-up round 4: six leads, phase 1 (measurement and design), 2026-11-21

**Standing.** `[pcc:unverified]`. Six parallel read-only agents on one shared 4-vCPU VM. They took only contention-insensitive measurements (callgrind instruction counts, counted operations, fault and allocation counts), with no source edits and no wall-time claims. Full per-lead results: `phase1/leads.json`. Scripts and outputs: `phase1/L*/`.

## Verdicts

| lead | verdict | key measurement | estimated gain (estimate) |
|:--|:--|:--|:--|
| L1 per-shape elimination templates | **pursue** | Hilbert-mode F4 is 1.93 G instructions over 104 tests, of which 61.8 % (inclusive) is symbolic preprocessing. The top shape covers ≤ 85 %; about 8 shapes cover ~95 %. | 50–60 % fewer F4 instructions in Hilbert mode |
| L2 reducer-search index | pursue-small | The scan is 3.3 % of F4 instructions (ccov) and 0.7 % (jv_quintic). Divisibility masks do not beat today's one-word probe. | ~2–3 % of F4 (AVX2 4-wide probe) |
| L3 narrow small-prime arithmetic | pursue-small | 110 M instructions go to u64→u32 Barrett narrowing, and add_lazy has fixed per-call overhead. | ~11–14 % of F4 instructions |
| L4 large-system allocation | **drop** (code) | The faults come from allocators returning memory to the OS, not from large buffers. jemalloc `lg_extent_max_active_fit:10` with decay off brings jv_quintic faults from 1.37 M to 0.24 M, equal to glibc. jv_cover has ~830 faults per run. | none on jv_cover |
| L5 remaining per-term allocations | pursue-small | 408,630 unpack calls; byte-wise pack/unpack; Pair lcm Vec; no pre-sized emitted Poly. | ~5 % of F4 (cheap set); the full `Poly` change does not pay for 56 files |
| L6 zero reductions / early abort | pursue-small | 0/104 systems have 1 in the ideal, so an early "no root" abort gains ~0. A signature (F5-style) criterion's upper bound is 54.5 % of reduced rows. | F5-style criterion: 1.1–1.2× C_cov plausible, upper bound 1.33× |

## Findings outside the six leads

- **`--trace` is a regression by construction in Full mode.**
  - Cause: a replay never consumes pairs. After the recorded steps, the run continues as a full run over all accumulated pairs, so pair rows rise 66 % (f4_fp.rs ~5074–5101, 5228). This explains the pilot (11.0 → 14.5 s; +25 % counted ops).
  - `--trace --stop hilbert` silently runs Full mode: `f4_record` and `f4_replay` bypass `f4_hilbert_guarded` (f4_fp.rs:1378–1408, 4425–4429).
- **The Hilbert stop rule is not the default for `jv_cover --exp dlp`/`ccov`.**
  - It was validated in `benchmarks/20261029-jv-cover-hilbert-stop`: S 1.85× lower at p = 53–503, identical relations.
  - Phase 1 measured it at 2.42× on F4 counted ops and 1.82× on C_cov.
