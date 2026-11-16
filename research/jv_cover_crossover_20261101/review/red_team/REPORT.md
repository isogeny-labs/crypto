# Red-team review (breakthrough tier): does the Joux–Vitse cover cost less than Pollard rho at p = 1451 and 1601?

Reviewed snapshot: `main` at `4def6153d1` (PR #3194), study `research/jv_cover_crossover_20261101/` (PROTOCOL sha256 `6643ce68…b761`, binary sha256 `4a1284e9…9752`; the binary at `target/release/examples/jv_cover` of the reviewed tree has the same hash, and every control below ran that binary). Predecessor: `research/jv_cover_p_scaling_20261024/`. Reviewer: Red Team, independent session, 2026-11-01. Nothing in `src/`, the study's files or research status was changed; this directory (`review/red_team/`) is new.

Labels: **measured** = read from a frozen study record or from a control record in `controls/`; **derived** = computed from measured values by the Rust tool in `analysis/` (`analysis.txt`, `analysis.json`; no Python computed any number); **model** = random-walk expectation, code-derived correction, or extrapolation.

## Claim under review

"The genus-3 cover-and-decomposition DLP (`examples/jv_cover.rs`, `src/cryptanalysis/jv_cover.rs`) costs less than measured Pollard rho on the weak class of E(F_{p^6}) with full rational 2-torsion at p = 1451 and 1601: S/rho = 0.775 [0.645, 0.970] and 0.587 [0.448, 0.811] against `rhodp`; not resolved against `rhostrong`." The study itself labels this a candidate result, a lower-bound (w = 0) comparison, and withholds it from the leaderboard.

## Summary of challenges

| # | Challenge | Severity | Overturns the claim? |
|---|---|---|---|
| C1 | The primary reference is not a competitive rho: no negation map. Against the negation-map rho (the reference the repository's own leaderboard names as "the matched §5a reference" for this pipeline) the cover is **above** at 1301 and 1451 and **not resolved** at 1601. | high | **Yes**, as a statement about rho. No, as a statement about `rhodp`. |
| C2 | The unit charges rho 331 F_p multiplications per step (schoolbook F_{p^6} products at 55, one inversion at 166 per step); textbook arithmetic with batched inversion costs 0.416 of that. In F_p multiplications the cover costs **2.2–3.5×** a negation-map rho at every prime measured. | high | **Yes**, in the unit the cover is measured in. |
| C3 | §5a band: the cover arm *does* emit a native counter (`f4_native_ops`, 15–24 per √l, a lower bound) that the study left unpriced. Priced like the rho arms' native work, the cover/rhodp order flips at w = 0.1 (1.5–2.0×) and is 5.8–7.6× at w = 1. No ranking holds across the band. | high for any ranking | Yes for any ranking; the study already concedes no band ranking. |
| C4 | The calibration (positive control) is contaminated: `rho_e_dp` polls the stop flag every 2^20 steps per thread, so with 4 threads the three non-finding walkers overshoot by about 2^19 steps each, +0.20 in S at p = 251 (model from the code). The reported +0.275 becomes +0.077 after correction. A clean null (120 runs, ≤ 2 threads) puts the instrument at 1.266 ± 0.059, on theory. At the new primes the overshoot is 0.001; the per-prime `rhodp` means (n = 9) are noise-limited, not biased. | medium | No. The new-prime `rhodp` numbers stand; the calibration as run does not show what it claims. |
| C5 | Statistics: 3 cover seeds with a 1.7× spread at 1601 (relation-yield luck, corr(ln S·p², ln residual-ratio) = 1.00); the 1451 "below" verdict has a 3 % margin (hi = 0.970); "0 mismatches" is vacuous (the mitm cross-check never ran: `cross_checked = 0` at every new prime). | medium | No; it narrows the claim and names its fragility. |
| C6 | Instrument change (lin_muls × 0.949 at p = 1009): the linear-stage default switched `sparse → fast` on 2026-10-27, after the predecessor binary. See the reproduction control below. The per-prime verdicts use only new-binary cover S; only the refit mixes instruments. | low | No. |
| C7 | Uncharged terms: cover F_p additions (≈ one per multiplication, unemitted), F4 pair selection / matrix assembly / sorting (unemitted), per-thread `Cover`/`ReIm` construction (counter reset, negligible); rho hash/DP/scalar ops (derived) and F_p additions (45/step in rhostrong's convention). | medium | Together with C3: no complete-accounting comparison exists, so no "beats rho" claim can be made under AGENTS.md "Success criterion" / §5a. |
| C8 | Scope and extrapolation: the class is the Joux–Vitse weak class (ePrint 2011/020 solved a DLP on a "130-bit degree 6 extension field", p ≈ 2^22). That the cover beats rho on this class at scale is literature; the study's content is the crossover of *this* implementation against *this* harness rho at p ≈ 2^10.3–2^10.6. Fair-reference crossover extrapolates to p ≈ 2330–2420 (model), outside the implementation's u64 range (p ≤ 2039). | n/a for the claim; high for its interpretation | Not a breakthrough; bears on nothing outside the class. |

**Cheapest falsification (zero compute):** read the study's own `rhostrong` records in the tuned F_p-multiplication unit (`s_tuned`, recorded in every row) and compare with the cover's `total_muls`: cover/rho = 3.5, 2.9, 2.2 at 1301/1451/1601, intervals excluding 1 at every prime. The claim "costs less than rho" fails in the unit the cover is measured in, against the reference the repository names as matched. I confirmed the reference with 64 further negation-map runs (pooled n = 82, mean 0.885 ± 0.054 steps/√l, theory 0.888).

**Did the claim survive?** Only in its narrowest reading: at p = 1451 and 1601 the cover's F_p multiplications (F4 native work and additions unpriced) are fewer than those of the harness's own no-negation, per-step-inversion, schoolbook r-adding DP walk on the same instances; that reading survives every attack here, including replacing the noisy per-prime reference by the theoretical plain-walk value (0.849 [0.771, 0.975] and 0.637 [0.523, 0.878]). As a statement about Pollard rho it does not survive.

## C1. The reference: `rhodp` is not the competitive rho

**Measured (code):** `rho_e_dp` (`src/cryptanalysis/jv_cover.rs` 2789–2896) is a 32-adding walk on points, no negation map, `EllE::add` with one `Fq3::inv` per step. `harness_prices` (`src/cryptanalysis/jv_rho_strong.rs` 146–161) measures the harness's own F_{q³} product, squaring and inversion at 55, 55, 166 F_p multiplications; an affine addition is 166 + 2·55 + 55 = 331 = `c_add_e`. The curve y² = x³ + a₂x² + a₄x with α ∉ F_q has only ±1 as cheap automorphism, so the negation map is the one standard acceleration available and it is implemented in the same tree (`rho_e_neg`, `run_strong_rho --neg 1`).

**Repository convention.** `docs/ic/LEADERBOARD.md`, section "Linear stage after F4 across Gröbner-basis corpora" (the postgb study on this very pipeline): "S over the negation-map rho, the matched §5a reference, is the distance remaining." The study's primary reference contradicts the convention the repository already applied to this pipeline.

**Derived (study rule, 20 000-resample percentile bootstrap, `analysis.txt`):**

| p | cover S geo-mean | vs rhodp this prime (study R1) | vs negation rho, pooled n = 82 (R5) | vs negation theory 0.888 (R6) | vs negation rho, tuned F_p-mul unit (R7) |
|---|---|---|---|---|---|
| 1301 | 1.310 | 0.793 [0.587, 1.117] not resolved | **1.480 [1.244, 1.759] above** | 1.475 [1.267, 1.682] above | **3.540 [2.971, 4.193] above** |
| 1451 | 1.081 | 0.775 [0.647, 0.971] below | **1.221 [1.042, 1.437] above** | 1.217 [1.106, 1.398] above | **2.922 [2.518, 3.453] above** |
| 1601 | 0.811 | 0.587 [0.448, 0.812] below | **0.916 [0.718, 1.239] not resolved** | 0.913 [0.749, 1.260] not resolved | **2.192 [1.710, 2.977] above** |

Per-seed ratios vs R5: 1301: 1.69 / 1.51 / 1.27; 1451: 1.11 / 1.17 / 1.40; 1601: 1.26 / 0.81 / 0.75. The study's own secondary reference (R4, n = 6 per prime) gives the same verdicts except 1451 (not resolved instead of above): the difference is only the reference's sample size. The R1 numbers are my independent reproduction of the study's table (bootstrap-RNG differences in the third decimal only), so the study's arithmetic is right; its choice of primary reference is what the verdict hangs on.

**Negation reference, measured.** Study rhostrong, 18 runs at the new primes: steps/√l 0.960 ± 0.091. Red-team rhostrong at p = 251, same settings (`--neg 1 --batch 16 --r 256`), 64 runs (`controls/rhostrong_neg_p251_run*.json`): 0.864 ± 0.064. Pooled 82: **0.885 ± 0.054**, theory √(π/4)/√(1−1/256) = 0.888 (z −0.05). The rho walk's expected S does not depend on p, so pooling across primes is legitimate and gives a reference five times tighter than the per-prime n = 6.

## C2. The unit: 331 F_p multiplications per rho step is the harness's slack, not rho's cost

The cover's `total_muls` are atomic F_p multiplications in F4 row reductions (77.8 % F4, 21.5 % linear stage, 0.04 % LA; derived from the nine records). They carry no extension-field arithmetic to tune. The rho step does: `TUNED` (`jv_rho_strong.rs` 167–171: Karatsuba 24, Chung–Hasan 18, adjugate inversion 62) with batch-16 Montgomery inversion gives `s_tuned`/`s_harness` = 0.416 (measured, identical in every rhostrong row). These are textbook operation counts, not a calibration. A comparison that charges the cover its actual F_p multiplications must charge rho the F_p multiplications a competent implementation needs: that is R7 above, 2.2–3.5× against the cover at every prime, and 1.4–1.9× (0.59–0.78 ÷ 0.416) even against the no-negation walk.

Mechanism check (where the "advantage" is created): cover cost ≈ 360 p·C_cov with C_cov = 9.55e5 F_p multiplications per Nagao test (measured, flat in p); rho cost = S_ρ·c·p³/2. With the study's (S_ρ, c) = (1.40, 331) the crossover is p ≈ 1290; with a negation walk at textbook prices (0.885, 138) it is p ≈ 2300. The headline crossover is produced by the reference's constants, not by anything in the cover.

**Model extrapolation** (slope −2.10 from the study's fit A, anchored at each prime): crossover against negation rho in group additions p* ≈ 1535–1596; in the tuned F_p-multiplication unit p* ≈ 2330–2420. The implementation stores l in u64, which caps p at 2039 (`group_order_4_prime` comment): the fair crossover is out of the current code's reach.

## C3. The §5a band with the counters that exist

The study states no band ranking is possible because the cover arm emits no native count. It does emit one: `f4_native_ops` (leading-monomial divisibility tests and monomial-table probes; `f4_fp.rs` says "incomplete by design … a lower bound"). Per √l (derived): 23.9 / 19.7 / 15.1 at 1301 / 1451 / 1601, of the same magnitude as rhostrong's complete native count (16.3). Priced the same way as the rho arms (`analysis.txt`, "5a band"):

| p | w | cover S_w (lower bound) | rhodp S_w (1 native/step; 4/step) | rhostrong S_w | cover/rhodp | cover/rhostrong |
|---|---|---|---|---|---|---|
| 1451 | 0 | 1.081 | 1.395; 1.395 | 1.017 | 0.775 | 1.063 |
| 1451 | 0.1 | 3.052 | 1.534; 1.953 | 2.742 | **1.990 (1.563)** | 1.113 |
| 1451 | 1 | 20.79 | 2.789; 6.974 | 18.27 | **7.454 (2.982)** | 1.138 |
| 1601 | 0 | 0.811 | 1.381; 1.381 | 0.892 | 0.587 | 0.910 |
| 1601 | 0.1 | 2.323 | 1.519; 1.933 | 2.403 | **1.530 (1.202)** | 0.967 |
| 1601 | 1 | 15.93 | 2.761; 6.903 | 16.01 | **5.770 (2.308)** | 0.995 |

The order against `rhodp` flips inside the band at every prime; against rhostrong the cover is above at 1451 at every w and within 3 % of parity at 1601 with its own native count a lower bound and its F_p additions (≈ 5e11, about one per multiplication) unemitted while rhostrong's are emitted (45.1 per step). Under "Rank only where the conclusion holds at every weight in the band", the arms are unresolved at best and never a win for the cover.

## C4. The calibration control is contaminated by a stop-flag artifact; a clean null puts `rhodp` on theory

**Model (from the code):** in `rho_e_dp`, a walker checks `stop` only when `local & (2^20 − 1) == 0` (line 2859) and at the top of its restart loop. After one walker finds the collision, each of the other `threads − 1` walkers runs on average 2^19 further steps, charged into `steps` and `ops`. Expected overshoot in S: (threads − 1)·2^19/√l = 0.199 at p = 251 with 4 threads, 0.066 with 2, 0 with 1; 0.0010 at the new primes.

**Measured / derived:**

| instrument | n | mean S | se | overshoot (model) | corrected | z vs theory 1.273 |
|---|---|---|---|---|---|---|
| study calibration `rhodp` p = 251, 4 threads | 16 | 1.539 | 0.166 | 0.199 | 1.340 | +0.40 |
| predecessor `rho_e` p = 251 (stored table) | 32 | 1.264 | 0.099 | 0 | 1.264 | −0.10 |
| red-team `rhodp` null p = 251, 2 threads (`controls/rhodp_null_p251_s{1,2}.json`) | 96 | 1.294 | 0.069 | 0.066 | 1.228 | −0.66 |
| red-team `rhodp` null p = 251, 1 thread (`controls/rhodp_null_p251_s1_t1.json`) | 24 | 1.150 | 0.113 | 0 | 1.150 | −1.08 |
| red-team null pooled | 120 | 1.266 | 0.059 | | | |
| study `rhodp` new primes, 4 threads | 27 | 1.475 | 0.121 | 0.001 | 1.474 | +1.66 |
| red-team strong harness, no negation, batch 1, r 32, p = 251 (`controls/rhostrong_noneg_p251_run*.json`) | 48 | 1.440 | 0.095 | 0 | 1.440 | +1.75 |

Welch, study calibration minus `rho_e`: +0.275 (t +1.42) as reported; +0.077 (t +0.40) after the overshoot correction. The calibration's "interval contains 0" passed for a reason that has nothing to do with the new primes (where the overshoot vanishes) and in spite of a +0.2 artifact at p = 251. The clean null shows no bias in the walk itself. The new-prime `rhodp` mean (1.475) and the single-thread strong no-negation arm (1.440) are each +1.7σ above theory with no common mechanism I could find (different code, different thread counts, different hashes); I record them as consistent with chance at sd/mean ≈ 0.5. Consequence: the per-prime `rhodp` means are not reliable point values (se 0.13–0.27 at n = 9); the appropriate no-negation reference is the pooled or theoretical 1.27–1.48, under which 1451 remains barely below (0.733–0.849) and 1601 below (0.54–0.64).

## C5. Statistics and record hygiene

- Cover S per seed (measured): 1301: 1.494 / 1.337 / 1.125; 1451: 0.982 / 1.036 / 1.241; 1601: 1.118 / 0.716 / 0.665. The p²-normalised S correlates 1.00 (derived) with the residual ratio (residuals ÷ 720·|F|, 0.79–1.44): the seed spread is how many residuals the instance needed per relation, ±30 %. Three seeds cannot pin the class mean to better than ±20 %; the 1601 verdict against the negation rho (0.92 [0.72, 1.24]) could resolve either way with eight seeds.
- The 1451 "below" has hi = 0.970 (study), 0.971 (my reproduction), 0.975 (fixed theoretical reference): a 3 % margin on a 10-multiset seed bootstrap.
- `cross_checked = 0` in every new-prime record: `table3` is built only for |F| ≤ 40, so the "0 mismatches" the README cites is vacuous at these sizes. Correctness rests on the final dd·G = Q check (`correct = true`), which is sufficient for the answer but is not an oracle cross-check.
- Probe reuse: `results/probe/dlp_p1301_s1.json` and `results/dlp_p1301_s1.json` are byte-identical (sha256 `0f8a0153…`); same binary hash in `versions.txt`. Post-freeze analysis-tool edits are disclosed; my tool reproduces every R1/R4 interval, so they changed no verdict.
- Six verdicts (3 primes × 2 references), two "below", both against the weaker reference and at the two primes where the cover's yield luck was best (1601 seeds 2–3 at residual ratios 0.86, 0.79).

## C6. The 0.949× `lin_muls` change

`lin_mode()` (`jv_cover.rs` 1988–1997): "default fast since research/postgb_linear_stage_20261026; sparse is the earlier default"; commit `e6ff48ea72` (2026-10-27, "fast becomes the default linear stage") postdates the predecessor binary (2026-10-24). The postgb study validated the new stage on 253 systems (same solution sets) and reports the counted change inside this pipeline. Reproduction control (red team, `controls/repro_sparse_p1009_s1.json`, `JV_LIN=sparse`, 2 rayon threads, 1046 s): **bit-identical to the predecessor** — `total_muls` 452313823812 and `lin_muls` 113394312494 exactly, same `f4_muls` (335940657368) and relation digest as the study's failed reproduction (measured). The failed bit-identity is entirely the `sparse → fast` default; with `fast` the same instance counts 429320392173 (0.949×). The per-prime verdicts never use the predecessor's cover points, so this affects only the refit (variant F, 1264 → 1253, inside the interval). Not a relabelling: the algorithm changed, in a separately recorded component study; but the study should have named the cause rather than reporting an unexplained failed control.

## C7. Uncharged terms (complete list found)

Cover arm: (1) F_p additions/subtractions in F4 and the linear stage — unemitted, about one per multiplication (≈ 4–6e11 per run); (2) `f4_native_ops` — emitted, lower bound (2.2–4.2e10 per run); (3) F4 pair selection, matrix assembly, sorting — unemitted; (4) `Cover::new` and `ReIm::new` per rayon thread in `map_init` — multiplications reset in `Ctx::with_trace`, so uncharged, negligible; (5) `by_x` hash probes in `nagao_decompose` — native, negligible; (6) Wiedemann ops priced at 16 F_p multiplications per 61-bit op (0.04 % of total, assumption-level pricing); (7) memory traffic of the F4 matrices (no byte counter; §5a prices operations, not bytes). Charged: setup/transfer, factor base, stream, Weil forms, F4 and linear multiplications, post-processing, verification, LA. Rho arm (`rhodp`): hash, DP test, index, two scalar updates per step (derived 1–4 per step, not emitted), F_p additions (unemitted), DP table (kB). `rhostrong`: all native counts and F_p additions emitted. Neither arm charges instance generation (`generate_spec`, group order) — correct, it is not attack cost. No inverse-success-probability term is missing: every run solved at its first LA attempt. Multithreading: the F4 counter is process-wide (`f4_fp::FIELD_OPS`); control `f4check_t{1,2}` shows `f4_muls` identical at 1 and 2 rayon threads (6197337097) while `f4_muls_per_test_sum` inflates 0.997 → 1.83×, so the global batch difference the study uses is the correct, thread-independent count and the cover is not under-counted.

## C8. Scope, nearby objects, extrapolation

- The class (E(F_{p^6}), full rational 2-torsion, genus-3 cover over F_{p²}, never prime order) is the Joux–Vitse weak class. ePrint 2011/020 (abstract verified): "a real-size example of discrete logarithm computations on a seemingly secure curve defined over a 130-bit degree 6 extension field", p ≈ 2^22, eleven doublings of p beyond this study. That cover-and-decomposition beats rho on this class is established; this study measures where *this* F4-based implementation crosses *this* harness's rho, at l ≈ 2^60–2^62. It bears on no prime-order curve, no deployed curve, and no row of leaderboard tables A–C (Table D already says "comparison withheld" for the class); as program evidence it is an engineering milestone of the `f4_fp` pipeline.
- Null for the cover: no nearby object on which the method runs but the effect must be absent exists (a non-weak curve has no cover). The internal null is the p^−2 trend, which holds (slope −2.10 [−2.21, −1.99]).
- Null for the reference instrument: run and reported in C4.
- Nearby instrument: the single-threaded strong harness without negation (batch 1, r 32) at p = 251 reads 1.440 ± 0.095, the `rhodp` null 1.266 ± 0.059; both no-negation instruments bracket the study's new-prime `rhodp` mean.
- Extrapolation: from p = 1601 to the fair crossover is a factor 1.45–1.5 in p (3.3–3.6 in √l); the first thing that breaks is the u64 group order at p = 2039.

## Controls run (red team, ≤ 2 threads, same binary; `controls/walls.txt` has every exit status)

1. `f4check_t1`, `f4check_t2`: `--exp dlp --sizes 53 --first-seed 2 --rho-runs 0` under `RAYON_NUM_THREADS=1` and `=2` (19 s, 15 s). F4 counter thread-independence: passed.
2. `rhodp_null_p251_s1`, `_s2`: `--exp rhodp --sizes 251 --rho-runs 48 --dp-bits 12 --threads 2` (284 s, 283 s); `rhodp_null_p251_s1_t1`: 24 runs, `--threads 1`.
3. `rhostrong_neg_p251_run0..31`: `--exp rhostrong --sizes 251 --seeds 2 --neg 1 --batch 16 --r 256 --dp-bits 12` (64 runs); `rhostrong_noneg_p251_run0..23`: `--neg 0 --batch 1 --r 32` (48 runs). All `correct`, none capped.
4. `repro_sparse_p1009_s1`: the study's reproduction command with `JV_LIN=sparse`, `RAYON_NUM_THREADS=2`.
5. Analysis: `analysis/` (Rust, `cargo build --release --offline`; run `./target/release/jv_crossover_red_team ../../../results ../../../../jv_cover_p_scaling_20261024/results ../controls ..`), outputs `analysis.txt`, `analysis.json`. Script: `controls/controls.sh`.

## Verdict: the defensible statement

Defensible: *On the `jv_cover` weak class at p = 1451 and 1601 (l ≈ 2^61–2^62), the cover pipeline's counted F_p multiplications (F4 native work, F_p additions and memory unpriced; w = 0) were 0.78× and 0.59× those of the harness's own no-negation, per-step-inversion, schoolbook r-adding distinguished-point walk on the same instances (0.85× and 0.64× against the theoretical plain-walk value); against the negation-map rho — the matched reference under the repository's own convention — the cover is above at 1301 (1.48 [1.24, 1.76]) and 1451 (1.22 [1.04, 1.44]) and not resolved at 1601 (0.92 [0.72, 1.24]) in group additions, and 2.2–3.5× above in F_p multiplications at textbook arithmetic prices; under the §5a band with the counters that exist the order never holds for the cover at any w > 0. The crossover against a competitive rho extrapolates (model) to p ≈ 2300–2400, beyond the implementation's range.*

Not defensible: "costs less than Pollard rho", "crosses rho", any leaderboard or `benchmarks/` row, any statement about curves outside the class. Not a breakthrough: the class is known-weak at far larger sizes; the result is engineering progress of the F4 pipeline whose distance remaining to the matched rho is 1.5× / 1.2× / ~0.9× (unresolved) in group additions and 3.5× / 2.9× / 2.2× in F_p multiplications.

Remaining uncertainty, named: (i) the +1.7σ excess of the 4-thread `rhodp` at the new primes is unexplained though consistent with chance (my null used ≤ 2 threads); (ii) the cover's S carries ±30 % relation-yield variance and only three seeds per prime, so the 1601 verdict against the negation rho is genuinely open; (iii) `TUNED` is a formula price (Karatsuba/Chung–Hasan/adjugate), not a measured kernel — a measured tuned rho could sit somewhat above 0.37 but not at 0.81; (iv) the fair crossover is an extrapolation on slope −2.10.

```yaml
red_team_report:
  id: RT-20261101-001
  task_id: null   # dispatched from the main session for PR #3194; no TASK id supplied
  claim_under_review: >-
    research/jv_cover_crossover_20261101: the jv_cover genus-3 cover-and-decomposition DLP on the
    weak class of E(F_{p^6}) with full rational 2-torsion costs less than measured Pollard rho at
    p = 1451 and 1601 (S/rhodp 0.775 [0.645, 0.970] and 0.587 [0.448, 0.811]); not resolved vs rhostrong.
  objections:
    - C1 reference: rhodp has no negation map; against the negation-map rho (pooled n=82, 0.885 +- 0.054
      steps/sqrt(l), theory 0.888) the cover is above at 1301 (1.48 [1.24, 1.76]) and 1451
      (1.22 [1.04, 1.44]) and not resolved at 1601 (0.92 [0.72, 1.24]). The leaderboard's own postgb
      section names the negation-map rho as the matched section-5a reference for this pipeline.
    - C2 unit: c_add_e = 331 is the harness's schoolbook/per-step-inversion price; textbook arithmetic is
      0.416 of it (s_tuned/s_harness, measured). In F_p multiplications the cover is 3.54 [2.97, 4.19],
      2.92 [2.52, 3.45], 2.19 [1.71, 2.98] x a negation-map rho at 1301/1451/1601.
    - C3 band: the cover's emitted f4_native_ops (15-24 per sqrt(l), lower bound) priced like the rho arms
      flips cover/rhodp to 1.5-2.0 at w=0.1 and 5.8-7.6 at w=1; vs rhostrong 1.11-1.14 at 1451 and
      0.97-0.995 at 1601 with the cover's count a lower bound. No ranking holds across the band.
    - C4 calibration: rho_e_dp polls the stop flag every 2^20 steps/thread; +0.199 S overshoot at p=251
      with 4 threads (model from code). Calibration difference +0.275 -> +0.077 corrected. Clean null
      (120 runs, <=2 threads) 1.266 +- 0.059 vs theory 1.273 and rho_e 1.264. New-prime rhodp 1.475 +- 0.121
      (27 runs) is +1.7 sigma, noise-limited per prime (n=9).
    - C5 statistics: three cover seeds, 1.7x spread at 1601 driven by relation yield (corr 1.00 with
      residual ratio); 1451 'below' margin 3% (hi 0.970-0.975); cross_checked = 0 everywhere, so
      '0 mismatches' is vacuous; correctness rests on the final dd*G = Q check only.
    - C6 instrument: lin_muls x0.949 is the JV_LIN sparse->fast default change of 2026-10-27 (postgb study),
      after the predecessor binary; affects only the refit, not the per-prime verdicts.
    - C8 scope: Joux-Vitse weak class, literature-solved at p ~ 2^22 (ePrint 2011/020); fair crossover
      extrapolates to p ~ 2300-2400, above the implementation's u64 limit p <= 2039.
  required_controls:
    - Adopt the negation-map rho (pooled across primes, >= 50 runs) as the primary reference and report S in
      both the group-addition and the textbook F_p-multiplication unit.
    - Emit the cover's F_p additions and a complete F4 native count (pair selection, assembly, sorting) so a
      section-5a band can be formed on both arms.
    - >= 8 cover seeds per prime at 1601 (and a larger prime if the u64 limit is lifted) before any per-prime
      verdict against the negation rho.
    - Fix rho_e_dp's stop polling (per DP or per 2^14 steps) or run the calibration single-threaded; re-run
      the positive control.
    - Reproduction control with JV_LIN=sparse (done here, see C6) recorded as the explanation of the failed
      bit-identity.
  counterexample_or_mutation: >-
    Zero-compute: the study's own rhostrong rows carry s_tuned; cover total_muls / (s_tuned * c_add_e * sqrt l)
    = 2.2-3.5 at every prime, intervals excluding 1. Mutation of the reference (negation on/off, arithmetic
    harness/tuned) moves the verdict from 'below' to 'above'; no mutation of the cover was needed.
  baseline_comparison: >-
    Correct baseline: negation-map r-adding rho with batched inversion and Karatsuba F_{p^6} arithmetic,
    0.885 sqrt(l) steps at ~138 F_p multiplications per step (~122 sqrt(l) F_p multiplications), versus the
    cover's 268-434 sqrt(l) F_p multiplications at 1301-1601. BSGS is irrelevant at l ~ 2^61 (memory). The
    closest specialised baseline is the literature cover attack itself, which is what the pipeline implements.
  heuristic_challenges:
    - The random-walk expectations (1.273 plain, 0.888 negation) are model values; the measured references
      agree with them (nulls in C4, C1) so they are used only as fixed cross-checks, never as the verdict.
  cost_model_challenges:
    - see C2, C3, C7: harness price of a rho step (331) vs textbook (138); unpriced cover additions and F4
      native work; derived rhodp native work (1-4 per step); LA priced by assumption (16 F_p mul per op).
  reduction_and_scope_challenges:
    - see C8: no transfer to prime-order or deployed curves; leaderboard Table D 'comparison withheld';
      implementation range p <= 2039 below the fair crossover.
  proof_architecture_challenges: []
  narrowest_supported_statement: >-
    At p = 1451 and 1601 on this weak class the cover's counted F_p multiplications (w = 0, native work and
    additions unpriced) are below those of the harness's no-negation, per-step-inversion r-adding DP walk
    (0.78x, 0.59x; 0.85x, 0.64x against the theoretical plain walk); against the negation-map rho the cover
    is above at 1301 and 1451 and unresolved at 1601 in group additions and 2.2-3.5x above in F_p
    multiplications at textbook prices. The p^-2 scaling (slope -2.10) is supported.
  next_concrete_action: >-
    Re-run the three primes' analysis against the pooled negation-map rho in both units (no new cover runs
    needed), file it as the study's distance-remaining table, and plan the next cover improvement against
    that number (a 2.2x reduction of F_p multiplications at p = 1601, or lifting the u64 limit to reach
    p ~ 2400).
  artifact_paths:
    - research/jv_cover_crossover_20261101/review/red_team/REPORT.md
    - research/jv_cover_crossover_20261101/review/red_team/analysis.txt
    - research/jv_cover_crossover_20261101/review/red_team/analysis.json
    - research/jv_cover_crossover_20261101/review/red_team/analysis/src/main.rs
    - research/jv_cover_crossover_20261101/review/red_team/controls/
  review_attestation:
    joints_owned: [reference, unit, section-5a band, calibration control, statistics, instrument change, scope]
    paths_read:
      - research/jv_cover_crossover_20261101/{README.md,PROTOCOL.md,table.md,run.sh,fit.txt,fit.json,fit/src/main.rs,results/*}
      - research/jv_cover_p_scaling_20261024/{README.md,PROTOCOL.md,results/dlp_p251_s*.json,results/dlp_p1009_s1.json}
      - examples/jv_cover.rs; src/cryptanalysis/jv_cover.rs (generate_spec, EllE, rho_e, rho_e_neg, rho_e_dp,
        run_rho_dp, unit_costs, factor_base, nagao_decompose, run_cover_dlp_mode, lin_mode);
        src/cryptanalysis/jv_rho_strong.rs (prices, strong_rho_core); src/cryptanalysis/f4_fp.rs (counters)
      - AGENTS.md (Success criterion, Tracks, section 5, 5a, 6); docs/ic/LEADERBOARD.md (Table D, postgb section)
      - research/postgb_linear_stage_20261026/validation/round2/out/dlp_{sparse,fast}_p53_s1.json
      - research/jv_sieve_strong_rho_20261019/review-20261020/README.md
    read_sibling_reports: false
    verdict: claim refuted as a statement about Pollard rho; narrowest statement above holds
```
