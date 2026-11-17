# T9: select the PKM isogeny tower seed by J ÷ columns (protocol, 2026-11-03)

**Status:** frozen before data. Once frozen, this file is not edited. Any
change goes in a dated addendum at the end, written before the data it
governs.

**Class:**
- (a) is an exact census computation. It was done at design time from
  existing data, and it is reported here as a fact, not as a prediction.
- (b) is an engine component measurement in counted F4 multiply-adds.
- (c) is an end-to-end ecbench session in the counted unit S (GAE ÷ √r)
  across the §5a band.
- The lead is exploratory. No row reaches the IC leaderboard or
  `benchmarks/README.md` from this lead (no validation gate yet).

Parent study: `research/curve25519_isogeny_seed_20261103` (T4/T5). Lead
definition: `research/curve25519_torsion_leads_20261103/LEADS.md` (T9).

## Question

- End-to-end F4 work per solve scales about as columns × r ÷ coverage:
  - relations needed ≈ columns;
  - oracle calls per relation ≈ r ÷ covered points;
  - F4 work per call is about the same for every seed (parent R4: 0.991×).
- T5 selected the seed by J (the log-free count of signed base pairs whose
  [r]-images cancel), which maximises coverage. It picked seed 1093 (46
  columns, 528 covered).
- The end-to-end gain was 0.641× at w ≥ 0.1, not the 1/3 that coverage
  alone suggests, because the solve needs 46 relations instead of 25.
- **Question:** does selecting by J ÷ columns, which is also log-free, lower
  the end-to-end S of the PKM arm further than selecting by J does?
  - At k = 20, t = 6 this rule picks seed 1308 (36 columns, 498 covered).
  - The model predicts 0.497× seed 0 and 0.830× seed 1093.

## Fixed elements

- **Binaries** (prebuilt; not rebuilt):
  - `/home/user/ctarget/release/curve25519_family`, sha256 `ba1e3c05…a5ecef`;
  - `/home/user/ctarget/release/ecbench`, sha256 `54737834…29641b`.
  - Full hashes are in `scripts/run.sh`, which aborts on a mismatch.
  - The `curve25519_family` hash differs from the parent's (`7a438b1d…`).
    Control C0 below checks the parent engine numbers under the new binary.
- **Census input:**
  - `research/curve25519_isogeny_seed_20261103/data/k{16,18,20,22,24}.jsonl`;
  - 2,000 seeds per (k, t); sha256 checked by `run.sh`.
- **Curves:** the five Curve25519-family miniatures k = 16 (A = 1058),
  18 (78), 20 (926), 22 (43186), 24 (318).
  - (b) and (c) use only k = 20: p = 1048573, r = 131203, √r = 362.22.
- **Selection rules** (`scripts/select_seeds.py`; exact rational
  comparison):
  - J-selected (parent rule, unchanged): argmax J, ties broken by more
    columns, then the lower seed.
  - J/cols-selected (T9 rule): argmax J ÷ columns, ties broken by larger J,
    then the lower seed.
  - The rule reads only J and columns, which are log-free. Covered points
    are used only to evaluate the rule.
- **Model cost proxy:** M = columns ÷ covered_points.

## (a) Census analysis: exact, computed now from existing data

`python3 -I scripts/select_seeds.py` writes `analysis/census_t9.json`. The
table covers t ∈ {5, 6, 7} and all five curves. "M ratio" means the ratio
of M = columns ÷ covered.

| k | t | seed 0 cols/cov | J-sel seed: cols/cov | J/cols-sel seed: J, cols/cov | M J/cols-sel ÷ seed 0 | M J-sel ÷ seed 0 | M J/cols-sel ÷ J-sel |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 16 | 5 | 13/56 | 348: 22/172 | 1597: 162, 18/160 | 0.485 | 0.551 | 0.880 |
| 16 | 6 | 28/198 | 1381: 40/614 | 1381: 632, 40/614 | 0.461 | 0.461 | 1.000 |
| 16 | 7 | 68/1038 | 483: 82/1630 | 278: 1678, 74/1514 | 0.746 | 0.768 | 0.972 |
| 18 | 5 | 9/14 | 700: 25/194 | 997: 188, 19/186 | 0.159 | 0.200 | 0.793 |
| 18 | 6 | 30/230 | 1277: 45/554 | 1236: 480, 36/480 | 0.575 | 0.623 | 0.923 |
| 18 | 7 | 61/912 | 918: 84/1696 | 1955: 1680, 78/1644 | 0.709 | 0.740 | 0.958 |
| 20 | 5 | 18/72 | 1550: 26/214 | 1307: 210, 22/210 | 0.419 | 0.486 | 0.862 |
| **20** | **6** | **25/172** | **1093: 46/528** | **1308: 500, 36/498** | **0.497** | **0.599** | **0.830** |
| 20 | 7 | 65/1188 | 804: 86/1846 | 1594: 1854, 83/1840 | 0.824 | 0.851 | 0.968 |
| 22 | 5 | 13/40 | 1034: 21/224 | 1034: 224, 21/224 | 0.288 | 0.288 | 1.000 |
| 22 | 6 | 32/270 | 1070: 46/568 | 284: 458, 34/458 | 0.626 | 0.683 | 0.917 |
| 22 | 7 | 62/956 | 454: 81/1906 | 454: 1914, 81/1906 | 0.655 | 0.655 | 1.000 |
| 24 | 5 | 17/76 | 797: 24/190 | 978: 190, 20/190 | 0.471 | 0.565 | 0.833 |
| 24 | 6 | 39/364 | 1490: 41/514 | 1340: 380, 29/380 | 0.712 | 0.744 | 0.957 |
| 24 | 7 | 59/900 | 702: 86/1864 | 702: 1864, 86/1864 | 0.704 | 0.704 | 1.000 |

Exact facts (not predictions):
- **Better or equal everywhere.** In all 15 cells, the J/cols-selected seed
  has M ≤ the J-selected seed's M.
  - The two rules pick the same seed in 4 cells: (16,6), (22,5), (22,7),
    (24,7).
  - In the other 11 cells the M ratio is 0.793–0.972.
- **Close to the best seed.** In 14 of 15 cells the J/cols-selected seed has
  the lowest M of the 2,000 seeds. At (20,7) one seed is lower, and the
  J/cols-selected seed's M is 1.001× that seed's.
- **The engine and end-to-end cell is (20, 6):**
  - seed 1308: J = 500, 36 columns, 498 covered;
  - seed 1093: 46 columns, 528 covered;
  - seed 0: 25 columns, 172 covered.
- **The seed search costs the same under both rules.** Both rules read the
  same 2,000-seed census. At (20, 6) the search cost is the sum of
  build_group_ops + selector_group_ops, which is 13,528,559 group
  operations, or 3.735·10⁴ in S (÷ √r).

## (b) Engine, k = 20, t = 6, G = 4

Three runs, strictly sequential:
`curve25519_family f4cost 20 <form> 6 4 100 2026111506 3600 2 full 0 30000000`,
with `<form>` ∈ {`isogeny` (seed 0), `isogeny@1093`, `isogeny@1308`}. The
outputs are `data/engine_k20_t6_g4_s{0,1093,1308}.json`.

The derived end-to-end engine cost is E = columns ×
muladds_per_relation_at_exact_coverage. The parent values were:
- seed 0: E = 25 × 5.417·10⁸ = 1.354·10¹⁰;
- seed 1093: E = 46 × 1.749·10⁸ = 8.05·10⁹ (0.594× seed 0).

## (c) End-to-end ecbench session, k = 20

- **Spec:** `ecbench/spec.json` (sha256 `f5b110d5…0c360c`; `ecbench plan`
  gives spec id `ECS1haf677984322e`, 160 executions, no ordering warning).
  - The curve is the k = 20 miniature (a = 63700, b = 872826,
    group_order = 1049624, G = (349837, 122)).
  - There are 16 public targets with fresh target_seed 2026111509, used in
    no other curve25519 spec.
  - Measurement seed 2026111510, order `alternate_measured`, 2 measured
    rounds, 0 warm-up, timeout 1800 s, isolation L0.
- **Arms** (all PKM arms use oracle `pkm-semaev:m=2,mode=full,guess=4`):
  - `rho-neg` (reference);
  - `proj-a` (baseline; `prime-projected-abscissa:size=32,scan_cap=512`,
    `mitm:negation_folded=1`);
  - `pkm-iso-t6-g4` (`prime-isogeny:t=6`, seed 0);
  - `pkm-iso-t6-g4-s1093` (`prime-isogeny:t=6,seed=1093`);
  - `pkm-iso-t6-g4-s1308` (`prime-isogeny:t=6,seed=1308`).
- **Commands** (all in `scripts/run.sh`):
  - `ecbench run --wait --spec ecbench/spec.json --out data/ecbench/s1`;
  - `ecbench band --dir … --ranks --json data/ecbench/band.json`;
  - `ecbench compare --dir … --a A --b B --weight w` for w ∈ {0, 0.1, 1}
    and (B ÷ A) ∈ {s1308 ÷ seed 0, s1308 ÷ s1093, s1093 ÷ seed 0,
    s1308 ÷ proj-a, s1308 ÷ rho-neg}. Each compare is saved as text and
    JSON. The default bootstrap seed is 20261002 with 4,000 resamples.
  - `ecbench verify --dir … --replay 16 --out data/ecbench/verify.json`.
    The auditor seed comes from OS entropy and is recorded in the receipt.
- **Ratio and interval:** each prediction uses the point ratio and the 95%
  interval that `ecbench compare` reports at the given weight, without
  post-processing.
- **The seed search is reported separately.** It costs 3.735·10⁴ in S
  (above). The s1308 arm is reported twice: amortised (S as measured), and
  with the search charged to one target (S + 3.735·10⁴, at each weight).
  The seed-0 arm needs no search.

## Predictions (numbered; each decided by the rule stated, no retuning)

- **C0 (control, engine reproduction).** Under the new binary,
  `engine_k20_t6_g4_s0` and `engine_k20_t6_g4_s1093` reproduce the parent
  files `research/curve25519_isogeny_seed_20261103/engine/isogeny_t6_g4.json`
  and `isogeny_1093_t6_g4.json`.
  - The following fields must be exactly equal: columns, covered_points,
    f4_muladds, solving_degree_max, poly_terms_per_specialisation, hits,
    hits_verified.
  - **Holds** iff all are equal.
- **P1 (engine, exact census agreement).** `engine_k20_t6_g4_s1308` reports
  columns = 36 and covered_points = 498, which are the census values.
  - It also reports solving_degree_max = 5, poly_terms_per_specialisation =
    16, capped = false, undecided = 0, and hits_verified = hits.
  - **Holds** iff all of these hold.
- **P2 (engine, same per-target system).** muladds_per_target(s1308) ÷
  muladds_per_target(s0) ∈ [0.90, 1.10]. The parent ratio for s1093 was
  0.991.
- **P3 (engine, model cost).** E = columns ×
  muladds_per_relation_at_exact_coverage.
  - **Holds** iff E(s1308) ÷ E(s0) ≤ 0.55 (model 0.497) **and**
    E(s1308) ÷ E(s1093) ≤ 0.90 (model 0.830).
- **P4 (end-to-end, correctness).** Every run of every arm verifies: 32 of
  32 per arm, 160 of 160 in total. The replay audit reports 0 problems.
- **P5 (end-to-end, main point estimate).** At w = 0.1 **and** w = 1, the
  paired ratio S(s1308) ÷ S(seed 0) ≤ 0.60.
  - Model 0.497. The parent's measured-to-model ratio of 1.07 implies about
    0.53.
- **P6 (end-to-end, interval).** At w = 0.1 **and** w = 1, the upper bound
  of the 95% interval of S(s1308) ÷ S(seed 0) is < 1.
- **P7 (J/cols versus J).** At w = 0.1 **and** w = 1, the paired ratio
  S(s1308) ÷ S(s1093) ≤ 0.90 (model 0.830).
  - Whether its 95% interval excludes 1 is reported, but it is not
    predicted.
  - The band order between s1308 and s1093 is reported as `ecbench band`
    gives it.
- **P8 (w = 0).** The point ratio S(s1308) ÷ S(seed 0) at w = 0 is < 1. No
  interval claim. In the parent, s1093 gave 0.885 at w = 0.
- **P9 (replication of T5).** At w = 0.1, the point ratio S(s1093) ÷
  S(seed 0) ≤ 0.75. The parent measured 0.641 on 6 targets.
- **P10 (scope).** In `ecbench band --ranks`, all three PKM arms rank
  behind `proj-a` and behind `rho-neg` at w = 0.1 and w = 1.

## Order of analysis and failure handling

- **Run order.** `run.sh` runs (a), then (b), then (c). Only one heavy
  process runs at a time. If an identity check fails, the script aborts
  before any run.
- **Precondition.** The main session fills and pushes `.resume/RESUME.md`
  (`scripts/checkpoint.sh init`) before launching `run.sh`. `run.sh` does
  not run git.
- **Failed predictions are reported as failed.** Thresholds, seeds, the
  target set, arms, weights and the selection rule are not changed after
  data.
- **If C0 fails:** the binary change altered the engine. P1–P3 are still
  decided within this session's binary, because all three runs use the
  same binary. The C0 failure and the differing fields are reported, and
  no comparison is drawn across binaries.
- **If an f4cost run is capped or has undecided targets:** it is reported
  as such. P1 fails for that run, and P2/P3 are decided on the reported
  numbers with the cap noted.
- **If an ecbench run fails, times out or is censored:** it is reported as
  censored. P4 then fails. P5–P10 are decided on whatever `ecbench compare`
  pairs, and the number of pairs is stated.
- **If the session is interrupted** (host or process failure): one fresh
  session with the identical spec may be run into `data/ecbench/s2`, and
  this is reported. The interrupted session is kept and not mixed in.
- **If `verify` reports a problem:** the session's results are reported as
  unaudited and no prediction is called held.
- **Reports are in counted units only** (GAE, S, F4 multiply-adds). Wall
  time is diagnostic only (isolation L0).

## Scope and distance remaining

- This is a constant-factor improvement of the incumbent algebraic arm
  `pkm-iso-t6-g4`, made by choosing its factor-base seed differently. It is
  not a change of exponent.
  - At the parent's T5 point, the arm sits behind the generic baseline
    `proj-a` by 8.3·10⁴× and behind `rho-neg` by 6.9·10⁵× (w = 0.1).
  - Even at the model's 0.497× seed 0, that gap would remain about
    6.4·10⁴× (proj-a) and 5.4·10⁵× (rho-neg) at w = 0.1. These figures
    use the T5 seed-0 values on T5's targets; this session's own band
    replaces them.
- The report states S(s1308) ÷ S(proj-a) and S(s1308) ÷ S(rho-neg) at
  w = 0, 0.1 and 1 from this session's band and compares as the distance
  remaining. **Rho remains ahead.** No row here is a leaderboard claim.
- The seed search (3.735·10⁴ in S at k = 20, t = 6) is reported separately,
  both amortised and charged to one target. With the search charged to one
  target, the w = 0 band order against seed 0 is not expected to favour
  s1308.
- The result holds for one curve (k = 20), one tower depth (t = 6) and one
  guess size (G = 4). The census in (a) shows the rule's M advantage at the
  other cells, but no end-to-end claim is made there.
