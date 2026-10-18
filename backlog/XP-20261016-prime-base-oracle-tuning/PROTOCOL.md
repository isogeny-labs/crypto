# Prime-field factor-base size and exact-oracle tuning: protocol

**Status: DRAFT — not frozen.** No train, holdout or control session has run.
The protocol freezes when the blockers in §12 are cleared, this file is
committed and its git blob is recorded in `card.json` (`protocol.blob`). Until
then any section may change. Once frozen, a change is a dated amendment, never
an edit. Card: [`README.md`](README.md). Orchestration:
[`run.sh`](run.sh). Specs: [`specs/`](specs/).

Every quantitative statement carries one label: *recorded* (a committed
row or session), *measured* (a pre-freeze sizing probe on this card, §11),
*derived* (a calculation shown here) or *predicted* (stated before the runs).

## 1. Question and class

Can joint selection of factor-base size and exact native decomposition
oracle reduce the complete, cold, one-target index-calculus (IC) cost below
the current verified prime-field incumbent, on fresh matched workloads?

**Class.** Engineering at most (AGENTS.md §3): the unit, the floor and the
pipeline stay the same. Only the base size `N` and the oracle change. A
verified holdout reduction at every pricing weight is a success and becomes
the new baseline at that size, even though IC stays above rho. A null,
failed or weight-dependent result keeps the incumbent.

**Out of scope.** Large-memory PKM degree questions, NIST-size coefficients,
multi-target or amortised cost, wall-time speed claims, and Koblitz curves.
No key-recovery or deployed-curve claim. Public synthetic curves and
hash-to-subgroup public targets only.

## 2. The frozen baseline (incumbent)

The incumbent is the prime-curve `ic-pair` arm of the IC frontier campaign
(`research/ic_frontier_campaign_20261015/`). That protocol froze it on
2026-10-15. Its Phase 1 sessions are the most recent verified native
prime-field IC pipeline on a size ladder priced across the §5a band.

| item | value |
|:--|:--|
| method | `ic.pipeline` |
| factor base | `prime-abscissa:size=N` (low-x: the `N` smallest abscissae that carry a point; one column each) |
| oracle | `mitm:negation_folded=1` (two-summand negation-folded pair table, `m = 2`) |
| fixed | `targets=walk`, `linalg=incremental-gauss` (stop at the target pin), `solver=""`, `solver_budget_seconds=0` |
| `N` and `max_trials` by size | 20 bits: 32, 1,410,236; 24: 162, 4,243,777; 28: 407, 26,479,085; 32: 1,024, 167,871,873 (`params.json`, `phase1/spec-P<b>.json`) |
| method ids | `ECM1h586f93741037`, `ECM1h7291b44e96ad`, `ECM1h1729b9b88605`, `ECM1h61a6431fe629` for 20, 24, 28 and 32 bits |
| evidence | sessions `phase1/sessions/P20`–`P32`; 32/32 measured runs verified per size; replay-all audits `phase1/audits/P*.json` ok (*recorded*) |
| source / binary | commit `9bef57337f5fcaa53e0326b21749a25c145078b8`; ecbench SHA-256 `79b0b594…6a57f69b` (*recorded*, `session.json`) |
| band `S` (w = 0 / 0.1 / 1) | 20 bits 12.50 / 14.47 / 32.19; 24: 14.35 / 15.86 / 29.43; 28: 20.56 / 22.63 / 41.28; 32: 32.07 / 35.29 / 64.22 (*recorded*, `analysis/band-prime.md`) |
| matched rho on the same curves | `rho.negation` 1.27, 1.19, 1.11, 0.79 at w = 0 (*recorded*) |

The method ids are a function of the parameters only, not of the curve.
`ecbench plan` on this card's train specs gives the same four ids for the
incumbent cells (*measured*, §11). So this card re-runs the incumbent
*unchanged* on every new workload. It never quotes the frontier numbers as
the comparison. The baseline in every decision is the incumbent arm measured
in the same session on the same targets and algorithm seeds.

Not used as the baseline: the leaderboard's Table B prime rows
(`docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json`). They come from the
legacy `ic boundary` harness, priced with pinned wall-time calibrations.
AGENTS.md §5a forbids setting a calibrated cost against this card's
band-priced costs. Nor are the single-curve 20-bit `mitm-prime-batch` B48
studies (`research/ecbench_rootfinding_*_20261013–14`) used. They used other
targets and `max_trials`, and their B40/B48/B56 selection did not hold on
holdout (`ecbench_rootfinding_lanes_20261013/REPORT.md` §3).

## 3. Workloads

All curves are prime-order (cofactor 1) curves over `F_p`. Each workload is
one public target `Q` with no planted scalar (`target_kind: public`). Every
run therefore solves one previously unseen point, as the IC measurement rules
require. No scalar label exists that could inform selection.

| panel | curve constructor | targets | target seed | algorithm seed (`measurement.seed`) | rounds (warm-up) |
|:--|:--|--:|--:|--:|:--|
| T: train | `prime_search`, bits ∈ {20, 24, 28, 32}, seed `20261018101` | 8 | `20261018111` | `20261018121` | 1 (0) |
| H: holdout | `prime_search`, same bits, seed `20261018202` | 48 | `20261018211` | `20261018221` | 2 (1) |
| K: coset control | the two prime-order rungs of the registered PKM ladder (`research/pkm_tower_ecbench_20261013/curves.jsonl` rows 2–3: `icv1-fp22-tm2947-e15fed69`, `icv1-fp24-t2705-27cbb01b`) | 16 | `20261018311` | `20261018321` | 2 (0) |
| identity | the frontier's own sessions P20–P32, replayed (§5, stage `identity`) | — | — | — | — |

- **Fresh.** `ecbench plan` reports all eight T and H curves as
  `registered=false` (*measured*). No study in this repository has used them,
  so neither the incumbent's selection nor any earlier tuning has seen them.
  Their `log₂ r` is 19.54, 23.14, 27.23, 31.58 (T) and 19.62, 23.51, 27.57,
  31.84 (H). The incumbent's `N` is keyed by the `bits` label, as the frontier
  froze it, not re-derived from `r`. Train and holdout curves are disjoint.
- **Registration.** By AGENTS.md §11 the eight slugs are registered
  (`docs/curves/sources/specs.txt`, `scripts/build_curve_registry.py`) in
  the pull request that lands the result, before any prose names them. This
  draft names them only by constructor and seed.
- **Regression only.** The frontier ladder curves and the PKM-ladder curves
  are not fresh. Panel K is a control and is never decision-bearing.
- **Why `prime_search`.** It covers 8–32 bits, with order by Hasse-interval
  stepping (`ic_boundary::PrimeCurve::prime_order`). The frontier's 36- and
  40-bit `find_a3_curve` rungs have no fresh, seeded counterpart without new
  code. So 32 bits is this card's ceiling (README "Extension").

## 4. Candidate grid

### 4.1 Fixed in every IC arm

`targets=walk`, `linalg=incremental-gauss`, the incumbent's `max_trials` for
the size, and `timeout_seconds` 1,800. These are the incumbent's values. A
cell can differ from the incumbent only in its factor base and oracle.

### 4.2 Base sizes

`N_k = round(N_inc · 2^{k/2})` for `k ∈ {−2, −1, 0, 1, 2}`, with `round` meaning
half up. This gives:

| bits | `N` grid (incumbent bold) |
|--:|:--|
| 20 | 16, 23, **32**, 45, 64 |
| 24 | 81, 115, **162**, 229, 324 |
| 28 | 204, 288, **407**, 576, 814 |
| 32 | 512, 724, **1024**, 1448, 2048 |

**Edge rule** (as frontier A4). For each oracle family and size, suppose the
minimum train mean `S` at `w = 0.1` among eligible cells (§4.4) sits at the
smallest or largest `N` run so far. Then the next `k` outward (±3, then ±4)
runs as an extension session (`train-ext-P<b>-<family>-n<N>`): the same
curve, targets and seeds, with `rho-neg`. Extension stops when the minimum is
interior, at `|k| = 4`, or at `N < 2`. `run.sh` applies the rule mechanically.

### 4.3 Oracles: every exact native prime-curve oracle in `solve_ic_prime`

| family | oracle string | summands | sizes |
|:--|:--|--:|:--|
| `mitm2` (incumbent) | `mitm:negation_folded=1` | 2 | all |
| `batch2` | `mitm-prime-batch:m=2` (batched-inversion table build, landed 2026-10-13) | 2 | all |
| `mitm3` | `mitm:m=3,negation_folded=1` | 3 | all |
| `rfmitm3` | `rootfind-mitm:m=3` | 3 | all |
| `rfsub3` | `rootfind-subtract:m=3` | 3 | all |
| `rfsub2` | `rootfind-subtract:m=2` | 2 | 20, 24 at `N_inc` only |
| `sub2` | `subtract` | 2 | 20, 24 at `N_inc` only |
| `rfpairs3` | `rootfind-pairs:m=3` | 3 | 20, 24 at `N_inc` only |

- **Retired above 24 bits.** The memoryless arms `rfsub2`, `sub2` and
  `rfpairs3` cost `Θ(r)` additions at any `N` (*derived*: `2N` additions a
  trial, a hit rate of about `2N²/r`, and `N` relations needed). Probe A
  measured them at `S` = 2,302, 2,302 and 5,879 at 24 bits, against 14.1 for
  the incumbent (*measured*, §11). They run at 20 and 24 bits as exact-oracle
  controls only.
- **Not in the grid.** `mitm-fiber` is the cofactor-fiber oracle; at
  cofactor 1 it adds nothing. `glv-orbit` and `cm-scalar-orbit` need curve
  endomorphisms these curves lack. `prime-isogeny` builds an auxiliary-curve
  tower (`research/pkm_isogeny_tower_ecbench_20261015`: no benefit over a
  plain base). At cofactor 1, `prime-projected-abscissa` selects the low-x
  point set again (*derived* from `plugins.rs`). `prime-kummer` needs
  `2^t | p − 1` and runs only in panel K.
- **Random base.** No uniformly random signed-point base plug-in exists for
  prime curves (§12, blocker 2). The holdout carries a strided-abscissa
  control (`prime-abscissa:size=N*,stride=7919`). It controls for small-x
  structure only and is not a random base.

Train cells per size (*derived*): 25 main cells, plus 3 controls at 20 and
24 bits. Each runs 8 targets.

### 4.4 Selection (train) and holdout arms

**Eligible cell:** every train run verified (8/8, no `exhausted`, `timeout`
or `error`), and no unpriced counter without a count (`ecbench band`
`uncounted` empty).

**Selected recipe `R*(b)`:** the eligible cell with the smallest band mean
`S` at `w = 0.1` on the train curve. Exact ties go to the incumbent oracle,
then to the smaller `N`. The **size-only** cell is the eligible `mitm2` cell
with the smallest mean `S` at `w = 0.1`.

`run.sh select` writes `select/selection-P<b>.json` and fills
`specs/holdout-P<b>.template.json` into `specs/holdout-P<b>.json`. It then
writes `select/FROZEN.sha256`. `run.sh holdout` refuses to start unless
those hashes still match.

| holdout arm | role | definition |
|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` |
| `inc` | baseline | the incumbent, §2, unchanged |
| `sel` | candidate | `R*(b)` |
| `size` | candidate | the size-only cell (attribution of any gain to `N` alone) |
| `stride` | control | `prime-abscissa:size=N*,stride=7919` with `R*(b)`'s oracle |
| `inc-aa` | control | `inc` again (operations A/A) |
| `rho-neg-aa` | control | `rho-neg` again (operations A/A) |

If `R*(b)` is the incumbent, then `sel` duplicates `inc`, the size is null by
construction, and the arm runs anyway as a further determinism check.

### 4.5 Panel K: coset (Kummer) control

On the 22- and 24-bit PKM-ladder prime-order curves (2-adicity 12), the
multiplicative-coset base `prime-kummer:t=T` is set against low-x at the same
actual column count. Both use the incumbent oracle. The column counts come
from `ecbench fb` before freezing (*measured*): 22 bits `t = 7`: 68 and
`t = 8`: 129; 24 bits `t = 8`: 119 and `t = 9`: 245. The arms are
`kummer-t<T>` and `lowx-c<cols>`. They answer the historical study's
coset-vs-low-x question in native code and the common unit. They never
select a recipe.

## 5. Stages (`run.sh STAGE OUT`, run from the repository root)

1. `host`: refuse a dirty tree. Record the commit, the ecbench SHA-256,
   `rustc`, `ecbench host --json`, `lscpu`, memory and `ecbench methods`.
2. `identity`: `ecbench verify --replay-all --exit-code` on the frontier
   sessions P20, P24, P28 and P32 with the pinned binary. All four must
   reproduce every count. A failure stops the run (instrument first, §7.5).
3. `train`: the four train sessions, then the edge extensions (§4.2).
4. `select`: §4.4. Hash-freeze the holdout specs.
5. `holdout`: the four holdout sessions.
6. `kummer`: the two panel-K sessions.
7. `audit`: `ecbench verify --replay-all --exit-code` on every session.
   Receipts go to `audits/`.
8. `analyse`: §7. `ecbench band --ranks`, `ecbench table`, and
   `ecbench compare` at each weight (§12, blocker 1).

Every session runs as
`taskset -c $PBO_CPU ecbench run --spec S --out D --cpus none --allow-busy --quiet`
with `RAYON_NUM_THREADS=1`: one core, sequential. `--cpus <list>` is not
used, because it exits silently on this host class. Rerunning a stage, or
passing `--resume`, skips complete sessions. An incomplete session directory
is renamed `<dir>.interrupted-<time>`, kept and rerun whole with the same
spec.

## 6. Cost model and unit

- **Unit.** `S = total GAE / √r`, cold and whole-pipeline (factor base,
  oracle set-up, relations, linear algebra, verification), one target, as
  `ecbench` charges `ic.pipeline` (docs/ecbench/README.md §7). Rho is charged
  in the same unit on the same point.
- **Pricing band (AGENTS.md §5a).** Every arm, rho included, is priced at
  `w = 0, 0.1, 1` GAE per unpriced native operation:
  `priced_S(w) = (G − C + w·(U + n_C))/√r` (`ecbench band`). None of the
  eight fresh curves, nor the panel-K curves, has a pinned calibration. So
  `C = 0` on every arm, and no calibrated cost meets an uncalibrated one.
- **Unpriced counters, all emitted with counts** (*measured*, probe A).
  IC arms: `sqrt_solves`, `lookups`, `row_ops`, `legendre_symbols`,
  `target_guard_probes`, and `inversions` for `rootfind-subtract`. Rho:
  `canonicalisations`. An arm that names a counter without a count is a
  lower bound and cannot be selected or ranked.
- **Secondary, never decision-bearing.** These are: the IC online window
  (`online.wall_ns`) and phase GAE split; peak RSS (`time.max_rss_kib`);
  relation trials, relations and rank at the pin; process wall. Runs are L0
  (shared spot host). Wall time is descriptive only, and no wall ratio is
  reported as a speedup.
- **Memory.** If `sel`'s mean peak RSS is more than 2× `inc`'s, the row is
  labelled a time–memory trade beside its `S` ratio (frontier A5).

## 7. Statistics and decision rule

### 7.1 Sample sizes

Train: 8 targets × 1 round per cell. That is enough to select, not to claim.
A cell mean has relative SE ≈ 0.10/√8 ≈ 3.5% at 24–32 bits (*derived* from
the incumbent's per-run CV in frontier Phase 1, which is 0.104, 0.112 and
0.099 at 24, 28 and 32 bits, and 0.305 at 20 bits; *recorded*).

Holdout: 48 targets × 2 rounds gives 96 matched pairs per arm and size. The
95% half-width of `log(Σ S_sel / Σ S_inc)` is about
`1.96·√2·0.10/√96 ≈ 2.8%` at 24–32 bits and about 8.8% at 20 bits
(*derived*, treating the arms as independent). Effects smaller than about 3%
(24–32 bits) or 9% (20 bits) are not expected to resolve.

### 7.2 Uncertainty

`ecbench compare` gives the paired ratio of totals `Σ S_B / Σ S_A` over
(workload, round) pairs, both arms sharing the algorithm seed. Its 95%
interval is a two-stage cluster bootstrap: targets first, then pairs within a
target. It uses 4,000 resamples and seed 20261002, the defaults. The ratio
and interval are needed at each `w ∈ {0, 0.1, 1}`. `ecbench compare`
computes them today only for the charged `S`, which here equals `w = 0`.
Blocker 1 (§12) adds `--weight`. It is to apply `band::priced_s` to every
record before the same bootstrap.

### 7.3 Per-size decision (holdout only)

At size `b`, `R*(b)` is an **improvement** (engineering), and becomes the
prime-field IC baseline at that size, only if all of the following hold:

1. **Instrument:** `identity` passes. Every holdout audit is `ok` with every
   measured replay identical. `inc-aa/inc` and `rho-neg-aa/rho-neg` are
   exactly 1.000 at every weight. `FROZEN.sha256` still matches.
2. **Completeness:** all 96 measured `sel` runs and all 96 `inc` runs
   verified `[d]G = Q`. Any `exhausted`, `timeout`, `error` or OOM leaves
   the size **incomplete**, which is never a win.
3. **Ratio:** the 95% upper bound of `Σ S_sel(w) / Σ S_inc(w)` is below 1 at
   all three weights.
4. **Band:** `ecbench band --ranks` gives `sel < inc` at every weight.

| outcome at size `b` | reading |
|:--|:--|
| 1–4 hold | improvement; `R*(b)` is the new baseline at `b`, with its ratio and interval as the delta |
| 3 holds at some weights, fails at others | unresolved inside the band; incumbent retained |
| interval includes 1 at any weight | null at this sample size; incumbent retained |
| upper bound of `Σ S_inc/Σ S_sel` below 1 at all weights | regression on holdout; incumbent retained |
| `R*(b)` = incumbent | train found no better cell; null by construction |
| a total falls at `w = 0` but rises at `w = 1` | relabelling (work moved into native operations); not an improvement |
| 2 fails | incomplete; reported with its failures |

Each size decides alone. A pooled ratio across sizes is never a claim.
`size/inc` and `sel/size` are reported for attribution. `stride/sel` is read
as a small-abscissa control only.

### 7.4 Always reported (every size and weight)

- `inc/rho-neg` and `sel/rho-neg` with intervals: the gap to matched rho, the
  distance remaining. Not beating rho never fails an improvement.
- The ratio to the floor `√(π/4)` (A = 2).
- The full train band table, every cell included.
- Panel K: `kummer-t<T>/lowx-c<cols>` per curve, with intervals at each
  weight. Descriptive control only.
- **Secondary, not a claim:** `ecbench bound fit` over the four sizes for
  `inc` and `sel` at each weight. An exponent change is reported, never
  claimed: the sizes are four single curves.

### 7.5 Stop rules

- A failed identity, a failed audit, or an operations A/A that is not
  exactly 1.000 stops the reading until explained. It is an instrument fault,
  not a result.
- A host loss reruns the affected session whole (§5). It is not a failure of
  the method.
- No other stop. Every planned session runs, including after a null train
  result.

## 8. Failures and retention

Exhausted, timed-out, errored and OOM runs stay in their sessions and in
every table. They make a cell ineligible (train) or a size incomplete
(holdout). Interrupted session directories are kept. The raw session trees
land as one deterministic `runs.tar.xz` (AGENTS.md §13a). The exception is
any record a receipt names by path.

## 9. Inadmissible after freezing

- Changing curves, seeds, target counts, rounds, the `N` grid or its
  rounding, the edge rule, the oracle list, `targets`, `linalg`,
  `max_trials`, the timeout, the unit, the band weights, or the selection
  rule.
- Editing a train or holdout spec after `select` has hashed it.
- Selecting on holdout data, or rerunning a session to get a different
  answer.
- Dropping a size, target, arm, failure or interrupted directory.
- Using frontier, leaderboard Table B or other-session numbers as the
  comparison for a decision.
- Pooling sizes into one claim.
- Reporting a wall ratio, a phase or stage ratio, or a relation-count drop
  as a speedup.

## 10. Predictions (before any run)

1. Identity passes; all IC arms verify within their caps.
2. `batch2` equals `mitm2` exactly at every weight. It builds the same table
   with fewer field inversions, and inversions are not counted for it (probe
   A: identical `S` at all three weights). The tie rule therefore keeps
   `mitm2`.
3. The three-summand oracles lose at every size. Probes measured them at
   2.9× (24 bits) and 2.7× (32 bits, best `N`) the incumbent's `S` at
   `w = 0`, and by more at `w = 1`. That is consistent with the derived
   table-plus-scan cost `≈ 3.1 r^{2/3}`, against `≈ 1.5 r^{2/3}` for the
   `m = 2` pair table.
4. `R*(b)` is `mitm2` at a neighbouring `N` (`√2` steps), or the incumbent
   itself. Any holdout gain is under about 10% and may not resolve. The most
   likely reading is null or unresolved at most sizes. Confidence is
   moderate. The frontier's factor-of-2 selection grid (`select/SELECTION.md`)
   had shallow minima at 24–32 bits.
5. Panel K: Kummer and low-x within about 10% of each other at matched
   columns. The historical 24-bit coset/random interval included parity.

## 11. What was run before this draft (disclosure)

All runs used the binary built from `abee07ee7337d04a7e734333669ca1bacd6c763a`
(`ecbench` SHA-256 `35a9017d9cb1819d…974a8490`, rustc 1.99.1). They ran with
`taskset -c 28-31`, `RAYON_NUM_THREADS=1`, `ulimit -v` 16 GiB and
`--cpus none --allow-busy` (L0), on the shared 32-vCPU spot host while
another experiment ran. They are sizing probes only. They used the frontier
ladder curves and probe target seeds 20261018901 and 20261018903, which are
disjoint from every protocol seed. Specs are in [`probes/`](probes/). The
sessions were not kept.

| probe | what | result (*measured*) |
|:--|:--|:--|
| A | frontier 24-bit rung, 2 targets, every oracle of §4.3 at `N = 162`, plus a stride-7919 base | 20/20 verified; 7.0 s wall in all; peak RSS 34 MiB. Band `S` (w = 0/0.1/1): `mitm2` 14.11/15.57/28.70; `batch2` identical; `mitm3` 41.08/45.40/84.24; `rfmitm3` 42.48/46.81/85.76; `rfsub3` 41.08/45.41/84.37; `rfsub2` 2,302; `sub2` 2,302; `rfpairs3` 5,879; stride 15.47/17.21/32.84; rho 1.20/1.31/2.31. Per-run wall 32–48 ms (m = 2, 3), 0.2–0.8 s (`rfsub2`, `sub2`), 2.1 s (`rfpairs3`) |
| B | frontier 32-bit rung, 1 target; `mitm2` and `mitm3` at `N` = 512, 1024, 2048; `rfsub3` at 2048 | 8/8 verified; 5.2 s in all. `mitm2` `S` 42.8/32.8/72.6, wall 0.76/0.42/0.60 s, RSS 96/94/188 MiB; `mitm3` `S` 171.7/94.7/88.0, wall 1.08/0.63/0.73 s, RSS 42/67/190 MiB |
| C | the same target, `mitm3` at `N = 4096` (largest possible extension) | verified; `S` 262.2; wall 2.84 s; RSS 658 MiB |
| plan | `ecbench plan` on every train, holdout (placeholders filled) and panel-K spec | incumbent method ids equal the frontier's; T/H curves unregistered (fresh); 232/232/208/208 train and 1,008 holdout executions per size; panel K 160 per curve |
| fb | `ecbench fb` for `prime-kummer:t` on the panel-K curves | 68, 129 (22-bit, t = 7, 8); 119, 245 (24-bit, t = 8, 9) columns |
| dry | `run.sh select` on probe B's records (arm names rewritten), and `run.sh verify` on probe A | template fill, hash freeze and replay check behave as written; no protocol session involved |

Nothing on a train, holdout or panel-K workload has run.

## 12. Before freezing (blockers)

1. **Weighted paired intervals.** `ecbench compare` has no weight option.
   See `src/bin/ecbench.rs:228` (`Compare`) and
   `src/cryptanalysis/ecbench/compare.rs:288` (`compare`). `ecbench band`
   (`src/cryptanalysis/ecbench/band.rs:187`) ranks means without intervals.
   Add `--weight W` and price each record with `band::priced_s` before the
   bootstrap. Until then `run.sh analyse` exits 3, and §7.3 condition 3 can be
   read at `w = 0` only.
2. **Random-base control.** `solve_ic_prime` admits only prime-abscissa,
   prime-projected-abscissa, glv-orbit, prime-kummer, prime-isogeny and
   cm-scalar-orbit (`src/cryptanalysis/ecbench/methods.rs:2125–2137`). There is
   no seeded random signed-point base. This does not block the primary
   decision. Either land a `prime-random:size=N,seed=S` plug-in and add a
   `random` holdout control before freezing, or freeze with the stride
   control alone and record the gap.
3. **Pin and record.** Merge the card files (and blocker 1). Set
   `code.commit` to that merge commit. Rerun the plan check so the incumbent
   method ids still equal the frontier's. Commit this file with the status
   line changed to FROZEN, and record `git hash-object PROTOCOL.md` as
   `protocol.blob`. Then run `backlog validate`.

The selection and template filling in `run.sh` are jq over `ecbench band`
JSON, which is thin orchestration under AGENTS.md "no Python". A reviewer may
ask for them to move into `ecbench` before freezing.

## 13. Outputs and landing

`OUT = research/prime_base_oracle_tuning_<run-date>/` holds the following:
`host/`, `progress.txt`, the `sessions/` tree, `audits/`, `select/` (band,
selection, `FROZEN.sha256`), `specs/` (filled holdout and extension specs),
`analysis/` (band, table, compares), the report, graph and PDF
(research-visuals), and benchmark copies. If an improvement is found, the
same PR also carries the leaderboard regeneration (§7b) as a prime-field
pipeline section outside tables A–C. Table B's legacy calibrated unit is not
mixed. `run.sh verify OUT` is the independent check. It re-executes every
measured run of every session into `OUT/verify/` and checks the selection
hashes and every measured scalar. A second-host replay
(`ecbench-independent-runner`) is required before a leaderboard promotion.
