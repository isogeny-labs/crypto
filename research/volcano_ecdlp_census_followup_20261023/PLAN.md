# Volcano ECDLP census, follow-up: a second j = 0 volcano and a deeper ℓ = 3 volcano

Status: **preregistered, frozen before any data** (2026-10-23). Changes after
measurement starts go to `AMENDMENTS.md`, each timestamped with a reason.

This study extends `research/volcano_ecdlp_census_20261017/`, called "the
census" below (PRs #2067 and #2402, validated). It reuses the census's
instrument, accounting and analysis unchanged unless this file says otherwise:

- the frozen ecbench binary, sha256 `ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35`;
- the arms, the transported-instance construction (`planted_shared` with
  φ(G_src)), the families, the A1.6 storage rules, and the census's
  `ANALYSIS_PLAN.md`;
- AMENDMENTS A2.1 (uncharged counts from `records.jsonl`) and A2.3 (curve =
  census vertex for resampling), which apply from the start here. Their
  post-hoc labels apply only to the census.

## 0. Questions

1. **Replication on a deeper ℓ = 3 volcano (V6).** Does the census finding
   hold on a volcano about 3× V2's size with one more level, depth 4? The
   finding is that rho, BSGS and IC cost does not depend on volcano level, and
   that no curve stands out beyond its isomorphic copies.
2. **A second j = 0 volcano (V5), ℓ = 2, depth 6, 64 curves.**
   - (a) Replicate the positive control: `rho-cubic` against `rho-neg` on the
     j = 0 crater.
   - (b) A level analysis on a j = 0 volcano. V3 had one curve per upper
     level; V5 has 8, 16 and 32 curves at levels 4–6.
3. **H-T: carry up, then solve with the automorphism.** On V5, carry an
   instance from any level up to the j = 0 crater with verified 2-isogenies,
   then solve it there with `rho-cubic`. Is that cheaper than solving it
   where it was born with `rho-neg`, once the transport is charged? Theory
   says about 1/√3 ≈ 0.58×, plus a transport cost below 1%. The census
   measured both pieces separately; this measures the composition.
4. **H-M: does IC cost depend on the Weierstrass model?** This is the census's
   finding F2. On the V3 crater, isomorphic models cost IC 57 / 67 / 73. This
   test checks whether that is a general IC property. The `prime-abscissa`
   factor base depends on x-coordinates, which change under (x, y) ↦ (u²x, u³y),
   or whether it is noise. Rho and BSGS are the comparison, because their cost
   should not depend on the model.

A null result on any question is a result.

## 1. Volcanoes (searched in Phase A; seeds recorded before the search)

| id | ℓ | requirement | expected size |
|:--|:-:|:--|:--|
| V5 | 2 | j = 0 crater (inert), depth 6, p ≡ 1 (mod 8), 26-bit, largest prime r with 3 \| r − 1 (so `rho.cubic` runs), cofactor ≤ 64 | 1 / 1 / 2 / 4 / 8 / 16 / 32 = 64 |
| V6 | 3 | split crater, depth 4, 26-bit, cofactor ≤ 64, whole volcano ≤ 2,000 vertices | about c·(1, 2, 6, 18, 54) with c ≈ 10–15 |

Each volcano is walked whole with `ca volcano --census-out --edges-out --ks`
(cryptanalysis `claude/volcano-census-export-20261017`). Per-level counts are
cross-checked with `volcano-rho --curve` at its radius. A truncated walk is
refused.

The first hit that meets a row's requirement is accepted. This is a size and
shape rule, not an outcome rule.

## 2. Instances and arms

Everything is as in the census (PLAN §3), with these additions:

- **Families on V6:** D, Ua, Ub, S, N and ctl.
- **Families on V5:** D, Ua, Ub, N and ctl. The crater has one vertex, so
  there is no S.
- **Extra V5 family `Tcr`, for H-T:** for every non-crater vertex v, the
  instance born at v is carried up to the crater and measured there.
  - Construction: a `prime_explicit` crater curve with generator ψ_v(G_v) and
    `planted_shared` targets, using the N family's target seed at v, so k_i is
    shared with v's own rows.
  - One spec per source vertex. All are on the same crater curve but have
    different generators.
  - Arms: `rho-cubic` and `rho-neg`.
  - The comparison is with v's own N rows under `rho-neg`.
  - Transport cost is measured per path with `ca volcano --transport-bench`.
- **`rho-cubic`** runs on every V5 crater spec. This arm is valid only where
  j = 0, which on V5 is the crater alone.
- **Extra family `M`, for H-M:** 8 isomorphic models per chosen curve,
  u drawn by a seeded hash with u ∉ {±1}, all with the native generator
  carried by the isomorphism. Chosen curves:
  - the V5 crater, plus 3 seeded V5 vertices from levels ≥ 4;
  - 8 seeded V6 vertices, spread over the levels.

  Arms: `rho-neg`, `bsgs`, `ic`. Use 16 targets per model and 4 rounds, so
  each model has 64 solves per arm.

## 3. Sizing and budget

The rule is unchanged: at least 2,000 `rho-neg` solves per level over the
isogenous families, using 8 targets per curve and more rounds on small levels.

- Each session is at most 30 minutes.
- The budget is 20 h of session wall time.
- A guard stops the campaign at 20 h, at under 1.5 GiB of free disk, or after
  3 failed audits in a row.
- The order is: V5 (all families, M), then V6 M, then V6 by family: D, Ua, Ub,
  S, N, ctl. An early stop therefore still leaves complete strata.

**Preregistered reduction.** Phase A estimates the total. If the estimate
exceeds 20 h, V6 `ctl` is cut to a seeded random subset of 200 vertices per
level for every level with more than 200 vertices. Other families are never
cut. If the estimate still exceeds 20 h, V6 `S` is dropped, with a coordinator
amendment. Nothing else may be reduced.

**Disk.** Free disk is about 1.4 GB. The driver archives and verifies each
session and prunes its raw directory as it goes (census A1.6, enforced per
session). No cargo build is allowed: the frozen binary is reused.

## 4. Analysis (frozen before Phase B)

Phase A writes `ANALYSIS_PLAN.md` by copying the census's plan with only the
changes listed below. It is committed before any Phase B session runs.

- **Q1 (V6):** census ANALYSIS_PLAN §3–§5 and §7–§9 as written. The TOST
  margins are the same.
- **Q2a (V5):** census §6. Pass if the 95% CI upper bound is below 0.85.
- **Q2b (V5):**
  - Census §3 is applied, with level 0 as the reference. Levels 1–3 have
    1–4 curves, so they are reported but their verdict is "descriptive".
  - Levels 4–6 get TOST verdicts.
  - `rho-cubic` is excluded from the level analysis.
- **H-T:** for each source vertex v,
  T_v = [mean S(`rho-cubic`, Tcr_v) + p · Vélu maps(path_v)] / mean S(`rho-neg`, N_v)
  at p = 0, 0.1 and 1 GAE, plus the pooled value per level. Use a cluster
  bootstrap over source vertices, 10,000 reps, seed 20261023. The `rho-neg` on
  `Tcr` gives T_v's control: the same transport, but no automorphism.
  - **Supported:** the pooled CI is below 1 at all three prices, and the
    control's CI includes 1.
  - **Refuted:** the pooled CI includes 1 or lies above it.
  - **Inconclusive** otherwise.
- **H-M:** for each chosen curve and arm, a one-way test across its 8 models:
  - statistic F = between-model variance / within-model variance;
  - permutation p, with 10,000 shuffles of rows across models, seed 20261024;
  - Holm across the curves of each arm.

  H-M is **supported for IC** if at least half the chosen curves are
  significant for `ic` after Holm, while `rho-neg` and `bsgs` are significant
  on at most 1 curve each. It is **refuted** if `ic` is significant on at
  most 1 curve. Otherwise it is **inconclusive**.

  The effect size is reported as the max/min model-mean ratio with a CI. The
  V3 crater values from the census (57 / 67 / 73) are quoted as the motivating
  observation, not pooled.
- **Wall time:** census §8.
- **IC against matched rho:** census §9, with the V5 crater against
  `rho-cubic` (census A3 F1).

## 5. Phases

| phase | executor | content | gate |
|:--|:--|:--|:--|
| A | executor agent | Seeded searches; whole-volcano walks and cross-checks; transported-instance identity on every curve (as census G-A); specs for D/U/S/N/ctl/Tcr/M; manifest and estimate, applying §3's reduction rule mechanically; `ANALYSIS_PLAN.md`. Commit and push. | G-A: every curve's identity check passes; `ecbench plan` passes on every spec; estimate ≤ 20 h after reduction; analysis plan committed |
| B | executor agent | The campaign under the census driver (`xz -t` before each progress line, prune as it goes); transfer-cost bench on V5/V6 paths; pushes every 4 sessions | G-B: every finished session audited OK, archived, pushed |
| C | executor agent | `analyze.py` to the frozen plan; figures; REPORT and PDF; benchmarks copy (no README row) | G-C |
| V | independent validator | replay certificate, own-code recompute, VALIDATION.json | gate valid, then README row |

## 6. Storage

git is the copy of record. Session archives are committed if the total
projected xz size is at most 200 MB. Drive gets the plan, the progress summary
and the report text (census A1.5).

## 7. Not claimed

- No speedup claim beyond H-T's measured ratio. H-T's composition is known in
  theory; this study measures it end to end.
- No claim about cryptographic sizes.
- One volcano per question.
