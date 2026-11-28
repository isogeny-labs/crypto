# Stacked D4 + double large primes on E(F_{p^4}), k = 4: what is committed

Read-only synthesis, 2026-11-04. Nothing was run. Every number is either
"from field X" (a field of a committed record) or "derived by A/B" (arithmetic
on such fields). Records: `R4 = research/ic_improve_20261015/04-d4-2lp-fix/raw/{test.jsonl,summary.json}`
(27 held-out curves, p = 101..1601, seeds 1-3); `R1 = research/ic_improve_20261014/01-d4-2lp/`
(24 curves, p = 101..1301); `README4`, `README1` are the READMEs beside them.

## Short answer

1. D4 + 2LP **was measured**, twice (item 01 curve ladder and in the ecbench
   harness). It regresses on the whole cost. The leads review's statement
   that "the stacked D4 + 2LP arm was not run"
   (`research/ic_leads_review_20261018/README.md:76`, `:243`) is contradicted
   by item 01's title and arms; the review never cites items 01 or 04
   (grep for `01-d4-2lp`, `04-d4-2lp`, `d4-ed` finds nothing in it).
2. `d4-ed` / `d4-ed-all` are **not** the stacked arm: they have no large
   primes. What is genuinely unmeasured is D4 + 2LP + eager first-dependency
   stop (a three-way stack). Its trial count is bounded below by `d4-ed`'s
   (section 2), so it cannot beat the pin by more than ~1e-8 (derived).
3. Linear algebra is ~5e-8 of the whole cost at the largest p; the Amdahl
   bound is 1 + 6e-8. LA does not matter in the measured range.

## 1. Which combinations were measured

Ratios are to `d4`, paired per curve, pooled geometric mean [95% CI]. Columns
"whole / relation / LA" are from fields `paired_vs_d4.<arm>.{whole,relation,la}.pooled.ratio`
and `.ci95` of `R4/summary.json` (n = 27), unless marked README1.

| arm | D4 base | large primes | stop rule | whole | relation | LA | source |
|---|---|---|---|---|---|---|---|
| `d4` (pin) | yes | none | first LA at cols+1, retry +5% | 1 | 1 | 1 | `R4/test.jsonl` arm `d4`; `README4:9` |
| `d4-ed` | yes | none | square-core test after every relation | 0.9792 [0.9725, 0.9855] | 0.9792 [0.9727, 0.9850] | 0.9693 [0.9552, 0.9827] | `R4/summary.json` `paired_vs_d4["d4-ed"]`; `README4:29-31` |
| `d4-ed-all` | yes | none | as `d4-ed`, every distinct row of a trial fed | 0.9746 [0.9659, 0.9822] | 0.9746 [0.9660, 0.9826] | 0.9657 [0.9506, 0.9802] | same, `["d4-ed-all"]` |
| `d4-2lp` (alpha 0.95, cap inf, uniform, omega 1) | yes | double, ~25% of columns large | cols+1 schedule (`"eager": false` in arm `cfg`) | 1.0264 [1.0179, 1.0362] REGRESSES | 1.0264 [1.0177, 1.0361] | 0.8201 [0.7979, 0.8456] | same, `["d4-2lp"]` |
| tuned D4+2LP, item 01 (24 curves) | yes | double | cols+1 | 1.025 [1.016, 1.037] | 1.025 [1.016, 1.035] | 0.827 [0.802, 0.853] | `README1:22-24` (README value, not recomputed) |
| D4+2LP, Gaudry alpha 0.75 (24 curves) | yes | double, \|F\| = \|B\|^(3/4) | cols+1 | 2.355 [2.043, 2.702] | 2.355 [2.035, 2.701] | 0.265 [0.243, 0.293] | `README1:25-27` (README value) |
| `d4-late110` (positive control) | yes | none | first LA at ceil(1.1(cols+1)) | 1.1119 [1.0970, 1.1338] | 1.1119 [1.0963, 1.1324] | 1.0812 [1.0600, 1.1045] | `R4/summary.json` `["d4-late110"]` |
| `d4-2lp-null`, `d4-adapt` | - | - | identical to `d4` | 1.0 exactly, 27/27 | 1.0 | 1.0 | `["d4-2lp-null"]`, `["d4-adapt"]` |
| **D4 + 2LP + eager stop** | yes | double | every relation | **not measured** | - | - | no such arm: `ed_cfg()` (`examples/ic_improve2_04_d4_2lp_fix.rs:650-654`) sets `eager = true` on `Cfg::incumbent()` only |

In the ecbench harness (operation counts only, "diagnostic only (unpriced
work)", L0, 12 pairs, p = 101..397):

- `ic-k4-d4-2lp / ic-k4-d4 = 1.0108 [1.001, 1.026]`
  (`R1/ecbench/compare.txt:2`; spec `R1/ecbench/spec-d4-2lp.json`, base
  `yline-d4-lp`, `large_primes: "2"`).
- Gaudry arm 1.5839 [1.423, 1.807] (`R1/ecbench/compare.txt:27`).
- `ic-k4-d4-ed / ic-k4-d4 = 0.9904 [0.977, 0.998]` (`README4:262`; spec
  `R4/ecbench/spec-d4-ed.json`, base `yline-d4-ed`).

Controls: `summary.json` fields `every_logarithm_verified` = true and
`null_identical` = true.

Why 2LP loses (`README1:105-114`, `README4:99-110`): relations used are still
about columns+1 (fields `full_relations`, `lp_hist`), and trials with >= 3
large primes are paid for and rejected. Mean of `arms[d4-2lp].trials /
arms[d4-ed].trials` over the 27 curves = 1.0486 (derived by that quotient,
`R4/test.jsonl`).

## 2. Are `d4-ed` / `d4-ed-all` the stacked arm?

No as arms, but they bound it. The argument:

- **Definition.** `ed_cfg()` is `Cfg::incumbent().named("d4-ed")` with
  `c.eager = true` (`examples/ic_improve2_04_d4_2lp_fix.rs:650-654`).
  `Cfg::incumbent()` has no large primes; the eliminator is built only when
  `max_lp > 0` (`:401`). `README4:39` says "No large primes, so no rejected
  trials." Field `lp_hist` of `d4-ed` at p = 1601 seed 2 is `[406,0,0,0,0]`
  (all full relations). So `d4-ed` is "D4 + eager stop", not "D4 + 2LP".
- **Eager is a separate switch.** `eager` changes only the first attempt (1)
  and the retry step (`:402-406`, `:447-453`). A config with `max_lp > 0` and
  `eager = true` is expressible, but every 2LP arm in the record has
  `"eager": false`, and `d4-ed*` have `lp_hist` all-full. The three-way
  stack was never built into an arm.
- **It cannot beat `d4-ed` on trials.** `README4:97-107` (derivation 1):
  every relation that large-prime elimination produces is a combination of
  the collected rows, so any LP scheme on a shared stream needs at least the
  first dependency (FD) of those rows, and it also loses rejected rows.
  `d4-ed` reaches FD (`summary.json` `accounting.d4_ed_relations_minus_first_dependency`:
  mean 0.037, max 1). Measured check: `dominance_violations_ed_vs_2lp` = `[]`;
  recomputed 0 of 27 curves with `d4-ed.trials > d4-2lp.trials`. Caveat:
  derivation plus a dominance check against the NON-eager 2LP arm; the
  eager-2LP arm has no record, so the conclusion for it is "derived,
  unverified".
- **The stack could gain only in LA.** 2LP's LA ratio is 0.82 (table), LA is
  at most 6.0e-8 of whole (section 3), so an 18% LA cut is worth at most
  1.1e-8 of whole (derived by 0.18 x 6.0e-8), against a rejected-trial
  penalty of about +4.9% (derived above, trials ratio 1.0486).

## 3. Does linear algebra matter?

LA share = `arms[].gae.la / arms[].gae.whole` (derived; fields in
`R4/test.jsonl`; `gae.whole` is in GAE before the 1/sqrt(r) division, which
cancels in the ratio). Largest p = 1601, arm `d4`:

| curve (seed) | field `gae.la` | field `gae.whole` | LA/whole (derived) | whole/(whole-LA) (derived) |
|---|---|---|---|---|
| 1 | 441735.26 | 8.71168e12 | 5.07e-8 | 1.0000000507 |
| 2 | 461261.20 | 9.68290e12 | 4.76e-8 | 1.0000000476 |
| 3 | 501773.03 | 8.36603e12 | 6.00e-8 | 1.0000000600 |

Maximum of the `d4` share over all 27 test curves is 5.9977e-8 (derived by
jq max of `gae.la/gae.whole`), so the Amdahl bound is 1.00000006 (derived
by 1/(1 - 5.9977e-8)). For `d4-ed` at p = 1601 the share is 4.7e-8..6.0e-8
(derived); for `d4-2lp`, 3.5e-8..4.5e-8 (derived). This agrees with
`README4:230` ("at most 6e-8") and `README1:30-31` ("under 1e-7"). Even a free
LA gains at most 6e-8 relative.

Consistently, whole ratios equal relation ratios to 8 digits (`d4-ed` pooled
whole 0.97917290 vs relation 0.97917290), so the LA ratios (0.97, 0.82) do
not move the whole ratio.

Extrapolation, NOT a measurement: fitted slopes of log2 cost on log2 #E for
`d4` are relation 0.2186 and LA 0.5107 (fields `fits.d4.relation.slope`,
`fits.d4.la.slope`). The share reaches ~1 where 42.6 + log2(1/6.0e-8) /
(0.5107 - 0.2186) = 42.6 + 24.0/0.292, about 2^125 (derived). The records
price a trial at C4 = 2.54e10, which the 2026-10-19 annotation
(`README4:54`, `README1:39`) says #1792 measured 311-370x too high; with
relations ~300x cheaper the p = 1601 share would be ~1.8e-5 and the crossing
about 2^97 (rough estimate on the same slopes). Either way LA is irrelevant
at every measured size; a toy-range record cannot settle larger sizes.

## 4. Smallest experiment for the remaining gap (sketch, NOT run)

Gap: D4 + 2LP + eager stop vs `d4-ed` (the current pin). Prediction (derived,
sections 2-3): whole/`d4-ed` >= about 1.0, likely +3..5%; any LA-driven
advance <= ~1e-8.

ecbench spec (`ecbench.spec/v1`, family of `R4/ecbench/spec-d4-ed.json`):

- Needs an additive base value (say `yline-d4-lp-ed`) for `ic.k4_gaudry`
  honouring `K4Params.eager_la` on the `yline-d4-lp` path (code change, not
  made here).
- Arms: `ic-k4-d4` (baseline); `ic-k4-d4-ed` (pin: `base: yline-d4-ed`,
  `large_primes: "0"`); `ic-k4-d4-2lp-ed` (candidate: `base: yline-d4-lp-ed`,
  `large_primes: "2"`, `lp_exponent: "0.95"`, `c4: "2.54e10"`); a null A/A
  `ic-k4-d4-2lp-ed-null` (large-prime set empty, i.e. all columns small; must
  equal `ic-k4-d4-ed`).
- Curves: `fp4_translation` p = 521, 1033, 1601 (sizes where 2LP regressed,
  `README4` per-size table), seeds 1, 2, 3 (the item-04 test split; so the
  pin and 2LP are re-measured on already-seen curves, state it), 
  `targets_per_curve: 3`.
- `measurement`: rounds 1, alternate order, seed 1, isolation L0 (counts
  only; any wall-time claim needs the isolab-eks route and its admission).
- Gates: `ic-k4-d4-ed` trials must equal item 04's on every curve; every
  logarithm verified; audit with replays.
- Decision: pooled paired ratio `2lp-ed / d4-ed` under item 04's rule;
  adopt only if the CI is below 1. Report trials and LA share so section 3 is
  re-derived on the new arm.
- Cheaper variant: set `eager = true` on a `Cfg::lp(..)` arm in the item-04
  example and replay its 27 test curves on the shared trial stream (oracle
  build 6-7 min per p = 1601 curve, `README4:289-290`).

Open: the leads-review wording (sections 2.2, 3.4, row 5 of the action list)
should be corrected by its owner; sparse-LA wiring (review section 3.3) is
orthogonal to this gap. CLAUDE.md's session-log instruction was not followed
because this task was read-only outside this directory.
