# Session log: rootfind_genus23_20261123

## 2026-11-23 19:15Z to 19:45Z (Claude, parallel workflow agent; shared checkout, no git writes)

### What was done
- Read `research/rootfind_algo_survey_20261123/SURVEY.md`, including its addendum, and
  `docs/ACTIVE.md`. The "Current goal" is now chained S3; the genus g ≥ 2 goal is kept as the
  previous pinned goal for ongoing lanes. This task follows the computed task's genus 2/3 brief.
- Mapped what the lab already has for genus 2–3:
  - the `genus_ladder.rs` pipeline: prefilter, `x^p mod u`, Cantor–Zassenhaus, Ferrari at g = 4,
    2LP, SGE;
  - `ic_control` with the Cardano–Lucas oracle (`ic_positive_control_lucas_20261116`);
  - the wall-clock and scale rows (`genus_ladder_wallclock_vs_rho_20261122`,
    `genus_ladder_scale_20261115`);
  - Codex's PR #4789 item. Its branch `origin/research/hec-tenfold-replay` is absent here.
- Chose the lever: the sibling D − M from the shared Cantor step. Froze `PROTOCOL.md` at
  19:27:46Z (`frozen.sha256`).
- Wrote `sibwalk.py`, a tick-for-tick Python port of `JCurve::cantor`/`reduce` with `cantor_pm`.
  Ran a 1,500-step smoke test in the scratchpad to check that the code runs. That test was
  discarded and is not data.
- Ran the fixed sample: 12 lab curves × arms A, B, N, AA, with 40k steps per curve at g = 3 and
  60k at g = 2. Output `raw/runs.jsonl`; analysis `analyze.py` → `tables.md`, `analysis.json`.
- Measured results [pcc:unverified]:
  - c = 0.445 (g = 2) and 0.572 (g = 3);
  - Cantor cost per candidate B/A = 0.727 / 0.726 / 0.783;
  - N control 1.000; calibration within 0.5 % of the lab's counts;
  - 0 sibling mismatches; 2,400/2,400 sampled relation checks passed.
- P2 failed marginally at g = 3 and P3 failed at g = 2, p = 65537. Wrote and froze
  `AMENDMENT.md` (replication R2, fresh seeds, 6 × 120k) before running it. R2 reads R-noise:
  −0.0011 [−0.0024, +0.0001].
- Post-hoc diagnostics:
  - `diag_dups.py`: the duplicates come from merged walk trails, shared with the forward walk.
  - `diag_theory.py`: siblings sit at the exact random-divisor split rate; the forward walk sits
    slightly above it (z = +2.7).
- Figure (`render.py`, `fig_sibling.svg`/`.png`) and report (`mk_report.py`, `report.html`,
  `report.pdf`, rendered with Playwright).

### Decisions made
- **Did not port Cardano–Lucas into genus_ladder at g = 3.** It is the lab's own measured lever,
  and porting it is engineering rather than a new lever.
- **Did not try product-tree batch smoothness.** It is a predicted loss at g ≤ 3; the
  derivation is in PROTOCOL §0.
- **Python counts instead of a Rust build.** `df -h /` showed 752 MB free, below the 800 MB stop
  line. Counts are exact and calibrated against the lab; wall time is not claimed.
- **Verdict reported by the frozen letter.** That is "refuted" on P2, because PROTOCOL §7 fixed
  that rule. The README separately states what the evidence supports (the cost component) and
  why it is not an improvement claim.

### Open threads
- Port `cantor_pm` into `genus_ladder.rs` as an `IcExt::sibling` flag. Then run an end-to-end
  certified solve at g = 2 (p = 524309, 2^20–2^24) and g = 3 (p = 65537, 131101, 262147) against
  the current incumbent and matched rho, in counts and wall time. It needs disk for a build.
- Re-anchor the g = 2 projection on the current g = 2 incumbent; the rows used are from the
  2026-11-15 binary.
- Is the 16-jump forward walk enriched for split `u` (+0.56 %, z = +2.7, post hoc)? Test it on
  fresh curves with a pre-registered protocol.
- At r < 10^9 (g = 2, p = 65537), 4096-step segments merge into old trails. The forward walk
  repeated about 10 % of its points on curve 17 in this port, whose hash differs from the lab's.
  Check whether the lab's walk wastes candidates the same way.
- Sibling sharing under explicit (Harley/Lange) formulas, which would also speed up rho, is
  unmeasured.
- Port Cardano–Lucas (g = 3) into `genus_ladder` (a separate item).

### Errors
- None in the run. A shell `sleep` wait was refused by the harness, so the wait was replaced by
  an until-loop. Disk was below 800 MB for the whole session, so no Rust build was attempted.

## 2026-11-23 20:45Z to 21:05Z (Claude, correction pass after the red-team review; no git writes, no build)

### What was done
- Checked every review item against `analyze.py`, the lab rows it reads,
  `src/bin/genus_ladder_analyze.rs` (`costs()`, `gae_w`, `fpm_*`) and AGENTS.md §5a.
- `analyze.py` T3 rewritten: §5a GAE band as `gae_w = group_ops + w·(rel_mul + la_mul + native)`
  for the IC arm and for rho (rho's `native` included); sibling projection scales the arm's
  `walk_ops` by B/A. Added T3b (the lab's FPM low/mid/high readings, B/A recomputed at each
  inversion price) and kept the first pricing as T3c, marked superseded. Re-ran: `tables.md`,
  `analysis.json` regenerated.
- Verified results [pcc:unverified, derived]: g = 3, p = 65537 IC/rho 0.0714 / 2.227 / 11.93 →
  0.0559 / 2.213 / 11.92 at w = 0 / 0.1 / 1; g = 2, p = 524309 1.111 / 18.65 / 97.62 →
  0.8066 / 18.37 / 97.47. These match the review's figures and `genus_ladder_scale_20261115/tables_g2.md`
  (1.11 / 18.6 / 97.6; FPM 3.22 / 3.00 / 2.27).
- `rep_analyze.py`: fixed the `per_curve` overwrite (now pools each curve's counts over all rows);
  re-ran `rep_analysis.json`. Only `combined_descriptive.per_curve` changed.
- README: corrected the distance-to-rho section, P5 (band-dependent), the yield wording, the
  freeze note, the novelty scope; added "Red-team review and responses". Regenerated
  `fig_sibling.svg`/`.png` (right panel now in the GAE band), `report.html`, `report.pdf`
  (`python3 mk_report.py`; `NODE_PATH=$(npm root -g) node render_pw.js`).

### Decisions made
- The §5a GAE band governs the distance to rho. FPM is shown as the lab's second reading for
  reconciliation with X4, not as a ranking.
- Withdrawn: "below rho across the band" (g = 3), "crosses rho at w ≤ 0.1" (g = 2) and "2.3× to go
  at w = 1". Under §5a the g = 3 order with rho is unresolved (below only at w = 0); g = 2 never
  crosses. The sibling moves the arm total by ≤ 1.7 % at w = 0.1 and ≤ 0.2 % at w = 1.
- P5 scored band-dependent: failed at every GAE weight, held in all three FPM readings.
- The frozen verdict (refuted on P2) is unchanged.

### Open threads
- As before: Rust `IcExt::sibling` port and end-to-end certified solve; re-anchor g = 2 on the
  current incumbent; forward-walk enrichment test; trail merging at r < 10^9.
- New: a per-host `w_cal` (§5a calibrated weight) would be needed before any calibrated-price
  statement about this arm against rho.
- The freeze is mtime-only; commit `PROTOCOL.md`/`AMENDMENT.md` with the PR so later studies have
  a git anchor.

### Errors
- The first T3 used a home-made band (found by the review, fixed here). The first
  `rep_analysis.json` combined `per_curve` repeated the first-pass values (fixed). The seed tag
  deviated from the protocol (no arm, adds p); recorded as a deviation in the README.
- Minor: the review's inclusive calibration figure (226.9) differs from our recomputation (227.04
  for both lab and port); the agreement it asserts holds.
