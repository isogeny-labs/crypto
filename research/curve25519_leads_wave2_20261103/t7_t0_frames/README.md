# T7: frame selection for the T₀-invariant base (`t0-bits@lo`), k = 20: results

Date 2026-11-03. Wave 2, lead T7. Protocol: [PROTOCOL.md](PROTOCOL.md), frozen in git
before any data (freeze commit `2b330f250a1bca382b81023d95d556fccd7078d3`). The
executor ran `scripts/run.sh` unchanged, once, from 17:05:12Z to 17:08:42Z (UTC,
`data/run.log`).

Class: a component measurement on the algebraic track (`pkm-semaev`, F4 over the bit
tower, m = 2, mode full). The metric is counted F4 mul-adds only. Wall times are in
the JSON, but no claim uses them. This lead makes no leaderboard row and no
`benchmarks/` row, and it does not pass the validation gate (there is no matched rho
row for a component result).

## Answer

- **Frame selection works for `t0-bits`, and it is large.** Selecting the frame raises
  coverage by 20.5× (t = 4), 5.9× (t = 5) and 2.9× (t = 6) over lo = 0. It cuts the
  derived F4 cost per relation by the same factors: 0.049×, 0.17× and 0.34× the cost of
  `t0-bits@0`. Per-target cost stays the same as `t0-bits@0` and as `bits` (P3 held).
- **The selected `t0-bits@L*` beats the best known `bits@L` per relation in all 4 primary
  cells, by 0.951× (t = 4) and 0.934× (t = 5).** P4 predicted the opposite (> 1) and
  **failed**. At t = 6 (secondary cell) it is 1.25× more expensive than `bits@908172`.
  The `bits@L` and `t0-bits` censuses differ in size and draw, so this is not a matched
  selection comparison. Also, `t0-bits@L*` needs more columns (18 against 11 at t = 4,
  28 against 24 at t = 5); see the exploratory items.
- **Decision (P7, held): `t0-bits@L*` is the cheapest algebraic arm per relation in
  0 of 5 measured cells.** The isogeny tower is cheapest in every cell: `isogeny@S*` in
  the 4 primary cells and `isogeny@1093` at t = 6. It is ahead of `t0-bits@L*` by 6.0×,
  3.9×, 32×, 17× and 79×. The only improvement T7 can claim is over the `t0-bits`
  baseline itself (P6 held).
- **Distance remaining: unchanged. Rho remains ahead** (see the last section).

## Instrument and integrity

- Binary `/home/user/ctarget/release/curve25519_family`, sha256
  `7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be`. It matched the
  frozen hash before the run (`data/binary.sha256`) and again after it. `lo_list.txt`
  matched `84dd67a9…a5c5`.
- Census: 3 × 1000 rows, 0 errors (`census_errors.log` was not created), empty stderr
  files.
- Engine: 28/28 cells with `rc=0` in `.status` (but see erratum E1: that rc is `date`'s, not the binary's; the validator's replay recorded the binary's own rc = 0 in 28/28 cells) and empty `.stderr`. None capped, none
  undecided. Load average at cell starts was 5.84 to 7.91 (a sibling lead's f4cost was
  running at the same time; T7 itself ran one process at a time). Counts are
  deterministic, so load cannot change them.

## Phase 1–2: census and selection (`data/selection.json`, verbatim)

| t | n lo | mean covered | sd | lo = 0 (cols, covered) | lo = 0 percentile | L* | L* cols | L* covered | gain vs mean | gain vs lo = 0 | t0 mean / bits mean | L* / best bits@L covered |
|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| 4 | 1000 | 12.224 | 10.5636 | (6, 4) | 0.159 | 940986 | 18 | 82 | 6.7081 | 20.5 | 0.64575 | 1.0513 (78) |
| 5 | 1000 | 41.688 | 26.0714 | (14, 36) | 0.453 | 388414 | 28 | 212 | 5.0854 | 5.8889 | 0.59745 | 1.0707 (198) |
| 6 | 1000 | 144.752 | 60.8070 | (32, 148) | 0.558 | 113278 | 56 | 424 | 2.9291 | 2.8649 | 0.53405 | 0.7910 (536) |

Selection rule (frozen and log-free): argmax `covered_points`; ties go to fewer columns,
then to the earlier position in the list. At t = 4 two lo values reach 82. The rule
resolved the tie inside `select.py`. Selected equals oracle-best by construction (see
PROTOCOL.md).

Search cost, derived by `select.py` (`search_group_ops_derived`, summed over 1000 lo):

| t | group ops | Legendre/sqrt lookups |
|--:|--:|--:|
| 4 | 2,122,020 | 16,000 |
| 5 | 7,860,633 | 32,000 |
| 6 | 29,300,272 | 64,000 |

This is a per-curve precomputation. Charged to a single target, it alone is
2.3·10³–5.4·10⁴ times a whole k = 20 rho solve (547–936 group ops).

## Phase 3: engine cells (`data/engine/*.json`, verbatim)

m = 2, mode full, 100 random targets, target seed `2026111500 + t`, plus 20 planted
targets reported separately. `per-rel` = `muladds_per_relation_at_exact_coverage`.

| cell | form | cols | covered | deg | terms | f4_muladds | mul-adds/target | per-rel | hits/ver | planted h/e/d |
|---|---|--:|--:|--:|--:|--:|--:|--:|:--|:--|
| t0lo0_t4_g0 | t0-bits@0 | 6 | 4 | 6 | 121.0 | 404606932 | 4046069.32 | 132714108247.99 | 0/0 | 15/15/15 |
| t0sel_t4_g0 | t0-bits@940986 | 18 | 82 | 6 | 121.0 | 404607829 | 4046078.29 | 6473873291.254512 | 0/0 | 20/20/20 |
| bitslo0_t4_g0 | bits | 7 | 12 | 6 | 121.0 | 404608545 | 4046085.45 | 44238212441.3625 | 0/0 | 18/18/18 |
| bitsbest_t4_g0 | bits@797820 | 11 | 78 | 6 | 121.0 | 404608323 | 4046083.23 | 6805875102.893462 | 0/0 | 17/17/17 |
| isosel_t4_g0 | isogeny@595 | 15 | 64 | 5 | 16.0 | 52287072 | 522870.72 | 1071909485.5649999 | 0/0 | 18/18/18 |
| iso1093_t4_g0 | isogeny@1093 | 12 | 34 | 5 | 16.0 | 52287100 | 522871.0 | 2017713053.3235295 | 0/0 | 19/19/19 |
| t0lo0_t4_g3 | t0-bits@0 | 6 | 4 | 4 | 22.0 | 8883985 | 88839.85 | 2914013709.8875003 | 0/0 | 15/15/15 |
| t0sel_t4_g3 | t0-bits@940986 | 18 | 82 | 4 | 22.0 | 8884000 | 88840.0 | 142147250.24390244 | 0/0 | 20/20/20 |
| bitslo0_t4_g3 | bits | 7 | 12 | 4 | 22.0 | 8884000 | 88840.0 | 971339543.3333333 | 0/0 | 18/18/18 |
| bitsbest_t4_g3 | bits@797820 | 11 | 78 | 4 | 22.0 | 8884000 | 88840.0 | 149436852.82051283 | 0/0 | 17/17/17 |
| isosel_t4_g3 | isogeny@595 | 15 | 64 | 4 | 8.0 | 1759200 | 17592.0 | 36064424.625 | 0/0 | 18/18/18 |
| iso1093_t4_g3 | isogeny@1093 | 12 | 34 | 4 | 8.0 | 1759200 | 17592.0 | 67885975.76470588 | 0/0 | 19/19/19 |
| t0lo0_t5_g0 | t0-bits@0 | 14 | 36 | 6 | 256.0 | 12729371631 | 127293716.31 | 463925485028.35913 | 1/1 | 19/19/19 |
| t0sel_t5_g0 | t0-bits@388414 | 28 | 212 | 6 | 256.0 | 12728295032 | 127282950.32 | 78773136466.20264 | 0/0 | 20/20/20 |
| bitslo0_t5_g0 | bits | 11 | 42 | 6 | 256.0 | 12728434431 | 127284344.31 | 397621138726.30786 | 0/0 | 18/18/18 |
| bitsbest_t5_g0 | bits@976503 | 24 | 198 | 6 | 256.0 | 12728430774 | 127284307.74 | 84343853678.84454 | 0/0 | 20/20/20 |
| isosel_t5_g0 | isogeny@1550 | 26 | 214 | 5 | 16.0 | 395876105 | 3958761.05 | 2427109000.2016354 | 1/1 | 20/20/20 |
| iso1093_t5_g0 | isogeny@1093 | 17 | 80 | 5 | 16.0 | 395725390 | 3957253.9 | 6490044793.02125 | 0/0 | 20/20/20 |
| t0lo0_t5_g3 | t0-bits@0 | 14 | 36 | 5 | 64.0 | 301041333 | 3010413.33 | 10971535003.7775 | 1/1 | 19/19/19 |
| t0sel_t5_g3 | t0-bits@388414 | 28 | 212 | 5 | 64.0 | 303691087 | 3036910.87 | 1879489702.2481604 | 0/0 | 20/20/20 |
| bitslo0_t5_g3 | bits | 11 | 42 | 5 | 64.0 | 303691176 | 3036911.76 | 9486950801.125713 | 0/0 | 18/18/18 |
| bitsbest_t5_g3 | bits@976503 | 24 | 198 | 5 | 64.0 | 303691611 | 3036916.11 | 2012386385.7592423 | 0/0 | 20/20/20 |
| isosel_t5_g3 | isogeny@1550 | 26 | 214 | 5 | 16.0 | 17646738 | 176467.38 | 108191820.83242992 | 1/1 | 20/20/20 |
| iso1093_t5_g3 | isogeny@1093 | 17 | 80 | 5 | 16.0 | 17711200 | 177112.0 | 290470321.7 | 0/0 | 20/20/20 |
| t0lo0_t6_g4 | t0-bits@0 | 32 | 148 | 5 | 88.0 | 4553605367 | 45536053.67 | 40368019254.49332 | 0/0 | 20/20/20 |
| t0sel_t6_g4 | t0-bits@113278 | 56 | 424 | 5 | 88.0 | 4473975894 | 44739758.94 | 13844317434.44533 | 2/2 | 19/19/19 |
| bitsbest_t6_g4 | bits@908172 | 42 | 536 | 5 | 88.0 | 4519476730 | 45194767.3 | 11062852712.802052 | 1/1 | 20/20/20 |
| iso1093_t6_g4 | isogeny@1093 | 46 | 528 | 5 | 16.0 | 70384751 | 703847.51 | 174899441.01236743 | 1/1 | 20/20/20 |

Every cell: `targets_done` 100, `undecided` 0, `capped` false.

### Per-relation ratios of `t0-bits@L*` (from `analysis/summary.json`)

The unpriced native counters of every cell are listed under "Validator errata (applied)", E4.

| cell | vs t0-bits@0 (P6) | vs best bits@L (P4) | vs min isogeny (P5) | cheapest arm |
|---|--:|--:|--:|---|
| t = 4, G = 0 | 0.04878 | 0.95122 | 6.0396 (isogeny@595) | isogeny@595 |
| t = 4, G = 3 | 0.04878 | 0.95122 | 3.9415 (isogeny@595) | isogeny@595 |
| t = 5, G = 0 | 0.16980 | 0.93395 | 32.456 (isogeny@1550) | isogeny@1550 |
| t = 5, G = 3 | 0.17131 | 0.93396 | 17.372 (isogeny@1550) | isogeny@1550 |
| t = 6, G = 4 (secondary) | 0.34295 | 1.25142 | 79.156 (isogeny@1093) | isogeny@1093 |

P3 mul-adds/target ratios of `t0-bits@L*`: against `t0-bits@0` 1.0000022, 1.0000017,
0.99992, 1.0088, 0.98251; against the best `bits@L` 0.9999988, 1.0, 0.99999, 0.9999983,
0.98993. Degree and terms are equal in every pair.

## Predictions (frozen rules; decided strictly)

| | prediction | rule | measured | outcome |
|---|---|---|---|---|
| P0a | census anchors reproduce lead E | 5/5 equal | `anchor_mismatches: []`; (4,0)→(6,4), (5,0)→(14,36) shown above | **held** |
| P0b | 6 engine replays equal in f4_muladds and covered | 6/6 | 8884000/78, 303691176/42, 12728434431/42, 303691611/198, 12728430774/198, 70384751/528, all equal | **held** |
| P0 | instrument | 5 + 6 | 11/11 | **held** |
| P1 | t0 census mean / bits mean in [0.3, 0.7] at t = 4, 5, 6 | 3/3 | 0.6457, 0.5975, 0.5340 | **held** |
| P2a | covered(L*) / mean ≥ 1.5 at t = 4, 5, 6 | 3/3 | 6.708, 5.085, 2.929 | **held** |
| P2b | covered(L*) / covered(lo = 0) ≥ 2 at t = 4, 5 | 2/2 | 20.5, 5.889 | **held** |
| P3 | same deg/terms as t0-bits@0 and best bits@L; mul-adds/target ratios in [0.95, 1.05]; coverage = census | 5/5 cells | all equal; ratios 0.9825–1.0088; coverage equal | **held** |
| P4 | per-rel(t0-bits@L*) / per-rel(best bits@L) > 1 in the 4 primary cells | 4/4 | 0.9512, 0.9512, 0.9340, 0.9340 | **failed** (0/4) |
| P4b | that ratio in [1.2, 5] | 4/4 | same values | **failed** (0/4) |
| P5 | per-rel(t0-bits@L*) / min isogeny ≥ 2 | 5/5 | 6.04, 3.94, 32.5, 17.4, 79.2 | **held** |
| P6 | per-rel(t0-bits@L*) / per-rel(t0-bits@0) ≤ 0.5 | 5/5 | 0.0488, 0.0488, 0.170, 0.171, 0.343 | **held** |
| P7 | t0-bits@L* cheapest (< 0.99 × min others) in 0 of 5 cells | 0/5 | 0/5 (isogeny cheapest in all 5) | **held** |
| P8 | planted h = e = d, hits_verified = hits, undecided 0, not capped | 28/28 | 28/28 | **held** |

Why P4 failed: the basis assumed that the best `bits` frame (4.1× and 2.8× above the
bits mean) would out-cover the best `t0-bits` frame. It did not. The t0 census spreads
much wider than its mean (sd/mean 0.86 at t = 4, 0.63 at t = 5), and its maximum
slightly exceeds the best known bits frame (82 against 78, 212 against 198). P3 held, so
the per-relation ratio equals that coverage ratio to within 1.1e-5 relative (78/82 = 0.9512; 198/212 = 0.9340; measured 0.951218, 0.951220, 0.933952, 0.933961).
P7 did not fail, so no fresh-seed confirmation is triggered.

## Exploratory (not preregistered)

- **Columns grow with selected coverage.** Across the census, covered points and columns
  correlate 0.831, 0.880 and 0.936 (t = 4, 5, 6). Columns range 0–20, 2–30 and 10–56,
  and 10 lo values at t = 4 give an empty base. L* has 18, 28 and 56 columns, against
  11, 24 and 42 for the best bits frames. Covered points per column for `t0-bits@L*` are
  4.56, 7.57 and 7.57; for the best `bits@L` they are 7.09, 8.25 and 12.76. The linear
  algebra needs about one relation per column. On a crude "columns × per-rel" proxy,
  `t0-bits@L*` is about 1.56× (t = 4) and about 1.09× (t = 5) **more** expensive than
  the best bits frame, in spite of the cheaper per-relation cost. That proxy is not a
  frozen metric, and this is not a claim.
- A covered-per-column selection would choose different frames: t = 4 → lo 785212 (14
  columns, 74 covered); t = 6 → lo 934310 (28 columns, 250 covered). At t = 5 it is the
  same L* (388414). None was engine-measured.
- The bits and t0 reference frames come from different censuses (2000 frames for bits at
  t = 4, the frame-search set at t = 5) with different draws. The P4 comparison is
  therefore between best-found frames, not between matched-size selections.

## Costs

- Census: 3000 f4cost calls with targets = 0, no F4 work. About 1.3 minutes elapsed.
- Engine: 28 cells. Total `f4_muladds` (random targets) 68,333,109,436, plus
  `planted_muladds` 11,475,927,434. Elapsed time 17:06:29Z–17:08:42Z.
- Derived selection search cost: 2.12·10⁶, 7.86·10⁶ and 2.93·10⁷ group ops (table above).
  The protocol expected about 10⁶–10⁷; t = 6 is above that range.

## Distance remaining (unchanged by T7)

- The best algebraic arm measured end to end is 18× (w = 0), 9.6·10⁵× (w = 0.1) and
  5.9·10⁶× (w = 1) behind rho-neg at k = 20 (`curve25519_algebraic_ecbench_20261102`).
  The generic index-calculus baseline in that study's own session is 7.04 / 7.06 / 7.17× rho-neg (w = 0 / 0.1 / 1); the 7.8× figure is an earlier 48-run baseline it quotes.
- Inside the algebraic track, the cheapest per-relation arm in every T7 cell is still the
  isogeny tower. `t0-bits@L*` is 3.9×–79× behind it.
- The selection search alone costs 10⁶–10⁷ group ops, against 547–936 for a whole rho
  solve at k = 20.
- None of this changes an order across the §5a band. **Rho remains ahead.**

## Deviations

- None in the frozen plan: `run.sh` and `select.py` ran unchanged, with no retries and no
  missing rows.
- The lab's pre-run duties were not done in this container: no Conductor task, no
  `.resume/RESUME.md`/S3 checkpoint, and the run was local rather than on the EKS route.
  The task said to run locally, and PROTOCOL.md assigns these duties to the launching
  session.
- Before launch, the executor made 3 untracked timing probes
  (`f4cost 20 t0-bits@0 t 3 0 1 …`, t = 4, 5, 6, output discarded). They gave the same
  census rows as the recorded ones.
- The executor ran read-only `git log`/`git status` on this directory once, to confirm
  the freeze commit. It made no commit and no push.
- No report PDF or figure was produced (the `research-visuals` duty), because this task
  was limited to the README and SHA256SUMS. `analysis/analyze.py` is a post-hoc decision
  script written after data. It is not part of the frozen plan.

## Files

`data/census_t{4,5,6}.jsonl`, `data/selection.json`, `data/selection.txt`,
`data/engine/<tag>.{json,stderr,status}`, `data/run.log`, `data/binary.sha256`,
`data/census.SHA256SUMS`, `data/engine/SHA256SUMS`, `data/SHA256SUMS` (all data files),
`analysis/analyze.py`, `analysis/analyze.out` and `analysis/summary.json`.

## Validator errata (applied)

Source: `validator-report.json`. No prediction verdict changed.

- E1: disclosed, not fixable here. In the frozen `scripts/run.sh`, `echo "end $(date ...) rc=$?"`
  records `date`'s exit status, not the binary's, so "28/28 rc=0 in .status" is not evidence of
  the engine's exit code. Instrument text amended to say so. No effect on results: the
  validator's replay recorded the binary's own rc = 0 in 28/28 cells with identical outputs.
  The logger should be fixed in future scripts; frozen evidence is not rewritten.
- E2: load average at cell starts corrected to 5.84 to 7.91.
- E3: "exactly" in "Why P4 failed" changed to "to within 1.1e-5 relative", with the measured
  ratios.
- E4: unpriced native counters listed below (from `data/engine/*.json`, field `native`; "n/a"
  means the counter is not present in that cell's JSON). The validator's band check (0, 0.1 and
  1 per native op) leaves every verdict and the cheapest arm in every cell unchanged.
- E5: the generic baseline cited from `curve25519_algebraic_ecbench_20261102` corrected to that
  study's own session value 7.04 / 7.06 / 7.17× rho-neg; 7.8× is an earlier 48-run baseline it
  quotes. The same sentence appears in "Distance remaining" (fixed there; PROTOCOL.md not edited).
  The 547-936 rho group-op range comes from the frame-search README and was not re-derived by
  the validator.

Unpriced native counters per cell (E4):

| cell | system Fp mults | grid Fp mults | t0 target Fp mults | t0 target inversions | t0 base map Fp mults | t0 base inversions |
|---|--:|--:|--:|--:|--:|--:|
| bitsbest_t4_g0 | 12300 | n/a | n/a | n/a | n/a | n/a |
| bitsbest_t4_g3 | 31700 | n/a | n/a | n/a | n/a | n/a |
| bitsbest_t5_g0 | 21300 | n/a | n/a | n/a | n/a | n/a |
| bitsbest_t5_g3 | 67700 | n/a | n/a | n/a | n/a | n/a |
| bitsbest_t6_g4 | 177457 | 4544 | n/a | n/a | n/a | n/a |
| bitslo0_t4_g0 | 11100 | n/a | n/a | n/a | n/a | n/a |
| bitslo0_t4_g3 | 31100 | n/a | n/a | n/a | n/a | n/a |
| bitslo0_t5_g0 | 19300 | n/a | n/a | n/a | n/a | n/a |
| bitslo0_t5_g3 | 65900 | n/a | n/a | n/a | n/a | n/a |
| iso1093_t4_g0 | 1800 | n/a | n/a | n/a | n/a | n/a |
| iso1093_t4_g3 | 11200 | n/a | n/a | n/a | n/a | n/a |
| iso1093_t5_g0 | 1800 | n/a | n/a | n/a | n/a | n/a |
| iso1093_t5_g3 | 14400 | n/a | n/a | n/a | n/a | n/a |
| iso1093_t6_g4 | 28548 | 4544 | n/a | n/a | n/a | n/a |
| isosel_t4_g0 | 1800 | n/a | n/a | n/a | n/a | n/a |
| isosel_t4_g3 | 11200 | n/a | n/a | n/a | n/a | n/a |
| isosel_t5_g0 | 1800 | 1248 | n/a | n/a | n/a | n/a |
| isosel_t5_g3 | 14346 | 1248 | n/a | n/a | n/a | n/a |
| t0lo0_t4_g0 | 12300 | n/a | 3800 | 100 | n/a | n/a |
| t0lo0_t4_g3 | 31700 | n/a | 3800 | 100 | n/a | n/a |
| t0lo0_t5_g0 | 21300 | 1248 | 3800 | 100 | n/a | n/a |
| t0lo0_t5_g3 | 67107 | 1248 | 3800 | 100 | n/a | n/a |
| t0lo0_t6_g4 | 178800 | n/a | 3800 | 100 | n/a | n/a |
| t0sel_t4_g0 | 12300 | n/a | 3800 | 100 | n/a | n/a |
| t0sel_t4_g3 | 31700 | n/a | 3800 | 100 | n/a | n/a |
| t0sel_t5_g0 | 21300 | n/a | 3800 | 100 | n/a | n/a |
| t0sel_t5_g3 | 67700 | n/a | 3800 | 100 | n/a | n/a |
| t0sel_t6_g4 | 175667 | 13632 | 3800 | 100 | n/a | n/a |
