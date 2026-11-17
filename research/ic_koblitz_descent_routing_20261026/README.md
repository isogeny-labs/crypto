# Where should the planner route large Koblitz curves?

> **2026-10-26, follow-up.** Re-tested at six sizes (36.6–47.2 bits) in
> [`../ic_planner_exhaustive_20261026`](../ic_planner_exhaustive_20261026/README.md): the descent costs
> 1.31–2.04× the pipeline, every interval above 1. The routing is unchanged.

Measured 2026-10-26 under [`PROTOCOL.md`](PROTOCOL.md) (frozen in
`cd3af7078`, before any session). PDF: [`REPORT.pdf`](REPORT.pdf).

`ecbench` gained `ic.koblitz_descent` in `b561ebca8`: the ledger §23 one-target
Koblitz pipeline, charged in counted group operations. On the same seeds its
logs and native counts are identical to `ic price --single-target`.

The leaderboard's Table A reports that pipeline at 4.7–11.8× rho, but its
units are mixed: counted IC work against rho in canonical step units. The
planner's own recipe for these curves (`compact-orbit-scan` +
`mitm-frobenius-counted:m=2`) reads 18× at 2^39 in `ecbench`'s counted unit.
This study runs both in one unit, in one session per curve, against
`rho.signed_frobenius_strong`.

## Result

**The planner's existing recipe is the cheaper one.** In counted `S` the §23
descent costs 1.41–1.49× the `ic.pipeline` plan at all three sizes, with every
interval above 1. The planner keeps its routing; `ic-planner/3` now cites this
study for log2 r ≥ 36.

| curve | slug | log2 r | S rho | S pipeline | S descent | descent / pipeline | pipeline / rho | descent / rho |
|:--|:--|--:|--:|--:|--:|:--|:--|:--|
| k47 (a = 1, n = 47) | `icv1-f2m47-t22705043-f4e44623` | 36.64 | 0.139 | 1.312 | 1.864 | 1.421 [1.310, 1.541] | 9.46 [7.93, 11.30] | 13.45 [11.19, 16.17] |
| k41 (a = 0, n = 41) | `icv1-f2m41-tm2308219-7f48b14a` | 39.00 | 0.132 | 1.966 | 3.185 | 1.489 [1.433, 1.547] | 16.19 [13.16, 19.92] | 24.10 [19.64, 29.57] |
| k53 (a = 0, n = 53) | `icv1-f2m53-tm56619371-dac20a85` | 44.26 | 0.114 | 2.728 | 3.845 | 1.409 [1.382, 1.438] | 23.99 [18.92, 30.42] | 33.81 [26.31, 43.44] |

- **Source:** paired `ecbench compare`, cluster 95% intervals, 24 pairs.
  Mean S comes from `ecbench band` at w = 0.
- **Verification:** all 96 runs of every session verified.

**Rho is far cheaper than both.** The distance remaining runs from 9.5× to
24× for the pipeline and from 13× to 34× for the descent. It grows with size
for both, as the §23 slope says.

**Why Table A read lower.** Its rho is priced in canonical step units, which
charge rho's canonicalisation. `ecbench` charges group operations only, and
`rho.signed_frobenius_strong` folds by 2n, so its counted `S` is 0.11–0.14.

**Where the descent's cost goes.** It is mostly the target-independent
precomputation. At k41, relation collection is about 2.05 M GAE and the pair
table 0.27 M GAE, while the online descent is 1.0 k–184 k GAE per target
(`records.jsonl` phases). That precomputation is reusable across targets.
A many-target comparison would need batch rho (Kuhn–Struik, `docs/ic/FRAMEWORK.md`
§3) and is a different question from the planner's one-target plan.

## Across the §5a band

`ecbench band` cannot price `ic.koblitz_descent` above w = 0. Four of its
native categories have no count yet:
- field arithmetic outside the group law;
- hashing;
- the descent's modular combination;
- declined AVX-512 lanes.

Its order against the other arms at w = 0.1 and 1 is therefore "not ranked",
as the protocol said. The pipeline and rho are priced across the band in
`analysis/band-*.txt`.

## Predictions (PROTOCOL.md)

- **P1** (`descent / pipeline < 1`, interval below 1): **failed at all three
  sizes**. The ratio is 1.409–1.489, with every interval above 1.
- **P2** (`descent / rho` grows with log2 r and stays above 1): **held**: 13.4,
  24.1, 33.8.
- **P3** (every run verifies): **held**, 288 of 288.

## Classification and decision

Engineering: a routing choice between two existing recipes, decided by the
protocol's rule.
- **Routing.** P1 failed, so the planner keeps `ic.pipeline` for Koblitz
  curves. `ic-planner/3` adds this study as evidence for log2 r ≥ 36.
- **Table A.** Its descent figures are mixed-unit (accounting). In the counted
  unit they are 13–34× rho, not 4.7–11.8×. Re-pricing Table A in this unit is
  left to the owner, and no leaderboard row moves here.
- **The new method.** `ic.koblitz_descent` stays in `ecbench`, so the §23
  pipeline can now be compared in one unit.
- **Still to do before band rankings.** Counting its four uncounted native
  categories.

## Deviations from the protocol

- **No resume checkpoint.** `scripts/checkpoint.sh gate` could not upload: the
  session has no `CHECKPOINT_UPLOAD_TOKEN`. The owner told the session to
  proceed. Every input was committed and pushed before the run, and every
  output after it.

## Evidence

- **Specs:** `specs/k47.json`, `k41.json` and `k53.json`, from `scripts/make_specs.sh`.
- **Card:** `cards/CARD-ic-improve-20261026-2.yaml` and its receipt. The
  chain was `experiment-director` → `executor-mechanical`.
- **Sessions:** `sessions/k47`, `k41` and `k53`, 96 records each, all verified.
  Binary sha256 `d0ff6d06…85fa`, built at `b561ebca8`; clean tree at
  `cd3af7078`.
- **Audits:** `sessions/*.audit.json`, all OK with 12 exact replays.
- **Derived:** `analysis/band-*.{txt,json}` (`ecbench band`) and
  `sessions/*/comparisons/` (`ecbench compare --save`). The figure comes from
  `scripts/plots.py`, which only lays out the plot.

## Note, 2026-11-01: byte counters are no longer priced as operations

This README and its `analysis/band-*` files are frozen as measured. The band
priced `factor_base_vec_bytes_floor_uncharged`, a byte count, at `w` GAE per
byte. The owner decided (option B, 2026-11-01) that the band prices operations
only (`band.rs` `NON_OPERATION_COUNTERS`); the counter stays listed.
[`analysis/exbytes/`](analysis/exbytes/) holds the bands re-run under that rule
(`sessions/` untouched): three arm rows change at w > 0, by at most 12%, and no
band-rank verdict changes. No weighted comparison was committed, so none was
re-run. Audit and rationale:
[`research/ic_byte_counter_pricing_20261101`](../ic_byte_counter_pricing_20261101/README.md).
