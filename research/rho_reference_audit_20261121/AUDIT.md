# Audit of rho references behind "IC at or below rho" rows: steps 0-5 (revision 5)

Date 2026-11-21. Read-only. No repository file was edited (repository HEAD `833e09be0d`). Every number below
is copied from the cited file and is an **unverified assertion** (no pcc claim id was checked in this audit;
where a study names one, it is quoted, and none was found signed).

**Scope and revision history.**

- Revisions 1-2 (previous agent) did steps 1-2 only: the knowledge-base check and the enumeration of
  rows 0-36, a headline triage and pointers. It stopped there on a misread instruction.
- An independent verifier accepted revision 2 except row 31.
- **Revision 3 (this file)** does the rest of the task:
  - step 0, row 31 corrected from the verified citations, re-opened and confirmed here;
  - step 3, the rho side of every row recorded with file:line;
  - step 4, a plausible strong-rho factor and a class for every row;
  - step 5, three rerun plans.
- **Revision 4** fixes the verifier's rejection of revision 3:
  - row 4's plane-quartic cell is AT-RISK, not ROBUST: its rho uses a `Pic` law of about 8,900
    multiplications per addition, 44× a Cantor step (`genus_ladder_20261114/RESULTS.md` L95-99), and FastJ
    does not apply; the pattern statement now says "hyperelliptic genus 4-5", not "genus ≥ 4";
  - row 18 is NOT-A-CLAIM (IC/rho 1.06-1.16 at h = 0, 1.00-1.05 at h = 0.1, unranked in every cell:
    `docs/ic/LEADERBOARD.md` L726-740), consistent with rows 6, 23 and 26;
  - Plan A now races the IC against `rho2xksp` and `rhodpup` (the rematch's best rhos at 2^24-2^28,
    `mt_rho_rematch_20261115/RESULTS.md` L26-31) as well as `rhodpb`, and builds with cargo instead of
    the missing `target/release/deps` rlibs;
  - row 28's q = 2011 cell (0.434 [0.388, 0.485], M 2.06-2.58) is AT-RISK as well as q = 991;
  - row 15's margin is stated band-wide (U = 0.0751, M ≈ 13; still above F);
  - Plan B no longer counts "unresolved" as "reversed", and removes the rho noise of its 6 runs per prime by
    exact repricing at the ideal step count, plus a 240-runs-per-prime measured check.
- **Revision 5** (applied by the lead session from the second verifier's findings on revision 4, not
  re-verified):
  - the §5a band rule is now applied consistently. Rows 4 and 5 are unranked across the GAE band
    (`genus_ladder_20261114/RESULTS.md` L82-89: no cell is below rho across the whole band, and g5 at w = 1
    is 0.48 [0.27, 1.14]; `genus_ladder_scale_20261115/RESULTS.md` L40-41: genus 3 is unresolved at every
    prime, above at w = 1). Like rows 6, 18, 23 and 26 they are therefore NOT-A-CLAIM. Their FPM-band
    readings are kept as notes: row 4 g4/g5 is ROBUST in the FPM band only, row 5 is AT-RISK in the FPM
    band only;
  - row 16's q ≤ 4051 cell is unranked/unresolved (B0/neg 1.19 [1.09, 1.3], LEADERBOARD L927; 0.51
    [0.47, 0.554] at w = 1), not reversed. Its q ≥ 65479 margin is about 2.9 at w = 1 (q = 131071 upper bound
    0.349), still above F_high = 2.3;
  - Plan B is an FPM-band reading of a row that §5a leaves unranked. Its step-1 prediction should be about
    1.3-1.5 at p = 32771, not 1.07, because the scale_g3 rho runs average 1.2-1.44× the ideal step count.
    A step-1 ratio above 1 is a reading against a lower bound, not proof of reversal. The genus-3 F could
    reach about 3.3 if a published law (about 1I + 70M, `hecdlp_fastgroup_20261116/NOTES.md` L37) is used.
- The step 1-2 sections below are unchanged except for row 31.
- Steps 3-5 are in the sections "Step 3-4" and "Step 5" after the enumeration table.

**Step 0 (row 31).** These citations were opened and confirmed against the files before the row was edited:

- `campaign_20260916/RESULTS.md`:
  - L182: round 0012, Ir 0.4369, native 0.7988;
  - L232: round 0016 heading, strict fails on eight cells;
  - L1168-1169: `walltime_strong_rho_20260929`, lead gone by n = 23;
  - L1173-1176: `sized_cache_rho_20260930`, IC 0.98-1.03 of the sized rho at four small cells and
    1.043 [1.015, 1.074] at n23a0;
  - L1184-1187: round 0016 strict false; round 0018b does not replicate.
- `DECISION-20260929-stop.md` L16-17 (rounds 0023-0025 produced no win against a matched rho; the line is
  stopped) and its evidence table L23-29.
- `sized_cache_rho_20260930/RESULTS.md` L23-31.
- The line offsets given in the task (L1167-1168, L1171-1175, L1182-1191) differ from the file by one to two
  lines. The content is as stated.

**Knowledge base.** `research-kb` is unavailable in this session as well: ToolSearch for "search_knowledge"
found no matching tool, and revision 1 found the CLI fallback missing. No `source_id` was recorded. Grep and
direct reading of the repository were used instead.

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

## The table (steps 1-2; row 31 corrected in revision 3)

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
| 31 | **Small-Koblitz single-target tournament (tiny2 and successors)** `research/ic_candidate_tournament_20260915/README.md` L14-40. `campaign_20260916/RESULTS.md` L4, L182, L232, L1168-1176, L1184-1187. `campaign_20260916/DECISION-20260929-stop.md` L16-17, L23-29 | **Already reversed by the repository.** Strict wins rounds 0008-0015 on a five-cell panel: round 0012 Ir **0.4369**, native **0.7988** (RESULTS L182); round 0013 0.7697 / 0.9098 (L4). The wins are scoped to the panel and the seed: round 0016 fails `beats_rho_strict` on eight cells (L232, L1184), and round 0018b does not replicate (L1185-1187). Rounds 0023-0025 give no win against a matched rho, and the line is stopped (DECISION L16-17); against the lean rho the collectors are 1.36-13.7× in instructions (DECISION L23). The remaining small-cell wall lead survives the strongest rho on main at 0.74-0.84 and is gone by n = 23 (RESULTS L1168-1169). **Sizing rho's recent-point cache removes it**: IC is 0.98-1.03 of the sized rho at the four small cells and **1.043 [1.015, 1.074]** at n23a0 (RESULTS L1173-1176; `sized_cache_rho_20260930/RESULTS.md` L23-31) | Ir (Valgrind instructions) and native wall, one process per job, pinned to one CPU | MEAS, three references: (a) the tournament's lean rho, single-thread jump-table rho in the IC arm's own arithmetic (DECISION L23), which zero-fills a fixed 160 KB cache per solve (DECISION L29); (b) the strongest rho on main, `examples/koblitz_rho_batch_ks_strong.rs` rung 3, 32 or 1 lanes, DP bits 2 (`walltime_strong_rho_20260929/PREREGISTRATION.md` L13-25); (c) the lean rho with its cache sized to the walk (`sized_cache_rho_20260930`) | per study; pre-gate; line stopped 2026-09-29 | Wall-time row. The IC lead was a rho-engineering artefact (cache zero-fill), and the repository itself records the reversal |
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

## Pointers recorded during enumeration (revision 2; superseded by steps 3-5 below)

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

## Step 3-4. Rho side, plausible strong-rho factor and class, per row

### How to read the columns

**Rho side** records, for each row:

- **kind**: MEAS (measured solves or steps/s) or IDEAL (a closed form times a per-step price);
- **step**: how the per-step price is implemented;
- **thr**: threads;
- **batch**: simultaneous inversion across walks;
- **SIMD**;
- **coords**: coordinates;
- **neg**: the negation map;
- **aut**: other automorphisms, such as Frobenius;
- **DP**: distinguished points;
- **host**: same host as the IC side;
- **asym**: IC multi-threaded while rho runs single-threaded.

"n/r" means not recorded in the files read. "n/a" means it does not enter the row's unit.

**Plausible factor F.** F is the range of factors by which an honest strong rho, run on the same host and
charged in the row's own unit, would be cheaper than the reference the row used.

- **Counted units** (GAE, FPM, M-eq, U2, bitop/v1) do not see threads or SIMD. In those units F comes only
  from the following:
  - the group law and coordinates (operation count per step);
  - batching, which in a counted unit turns an inversion per step into about 3M per step;
  - missing automorphisms, a factor of √|Aut| in steps;
  - setup and DP overheads that a better rho avoids.
- **Wall rows** multiply in threads × batching × SIMD where the IC side gets the same.

**Margin M** = 1 / (IC/rho) for the best still-standing cell.

**Class.** Each row gets one primary class:

- **ROBUST**: the reference is already strong, or M > F_high.
- **AT-RISK**: M lies within F.
- **ALREADY-REVERSED**: the repository itself records the reversal.
- **NOT-A-CLAIM**: there is no below-rho claim across the band (below only at w = 0, unranked, above when
  pooled, desk models, self-retracted framing).
- **UNKNOWN**: the files do not settle it.

Mixed rows give the per-cell split in the reasoning.

### Table

| # | Rho side (step 3; file:line) | F (step 4) | M | Class | Reasoning |
|--:|:--|:--|:--|:--|:--|
| 0 | MEAS, three generations (slope `REPORT.typ` L14-19, L47-50; `src/rhopar.rs` L1-20). Step: E(K4) affine adds, u64 Barrett K4 (`_FASTMUL`), then f64 lane kernel (`_SIMD`, p < 2^16). Thr 4 (rho_m, SIMD) vs 1 (rho_s). Batch: yes, Montgomery, 64 walks, then SIMD batch rounded to LANES. SIMD AVX-512/AVX2. Coords affine. Neg yes (fruitless-cycle escape). Aut: none other. DP yes (vOW). Host same 4-vCPU VM, L0 (L21-24). Asym: was present with rho_s; removed by rho_m | ~14 × 4.5 ≈ 60 between rho_s and SIMD; further levers named open (L99-100) | IC/SIMD rho 1.37-3.89, so no margin | **ALREADY-REVERSED** | Reversed twice (L71-73); IC is above the SIMD rho at every q. This is the reference case |
| 1 | IDEAL `sqrt(pi r/4)` Jacobian ops, nothing charged (RESULTS L9-11, L49-51); MEAS plain rho 5 runs as a sanity check only (0.80× `sqrt(pi r/2)`, L49-51). Step: 1 group op = 1 by convention (Cantor in the instrument, L52). Thr, batch, SIMD: n/a (counted). Neg: in the bound. Aut: none beyond ±1 on these prime-order curves (n/r explicitly). DP: n/a. Host: local 4-core (PROTOCOL L95). Asym: n/a | ≤ 1.0-1.01 in the counted unit: the bound is a floor, and ERRATA E1 adds 1% to IC | 1.14 (p = 16411), 1.56 (32771) at w = 1; ~11-15 at w = 0.1 | **ROBUST** | No rho can beat the ideal bound in group ops. The only exposure is the price convention: a faster group law (FastJ g = 4: 199 vs 458 M-eq per add, `hecdlp_fastgroup_20261116/NOTES.md` L32) makes IC's native ops relatively dearer. Even so, realistic w is about 1/200 of a group op, far below w = 1. Hyperelliptic transfer only, not ECDLP |
| 2 | MEAS plain rho (Q3, Q4) and IDEAL `sqrt(pi r/4)` (RESULTS L16) | √2 (plain → negation) | none at w = 1 vs ideal (1.240) | **ALREADY-REVERSED** (w = 1) | The repository records 1.240 at w = 1 against ideal negation, so §5a does not hold. Below only at w ≤ 0.1 (0.126) |
| 3 | MEAS plain rho; the validator found a window-free rho cheaper (`genus4_scale_20261116/RESULTS.md` L30) | as recorded: w = 1 goes to 1.05-1.15 | none | **ALREADY-REVERSED** | The repository records the flip |
| 4 | MEAS negation rho, 7 primes. **Hyperelliptic cells**: step **Cantor**, 90 / 219 / 393 / 693 FPM per step at g = 2-5 (`genus_ladder_20261114/RESULTS.md` L62-64); measured 227 FPM/step at g = 3 (`genus_ladder_scale_20261115/scale_g3.jsonl`, `cantor_mul/steps`). **Plane-quartic cell**: a different law, about **8,900 field multiplications per `Pic` addition**, 44× the hyperelliptic rho at the same prime (RESULTS L95-99; group law of `src/cryptanalysis/plane_quartic_pic*.rs` used by `quartic_ladder.rs`); FastJ does not apply to it. Thr: `--threads 4` for the harness (`genus_ladder_scale_20261115/PROTOCOL.md` L17-20), counted, so n/a. Batch: no. SIMD n/a. Coords: Mumford (Cantor) / `PicClass` (quartic). Neg yes. Aut: n/r. DP: jump table and DP (L72-74). Host: n/a (counted) | g = 3/4: **2.3-2.5** (FastJ g3 97-100 M + ≤1 I vs Cantor 219 M + 5 I; g4 180-186 M + ≤2 I vs 393 M + 7 I: `hecdlp_fastgroup_20261116/NOTES.md` L22-33); net ~2.0-2.2 since the IC's relation walk also uses Cantor and LA is ~75% of IC. g = 5: no fast law in the repo (L46); ~1.5-3 (estimate). **Quartic: 1-~45** (estimate): the repository records its Pic law at 44× a Cantor step at the same p; a dedicated plane-quartic addition of a few hundred multiplications (published non-hyperelliptic genus-3 laws are in that range; not checked against research-kb, unavailable) could cut rho by tens of times, while the quartic 2LP arm's cost is mostly line decompositions and LA, so it would fall much less | g3 1.33 (0.71-2.17 over the interval); g4 8.5 (4.6-13); g5 119 (50-213); **quartic 37 (27-53)** | **NOT-A-CLAIM**† (rev. 5; FPM band only: ROBUST for g4 and g5 only; g = 3 cell AT-RISK; quartic cell AT-RISK (F_high ≈ 45 ≥ M_low 27)) | g3 (0.75 [0.46, 1.40]) is already unresolved and flips at F ≈ 2. g4 → about 0.27 at F = 2.3; the repository's own projection for the genus-4 scale row moves 0.008 → 0.014-0.024 (`projection.md`). The quartic's small ratio is, in the repository's own words, "why its IC/rho ratio is so small" (RESULTS L97-100): a cheap-law rho could take it to about 0.6-1.2 (0.027 × 25-45), so the cell is AT-RISK, not robust. Both the ROBUST label and the earlier 'genus ≥ 4' pattern statement were corrected in revision 4. Band note: these are FPM-band readings; under the GAE band no cell is below rho across the band (RESULTS L82-89) |
| 5 | as row 4 (genus 3: Cantor 219 FPM, neg, 6 runs/prime; RESULTS L31-43) | **2.0-2.5** net (rho ×2.3-2.5; IC relation phase ~23-25% also cheaper; LA share 0.75-0.77, L33-35) | 1.35-1.86 | **NOT-A-CLAIM**† (rev. 5; FPM band only: AT-RISK) | Projected 2LP/rho with FastJ in both arms: 0.537 × 2.3 × (0.77 + 0.23/2.3) ≈ 1.07 at 32771, and about 1.17 at 8209. The composition with SGE (LA to about ¼, `genus_ladder_sge_20261115`, cited L42-44) would take it to ≈ 0.36. So both levers must be measured together. **Rerun plan B** |
| 6 | MEAS `rho_neg` + canon, r = 64 adding walk, DP, 16 walks/curve; c_add = 81 F_q-mult-eq affine E0(F_{q^3}) add (benchmarks/README L107) | n/a | none above w = 0 | **NOT-A-CLAIM** | Below only at w = 0; `order_holds_across_band: false` |
| 7 | MEAS full-width rho (`--rho-variant full-width`, one thread: `hec_bitop_20261020/run.sh` L10-14), 40 runs/row; 16 full-width jump points precomputed by scalar mults (README L46-50). Step: Cantor (bitop/v1). Batch no. Neg n/r. DP full-width | precompute removal: ×1.1-8.5 on small rows (LEADERBOARD L1182-1199, "post hoc: no rho precompute" column); Cantor → fast law at g3/g4: ×2.0-2.5 on top | after precompute removal, the remaining rows: g3 p ≥ 61 1.37-1.61; g4 p = 31, 41 1.10-1.19; g4 p = 61 7.5 | **ALREADY-REVERSED** (13/18 rows); 4 rows AT-RISK; g4 p = 61 ROBUST | The repository's post hoc column already reads > 1 on 13/18 rows. The 4 rows at 0.62-0.91 fall within the ×2-2.5 group-law factor |
| 8 | Hybrid: `rhoE` = IDEAL `sqrt(pi r/4)` negation steps × the **executed** walk's measured cost per step on the same instance (`ic_genus2_16_speedup_20261117/P1/PROTOCOL.md` L40-44). Step: Cantor (`FPM_1 = cantor_mul + 10 cantor_inv + ...`, L41). Batch no. Neg yes. Aut n/r. Counted | 1.2-2.5 (Cantor vs NUCOMP-style or projective laws at g = 6-10; no fast law for g ≥ 5 in the repo) | per cell: 0.21-0.73 → M 1.4-4.8; ≤ 0.07 → M ≥ 14 | **AT-RISK** (cells g7 p31 0.214, g8 p17 0.444, g10 p11 0.725, g6 p31-53 0.13-0.43); ROBUST at g6 p ≥ 101 (E 0.072, 0.024), g7 p53 (0.064), g8 p31 (0.066) | The "LA at g²" column (P1/RESULTS L14-26) already moves several cells above 1, so the reading depends on the price convention as well as on rho. Standing `[pcc:unverified]`, one producer |
| 9 | MEAS `rho.signed_frobenius_full` (rho-frob); also strong. Koblitz: signed-Frobenius classes A = 2n (`koblitz_rho_reference_20261115/README.md` §1 table) | v5 / strong 0.079 at log2 r 15.4 (README verdict 2) | none vs v5 (1.69 at w = 0.1) | **ALREADY-REVERSED** | README verdict 3: "does not survive v5" |
| 10 | MEAS `rho.signed_frobenius_strong_full`. Step: 32 lockstep lanes, batched inversion, normal-basis canonical form, DP bits 4; setup 32 scalar mults charged 1.5 log2 r adds each (§1 table) | v5 / strong 0.079-0.168 at log2 r 15.4-20.5, so ×6-12 (verdict 2) | 1.7-3.1 | **ALREADY-REVERSED** (inferred) | The audit lists this section as "affected below 2^28" (§2 table, "Lazy orbit-member index"). F ≫ M. The repository has not re-run the row against v5, so this is the audit's factor applied to the row, not a new measurement |
| 11 | MEAS rho-strong at n13 (log2 r ≈ 11) | ≥ 12 (v5/strong is 0.079 already at 15.4 and falls with r) | 1.3 | **ALREADY-REVERSED** (inferred) | Same setup-dominance mechanism, below the audit's smallest size |
| 12 | MEAS strong | — | — | **NOT-A-CLAIM** | Above rho |
| 13 | MEAS `jv_cover --exp rhostrong`: negation, **batch 16**, r = 256, dp-bits 16, same frozen binary (`jv_sieve_line_vs_rho_20261103/README.md` R3 row; PC: steps/sqrt(l) ≈ 0.78-0.81 vs 0.888). Step: textbook F_p mults of an E(F_{p^6}) add (U2). Thr 1. Aut: n/r (if E is defined over a subfield, Frobenius could add √3-√6) | 1.5-4 (Karatsuba F_{p^6} ×~2; possible Frobenius ×1.7-2.4) | 11-60 (pooled with walk 11-65) | **ROBUST** | Weak class only. The margin exceeds every plausible factor |
| 14 | MEAS rho-neg (same as 13) | — | IC is **above** rho-neg 1.8-272× (LEADERBOARD L160-163) | **NOT-A-CLAIM** | A component speed-up, not a below-rho row |
| 15 | MEAS, both arms executed, bitop/v1 (`ic_acct1_cover_bitop_20261021/REPORT.typ` L20, L46-53) | 1.5-4 (as row 13) | **~13 across the §5a band** (U = 0.0751 at w = 1, L53-54); ~100 at the bitop/v1 point (0.0098 [0.0069, 0.0144], L53) | **ROBUST** | The band-wide margin of ~13 still exceeds F_high = 4 (revision 4 replaced the w = 0-only margin of ~100). Weak class only |
| 16 | MEAS **plain rho, no negation** (LEADERBOARD L820). Step: `ec.add` affine E0(F_{q^3}), one K-inversion per step, single walk, R = 16 adding walk (`quartic_n4v2_20261020/rho2.rs`; same code at `ic_gw_nrm_lz_20261115/src/rho2.rs` L28-45). M-eq = mul + r_inv·inv + r_lmul·lmul, r_inv 15-26 measured (`quartic_n4v2_20261020/PARAMS.md` L23-30; PROTOCOL L115) | √2 (neg) × 1.2-1.6 (batched K-inversion: inv_K ≈ 1 F_q inversion at r_inv 15-26 plus a few M_K, against ~6 M_K per add) → **1.7-2.3** vs plain | q = 8191-32719: 2.1-3.0; q ≥ 65479: 4.5-8 | **AT-RISK** (q = 8191-32719); ALREADY-REVERSED at q ≤ 4051 (B0/neg **1.19** unresolved, LEADERBOARD L927); ROBUST at q ≥ 65479 | GHS-weak family only. Row 17 already re-ran with negation |
| 17 | MEAS `rho2::rho_neg`: R = 32 adding walk, look-ahead, snapshot cycle checks, doubling escape, DP (`ic_gw_nrm_lz_20261115/src/rho2.rs` L126-150); single walk, unbatched; frozen supplement k = 22 | 1.2-1.6 (batching) | 700-860 | **ROBUST** | GHS-weak family only |
| 18 | MEAS cheapest of Kuhn-Struik retention rho and DP rho, counted at I = 10M, S = M, 1 GAE = I + 2M + S (LEADERBOARD L726-742). Step: affine, one inversion per step (`examples/mt_fieldops.rs` L1-6). Batch: not in these rows | rho ×~2.6-3.0 from batching (wall: batched rho 0.33-0.38 of unbatched, `mt_batch_inv_20261115/RESULTS.md` L29-36); IC also batches (0.44-0.51), so the **net is 1.2-1.4** | none across the band: IC/rho 1.06-1.16 at h = 0, 1.00-1.05 at h = 0.1 (0.998 [0.992, 1.005] at 2^32 T = 16384), 0.71-0.80 only at h = 1 | **NOT-A-CLAIM** | Below rho only at the h = 1 end; every cell, 2^32 included, is **unranked** under §5a (LEADERBOARD L728 text, table L732-740). By the class key this is NOT-A-CLAIM, as for rows 6, 23 and 26 (revision 3 wrongly called it AT-RISK from the h = 1 margin alone). A batched rho would push the h = 0 and h = 0.1 readings further above 1. Plan A's counted fields re-read it at no extra cost |
| 19 | MEAS same-point rho online wall, no table | equal-advice precomputed rho: precomputed rho faster in every matched pair (benchmarks/README L133) | none | **ALREADY-REVERSED** | LEADERBOARD L237: retained-setup claims superseded by equal-advice rho controls |
| 20 | MEAS rho online Ir | — | cold 5.03 | **NOT-A-CLAIM** | Online-only diagnostic |
| 21 | IDEAL "rho's theoretical cost" (`gaudry_k4_char2_crossover_20261015/REPORT.typ` L7, L44); `rho.negation` rows are lower bounds with canonicalisations uncharged (redteam L15). Aut: ±1 only (generic j) (redteam L13) | unit-price corrections 0.3-3.1× unresolved (crossover L46) | 2.2 at m = 18 (no clean run) | **UNKNOWN** | Red team: "not established (unsupported, not refuted)" (redteam L8-16). The fully priced `rho-strong` (batched inversion) is only planned (redteam L24) |
| 22 | IDEAL `sqrt(pi l/4)` | — | none above w = 0 | **NOT-A-CLAIM** | Below only at w = 0 |
| 23 | per study | — | — | **NOT-A-CLAIM** | Below only at w = 0; unranked |
| 24 | MEAS rho-neg | — | — | **ALREADY-REVERSED** | Self-retracted: "an artifact of what each arm lists as unpriced work" |
| 25 | not read | — | — | **UNKNOWN** | Online after target-blind precomputation. Whether an equal-advice rho was compared was not checked |
| 26 | MEAS rho arm R, negation, r = 64, DP trails, fruitless-cycle escape (benchmarks/README L115); per op 12M + I vs IC 41M + I (L114) | — | band 0.59-2.04 | **NOT-A-CLAIM** | The order flips across the band |
| 27 | none | — | — | **NOT-A-CLAIM** | The report says the existence proof gives no relation or descent costs below rho (`ghs_cubic_tate_bridge_20261026/REPORT.md` L20-24) |
| 28 | MEAS **plain rho without negation** (`quartic_n4_la_20261021/REPORT.typ` L68), same N4 `rho2` walk as row 16, 150 runs at small q down to 12 at q = 1048507 (L9-22) | 1.7-2.3 (as row 16) | q = 991: 1.26 (1.14-1.41); **q = 2011: 2.30 (2.06-2.58)**; q = 4051: 3.9 (3.5-4.3); q ≥ 32719: 11-42 | **ROBUST** (q ≥ 4051); **q = 991 and q = 2011 AT-RISK** | 0.434 [0.388, 0.485] at q = 2011 (L13) gives M 2.06-2.58, which overlaps F 1.7-2.3, so it is at risk as well as q = 991 (revision 4). The headline (0.024 at q ≈ 2^20) survives any plausible factor. These are w = 0 readings (the report gives no band table) |
| 29 | IDEAL ≈ 2^159.3 steps × 2^9.7 word ops (`ecgfp5_lead_evaluation_20261031/README.md` L7-9) | — | — | **ALREADY-REVERSED** (desk) | Reversed under a memory cap: about 2^27 above rho (L24-27) |
| 30 | MEAS source negation rho in the same process as each collector (`benchmarks/20261022-projective-law/VALIDATION.json` L11-17) | — | pooled: none | **NOT-A-CLAIM** | Above rho in every pooled cell. Below in only 5/72 per-slice cells |
| 31 | see the corrected row 31 above | sized cache: ×1.07-1.15 in wall (sized/lean 0.875-0.944) | none | **ALREADY-REVERSED** | Recorded by the repository (Step 0) |
| 32 | MEAS fastest of `rho2xks` (4 κ) and `rhodpu` (6 settings), multi-target, same binary `examples/mt_fieldops.rs` (`mt_timed_20261112/RESULTS.md` L1-11). Step: affine add with **one full inversion per step**, 214-294 ns, I = 23-31 M (L56-57). Thr: **1 on both arms**, pinned. Batch: **no**. SIMD no. Coords affine, x-only database (± folded by x). Aut none. DP yes (`rhodpu`). Host: same 4-vCPU Xeon 2.10 GHz VM. Asym: none. Later references: tuned `rho2xksp`/`rhodpup` (`mt_rho_rematch_20261115/RESULTS.md` L26-35, L46); batched `rhodpb` with Montgomery inversion and lanes (`mt_batch_inv_20261115/RESULTS.md` L29-38; `mt_lanes_20261115/RESULTS.md` L25-36; `examples/mt_fieldops.rs` L3779-3840) | rho batching ×2.6-3.0 against IC batching ×2.0-2.3 → **net 1.2-1.4** (`mt_batch_inv` L29-36) | 2^24: 1.04-1.15 (rematch 0.872-0.964); 2^28: 1.06-1.14 (0.879-0.942) | **AT-RISK** (2^24, 2^28); **ALREADY-REVERSED at 2^32** (batched: IC/rho 1.245-1.269, `mt_batch_inv` L31-33; lanes tuned: level 0.91-1.00, `mt_lanes` L29-31) | Against an unbatched rho at 2^24-2^28 the rematch leaves IC ahead (rematch L26-35, L46). Batched rho was never run at 2^24-2^28 (`mt_batch_inv` and `mt_lanes` SIZES are 32 and 36 only, `mt_lanes_20261115/run.py` L12). **Rerun plan A** |
| 33 | MEAS rho at 0/0.1/1 GAE per canonicalisation | — | — | **NOT-A-CLAIM** | Below-1 is a lower-bound artefact |
| 34 | IDEAL rho at n = 131 | — | — | **NOT-A-CLAIM** | Desk what-if |
| 35 | IDEAL sqrt of the subgroup order | — | — | **NOT-A-CLAIM** | Formula screen only |
| 36 | MEAS rho-neg | — | none above w = 0 | **NOT-A-CLAIM** | Below only at w = 0 |

### Counts per primary class (37 rows, 0-36; revision 5)

| class | n | rows |
|:--|--:|:--|
| ROBUST | 5 | 1, 13, 15, 17, 28 (q ≥ 4051) |
| AT-RISK | 3 | 8, 16, 32 |
| ALREADY-REVERSED | 11 | 0, 2, 3, 7, 9, 10*, 11*, 19, 24, 29, 31 |
| NOT-A-CLAIM | 16 | 4†, 5†, 6, 12, 14, 18, 20, 22, 23, 26, 27, 30, 33, 34, 35, 36 |
| UNKNOWN | 2 | 21, 25 |

† Unranked across the §5a GAE band, so NOT-A-CLAIM under the audit's own key (revision 5). In the FPM band
alone, row 4's g4/g5 cells would be ROBUST and its g3 and plane-quartic cells AT-RISK, and row 5 would be AT-RISK.

Changes from revision 3 (in revision 4): row 18 moved from AT-RISK to NOT-A-CLAIM (below rho only at h = 1, unranked in every
cell); row 4's plane-quartic cell moved from ROBUST to AT-RISK (its rho uses a Pic law 44× dearer than Cantor);
row 28's q = 2011 cell added as AT-RISK; row 15's margin restated band-wide (~13, still ROBUST).
Changes in revision 5: rows 4 and 5 moved to NOT-A-CLAIM (§5a band, see †).

`*` These two are inferred from the Koblitz reference audit's measured v5/strong factor. The repository has
not re-run them.

Cells at risk inside rows of another class:

- row 4: the g = 3 hyperelliptic cell and the plane-quartic cell;
- row 7: g3 p ≥ 61 and g4 p = 31, 41;
- row 28: q = 991 and q = 2011;
- row 16: the q ≥ 65479 cells are ROBUST.

**Pattern.**

- **Every robust row is in a weak or special family, or is a hyperelliptic transfer at genus 4 or 5:**
  - JV cover, rows 13 and 15;
  - GHS-weak, rows 17 and 28 (q ≥ 4051);
  - row 1 (counted in group operations against the ideal step floor). Row 4's hyperelliptic genus 4-5
    cells hold in the FPM band only and are unranked across the §5a band (revision 5). The genus-3 plane
    quartic cell rests on an expensive Pic law.
- **No row on a generic prime-order elliptic curve or a standard Koblitz curve is ROBUST.** Those rows are
  in one of these groups:
  - already reversed (rows 0, 9-11, 19, 31);
  - not a claim (row 18 among them: below only at h = 1, unranked);
  - at risk (row 32 at 2^24-2^28).
- **The wall-time rows** (0, 19, 31, 32) all reversed, or are at risk, once the rho was engineered like the
  IC.

## Step 5. Rerun plans (cheapest and most decisive on a 4-vCPU host)

These are plans only. No experiment was run; the only command executed for step 5 was the build check of
Plan A's binary (BUILD_STATUS in Plan A). Any run follows the repository's rules:

- `isolab-eks` by default (CLAUDE.md, AGENTS.md §9a), or a pinned local diagnostic labelled L0;
- a frozen protocol before data;
- `scripts/checkpoint.sh init` and `tools/benchrec.py` records.

The study commits below are the most recent commits touching those files at HEAD `833e09be0d`.

### Plan A: row 32, multi-target IC vs the best rho, batched and unbatched, at 2^24 and 2^28

- **Why it is decisive.**
  - At 2^32-2^36 the IC lost its lead once rho was batched (row 32).
  - At 2^24-2^28 the reference is the rematch's best rho, which is **`rho2xksp`** (flat-table, prefetched
    Kuhn-Struik retention walk) in 23 of 24 instance-cells and **`rhodpup`** once (2^28, T = 1024)
    (`mt_rho_rematch_20261115/RESULTS.md` L26-31). IC-B / best rho there is 0.872-0.964 (L26-31). Both
    rhos pay one full inversion (23-31 M) per step.
  - The batched DP walk `rhodpb` was never run at 2^24-2^28 (`mt_lanes_20261115/run.py` L12). At these sizes a
    DP walk is not obviously the strongest (the retention walk won the rematch), so a batched DP walk could be
    weaker than `rho2xksp` there. The rerun therefore takes **best rho = min over `rho2xksp`, `rhodpup` and
    `rhodpb`** per instance, and reports IC against each.
  - Projection if `rhodpb` beats `rho2xksp` by the 2^32 batching gain: IC/rho ≈ 0.87-0.96 × (0.46 / 0.36)
    ≈ 1.1-1.25. If `rhodpb` does not beat `rho2xksp` at these sizes, the row stands at the rematch value and
    the remaining lever is a batched retention walk, which does not exist in the repository.
- **Cost.** Cheapest run in the audit; every arm already exists and runs take milliseconds to under a second.
- **Code.** `examples/mt_fieldops.rs`, unchanged, last commit `3a53f555ba` (also the `mt_ic_hp_20261118`
  protocol commit). Arms (all single-threaded, pinned, no SIMD, same binary):
  - IC: `ic5b 4 1.2 flatf <B> bi pf` for B ∈ {32, 128, 256} (batched inversion, flat table, prefetch;
    `mt_lanes_20261115/run.py` L14-16), plus the rematch's IC-B `ic5b 4 1.2 flat 32` as the control that ties
    the run to the rematch;
  - rho: `rho2xksp` at κ ∈ {0.05, 0.1, 0.25, 0.5} and `rhodpup` at κ ∈ {0.0, 0.25} × off ∈ {-1, 0, 1}, exactly
    the rematch's tuned arms (`mt_rho_rematch_20261115/run.py` L9-20; `examples/mt_fieldops.rs` L3008, L3599);
  - batched rho: `rhodpb <κ> <off> <lanes>` with lanes ∈ {16, 32, 64, 128} (L3779-3840; `mt_lanes` grid);
  - unbatched DP control `rhodpu` (as in the rematch), for the batched/unbatched ratio at these sizes.
- **Build.** The `check.out` build line (`mt_ic_hp_20261118/check.out` L4) links prebuilt rlibs under
  `target/release/deps`, which do not exist in this checkout, and its note "main's library does not compile"
  is stale (the verifier reports `cargo check --release --bin genus_ladder` passes at `833e09be0d`). Build
  through cargo instead, into a scratch target directory:
  `CARGO_TARGET_DIR=$SCRATCH/tgt cargo build --release --example mt_fieldops` (`mt_fieldops` is an
  auto-discovered example; it uses only `rand` and `serde_json`, both in `Cargo.toml` L301, L313). Record the
  binary's sha256 and check it reproduces three rematch rows' counted fields exactly (the rematch's
  `check_rho.py` does this for rho arms) before the main phase. **BUILD_STATUS (observed in this audit):** the command
  succeeded at HEAD `833e09be0d` (`Finished release profile in 10m 54s`, exit 0, `build.log`), binary sha256
  prefix `8e1b1d9df45f24eb` (differs from the `mt_ic_hp` instrument's `c049a3233c674818`, as expected for a
  different toolchain/link; the counted fields, not the hash, are what must reproduce). Two single
  unpinned smoke executions on W24-0, T = 1024, seed 43 ran and solved 1024/1024 targets each (`rhodpb … 0.0 0
  64`: I 40,019, M 914,350, S 230,831; `rho2xksp … 0.1`: I 110,621, M 292,071, S 205,400). These are
  build checks only, not measurements: they show the arms run at 2^24 and that, at I = 10M, the two are
  within a few percent of each other in counted cost there, which is exactly why `rho2xksp` must be an arm.
- **Instances.** `research/mt_fieldops_v5_20261112/instances/W24-{0..3}.json` and `W28-{0..3}.json` (the
  rematch's), T ∈ {1024, 4096, 16384}, seed 43.
- **Commands.** Copy `research/mt_rho_rematch_20261115/run.py` (it already enumerates `rho2xks`, `rho2xksp`,
  `rhodpu`, `rhodpup` and IC-A/IC-B), restrict `cells` to pb ∈ {24, 28}, and append to `keys()` the `ic:B*`
  keys and the `rhodpb:L*` keys of `mt_lanes_20261115/run.py` L13-21 (functions `ic` L14, `rho` L17). Then, with `bin/` holding the new
  binary and `instances/` the eight files:
  - `PIN=2 python3 run.py smoke`, then `PIN=2 REPEATS=3 python3 run.py holdout`;
  - one execution looks like `taskset -c 2 bin/mt_fieldops rhodpb instances/W24-0.json 1024 43 0.0 0 64`;
  - analyse with the rematch's `analyze.py`, extended so "best rho" is the min over all rho keys and the
    table also reports IC / best unbatched rho and IC / best `rhodpb` separately;
  - A/A: repeat one key (`rho2xksp:0.1`) as two labelled keys to read the noise floor.
- **Runtime.** Per (size, T, instance): 2 + 3 IC keys and 8 + 12 + 24 rho keys ≈ 49 keys; 2 × 3 × 4 × 49 × 3
  ≈ 3,500 executions at milliseconds to ~0.5 s each, so **about 30-45 min on one pinned CPU** (the 2^32-2^36
  lanes round took 38 min for 1,944 executions, `mt_lanes` L7-8, at larger sizes).
- **What flips the row.**
  - **Reversed** if best IC / best rho (min over all three rho families) is > 1 on 4 of 4 instances in all six
    2^24-2^28 cells.
  - **Survives** if it is ≤ 1 on 4 of 4 in a cell with the per-instance ratios below the A/A spread, and the
    IC-B / `rho2xksp` control reproduces the rematch's 0.87-0.96 (L26-31).
  - **Not decided by batching** if `rhodpb` is slower than `rho2xksp` at every 2^24-2^28 cell: then the row
    stays AT-RISK with the remaining lever named (a batched retention walk), not reversed.
- **Row 18.** Now NOT-A-CLAIM; the run's counted fields re-read it at h = 0, 0.1, 1 at no extra cost.

### Plan B: rows 5 and 4 (g = 3 cell), genus-3 IC vs a negation rho with the fast group law

- **Why it is decisive.**
  - Rho is priced at Cantor, 219-227 FPM per step, but `FastJ` already exists and computes the same result
    for about 100 M + ≤ 1 I (2.3-2.5×), and 2.3× less in time
    (`src/cryptanalysis/genus_ladder_fastgroup.rs` L1-40; `research/hecdlp_fastgroup_20261116/NOTES.md`
    L22-33).
  - It was smoke-tested but never wired into the ladder's rho: "No `rho` driver wiring (`run_rho_walk` still
    uses Cantor)" (NOTES L49).
  - The projection puts the row at about 1.07-1.17 with FastJ alone, and at about 0.36 if SGE is also
    composed into the IC. Measuring both, with the rho noise removed (step 1 below), decides the row.
- **Cost.** The walk is unchanged by a law swap, since results are identical. The rho side can therefore be
  re-priced exactly from the recorded step counts in
  `research/genus_ladder_scale_20261115/scale_g3.jsonl` before any run. That file holds **6 rho runs per prime**
  (`R_rho_neg` × 6 at each of 8209, 16411, 32771; 3 curves × 2 seeds), the same count as each IC arm, so its
  rho means carry about ±25-50% (95%) and cannot by themselves separate a projected 1.07 from 1. Computed in
  this audit from the file: the coefficient of variation of rho's counted `cantor_mul + native` per run is
  0.34 / 0.61 / 0.31 at p = 8209 / 16411 / 32771, against 0.01-0.03 for the IC arms' relation-phase counts, so
  the interval of 0.537 [0.407, 0.686] at 32771 (`genus_ladder_scale_20261115/RESULTS.md` L34-38) is almost
  all rho noise.
- **Code.**
  - Crate root, `genus_ladder` bin (`src/bin/genus_ladder.rs`).
  - `src/cryptanalysis/genus_ladder.rs` (`JCurve::cantor`, L536).
  - `genus_ladder_fastgroup.rs` (`FastJ::add`, `add_batch`). The latest commit touching the study is
    `64fb7188a0`.
- **Change needed (small).** Add a `--group-law fast` switch that routes rho's walk step and the IC's
  relation-walk Cantor calls through `FastJ`. The fast-path counts are tallied as executed, and degenerate
  inputs fall back to Cantor (NOTES L13).
  - Batching (`add_batch`, W = 16-64) is optional. In counted FPM it saves ≤ 1 I per step, and the smoke run
    showed no time gain at p < 2^31 (NOTES L32-33).
  - Strong-rho note: in counted FPM, threads and SIMD do not apply. The `rhopar.rs` design (DP, threads,
    batched lanes) is needed only for a wall-time version.
- **Instances.** The frozen config of `scale_g3.jsonl`: p ∈ {8209, 16411, 32771}, 3 curves, 2 seeds,
  `--arms acR --cc 2.0` (6 runs per arm per prime), extended as in the commands below. Add the SGE arm from `research/genus_ladder_sge_20261115/`.
- **Commands.**
  1. **Exact repricing (decides the fast-law-only reading without rho noise).** Price rho as the ideal
     negation step count sqrt(πr/4) times FastJ's measured FPM per step plus the recorded per-run setup and
     DP overhead, on the same instances: the hybrid `rhoE` convention the repository already uses
     (`ic_genus2_16_speedup_20261117/P1/PROTOCOL.md` L40-44). Report three readings: (a) Cantor/Cantor,
     (b) rho fast / IC Cantor, (c) both fast (IC relation walk repriced at FastJ too). The ratio's interval
     then comes from the IC runs alone. Script: `hecdlp_fastgroup_20261116/projection.py` adapted to
     `scale_g3.jsonl`.
  2. **Measured rho with enough runs.** Run the rho arm alone with the fast law and many more seeds, e.g.
     ```
     genus_ladder run --primes 8209,16411,32771 --genera 3 --curves 3 --seeds 80 --arms R --cc 2.0 --max-h 8 --threads 4 --rho-cap 3e7 --seed-search 600 --group-law fast --out g3_rho_fast.jsonl
     ```
     240 rho runs per prime bring the 95% half-width of the rho mean to about ±4-8% (CV 0.31-0.61 above),
     which separates 1.07 from 1 only at the low-CV primes; step 1 is the decisive reading, step 2 its
     measured check; also run `--seeds 80` without `--group-law fast` on a
     subset to check the steps are identical (the identity control). `--group-law` does **not** exist yet
     (flags at `src/bin/genus_ladder.rs` L134-183); it is the code change above.
  3. **IC arms**, fast law in the relation walk, with and without SGE (`--sge W,F` exists: L172;
     `genus_ladder_sge_20261115/PROTOCOL.md` L78 used `--sge 2,3 --sge 4,3 --sge 8,3`):
     ```
     genus_ladder run --primes 8209,16411,32771 --genera 3 --curves 3 --seeds 4 --arms ac --cc 2.0 --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600 --group-law fast --sge 4,3 --out g3_ic_fast.jsonl
     ```
     (12 IC runs per prime, double the frozen grid, to narrow the IC side as well.)
- **Runtime.**
  - Rho at p = 32771 is 2.8-5.8 M steps per run (records). At FastJ's ~400 ns that is 1-2.5 s per run, so
    720 rho runs are about 10-20 min on 4 threads.
  - The IC is 55 M FPM per run plus LA; 36 IC runs × 2 (with/without SGE) about 15-30 min.
  - The repricing step (1) takes seconds and is done first.
  - Total **under about 1 h on 4 vCPUs**.
- **What flips the row.**
  - **Reversed** if 2LP/rho in FPM with the fast law in both arms has its 95% interval above 1 at p = 32771
    (reading c of step 1, or the measured ratio of steps 2-3). The prediction is about 1.07 with the current
    LA, which may well land **unresolved** rather than reversed.
  - **Unresolved** (interval contains 1) is reported as unresolved with the point ratio as the distance
    remaining. It is **not** counted as reversed (revision 3 wrongly did so). The row then stays AT-RISK and the
    fast-law-only reading is decided by step 1's interval, or by more rho runs.
  - **Survives** if the interval stays below 1 with the fast law in both arms, with or without SGE; with SGE
    composed into the IC the projection is about 0.36.
- **Same lever elsewhere.** Row 4's g = 3 cell (p = 1031) and row 7's four at-risk g3/g4 rows get the same
  law swap. Row 7 also needs the bitop/v1 counter in `examples/hyperelliptic_ic_vs_rho_bitops.rs`.

### Plan C: row 16 (and row 28 at q = 991 and 2011), GHS-weak B0/B2 vs a batched negation rho at mid q

- **Why it is decisive.** The at-risk cells have small margins against the plain, unbatched rho of
  `quartic_n4_20261019`:
  - B0 at q = 8191-32719: 0.33-0.48;
  - B2 at q = 991: 0.79.
  - The negation map alone gives √2. A K-inversion per step priced at r_inv = 15-26 F_q mults could give
    another 1.2-1.6.
- **Cost.** Each rho run is seconds at these q. A step costs about 2 µs, and q = 32719 needs about 3 M
  steps.
- **Code.**
  - `research/quartic_n4v2_20261020/{v2.rs, rho2.rs}`, built with `rustc --edition 2021 -O v2.rs -o v2`
    (README L163). This includes N4's sources by path.
  - Add `rho_neg_lanes`: the existing `rho_neg` walk (R = 32, look-ahead, cycle checks) advanced in L = 16-64
    lockstep lanes, with one Montgomery simultaneous inversion in K = F_{q^3} per round. Count it through the
    same `Cnt` (M-eq = mul + r_inv·inv + r_lmul·lmul at the frozen `N4V2_FIXED=15,1.1`).
  - This is the counted analogue of `rhopar.rs`'s batching. Threads and SIMD do not enter M-eq.
- **Instances.** The frozen N4 instances at q ∈ {991, 4051, 8191, 16363, 32719}, the same units as the
  LEADERBOARD L818-852 rows, with arms B0 and B2 against rho_plain, rho_neg and rho_neg_lanes.
- **Commands.**
  ```
  ./run_grid.sh $PWD/v2 $PWD/../quartic_n4_20261019/n4 results/grid-C params_C.sh
  ```
  `params_C.sh` restricts the grid to those q and adds the rho_neg_lanes arm. Summarise with
  `./v2 summarize`.
  - Controls:
    - rho_neg_lanes with L = 1 must equal rho_neg in steps;
    - the mean steps must be about `sqrt(pi l/4)`.
- **Runtime.** About 5 q × 5 instances × 30 rho runs × seconds, plus B0/B2 runs of the same size as the
  frozen grid, so **about 1-2 h on 4 vCPUs**.
- **What flips the row.**
  - **Reversed** at a q if B0 / rho_neg_lanes has its interval above 1 there. The prediction is near 1 at
    q = 8191 and about 0.6-0.8 at 32719.
  - **Robust** if B0 stays below 1 at w = 0 and w = 1 at q ≥ 16363.
  - For row 28, B2 at q = 991 and q = 2011 against the batched negation rho settles those cells
    (predictions: about 1.1-1.4 at 991, about 0.74-1.0 at 2011, i.e. 0.434 × 1.7-2.3). Add q = 2011 to `params_C.sh`.
- **Why it is third.** The robust headline cells (DL1 at about 700×, B2 at 0.024) cannot flip, so this plan
  settles only the boundary.

### Not chosen

| row | why not |
|:--|:--|
| 8 | Needs a faster group law at g = 6-10, which does not exist in the repository. The "LA at g²" price convention moves the result as much as rho does |
| 4 (quartic cell) | Needs a dedicated plane-quartic group law (the repository's `Pic` addition is ~8,900 multiplications, `genus_ladder_20261114/RESULTS.md` L97-99), which does not exist here. Writing one is a research task, not a cheap rerun; until then the cell is AT-RISK |
| 18 | Now NOT-A-CLAIM (unranked across the band); Plan A's counted fields re-read it anyway |
| 9-11 | Already reversed by `koblitz_rho_reference_20261115`. Re-quoting rows 10-11 against v5 is a leaderboard edit, not a run |
| 21 | Its red team already specifies the run (`rho-strong`, about 30 h, redteam L24). That is too long for "cheapest" |

## Files in this directory

- `AUDIT.md`: this file.
- `lb_low_ratios.txt`: the rho-ratio fields of `leaderboard.json` at or below 1.5.
- `validation_files.txt`: the 191 VALIDATION.json paths.
- `below_rho_dirs.txt`: the 174 research directories that contain a below-rho phrase.
- `headlines.txt`: the 27 headline hits (dispositions in "Headline triage").
- `new_rows.md`, `triage.md`: the revision-2 additions, as merged into this file.

- `AUDIT.rev2.bak`: revision 2 as accepted by the verifier, before this revision.
- `rev3_head.md`, `rev3_row31.md`, `rev3_steps345.md`: the revision-3 fragments merged into this file.
- `AUDIT.rev3.bak`: revision 3 as rejected by the verifier, before revision 4.
- `counts_r4.md`, `planA_r4.md`, `planB_r3.md` (revision-4 text of Plan B): the revision-4 fragments merged into this file.
- `build.log`: output of Plan A's cargo build check (target directory `tgt/`, outside the repository).
