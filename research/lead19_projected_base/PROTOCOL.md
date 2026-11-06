# Protocol: LEAD-19, the cofactor-projected factor base on the random-binary ladder

Frozen 2026-10-23, before any run, from the discriminating test of
[`../pdp_literature_leads_20261022/leads/LEAD-19.md`](../pdp_literature_leads_20261022/leads/LEAD-19.md)
§6 (the card: [`../pdp_literature_leads_20261022/LEADS.md`](../pdp_literature_leads_20261022/LEADS.md),
LEAD-19).  The commit that adds this file also adds the code the commands
need; the task card under `cards/` binds this file's SHA-256 and that
commit, so the protocol a run cites is the one that was frozen.

**Result class, stated before measuring.**  A constant-factor change to
the relation phase of the random-binary rows of `docs/ic/LEADERBOARD.md`
Table B, measured in the ledger's own unit (`S` = group-addition
equivalents per `√r`, every phase, cold) against the ledger's own matched
rho.  Not an exponent, not a Gröbner lead, not a claim about any deployed
curve.  Every row stays above matched rho; the gap is reported as a
distance.  If the success criterion below holds, the result is an
**advance** in the sense of AGENTS.md "Success criterion" only once a
leaderboard row lands through the validation gate; this run alone is a
*stage-and-pipeline measurement* of the lever.

## 1. Question

The random-binary rows build their factor base from raw points of
`E(F_{2^n})` with cofactor `h ∈ {2, 6, 8}` and no projection into the
prime-order subgroup `⟨G⟩`.  The walked targets lie in `⟨G⟩`, so a folded
pair-table entry whose cofactor class is non-zero can never match: the
fraction of useful entries is the class-zero fraction (desk census: about
`1/2` at `h = 2`, about `1/6` at `h = 6`).  Does moving the base into
`⟨G⟩` lower `S` on the same instances, with every answer verified?

## 2. Arms

| arm | name in the record | base | what changes |
|:--|:--|:--|:--|
| A | `mitm_m2_negfold_walk`, `…_balanced` | raw subspace base, as landed | nothing: the ledger's rows, re-run in the same process |
| A′ | derived from A | the same rows minus the class test | A's `total_gae` minus the group operations of `summands_admissible` (the counters `admissibility_adds` and `admissibility_doubles` the run now records; the balanced rung never carried the test) |
| B | `mitm_m2_negfold_walk_projected`, `…_projected_balanced` | each raw abscissa's point `P` replaced by `[h]P` and its negative, one column per distinct projected abscissa | same `|F|`, every point and every table entry in `⟨G⟩`; `[h]P` charged to the base phase; no class test |
| C | `mitm_m2_negfold_walk_subgroup`, `…_subgroup_balanced` | the ambient field scanned in the subspace scan's order, points kept only when in `⟨G⟩`, stopped at the raw rung's abscissa count | membership by `Tr(x) = Tr(a)` at `h = 2` (a native parity, priced nowhere) or `[r]P = O` otherwise (charged) |

The oracle, pair table, walk, relation loop, linear algebra and
verification are the landed code, untouched
(`src/cryptanalysis/ic_boundary.rs`: `project_factor_base`,
`binary_subgroup_factor_base`, `prime_subgroup_factor_base`,
`folded_table_class_census`; `src/bin/ic/boundary.rs`: `--base-arms`).
With `--base-arms raw` (the default) the record is what it was.

## 3. Instances

- **Primary**: `icv1-f2m15-t131-ed7256f0` (`h = 2`), `icv1-f2m21-t2591-de54d72e`
  (`h = 2`), `icv1-f2m27-t12543-569dca8b` (`h = 6`): the Table B
  random-binary rows, as `ic boundary --regime char2` draws them from the
  frozen seed.
- **Secondary**, same command, no extra code: the `n = 18` and `n = 24`
  rungs (`h = 8`, bases already in `2E`; the dossier predicts a larger
  yield change, an unverified prediction from ledger §10.5).
- **Negative control**: the 24-bit prime-order instance of the prime
  ladder (`h = 1`), on which B and C are the identity by construction.

Seeds: `--seed 123212651130` fixes the curves; `--repeats 5` draws five
targets per instance, of which repeats 0 to 2 are the frozen round-5
targets (seeds 123212651130, 123212691633, 123212732136).

## 4. Commands (frozen; the task card carries them as argv arrays)

1. `cargo build --release --bin ic`
2. `./target/release/ic boundary --regime char2 --char2-degrees 15,18,21,24,27 --repeats 5 --seed 123212651130 --s4-max-degree 0 --base-arms raw,projected,subgroup --out research/lead19_projected_base/char2.json`
3. `./target/release/ic boundary --regime prime --prime-bits 24 --repeats 5 --seed 123212651130 --s4-max-degree 0 --base-arms raw,projected,subgroup --out research/lead19_projected_base/prime24.json`

`--s4-max-degree 0` skips the `S₄` oracle (937 of the 951 s at `n = 27` in
the frozen run); it is not a row of this comparison.  The metric is
counted group operations, so no isolation tier applies; wall times are a
practicality note.

## 5. Recorded, per instance, repeat and arm

`S`, per-phase GAE, group operations (`adds`, `doubles`,
`scalar_mults`), native counters, `trials`, `relations_found`, `rank`,
`verified`, `trials_over_floor_exact`, `yield_over_ceiling` (uniform and
exact), the class census of every base (classes, class sizes, folded
pairs, class-zero pairs and their fraction), the projection and
selection counters (`projection_*`, `subgroup_*`), the admissibility
counters of the raw std rung, and the matched rho `S` of the same
process (`rho_s_mean`, eight negation walks per target).

## 6. Checks that stop the analysis (bugs, not results)

1. Any `verified: false` row, or `status` other than `complete`.
2. On the prime-order control, the `raw`, `projected` and `subgroup`
   rows of a repeat not bit-identical in `trials`, `relations_found`,
   `total_gae` and every `group_ops` and `native` counter of every phase.
3. Arm A on the three primary rows, repeats 0 to 2, not equal to the
   frozen round-5 counters (`trials`, `relations_found`, `total_gae`) of
   `docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json`.  If the
   landed code has drifted, report the drift and keep the in-process A
   as the baseline.

## 7. Predictions (desk, from the dossier; unverified until the run)

- **P1.** The class-zero fraction of A's folded table at the std rung is
  within 10 % of the census: 0.50 (`f2m15`), 0.50 (`f2m21`), 0.17
  (`f2m27`).
- **P2.** Pooled relations per trial, B over A at the same rung, lies in
  `[0.7h, 1.3h]`: `[1.4, 2.6]` at `h = 2`, `[4.2, 7.8]` at `h = 6`.
- **P3.** `S(A′)/S(B)` at the matched rung: 1.4 to 1.5 (`f2m15`, `f2m21`),
  about 3 (`f2m27`); `S(A)/S(B)` against the published rows about 2.8,
  2.1, 3.4.
- **P4.** B and C relations per trial within 15 % of each other on the
  `h = 2` rows (every sum in `⟨G⟩` for both).
- **P5.** The prime-order control: ratios exactly 1.000.

## 8. Analysis plan (frozen)

Script: `docs/ic/tools/lead19_projected_base_compare.py <char2.json> <prime24.json> --frozen docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json --out research/lead19_projected_base/analysis.json`.
Python lays out recorded values only; it computes no group operation.

1. **Pairing.**  Within an instance, rows of the same repeat share the
   target, the seed and the curve.  A ratio is always between two rows of
   one repeat.
2. **A′.**  `S(A′) = (total_gae(A) − admissibility_adds − admissibility_doubles) / √r`
   for the std rung; the balanced rung carries no class test, so
   `A′ = A` there.
3. **Primary estimand.**  At the matched rung (same `|F|`), per repeat,
   `R = S(A′)/S(B)`; summarised by the geometric mean over the five
   repeats, with a paired bootstrap of the mean log-ratio (10,000
   resamples of the five repeats with replacement, seed 20261023), 95 %
   percentile interval.  The kill and success criteria read the
   **published rung** of each primary row (`f2m15`: std; `f2m21`,
   `f2m27`: balanced).  The other rung and the best-rung ratio (rungs
   chosen by the mean `S` over the five repeats) are reported beside it.
4. **Published-row ratio** (accounting, not the lever):
   `S(A, frozen round-5 mean)/S(B, this run)` per primary row.
5. **Pricing band** (AGENTS.md §5a).  Unpriced native work in this
   comparison is the trace test of arm C (`subgroup_trace_tests`); every
   other counter is either charged as group operations or priced by the
   pinned calibration.  Every arm's `S` is re-stated at `w ∈ {0, 0.1, 1}`
   GAE per unpriced native operation, rho included (rho carries none),
   and two arms are ranked only where the order holds across the band.
6. **Distance to rho.**  `S(arm)/S(rho matched)` per row at the three
   prices, as a distance.  No arm is expected to reach rho.
7. **Yield.**  Relations per trial per arm and rung, pooled over the five
   repeats, and `yield_over_ceiling_exact`.

## 9. Kill, success, and the pre-registered extension

- **Kill**: the geometric-mean `S(A′)/S(B)` at the published rung is below
  1.10 on all three primary rows; or the prime-order control moves; or P2
  fails (then the mechanism is misread: stop and report).
- **Success**: `S(A′)/S(B) ≥ 1.10` at the published rung on at least one
  primary row with the paired 95 % bootstrap lower bound above 1, every
  answer verified.
- **Extension**: if the interval straddles 1.10 on a row, re-run the same
  command with `--repeats 20`, nothing else changed, as a second card.
- Both ratios (A′ and published) are reported; only A′ is the lever.  A
  leaderboard row, a `benchmarks/` entry and the validation gate are a
  further step, not this run.

## 10. Cost

Code: landed with this file.  Run: under five CPU-minutes without `S₄`
(frozen non-`S₄` wall at `n = 27` about 15 s for three repeats), under
0.5 GiB.
