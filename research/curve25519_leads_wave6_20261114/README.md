# Curve25519 index calculus, wave 6: corrections to wave 5, a faithful miniature family, closures and the leads that survive (2026-11-14)

**Class.** Lead proposals, desk checks, two censuses computed here, and one small preregistered measurement
(three F4 cells with the frozen binary, solving degree only). No ecbench session, no row, no leaderboard or atlas
change. No pcc claim is registered, so every statement is an unverified assertion [pcc:unverified].

**Owner's request (2026-11-14).** "Devise more leads." Wave 5 (`../curve25519_leads_wave5_20261112/`, PR #3975)
is the input. Wave 6 opens with an adversarial review of it, because several of its headline statements turned
out to be wrong, and wrong statements make bad leads.

**How.** Three lanes in parallel, records in `lanes/`: `redteam_wave5.md` (red-team, 13 findings), `theory.md`
(5 records on what the solving-degree law means), `schedules.md` (7 verdicts on walk-arm schedules, with the
Bernstein–Lange, Galbraith–Ruprai and Bos constants fetched and quoted). The main session computed the
cyclotomic-torus census and the faithful miniature family (`checks/`), checked both solving-degree rules on all 66
recorded subgroup cells, and ran the out-of-sample test under a frozen micro-protocol (`sdrule/`).

## 1. Answer

1. **Wave 5 overstated three things; the errata are applied to its README in this PR.** The "eight preregistered
   values the lane had not read" were in a table the lane had read, so the 8-of-8 match is in-sample. The
   "solving degree" is the highest productive F4 pair degree, which tracks the largest generator degree plus 0 to
   6; it does not predict cost (R² = 0.09 of ln T on ln SD over 60 cells, with an 8.0e4× cost spread at equal SD).
   The security statement "SD ≥ 65,148 for Curve25519's subgroup bases" holds by construction for chain and flat
   presentations and is false for other encodings; the real statement about Curve25519 is the known
   Petit–Kosters–Messeng smoothness condition, confirmed by the census below [pcc:unverified].
2. **Neither solving-degree rule survives out of sample.** The red team's reading (SD = largest generator degree
   plus a small, form-dependent offset) is the one that stands, with the offset now seen up to 6. Wave 5's band
   missed d = 825, d = 1271 and the k = 18 cell d = 923; the theory lane's fitted chain rule missed d = 341 by one
   and d = 1271 by three (§3). Failed predictions stay failed.
3. **A faithful miniature family exists but is sparse.** Primes 2^k − c with Curve25519's shape (p ≡ 5 mod 8,
   p − 1 = 4·3·q·C with q small and C prime, v₂(p + 1) = 1) and a cyclic cofactor-8 Montgomery curve with a prime
   twist cofactor: k = 20 gives p = 2^20 − 315, A = 383; k = 24 gives p = 2^24 − 75, A = 667 (#E = 8 × 2,096,791,
   twist 4 × 4,194,989). No such prime exists at k = 16, 18 (c < 60,000) or k = 22 (c < 4,000). On the k = 20
   member the only subgroup orders below 17,471 are the divisors of 60, against the lab's family member's rich
   set of divisors of 4·3³·7·19·73. Every verdict of the exponent studies was measured on the smooth family
   [pcc:unverified].
4. **The torus census closes the "other tori" question for the real curve.** Across the tori of degree 1 to 12
   the largest subgroup whose prime factors stay below 2^20 has about 1.3 × 10^10 elements (degree 8), below 2^20
   when the factors stay below 2^12 at every degree, against the useful window of 2^64 to 2^85 [pcc:unverified].
5. **The walk arms are at a floor no schedule moves.** In an oblivious walk, collisions whose difference
   functionals are independent are independent events, so rows cannot arrive faster than Poisson; the K = 3 floor
   is 1.662 and interleaved grumpy giants, shared tables, adaptive table size, kangaroo variants and tag tracing
   are all rejected with numbers (`lanes/schedules.md`). Wave 5's "3 % walk excess" finding is the published
   Brent–Pollard factor √(1 − 1/r), 1.0352 at 16 jumps [pcc:unverified].
6. **What survives as measured-improvement candidates** are two small pipeline levers, reworded by the red team,
   and one algebraic-component ablation: both [d]G = Q checks from the stored ladder (about −4 % at k = 20 on the
   tw-lazy arms), recycling set-up points on the target-walk arms only (about 0.7 to 0.8 of the set-up share),
   and the flat-versus-chain presentation ratio at d ≤ 36. None changes an exponent. The distance to matched rho
   stays about 1.7× at k ≥ 28 for the walk arms and far larger for every algebraic arm [pcc:unverified].

## 2. Ranked leads

"Improvement" leads could give a verified gain over the lab's IC baseline; "rigor" leads remove a confounder or
close a question; "theory" leads are provable statements for the Lean repository or the hardness corpus. Priors
are the lanes' or, for the main session's items, mine [pcc:unverified].

| # | lead | class | prediction | cheapest test | prior |
|--:|:--|:--|:--|:--|--:|
| 1 | W6-1 faithful-family replication of the exponent study's cheap cells (m = 2, 3; d ≤ 60 on p = 2^20 − 315, A = 383) | rigor | prime-d cells reproduce T within 2× of the smooth family's at equal d; the sparse divisor set leaves the m = 3 estimator inapplicable by construction | minutes with the frozen binary once `curve25519_family` accepts an explicit (p, A); a one-line CLI change | 0.7 |
| 2 | W5-PL-3 reworded: both [d]G = Q checks of the tw-lazy arms from the stored [2^i]G ladder, both kept | improvement | −4.1 % at k = 20, −1.7 % at k = 24 on tw-lazy arms; −2 %, −0.9 % on WalkTable arms; walks unchanged | hours of Rust; paired runs identical except the check delta | 0.85 |
| 3 | W6-2 flat y^d = 1 against the chain, as a component ratio at d ≤ 36 (m = 3) and d < 400 (m = 2), with msolve as a second engine | improvement (component) | flat lies on the prime-d cost curve; chain/flat 70 to 260× at d = 27, 28; both have SD = d + 1 | under 1 h; chain cells on disk | 0.6 |
| 4 | W5-PL-1 reworded: store the distinct set-up points, target-walk arms only; measure wwalk-lazy at k = 20 before any strict-IC claim; report against rho-neg and rho-neg-rec | improvement | walk steps fall by about 0.7 to 0.8 × distinct set-up points on the WalkTable, by 0.8 to 1.0 on tw arms; no run costs more at w = 0 | Stage 0: count distinct coefficient vectors from stored records; then about a day of Rust | 0.6 |
| 5 | W6-3 F4 bookkeeping check: dump the step trace of one k = 20, m = 2, d = 19 target; rerun d = 19, 57 with the e-variables first | rigor | the last productive step at degree d + 1 is the binomial–quadric pair; SD is order-dependent by at most 1 | minutes | 0.8 |
| 6 | W6-4 memory profile of `f4_fp` at k = 20, m = 3, d = 28 and 36 (peak RSS, dense block shapes); replaces W5-RP-4, whose split is already in the engine | instrument | the dense block at d = 36 exceeds 12 GB by itself; a sparse-row representation of the reducible part cuts peak memory 3 to 10× | hours (instrumentation only) | 0.5 |
| 7 | W6-5 Lean lemma L1: 1 is not in the ideal generated by the curve relations and the binomials y_i^d − 1 below degree d + 1 (a sum of pure d-th powers is not divisible by a mixed monomial) | theory | provable; gives the rigorous lower bound SD ≥ d + 1 for the flat and chain presentations only | a day in crypto-lean | 0.8 |
| 8 | W6-6 corpus records: HEUR-FIRST-FALL-DEGREE holds on prime-d μ_d cells and fails on chains (SD exceeds the first fall by up to 19×); HEUR-SP-SEMAEV15-ASSUMPTION1 is a different setting ("no transfer") | theory / corpus | desk | an hour | 0.9 |
| 9 | W6-7 encodings of μ_d with low generator degree (exponent-bit map, baby–giant split, sumset): measure SD and T at k = 20, m = 3, d = 19 against the recorded 20 and 2.2e8 | security | the exponent-bit encoding is the digit presentation in multiplicative form; the lab's digit study found the last fall degree rising one per two digits and a cost exponent ≥ 0.70, so no encoding is expected to be cheaper than the flat one | minutes per cell | 0.7 |
| 10 | W6-SC-8 late-target stored walk (table target-free to rank 2, then chained [β]Q jumps) | improvement, classification pending | 0.92 of inc-lazy at k ≥ 24; 1.00 [0.97, 1.03] of tw2cj-lazy | owner's ruling on tw2 first | 0.6 |
| 11 | W6-TH-4 preregistered prime cells at m = 3: k = 24, d = 23 (SD 24, T 5.1e8); k = 20, d = 73 (SD 74, T 1.15e11 [6e10, 2e11]); k = 24, d = 89 (SD 90, T 3.1e11) | theory check | as stated; d = 73 and 89 need 2 to 50 GB and 700 to 8,000 s | d = 23 in seconds; the others on a 64 GB node | 0.6 |
| 12 | W6-TH-2 chain-offset rule as a bounded statement: SD − e_max ∈ [1, 6] so far, rising with the smaller factors | descriptive | superseded as an exact rule by §3; keep only as a band | done | — |

Closed by this wave (no further work recommended): interleaved grumpy giants, multi-walk shared tables, adaptive
table size, kangaroo and Gaudry–Schost variants, tag tracing (all `lanes/schedules.md`); the exact solving-degree
rules of wave 5 and of the theory lane (§3); W5-RP-4 as written (its mechanism already exists in `f4_fp`);
W5-SYN-3 as a cost law or a security statement; W5-SYN-1 as an improvement (it is a census, and its sign
conflicts with the T₀ study) [pcc:unverified].

## 3. The out-of-sample solving-degree test (`sdrule/`)

Frozen in `sdrule/PROTOCOL.md` (commit `df16798eb1`) before data: k = 22, m = 2, sym form, two targets per
cell, the exponent studies' binary and seed rule, 1,800 s per target, 12 GB. The theory lane's rule
SD = e_max + 1 + ⌈s/4⌉ (s the sum of e_j − 1 over the smaller prime factors) had matched 66 of 66 recorded cells
(`checks/sd_rule_check.out.txt`); wave 5's band was e_max + 2 to 3.

| d | factors | recorded SD (both targets) | theory-lane rule | wave-5 band | T (field ops) | wall |
|--:|:--|--:|--:|:--|--:|--:|
| 341 | 11 × 31 | 34 | 35 (miss) | 33 to 34 (hit) | 4.2e8 | 8 s |
| 825 | 3 × 5 × 5 × 11 | 15 | 15 (hit) | 13 to 14 (miss) | 2.5e10 | 153 s |
| 1271 | 31 × 41 | 47 | 50 (miss) | 43 to 44 (miss) | 4.8e10 | 483 s |

P1 (every cell equals the rule) failed at the first cell; P2 (some cell outside the band) held at the second; P3
(all cells complete) held. The third cell misses both rules by three: SD − e_max = 6, one more than the largest
offset in the 66 recorded cells. Together with the k = 18 cell d = 923 (SD 75 = e_max + 4) the honest statement is
the red team's, widened by one: SD equals the largest generator degree plus a form-dependent offset, 0 to 6 so far,
that grows with the smaller prime factors; no exact formula is supported [pcc:unverified].

## 4. Censuses computed here

**Cyclotomic tori of the real curve** (`checks/cyclotomic_census.py`, `.out.txt`; trial division to 2 × 10^6 and
time-capped Pollard–Brent; figure left panel). For each degree k ≤ 12, the largest subgroup order dividing Φ_k(p)
whose prime factors are all at most B:

| k | small factors of Φ_k(p) | unfactored part | max |V|, B = 2^8 | B = 2^12 | B = 2^16 | B = 2^20 |
|--:|:--|:--|--:|--:|--:|--:|
| 1 | 2², 3, 65147 | 236-bit prime | 12 | 12 | 781,764 | 781,764 |
| 2 | 2, 5², 79, 60824497, 213156431 | 190-bit composite | 3,950 | 3,950 | 3,950 | 3,950 |
| 3 | 3, 37, 5689, 14149, 289120090447 | 439-bit composite | 111 | 111 | 8.9e9 | 8.9e9 |
| 4 | 2, 229, 2506481 | 480-bit composite | 458 | 458 | 458 | 458 |
| 6 | 7, 19, 57529, 1643221, 17291851 | 443-bit prime | 133 | 133 | 7.7e6 | 7.7e6 |
| 8 | 2, 57649, 110273, 1698121 | 966-bit composite | 2 | 2 | 115,298 | 1.3e10 |
| 5, 7, 9, 10, 11, 12 | at most three small factors each | large composite | ≤ 23 | ≤ 2,255 | ≤ 9,131 | ≤ 9,131 |

The unfactored parts could hide further factors between 2 × 10^6 and the rho reach, so the B = 2^16 and 2^20
columns are lower bounds; the conclusion (nothing near 2^64) does not depend on them unless a composite splits into
many factors below 2^20, which the Pollard–Brent passes would have found [pcc:unverified].

**Faithful miniature family** (`checks/faithful_family.py`, `.out.txt`): rule and results in §1.3. The lab's
member at k = 20 (p = 2^20 − 3, A = 926) shares p ≡ 5 mod 8 and v₂(p − 1) = 2 with the real curve but has
p − 1 = 4·3³·7·19·73 and p + 1 = 2·(2^19 − 1), so it has many small subgroups and no torus base at all. The
faithful member has p + 1 = 2 × 524,131 and subgroup orders 1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60, then
17,471 [pcc:unverified].

## 5. Checks done in this session

| check | result | file |
|:--|:--|:--|
| both solving-degree rules on every recorded subgroup cell (sym and plain, m = 2, 3, 4, k = 18 to 24) | 30 of 30 prime-d cells and 36 of 36 composite cells match the theory lane's rules in-sample | `checks/sd_rule_check.py`, `.out.txt` |
| out-of-sample cells under the frozen protocol | §3 | `sdrule/` |
| cyclotomic census | §4 | `checks/cyclotomic_census.*` |
| faithful family | §4 | `checks/faithful_family.*` |
| red team's claim that `solving_degree_max` is the highest pair degree | read in the red-team file; not re-derived from the engine source here | `lanes/redteam_wave5.md` §1a |
| Brent–Pollard factor (1 − 1/16)^(−1/2) = 1.0328 | arithmetic | session log |

## 6. Not done and caveats

- research-kb, ecdb and the lead atlas bodies were unreachable; novelty is checked against the 176 atlas ids and
  repository files only. A knowledge-base pass is owed before any lead becomes a task card.
- Lanes had no shell; their hand-derived numbers are unchecked unless §5 lists them. The schedules lane ran no
  simulation (it specifies cells for `src/bin/nc2_stage0.rs` instead), so its predicted S values are analytic.
- The `sdrule` cells are an engine measurement with a frozen protocol but no benchmark record, Conductor task,
  resume checkpoint or validator; the claimed quantity is the solving degree only.
- No model identifiers appear in committed files; the lanes ran on a different tier from the agents' defaults.

## 7. Next steps

The owner decides. The cheapest decisive items are #5 (minutes; settles whether the solving-degree law is
bookkeeping) and #1 (a one-line CLI change, then minutes; settles whether the exponent-study verdicts depend on the
family rule). The cheapest likely measured gain is #2. Everything else waits on #1 to #5 or on the owner's ruling
on tw2.

## Files

| file | content |
|:--|:--|
| `lanes/redteam_wave5.md` | 13 findings on wave 5 (R1 to R13), with the cheapest falsification for each |
| `lanes/theory.md` | 5 records: the d + 1 lower bound, the chain offset, Lean statements and corpus scope, preregistered prime cells, the flat presentation |
| `lanes/schedules.md` | 7 verdicts on walk-arm schedules with fetched constants, and W6-SC-8 |
| `checks/` | census scripts and outputs, the SD-rule check, the figure script |
| `sdrule/` | frozen protocol, runner, cells, run log of the out-of-sample test |
| `wave6.svg`, `wave6.png` | figure: census (left) and the out-of-sample test (right) |
| `REPORT.typ`, `REPORT.pdf` | this README as a report |
