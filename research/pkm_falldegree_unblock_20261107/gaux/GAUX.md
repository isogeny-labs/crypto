# Gate G-AUX (Amendment R4 draft): result, 2026-11-14

Every number is `[pcc:unverified]` and measured from the files in this directory unless marked.

## Verdict: pass, on the third attempt. The first two failures are kept, with corrected diagnoses.

| check | result |
|:--|:--|
| 1. `mestre_exact_order_equals_point_count` (772 curves, p in {233, 1009, 40961, 65519, 786433}) | pass (main + patch, glibc) |
| 2. `mestre_and_naive_build_the_same_isogeny_tower` (P0, t = 1..5, seeds 0..2) | pass |
| 3. Row-level positive control: 16 P0 isogeny draws, t = 1..8, against `pfdA_summary.json` | **pass in `v3/`** (16 of 16 equal on `x_r`, `closure_dF`, `falls`, `ffd`, `closure_muladds`, `closure_max_cols`, `D`, `f4_muladds`) |

The full example test suite (17 tests) passes on main + patch.

## The attempts

1. `./` (main HEAD `31d51c6d8e`, glibc, 4 threads): **FAIL**, 6 of 16 draws. Only `f4_muladds` differs, at t = 6, 7, 8. The
   commit `007bdf84f3` blamed the F4 engine change on main (`3ab8291287`). **That diagnosis was wrong.**
2. `v2/` (frozen f13a690df0 + `aux_order.patch`, musl, 4 threads): **FAIL**, the same 6 draws with the same values. That
   rules out the engine change.
3. Diagnosis (`diag/`), with the **frozen binary**, bit-identical to the image (`runpod/REBUILD.md`), `--aux-order` not used,
   t = 6..7:
   - `RAYON_NUM_THREADS=1`: 54,927,560 at t = 6, draw 0
   - `RAYON_NUM_THREADS=4`: 55,512,274
   - `RAYON_NUM_THREADS=12`: 55,685,928, which equals the admitted value
   
   **F4's `muladds` counter depends on the rayon thread count** (by about 1%; `solving_degree_max` does not change). The
   admitted rows ran at 12 threads (Amendment 9). The closure engine's counters did not change with threads here.
4. `v3/` (frozen + patch, musl, `RAYON_NUM_THREADS=12`): **pass**, 16 of 16.

## Consequences

- The isogeny `P1` jobs must run at `RAYON_NUM_THREADS=12` (as `entry_r2.sh` sets by default). Otherwise their `f4_muladds`
  are not comparable with the Kummer rows.
- The `f4_muladds` of this study are a thread-count-dependent measure. Any comparison of F4 work across rows needs equal thread
  counts. This is recorded here and not acted on. It is an instrument property for the owner to note.
- The binary for the isogeny `P1` arm is `v3/`'s: `pkm_tower_pilot` built from f13a690df016d0a457bd40579fbb9d054f486adf plus
  `aux_order.patch`, musl static, SHA-256 in `v2/binary.sha256` (705b151b…). The other three study binaries are the frozen ones.
