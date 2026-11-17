# Review of the factor-base line search (`research/jv_cover_fbline_20261102`, PR #3286)

This review has two parts, run independently on main at `5f3b5c209a`: a validator ([`validator/REPORT.md`](validator/REPORT.md)) and a red team ([`red_team/REPORT.md`](red_team/REPORT.md), with [`red_team_report.yaml`](red_team/red_team_report.yaml)). Neither read the other's report. Both harnesses blocked writing the report files themselves, so the main session wrote them from the text each returned, verbatim apart from a one-line header. The graph is [`frontier.svg`](frontier.svg) and the PDF is [`REVIEW.pdf`](REVIEW.pdf).

## Bottom line

- **The measurement is sound and reproduces.** The line search cuts the Nagao-only cover arm's cost by **×1.71** [t(8): 1.50, 1.94; bootstrap: 1.54, 1.89], with the search charged.
  - The validator recomputed every number and replayed one run bit-for-bit.
  - The red team censused all ~10⁶ lines per instance. The B⁻⁵ mechanism, the rate law and the per-test cost all hold.
  - The plain instances were not unlucky: the fair random-instance gain is ×1.71.
- **The framing "the cover passes the negation-map rho" is refuted.** The improved arm is not the frontier for this curve class. The repository's own sieving variant of the same cover (`jv_sieve.rs`, JV12 §3.2) costs S = 0.00615 at p = 1511 in the same unit. That is **83–100× cheaper** than the line-search arm (21–25× with its additions and lookups charged). It had already been measured at **0.0098 of the strong rho** in `bitop/v1` (`research/ic_acct1_cover_bitop_20261021`).
  - So the cover route was already far below rho.
  - #3286 is a component speed-up of a dominated arm, not a new crossing.
  - The comparison #3194, #3278 and #3286 made against rho should have been checked against that sieve frontier first. None of the three did.
- **U1 ("below rho in group additions") is downgraded.**
  - U1 prices a rho step at the harness's schoolbook 331 F_p multiplications, 2.40× the tuned 137.7. The cover gets no matching discount.
  - The cover's F_p additions are not emitted. With about one addition per multiplication (a model, not measured), the p = 1301 lead disappears once an addition costs 0.32 multiplications.
  - Under a strict §5a reading the arm cannot be ranked until those counters are emitted.
  - **U2 (F_p multiplications) is the like-for-like unit, and there the arm is above rho at all three primes:** 1.90 / 1.58 / 1.32.
- **Rho is the strongest available.** On all 9 instances j(E) ∉ F_{p²} ∪ F_{p³} and j ≠ 0, 1728, so Aut(E) = {±1}. The q-power Frobenius maps E to a non-isomorphic conjugate, so no √3 speed-up exists. (I had suggested one in the brief; that suggestion was wrong.)
- **The gain decays with p.** It goes as about (1 + 4.2–4.3/√p)⁻⁵ (model): ×1.57 at p = 2039, ×1.23 at p = 10⁴. It is a constant-factor improvement, not an exponent change. With that decay in the slope, the U2 model crossover moves from p* ≈ 1799 to **≈ 1845–1865**, still inside u64.

## Errata to the study README (recorded here; the merged study files are not edited)

1. **The sha "af4e1355…"** belongs to #3278's `negrho_rescore` tool. This study's own `fit/src/main.rs` is `f4149680…`.
2. **"Controls (all pass)"** refers to C1–C4. The committed `analysis/rescore_fit.json` has `controls_pass: false`. These are #3278's consistency controls (they reproduce #3278's plain-arm numbers), and they fail by construction when the line records are the input. The README should have said so.
3. **The re-score command and layout were not committed.** The re-score fed the 9 line records (renamed `dlp_p*_s*.json`) as `inputs/cover/` of a copy of #3278's input tree, with `new/`, `old_rhostrong/`, `redteam_p251/` and `old_rhodp/` unchanged. With the plain records instead, the tool reproduced #3278's `table.md` byte-for-byte. Both the validator and the red team reproduced the numbers independently.
4. **"A tuned F4 is the remaining lever for U2"** is not supported. The sieve route, an exhaustive table-driven line search (+×1.075 measured ceiling) and other levers are untested.

## Small uncharged items (validator, all negligible)

- The square table charges (p−1)/2 of the p−1 products it executes. About 700 multiplications per run go uncharged, a 3e-9 share.
- The search's 65,536·p table lookups are not emitted. Priced as native operations, they move U1 at w = 1 from 0.838 to 0.843. No verdict changes.

## What this means for "closing the distance"

| route (weak class, p ≈ 1500) | S (U1 harness unit) | vs negation rho 0.93 |
|---|---|---|
| Nagao, plain line (#3194) | 0.81–1.31 | 0.87–1.40 |
| Nagao, best line (#3286) | 0.51–0.74 | 0.55–0.79 (U1); 1.32–1.90 in U2 |
| **Sieve route (V2 T1, p = 1511)** | **0.00615** (S⁺ 0.0248) | about 0.0066–0.027; **0.0098 in `bitop/v1`** (ic_acct1) |

**The red team's pilot** (exploratory, not frozen, not evidence) applied the study's `search_fb_line` to the sieve route at p = 1511, seeds 17–24, with all runs correct:

| search size | route only | search charged |
|---|---|---|
| n = 65,536 | ×1.75 | ×0.92, a net loss (the search is 47% of the cost) |
| n = 4,096 | ×1.49 | **×1.42 [1.18, 1.73]** |

**Recommended next step (red team, endorsed).** Freeze a protocol applying the line search to `jv_sieve::run_cover_sieve_dlp_on`:
- **Instances and options:** V2's (p ∈ {1009, 1511, 1823}, seeds 17–32, `--stop 64`, sparse). The plain arm must reproduce V2 exactly.
- **Search cost:** charged in full. Fix n by a rule set before any data, or use a χ(h) table over F_q (about 38q multiplications plus priced native lookups).
- **Pricing:** both arms in `bitop/v1` through the `jv_bitops` hooks, against the matched strong rho with the §5a band.
- **Before any U1 ranking of the Nagao arm:** emit its F_p additions.

## Validation gate

There is no `VALIDATION.json` for a leaderboard row. The validator passed the evidence as admissible for the paired component claim. The red team refuted the frontier framing. No leaderboard or `benchmarks/` row is warranted for the Nagao arm, because it is dominated.

## Sources for the graph and table

- `research/jv_cover_crossover_20261101/results/dlp_*.json`
- `research/jv_cover_fbline_20261102/analysis/paired.txt`
- `research/jv_sieve_strong_rho_v2_20261020/TABLES.md`, T1 row p = 1511
- `research/jv_cover_negrho_rescore_20261101/table.md`
- `research/ic_acct1_cover_bitop_20261021/README.md`
