# Protocol: K1 set-up levers at the w = 1 end (lazy negatives + Z[τ] same-orbit plan), n23 / n29 / n37 (2026-11-18)

Frozen before any data of this study (hashes in `REGISTRATION.sha256`; the registration commit is named in
`registered-at.txt`, written after it). Class: **confirmatory engineering-lever study**, three curves (K1 a = 1 n = 23,
a = 1 n = 29, a = 0 n = 37), T0 scope. Every number is `[pcc:unverified]` until a validator signs it.

## 0. What was seen before freezing (disclosed)

Attribution of the current best arm per curve from the earlier studies' raw sessions (scratchpad `sw-sessions/`,
`lsc-sessions/`, `sw8-sessions/`; other seeds, not reused), by the count model `model.py` (copied from
`koblitz_n37_sibwalk_k8_20261118/model.py`, which reproduces all 1,296 incumbent records of those sessions exactly).
Mean per solve, priced at w = 1 (every native operation 1 GAE):

| curve (arm) | base | oracle set-up | walk (relations) | check + LA | total w = 0 / 0.1 / 1 |
|:--|:--|:--|:--|:--|:--|
| n23 (sw, k = 4) | 15 dbl + 92 Frobenius + 92 negations + 4 checks | 186 add + 368 canon + 368 inserts + 182 sib | ~3 per target tried | ~29 | 430 / 637 / 2493 |
| n29 (ls, k = 1) | 29 Frobenius + 29 negations + 1 check | 15 add + 29 canon + 29 inserts + 14 sib | ~3 per target tried | ~27 | 66 / 86 / 265 |
| n37 (sw, k = 8) | 35 dbl + 296 Frobenius + 296 negations + 8 checks | 1188 add + 2368 canon + 2368 inserts + 1180 sib | ~3 per target tried | ~32 | 2157 / 3236 / 12940 |

Findings that fixed the levers (all before data):

1. **Negatives are formed and never read.** The layout base pushes `(σ^t(S), −σ^t(S))`; with `sib=1` the table build
   forms `P − X` from `X` and never reads an odd slot (except a non-sibling fallback, never taken on these curves), and
   the walk, ladder check and linear algebra never read base points. Only the probe of the single relation per solve
   reads its two summands, one of which may be a negative. The base charges `k·n` `point_negations_uncharged`
   (92 / 29 / 296) per solve.
2. **Insert attempts that repeat a key.** An integer model of the layout build (logs mod r, λ the Frobenius
   eigenvalue) reproduces attempts / entries exactly (n23 368 / 350, n29 29 / 26, n37 2368 / 2326). Three of the
   repeats in every orbit (and the doubling `P + P`) follow from identities of `Z[τ]` that hold for every point:
   `2 = μτ(1 − μτ)`, `1 + τ² = −(1 − μτ)`, `1 + μτ³ = −(1 + μτ)` (μ the trace). The other repeats (n23 6, n37 18)
   come from the seed spacing `2^5` and wrap-around of `τ^n = 1`; they are not targeted.
3. **Table size (balance of table against walk) is not targeted.** At k = 8 on n37 the table costs ~1.7× the expected
   walk at every weight (the per-entry and per-trial prices are nearly equal at w = 0 and w = 1), so a smaller table
   (k = 7) is expected to win at all three weights by ~3–4%; but changing k changes the walk (`targets_tried` is
   geometric-like and unpaired across k), and the earlier k = 9 test failed on exactly that noise. Not testable at this
   protocol's size; left as an open thread.
4. Not removable without dropping a check or changing the walk: Frobenius maps (the build reads every image of every
   later orbit; the closure check reads all n), canonicalisation + lookup per target tried, canonicalisation + insert
   per entry, the seed's curve check, the recovery check.

Code dd2fae0bab adds the two default-off options and the unit test
`k1_lazy_negatives_and_orbit_identities_keep_the_solve` (off = incumbent bit for bit; same table keys and probe
answers; exact count deltas; same targets, relation count and recovered log end to end on n23, n29, n37).

## 1. Candidate (fixed before data; no tuning parameter)

`w1` = the current best arm of the curve with `lazy_neg=1` (base) and `orbit_ids=1` (table):

- **lazy_neg** (compact-orbit-scan, needs `lean_layout=1`): the negatives are not formed (slot holds the identity;
  index, column, coefficient and negation map unchanged). A probe hit that names a negative forms it from its partner
  and counts `point_negations_uncharged` in the relation phase (priced at w like the base's). Other readers refuse
  such a base (non-layout tables, m = 3, `tau_verify`).
- **orbit_ids** (layout table, needs `sib=1`, n ≥ 7): per orbit, `P + P`, the plus half of rotation 2 and the sign-μ
  half of rotation 3 are not formed; a pair formed without one half is charged one addition, no sibling second
  point. Same key set and entries; the key of `2P` is stored as `(P, −μσ(P))`.

Expected per solve: base −k·n native; set-up −k additions, −2k sibling second points, −3k canonicalisations, −3k
inserts; relation phase +0..2 negations; walk, targets, relation, LA and check unchanged.

## 2. Arms (per session; identical arm list in the 3 sessions of a curve)

| arm | definition | role |
|:--|:--|:--|
| rho-strong | `rho.signed_frobenius_strong` | reference (leaderboard's current) |
| rho-v5 | `rho.signed_frobenius_v5` | reference (strongest measured) |
| inc | current best arm (n23: sp-c4-d5 + lean + sib + walk_sib=8 + ladder_verify + orbit_key_reuse; n29: gen-c1 + lean + sib; n37: sp-c8-d5 + same levers as n23) | incumbent |
| inc-aa | inc again | A/A control |
| inc-off | inc + `lazy_neg=0` / `orbit_ids=0` | default-off control |
| **w1** | inc + `lazy_neg=1` / `orbit_ids=1` | **candidate** |
| w1-neg | inc + `lazy_neg=1` | single lever (secondary, descriptive) |
| w1-ids | inc + `orbit_ids=1` | single lever (secondary, descriptive) |

Specs: `make_specs.py` -> `specs/n{23,29,37}-r{1,2,3}.json`. 48 public targets, 3 rounds, 1 warm-up, order
`alternate`. Fresh target seeds (unused in the repository at freeze): n23 20261118771-773, n29 20261118774-776,
n37 20261118777-779 (replicates 1-3); measurement seed = target seed + 1000. Sessions run in the order of `run.sh`.

Binary: ecbench built from f54326cee1 (= dd2fae0bab plus the one-line compile fix in `cm_orbit.rs`, which sets
`negations_deferred: false` in a `FactorBase` literal that dd2fae0bab left out; no behaviour change) in a clean sparse scratch worktree (own CARGO_TARGET_DIR, CARGO_BUILD_JOBS=1,
under the CPU lock), sha256 `a6a3890498508b3fcb65bedcea2ce3708f9d98d9c44d775a88818f0fc2563a34`; `run.sh` refuses any other. Run with
`--cpus none --allow-busy` (L0), launched through `tools/benchrec.py run`.

## 3. Pricing, statistics, decision

`ecbench verify --replay-all` on every session (0 problems required). `ecbench band --ranks` per session and pooled
per curve. `ecbench compare --weight w --resamples 4000 --seed 20261002`, paired by target, per replicate and pooled.
Upper bound used = the larger of compare's two 95% upper ends (`ci95`, `ci95_bootstrap`). `analyze.py`,
`summarize.py` adapted from `koblitz_n37_sibwalk_k8_20261118` (three curves, arm names, single-lever arms).

- **Primary (one test per curve):** w1 / inc. **Improvement on a curve iff the upper 95% bound is < 1 at each of
  w = 0, 0.1, 1, pooled AND in each of the 3 replicates.** One rule at all three weights; no w-specific fallback for
  w1. Otherwise reported per weight and replicate as measured; no pass.
- **Secondary (descriptive, no claim):** w1-neg / inc and w1-ids / inc pooled. w1-neg is expected to be exactly 1.000
  at w = 0 (a tie by construction: no group operation moves) and below 1 at w = 0.1 and 1.
- **Gap (never a gate):** w1 and inc against rho-v5 and rho-strong, pooled, at the three weights, with the §5a band
  order (ranked only where the order holds at all three weights).
- **Controls (void the study for a curve if they fail):** inc-off and inc-aa identical to inc on every record (cost,
  phase counts and native maps without wall time, counters, outcome, `points_sha256`); every solve `verified` with
  `matches_target`; w1, w1-neg, w1-ids try the same `targets_tried` and recover the same log as inc on every
  (replicate, round, target).
- **Model check (secondary):** `model_check.py`: model vs every inc record; the candidate model applied to the paired
  inc record (with the relation phase's 0..2 negations read from the candidate record) vs every w1 / w1-neg / w1-ids
  record; model ratio vs measured.

## 4. Model expectation (not a measurement)

From the earlier sessions' incumbent records (`model.py` `model_w1`, one hit negation per solve), w1 / inc:
n23 0.991 / 0.974 / 0.949; n29 0.985 / 0.946 / 0.860; n37 0.996 / 0.986 / 0.972 (w = 0 / 0.1 / 1). Lever split at
w = 1: lazy_neg 0.964 / 0.894 / 0.977, orbit_ids 0.986 / 0.966 / 0.994 (n23 / n29 / n37). Every record of w1 should be
cheaper than its paired inc record at every weight, so the intervals are expected to be narrow.

## 5. Reading declared in advance

- The removed negation is `(x, x + y)`: one field addition (bitop/v1 prices `point_negations_uncharged` as one
  `FieldAdd`). The §5a band prices it at 0 / 0.1 / 1 GAE, so the w = 1 end overstates its real cost; most of the w = 1
  gain of w1 is this lever. The `orbit_ids` part removes heavier work (an addition or a second point, a
  canonicalisation and a hash insert each). The report gives the split and the wall time per phase (L0, descriptive).
- No change to the walk, the table's keys or the recovery check; no check is dropped.

## 6. Deviations declared in advance

- No EKS/Iso Lab, no AWS/S3: local L0 run; raw sessions gzipped in the scratchpad (`w1-sessions/`), SHA-256 of each
  uncompressed `records.jsonl` in `summary.json`; compact tables in `tables/`.
- No conductor and no `research-kb` server in this container; no novelty claimed.
- Registration is a local commit before data (not pushed). No pilot.
- rho-v5 leaves its per-step jump-index selection untallied (koblitz_rho_reference_20261115 section 1).
- Report figures and README are written after data.
