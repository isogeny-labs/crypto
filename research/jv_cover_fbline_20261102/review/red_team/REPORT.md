<!-- Written by the main session from the red team's returned text, verbatim apart from this line: the red team's harness blocked it from writing this file. The YAML report it returned is in red_team_report.yaml. -->
# Red team: factor-base line search for the JV cover (`research/jv_cover_fbline_20261102`)

Report id `RT-20261102-172964`. The suffix is a random 6-hex value, because `allocate_id.py` has no RT type.

Snapshot reviewed: `main` at `5f3b5c209a`, which contains PR #3286. `src/cryptanalysis/jv_cover.rs` differs from the study's build commit `0c2258ead2` only by rustfmt line breaks. The review was read-only. I did not read the sibling validator report; I saw only its commit subject.

Labels: **measured** means read from a run record. **Derived** means computed in Rust from records. **Model** means an extrapolation or assumption, named as such. **Exploratory** means a red-team pilot that is not frozen and is not evidence.

Provenance:
- My binaries link the compiled `crypto_lib` at main: rlib sha256 `c8a3a56c…6e41`, rustc 1.97.0.
- `line_census` sha256 `e91c947b…09cf`.
- `sieve_transfer_pilot` sha256 `ded3fc4d…2724`.
- `fbline_rt_records` sha256 `6afb3178…cd60`.

## Verdict per claim

| # | Claim | Verdict | Numbers |
|---|---|---|---|
| 1 | 1.71× on the 9 instances | survives | Geo-mean 0.5865 measured, 0.5872 model. t(8): ×1.705 [1.499, 1.939]; the bootstrap gave [1.54, 1.89]. |
| 2 | Not regression to the mean | survives | Plain-line percentiles 0.03–0.88 (mean 0.507). Mean z(B_plain) −0.02. Fair speed-up ×1.710 against ×1.705 realised. |
| 3 | Rate ∝ (2B/p)⁶; per-test cost unchanged | survives | Paired log(rate/model), line − plain: +0.005, t(8) = 0.45. F4 per residual ×1.0001; linear stage ×1.0011. |
| 4 | Search fully charged | survives | 0.51–0.57% of each run. Native lookups 0.05–0.23 GAE/√ℓ; §5a band unchanged. |
| 5 | U1 below rho (0.791 / 0.659 / 0.548) | survives arithmetically; downgraded in meaning | Rho is the full-automorphism rho. t(2) still below. But a rho step is priced at the schoolbook 331, 2.40× the tuned 137.7, and §5a strict admissibility fails (unemitted cover counters). |
| 6 | U2 above (1.900 / 1.585 / 1.318) | survives; like-for-like unit | τ* = 0.526 / 0.631 / 0.759. |
| 7 | Rho reference matched and strongest | survives | Same ℓ and c_add_e on 126 runs; `--neg 1 --r 256 --batch 16`. j ∉ F_{p²} ∪ F_{p³}, j ≠ 0, 1728 on all 9 instances. No Frobenius speed-up, no √3. |
| 8 | Framing "passes the negation rho" | refuted as framing | The in-tree sieve variant has S = 0.00615 at p = 1511. The line arm costs 83–100× more (21–25× more than the sieve's S⁺). |
| 9 | p* at slope −2.10 | weakened | d ln G / d ln p = −0.26, so the slope is about −1.84. U2 p* ≈ 1845–1865 (not 1799); U1 p* ≈ 1145–1157. |
| 10 | Statistics | survives with a caveat | t-intervals wider, verdicts unchanged. sd of log(measured/model) 0.035, below the 0.054 count-noise floor. |
| 11 | "Controls (all pass)" | minor objection | `analysis/rescore_fit.json` has `controls_pass: false` ("consistency controls … FAILED"), expected on line inputs but not disclosed. The byte-for-byte reproduction and the re-score command are not recorded. |
| 12 | "Tuned F4 is the remaining lever" | under-supported closure | Untested: sieve route, large primes (`ic_cover_leads` L6), an exhaustive table line search (+×1.075), other F_p-sublines. |

## The strongest objection: the improved arm is not the frontier for this class

**What the study improves.** It speeds up `run_cover_dlp_mode`, which finds relations with Nagao six-point tests only.

**The repository's sieving variant of the same route.** `jv_sieve.rs` implements [JV12] §3.2 with:
- the same `generate_spec` class,
- the same F_p-abscissa factor base (mean B = 756 at p = 1511),
- the same Jacobian and the same six-point test for the descent.

**Same unit, measured (V2 T1, 16 seeds).** The sieve route has S = 0.00615 [0.0055, 0.0069] at p = 1511 and S⁺ = 0.0248. The line-search arm has S = 0.615 at p = 1451 and 0.511 at p = 1601:

| comparison | at p = 1451 | at p = 1601 |
|---|---|---|
| line-search arm S / sieve S | ×100 | ×83 |
| line-search arm S / sieve S⁺ | ×25 | ×21 |

**Pareto.** Both routes have the same factor base and relation count, so memory is comparable. The line-search arm is dominated.

**Bit operations.** `research/ic_acct1_cover_bitop_20261021` measured the sieve route's whole method at 0.0098 [0.0069, 0.0144] of the strong rho at p = 1511.

**These numbers are current.** My pilot reproduced V2's plain `total_muls` on 8 of 8 seeds at current main. Leaderboard Table D withholds only the sieve route's *rho* comparison; this comparison is cover against cover and needs no rho.

**Exploratory pilot.** The study's unchanged `search_fb_line` placed in front of `run_cover_sieve_dlp_on`, at p = 1511, seeds 17–24, all runs correct:

| search size | route only | search charged | search share of the line arm |
|---|---|---|---|
| n = 65,536 | 0.571 [0.490, 0.664] (×1.75) | 1.082 [0.930, 1.259] (×0.92) | 47% |
| n = 4,096 | 0.670 [0.554, 0.812] (×1.49) | 0.702 [0.579, 0.850] (×1.42) | 4.5% |

## Hidden assumptions and the random model (claims 2 and 3)

**Census of all lines.** I computed B for every square-direction line a·F_p + b: 0.85–1.28 M lines per instance.
- **Replication.** Every `b_plain`, `b_best` and `best_index` matched the records, using the study's own RNG stream, on 9 of 9 instances.
- **Distribution.** Mean ≈ p/2 (650.50 / 725.50 / 800.50). sd = √p/2 to within 0.2%. Skew within 0.004 of zero, excess kurtosis within 0.009. The Gaussian random-line model holds where it is used.

**Extreme values.**
- z(B_best) averages 4.22 ± 0.18; extreme-value theory predicts 4.29 ± 0.28.
- The exact census expectation of the maximum of 65,536 lines at p = 1301 is 727.3 / 726.6 / 727.0, against observed 728 / 725 / 730.
- P(max ≥ observed) lies in 0.27–0.96.

**Were the plain instances unlucky?** No.
- Percentiles of the plain line: 0.18, 0.51, 0.74, 0.69, 0.57, 0.12, 0.03, 0.84, 0.88.
- Their mean, 0.507, is consistent with uniform (0.5 ± 0.096).

**Fair speed-up.** E[residuals for the plain line] / E[residuals for the best of 65,536], search charged:
- ×1.710 overall (geo-mean);
- ×1.757 at p = 1301, ×1.709 at 1451, ×1.666 at 1601.

**What the quantity should do as the destroying parameter grows.** The gain comes from σ_B/μ_B = 1/√p, so it should decay like (1 + 4.29/√p)⁵ (model): ×1.57 at p = 2039, ×1.23 at p = 10⁴, ×1.07 at p = 10⁵. The measured per-prime ratios 0.563 / 0.569 / 0.630 decay as predicted. A random-line null would give a ratio of about 1 (model), since the plain line is a typical line. C2 already passed as the machinery null.

**Rate law and per-residual cost.**
- Rate against the falling-factorial model 64·C(B,6)/p⁶ (geo-means): 0.981 plain, 0.987 line. Choosing the max-B line carries no rate penalty.
- F4 multiplications per residual: 745.3–745.6 k on both arms.
- Linear stage: 205–206 k on both arms.
- Weil, stream and post work: +2–4% (more decompositions per residual; under 0.6% of the total).
- The sum of phase counters equals `total_muls` exactly on all 18 records.

## Cost model (claim 4)

**What is charged.**
- Per line: p(2·7 + 3) multiplications.
- Once: (p−1)/2 for the square table.
- 21 per `line_poly` inversion.
- The `line_div` moves of G and Q.
- Total 1.47–1.81·10⁹ per run, consistent with C2's 22,772 for a single line (17p + (p−1)/2 plus a few).

**What is not charged, and does it matter.**
- Native: 65,536·p table lookups, 0.05–0.23 GAE/√ℓ for 1–3 native operations per point. U1 stays below in every band cell; worst case 0.852 / 0.710 / 0.590 at w = 1.
- Point counting in `generate_spec`: instance generation, uncharged on both arms.

**The search is overpriced (a candidate, not evidence).**
- A χ(h) table over F_q costs about 38q multiplications (8.7·10⁷ at p = 1511), 20× less than the study's search.
- After that, each line costs only additions and lookups.
- The exhaustive census ran in seconds per instance on 4 cores.
- The global maximum adds a further ×1.075 (×1.025–×1.111).
- Its p³/2 lookups must then be priced in the §5a band.

## Units (claims 5 and 6)

**U1 is unfair to rho by up to 2.40×.**
- U1 converts the cover's work at the harness price of a rho step: 331 F_p multiplications (schoolbook F_{q³} product 55, per-step inversion 166).
- The tuned step costs 137.7 (s_tuned/s_harness = 0.4161).
- The cover's work is 77% F4 and 21% linear stage, all native F_p, with no extension-field arithmetic to discount.

**U2 is close to like-for-like.**
- `f4_fp` matches msolve wall time on generic benchmark systems (#1717).
- The linear stage is already the sparse/fast variant.
- `--trace` replay is a measured regression (`ic_cover_leads` L3), and `--stop` staircase saves nothing (L4).
- What remains is unmeasured: F5 criteria (≈1.08×, heuristic) and an early F4 stop (at most 37% of F4 rows, speculative).

**Tuned-cover model** (labelled model): tuning factor τ_c ∈ [0.65, 1.0], so tuned cover against tuned rho is U2·τ_c.

| p | tuned / tuned | reading |
|---|---|---|
| 1301 | 1.24–1.90 | above |
| 1451 | 1.03–1.58 | above to parity |
| 1601 | 0.86–1.32 | around parity |

**F_p additions** (model: the cover emits none and makes about r_c ≈ 1 addition per multiplication; rho emits 45.2 per step).

- **Physical price, 0.1–0.3 of a multiplication:**

  | unit | p = 1301 | p = 1451 | p = 1601 |
  |---|---|---|---|
  | U1 | 0.86–0.99 | 0.72–0.82 | 0.60–0.69 |
  | U2 | 2.02–2.25 | 1.69–1.88 | 1.40–1.56 |

- **Additions priced as §5a native operations:** U1 flips to above at w = 0.1 (3.94 / 3.28 / 2.73).
- **Admissibility:** §5a says an arm with an unemitted named counter "cannot be ranked", yet the README declares "the order holds".

**Cheapest discriminating control.** Run one line-search instance per prime with the `jv_bitops` `bitop/v1` tally on. The tooling exists (`examples/jv_cover_ic_vs_rho_bitops.rs`).

## Rho reference (claim 7)

**Matched:**
- `l` and `c_add_e` equal on all 126 runs.
- Flags `--neg 1 --batch 16 --r 256 --dp-bits 16`.
- Positive control z = +1.15.

**Automorphisms** (measured):
- On all 9 instances, j(E) is in neither F_{p²} nor F_{p³}, and j ∉ {0, 1728}.
- The q-power Frobenius maps E to E^σ, which is not isomorphic to E because j^q ≠ j. It is not an endomorphism of E, so there is no order-3 class speed-up.
- There is no GLS map ψ with ψ² = −1, because j ∉ F_{p³}.
- So Aut(E) = {±1}, and the negation map is the full-automorphism rho.

A hypothetical √3 would have given U1 = 1.37 / 1.14 / 0.95; it does not exist. Rho slack (model): about 15% at most from a larger inversion batch, which would only widen U2 against the cover.

## Statistics (claim 10)

- **Paired ratio:** the t(8) interval 0.5865 [0.5156, 0.6671] is wider than the bootstrap's [0.5301, 0.6508]. The conclusion is unchanged.
- **H1's "every instance cheaper" is almost guaranteed by design:** the plain line is candidate 0, so B_best ≥ B_plain. The informative test is the model match.
- **Per prime, t(2) on the cover side combined with the reference standard error:**

  | p | U1 | U2 |
  |---|---|---|
  | 1301 | 0.791 [0.677, 0.924] | 1.900 [1.626, 2.220] |
  | 1451 | 0.660 [0.594, 0.732] | 1.585 [1.428, 1.759] |
  | 1601 | 0.548 [0.498, 0.604] | 1.318 [1.197, 1.450] |

## Crossover (claim 9)

The README applies the plain arm's slope, −2.10, to the line arm. The line-search gain decays with p (d ln G / d ln p = −0.26), so the line arm's slope is about −1.84 (model). Anchored at each measured prime:
- **U2 p*:** 1844 / 1864 / 1860, against 1766–1826 at slope −2.10 and the README's 1799.
- **U1 p*:** 1145–1157.

The planned run at p ≈ 1850–2000 still fits inside the u64 limit.

## Narrowest supported statement

**Setting.** On the 9 `jv_cover` weak-class instances (p = 1301, 1451, 1601; genus-3 cover; #E = 4ℓ; never prime order), choose the best of 65,536 square-direction F_p-lines, with the search charged.

**Effect.** This cuts the cost of the Nagao-only arm by ×1.71 [1.50, 1.94].
- The cut matches the B⁻⁵ model; the rate law and the per-test cost are unchanged.
- The fair expected gain for a random instance is ×1.71.
- It decays as (1 + 4.29/√p)⁵ (model).

**Against the matched negation rho,** which is the full-automorphism rho:
- In like-for-like F_p multiplications (U2), the arm is **above**: 1.90 / 1.58 / 1.32.
- In harness schoolbook-priced group additions (U1), it is **below**: 0.79 / 0.66 / 0.55. That is a lower-bound comparison, because the cover's additions and pair selection are not emitted.

**Frontier.** The arm is 83–100× costlier than the in-tree sieve variant of the same cover route. Until the lever is shown on that route, this is a component improvement of a dominated arm.

## Next concrete action

**What to freeze.** A protocol applying the line search to `jv_sieve::run_cover_sieve_dlp_on`.

**Search cost.** Charge it in full, and either:
- use the χ(h) table (about 38q multiplications plus n·p native lookups, priced in the band), or
- fix n by a rule set before any data.

The pilot shows n = 65,536 is a net loss there, and n = 4,096 gives ×1.42 [1.18, 1.73].

**Instances and options.** V2's: p ∈ {1009, 1511, 1823}, seeds 17–32, `--stop 64`, sparse. The plain arm must reproduce V2 exactly.

**Pricing.** Both arms in `bitop/v1` through the existing `jv_bitops` hooks, against the matched strong rho with the §5a band.

**Before any U1 ranking of the Nagao arm.** Emit its F_p additions.
