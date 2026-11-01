# Validator report (breakthrough tier): research/jv_cover_crossover_20261101

**Report id:** VAL-20261101-8c569f, 2026-11-01. The contract record is [`validation_report.yaml`](validation_report.yaml). The check code is [`recompute.rs`](recompute.rs) and [`verify_log.rs`](verify_log.rs), with outputs in [`receipts/`](receipts/).

**How this file was written.** The validator agent's harness blocked it from writing this file. The main session placed the agent's returned text here unedited, apart from Markdown layout. Every other file in this directory was written by the validator itself.

**Labels.**
- **measured**: a run record, or a run made by the validator.
- **derived**: computed by the validator's own Rust code, which shares no code with `fit/` or `jv_cover.rs`.
- **model**: a sensitivity outside the frozen rule.

**Compute.** About 48 min of 2-thread time:
- build: 10 min 15 s;
- replay: 764 s;
- follow-ups: 1445 s.

## Overall verdict: `passed` (the receipt is admissible evidence)

**What stands.** The claim "below measured rho at p against rhodp at 1451 and 1601" **stands as stated**, but only under all of these conditions:
- the frozen rule;
- the harness unit, 331 F_p multiplications per E(F_{p^6}) affine addition;
- §5a weight 0;
- the rhodp reference;
- 3 seeds per prime, at l ≈ 2^61;
- the weak class only.

**What does not follow.** It does **not** support a claim of beating rho in any broader sense:
- It fails against the study's own strongest reference, rhostrong with the negation map.
- It reverses under the binary's own tuned price.
- It reverses under any §5a weight above 0.
- It resolves nothing under a t-based interval.
- The 1451 verdict lies inside the 5.4% instrument-change margin.

**Routing.** The breakthrough-tier condition (a claimed below-measured-rho result) holds. It is not a closure, and it contradicts no prior validated evidence.

## Snapshot

- `main`: 4def6153d1, clean.
- Study commit: 3b946d69b6, 2026-11-01 09:40:23Z.
- Artifact-store commit: 470a02cdea.
- PR #3194 merge: 14e56a5e85.
- Predecessor: 6e9bb8e97f.
- Host: a 4-vCPU Xeon shared with the red team. Every run here used 2 threads or fewer.

## Verdict table

| # | Check | Result |
|---|---|---|
| 1 | Receipts and integrity | PASS. The freeze-before-data timestamp rests on the producer's record (1.4). |
| 2a | Cover S computed per its definition, and comparable to rho S | PASS as frozen. It is comparable **only** inside the harness unit at w = 0; this is a scope limitation (2.3). |
| 2b | c_add_e and l | PASS |
| 2c | Every cover run solved the right log, independently verified | PASS for the replayed instance, p = 1301 seed 3, checked with the validator's own arithmetic. The other 8 cover runs and all rho runs rest on the binary's planted-log flag, on the same code path. |
| 3 | Replay | PASS. p = 1301 s3 replayed bit-for-bit (all non-time fields). Repeated rhodp and rhostrong invocations were bit-identical. |
| 4 | Reproduction-control failure | PASS. The cause is identified and measured: with `JV_LIN=sparse` the rebuilt binary reproduces the predecessor's records bit-for-bit. |
| 5 | Calibration | PASS on arithmetic, which reproduces exactly. Qualified as a control: most of the +0.276 (0.199 of it) is the 2^20 stop-poll lag of `rho_e_dp` at p = 251. |

## 1. Receipts and integrity

**1.1 Frozen files** (measured).
- sha256 of PROTOCOL.md is `6643ce68…b761`; sha256 of run.sh is `f2f4b8da…6362`.
- Both equal `results/FROZEN.sha`.
- Each file was touched by exactly one commit, 3b946d69b6.

**1.2 Records** (measured).
- **Counts:**
  - 9 cover runs;
  - 1 reproduction run;
  - 16 calibration runs (2 instances);
  - 27 rhodp runs (9 instances);
  - 18 rhostrong runs (6 instances).
- **Exits:** `walls.txt` shows every exit 0 and `ALL DONE total_s 24850`.
- **Probe reuse:** `probe/dlp_p1301_s1.{json,log}` is byte-identical to the reused `dlp_p1301_s1.*`.
- **Cover records:**
  - All are solved, correct, mismatches 0, timed_out 0, la_attempts 1.
  - There is one incomplete test at 1451 s3 and one in the 1009 reproduction; both are charged.
- **Rho records:** all rhodp runs are correct. All rhostrong runs are correct and not capped.
- **Artifact store:** `scaling.png` is in the artifact store (sha `c351ac6c…`), as expected.

**1.3 Recomputation** (derived, by two independent paths).

*Path 1: the study's own tool.* The study's `fit/` was rebuilt offline and re-run. `fit.txt`, `fit.json`, `table.md` and `scaling.svg` came out **byte-identical** to the committed files.

*Path 2: the validator's own code, `recompute.rs`.*
- **Per-run S** equals every recorded `s` to within 1e-15.
- **Component sums:** setup + base + stream + weil + f4 + lin + post + verify + la_muls equals `total_muls` on all 10 cover records. The README's formula omits `la_muls`; the binary includes it. This is harmless.
- **Geometric means of cover S:** 1.3097 / 1.0810 / 0.8108.
- **rhodp means:**
  - 1301: 1.6509 ± 0.7953
  - 1451: 1.3947 ± 0.6707
  - 1601: 1.3805 ± 0.3849
  - n = 9 each.
- **rhostrong means:**
  - 1301: 0.9709 ± 0.3475
  - 1451: 1.0172 ± 0.4768
  - 1601: 0.8915 ± 0.3825
  - n = 6 each.
- **Point ratios:**
  - vs rhodp: 0.7933 / 0.7751 / 0.5874;
  - vs rhostrong: 1.3489 / 1.0628 / 0.9095.
  - All equal the study's digits.

*Bootstrap 95% intervals, by p = 1301 / 1451 / 1601* (own RNG):

| Reference | Resamples | 1301 | 1451 | 1601 |
|---|---|---|---|---|
| rhodp | 20000 (validator) | [0.587, 1.118] | [0.645, 0.977] | [0.445, 0.813] |
| rhodp | 10^6 (validator) | [0.588, 1.118] | [0.647, 0.970] | [0.448, 0.812] |
| rhodp | study | [0.587, 1.119] | [0.645, 0.970] | [0.448, 0.811] |
| rhostrong | 20000 (validator) | [1.089, 1.699] | [0.810, 1.477] | [0.649, 1.339] |
| rhostrong | study | [1.085, 1.699] | [0.819, 1.477] | [0.658, 1.339] |

*Frozen-rule verdicts* reproduce:
- vs rhodp: 1451 and 1601 "below"; 1301 not resolved.
- vs rhostrong: 1301 "above"; the others not resolved.

The 1451 upper end is 3% below 1. Monte Carlo noise of the 97.5% percentile at 20000 resamples is about ±0.005, so the verdict is not an artifact of the resample count.

*Refit A:*

| | Slope | Slope interval | p* |
|---|---|---|---|
| Validator | −2.0985 (se 0.0533) | [−2.208, −1.989]; own cluster bootstrap [−2.236, −1.974] | 1263.6, bootstrap [1196, 1350] |
| Study | −2.098 | [−2.208, −1.989]; bootstrap [−2.234, −1.977] | 1264, bootstrap [1197, 1348] |

*Other numbers:*
- The calibration and reproduction numbers reproduce exactly (sections 4 and 5).
- §5a native counts reproduce `fit.txt`:
  - rhodp: 1.6509 / 1.3947 / 1.3805 per √l (derived);
  - rhostrong: 16.47 / 17.25 / 15.12 (measured).

**1.4 Freeze before data.** This is not verifiable from git, because there is a single post-run commit. The consistent evidence:
- The probe binary was built at 02:11Z. The rhodp timing probe ran at 02:29Z. `run.sh` wrote T0 = 02:32:47Z.
- The preregistered predictions are exactly the predecessor fit, (p/1329)^−2.05.
- The README discloses 3 post-freeze changes to the analysis tool (not the protocol) and the post-hoc variant F.
- The producer's scratch files `ft/fit.*` (05:38Z) show the tool was run mid-campaign. The protocol does not forbid this.

The freeze is taken on the producer's record.

**1.5 Frozen rule** (derived). `fit/src/main.rs` implements exactly the frozen rule:
- geometric mean over the ok seeds;
- reference mean over all correct runs, pooled;
- joint resampling of seeds × runs-within-instance, 20000 resamples, percentile intervals;
- "below" when hi < 1, "above" when lo > 1.

The cut to 3 runs per instance is declared in the frozen protocol.

## 2. Metric definitions

**2.1 Cover S** (derived from source).
- `total_muls` = setup + base + stream + oracle (weil + f4 + lin + post) + verify + la_muls.
- s = total_muls / (c_e·√l) (`jv_cover.rs` 3590–3599).
- **Counters:**
  - `Fq`: an F_{p²} product counts 5, an inverse 21;
  - `f4_fp::field_ops_total` for F4;
  - `FastStats` for the linear stage;
  - la_muls = 16·la_ops.
- **Unpriced:**
  - F_p additions;
  - memory;
  - F4 native operations. These are emitted as `f4_native_ops`, a lower bound of 2.2–4.2·10^10 per run.

**2.2 c_add_e and l** (measured/derived).
- **c_add_e = 331.0** on every instance.
  - It is measured by `unit_costs`: 64 affine additions on the instance, divided by 64.
  - Composition: an F_{p⁶} product is 55, a squaring 55, an inversion 166, and 2M + S + I = 331.
  - The test `harness_price_of_the_weak_walk_is_331` in `jv_rho_strong` asserts this.
- **l and #E:**
  - l passes Miller–Rabin on all records.
  - p^6/(4l) = 1.000000 on all 9 instances, so #E = 4l, as `group_order_4_prime` constructs.
- **Same instances across arms:** the rhodp and rhostrong records carry the same l per (p, seed).
- **rhodp overhead:**
  - group_ops − steps = 23.0k–24.1k.
  - This equals 4 walkers × 32 multipliers plus the starts, where each `ec.mul` is about 90 ops.
  - So the multipliers are computed "once per walker", as stated.

**2.3 Comparability** (derived; this is the central scope finding).

*Inside the unit the arms are consistent:*
- **rhodp:** S = group ops / √l.
- **rhostrong:** s_harness = (55·mul + 55·sq + 166·inv) / (331·√l). Reproduced from its op counts at 330.93–330.95 per step.
- **Cover:** S = F_p mults / (331·√l).

*What would make "below rhodp" false* is 331 not being a fair price for a rho step:
- The same records price a rhostrong step at **137.7 F_p mults in the tuned unit**: TUNED = 24/18/62, which is 0.416 of the harness price.
- Repricing rhodp at that measured ratio gives S/rho = 1.91 / 1.86 / 1.41 (model; rhodp emits no mul/sq/inv counts).
- Against rhostrong's measured `s_tuned` means (0.404 / 0.423 / 0.371), the cover is at 3.24 / 2.55 / 2.19.
- The cover's count cannot shrink the same way: 99.5% of it is base-field F4 and linear-stage products. The F_{p²} stream and Weil work are 0.5%.

*§5a cannot rescue the study:*
- Pricing the cover's emitted lower bound `f4_native_ops` at w = 0.1 GAE raises cover S to 1.9–4.2.
- At w = 1 it raises it to 13–29.
- rhodp moves only to 1.52–1.82.
- So the cover is above both references at every w > 0 (model).

The README says no band ranking is possible and that every number is a w = 0 lower-bound comparison. That is confirmed. In addition, the tuned price alone reverses the sign.

**2.4 Correctness.**
- **What the flags check:**
  - cover: `correct` = (dd == planted d);
  - rhodp: `found == spec.d`;
  - rhostrong: likewise.
  - All three are self-reported by the binary.
- **Independent check on one instance:**
  - p = 1301 s3 was replayed with `JV_DUMP_INSTANCE`.
  - `verify_log.rs` rebuilds the field tower with its own arithmetic (adjugate inversion, Fermat inversion, its own double-and-add):
    - F_{p²} = F_p[i]/(i² − w);
    - F_{p⁶} = F_{p²}[θ]/(θ³ − s);
    - the curve y² = x³ + a₂x² + a₄x.
  - Results:
    - l prime;
    - G and Q on the curve;
    - G ≠ O;
    - l·G = O;
    - dd·G = Q;
    - dd = d = 886293798800496412.
    - **VERIFIED.**
  - Negative control: (dd − 1)·G = Q is false, as expected.
- **Not independently checked:** the other 8 cover runs and all rho runs are accepted on the producer's flag. They use the same code path: a single recovered scalar compared with the planted one.

## 3. Replay

**3.1 Rebuild** (measured).
- `cargo build --release --example jv_cover -j 2` on clean 4def6153d1 took 10 min 15 s.
- **Hashes differ:**
  - rebuilt sha: `50d0be859ad46514891a4d63ce28fd29dcf4769249d03cf2c7c9d2df7364274e`;
  - recorded sha: `4a1284e9a580379a34790a2bf3a1bde7273230b0d5570981986672ef27159752`.
- **Old binaries still present** (measured):
  - the study binary, at its scratchpad path, with the recorded sha;
  - the predecessor binary `a045cb80…e34d`.
- **Cause of the difference** (derived from git):
  - The study binary was built at 02:11Z, when `main` was 692afe2e4c.
  - 39 src files have changed since (13,778 insertions: `prime_orbit_index_calculus.rs`, `ml_dataset/*`, and others).
  - None of these changed: `jv_cover.rs`, `zero_dim_la.rs`, `f4_fp.rs`, `gaudry_cubic.rs`, `residual_walk.rs`, `jv_rho_strong.rs`, `examples/jv_cover.rs`.
  - The study commit's parent, a67ec48348, is src-identical to HEAD.
- The byte difference is expected and immaterial. Behaviour is tested by the replay below.

**3.2 Cover replay** (measured).
- p = 1301 s3: same command as `run.sh`, rebuilt binary, `RAYON_NUM_THREADS=2` (the study used 4). It took 764 s.
- **Every non-time field is identical to the record:**
  - total_muls 409975021213;
  - relation_digest 4667961897523824477;
  - f4_muls 319112344483;
  - lin_muls 87945577261;
  - residuals 428147;
  - relations 663;
  - s 1.1249352074611745;
  - solved and correct true.
- **The one field that differs:** `f4_muls_per_test_sum`, 608792284232 here vs 1199281738823 recorded.
  - It is a per-test difference of a process-wide counter, so it scales with thread count: about 2× `f4_muls` at 2 threads and about 3.76× at 4.
  - The source documents this. `f4_muls` itself is thread-independent.

**3.3 Rho determinism** (measured).
- **rhodp:** p = 251 s1, `--rho-runs 2 --dp-bits 12 --threads 2`, run twice. Bit-identical:
  - steps 18023625 / 9915180;
  - group_ops 18032283 / 9923907;
  - S 2.2807 / 1.2551;
  - correct.
- **rhostrong:** p = 503 seeds 1–2, `--run 0`, run twice. Bit-identical:
  - steps 55785941 / 15031819;
  - s_harness 0.8773 / 0.2370.
- **Not done:** comparing against the recorded 4-walker calibration runs is impossible under the 2-thread cap, because the walker set differs. This is stated rather than exceeding the cap.

## 4. Reproduction-control failure

**4.1 What differs** (derived). `repro_dlp_p1009_s1` against the predecessor's `dlp_p1009_s1`:
- **total_muls:** 429320392173 vs 452313823812, ratio 0.949165.
- **Identical:**
  - f4_muls, 335940657368;
  - residuals, relations, la_ops;
  - the stream, weil, setup, base and verify counts.
- **lin_muls:** 90400879953 vs 113394312494, ratio 0.797226.
- **post_muls:** +902. total − lin differs by exactly 902.

**4.2 Cause** (derived from `git log -p`, then measured).

*The predecessor binary:*
- `a045` (see `results/probe/bin.sha`) is from the 2026-10-24 tree.
- In it, `JV_LIN` defaults to Sparse, and the sparse stage is inline in `solve_zero_dim_mode`.

*The commits since:*
- **5489ca74d8** (2026-10-26):
  - added `zero_dim_la.rs` `fast_stage`: transposed Krylov, Berlekamp–Massey, roots, rational read-out;
  - refactored the incumbent into `linear_stage_from_basis`.
- **b8b4a97983:** made `fast_stage` count executed products instead of formulas. This was amendment 3 of `research/postgb_linear_stage_20261026`.
- **e6ff48ea72** (2026-10-27 05:49Z):
  - switched the default to Fast;
  - `JV_LIN=sparse` keeps the incumbent.

The incumbent's body is unchanged between the predecessor tree and HEAD, except for refactor plumbing and two addition-counter hooks.

*Measured confirmation:*
- **p = 1009 s1, rebuilt binary, `JV_LIN=sparse`** (1055 s, 2 threads):
  - total_muls 452313823812;
  - lin_muls 113394312494;
  - post_muls 692901264;
  - s 2.660531246537169;
  - bit-identical to the predecessor's record. Only new schema fields and times differ.
- **p = 251 s1:**
  - under sparse: total 80020479045, lin 18413836422, which equal the predecessor's probe record;
  - under the fast default: lin 15793800288 (0.858×), total 77400442514 (0.967×).

So the whole 0.949 is the `JV_LIN` default. Relations (same digest), F4 and logs are identical under both stages.

*Classification:*
- It is a real change of algorithm: a different linear stage that finds the same solutions.
- It is also measured under a different counting discipline:
  - fast counts executed products;
  - the incumbent charges several phases by formula, for example cnt += n·δ².
- The post-GB study's own validation found counted sparse ÷ fast ratios of 0.77–2.14 across its corpus (geometric mean 1.13 on jv-nagao). Here the ratio is 1.254 at p = 1009 and 1.166 at p = 251.
- It is neither a pure algorithmic saving nor pure relabelling. It is a new instrument whose linear-stage count is 14–20% lower, and part of that is the stricter count.

**4.3 Comparability.**
- **Per-prime verdicts:** they use no predecessor point. Cover and rho were measured with the same binary on the same instances, so they are internally consistent.
- **The refit:**
  - It mixes instruments.
  - The study applied its frozen consequence ("not comparable", reported as a finding) and the post-hoc variant F (×0.949).
  - Variant F gives p* 1253 vs 1264, and slope −2.08 vs −2.10.
  - The shift is near-uniform, because the linear stage is a stable share of the cover count: 21.4–21.5% at the new primes, and 25.1% at 1009 under the old stage.
- **A consequence the README does not draw** (derived):
  - Under the predecessor's instrument, the new cover S would be about 5.4% higher.
  - The 1451 upper end would become 0.970 × 1.054 ≈ 1.02, which is "not resolved".
  - 1601 would stay below: 0.811 × 1.054 ≈ 0.855.
  - So the 1451 verdict holds for the current pipeline (which was validated separately), not for the predecessor's.

## 5. Calibration

**5.1 Arithmetic** (derived). Reproduced exactly:
- **rhodp at p = 251:** 16 runs, mean 1.5393, sd 0.6659. By seed: seed 1 1.828, seed 2 1.251.
- **rho_e:** 32 runs, mean 1.2638, sd 0.5583.
- **Difference and interval:**
  - difference +0.2755;
  - Welch df 25.85;
  - t.975 = 2.0561, from the validator's own incomplete-beta inversion. The study's table interpolation gives the same interval to 3 decimals.
  - CI [−0.122, +0.673] contains 0, so the frozen rule is met. |diff| < 0.3 also holds.

**5.2 Is it the right control?** (derived from `rho_e_dp`, `jv_cover.rs` 2789–2895)

*Why it is the right pair:* it uses the same instances, the same 32-adding walk and hash, and the same unit, at the only p where rho_e exists.

*The artifact it carries* (a scale-dependent one the protocol missed):
- Each walker polls the shared stop flag only every 2^20 local steps.
- After one walker finds the collision, the other three run on for 0 to 2^20 steps each, and all of those steps are charged.

*How large it is:*
- Expected excess: 3·2^19 steps.
  - At p = 251 that is 0.199 S-units, 72% of the observed +0.276.
  - At 1301 / 1451 / 1601 it is 0.0014 / 0.0010 / 0.0008.
- The DP latency the protocol prices (4 × 2^dp_bits) is 0.002 at 251 and 0.001 at the new primes.
- Net of the lag, the difference is about +0.08.

*Consequences:*
- The control passes the frozen rule but is confounded as a test of "rhodp ≡ rho_e".
- The README's caution (b), that a possible 0.28 upward bias would move the ratios by 25%, attributes the gap to the wrong mechanism. The mechanism that explains the p = 251 excess is absent at the new primes.
- The pooled new-prime rhodp (27 runs, 1.476, se ≈ 0.12) exceeds the predecessor's pooled rho_e (1.344, n = 176) by 0.13, about 1.1 se. That is consistent with no bias.
- A cleaner control would poll the stop flag more finely, or run `rho_e_neg` in-process. Neither was frozen.

PASS on arithmetic, qualified on design. The qualification removes the main reason to suspect the rhodp denominator rather than adding one.

## 6. Falsifiers checked

| Falsifier | Outcome |
|---|---|
| Records → numbers | Two independent recomputations; holds. |
| Frozen rule applied | Yes. |
| Wrong logs | Independent dd·G = Q check on the replayed instance; verified. |
| Irreproducible binary or record | Rebuild plus bit-identical replay; holds. |
| rhodp biased by its implementation | The stop-poll lag and DP latency are ≤ 0.0024 S-units at the new primes. |
| Weak reference | The cover is **not** below rhostrong at any prime, and is above it at 1301. |
| Unfair exchange rate | The tuned price of 137.7 per step reverses the sign (model). |
| Unpriced work | Any w > 0 puts the cover above (model). |
| Interval method flatters n = 3 | Welch-t on ln(S/rho) resolves nothing: 1301 [0.53, 1.19], 1451 [0.52, 1.15], 1601 [0.34, 1.02]. The support in the seed direction has 10 points; the largest multiset ratio is 0.890 at 1451 and 0.810 at 1601. |
| Instrument change | 1451 flips under the old linear stage; 1601 does not. |
| Null object | Not applicable: this is a ratio of two measured solver costs on the same instances, and the in-run rho is the baseline. |

## 7. What is charged

- **Time:** all cover phases, including the Wiedemann linear algebra. For the rho arms, every group op of every walker, plus the multipliers and starts.
- **Memory:** not reported by any arm. It does not bind at l ≈ 2^61, but there is no number.
- **Preprocessing:**
  - setup + base is about 2·10^6, which is negligible;
  - instance generation is uncharged on both sides, which is correct for a given-instance comparison.
- **Oracle calls:** none.
- **Randomness:** seeds are recorded per test (seed ^ idx·const) and per rho run (seed·1000 + run).
- **Success probability:** 1. Every run solved at the first linear-algebra attempt, and no walk was capped. The incomplete tests are charged.
- **The one uncharged term that matters:** native work, uncharged on both sides. At any common weight, the cover's share is larger by an order of magnitude.

## 8. Independent recomputation

**Recomputed by the validator** (`recompute.rs`, `verify_log.rs`, `cmp_dlp.sh`; outputs in `receipts/`):
- every per-run S of all arms, and of the calibration;
- the component sums;
- l primality and #E = 4l;
- the harness and tuned price vectors against the rhostrong op counts;
- geometric means, and reference means and sd;
- the per-prime ratios;
- percentile bootstraps (own RNG; 20000 and 10^6 resamples);
- the Welch interval, with its own t quantile;
- refit A's slope, intercept and p*, with a cluster bootstrap;
- §5a native counts on both sides;
- the `rho_e_dp` stop-poll and DP-latency biases;
- the reproduction ratios;
- the fast/sparse ratios at p = 251 and p = 1009.

**Re-verified with a disjoint checker:** the solved log of p = 1301 s3. The checker has its own tower arithmetic, adjugate inversion and group law, and shares nothing with `jv_cover.rs`.

**Replayed or rebuilt:**
- `jv_cover` on the current tree. The sha differs, but behaviour is identical: bit-for-bit replay of p = 1301 s3 at 2 threads.
- The `fit/` tool: byte-identical outputs.
- rhodp and rhostrong: repeated invocations were bit-identical.
- `JV_LIN=sparse` reproductions of the predecessor's p = 1009 s1 and p = 251 s1 records: bit-identical.

**Taken on the producer's word:**
- (a) That the F_p counters (`f4_fp`, `Fq`, `FastStats`) count what the arithmetic executes.
  - Their mutual consistency was verified: the component sums, and the thread-independence of `f4_muls`.
  - F4 was not re-implemented.
  - Prior independent validations exist (the postgb study, 151 numbers).
- (b) `correct = true` on the 8 cover runs that were not replayed, and on all rho runs.
- (c) The pre-data freeze.
- (d) The 27 recorded rhodp runs at the new primes. They were recomputed, not rerun: the budget did not allow it, and the study used 4 walkers against this validator's 2-thread cap.
- (e) The walls and host facts.

## Scope restated

- **Cover:** the genus-3 JV cover of E(F_{p^6}) over F_{p^2}.
- **Class:** the `generate_spec` weak class — full rational 2-torsion, #E = 4l, l prime ≈ 2^60.1 / 2^61.0 / 2^61.9.
- **Instances:** p = 1301 / 1451 / 1601, seeds 1–3.
- **Solver:** `jv_cover --exp dlp`, with the fast post-GB linear stage, F4 max degree 24, 600 s per test.
- **Reference:** rhodp — a 32-adding walk with no negation map, 4 walkers, dp 18, 3 runs per instance.
- **Unit:** harness F_p multiplications at 331 per addition, w = 0.
- **Interval:** the frozen percentile bootstrap.

This is not cryptographic scale. The study presents it as a weak-class lower-bound comparison, which respects AGENTS.md rule 4.

**Left unresolved by this validation's budget:**
- the 27 rhodp runs were not re-measured;
- the other 8 cover runs were not replayed;
- memory is unreported.
