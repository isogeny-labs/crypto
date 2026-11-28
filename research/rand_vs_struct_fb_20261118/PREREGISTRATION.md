# Preregistration: random vs structured factor base — does structure lower the fall degree, or only the cost of writing the base down?

Date: 2026-11-18. Worktree `/home/user/crypto-wt/rand-vs-struct`, branch
`wt/rand-vs-struct`, commit `2d918fed3b` (+ the `f4_fp.rs` dedup fix in §7).
Everything here is `[pcc:unverified]` pilot data: no pcc claim, no registered
study result, no leaderboard or `benchmarks/` row. This file is frozen (committed)
**before** the formal data rows under `raw/` are produced. Follows the
`controlled-experiment` skill and AGENTS.md §5a. Rust only (owner rule); jq/shell
for glue; no Python.

## 1. Question and claim (classified)

**Question.** We have deep last-fall-degree (`d_F`) data for the structured
`2^k`-subgroup ("tower") factor base, but no clean random baseline. The one prior
random run (`research/cover_image_fb_20261112/`) was degree-floored: a random
factor base of size `|F|` expressed by the single membership polynomial
`∏_{f∈F}(x_i − f)` has degree `|F|`, so its F4 solving degree `D` is floored at
`≈ |F|+2` regardless of any summation structure. Does the **structure** of the
factor base lower `d_F`, or does the structured base only win because it is
cheaper to **write down**?

**Claim (class: comparative/statistical — needs controls, this skill).**
"The structured subgroup-chain factor base has `d_F` bounded well below `|F|`,
while a random factor base of the same size `|F|` has `d_F` floored by `|F|`
(grows with size)."

Secondary descriptive claims (same class): (i) randbase `d_F` equals the floor
`|F|+2` with no extra structural signal; (ii) there is a **crossover** size at
which the random base's membership degree `|F|` alone exceeds the structured
base's full `d_F` — i.e. where the random base costs more just to *express* than
the structured base costs to *solve*.

## 2. Arms, matched by size `|F| = 2^t` at each `t` (= `k`), m = 2, prime P1 = 2013265921

Same prime, same `S₃` summation, same curve per cell, Kummer tower. At each `t`
the three arms use a factor base of the **same size** `|F| = 2^t`:

- **(A) tower** — the structured `2^k`-subgroup base, membership presented as the
  tower chain (`m·t` low-degree chain relations, each degree ~2), two unknowns
  `x_1,x_2` for the `S₃` summation. `--controls tower`.
- **(B) randbase** — a random factor base of the same size `|F|`, two unknowns
  `x_1,x_2`, membership `∏_{f∈F}(x_i − f)` (a single univariate polynomial of
  degree `|F|`), `F` random abscissae on the curve. `--controls randbase` (m=2).
- **(C) null** — random coefficients on `S₃`'s monomial support, planted zero,
  **plus the same tower chain membership** (context arm: isolates the summation
  polynomial from the membership presentation). `--controls null`.

## 3. Instrument (one instrument; and why two engines appear)

`examples/pkm_tower_pilot.rs`, built by `harness/` (the root crate does not
compile; the harness compiles only the modules the pilot needs, from the
repository's own sources). Binary SHA-256 in `binary.sha256`.

- **Primary metric, same instrument on every arm: F4 solving degree `D`**
  (`--engine f4`, `solving_degree_max`). This is the one engine that can express
  all three arms. It is the `d_F`-equivalent here: the prior PKM fall-degree study
  (`research/pkm_falldegree_unblock_20261107/`) found `D = d_F` for the tower
  (median `D − d_F = 0`, 0/22 draws with `d_F < D`), and §6 of this prereg
  cross-checks `D = d_F` on the two arms that admit the true-`d_F` engine.
- **True `d_F` cross-check: HKY closures `last_fall`** (`--engine hky`,
  the degree `c` at which `1 ∈ V_{F,c}`, read on **random** targets whose system is
  generically inconsistent). This is the canonical index-calculus last fall degree.
  **The HKY engine cannot express randbase**: its ring is the tower quotient with
  `n − m·t` free variables, and randbase has `n = 2` non-tower variables, so
  `n − m·t < 0` underflows and the run panics (verified: exit 101 in
  `HkyRing::new`). That a random base has *no* tower-quotient presentation is
  itself part of the asymmetry under test, not a defect. So the true-`d_F` engine
  runs on **tower and null only**; randbase's `d_F` is read as F4 `D` (= `|F|+2`),
  the only instrument that can represent it.

Budgets identical across arms (`--budget` per §5). Seeds derived from `--seed`
(explicit list below). Arm order interleaved per `t`.

## 4. Metrics and the degree floor (reported explicitly)

For each arm × `t` report: `|F| = 2^t`; raw `d_F` (HKY `last_fall` for
tower/null; F4 `D` for randbase; F4 `D` also reported for all three as the uniform
same-instrument column); and **excess over the degree floor `= d_F − |F|`**.

The "floor" is the degree a **single-polynomial** membership of size `|F|` forces:
`|F|` (so the system's degree is `≥ |F|`, i.e. `D ≈ |F|+2` with the two-unknown
`S₃`). It applies to randbase (and naive). The tower's membership is the **chain**
(no single polynomial of degree `|F|`), so the floor does not apply to it; its
`d_F − |F|` is therefore expected to be large and negative. Reporting
`d_F − |F|` makes the confound explicit: if randbase's excess is a small constant
independent of `t`, its `d_F` carries **no** information beyond the floor.

**Crossover.** Report the size at which `|F|` (randbase membership degree) first
exceeds the tower's full `d_F`.

## 5. Decision rule, A/A, controls, sampling

- **Primary reading.** "Structure lowers `d_F`" is supported iff tower `d_F` stays
  bounded (`≤` a small constant, flat in `t`) while randbase `d_F` grows with `|F|`
  (tracks `|F|+2`), **and** the excess `d_F − |F|` is a size-independent constant
  for randbase. If instead randbase's excess were to grow with `t`, randbase would
  carry structural signal beyond the floor — reported as a finding.
- **A/A (noise floor).** For each randbase cell, **≥ 3 independent draws of `F`**
  (seeds 11, 22, 33; each draws a fresh random base). Report the spread of `D`.
  Prediction: spread = 0 (randbase `D` is deterministically `|F|+2`). A non-zero
  spread is itself a finding.
- **Positive control (detector power).** The tower cell must show a low `d_F`
  (`≈ 4`) at small `t`, or the instrument is broken. Expected: tower `d_F = 4`
  at `t = 2..5`, `5` at `t = 6`. (Dry-run/§6: confirmed with the frozen binary.)
- **Negative/sanity.** randbase at `t = 2` (`|F| = 4`): `D` must equal `|F|+2 = 6`
  (floor), and `6 > ` tower `d_F = 4`; the smallest cell, confirming floor
  behaviour and that randbase `D ≥` tower `d_F` at every size.
- **Range.** `t` (= `k`) from 2. randbase under F4 is feasible further than the
  task's `k ≤ 5` guess — the two-variable system keeps the matrices narrow — so
  the **local ceiling** is set by wall time on 4 shared vCPU: randbase
  single-draw wall times (frozen binary, §6) are t=8 0.4 s, t=9 2.0 s, t=10 10.5 s,
  t=11 66 s, so the 3-draw A/A runs go to **t = 10** and a single-draw randbase
  point is taken at **t = 11** (ceiling). tower/null `d_F` via HKY (reference
  engine) to **t = 8** (t=9 ≈ 40 s, t=10 ≈ 1000 s on this engine), and tower F4 `D`
  to t = 10; recorded tower values out to t = 11+ are cited from
  `research/pkm_falldegree_unblock_20261107/` and `cover_image_fb_20261112/` to
  show the tower stays flat beyond the fresh range.

## 6. Variable ledger and dry run (frozen-binary calibration taken before this freeze)

| factor | treatment |
|:--|:--|
| instrument (commit, binary SHA, engine, budget) | fixed per column; binary SHA in `binary.sha256` |
| metric | fixed: F4 `D` (uniform), HKY `last_fall` (true `d_F`, tower/null) |
| host | measured: Intel Xeon @2.10GHz, 4 vCPU, 16 GiB, kernel 6.18.44, rustc 1.97.0; load avg recorded per run (box shared with 3 workers) |
| order | arms interleaved within each `t` |
| seeds | derived from `--seed`; randbase A/A seeds {11,22,33}; tower/null seed 1 |
| clock | **not a primary metric** — `d_F`/`D` are deterministic counts; wall times reported as context only, so host contention does not affect the reading |
| spot preemption | n/a (local) |
| analyst | metric is a deterministic integer per cell; no masking needed (no subjective call), labels kept |

Dry run (the skill's required scratch dry run; frozen binary SHA
`37d5ce4f…`, files `raw/validate_*`, `raw/probe_*`, `raw/calib_*`):
- tower `d_F` (HKY, random) t=2..6 → **4,4,4,4,5** (positive control passes).
- null `d_F` (HKY, random) t=2..6 → **4,4,4,4,5** (keeps the tower chain).
- randbase `D` (F4) t=8,9,10,11 → **258, 514, 1026, 2050** = `|F|+2` exactly.
- randbase A/A at t=4, seeds {11,22,33} → `D = 18` identical (spread 0).

No hypothesis, metric, null, range or decision rule changes after seeing data; any
change would be a dated amendment stating whether it preceded the data it affects.

## 7. Build note (instrument repair, disclosed)

At commit `2d918fed3b` the file `src/cryptanalysis/f4_fp.rs` does not compile:
`VisitReport` declares `first_fall_degree`/`last_fall_degree`/
`degree_of_regularity_observed` twice, and `F4Options::recording_degree_falls`
is defined twice (E0124/E0062/E0592). Git history shows a "restore … to unbreak
main" commit followed by a `Merge branch 'main'` that re-introduced the same
fields and method — a merge duplication. The fix is a pure dedup (remove the
second declaration of each; keep the flag-setting `recording_degree_falls`): it
changes no behaviour and makes the instrument build. Committed on
`wt/rand-vs-struct` with this preregistration.

## 8. Deliverables

`PREREGISTRATION.md` (this file), `raw/` (JSONL rows + stderr + per-run load),
`README.md` (matched-size table, crossover, A/A spread, honest interpretation).
All local, committed on `wt/rand-vs-struct`; no push, no pods.
