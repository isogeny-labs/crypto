# Constants of the prime-field pair-table pipeline: every existing variant at the optimal |F| (2026-11-19)

Every figure is **[pcc:unverified]**: counted cost at isolation L2 on one host, no
validator report, no `VALIDATION.json`, no leaderboard row. Protocol:
[`PROTOCOL.md`](PROTOCOL.md) (SHA-256 `97cfed6c49b9545241ac9c82af05e1c7cf84633581b41ee4a8042749c07791e1`,
written before the measured sessions; one 1-target smoke test preceded it, see
Deviations). Study of the model it squeezes: `research/ic_fb_size_model_20261119`.

![ratios](figures/ratios.svg)

## Result

**No existing variant that keeps the 2-summand pair-table structure lowers either constant
a (table) or b (trials) of the incumbent.** Across 6 oracles, 6 target strategies and 3
linear-algebra rules, at p24 (|F| = 128) and p28 (|F| = 256), 8 targets x 3 rounds, every
m = 2 arm is either a tie by construction (identical charged work, interval [1, 1]) or
dearer than the incumbent, at every weight w = 0, 0.1, 1. The one m = 2 arm with an
interval below 1 anywhere, `stride-renew-deferred-guard` (dg), is below 1 **only at w = 1**
(0.811 [0.711, 0.932] p24, 0.735 [0.659, 0.823] p28), is 1.055 / 1.024 at w = 0 with
intervals containing 1, and straddles 1 at w = 0.1: not an improvement under the rule
(w = 0 upper end below 1 and the order holding across the band, AGENTS.md 5a).

**The only arms that beat the incumbent are `ksum:m=3` (and, post hoc, m = 4, 5) at a
much smaller |F|.** That is a different structure (a 3-sum table, ~|F|^3 entries and
~N/|F|^3 trials), not a squeeze of a and b, and the k-sum family is not new here
(`research/ic_tableb_ksum_20261115`, `ic_ksum_prune_20261115`, `ic_ksum_sibling_20261118`:
m = 5, 5 to 7x rho at 20 to 24 bits). Preregistered m = 3 arms against `inc`: best
0.666 [0.597, 0.755] (p24, |F| = 32) and 0.488 [0.431, 0.566] (p28, |F| = 48) at w = 0, with
the order holding at w = 0.1 and 1. This says the "incumbent" the size-model study
squeezes is itself 1.5 to 3x dearer than the best k-sum arm already in the repository; it
is not a new result against the leaderboard baseline, and it was not measured against the
registered ksum5 rows on these curves.

### Distance remaining to matched rho (rho.negation, same session, same targets)

Mean S arm / mean S rho, with rho S = 1.006 / 1.097 / 1.921 (p24) and 1.011 / 1.109 / 1.998 (p28) at w = 0 / 0.1 / 1.

| curve | arm | w = 0 | w = 0.1 | w = 1 |
|:--|:--|--:|--:|--:|
| p24 | `inc` (best m = 2 arm at w = 0 and 0.1) | 12.7 | 13.6 | 17.5 |
| p24 | `t_dg` (best m = 2 arm at w = 1) | 13.4 | 13.6 | 14.2 |
| p24 | `k3_b` (m = 3, \|F\| = 32, preregistered) | 8.5 | 8.8 | 10.3 |
| p24 | `k5_11` (m = 5, \|F\| = 11, post hoc) | 6.6 | 6.9 | 8.2 |
| p28 | `inc` | 20.1 | 21.6 | 28.4 |
| p28 | `t_dg` | 20.6 | 20.6 | 20.9 |
| p28 | `k3_a` (m = 3, \|F\| = 48, preregistered) | 9.8 | 10.3 | 12.6 |
| p28 | `k4_20` (m = 4, \|F\| = 20, post hoc, edge of sweep) | 6.7 | 7.1 | 8.6 |

Rho stays ahead at every row and weight. The incumbent distances here (12.7 / 20.1)
are lower than the size-model study's 14.6 / 24.4 because the targets differ (seed
20261120, rho S 1.006 and 1.011 against 0.93 and 0.88 there); the gap between the two
curves is the same shape.

## Which constant each variant moves

`a_eff` = `oracle_setup` GAE / |F|^2; `b_eff` = `relations` GAE / (N / 4|F|); means over the
24 measured runs per arm (full table: [`analysis/summary.md`](analysis/summary.md)).

| variant | a_eff p24 / p28 | b_eff p24 / p28 | ratio to `inc` at w = 0 / 0.1 / 1 (p24) | (p28) |
|:--|:--|:--|:--|:--|
| `inc` (blocked, walk, gauss) | 1.008 / 1.004 | 1.096 / 1.018 | 1 | 1 |
| oracle `mitm-prime-batch`, `mitm:negation_folded=1`, `rootfind-mitm-prime-batch` | same | same | 1.000 [1.000, 1.000] at all w | same |
| oracle `mitm:m=2` (unfolded) | **2.008 / 2.004** | same | 1.312 [1.281, 1.354] / 1.296 / 1.238 | 1.197 [1.177, 1.225] / 1.184 / 1.141 |
| oracle `rootfind-subtract`, `subtract` | 0 | **264 / 517** | 165 [154, 174] / 156 / 126 | 407 [392, 418] / 380 / 292 |
| targets `walk-prime-fast` | same | same | 1.000 at all w | same |
| targets `walk-prime-batch-16` | same | 1.264 / 1.023 | 1.104 [0.984, 1.246] / 1.104 / 1.104 | 1.004 [0.854, 1.185] / 1.003 / 1.002 |
| targets `stride-renew` | same | 1.185 / 1.048 | 1.055 [0.937, 1.193] / 1.063 / 1.090 | 1.024 [0.923, 1.137] / 1.025 / 1.030 |
| targets `stride-renew-deferred-guard` (= `dg_leader`, identical records) | same | 1.185 / 1.048 | 1.055 [0.937, 1.193] / 1.000 [0.885, 1.135] / **0.811 [0.711, 0.932]** | 1.024 [0.923, 1.137] / 0.955 [0.861, 1.063] / **0.735 [0.659, 0.823]** |
| targets `stride` | same | 2.94 / 3.01 | 2.150 / 2.197 / 2.355 | 2.570 / 2.600 / 2.696 |
| targets `random` | same | 76 / 86 | 48 [41, 57] / 41 / 19 | 68 [59, 77] / 58 / 25 |
| linalg `incremental-gauss-echelon` | same | same | 1.000 / 0.999 / 0.998 | 1.000 / 1.000 / 0.999 |
| linalg `incremental-gauss-full-rank` | same | 5.71 / 6.31 | 3.88 [3.42, 4.48] / 3.98 / 4.30 | 5.17 [4.48, 6.03] / 5.24 / 5.47 |
| `ksum:m=3`, \|F\| = 24 / 32 / 48 (p24) and 48 / 64 / 96 (p28) | not an m = 2 table: setup 8.6k / 20.8k / 71.4k GAE (p24), 71.4k / 170.7k / 580.6k (p28) | trials 26.9k / 14.1k / 6.8k (p24), 91.0k / 48.2k / 22.2k (p28) | 0.677 [0.608, 0.771] / 0.666 [0.597, 0.755] (best) / 1.491 | 0.488 [0.431, 0.566] (best) / 0.658 / 1.812 |

`dg` is the one variant that trades them: b rises 8% / 3% (counted at w = 0), while its
native work falls (no guard probe per target), which is why it wins only when native work
is priced at w = 1. Its w = 1 gain here (19 to 26%) agrees with
`research/ic_srenew_deferred_guard_20261029` (dg / srenew 0.756 to 0.766 at w = 1); the
w = 0 result there was at planner sizes, not the optimum. All ordering claims above are
paired (same targets, same seeds), 95% intervals are the cluster bootstrap over the 8
targets (`ecbench compare --weight w`, 4,000 resamples, seed 20261002). Identical-work arms
have intervals [1, 1].

## Inventory: what was already compared at the optimal |F|

Nothing in the m = 2 pipeline had been compared at |F| = 128 (p24) or 256 (p28). Prior
paired comparisons used other sizes: oracle builders at |F| = 1,024..16,384
(`research/ic_oracle_setup_blocked_20261020`: identical counts, wall only), dg vs dgb at
|F| = 162 / 407 / 1,024 (`research/ic_dg_blocked_20261029`: identical charged work), dg at
130 / 267 / 741 against walk (`research/ic_srenew_deferred_guard_20261029`), linalg at
|F| >= 1,024 (`research/ic_linalg_pipeline_20261019`: S identical), planner rules
(`research/ic_planner_weight_rule_20261021`, withdrawn 2026-10-26;
`ic_planner_exhaustive_20261026`). The full axis table with the variants not run
(`glv-orbit`: needs an endomorphism, a special class and the two curves have none;
`prime-projected-abscissa`: the wwalk base) is in PROTOCOL.md, Step 1.

## Predictions (PROTOCOL.md; frozen before the measured sessions)

| id | prediction | outcome |
|:--|:--|:--|
| P1 | batch, mitm_nf, rfmitm identical to `inc` | **held**: ratio 1.000 [1, 1] at all w, both curves |
| P2 | unfolded: ratio in [1.15, 1.60] p24, [1.05, 1.40] p28; a_eff 1.7 to 2.3x, b same | **held**: 1.312 and 1.197; a_eff 2.008 / 2.004 |
| P3 | rfsub, sub ratio > 20 | **held**: 165 / 407 |
| P4 | random > 20 | **held**: 48 / 68 |
| P5 | no targets variant meets the rule on both curves; pfast tie; dg upper end above 1 at w = 0 on at least one curve | **held** (dg above 1 on both, below 1 at w = 1 only) |
| P6 | echelon ties (within 0.5%); full-rank >= 3 | **held**: 0.998 to 1.000; 3.88 / 5.17 |
| P7 | A/A exactly 1 | **held** |
| P8 | an m = 3 arm below 1 on p28; best p28 ratio in [0.45, 0.85]; p24 best in [0.70, 1.20]; p24 gain no larger than p28's | **partly failed**: p28 0.488 held, p24 gain smaller than p28's held, but p24 best 0.666 is below the predicted [0.70, 1.20] |
| P9 | no m = 2 arm meets the rule on both curves; best-arm distance within 15% of 14.6x / 24.4x | **first clause held; distance clause failed at p28** (20.1x, 17.6% below 24.4x; p24 12.7x is 13% below 14.6x): a target-set difference, not a variant |
| P10 | best m = 3 distance on p28 in [10x, 20x], below 24.4x | **failed narrowly**: 9.8x at w = 0 (below 24.4x holds) |

## Deviations and caveats

- **D1.** 3 measured rounds is unbalanced for `alternate_measured` (ecbench warns; the counted cost is deterministic, so only wall order is affected, and no wall claim is made).
- **D2. Post hoc, not preregistered:** `sessions/x24`, `x28` (`specs/x24.json`, `x28.json`, [`analysis/followup.md`](analysis/followup.md)) sweep `ksum` m = 3, 4, 5 at smaller sizes, same curves, targets and `inc`. Reason: the preregistered m = 3 sweep put the p28 optimum at the lower edge; the results show the m = 3 optimum is bracketed (p24 |F| 24 to 32; p28 about 40 to 48) while m = 4 at p28 is not (|F| = 20 is the best and largest tried, 0.335 [0.297, 0.381], so the optimum is at or above 20) and m = 5 is flat from 13 to 16. Exploratory; no claim beyond "m >= 4 is cheaper than the pair pipeline here", already known from the k-sum studies.
- **D3. Smoke test.** A 1-target, 1-round run of every arm on target seed 20261031 preceded PROTOCOL.md (checking that each arm string runs). Its band table was read; P2, P8, P10 were widened after it. The measured sessions use seed 20261120.
- **D4. Not run / not claimed:** no EKS or S3 route (no AWS identity in this container; run locally, `--allow-busy`, RAYON_NUM_THREADS=1); no benchmark record, replay certificate beyond `ecbench verify --replay 12`, validator report, `VALIDATION.json` or leaderboard row (so nothing here is a leaderboard row); no pcc claim id. The research-kb and lead-atlas servers were not queried; prior work was found by searching `research/`.
- **D5.** `glv-orbit` was not measured (see Inventory). `ksum` m = 3 is inside the owner scope (generic curve, one target) but is a different structure; it is reported apart.
- **D6. Native-work band.** All four sessions are priced by `ecbench` at w = 0, 0.1, 1 with table inserts priced (default). The `ksum` and `rootfind` arms carry the largest unpriced lookup counts; their order against `inc` held at all three weights (`k3_a`, `k3_b`, `k4_*`, `k5_*` rows). The dg gain exists only at w = 1 because its saving is native guard probes.
- **D7.** The session directories are ignored by Git (a hook appended `sessions/` to `.gitignore`); only specs, derived tables, audits and band files are committed material.

## Raw data (not committed)

SHA-256 of the raw records (deterministic counted values; regenerate with the commands below):

```
5aa2a52134a96c46c35365fbd45001ec11faea7d3c4ba6a915a080b5c6fde3ab  sessions/p24/records.jsonl
03631653dbd075a83be6a42c7605306f17c4ebe657c24f4fd0f72651c1560501  sessions/p28/records.jsonl
75cd2611bac94621faab584b43aac3b68af39d143e312493bf6d93cba3c2511a  sessions/x24/records.jsonl
435e3d048088866a0df071629e9aeafe0e7dad49ba7b952c5493a0f86403ce65  sessions/x28/records.jsonl
```

Audits: `ecbench verify --replay 12` reports 672/672 verified, 12/12 replays reproduced on p24 and
p28, 352/352 and 12/12 on x24 and x28 ([`analysis/*.audit.txt`](analysis)). Bands: `analysis/*.band.txt`.
Derived: `analysis/summary.json` (45 KB, all comparisons), `analysis/followup.json`.

## Reproduce

```sh
cd research/ic_prime_pair_constants_20261119
python3 analysis/gen_specs.py 8 3          # specs/p24.json, p28.json
python3 analysis/gen_followup.py           # specs/x24.json, x28.json (post hoc)
export RAYON_NUM_THREADS=1
for c in p24 p28 x24 x28; do
  ../../target/release/ecbench run --wait --allow-busy --spec specs/$c.json --out sessions/$c
  ../../target/release/ecbench verify --dir sessions/$c --replay 12
  ../../target/release/ecbench band --dir sessions/$c
done
python3 analysis/analyse.py; python3 analysis/followup.py
```

About 15 min for p24 and p28 together plus 4 min for the sweeps, single thread, on a 4-CPU host shared with another lane.
