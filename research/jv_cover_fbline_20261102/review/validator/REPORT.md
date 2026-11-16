<!-- Written by the main session from the validator's returned text, verbatim apart from this line: the validator's harness blocked it from writing this file. -->
# Validator report: factor-base line search for the JV genus-3 cover (`research/jv_cover_fbline_20261102`)

**Snapshot:** main at `5f3b5c209acabf8ff948d95164b1508668e6b124` (PR #3286). The tree was clean apart from `review/`.

**Validator:** an independent session. It did not open any red-team report. Every number comes from its own Rust code (`check/`). Python was used only to edit that code's text, never for arithmetic.

**Verdict:** **passed, with scope limits.** The records are admissible evidence for the paired measured result on the 9 weak-class instances. They are not evidence of an asymptotic improvement. The U1 "below rho" ranking and the model p* values carry the limits in section 7.

## 0. Claim
`JV_FB_LINE=n` turns on a search over factor-base lines (`search_fb_line`, `line_poly`, `line_div`, `line_count`) and moves each instance to the isomorphic model H′ with the largest factor base. Claimed results:
- Geometric mean of S_line/S_plain: 0.587 [0.530, 0.651], a 1.71× speed-up, over p ∈ {1301, 1451, 1601} × seeds 1–3.
- Against #3278's negation-map rho, estimator (b) pooled (0.9322 in U1, 0.3879 in U2):
  - U1: 0.791 / 0.659 / 0.548, below rho.
  - U2: 1.900 / 1.585 / 1.318, above rho.

## 1. Receipts: pass
Checked field by field in `check_output.txt`, with 0 failures.
- **Results.** 9/9 runs are solved and correct, with mismatches 0 and timed_out 0. One test was incomplete in arm B (p1301 s3) and one in arm A (p1451 s3); an incomplete test can only lose relations.
- **Instance identity.** `l`, `c_add_e` (331) and `c_add_j` (2122) are equal between the arms on all 9. `stop_mode` is full in both arms.
- **`fb_line` fields.**
  - candidates = 65536.
  - `b_plain` equals A's base: 634, 651, 662, 735, 729, 703, 764, 820, 824.
  - `b_best` equals B's base: 728, 725, 730, 811, 800, 804, 884, 883, 883.
  - best_index > 0 on all 9.
- **Phase sums.**
  - total_muls = setup + base + stream + oracle + verify + la, exactly, on all 18 records.
  - oracle = weil + f4 + lin + post.
  - la_muls = 16·la_ops.
  - setup_muls ≥ fb_line.muls.
  - Setup minus the search differs from A by between −23,413 and +38,054. This comes from the stream start and step on the moved model, after the rng draws diverge. It is below 1e-7 of the total.
- **Recomputation.** S = total_muls/(c_add_e·√l) matches the recorded value to 1e-12. decomposition_rate = relations/residuals.
- **Search charge, derived independently.** (p−1)/2 + (17p+5) + (n−1)(17p+346) + 374.
  - It matches fb_line.muls on 6 of 9 runs. The other three are lower by 20, 10 and 10.
  - Each gap is a whole number of `pmul` zero-coefficient skips. Each skip is 2 F_q products, 10 multiplications not executed either. 4 skips were observed against about 6.0 expected.
- **Search share:** 0.508–0.574% of each run.
- **C4:** rate/((2B/p)⁶/720) lies between 0.921 and 0.994, inside the band [0.8, 1.25].
- **C1:** every deterministic field is equal to the crossover record: residuals 428147, digest 4667961897523824477, total_muls 409975021213, decompositions 663, and all phases. Only these differ: the timing fields, the racy multi-thread diagnostic `f4_muls_per_test_sum`, and the `fb_line: null` key, which the older schema lacks.
- **C2:**
  - Residuals, digest and decompositions are equal to C1.
  - Total and setup are each higher by exactly 22,772 = fb_line.muls = (p−1)/2 + 17p + 5.
  - Nothing else differs.

## 2. Freeze ordering: pass
- **Commits.**
  - `PROTOCOL.md` was added in `0c2258ead2` (00:54:43Z).
  - `run.sh`, `FROZEN.sha` and `versions.txt` were added in `795eb00bed` (01:01:29Z).
  - `fit/` and `FIT.sha` were added in `112da2ea31` (01:03:03Z).
  - None of these was modified afterwards.
- **Hashes match.** PROTOCOL 96420747…, run.sh 1da32f05…, fit/src/main.rs f4149680…, Cargo.toml f273020c….
- **Arm B started between 01:20:18 and 01:24:55**, after both freezes. The lower bound is the binary's mtime (00:57) plus C1 (678 s) and C2 (720 s). The upper bound is the first arm-B artifact, which appears in `09b723b71f` at 01:24:55Z. The FIT.sha directory listing taken at 01:02:43Z contains no `line_*` file.
- **Binary.** The target/ binary before my rebuild is dabf56bc…, matching versions.txt.
- **Notes:**
  - C1's start is bounded only from above (≤ 01:01:39), so it may have begun seconds before the freeze commit. This is immaterial, because C1's pass criterion is fixed bit-equality.
  - walls.txt already existed at 01:02:43, consistent with the launcher creating it in advance. There is no evidence of a discarded attempt.
  - Arm-B wall time totalled 4,850 s, under the 4 h cap, so nothing was cut.

## 3. Independent recomputation: agrees to 3 dp
I used my own xoshiro256** bootstrap, type-7 quantiles and 20,000 resamples.

| quantity | claimed | validator |
|---|---|---|
| S_B/S_A per instance | 0.488 0.585 0.625 0.612 0.611 0.492 0.468 0.704 0.761 | 0.4878 0.5845 0.6252 0.6125 0.6108 0.4917 0.4679 0.7035 0.7609 |
| geo-mean | 0.5865 [0.5301, 0.6508] | **0.5865** [0.5295, 0.6495] |
| speed-up | 1.705 [1.537, 1.886] | 1.705 [1.540, 1.888] |
| model geo-mean (B_plain/B_best)⁵ | 0.5872 | 0.5872 |
| spread of S across instances, plain → line | 2.2× → 1.55× | 2.245× → 1.550× |
| reference runs | 126 | 126 admitted, 0 excluded, 14 per instance; `l` and `c_add_e` equal to the cover's; s_harness and s_tuned recompute to 2e-16 |
| pooled reference means | 0.9322 / 0.3879 | 0.9322 / 0.3879 |
| U1 at p = 1301 / 1451 / 1601 | 0.791 / 0.659 / 0.548 | 0.791 [0.720, 0.871] / 0.659 [0.607, 0.719] / 0.548 [0.506, 0.596], below |
| U2 at p = 1301 / 1451 / 1601 | 1.900 / 1.585 / 1.318 | 1.900 [1.731, 2.094] / 1.585 [1.459, 1.729] / 1.318 [1.215, 1.433], above |
| plain-arm "before" values | 1.405 / 1.160 / 0.870; 3.376 / 2.787 / 2.090 | same |
| §5a band (w = 0.1, 1) | rescore_table.md | same. U1 is below at every w. U2 is above at w = 0 and not resolved or below at w ≥ 0.1 |
| model p* | 1185 / 1799 | 1185 / 1799 |

- **H1 decision rule:** holds (upper end < 1 and every S_B < S_A).
- **Fit of the model:** log(measured) against log(model) gives r = 0.992, slope 1.20.

## 4. Mathematics and code
- **Isomorphism, not twist.**
  - The map H′→H is (s, y′) ↦ (as+b, t⁷y′) with a = t², t ∈ F_q^×. Its inverse is ((x−b)/a, y/t⁷).
  - h = F·N is monic of degree 7, so h(as+b)/a⁷ is monic and the map is defined over F_q.
  - A non-square a would need √(a⁷) ∉ F_q, which gives the twist. The code only uses a = t·t.
  - The single point at infinity is fixed, so the Jacobians are isomorphic over F_q and logarithms are preserved.
  - Remark: square a reaches only half of the p+1 line directions.
- **line_div.**
  - u′ = u(as+b)/a^deg u is monic. v′ = v(as+b)·t⁻⁷.
  - v′² − h′ = (v² − h)(as+b)/a⁷, which is divisible by u′, and deg v′ < deg u′.
  - The identity element is left unchanged.
- **Confirmed in the runs.** The `is_valid` assertion on G′ and Q′ held (rc 0 on all runs). The logarithm is checked on the original E(F_{p⁶}) points, which the move does not touch.
- **Moved model used everywhere.** `Ctx::with_trace` moves `cov.hx`. These consumers all read the moved model:
  - `factor_base`
  - `weil_forms`
  - the y-recovery check
  - `verify_dec` (through `c.jac()`)
  - the residual stream
  - `unit_costs`
  - every worker context (`map_init` on the moved `spec`)

  `cov.nx`, `cov.fx`, `cov.d` and `transfer` are used only before the move or in other experiments. `mitm6` is inactive (small > 40).
- **Charging, F_p multiplications in the harness unit.**
  - `line_count` charges 17p per line, which equals the `mm` calls it executes (Horner 14, norm 3).
  - The norm-based squareness test is valid: z is a square in F_q^× iff N(z) is a square in F_p^×, with F_q = F_p[i]/(i² − w).
  - line_poly, t², the inverse and the moves of G and Q go through the counted Fq operations (5 per product, 21 per inverse).
  - The total is read after the moves and enters setup_muls and total_muls.
- **Uncharged or unemitted items:**
  1. The square table executes p−1 products and charges (p−1)/2. That leaves 650–800 uncharged per run, about 3e-9 of the total.
  2. The search's 65536·p lookups (8.5e7–1.05e8 per run) are not emitted. As native operations, they move U1 at w=1 from 0.838/0.699/0.581 to 0.843/0.703/0.584, and U2 at w=0.1 from 1.050 to 1.054. No verdict changes.
  3. The search's F_p additions, about 15 per point per line, are not emitted (as for all cover additions).
  4. `line_poly` is recomputed in each Ctx (about 341 multiplications), re-deriving a value the run already holds. `Cover::new` and `is_valid` are uncharged on both arms.

## 5. Replay: bit-identical
- **Build.** `cargo build --release --example jv_cover` at main 5f3b5c209a (clean tree): 6 min 50 s, sha256 bb2b96d8….
  - It differs from the producer's binary because other modules changed, and `jv_cover.rs` had rustfmt-only edits since 0c2258ead2.
  - `rref_mod_p`, `square_core`, `wiedemann_u64` and `f4_fp` are unchanged.
- **Run.** `env -u JV_LIN JV_FB_LINE=65536 jv_cover --exp dlp --rho-runs 0 --rho-ref 1.3 --check-every 64 --sizes 1301 --first-seed 3 --seeds 1`: rc 0, 361 s on 4 cores.
- **Comparison.** 63 of 67 fields are identical:
  - total_muls 256314105656
  - residuals 265906
  - relation_digest 7945254227474834773
  - decompositions and relations 731, base 730
  - setup 1473116883, f4_muls 198197818058
  - f4_native_ops 13962232205, la_ops 13223299
  - solved and correct true
  - fb_line {b_plain 662, b_best 730, best_index 8085, muls 1472135851}

  Only wall_ms, f4_ms, lin_ms and f4_muls_per_test_sum differ. The log is identical apart from the wall time.

## 6. Heuristic, null-control and cost-model checks
- **Pre-registration.** The rate model, the B⁻⁵ scaling, the per-instance (B_plain/B_best)⁵, C1–C4 and the decision rule were frozen before arm B. Predicted 0.5872; measured 0.5865. The largest per-instance deviation is +7.5% (p1601 s3: 0.761 against 0.708).
- **Null and dose response.**
  - C2 (n=1) is the null of the mechanism: the residuals and relations are identical, and the cost ratio is 1 + 5.6e-8.
  - The measured ratios follow the model with r = 0.992.
  - The plain bases' z-scores have sd 1.03, which confirms σ_B ≈ √p/2. The best bases' z-scores average k = 4.20, the expected maximum of 65536 draws.
- **Decay.** At fixed n the gain fades with p: g(p) ≈ (1 + 4.2/√p)⁻⁵, which is 0.577 at p=1301, 0.641 at 2039, 0.728 at 4096 and 0.922 at 65536. The README does not state this (L1).
- **Unit.** The unit is declared: harness F_p multiplications / (c_add_e·√l). U2 compares harness counts on the cover with tuned counts on rho; this is stated and inherited from #3278. Memory is not reported for either arm, and byte counters are not charged under §5a.
- **§6 ladder.** The primary claim is measured as a paired ratio on the stated instances. Only p* is a model, and it is labelled as one. The reproducibility pointer was exercised (section 5).

## 7. Limitations and scope
- **L1.** This is not an asymptotic improvement. At fixed n the gain is about (1 + 4.2/√p)⁻⁵, and the 1.71× applies only to p ≈ 1301–1601.
- **L2.** The model p* reuses the plain arm's slope (−2.10) and ignores L1. Adding the gain's p-dependence (d ln g/d ln p ≈ +0.24) moves U2 p* from 1799 to about **1850** and U1 p* from 1185 to about 1152. Both remain model values, both ≤ 2039.
- **L3.** "U1 below at every prime" ranks a cover whose cost is a lower bound: its F_p additions are not emitted, while rho's are (0.127 S-units).
  - Assume one addition per counted multiplication (not measured). Then the U1 point estimate reaches 1 when an addition costs α* = **0.32** multiplications at p=1301, 0.65 at p=1451 and 1.10 at p=1601.
  - With 0.5 additions per multiplication, α* is 0.81, 1.76 and 3.28.
  - So the 1301 "below" is fragile under complete counts. `research/ic_acct1_cover_bitop_20261021` could settle the additions-per-multiplication ratio.
  - The paired S_B/S_A is unaffected.
- **L4.** There are 3 seeds per prime and one residual stream per instance. The rng draws diverge after `factor_base`, so the pairing is per instance.
- **L5.**
  - The runs were local, not on isolab-eks; this is disclosed.
  - How the line-search records were staged into negrho_rescore's `inputs/cover/` input is not recorded. My numbers reproduce it.
  - The README's "fit/src/main.rs sha af4e1355…" refers to #3278's tool, not this study's `fit/` (f4149680…).
- **L6.** The claim covers the weak class only (full rational 2-torsion, #E = 4l), with l ≈ 2^60–2^62.
- **L7.** The square-table undercharge and the unemitted search lookups and additions are quantified in section 4 and are negligible.

## 8. Files
- `check/` (Cargo.toml, src/main.rs, Cargo.lock). Build and run with `CARGO_TARGET_DIR=<tmp> cargo build --release --offline && <tmp>/release/fbline_validator_check <repo> <out> [replay.json]`.
- `check_output.txt`.
- `replay/` (REPLAY_ENV.txt, line_p1301_s3.json, line_p1301_s3.log).
- `artifact_hashes.txt` and `reference_runs.sha256`.
- `validation_report.yaml`.
