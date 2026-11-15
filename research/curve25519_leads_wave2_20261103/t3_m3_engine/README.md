# T3: does m = 3 frame selection survive the F4 engine? (results, 2026-11-03)

**Protocol:** [`PROTOCOL.md`](PROTOCOL.md), frozen with `scripts/run.sh` and
`scripts/select_frames.py` in commit `2b330f250a1bca382b81023d95d556fccd7078d3`, before any T3
data was taken. The run was `bash scripts/run.sh` once, unchanged, from 2026-11-03T16:55:10Z to
17:56:02Z. Nothing was rerun or retuned.
**Class:** component measurement on the algebraic track (`pkm-semaev`, Curve25519-family
miniature k = 20, A = 926, p = 1048573, r = 131203). Not a leaderboard row. Exact counts and
counted F4 mul-adds only. Wall time is shown for cost and sizing only and is not a timing claim.

## Summary

- **Five of six predictions held. P2 failed at t = 5 under its strict rule.**
- The J3-selected m = 3 frame held under P2 at t = 4 (same solving degree, same terms per
  specialisation). At t = 5 P2 failed (terms unequal), and the per-call mul-add agreement
  (ratio 1.0000484 at t = 4 and 1.0000044 at t = 5) is exploratory, not a preregistered
  quantity.
- Its exact coverage equals the census m3_cov (458 and 2578).
- So the census gain becomes an F4/relation gain within m = 3: **g3 = 7.63× at t = 4 and
  11.80× at t = 5**. That is larger than the m = 2 selection gain (6.50× and 4.38×).
- **P2 fails at t = 5** because `poly_terms_per_specialisation` differs in the 8th significant
  digit (relative difference 5.6e-8): 3843.991452991453 against 3843.991666666667. Arm B found
  one verified relation, and that target solved 2 of its 8 guess branches and skipped 6 (234
  against 240 F4 calls; the F4 loop breaks at the first consistent branch). The frozen rule
  requires equality, so P2 is recorded as failed at t = 5. The diagnosis is exploratory (see
  below).
- **m = 3 is never cheaper per relation than m = 2.** At matched t and selected frames,
  q = 2297× (t = 4) and 6643× (t = 5). P5 held. This lead brings no improvement over the
  index-calculus baseline.

## Step S: census and frame selection

- The census was regenerated with the torsion study's frozen script: sha256
  `dc08cc8ea25f6ff8598603e5751394d7a4448c4353819cc49952937d71d1099f`, equal to the published
  manifest. F1 did not trigger.
- `select_frames.py` passed every check (`all_checks_pass: true`, `data/selection.json`).

| t | L3\* | L3\* m3_cov | L3\* J3 | L3\* cols | L3\* m2 cov | mean m3_cov (300) | L2\* | L2\* m2 cov |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 4 | 598026 | 458 | 484 | 14 | 46 | 103.97333333333333 | 797820 | 78 |
| 5 | 546312 | 2578 | 2748 | 25 | 178 | 757.22 | 233902 | 184 |

## Results (verbatim from `data/cells/*.json`)

All cells: k = 20, G = 3, full mode, max_nnz 30,000,000. In every cell `capped` = false and
`undecided` = 0, and `hits_verified` = `hits`.

| # | tag | form | m | t | targets | f4_calls | f4_muladds | covered_points | sd_max | terms/spec | muladds_per_target | C = muladds/rel @ exact cov | hits | wall_s |
|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 1 | m2_t4_lo0 | bits | 2 | 4 | 100 | 800 | 8884000 | 12 | 4 | 22.0 | 88840.0 | 971339543.3333333 | 0 | 0.65 |
| 2 | m2_t4_L2 | bits@797820 | 2 | 4 | 100 | 800 | 8884000 | 78 | 4 | 22.0 | 88840.0 | 149436852.82051283 | 0 | 0.84 |
| 3 | m2_t4_L3 | bits@598026 | 2 | 4 | 100 | 800 | 8884000 | 46 | 4 | 22.0 | 88840.0 | 253392924.34782606 | 0 | 0.79 |
| 4 | m2_t5_lo0 | bits | 2 | 5 | 100 | 800 | 303691176 | 42 | 5 | 64.0 | 3036911.76 | 9486950801.125713 | 0 | 3.07 |
| 5 | m2_t5_L2 | bits@233902 | 2 | 5 | 100 | 800 | 303690989 | 184 | 5 | 64.0 | 3036909.89 | 2165498305.9655976 | 0 | 3.14 |
| 6 | m2_t5_L3 | bits@546312 | 2 | 5 | 100 | 800 | 303690881 | 178 | 5 | 64.0 | 3036908.81 | 2238491834.8226404 | 0 | 2.96 |
| 7 | m3_t4_lo0 | bits | 3 | 4 | 100 | 800 | 119824176893 | 60 | 10 | 512.0 | 1198241768.93 | 2620215246815.38 | 0 | 78.9 |
| 8 | m3_t4_L3 | bits@598026 | 3 | 4 | 100 | 800 | 119829979909 | 458 | 10 | 512.0 | 1198299799.09 | 343276263187.7844 | 0 | 73.7 |
| 9 | m3_t5_L3 | bits@546312 | 3 | 5 | 30 | 234 | 8479559465325 | 2578 | 11 | 3843.991452991453 | 282651982177.5 | 14385100084419.912 | 1 (verified) | 1202.9 |
| 10 | m3_t5_lo0 | bits | 3 | 5 | 30 | 240 | 8696945835182 | 224 | 11 | 3843.991666666667 | 289898194506.06665 | 169801396490086.88 | 0 | 1500.5 |
| 11 | m3cov_t4_L2 | bits@797820 | 3 | 4 | 0 | 0 | 0 | 400 | – | – | – | – | – | 0.006 |
| 12 | m3cov_t5_L2 | bits@233902 | 3 | 5 | 0 | 0 | 0 | 2142 | – | – | – | – | – | 0.060 |

Columns: cells 1 and 4 have 7 and 11; cells 2, 3, 5, 6 have 11, 14, 25, 25; cells 7–10 have
7, 14, 25, 11. Coverage_exact for cells 7–10 is 0.00045730661646456255, 0.0034907738390128275,
0.019648940954094037 and 0.0017072780348010336.

### Derived quantities (definitions from PROTOCOL §2.5)

| t | ρ_t = mpt(L3\*)/mpt(bits), m = 3 | g3_t | g2_t | q_t = C(m3 L3\*)/C(m2 L2\*) |
|--:|--:|--:|--:|--:|
| 4 | 1.0000484 | 7.6330 | 6.5000 | 2297.13 |
| 5 | 0.9750043 | 11.8040 | 4.3810 | 6642.86 |

## Predictions (decided by the frozen rules)

| # | prediction | rule | observed | outcome |
|:--|:--|:--|:--|:--|
| P0 | instrument identity (binary 7a43…) | exact equality, cells 1, 2, 4, 5, 7 | 8884000/12, 8884000/78, 303691176/42, 303690989/184; cell 7 119824176893 over 800 calls, undecided 0 | **held** |
| P1 | coverage = census | cells 8, 9 = 458, 2578; cells 7, 10 = 60, 224 | 458, 2578, 60, 224 | **held** |
| P2 (t = 4) | same F4 system | sd equal, terms equal, undecided 0, 0.90 ≤ ρ ≤ 1.10 | sd 10 = 10, terms 512.0 = 512.0, undecided 0/0, ρ = 1.0000484 | **held** |
| P2 (t = 5) | same F4 system | same | sd 11 = 11, undecided 0/0, ρ = 0.9750043, but terms 3843.991452991453 ≠ 3843.991666666667 | **failed** (terms not equal) |
| P3 | gain survives the engine | g3 ≥ 5.0 at t = 4 and t = 5 | 7.6330, 11.8040 | **held** |
| P4 | m = 3 selection pays more than m = 2 selection | g3 > g2 at both t | 7.6330 > 6.5000; 11.8040 > 4.3810 | **held** |
| P5 | m = 3 not cheaper per relation than m = 2 | q ≥ 100 at both t | 2297.13, 6642.86 | **held** |

F2 and F3 did not trigger: no cell failed, was capped, or was left undecided.

## Exploratory (not preregistered)

- **Why P2 failed at t = 5 (diagnosis only; the outcome stands).** `poly_terms_per_specialisation`
  is a mean over the specialisations that were solved:
  - arm B: 899494 terms / 234 specialisations;
  - arm A: 922558 / 240.
  Arm B's one relation ended its target after solving 2 of its 8 guess branches (6 skipped;
  cell 9 has 233 refutations + 1 consistent system = 234 = 29 x 8 + 2), so it solved 234
  systems instead of 240. Then:
  - per F4 call, mul-adds agree to a ratio of 1.0000044 (36237433612.5 against
    36237274313.3);
  - ρ_5 = 0.975 is almost exactly 234/240 (0.975).

  So the system looks unchanged, but this is an interpretation. The frozen rule asked for
  equality and did not get it.
- **m = 2 on the m = 3-selected frame** (cells 3, 6):
  - coverage 46 at t = 4 and 178 at t = 5, against 78 and 184 on L2\*;
  - C is 1.696× (t = 4) and 1.034× (t = 5) worse than the m = 2-selected frame;
  - per-target cost is unchanged at t = 4 (88840.0). At t = 5 the m = 2 per-target cost is
    3036908.81 (L3\*), 3036909.89 (L2\*) and 3036911.76 (lo = 0): equal to within 1e-6
    relative, not exactly equal.
- **m = 3 coverage of the m = 2-selected frame** (cells 11, 12): 400 at t = 4 and 2142 at
  t = 5, against 458 and 2578 on L3\*. J3 selection beats J-selection for m = 3 by 1.145× and
  1.204×.
- **m = 3 against m = 2 on the same frame L3\*:** q = 1354.72 (t = 4) and 6426.25 (t = 5).
- **One observed relation:** cell 9 has `hits` = `hits_verified` = `hits_exact` = 1, at
  8479559465325 counted mul-adds for the cell. With n = 1 this is not a rate estimate. The
  coverage-derived C is used throughout.
- **Realised gain against coverage ratio:**
  - t = 4: g3 = 7.6330 against a coverage ratio of 7.6333 (ρ ≈ 1);
  - t = 5: g3 = 11.8040 against 11.5089. The difference is the early-stop effect in ρ_5, so
    the 11.80× at t = 5 includes about 2.5% from that one hit and is not frame structure.

## Costs

- Counted F4 mul-adds over all cells: 17,417,097,182,355.
- Summed `wall_s`: 2867 s (about 48 min). Total run, including the census regeneration
  (about 16 s), was about 61 min.
- The two t = 5 m = 3 cells took 1203 s and 1500 s, each well under the 3600 s cap.
- Peak RSS for cell 9 was about 2.4 GB, under the 6 GB `ulimit -v`. This is an unrecorded
  observation (no RSS is in `data/`; the validator saw 2.38 GB on its own rerun), for sizing
  only and not a claim.
- Native work that is not charged is listed in each JSON under `native`, for example
  `pkm_system_fp_mults_uncharged` 3549150 in cell 9. It is not priced here.
- Frame search: 300 · 2^t Legendre-class evaluations, as priced in the parent study.

## Distance remaining

Rho remains ahead, and this lead does not change that.

- At k = 20 the cheapest m = 3 cost per relation measured here is C = 3.43·10¹¹ F4 mul-adds
  (t = 4, L3\*), still worse than m = 2 even after the selection.
- The cheapest m = 2 per relation in these cells is 1.49·10⁸ (t = 4, L2\*).
- A rho-neg solve at k = 20 costs S = 1.447 / 1.547 / 2.448 (w = 0 / 0.1 / 1), about 524–887 GAE (ecbench session `ECBS1h405f20219036`, `curve25519_isogeny_seed_20261103`).
- The generic index-calculus baseline (proj-a) is 8.3–8.5× rho-neg in that session (12.05 / 12.93 / 20.83). The best algebraic arm is 18× (w = 0) to
  9.6·10⁵× (w = 0.1) behind rho-neg (`curve25519_algebraic_ecbench_20261102`).
- P3 and P4 are a verified cut of 7.6–11.8× **within m = 3**. They are not an improvement
  over the index-calculus baseline, because m = 3 stays 2.3·10³–6.6·10³× costlier per
  relation than the selected m = 2 frame.
- No cross-method ranking is made, so no §5a band is applied.

## Deviations and notes

- None from the frozen commands; `run.sh` ran unchanged and completed (exit 0).
- PROTOCOL §2.2 says the census bytes were not on this machine. By run time the torsion
  study's `data/k20.json` was present locally with the same sha256. `run.sh` regenerated the
  census anyway, as frozen, and the hash matched. No substitution was made.
- The machine was shared. During this run another lead's `f4cost` process (T6, a separate
  agent) and an `ecbench` job also ran. That changes wall times, which are not claims. The
  counts are deterministic, as the P0 exact matches show.
- `env.txt` records `nproc` = 3.
- The frozen `data/SHA256SUMS` written by `run.sh` covers the JSON outputs and provenance
  files. A second list, `data/SHA256SUMS.supplement`, added after the run, covers the logs and
  stderr files that `run.sh` does not hash. No frozen output was edited.

## Files

- `data/census/k20.json` (hash-checked), `data/census/SHA256SUMS`, `data/census/k20.log`
- `data/selection.json`, `data/selection.log`, `data/frames.txt`
- `data/cells/<tag>.json` and `.stderr` (all stderr empty)
- `data/binary.sha256` (`7a438b1d…ba98be`), `data/env.txt`, `data/run.log`
- `data/SHA256SUMS` (frozen output), `data/SHA256SUMS.supplement`

## Validator errata (applied)

Source: `validator-report.json` (status pass_with_errata). No table value or verdict changed.

- E1: Summary P2 bullet and Exploratory "Why P2 failed" corrected: the hit target solved 2 of
  its 8 guess branches and skipped 6 (not "stopped after 6"). The 234-vs-240 count and
  rho_5 = 234/240 are unchanged.
- E2: Summary second bullet reworded: held under P2 at t = 4; at t = 5 P2 failed (terms
  unequal) and the per-call agreement (4.4e-6) is exploratory.
- E3: "7th significant digit" changed to "8th" (relative difference 5.6e-8).
- E4: Costs RSS statement marked as an unrecorded observation for sizing only, not a claim.
- E5: Exploratory cells 3/6 per-target cost stated as exact at t = 4 only; at t = 5 values
  3036908.81 / 3036909.89 / 3036911.76 differ by less than 1e-6 relative.
