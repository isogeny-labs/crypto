# Adversarial review 1 of the replication instruments (2026-10-27)

Verbatim review by a workflow agent of `git diff e403626cb..b0547c582`.

**Verdict: not ready.** The instruments mostly work, and counts are pinned where tests reach. But the shared unit has a pricing artifact that tilts toward IC, by a different amount at each m, right at the m≈17 crossover. On top of that, the in-situ check behind the unit price doesn't measure IC's real work, and the §5a band isn't comparable between the two arms.

## Blocking issues

1. **The GAE divisor depends on how naively F_{q⁴} is coded, and it shifts with m.**
   - Where: `src/cryptanalysis/gaudry_quartic_bin.rs:300` (`reduce`), `:383` (`inv`), `:373` (`frob`), `:406` (`sqrt`), `:2628` (`fq_muls_per_add`).
   - F_{q⁴} multiplication is schoolbook: 16 products plus up to 9 reduction products. Products by the constant c₃ = 1 are counted too.
   - Frobenius costs 12 counted multiplications. Inversion is 3 Frobenius maps plus 3 F_{q⁴} multiplications, about 116 operations.
   - That is how one affine addition reaches 159 to 179 F_q operations. A Karatsuba/tower implementation would need about 50 to 60.
   - The arms are not affected alike:
     - Rho in GAE is about 1 per step whatever the arithmetic, but rho in F_q operations is roughly 3× worse than a competent rho.
     - IC's solver works in F_q, so IC's GAE = its F_q operations ÷ 179 is deflated by the same factor.
     - IC's own F_{q⁴} work (lifts, `x_of_t`, `sqrt` by 4m−1 counted squarings) is inflated the other way.
   - The divisor also jumps 179 / 159 / 179 / 161 at m = 14 / 15 / 16 / 17. That comes from which irreducible quartic `Fq4::new` picks (c₃ = 1 is tried first when m > 15), not from the problem.
   - Fix, without changing existing counts:
     - Add a reference-cost pricing variant: Karatsuba F_{q⁴} multiplication, linear squaring, Frobenius and square root, norm-tower inversion, and no charge for multiplying by 0 or 1. Use it for the divisor, rho's walk F_q operations and IC's lift and square-root F_q operations. Make the divisor m-independent.
     - Report the current and reference divisors side by side, plus IC's F_{q⁴} share, so the bias in each direction is visible.

2. **IC's u128 linear algebra is priced from a slow mulmod, a 19× step exactly at m = 17.**
   - Where: `src/cryptanalysis/koblitz_strong_rho.rs:262` (`mul_mod_u128`), measured at `src/cryptanalysis/ecbench/f2m4_price.rs:692`.
   - The unit measures `mul_mod_u128`, which does chunked u128 `%` plus extra `a % m, b % m`.
   - `row_op` costs 1.36 FME at m=16 and 25.9 FME at m=17 (`m17_u128`). The large-prime units (`lp_mulmod`) and rho's scalar units take the same u128 path.
   - At r ≈ 2^67 this is an implementation artifact that lands exactly at the crossover and inflates IC.
   - Fix: price the row and large-prime units at m ≥ 17 from a Barrett or Montgomery 2-limb mulmod (a microbench is enough). Or report the band for each unit with `row_op` at the u64 rate × 2 to 3 alongside the measured rate.

3. **The §5a band adds up "operations" of very different sizes, and the split differs between arms.**
   - Where: `f2m4_price.rs:430` (`band`) and the rules at `:166-:310`.
   - `native_operations` is the plain sum of counts:
     - One IC `walk_steps` count bundles a key evaluation, a mix and 2 operations mod r, but counts as 1.
     - Unbatched rho counts its key, mix, negation, mod-r, window-element and push work separately, about 20 to 30 per step.
     - An `elem_scan` (0.12 FME) and a `dp_insert` (109 FME) each count as 1.
   - So the 0.1 and 1 GAE-per-operation points measure how finely each arm's counters are split, not unpriced work. The 1 point inflates rho per step and IC per Macaulay element.
   - §5a also meant "per *unpriced* op". The legacy `ic.pipeline` records never counted their unpriced work, so the band can't be computed for those records at all.
   - Fix:
     - Define one unit per rule application (or weight each count by its unit), the same on both arms.
     - Emit the band per counter so a reader can recompute it.
     - Say in the records and docs that legacy IC has no band.

4. **The unit-price IC in-situ readout is wrong, and the IC run never reaches relations.**
   - Where: `src/cryptanalysis/ecbench/unit_price.rs:344`, `:351-352`.
   - The code reads `trials` / `walk_trials` / `decompositions` / `accepted_relations`, but `ic_report` only emits `targets_tried`, `relations_found` and `matrix_*` (`methods.rs` ~1823).
   - So `legacy_trials`, `full_trials` and `legacy_decompositions` are always 0. The builder's statement that "IC found zero decompositions" is an artifact of this lookup and should be withdrawn.
   - With `--ic-trials 8`, the run is almost all set-up. The "IC ns/op" therefore doesn't measure the relation search, where IC's cost actually is.
   - It also uses `linalg=wiedemann` and `charge_unsolved=1` (`:309-312`), and builds the curve with `cof = 2 if m ≥ 15 else 16` (`:377`). Whether that matches the claim's spec and curves is unverified.
   - Fix:
     - Read `targets_tried` and `relations_found`.
     - Take the oracle and linalg parameters from the claim spec.
     - Run IC to complete solves (or at least well into relations) at m=10 to 12.
     - Measure rho's `_full` arms too, so both arms report ns per full-priced FME. Right now rho reports ns per counted op and IC reports ns per FME, which can't be compared.

5. **Full pricing hasn't been shown to leave no lower bound on the IC side.**
   - The builder's own smoke run at m=10: IC used about 184 s of user CPU (plus 238 s sys), against about 40 s priced. Rho's measured time is also above its price.
   - Even after setting aside rayon wake-ups, user CPU is about 4.6× the priced cost. That points to IC work that is still unpriced or underpriced, which favours IC.
   - Also, `reprice_ic` keeps the `solver_*` and `counts_lost*` unpriced entries (`f2m4_price.rs` ~553), and no test asserts that IC's `unpriced` is empty. The rho tests do.
   - Fix:
     - Add `assert!(full.unpriced.is_empty())` to `ic_pipeline_f2m4_full_prices_every_native_counter` (`methods.rs:3704`).
     - Do a one-thread closure check (`RAYON_NUM_THREADS=1`, complete solves at m=10 to 12, both arms): priced FME × ns/FME against measured CPU within a stated tolerance, before any crossover is read.

6. **It is not yet shown that existing counts are unchanged for u128 `rho.negation`.**
   - `ic_wide::rho_negation` was modified (tally, rustfmt). It is pinned step for step only at m=6..10.
   - The m=17 replay (PID 10993, `/tmp/claude-0/replay.log`) has produced no output yet.
   - Fix: finish that replay. Or, more cheaply, run base `e403626cb` and HEAD binaries on the m=17 curve with a capped step budget and diff every counter. Compare against seq 0/1 in the brief's table: steps 15,986,242,844 / 11,722,773,009, canonicalisations 16,538,445,888 / 12,120,249,275.

7. **Lint and format failures introduced by this branch.**
   - `src/cryptanalysis/ecbench/mod.rs:46-47`: `f2m4_price` / `unit_price` are out of order for rustfmt. Move `unit_price` after `stats`.
   - `methods.rs:2592`: the `as u32` cast is redundant; use `c.f.f.q`.
   - `f2m4_price.rs:861`: `|i| mix(i)` should be `mix`.
   - The `gaudry_quartic_bin.rs` 2319, 2553 and 2786 errors predate the branch (blame `5a5828371`, `e6375d605`), as do the `curve_id`, `workload` and `gaudry_quartic_d4` ones. Fixing those is optional.

## Non-blocking notes

- **Rho is otherwise a legitimate strongest rho.**
  - b ∉ F_{q²}, so negation is the only cheap automorphism.
  - Look-ahead, cycle escape and DP handling match `rho.negation`.
  - Batched and unbatched rho are counted in the same F_q unit with the same divisor.
  - Pricing inversion at 1 multiplication is fair for log-table F_q (measured independent inv 4.5 ns vs mul 3.6 ns).
  - Once issue 1 is fixed, check whether batching still helps at all. With cheap inversion it may not.
- **The FME reference is a throughput axpy element** (2.9 ns at m=17). Rho's counted F_q operations run at 9 to 11 ns in situ. The assumption "counted op = 1 FME" is only safe if both arms' counted operations have the same in-situ ratio. Issue 4's measurement should test that.
- **`f2m4_full_prices.json` shows host drift.** Every ratio at m=8 and m=13..15 sits 20 to 40% below its neighbours, consistent with the reference denominator moving. Because IC's native share (about 2.1× counted) is larger than rho's, the drift doesn't cancel.
  - `ns_per` has no warm-up, and `fq_mul` is measured first in each pass.
  - Fix: re-measure interleaved on an isolated host (isolab L2) with a warm-up pass.
- **`lp_estimate` fails silently.** `methods.rs:2587` uses `.unwrap_or(0)`. If the JSON path ever changes, the legacy large-prime estimate is silently double-counted against IC. Make it an error when `large_primes > 0`, and assert `large_prime_estimate_replaced > 0` in the lp test.
- **`GfTab::invs` is an `AtomicU64` bumped in the solver's parallel hot loops.** Counts are unchanged, but legacy `ic.pipeline` wall time (and the in-situ ns/op) now carry contention the claim snapshot didn't have. A per-thread tally merged at the end would avoid it.
- **`par_region` is priced at one-thread dispatch** while the evidence solves may run multi-threaded. Record the rayon thread count in each full record.
- **The u128 path of the new methods through `ecbench` is untested at m ≥ 16.** It is covered only at m=6 (`wide=true`) and m=6..10 (unit tests).
- **Unrelated files in the branch.** The diff includes about 120 research session and log files from the replay. Keep them out of the instrument commit, or document them.