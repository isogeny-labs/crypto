# LEAD-19 — Cofactor-projected factor base on the random-binary ladder (LEAD-04, arm C)

**Verdict.** constant-factor candidate (not an exponent; the oracle is a pair table and a hash probe, so it sits on the generic line).
**Confidence.** medium: the mechanism is checked exactly on the three curves (class census, table census, walked targets stay in the subgroup) and the waste it removes is visible in the frozen counters; the size of the effect on `S` is a desk model, and the baseline rows are 3-seed means with a heavy tail.
**Expected gain if it works.** Desk estimates, unverified, in `S` (GAE per √r, Table B). Lever only (projected base at its best rung against the best raw rung with the class-test charge removed): about 1.4–1.5× on `f2m15` and `f2m21` (the card's 1.4–1.6× is this number), about 3× on `f2m27`. Against the published rows (26.3, 24.4, 36.9): about 2.8×, 2.1×, 3.4×. Distance to matched rho (13.1×, 18×, 38.2× now) would become about 4.7×, 8.7×, 11×: still above rho on every row.
**Cost of the discriminating test.** About 1 person-day of Rust (150–200 lines with tests), then under 5 CPU-minutes with S₄ off and under 0.5 GiB. Landing a row adds the validation gate (replay certificate, validator report, §5a band).

## 1. Mechanism (precise)

This is not a Gröbner lead. The three rows are `mitm_m2_negfold_walk`: a negation-folded pair table of `a²` additions (`a` = abscissae = columns), one walked target per trial, one hash probe, about `K/2` relations to close the elimination through a cycle (ledger §11.2). There is no polynomial system and no degree of regularity; the bound the row obeys is the ledger's shape law `ops(F) = F²/4 + c·#E/(2kF)`, with the **`#E` in the relation term** (the counting ceiling `C(F+1,2)/#E`).

**Group structure** (frozen `docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json`; class census is an *unverified desk check*, an exact computation in a scratchpad script, not committed):

| row | curve | `#E` | h | `E(F)` | leaderboard rung (signed points) |
|:--|:--|--:|--:|:--|:--|
| `icv1-f2m15-t131-ed7256f0` | `n15-b524b`, a=1 | 32,638 = 2·16,319 | 2 | Z/2 × Z/r | std, dim 5 (30) |
| `icv1-f2m21-t2591-de54d72e` | `n21-b1b6f3b`, a=1 | 2,094,562 = 2·1,047,281 | 2 | Z/2 × Z/r | balanced, dim 6 (62) |
| `icv1-f2m27-t12543-569dca8b` | `n27-b845462`, a=1 | 134,205,186 = 6·22,367,531 | 6 | Z/2 × Z/3 × Z/r | balanced, dim 8 (254) |

`Tr(a) = Tr(1) = 1` (n odd), so the 2-torsion point `(0, √b)` is not halvable and `E(F)[2^∞] = Z/2`; for h = 2, `2E = ⟨G⟩` and `P ∈ ⟨G⟩ ⇔ Tr(x) = Tr(a)`. On every base checked, the abscissae with `Tr(x) = Tr(a)` are exactly those with `[r]P = O` (7/7, 4/4, 29/29, 14/14). For `f2m27` the 2-part is the same trace test; the 3-part has no cheap test (3 ∤ 2²⁷−1, so no cubic character), only `[r]P`.

**The base is raw.** `run_char2_instance` (`ic_boundary.rs` L7760) uses `binary_subspace_factor_base` on `⟨1, z, …, z^{l−1}⟩`, no projection. `Tr(1) = 1` makes `Tr` non-zero on the window, so the base splits across classes (unlike `f2m18`/`f2m24`, a=0, even n, whose base the ledger shows already lies in `2E`).

**What lies outside ⟨G⟩** (desk, exact; first lift per abscissa, `−P` has class `−c`):

| rung | abscissae | outside ⟨G⟩ | folded table entries in ⟨G⟩ |
|:--|--:|--:|--:|
| f2m15 std / bal | 15 / 7 | 8 (53%) / 3 | 113/225 (50.2%) / 25/49 |
| f2m21 std / bal | 61 / 31 | 32 (52%) / 17 | 1,865/3,721 (50.1%) / 485/961 |
| f2m27 std / bal | 263 / 127 | 224 (85.2%) / 109 (85.8%) | 11,551/69,169 (16.7%) / 2,719/16,129 (16.9%) |

Walked targets stay in ⟨G⟩: the first point is `[a]G + [b]Q`, the 16 jumps and the restart offsets come from `draw_jumps` / `draw_pool_offset` as `[α]G + [β]Q` with `Q = [d]G` (`ic_boundary.rs` L6346–6519). So a class-nonzero table entry can never match: the raw yield per trial is `N₀/r` (the ledger's "exact" ceiling, ≈ uniform/1, measured `yield/ceiling` 0.8–1.1), a fraction `1/h` of what the same table would give inside ⟨G⟩.

**Arm B (project, same |F|).** For each abscissa take one point `P`, `Q = [h]P` (h = 2: one doubling; h = 6: three operations), keep `±Q`, one column per distinct abscissa of `Q`. `[h]` kills `E[h]` and is an automorphism of ⟨G⟩, so every `Q ∈ ⟨G⟩` (desk: all points in ⟨G⟩; 0 abscissa collisions at the three bases; 3,718/3,721 distinct table abscissae at f2m21 std, 218/225 at f2m15 std). Every table entry is useful: yield per trial ×`h` at fixed `F`, table additions unchanged. The unknown of a column is `log_G(Q)`; the linear algebra and verification are untouched.

**Arm C (subgroup only, equal |F|).** Scan a larger window, keep points in ⟨G⟩, truncate to the same `|F|`. h = 2 rows: membership is `Tr(x) = Tr(a)`, one masked parity, window dimension `l+1`. `f2m27`: `[r]P = O`, about 36 GAE per candidate and about 1,000 lifted abscissae to find 127 in-subgroup ones (14.2–14.8% hit rate), so C costs 4–8 units of `S` more than B there.

**Predicted yield change.** Trials per relation fall by `h` (1/valid fraction: 1.99, 1.98, 5.99, 5.93 from the census). Relations *needed* rise slightly because B's relation graph is one component while A's is several (h = 2: 2; h = 6: 4 abscissa types `{0},{3},{1,5},{2,4}`), and a union of small graphs closes its first cycle earlier. Unverified desk simulation (random signed edges mod a prime, `d` determined when `e_d` enters the row space): edges to close, A vs B, `K = 31, 61`: 16.7 vs 18.4, 29.2 vs 35.1; `K = 127, 263` (h = 6): 49.8 vs 63.9, 90.2 vs 122.3. Net trial ratio A/B: 1.64–1.76 (h = 2), 4.4–4.7 (h = 6). A's measured relation counts agree with the simulation's A (e.g. 90.3 vs 90.2 at K = 263).

**Which part of the published `S` is not this lever.** `run_char2_instance` runs `summands_admissible` for m = 3 and m = 2 on the std base (L7789–7796): `[r]P` for every base point twice (60, 244, 1,052 scalar mults) and merges the cost into the std rung's factor-base phase only; the balanced rung is never charged. At f2m15 (published rung = std) that is about 1,540 of 1,758 GAE, about 12.0 of 26.3 units; at f2m21 std about 7.9 of 25.0, at f2m27 std about 8.1 of 41.7. Arm B needs no class test. That saving is accounting (arm A could skip it), so the lever is measured against **A′ = A minus the class-test operations**, and the published-row ratio is reported beside it.

## 2. Prior art: literature

- Kosters, Yeo, *Notes on summation polynomials*, arXiv:1503.08001v3, 2015, https://arxiv.org/pdf/1503.08001. **read-at-source** (PDF through `pdftotext`). Prop. 4.2: surjective morphism `E(F_{2^n}) → F₂`, `P ↦ Tr((x(P)+a₂)/a₁²)`, kernel `2E(F)`. Remark 4.8: "one can just add the equation from the start ... Another option is to only look for relations in the kernel of the map". No cofactor or speed-up figure.
- Fong, Hankerson, López, Menezes, *Field inversion and point halving revisited*, IEEE Trans. Computers 53(8), 2004, https://www.ece.uvic.ca/~fayez/research/gf3m/fong_j04.pdf. **read-at-source**. Lemma 3.1.4: for `G` of odd order, `(x,y) ∈ ⟨G⟩ ⇒ Tr(x) = Tr(a)`; the converse comes from KY's "kernel = 2E".
- Galbraith, Granger, Merz, Petit, *On index calculus algorithms for subfield curves*, ePrint 2020/1315. **abstract-only** (page 200; no cofactor or subgroup statement on it).
- Galbraith, Gaudry, *Recent progress on the elliptic curve discrete logarithm problem*, ePrint 2015/1022. **abstract-only**; no statement on cofactors.
- KN-FIND-007 (yield conservation): `E[c_D(r)] = C(B+m−1,m)/N` for every base. Read in the repository. It fixes `N` as the order of the group the sums live in; B/C change `N` from `#E` to `r` (targets are in ⟨G⟩), not the geometry, so it predicts this lead rather than closing it.
- Corrigan-Gibbs–Kogan, Shoup, CGHW 2026/384, DKM 2025/783: **recalled**, via `LEADS.md`.
- Search scope: one web search on cofactor multiplication in index-calculus factor bases returned only general index-calculus pages. I found no source that states the base projection for binary random curves; that is not proof none exists.

## 3. Prior art: these repositories

- `src/cryptanalysis/koblitz_index_calculus.rs` L3000–3120, `build_subgroup_orbit_factor_base_with_cost`: "Multiply by the cofactor rather than rejecting: `[h]P` has order dividing `r` for every `P`, so no sample is wasted". **Supports** (the lever exists for Koblitz; absent for random binary). Also `ProjectedFactorBase` (L9164), a column fold, not a base change.
- `src/cryptanalysis/ic_framework/plugins.rs`: `PrimeProjectedAbscissaBase` (L137–375), `compact-orbit-scan` (doc table L15: "subgroup projection and signed Frobenius closure"). `BinarySubspaceBase` (L379–415) has no projection hook. **Supports**; the binary-random hook does not exist.
- `src/cryptanalysis/ic_framework/fiber.rs` and `research/isogeny_conductor_gap_ic_20261010/README.md` §3.1: fiber-aware oracles. Random binary n = 17, 19, h ∈ {4,6,8}: yield ÷ blind 3.5; `lifts` cost per relation 0.99 (no gain per operation), `closed` (h× table) 0.65 per relation but `S` 5.8× blind. **Supports** the raw-base waste (measured) and shows the table-pays-for-it version; B gets the yield without the h× table.
- `src/cryptanalysis/ic_boundary.rs`: `run_char2_instance` L7760–7983, `summands_admissible` L4777, `binary_subspace_factor_base` L4619, `exact_decomposition_ceiling` L467. **Supports**; shows the admissibility asymmetry.
- `research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md` §3.5, §10.1–10.5, §11.2: exact ceiling is accounting, "no S moves"; shape law uses `#E`. **Supports**; the projection changes the `#E` the law divides by to `r`, which §11.2 treats as given.
- `docs/ic/LEADERBOARD.md` Table B (rows, phase shares), `docs/ic/boundary_targets.json` L4712 (a = 0, even n bases lie in 2E). **Supports**.
- `research/pdp_literature_leads_20261022/leads/LEAD-04.md` §4, §6: arm C, `A^{2/3}` law, negative control. **Supports**; its desk 1.4–1.6× is B vs A′ on h = 2.
- `docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json`: all phase, group-op and native counters used above. `docs/VALIDATION_GATE.md`.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does not touch either; these rows have no Gröbner step. It lowers `S` through the relation phase, which is 95.9% (f2m21), 90.7% (f2m27) and 47.1% (f2m15) of the published `S`.

1. Per trial the cost is unchanged (about 1.17 GAE at f2m21, 1.04 at f2m27: a walk step, a guard probe, a lookup).
2. Trials fall by 1.64–1.76 (h = 2) and 4.4–4.7 (h = 6) (§1; assumption: class-balanced base, confirmed by the census; relation-graph model, unverified).
3. Table and base: unchanged table, plus `a` doublings (h = 2) or `3a` operations (h = 6), minus the class test where the rung pays it.

Desk `S` (unverified; GAE/√r; relation phase divided by the trial ratio):

| row | A published | A′ best rung | B at same F | B best | B/published | B/A′ | gap to rho now → B |
|:--|--:|--:|--:|--:|--:|--:|:--|
| f2m15 (rho 2.01) | 26.3 (std) | 14.3 (std) | 9.4 | 9.4 | 2.8× | 1.5× | 13.1× → 4.7× |
| f2m21 (rho 1.36) | 24.4 (bal) | 17.0 (std) | 14.3 (bal) | 11.9 (std) | 2.1× | 1.43× | 18× → 8.7× |
| f2m27 (rho 0.966) | 36.9 (bal) | 33.6 (std) | 11.0 (bal) | 11.0 | 3.4× | 3.1× | 38.2× → 11.4× |

Cross-check with the shape law: re-optimised gain `h^{2/3}` = 1.59 (h = 2), 3.30 (h = 6), reduced by the closure effect. The published f2m21 and f2m27 rows are 3-seed means with heavy tails (f2m27: 25.5, 59.5, 25.7), so these ratios are good to about ±30%.

## 5. Obstructions

- **O1, prime order (theorem).** `h = 1`: `[1]` is the identity, B and C are bit-identical to A. All eight Table B prime rows are unmoved.
- **O2, Koblitz is already done.** `build_subgroup_orbit_factor_base` and `compact-orbit-scan` project; Table A/C bases are not this lead (which recipe each uses was not opened).
- **O3, accounting inside the baseline.** The published rows mix a diagnostic charge (f2m15) and an asymmetric rung choice (f2m21's best raw rung without the class test is the std base, 17.0, not the balanced 24.4). A claim must be lever-only (B vs A′), or it is partly relabelling.
- **O4, closure.** B needs more relations than A; the naive `h×` is an upper bound (1.64–1.76 vs 2, 4.4–4.7 vs 6 in the simulation).
- **O5, noise.** Five repeats give a paired-ratio standard error near 0.15–0.2 in log terms (per-seed `S` spread in the frozen rows is 0.27–0.55 CV); fine against the 1.1× kill, thin for a tight interval.
- **O6, algebraic oracles.** B destroys the subspace form, so S₄ pairs-and-solve cannot use it. C at h = 2 is the Kosters–Yeo trace equation on the abscissa bits (already implemented as `trace_constraint`; no degree change measured per LEAD-04 §3).
- **O7, generic line.** Group operations, `[h]`, membership tests and a hash table: Corrigan-Gibbs–Kogan/Shoup apply; no structure on a `q^{-1/2}` fraction is used; the `r^{1/6}` shape of the family is unchanged (ledger §11.2). Constant factor only, and every row stays above matched rho.
- **O8, native work.** Fewer lookups and guard probes in B, equal table inserts and `as_solves`: B ≤ A at every price `w ∈ {0, 0.1, 1}` GAE per unpriced native op, so the A/B order holds across the §5a band (to be recorded, not assumed).

## 6. Minimal discriminating test (frozen protocol)

**Code first (research Rust, not Python).** New arms in `run_char2_instance` and `run_prime_instance` (`src/cryptanalysis/ic_boundary.rs`) and one flag in `src/bin/ic/boundary.rs`; the oracle, pair-table, walk, linear algebra and verification code are not edited.
- `project_factor_base(inst, fb, h)`: about 35 lines, modelled on `binary_subspace_factor_base` and `FactorBase::from_column_map`; `[h]P` charged to the base phase, dedupe by abscissa, drop `[h]P = O`. `h = 1` returns the base unchanged with zero operations.
- `subgroup_factor_base(inst, basis_dim, want_abscissae)`: about 40 lines; trace test (h = 2, counted as a native op) or `[r]P = O` (charged); truncate to `want`.
- Wiring: variants `…_walk_projected[_balanced]` and `…_walk_subgroup[_balanced]` appended per `--base-arms raw,projected,subgroup` (default `raw`: output byte-identical to today); class census (class sizes, class-valid table entries) and `admissibility_adds` / `admissibility_doubles` counters next to the existing `admissibility_scalar_mults` (so A′ is exact); three unit tests (projected base ⊂ ⟨G⟩; class-valid fraction 1.0; h = 1 identity). About 150–200 lines. The analysis script (paired ratios, bootstrap) goes under `docs/ic/tools/` like the existing `boundary_*.py`; ~60 lines, to be added.

**Instances.** Primary: `f2m15`, `f2m21`, `f2m27` as above. Secondary, same command, no extra code: `f2m18`, `f2m24` (cofactor 8, 4 classes, base in `2E`; predicted yield about ×4, an unverified prediction from ledger §10.5). Negative control: `icv1-fp24-t1577-77336155`.

**Commands** (after the code PR; `ic boundary` fixes the curve by `--seed`, so "5 seeds" is `--repeats 5`, and repeats 0–2 use the frozen seeds 123212651130, 123212691633, 123212732136):
1. `["cargo","build","--release","--bin","ic"]`
2. `["./target/release/ic","boundary","--regime","char2","--char2-degrees","15,18,21,24,27","--repeats","5","--seed","123212651130","--s4-max-degree","0","--base-arms","raw,projected,subgroup","--out","research/lead19_projected_base/char2.json"]`
3. `["./target/release/ic","boundary","--regime","prime","--prime-bits","24","--repeats","5","--seed","123212651130","--s4-max-degree","0","--base-arms","raw,projected,subgroup","--out","research/lead19_projected_base/prime24.json"]`

`--s4-max-degree 0` skips S₄ (937 of the 951 s at n = 27 in the frozen run). Metric is counted group operations, so no isolation tier applies; wall times are a practicality note.

**Recorded** per instance, repeat and arm: `S`, per-phase GAE, group ops, native counters, trials, `relations_found`, relations per trial, `verified`, `trials_over_floor_exact`, class census, class-valid table entries, `[h]P` / selection / admissibility ops, the matched rho `S` of the same process, and `S` at `w = 0, 0.1, 1`.

**Checks that stop the run (bugs, not results).** Any `verified` false. fp24 `raw`, `projected`, `subgroup` not bit-identical in every counter. Arm A repeats 0–2 on the three primary rows not equal to the frozen round-5 counters (`trials`, `relations_found`, `total_gae`); if the code has drifted, report it and keep the in-process A as baseline.

**Predictions.** P1: class-valid fraction of A's table 0.50 / 0.50 / 0.17 within 10% of the census. P2: pooled relations per trial, B/A, within `[0.7h, 1.3h]` (`[1.4, 2.6]`, `[4.2, 7.8]`). P3: `S(A′)/S(B)` 1.4–1.5 (f2m15, f2m21), about 3 (f2m27); `S(A)/S(B)` per §4. P4: B and C relations per trial within 15% on the h = 2 rows (all sums in ⟨G⟩ for both). P5: fp24 exactly 1.000.

**Kill criterion** (the card's, made exact): mean paired `S(A′)/S(B)` below 1.10 on all three primary rows, or the fp24 control moving, or P2 failing (then the mechanism is misread: stop and report). Success: `S(A′)/S(B) ≥ 1.10` on at least one row with the paired 95% bootstrap lower bound above 1 (5 repeats, 10,000 resamples, fixed seed), all answers verified. If the interval straddles 1.10, the pre-registered extension is `--repeats 20` on the same command, no other change. Report each row's gap to matched rho at the three prices; both ratios (A′ and published) are reported, only A′ is the lever.

**Cost.** Code about 1 person-day plus review; run under 5 CPU-minutes (frozen non-S₄ wall at n = 27 about 15 s for 3 repeats), under 0.5 GiB. A leaderboard row, benchmarks entry and report/graph/PDF are a further step with the validation gate.

## 7. Ranking inputs

expected gain: 1.4–1.5× (h = 2 rows), about 3× (f2m27) lever-only, desk | confidence: medium (mechanism high, magnitude medium) | cost: about 1 person-day, under 5 CPU-minutes | tier: constant | depends on: the Rust arms; the validation gate for any leaderboard row.

## 8. Open questions

- Whether the relation-graph closure model holds at `K = 127–263` with h = 6; the run measures `relations_found` directly.
- Whether B's best rung differs from the leaderboard rung (the desk model puts it at the std base for f2m21, 122 points); needs the extra rung `l_bal − 1` and `l_std`, which `--base-arms` already covers via the two bases.
- Table A/C Koblitz bases: confirmed only that `compact-orbit-scan` and `build_subgroup_orbit_factor_base` project; which recipe each row used was not opened.
- Whether a Witt-vector or Tate class makes arm C's 3-part test cheaper than `[r]P`; not needed for B.
- The f2m18/f2m24 yield (×4) and any m = 3 effect are predictions only. The same projection makes every m = 3 probe `h×` more productive, but m = 3 is not a leaderboard recipe.
- The census, class tables and closure figures are scratchpad desk checks (`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/l19/{desk2.py,closure.py}`); the model for B's `S` was not run in the repository.

## 9. Sources

Fetched: `https://arxiv.org/pdf/1503.08001` (200, binary PDF, text read with `pdftotext`; the fetch tool's own summary failed to decompress it); `https://www.ece.uvic.ca/~fayez/research/gf3m/fong_j04.pdf` (200, PDF read with `pdftotext`); `https://eprint.iacr.org/2020/1315` (200, abstract page); `https://eprint.iacr.org/2015/1022` (200, abstract page). One web search (cofactor, index calculus, prime-order subgroup), general results only.

Repository paths read: `research/pdp_literature_leads_20261022/{BRIEF_SONNET.md,LEADS.md,leads/LEAD-04.md,leads/LEAD-04.verify.md}`; `docs/ic/{LEADERBOARD.md,boundary_targets.json}`, `docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json`, `docs/ic/tools/boundary_round_compare.py` (head), `docs/VALIDATION_GATE.md` (head); `research/ic_tool_program/rule/v3/README.md`; `research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md` (§3.3, §3.5, §10, §11.2); `research/isogeny_conductor_gap_ic_20261010/README.md`; `crypto-autoresearcher/knowledge/findings/KN-FIND-007.md`; `src/cryptanalysis/ic_boundary.rs` (L4225–4265, 4360–4415, 4617–4811, 6320–6519, 7700–7983, 8358–8386); `src/cryptanalysis/ic_framework/{plugins.rs,fiber.rs}`; `src/cryptanalysis/koblitz_index_calculus.rs` (L3000–3120, 9120–9250); `src/bin/ic/boundary.rs`.
