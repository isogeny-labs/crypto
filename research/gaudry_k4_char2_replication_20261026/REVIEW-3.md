# Adversarial review 3 of the replication instruments (2026-10-27)

Verbatim fresh-reviewer output after the round-2 and round-3 fixes. Threshold for blocking: can move IC/rho by more than about 10% at some m in 14-17.

**Verdict: not ready.** The three blockers from review 2 are mostly closed: the Weil-restriction work is fixed, and the mod-r units are fixed for the `*_refmm` ids. But the parity-aware divisor overcharges odd m. That is new issue 1 below, and it can move IC/rho by about 19% at m = 15 and 17, in IC's favour. No id that decides the comparison has run at m = 14–17.

## Per-blocker status

| Review / # | Status | Evidence |
|---|---|---|
| R1-1: GAE divisor depends on naive F_{q⁴} code | **Mostly fixed** | The divisor no longer depends on which quartic was picked: `RefCosts::new` (`gaudry_quartic_bin.rs:305-321`) gives 50 at odd m and 64 at even m. The odd-m figure is wrong (new issue 1). |
| R1-2: slow u128 mulmod, 19× step at m = 17 | **Fixed for `*_refmm`, not fixed for `*_ref`** | `*_refmm` uses `Mulmod::Onepass` (`methods.rs:1127,1133`). `*_ref` still uses `Mulmod::Code` on `f2m4_full_prices.json` (`methods.rs:1125,1131`), so `row_op` at m17_u128 is still 25.9 FME (new issue 3). |
| R1-3: band adds operations of different sizes | **Fixed** | As review 2 found; nothing on the branch changes it. |
| R1-4: IC readout wrong, never reaches relations | **Mostly fixed** | The readout is fixed and tested (`unit_price.rs`, 3 tests). `WITHDRAWN.md` withdraws the bad fields and the "zero decompositions" statement. The m = 14–17 re-run reaches relations (2000 trials, 10–12 relations). Still open: in the re-run, `in_situ.rho_full` is `null` and `ns_per_fq_mul_local`/`_table` are `null` at every m. So at m = 14–17 rho is still reported per counted op and IC per full-priced FME, the unit mismatch R1-4 asked to remove. No complete solve at m ≥ 14. |
| R1-5: full pricing may leave IC work unpriced | **Mostly fixed** | `unpriced.is_empty()` is asserted. Closure was run for `*_ref`/`*_refmm` at m = 10–12, one rep: IC 2.0–2.8, rho 2.5–3.2. In `unit_price.rs:567-579`, the `ref` closure divides by the old host's ns per F_q multiplication even though it ran on the new host, so its absolute ratios mean little. The IC-against-rho ordering is still informative. |
| R1-6: u128 `rho.negation` counts unchanged | **Fixed for the scope asked** | Unchanged since review 2. |
| R1-7: lint and format failures | **Regressed (non-blocking for the ratio)** | Two new clippy errors on added lines: `methods.rs:2657` (`c.f.f.m % 2 == 0`) and `gaudry_quartic_bin.rs:307` (`m % 2 == 0`). Both want `.is_multiple_of(2)`. Because the lib fails clippy, the bins were never linted. |
| R2-1: Weil-restriction F_{q⁴} work in the schoolbook unit | **Fixed** | `weil_ref_muls` and `precompute_ref_muls` bracket the same work as the counted figures (`gaudry_quartic_bin.rs` `add_weil`, `precompute`). `reprice_ic` swaps them in before dividing (`f2m4_price.rs:699-705`). The wiring is at `methods.rs:2905-2956`, and the F_{q⁴} share now includes them (`methods.rs:2981-2985`). Every IC record emits the counts. Verified only at m = 6–7; the wide path uses the same `BinD4Oracle`, so it should carry the fields, but no test runs it. |
| R2-2: `*_refmm` mulmod units mix hosts and kernels | **Fixed for `*_refmm`** | One interleaved pass with flanking F_q-multiplication timings (`f2m4_price.rs:1895-1950`). u64 one-limb Montgomery for narrow r, min(code, Montgomery) per unit, and a test that the code kernel reproduces the production `row_op`. The min `row_op` is smooth (1.25 / 1.22 / 0.88 / 1.74 at m = 14–17). Residuals are in the notes (not isolated, bimodal F_q timing, `*_ref` left on the old table). |
| R2-3: reference divisor of 50 not reachable at even m | **Partly fixed; the fix overcorrects odd m** | Even m = 64 matches a recount. Odd m stays at 50 although the same log-table model gives 42 (new issue 1). |

## New blocking issues

**1. At odd m the reference divisor is 50, but the model's own tower gives 42. IC is underpriced by up to about 19% at m = 15 and 17.**
- Where: `gaudry_quartic_bin.rs:310` (`let c = u64::from(model == RefModel::Tower)`, which charges the δ-multiple 2 at every m), and the doc at `:283`.
- Why 42:
  - The builder notes that δ ∈ F_q never makes w²+w+δ irreducible over F_{q²}, so δ has to lie in F_{q²}.
  - At odd m, ε = 1 and Tr_{F_{q²}/F_2}(u) = Tr_{F_q/F_2}(1) = 1, so δ = u works.
  - Multiplying by u is then free: (a₀+a₁u)·u = a₁ + (a₀+a₁)u.
  - That gives mul 9, sq 4, inv 7 + 7 + 6 = 20, so an addition costs 20 + 18 + 4 = **42**, the same as `PolyFreeConst`.
  - Frobenius is also free, since w^q = w + u, but an addition does not use it.
  - At even m, δ = μu with Tr(μ) = 1 forces μ ≠ 1, and the multiple costs 2 (μ and με precomputed). So 64 stands.
- Effect on IC/rho: `rho.negation_f2m4_*` charges one GAE per group operation (`methods.rs:2840`), while IC's whole F_q cost is divided by the divisor. IC/rho at m = 15 and 17 is too low by up to 50/42 ≈ 1.19, less the rho native share. The even/odd zigzag becomes 42/64 ≈ 1.52, not 1.28, right next to the crossover.
- Fixes:
  - Set the δ charge to 0 at odd m (choose δ = u, frob 0) and update `reference_costs_by_parity` to expect (9, 4, 4, 0, 20, 16) and 42 at odd m.
  - State in the decision rule that a crossover claim must hold under both the tower model (42 / 64) and `PolyFreeConst` (42 at every m), or use the lower divisor, which is conservative for "IC beats rho".

**2. The only m = 14–17 in-situ evidence shows IC's CPU per priced unit far above rho's at m = 14–15, with the gap closing at m = 17. It is unexplained and is the shape that would flatten IC's slope.**
- Where: `research/.../unit-price/unit-price-f2m4-m14-17-rerun.json`.
  - IC `ns_per_full_priced_fme`: 11.2 / 11.1 / 12.0 / 16.2 at m = 14 / 15 / 16 / 17.
  - Rho `ns_per_counted_fq_op`: 4.4 / 4.4–4.8 / 8.2 / 13.6.
- Rho's full-priced FME is at least its counted operations, so rho's ns per full FME is at most these figures. IC's in-situ cost per priced unit is therefore at least 2.5× rho's at m = 14–15, at least 1.45× at m = 16 and at least 1.2× at m = 17. At m = 10–12 on complete solves the gap was about 1.17×.
- IC's sys CPU is also large: 2.7 s of 12.9 s wall at m = 14 with one thread, and 4m52s of 25m in the killed complete solve.
- Either some IC phase is priced too low at m ≥ 14 (set-up, failed decompositions, allocation), or the partial run is unrepresentative. The builder asserts the second; nothing shows it. This could move IC/rho by more than 10% at m = 14–15 relative to m = 17.
- Fix: re-run with `rho_full` populated and `ns_per_fq_mul_local` recorded. Add per-phase CPU timers to the IC partial run (precompute and factor base, system build, solve, linear algebra) and compare each against its priced share. Do this at m = 14 and 17.

**3. `*_ref` still carries the R1-2 u128 step at m = 17, and no decision rule says which id variant decides.**
- Where: `methods.rs:1125,1131` and the `ic.pipeline_f2m4_ref` branch use `Mulmod::Code` on `f2m4_full_prices.json`, where m17_u128 `row_op` is 25.9 FME against 1.36 at m = 16. The `*_refmm` and `*_ref` totals at m = 10 already differ by 20% for IC and 12% for rho (`closure/ref-m10.json`).
- Effect: if anyone reads `*_ref` at m = 17, IC is overcharged by an amount comparable to its linear-algebra share. Review 1 put that at several× on `row_op`.
- Fix (one line, as the builder offered): move `*_ref` onto `f2m4_onepass_prices.json` with `OnepassCode`, or freeze a rule that only `*_refmm` (and its poly-model block) decides, and label `*_ref` "not decision-grade" in its summary.

## Non-blocking notes

- **Noisy, non-isolated one-pass table.**
  - The table host has `in_isolcpus=false`, and the F_q-multiplication timing is bimodal (about 2.2 against 4–5 ns). At m = 16 the median, 3.09, lands on the edge between the two modes (8 fast samples, 6 slow).
  - Every native unit's FME scales with that median. With native shares of roughly 10–30% in both arms, the swing on IC/rho is probably under 10%.
  - Re-measure on isolab before final numbers.
- **The FME unit itself drifts with m.** The F_q-multiplication median goes 2.28 → 2.45 → 3.09 → 4.85 ns over m = 14–17 as the log tables leave L2, so native work gets about 2× cheaper in FME at m = 17. That favours whichever arm carries more native work (IC, per the `ref`/`refmm` deltas). Report native units in ns beside FME, or under a cache-independent F_q.
- **Reference closure ratios on the wrong denominator.** `ref` closure divides by the old host's F_q time (`unit_price.rs:567-579`), so its 2–3× undercount is partly host drift. Report `user_over_priced` (the local figure) as the headline.
- **The R2-1 bound is unchecked.** The "e-m15 34% other" bound review 2 cited has not been rechecked with the new `weil_fq_muls` fields at m = 14–17.
- **Missing records.** There is no `benchmarks/` row or research-visuals PDF, as CLAUDE.md requires.
- **Frobenius charge at odd m.** In the tower model Frobenius is 3 at odd m but achievable at 0. It only affects IC's F_{q⁴} share (about 2%).
- **Lint.** Fix the two `is_multiple_of` clippy errors and lint the bins once the pre-existing lib errors are cleared or allowed.

Files referenced:
- /home/user/crypto/src/cryptanalysis/gaudry_quartic_bin.rs
- /home/user/crypto/src/cryptanalysis/ecbench/methods.rs
- /home/user/crypto/src/cryptanalysis/ecbench/f2m4_price.rs
- /home/user/crypto/src/cryptanalysis/ecbench/unit_price.rs
- /home/user/crypto/src/cryptanalysis/ecbench/f2m4_onepass_prices.json
- /home/user/crypto/research/gaudry_k4_char2_replication_20261026/unit-price/unit-price-f2m4-m14-17-rerun.json
- /home/user/crypto/research/gaudry_k4_char2_replication_20261026/closure/ref-m10.json