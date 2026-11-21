# Audit of rho references behind "IC at or below rho" rows: steps 1-2 (KB check, enumeration)

Date 2026-11-21. Read-only. No repository file was edited. Every number below is copied from the cited
file and is an **unverified assertion** (no pcc claim id was checked in this audit).

**Scope.** The relayed user request was "1-2". Where it conflicts with the computed task, the user request
wins, so this audit runs **steps 1 and 2 only**: the knowledge-base check and the enumeration of rows.
For each row the table gives the claim, the ratio and the unit, plus the rho reference *as the row itself
states it*. Steps 3 to 5 were **not** done: no per-step price provenance, no strong-rho factor estimates,
no ROBUST / AT-RISK / UNKNOWN classes and no rerun plans. The "flags" column only records problems that
the repository's own documents already state. These flags are pointers for step 3, not assessments.

## Step 1. Knowledge base

- The MCP server `research-kb` (`kb_status`, `search_knowledge`) is **not available** in this session.
  ToolSearch for "research-kb search_knowledge" found no matching tool.
- The CLI fallback is also unavailable: `scripts/research-kb status` failed with
  `research-kb: no executable found; set RESEARCH_KB_BIN or put research-kb on PATH`.
- **No source ids were recorded.** The KB review is marked unavailable (per skill `research-kb` §6). As a
  substitute I searched the repository with grep and read the files directly. The most relevant prior
  audit this found is `research/koblitz_rho_reference_20261115/README.md` (see the flags below).

## Step 2. Sources enumerated

- `docs/ic/leaderboard.json`: walked every section except `roster`. I collected every numeric field whose
  key contains `rho` with a value at or below 1.5, and every string containing "below rho". The raw
  output is `lb_low_ratios.txt` in this directory.
- `docs/ic/LEADERBOARD.md`: 1634 lines, all sections. Cited lines are those at the time of reading.
- `benchmarks/README.md`: 4484 lines, searched for below-rho, IC-lower and "of ideal/matched rho".
- `VALIDATION.json`: 191 files found with `find . -name VALIDATION.json` (list:
  `validation_files.txt`). Their `rho_reference` blocks were summarised. Only the blocks behind
  below-rho rows are quoted below.
- `research/*/`: 772 study directories mention rho comparisons. 174 contain a "below … rho" phrase
  (`below_rho_dirs.txt`). 27 carry such a phrase in the first 25 lines of their
  REPORT/README/RESULT (`headlines.txt`). **Revision 2.** The first version tabled only part of them; every
  one of the 27 now has a disposition in the section "Headline triage" below. Rows 28-36 were added in this
  revision (two verifier findings, the projective-law per-slice cells, and five further positive or
  partially positive headlines found by checking every headline directory against the table).
- `docs/ic/SUCCESS_CRITERIA.md` tiers (lines 101-238). **No row in the lab claims T1, T2 or T3.**
  - T1 needs both of the following:
    - cold, one public target, the interval below 1 at every weight of the §5a band, ≥ 32 targets
      below b = 64;
    - controls, the validation gate, a cross-host audit receipt, pcc `verified` and
      `validator-breakthrough`.
  - Line 148 cites the genus-3 positive control (`research/ic_positive_control_20261112/`) explicitly as
    "would be T1 at neither reading". The other rows below are T0, component, accounting, online-only,
    weak-class, or Jacobian-DLP diagnostics (Jacobian DLP is not an instance of SUCCESS_CRITERIA §1).

### Key to the "rho as stated" column

- **MEAS**: rho solves were actually run (the measured walk, in counted operations or in wall time).
- **IDEAL**: a closed form such as sqrt(pi r/4), or sqrt(pi r/2) / sqrt(pi r/2A), times a per-step price.
- **Counted**: the ratio is in counted units (GAE, F_q-mult equivalents (FPM, M-eq), U2, bitop/v1 or Ir),
  not wall time. Threads, SIMD and batching do not enter a counted unit directly. They enter only through
  what the rho walk counts: the group law used, the setup it charges, the DP/restart overhead and its
  automorphism fold.
- **Wall**: the ratio is a same-host wall-clock comparison. This is the regime of the genus-5 failure mode.

## The table (steps 1-2)

| # | Row (where) | Claim and ratio as written | Unit | Rho as stated by the row | Standing as stated | Flags already in the repo (pointers for step 3) |
|--:|:--|:--|:--|:--|:--|:--|
| 0 | **Genus-5 Humbert cover** (reference case). `research/humbert_genus5_cover_20261021/slope_20261120/REPORT.typ` L8, L60-73. `next_leads_20261120/REPORT.typ` L8, L61, L85-87 | Now **above** rho: IC/SIMD rho 3.89 / 1.97 / 2.00 / 1.37 at q = 16001 / 32003 / 48017 / 65521. Against the v7 4-thread rho: 0.31-0.46 for q ≥ 32003. Against the earlier scalar rho: 0.04-0.09 | **Wall**, 4-vCPU shared VM, L0 | **MEAS**, three generations: (a) scalar Brent `rho_s`, about 1.8 M steps/s, single thread; (b) `rho_m`, 4 threads, DP, 64 batched affine walks, Barrett K4, negation, about 25 M steps/s; (c) `HUMBERT_RHO_SIMD` AVX-512, 86-133 M steps/s | No VALIDATION.json and no leaderboard row (L23) | "below rho" disappeared twice (rho_s → rho_m → SIMD). The report notes that rho is still not finished (L99-100). §5a: the order flips across the band (L78-80) |
| 1 | **Genus-4 prime-order** `benchmarks/README.md` L87. `research/genus4_prime_order_20261118/RESULTS.md` L9-21. `benchmarks/20261118-genus4-prime-order/VALIDATION.json` | x*/ideal negation rho **0.876 [0.872, 0.880]** (p = 16411) and **0.640 [0.638, 0.641]** (p = 32771) at w = 1. Below at w = 0 and 0.1 (0.003 / 0.090 and 0.002 / 0.066). Arm x: 0.999 / 0.769 | **Counted**: 1 Jacobian group op = 1; F_p and mod-r native ops at w each | **IDEAL**: `R_neg = sqrt(pi r/4)` group ops, nothing charged. Measured plain rho Q, 5 runs at p = 16411 only, as a sanity check (PROTOCOL L54-56). The VALIDATION `strongest_available: true` | validated | The IC side is counted ops, not threads. ERRATA E1: per-step walk bookkeeping not counted (x* stays below at 0.885). Hyperelliptic, not ECDLP. No wall-time comparison was made |
| 2 | **Genus-4 follow-up** `benchmarks/README.md` L88. `research/genus4_followup_20261118/RESULTS.md` L16 | Arm x / plain rho at w = 1: **0.874 [0.767, 0.9996]**, "fragile". Against ideal negation: 0.126 at w = 0.1, **1.240** at w = 1, so the §5a order does not hold | Counted | MEAS plain rho (Q3, Q4) and IDEAL sqrt(pi r/4) | validated | Upper end below 1 for only 18 of 50 bootstrap seeds (README L88) |
| 3 | **Genus-4 scale / cheap IC / lean rho** `research/genus4_scale_20261116/RESULTS.md` L30. `genus4_cheap_ic_20261117/RESULTS.md`. `genus4_rho_refs_20261116`. `genus4_lean_rho_20261116` | Scale: arm w below rho at all w under the frozen references, then **reversed**: against plain rho without its unused window, w = 1 gives 1.05-1.15. Cheap IC: no arm below rho at w = 1, every arm below at w = 0.1 | Counted | MEAS plain rho. A cheaper rho (window-free) was found by the validator | per study | This is the counted-unit version of the genus-5 story: removing the window from the rho walk flipped w = 1 |
| 4 | **Genus ladder g = 2-5 + quartic** `research/genus_ladder_20261114/RESULTS.md` L52-60 | At p = 1031, IC/rho: g2 2.82, **g3 0.75 [0.46, 1.40]**, **g4 0.117**, **g5 0.0084**, **plane quartic g3 0.027** | **Counted** FPM (F_p mults) | MEAS negation-map rho. Group law: **Cantor**, 90 / 219 / 393 / 693 FPM per step at g = 2 / 3 / 4 / 5 (L62-64) | per study | Rho is priced through Cantor. Explicit genus-2/3 formulas are much cheaper per step, so a counted reference here depends on the group law. Not checked |
| 5 | **Genus-3 lane, ladder scale** `research/genus_ladder_scale_20261115/RESULTS.md` L31-43 | 2LP+pre / rho **0.591** (8209), 0.741 unresolved (16411), **0.537 [0.407, 0.686]** (32771) | Counted FPM | MEAS negation rho, 6 runs per prime (exponent 1.413 [1.145, 1.751]) | per study | The GAE band is "unresolved at every prime (above at w = 1)" (L40-41). Same Cantor-pricing question as row 4 |
| 6 | **Genus-3 LA exponent (blindspots)** `benchmarks/README.md` L107. `benchmarks/20261018-blindspots-genus3-la-exponent/VALIDATION.json` | Every arm below rho at w = 0 (0.38×). Above at w = 0.1 (1.6-2.6×) and at w = 1. UNRESOLVED | Counted GAE (c_add = 81 F_q-mult-eq per affine E0(F_{q^3}) add) | MEAS round-4 `rho_neg` + canon, r = 64 adding walk, DP, 16 walks per curve | validated; `order_holds_across_band: false` | Below only at w = 0 |
| 7 | **Hyperelliptic bitop re-price g = 2-4** `docs/ic/LEADERBOARD.md` L1164-1201. `leaderboard.json/hyperelliptic_bitop_reprice`. `research/hec_bitop_20261020/` | IC/rho bitop/v1 **0.114-0.767** on all 18 rows (g4 p = 61: 0.114) | Counted bitop/v1 | MEAS full-width rho walk (`hec.rho_full_width`, `pollard_rho_jacobian_with(DistinguishedPointsFullWidth)`), 40 runs per row | accounting; outside tables A-C | **Post hoc without rho's fixed precompute the ratio is above 1 on 13/18 rows** (1.10-4.43; g2 all, g3 at p ≤ 41, g4 at p ≤ 23). `rho_precompute_share` 0.876 at g2 p = 41. `hec_bitop_20261020/README.md` L47 qualifies the small rows |
| 8 | **Genus-2 g = 6-10 degree-2 base** `research/ic_genus2_16_speedup_20261117/README.md` L33-38 | "puts it below rho from p = 31" at g = 7-10. Crossover p* about 270 / 830 at g = 6-8 | Counted | **IDEAL**: "expected matched rho" | `[pcc:unverified]`, not validated | Ideal-formula reference. Per-step price provenance not read |
| 9 | **Generator-orbit Koblitz n29** `docs/ic/LEADERBOARD.md` L120-136. `research/ecbench_generator_orbit_20261022/VALIDATION.json` | At n29 (log2 r 15.4) the IC is below rho-frob at every band weight: 0.546 / 0.611 / 1.196 against rho 1.569 (S) | **Counted** ecbench GAE | MEAS `rho.signed_frobenius_full` (rho-frob), same session. rho-strong also run. `strongest_available: true` | validated | **Already contradicted**: `research/koblitz_rho_reference_20261115/README.md` L10-12 gives IC/rho at w = 0.1 of 0.142 vs strong but **1.69 vs v5**, so "below at every band weight does not survive v5". rho-frob is itself 0.26-0.40× strong at n ≤ 31. Rho setup dominates at r = 2^15.4 |
| 10 | **Lazy columns, rho-strong** `docs/ic/LEADERBOARD.md` L138-150 | frozen / rho-strong **0.325** (n13), **0.602** (n17), **0.396** (n29) | Counted GAE | MEAS `rho.signed_frobenius_strong_full` | not validated (pre-gate) | The koblitz_rho_reference audit says v5/strong is 0.079-0.168 at log2 r ≤ 20.5, so these rows are understated by roughly 6-12× (its §2 table) |
| 11 | **Rootfind speed-up n13** `research/ecbench_rootfind_speedup_20261014/REPORT.typ` L13 | Complete IC below rho-strong at d = 4: **0.770 [0.720, 0.824]** | Counted GAE | MEAS rho-strong | per study | Same strong-rho setup issue at log2 r ≈ 11 (v5/strong at small r is about 0.08) |
| 12 | **K1 pair-table rows** `docs/ic/LEADERBOARD.md` L192-209 | Above rho (gap 5.6-9.9 bits). Listed because `s_rho` = 0.908 at w = 0 is rho's own S, not a ratio | Counted GAE | MEAS `rho.signed_frobenius_strong` | validated | The koblitz audit says the distance is understated (n29 recipe 766× vs strong, 9134× vs v5). This is not a below-rho row |
| 13 | **JV cover sieve, weak class** `docs/ic/LEADERBOARD.md` L169-190. `research/jv_sieve_line_vs_rho_20261103/VALIDATION.json`. `research/jv_walk_charge_20261111/REPORT.typ` | IC/rho **0.008-0.066** (U2). Pooled with the walk charged: 0.0155 / 0.0858 / 0.0900 at w = 0 / 0.1 / 1 | **Counted** U2 = textbook F_p mults / 331 per sqrt(l) | MEAS `jv_cover --exp rhostrong --neg 1 --batch 16 --r 256 --dp-bits 16`, same binary | validated | Weak class only (F_{p^6} with genus-3 cover). Margin about 11-60×. The earlier `jv_cover_crossover_20261101` "0.775 below measured rho at p = 1451" (README L16) was superseded by `jv_cover_negrho_rescore_20261101` (L11: the cover costs **more** than negation rho) |
| 14 | **IC cover weak class / Hilbert stop** `docs/ic/LEADERBOARD.md` L154-167. `research/ic_cover_weakclass_20261023/VALIDATION.json`. `benchmarks/20261029-jv-cover-hilbert-stop/VALIDATION.json` | Weak-class cover components measured against rho | Counted | MEAS `jv_cover --exp rhostrong` (neg, batch 16, dp 16) | validated with caveats | Same reference as row 13 |
| 15 | **Weak-class cover in bitop/v1** `research/ic_acct1_cover_bitop_20261021/REPORT.typ` L7, L20, L46 | Does the weak-class cover row stay below rho with both arms executed (bitop/v1)? | Counted bitop/v1 | MEAS (both arms executed) | accounting | Verdict line not extracted |
| 16 | **GHS-weak E0/F_{q^3} N4 (B0)** `docs/ic/LEADERBOARD.md` L818-852 | B0/rho **0.735 → 0.122** for q = 4051…262111 ("IC lower") | Counted M-eq | MEAS **plain rho, no negation map** (L820) | advance (restricted family) | The reference lacks the negation map by construction, a known factor of √2 in steps. Row 17 re-ran with negation rho |
| 17 | **GHS-weak N4 v2 and DL1 headline** `docs/ic/LEADERBOARD.md` L900-935, L1061-1072. `research/ic_gw_nrm_lz_20261115/validation/VALIDATION.json` | DL1 / rho-neg **0.00116 / 0.00125 / 0.00143** at w = 0 / 0.1 / 1. B0/neg 0.403 at q = 65479 and 1.19 (unresolved) at 4051 | Counted M-eq with per-size r_inv, r_lmul | MEAS `rho2::rho_neg` at k = 22 from the frozen supplement (`quartic_n4v2_20261020/results/summary_supp.json`). Replayed field for field | validated | Special GHS-weak family. Margin about 700-860×. rho at k = 22 is "the frozen band", so how rho's cost is extrapolated to the headline size needs reading |
| 18 | **Multi-target IC vs multi-target rho** `docs/ic/LEADERBOARD.md` L726-742 | IC/rho **0.713-0.799 at h = 1**, 1.06-1.16 at h = 0, unranked | Counted field layer (I = 10M, S = M) + h·GAE natives | MEAS cheapest of Kuhn-Struik retention rho and DP rho, 10 runs per configuration | unranked by §5a; multi-target class | Not single-target. Earlier rows (v2, v3) used rho without retention |
| 19 | **N83 Tier 1 finalist, online** `leaderboard.json/n83_tier1_finalist`. `research/n83_e2e_finalist_results_20261017/RESULT.md` | Paired online IC/rho **0.042-0.088**. Cold mean / rho online **47-60×** | **Wall**, Tier 1 cloud instance | MEAS same-point rho online wall | online-only (never supports T1) | Online IC (reused setup) against rho with no precomputed table. The equal-advice control is `benchmarks` L133 (`20261016-n83-precomputed-rho-control`, Bernstein-Lange table). LEADERBOARD Table A L237 says retained-setup claims were superseded by equal-advice rho controls |
| 20 | **n37 online Ir diagnostic** `leaderboard.json/n37_online_ir_diagnostic` | k16 / rho online Ir **0.153 [0.092, 0.240]** | Ir (valgrind instructions) | MEAS rho online Ir | diagnostic | Online only. Cold counted IC/rho is 5.03 (LEADERBOARD L472) |
| 21 | **Gaudry k = 4, char 2** `research/gaudry_k4_char2_crossover_20261015/REPORT.typ` L7. `research/gaudry_k4_char2_redteam_20261026/REPORT.typ` | IC falls from 3.6× to **0.45× of rho's theoretical cost** at m = 15-18 | Counted | **IDEAL** ("rho's theoretical cost") | crossover "indicated, not established" | Red team: no clean IC run at m ≥ 16. The m = 17 record came from a killed run |
| 22 | **Weak points sweep** `research/ic_weakpoints_20261102/README.md` | IC / rho (analytic negation map √(πℓ/4)): **w = 0: 0.29-3.4**, w = 0.1: about 10^4-10^5 | Counted | **IDEAL** √(πℓ/4) | per study | Below only at w = 0 |
| 23 | **SAT PDP pricing** `research/sat_pdp_pricing_band_20261111/REPORT.typ` | SAT below rho only at w = 0. The order flips in the band | Counted | per study | unranked | Below only at w = 0 |
| 24 | **Curve25519 F1 target walk** `research/curve25519_f1_targetwalk_20261108/REPORT.typ` | Below rho-neg at w = 1 only, described as "an artifact of what each arm lists as unpriced work". 1.40× at w = 0 | Counted | MEAS rho-neg | per study | Self-retracted |
| 25 | **Prime precompute regime** `research/prime_precompute_regime_20261028/README.md` L11, L182 | Table online cost 0.117 below rho only after target-blind precomputation | Counted | per study | online / precomputation | Equal-advice question. Not read further |
| 26 | **Weil descent n = 2 / round-4 genus-2 IC** `benchmarks/README.md` L114-115 | IC 0.27-0.44·p Jacobian ops vs rho (h = 1) | Counted, 1 GAE = the run's measured affine E(F_{p^2}) add at I = 3M | MEAS rho (arm R), `examples/n2_routes.rs` | validated (VAL-20261024-wd3/wd4) | Text truncated in README. Needs reading for the exact rho/IC ratio |
| 27 | **GHS cubic Tate bridge** `research/ghs_cubic_tate_bridge_20261026/REPORT.md` L22 | Relation/decomposition and descent cost "below rho" | not extracted | not extracted | per study | Headline only, not read further |
| 28 | **GHS-weak N4 iteration 2, B2 (double large primes)** `research/quartic_n4_la_20261021/REPORT.typ` L7, L12-22, L68. `README.md` L15, L137 | B2/rho at w = 0: **0.791 (q = 991) → 0.082 (32719) → 0.049 (65479) → 0.035 (131071) → 0.036 (262111) → 0.024 (524287) → 0.024 [0.019, 0.031] at q = 1048507**. Above rho at q = 223 (3.85) and unresolved at 499 (1.40). Exponents on q ≥ 4051: B2 1.045 against rho 1.476 | **Counted** M-eq (F_q-multiplication equivalents) | MEAS **plain rho without the negation map** (L68: "up to sqrt 2"), 150 runs at small q down to 12 at q = 1048507. L0 shared VM | per study ("B2 is the new baseline"); GHS-weak family only | This is the strongest GHS-weak ratio against the plain-rho reference; row 16 (B0) and row 17 (DL1 against rho_neg) are the same family. Known reference gap: negation map, factor up to √2. Rho at the largest q rests on 12 runs |
| 29 | **EcGFp5 desk evaluation** `research/ecgfp5_lead_evaluation_20261031/README.md` L7-9, L22-27 | "In raw time every plausible variant is **32-45 bits below rho**" (rho ≈ 2^159.3 steps ≈ 2^169 word ops). **Reversed under a memory cap** (L24-27): 2^64-2^71 bytes needed, and capped at one petabyte the method is **about 2^27 above rho**; "rho remains the better attack in any memory-aware cost model" | Modelled 64-bit word operations; memory in bytes | **IDEAL**: rho ≈ 2^159.3 steps, priced at about 2^9.7 word ops per step. **Nothing measured** (L7-9: "Nothing here is measured, nothing is an improvement") | desk model; verdicts REVISE / CLOSE | Not a measured row. Below rho in raw time only; above rho once memory is charged. The rho price per step is a model constant, not a measured implementation |
| 30 | **Genus-2 projective law, retained-state collector** `benchmarks/README.md` L97. `benchmarks/20261022-projective-law/VALIDATION.json` L11-17. `research/projective_law_followup_20261022/REPORT.md` L40-44 | Pooled: neither arm below rho in any price cell (wrapper/rho 2.10-21.5, state/rho 1.47-28.0). Per bit slice: **retained state below matched negation rho in 3/36 cells at 16 bits and 2/36 at 20 bits**. Representative cell (`M+0.5S+10I`, 0.1 GAE): state/rho 4.18 | Counted cold GAE | MEAS source negation rho executed in the same process as each collector (VALIDATION L12), 1,260 source-verified solves per arm | validated | Below rho only in a minority of per-slice cells at the smallest sizes; above in every pooled cell. External fastest-tool wall comparison not run |
| 31 | **Small-Koblitz single-target tournament (tiny2 and successors)** `research/ic_candidate_tournament_20260915/README.md` L14-40. `campaign_20260916/RESULTS.md` L4, L116, L130, L182-196 | Winner/rho, complete cold one-target jobs: instructions **0.48-0.77**, native process time **0.80-0.94**, `beats_rho_strict` on every cell (rounds 0008-0013). Round 0013: 0.770 Ir and 0.910 native | Ir (profiled instructions) **and native process wall** (one process per job, about 1-8 ms) | MEAS the harness's "healthy Pollard-rho reference using the same" library (README L176): jump-table rho on small Koblitz curves (n13a0-n23a0). On n13a0 about 4.3 M of rho's 6.2 M instructions are setup shared by both arms; the rho solve is about 1.9 M (RESULTS L116) | per study; pre-gate | A **wall-time** row (process time), so it belongs with rows 0 and 19. The ratio is dominated by shared fixed cost (spawn, curve construction, final check), and round 0013 found and halved rho's own scalar-product cost (L189). Whether this rho uses the Frobenius and negation fold, batching and a tuned step is a question for step 3 |
| 32 | **Multi-target IC vs multi-target rho in wall time** `research/mt_timed_20261112/RESULTS.md` L1-30. `research/relation_phase_survey_20261114/README.md` L25-30 | Fastest IC / fastest rho, solve-phase wall: **0.778 / 0.792 / 0.829** at 2^24 (T = 1024 / 4096 / 16384), 0.932 at 2^28 T = 1024, unranked at the other 2^28 cells, **1.11-1.29 (dearer)** at 2^32 | **Wall**, one pinned CPU of a 4-vCPU cloud VM (Intel Xeon 2.10 GHz), diagnostic, not L2 | MEAS fastest of `rho2xks` (4 kappa) and `rhodpu` (6 settings), multi-target, same binary `examples/mt_fieldops.rs`, single thread on both arms | validated (independent validator report); claim PCC-20261112-ec2f3f [pcc:unverified] | Multi-target class, not single-target. A **wall-time** row, both arms single-threaded on the same host, so the genus-5 thread asymmetry is absent; whether rho's walk is batched (simultaneous inversion) is for step 3. Related counted row: row 18 |
| 33 | **Many-target crossover, count band** `research/ic_many_target_crossover_20261030/README.md` L8-16 | At n ≤ 45 and n = 47 the **IC lower bound** (emitted additions only) is below rho; "those sizes are unresolved by counts". Clock-priced G there is 5.3-24. At n = 41-61 G > 1 across the band | Counted GAE lower bound; clock calibration | MEAS rho priced at 0 / 0.1 / 1 GAE per canonicalisation | per study | Not a below-rho claim: the below-1 reading is an artefact of unemitted descent and construction counts. Listed so it is not mistaken for one |
| 34 | **Factor-base arity what-if** `research/factor_base_arity_review_20261024/README.md` L14, L90-100 | A formulation whose unknown count does not grow with n "would put m = 5-6 within roughly 4-15 bits of rho at n = 131"; m = 8 "nominally below rho" (L98) | Modelled (Macaulay-matrix price) | **IDEAL** rho at n = 131 | desk what-if, "heavily caveated" | Assumes a bounded degree at m ≥ 5 that nothing has measured (L100 caveat (i)) |
| 35 | **Jacobian transfer scan, formula advantages** `research/jacobian_transfer_scan_20261010/README.md` L60-80 | Oakley Group 4 (genus 15 over GF(2^37)) ratio +10.9 bits; c2pnb368w1 double-large-prime term q^(12/7) = 2^157.7 below rho's 2^176 (+10.1 bits); corrected c2pnb176w1 +23.6, c2pnb272w1 +72.2, c2pnb304w1 +2.0 | Literature complexity formulas | **IDEAL** rho sqrt of subgroup order | screen; "everything else in the row is a formula" | Formula rows only, on deprecated or unused standards. No measurement on either side |
| 36 | **Curve25519 structure levers** `research/curve25519_structure_vs_rho_20261107/REPORT.typ` L21-23 | No lever below rho-neg across the band. "The only rows below 1 are endomorphism-orbit folds at w = 0" | Counted | MEAS rho-neg | per study [pcc:unverified] | Below only at w = 0; same pattern as rows 6, 22, 23 |

## Headline triage (all 27 entries of `headlines.txt`)

| Headline directory | Disposition |
|:--|:--|
| `curve25519_f1_targetwalk_20261108` | row 24 |
| `curve25519_structure_vs_rho_20261107` | row 36 (below only at w = 0) |
| `ecbench_all_candidates_20261003` | excluded: "no index-calculus arm is below the matched rho reference" |
| `ecbench_frobenius_orbit_vs_rho_20261014` | excluded: "not below rho-frob on any of eight Koblitz curves" |
| `ecbench_rootfind_speedup_20261014` | row 11 |
| `ecgfp5_lead_evaluation_20261031` | row 29 (desk model; reversed under a memory cap) |
| `factor_base_arity_review_20261024` | row 34 (desk what-if) |
| `gaudry_k4_char2_crossover_20261015`, `gaudry_k4_char2_redteam_20261026` | row 21 |
| `ghs_cubic_tate_bridge_20261026` | row 27 |
| `hec_bitop_20261020` | row 7 |
| `ic_acct1_cover_bitop_20261021` | row 15 |
| `ic_candidate_tournament_20260915` | row 31 (Ir and native process time) |
| `ic_frontier_campaign_20261015` | excluded: "No index-calculus arm beats generic rho on any curve at any band weight" (L5-6) |
| `ic_many_target_crossover_20261030` | row 33 (lower-bound artefact, not a claim) |
| `ic_weakpoints_20261102` | row 22 |
| `jacobian_transfer_scan_20261010` | row 35 (formulas only) |
| `jv_cover_crossover_20261101`, `jv_cover_negrho_rescore_20261101`, `jv_walk_charge_20261111` | row 13 (the crossover's 0.775 superseded by the negation-rho rescore) |
| `koblitz_rho_reference_20261115` | an audit of the reference, cited in rows 9-12 |
| `pencil_markowitz_20261022` | excluded: "0 are below rho" in the 36-cell grid |
| `prime_precompute_regime_20261028` | row 25 |
| `quartic_n4_20261019` | row 16 (B0) |
| `quartic_n4_la_20261021` | row 28 (B2) |
| `relation_phase_survey_20261114` | row 32 (wall) and row 18 (counted) |
| `sat_pdp_pricing_band_20261111` | row 23 |

Rows reached from `benchmarks/README.md` rather than from a headline: rows 1, 2, 6, 26 and 30. Benchmarks lines
checked for a below-rho phrase: L87, L97, L98 (pencil peel: 0/36 below rho, excluded), L105 (no arm below rho,
excluded), L107, L161 (no IC exponent below rho, excluded), L360 (row 9).

Rows that are not at or below rho and are only context:

- The ecbench `ratio_rho` board rows (all above 1.5 in `leaderboard.json/board`).
- Table A (4.08-11.1×, LEADERBOARD L228-245). Its rho is a one-target clock-unit rho "as built" with
  batched canonicalisation, recorded as `rho_reference_ok=partial`.
- The native tournament (10.7-14.6×, Ir).
- Planner and residual collectors (2.59-14.4×).
- Generic k-sum tracks ("cannot beat rho", 5.4-9.7×).
- F6/F4 ladder (f4 S lower bound / rho 1.105 at m13, a lower-bound quotient).

## Pointers for steps 3-5 (not done; observations made while enumerating)

These come from the repository's own documents and are not this audit's estimates.

1. **Wall-clock rows** (the regime in which genus-5 failed): rows 0, 19, 31 (native process time, with Ir) and
   32 (multi-target, single thread on both arms). Every other row is in a counted unit or a desk model. There, threads and SIMD do not matter. What matters is the group law, setup, DP/restart
   overhead and automorphism fold that the counted rho walk charges.
2. **Counted-reference weaknesses the lab has already documented:**
   - Koblitz strong rho setup at r < 2^28 (rows 9-12; `koblitz_rho_reference_20261115`);
   - hec rho fixed precomputation (row 7);
   - plain rho without negation (rows 16 and 28);
   - the window removed from rho flipping genus-4 w = 1 (row 3);
   - Cantor group-law pricing of rho in the genus ladder (rows 4-5): an inference from L62-64, not yet
     checked against what the IC side's own group ops are priced at.
3. **Reference is an ideal formula:** rows 1, 2, 8, 21, 22, 34 and 35; row 29 is a desk model on both sides.
4. **The largest margins** are weak or special families: JV cover about 11-60× (row 13), GHS-weak DL1
   about 700× (row 17), genus-5 ladder 0.0084 (row 4).

## Files in this directory

- `AUDIT.md`: this file.
- `lb_low_ratios.txt`: the rho-ratio fields of `leaderboard.json` at or below 1.5.
- `validation_files.txt`: the 191 VALIDATION.json paths.
- `below_rho_dirs.txt`: the 174 research directories that contain a below-rho phrase.
- `headlines.txt`: the 27 headline hits (dispositions in "Headline triage").
- `new_rows.md`, `triage.md`: the revision-2 additions, as merged into this file.
