# T7: frame selection for the T₀-invariant base (`t0-bits@lo`), k = 20

Date 2026-11-03. Wave 2, lead T7. Status: **frozen before data**. No measurement
has been taken for this lead. The only inputs read while writing this were existing
files:

- `research/curve25519_t0_algebraic_20261102` (README, `run_cells.sh`, `cells/*.json`);
- `research/curve25519_frame_search_20261103` (README, `analysis/summary.json`, `engine/*.json`,
  `scripts/run_engine.sh`);
- `research/curve25519_torsion_leads_20261103` (README, PROTOCOL, `data/k20.json`,
  `engine_confirm/*.json`);
- `research/curve25519_isogeny_seed_20261103` (PROTOCOL, `data/k20.jsonl`, `engine/*.json`,
  `analysis/summary.json`);
- the `f4cost` source in `src/bin/curve25519_family.rs` and `t0_factor_base` /
  `t0_abscissae` in `src/cryptanalysis/ic_framework/pkm_semaev.rs` (read only, to fix
  what `covered_points` means and how `lo` wraps).

## Question

The `t0-bits` base holds every point whose T₀-invariant `w = z + d/z` lies in
`[lo, lo + 2^t)`. It is closed under T₀ and −. Lead E (`curve25519_t0_algebraic_20261102`)
measured it only at hand-chosen `lo` (0, 1, 34, 68). There the F4 system had the same
degree, terms and mul-adds per target as `bits`, but lower `coverage_exact`, so it cost
1.2–3.0× more per relation. Frame selection was large and free for `bits`
(F4/relation 4.71× lower at t = 5, 3.33× at t = 6) and for the isogeny seed (3.10× at
t = 6, G = 4).

T7 asks:

1. How much does choosing `lo` raise the coverage of `t0-bits` (census over 1000 `lo`)?
2. Does the selected `t0-bits@L*` keep the per-target F4 cost of `t0-bits` (lo = 0) and of
   `bits`?
3. **Decision:** is frame-selected `t0-bits` the cheapest algebraic arm per relation in
   any measured cell, against `t0-bits` (lo = 0), `bits` (lo = 0), the best known `bits@L`,
   and the isogeny tower at its census-selected seed and at seed 1093?

## Instrument

- Binary `/home/user/ctarget/release/curve25519_family`, sha256
  `7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be` (read 2026-11-03).
  It is the binary of the isogeny-seed engine cells. It is not the binary of lead E
  (`fa565fd4…8846`), of the frame study (`0459a725…b4c5`) or of the torsion pilot
  (`8e6dff92…07ae`). Do not rebuild or edit it. `run.sh` refuses to start if the hash
  differs (F1).
- Census call, one per (t, lo):
  `curve25519_family f4cost 20 t0-bits@<lo> <t> 3 0 1 3600 2 full 0 30000000`
  (targets = 0: the base is built, the oracle is prepared and the exact m = 2 coverage is
  computed; no F4 call is made). `F4COST_PLANTED` is unset.
- Engine call, one per cell:
  `F4COST_PLANTED=20 curve25519_family f4cost 20 <form> <t> <G> 100 <2026111500+t> 3600 2 full 0 30000000`
  (m = 2, mode full, lifts 0, max_nnz 30,000,000, 100 random subgroup targets, then 20
  planted targets that are reported separately and do not enter `muladds_per_target`).
- Wrapper: `timeout 4000`, `ulimit -v 3500000`. Strictly sequential: one engine
  process at a time on the shared 4-core host. Counts are deterministic, so order and load
  cannot change a count. Load can only trip the cap (F3). No timing is used for any claim.
- What the fields mean (from the source). `covered_points` = the number of distinct points
  `R ≠ O` with `[r]R = O` among all sums of 2 base points (`decomposable_points` followed
  by the subgroup filter). `coverage_exact` = `covered_points / r`. The F4 cost per relation
  is **derived**: `muladds_per_relation_at_exact_coverage` = `muladds_per_target /
  coverage_exact`. No relation needs to be returned for it.

## Curve

The k = 20 family-rule miniature: A = 926, p = 1048573, r = 131203, #E = 8r. The binary
derives it from k. Other family curves (k = 16, 18, 22, 24) are out of scope here.

## Design

### Phase 1: census (coverage only)

- **lo values:** `scripts/lo_list.txt` holds 1000 values, sha256
  `84dd67a906ad91ab48f4d333c335b3da6d0c540ffa6588a106dcf2086b54a5c5`, generated now
  (no measurement):
  - four anchors 0, 1, 34, 68 (the `lo` of lead E);
  - then 996 distinct values from `random.Random(2026110307).randrange(1, p)`,
    skipping repeats and anchors.

  The same list is used for every t. `lo + k` wraps mod p in `t0_abscissae`, so every
  `lo` in `[0, p)` is a valid frame.
- **t:** 4, 5 (primary) and 6 (secondary). That is 3 × 1000 = 3000 census calls.
- Output: `data/census_t<t>.jsonl`, one engine JSON line per lo.

### Phase 2: selection rule (frozen, log-free)

- `L*(t) = argmax covered_points` over the 1000 lo.
- Ties: fewer `columns` first, since equal coverage with fewer columns needs fewer
  relations for the linear algebra. Then the earlier position in `lo_list.txt`.
- **Why this is log-free.** The engine computes `covered_points` from point additions
  over the base (all signed pairs) and one `[r]Q = O` test per distinct sum. No discrete
  logarithm is used. So the rule selects on a quantity any attacker can compute from the
  public curve. It is not an oracle on the target's log.
- It is also the selection target itself. Unlike the `J` proxy of the earlier studies,
  selected = oracle-best by construction, so no "selector vs oracle" prediction is made.
- **Search cost (priced, derived from counts):** per lo,
  - 2^t quotient lookups (one Legendre symbol / square root each);
  - N(N+1)/2 additions for the m = 2 sums of the N = `base_points` points;
  - one `[r]Q` per distinct sum (`covered_points_all_cosets`), at 1.5·log2 r group
    operations each.

  The total over the 1000 lo is reported, both charged to one target and as an amortised
  per-curve precomputation (`select.py`, field `search_group_ops_derived`).
- `scripts/select.py` writes `data/selection.json` and passes `L*(4), L*(5), L*(6)` to
  phase 3.

### Phase 3: engine cells

Target seed `2026111500 + t` for every arm of a (t, G) cell. This is the seed of the
frame-search, torsion Addendum A and isogeny-seed engine cells, so the reference arms
replay exactly (P0b).

Reference frames and seeds are frozen now from existing files:

- **Best known `bits@L`** (engine-measured, highest coverage at k = 20):
  - t = 4: `bits@797820`, covered 78 (torsion `engine_confirm/bits_797820_t4.json`;
    `data/k20.json` max over 2000 frames is 78);
  - t = 5: `bits@976503`, covered 198 (frame search `engine/frame_t5_g*.json`; the
    torsion census max is 184 at `bits@233902`);
  - t = 6: `bits@908172`, covered 536 (frame search `engine/frame_t6_g3.json`).
- **Isogeny at the census-selected seed** (isogeny-seed rule, argmax J, then columns,
  then lowest seed, over seeds 0–1999 in `data/k20.jsonl`; selected = oracle at both t):
  - t = 4: `isogeny@595`, J = covered 64, 15 columns;
  - t = 5: `isogeny@1550`, J = covered 214, 26 columns.
- **`isogeny@1093`** (the T4 seed, selected at t = 6). Census coverage: 34 (t = 4),
  80 (t = 5), 528 (t = 6).

Cells:

| block | t | G | arms (in run order) |
|---|---|---|---|
| primary | 4 | 0, 3 | `t0-bits@0`, `t0-bits@L*(4)`, `bits`, `bits@797820`, `isogeny@595`, `isogeny@1093` |
| primary | 5 | 0, 3 | `t0-bits@0`, `t0-bits@L*(5)`, `bits`, `bits@976503`, `isogeny@1550`, `isogeny@1093` |
| secondary | 6 | 4 | `t0-bits@0`, `t0-bits@L*(6)`, `bits@908172`, `isogeny@1093` |

That is 24 primary + 4 secondary = 28 engine cells. Output: `data/engine/<tag>.json`,
`.stderr` and `.status`. The tags are listed in `run.sh`.

The prior lead-E t0 cells used target seed `2026110200 + t`, so they are not replays here.
They are cross-checked through the census anchors (P0a).

## Predictions (frozen; each with its decision rule)

**P0 (instrument).**
- (a) The census rows reproduce lead E exactly, as (t, lo) → (columns, covered_points):
  (4, 0) → (6, 4); (5, 0) → (14, 36); (5, 1) → (12, 34); (6, 34) → (26, 94);
  (6, 68) → (22, 102).
- (b) The engine replays match the published cells exactly in `f4_muladds` and
  `covered_points`:

  | cell | `f4_muladds` | `covered_points` |
  |---|--:|--:|
  | `bits@797820` t4 G3 | 8,884,000 | 78 |
  | `bits` t5 G3 | 303,691,176 | 42 |
  | `bits` t5 G0 | 12,728,434,431 | 42 |
  | `bits@976503` t5 G3 | 303,691,611 | 198 |
  | `bits@976503` t5 G0 | 12,728,430,774 | 198 |
  | `isogeny@1093` t6 G4 | 70,384,751 | 528 |

  Decision: P0 holds iff all 5 + 6 equalities hold.

**P1 (census mean).** For each t ∈ {4, 5, 6}, the mean of `covered_points` over the 1000
lo, divided by the `bits` translation-frame census mean at k = 20 (18.93, 69.776, 271.048
from the frame study), lies in [0.3, 0.7]. Held iff all 3 ratios are inside.
- Basis: a w-class holds {±P, ±(P+T₀)}. Its pairs cancel into E[r] on cosets mod 4
  instead of mod 8, and both points of a class give the same subgroup sum. So there are
  about half the distinct subgroup sums of `bits` at equal t.
- Lead E's single lo values gave 0.21–0.52.

**P2 (selection gain).**
- (a) `covered(L*) / mean covered ≥ 1.5` at each t ∈ {4, 5, 6} (3/3 required).
- (b) `covered(L*) / covered(lo = 0) ≥ 2` at t = 4 and at t = 5 (2/2 required).

**P3 (per-target invariance in the engine).** In each of the 5 (t, G) cells:
- `t0-bits@L*` has the same `solving_degree_max` and `poly_terms_per_specialisation` as
  `t0-bits@0` and as the best-known `bits@L` of that t;
- its `muladds_per_target` ratio against each of them is in [0.95, 1.05];
- its `coverage_exact` equals the census value for L*.

Held iff all of this holds in 5/5 cells.

**P4 (primary decision ingredient: against the best bits frame).** In each of the 4
primary cells, the derived F4 per relation satisfies
`per-rel(t0-bits@L*) / per-rel(best bits@L) > 1`. Held iff 4/4. Its value is also
predicted in [1.2, 5] in 4/4 (P4b).
- Basis: with P3, the ratio equals `covered(bits@L) / covered(L*)`. P1 puts the t0 mean
  at about half the bits mean.
- The best bits frames are themselves 4.1× (t = 4) and 2.8× (t = 5) above the bits mean.

**P5 (against the isogeny tower).** In each of the 4 primary cells and the t = 6, G = 4
cell,
`per-rel(t0-bits@L*) / min(per-rel(isogeny@S*), per-rel(isogeny@1093)) ≥ 2` (at t = 6
only `isogeny@1093`). Held iff 5/5.
- Basis: at t = 6, G = 4 the isogeny system has 16 terms per specialisation against 154
  for `bits` at G = 3, and 0.70 M mul-adds per target against 172 M.
- The selected isogeny coverage (64, 214, 528) is at or above what P1 and P2 allow for
  `t0-bits`.

**P6 (improvement over the t0 baseline).** In each of the 4 primary cells and the t = 6
cell, `per-rel(t0-bits@L*) / per-rel(t0-bits@0) ≤ 0.5`. Held iff 5/5. This is the only
improvement T7 can claim, and only over `t0-bits` itself.

**P7 (decision).** `t0-bits@L*` is the cheapest arm per relation in **0 of 5** measured
cells.
- Rule: an arm is cheapest in a cell iff its derived per-relation cost is below 0.99 × the
  minimum over the other arms of that cell.
- An arm with `undecided > 0` or `capped = true` is excluded from the minimum and
  flagged (F3).
- If P7 fails (t0-bits@L* is cheapest in ≥ 1 cell), that cell goes to a fresh-seed
  confirmation (new target seed, new lo list draw) before any claim is made.

**P8 (completeness and exactness).** In all 28 cells:
- `planted_hits = planted_exact = planted_done`;
- `hits_verified = hits`;
- `undecided = 0`, `capped = false`.

Held iff 28/28.

## Failure handling

- **F1. Instrument and inputs.** If the binary or `lo_list.txt` hash differs, `run.sh`
  stops before any call. If a census row is missing, `select.py` exits 4 and phase 3 does
  not run. Census call failures are listed in `data/census_errors.log`, and the call is
  retried once by re-running `run.sh` (it is resumable). Rows still missing are reported
  with their count, and L* is taken over the rows present only if ≤ 1% are missing,
  labelled as a deviation.
- **F2. P0a mismatch.** `select.py` exits 3 and the engine phase does not run. The
  mismatch is reported as an instrument change, and no prediction is decided on this
  binary. A P0b mismatch does not stop the run: every prediction is still decided, and the
  report states that the reference arms were re-measured, not replayed.
- **F3. Cap or budget.** A cell with `capped = true` or `undecided > 0` is reported as
  is, with its counts, and is excluded from P7's minimum. It is never re-run with a
  different cap, max_nnz or target count.
- **Failed predictions are reported as failed.** No band, rule, lo list, seed or arm is
  retuned after data. Anything computed from these data that is not listed above (for
  example m = 3 coverage, other G, a J-style proxy, or coverage-vs-columns fits) is
  labelled **exploratory**.

## Scope and distance remaining

- This is a component measurement on the algebraic track (`pkm-semaev`, F4 over the bit
  tower):
  - one curve (k = 20), m = 2, mode full;
  - t ∈ {4, 5} (primary) and t = 6 (secondary);
  - G ∈ {0, 3} (primary) and G = 4 (secondary);
  - one target seed per t, 100 random and 20 planted targets.

  Nothing is claimed about m = 3, other G, other k or curves, the (w, D) presentation of
  lead E, 255-bit Curve25519, a complete DLP, or wall time.
- It makes no leaderboard (tables A–C) or `benchmarks/` row. There is no same-unit
  matched rho row for a component result, so the validation gate is not passed by
  construction.
- **Distance remaining (unchanged by any T7 outcome):**
  - the best algebraic arm measured end to end is 18× (w = 0), 9.6·10⁵× (w = 0.1) and
    5.9·10⁶× (w = 1) behind rho-neg at k = 20 (`curve25519_algebraic_ecbench_20261102`);
  - the generic index-calculus baseline is 7.8× rho there;
  - a rho solve at k = 20 takes 547–936 group operations, and the census search here
    alone is expected to cost about 10⁶–10⁷ group operations (priced in `selection.json`).

  Even if P7 fails in T7's favour, a per-relation cut of a few × on the algebraic track
  changes no order across the §5a band. **Rho remains ahead.**

## Run conditions (for the session that launches `run.sh`)

- `run.sh` makes no git call and writes only under this directory (`data/`).
- The lab's pre-run duties (Conductor task, `.resume/RESUME.md` checkpoint and S3 push,
  the EKS route) belong to the launching session. If they are absent in this container,
  the report records that, as earlier wave studies did.
- Expected size: 3000 census calls without F4 work, then 28 engine cells. The largest is
  t = 5, G = 0, at about 1.3·10¹⁰ mul-adds per cell, from the published 127 M per target.
