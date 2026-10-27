# Adversarial review 2 of the replication instruments (2026-10-27)

Verbatim fresh-reviewer output after the round-1 fixes.

**Verdict: not ready.** Issues 3, 6 and 7 are fixed. Issues 1, 2 and 4 are only partly fixed, and issue 5 is mostly fixed. Three new problems bias the IC-vs-rho comparison at the crossover, and two of them work against IC at m = 14–16.

## Per-issue status

| # | Status | Evidence |
|---|---|---|
| 1 GAE divisor depends on naive F_{q⁴} code | **Partly fixed** | `RefCosts` (`gaudry_quartic_bin.rs:253-261`) and its tally are sound for odd m: inv 22 = norm 9 + F_{q²} inverse 7 + 6, frob 3 holds because w^q = w + γ with γ in F_{q²}, and outermost-only charging is tested. Two gaps remain: the 50 is not achievable at even m (new issue 3), and the Weil-restriction F_{q⁴} work stays in the old schoolbook unit (new issue 1). No m = 14–17 runs. |
| 2 Slow u128 mulmod, 19× step at m = 17 | **Partly fixed; the fix adds its own bias** | Montgomery pricing is applied at every m, but it is measured on a different host and with a different kernel from the committed table. At m = 14–16 it is about 3× more expensive than the committed code price, while at m = 17 it is 7× cheaper (new issue 2). |
| 3 Band adds operations of different sizes | **Fixed** | `band_rule_applications` (`f2m4_price.rs:529`): one IC `walk_steps` counts as 4 applications, the same as rho's split counters. The block has per-counter output and the `legacy_ic` caveat. `check_full_block` asserts sums, the 0 point and the weighted point. Not yet run on real sessions. |
| 4 Unit-price IC readout wrong, never reaches relations | **Partly fixed** | The readout keys are fixed. The claim spec (oracle, linalg, cofactor) is matched in `unit_price.rs`. `--ic-complete` and `--rho-full` exist, and the closure runs reached relations at m = 10–12 (251 / 491 / 1028). Still open: the committed `research/.../unit-price/unit-price-f2m4-m14-17.json` keeps the bad `legacy_decompositions` / `trials` fields (lines 205, 660, 1115, 1570), and the "zero decompositions" statement is not withdrawn in any record. The new readout has no test. |
| 5 Full pricing may leave IC work unpriced | **Mostly fixed** | `check_full_block` asserts `rep.unpriced.is_empty()` (`methods.rs` ~3930), and `--closure` treats any unpriced entry as an error. Measured user CPU over priced time, IC against batched rho: 1.61 vs 1.38, 1.66 vs 1.59, 1.66 vs 1.39 at m = 10 / 11 / 12. The rayon scheduler overhead was found and removed. Caveats: one rep, units priced on a different host from the table, and closure was checked only for the `*_full` (current-unit) ids, not the `*_ref` / `*_refmm` ids that will decide the comparison. |
| 6 u128 `rho.negation` counts not shown unchanged | **Fixed for the scope asked** | The capped diff (caps up to 1e8 steps, two targets, every counter plus float bit patterns) is byte-identical between `e403626cb` and HEAD. The seq 0/1 totals were not reproduced. The batched rho and the u128 IC path are not covered. |
| 7 Lint and format failures | **Fixed** | No clippy findings on lines the branch added, and rustfmt is clean on all 8 changed files. The remaining clippy errors predate the branch. Because the lib fails clippy, `src/bin/ecbench.rs` was never linted at the bin level. |

## New blocking issues

**1. The Weil-restriction F_{q⁴} work is still in the schoolbook unit but divided by the reference divisor. This biases against IC.**
- Where:
  - `gaudry_quartic_bin.rs:1066` (`SymmetrisedBin::at`, 8 + 9·|monos| F_{q⁴} products per solve)
  - `:2480`, `:2506`, `:2528` (`weil_muls = f.f.muls() - before`, schoolbook count)
  - `:3491` (`precompute_muls`) and `:3563` (`total_muls`) feed `solver_ops`
  - `methods.rs:2862` uses `solver_fq = solver_ops − lp_estimate`, divided by 50.
- These products cost about 16–25 counted F_q multiplications each in the current code, against 11 in the reference, or 0 for the many zero coefficients. Yet the reference IC price divides them by the reference divisor of 50.
- The IC record's "F_{q⁴} share" (`methods.rs:2889`) counts only lifts and `x_of_t`, so the reported share under 0.0001 understates IC's F_{q⁴} exposure.
- Bound from the claim's e-m15 records: echelon + normal form + charpoly are 254e9 of 384e9 `ops`. The other 34% includes `weil_muls` and is not broken down anywhere, so the overcharge could be several percent of IC's total.
- Fix:
  - Bracket the system build with `f.ref_muls()` and record `weil_ref_muls` and `precompute_ref_muls` beside the current counts.
  - In `ic.pipeline_f2m4_ref*`, substitute them for `weil_muls` and `precompute_muls` before dividing by 50.
  - Add both to the F_{q⁴} share.
  - Emit `weil_muls` per record so the share is visible.

**2. The `*_refmm` mulmod units mix two hosts and two kernels. They raise IC's price at m = 14–16 and cut it at m = 17, which flattens IC's slope exactly across the crossover.**
- Where:
  - `f2m4_price.rs:1555-1579`, the `row_time` kernel. It uses `Vec<u128>` rows with u128 subtract and select even when `narrow`.
  - `f2m4_price.rs:170` (`row_mm`) applies Montgomery at every m.
  - `f2m4_mulmod_prices.json` against `f2m4_full_prices.json`.
- `row_op` in FME:

  | m | Committed code price | New Montgomery price | Effect on IC |
  |---|---|---|---|
  | 14 | 1.87 | 5.15 | about 3× more expensive |
  | 15 | 1.43 | 4.55 | about 3× more expensive |
  | 16 | 1.36 | 3.75 | about 3× more expensive |
  | 17 | 25.9 | 3.69 | 7× cheaper |

  `lp_mulmod` moves the same way below m = 17: 9.53 → 4.19 at m = 14, 6.9 → 3.06 at m = 16. The one-word u64 path is cheaper than this Montgomery kernel on the committed host.
- The same-run "code" kernel does not reproduce the committed `row_op` either:
  - at m = 9–12 it reads 15.6–16.5 FME, against 2.4 committed for the same one-word path;
  - in ns at m = 12 that is about 30 ns, against 3.2 ns.

  So "the step is gone" compares different microbenchmarks, not just different hosts.
- The F_q-multiplication time behind the FME unit also differs: 4.11 ns against 2.9 ns at m = 17.
- Fix:
  - Re-measure the whole table, all units including the Montgomery ones, in one interleaved pass on one isolated host with warm-up.
  - Use u64 storage and a one-limb Montgomery for narrow r.
  - Price each mod-r unit as min(code, Montgomery) from the same pass.
  - Add a test that the `code_row` kernel reproduces the committed `row_op` kernel within a tolerance on the same host.

**3. The m-independent reference divisor of 50 can't be reached at even m in the stated model. This biases against IC at m = 14 and 16, a zigzag next to m = 17.**
- Where: `gaudry_quartic_bin.rs:234-261`.
- The model's 3-multiplication F_{q²} product with free reduction needs u² + u + 1, which is irreducible over F_{2^m} only when m is odd.
- At m = 14 and 16, ε ≠ 1 and every F_{q²} product, square and norm pays for the ε multiple. In a log-table F_q that multiple is a full multiplication.
- My recount at even m: mul about 15, sq about 9, inv about 28, so one addition is about 67 rather than 50.
- Rho in `rho.negation_f2m4_ref*` is charged 1 GAE per group operation whatever the divisor, so IC's GAE at even m is inflated by about 1.3× relative to odd m.
- Fix: either
  - state the model as polynomial-basis F_q, where multiplying by a constant is free, and justify it; or
  - make `RefCosts` depend on m parity, or measure each m's actual tower, and report the divisor per m.

  Then check whether the crossover moves.

## Non-blocking

- **Rho strength.** In the reference unit, batched rho costs 3 mul + (2 mul + sq) + inv/n, about 61 per step, against 50 for an affine addition. So `rho.negation_f2m4_ref*` is the strongest rho, and any IC/rho ratio must use the cheaper rho arm at each m.
- **The FME unit itself drifts with m.** The committed F_q-multiplication time rises from 1.27 ns to 3.82 ns over m = 6–18, as the log tables leave L1/L2. Native units (row_op 2.45 → 1.36) get cheaper in FME as m grows, and IC carries more native work than rho, so this drift favours IC toward m = 17. State it, or give native units in ns alongside FME.
- **u128 coverage.** The u128 IC path (`solve_f2m4_wide_ic`) and the `*_ref` / `*_refmm` ids are tested only at m = 6–7. No id that decides the comparison has run at m = 14–17.
- **Records.** The issue 2–6 work has no `benchmarks/` row or research-visuals PDF yet; CLAUDE.md requires both.

Files referenced:
- `/home/user/crypto/src/cryptanalysis/gaudry_quartic_bin.rs`
- `/home/user/crypto/src/cryptanalysis/ecbench/methods.rs`
- `/home/user/crypto/src/cryptanalysis/ecbench/f2m4_price.rs`
- `/home/user/crypto/src/cryptanalysis/ecbench/f2m4_mulmod_prices.json`
- `/home/user/crypto/src/cryptanalysis/ecbench/f2m4_full_prices.json`
- `/home/user/crypto/research/gaudry_k4_char2_replication_20261026/unit-price/unit-price-f2m4-m14-17.json`