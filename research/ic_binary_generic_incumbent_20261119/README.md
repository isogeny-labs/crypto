# Lane B: generic binary one-target IC incumbent, and `free=1` on the pruned k-sum table (2026-11-19)

Every number is `[pcc:unverified]`. Counted cost (L0), single thread, one local host, not an admitted measurement: no leaderboard row, no `benchmarks/` row,
no `VALIDATION.json` (no second-host replay, no independent validator). Scope: generic random-binary curves, one target, charged cold; no multi-target, no weak family.
Frozen design: [`PROTOCOL.md`](PROTOCOL.md) (sha256 in `PROTOCOL.sha256`, registered 2026-11-19T19:49:03Z, before any IC record).

## Result

**Negative at the level that counts.** `ksum:m,prune=1,free=1` (verified build collisions fed to the solver as target-free rows) does not beat the
strictest pruned k-sum incumbent on any of the three curves (n = 21, 24, 27; 432 paired runs per arm per curve, 3 replicates). Strictest-against-strictest
(arms chosen on the same data, 8 incumbent and 8 candidate arms), cost ratio candidate / incumbent, pooled, 95% cluster-bootstrap interval:

| curve | w = 0 | w = 0.1 | w = 1 | strictest incumbent -> strictest candidate (w = 0) |
|:--|:--|:--|:--|:--|
| B21 | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] | k5-p-N0 -> k5-pf-N0 (same arm: no collision in a 9-column k5 table) |
| B24 | 0.991 [0.969, 1.013] | 0.999 [0.985, 1.015] | 1.000 [1.000, 1.000] | k5-p-N0 -> k4-pf-N0 |
| B27 | 0.988 [0.972, 1.003] | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] | k5-p-N0 -> k4-pf-(N0-1) |

Per-replicate point estimates at w = 0 (B24 0.982 / 0.979 / 1.012; B27 0.977 / 0.969 / 1.018) straddle 1. The registered kill rule (upper end >= 1 at some w on every curve) fires.
Not an improvement over the incumbent; the incumbent stands.

What did move, at a fixed configuration (paired, same (m, N)): `free=1` on the **m = 4** table is a real but small saving, 0.939 [0.935, 0.944] / 0.937 / 0.926 (w = 0 / 0.1 / 1) at B27
N = 16, 0.978 [0.975, 0.981] / 0.977 / 0.973 at B24 N = 12, none at B21; on m = 5 it is exactly 1.000 at the incumbent sizes (no collisions to use). The m = 4 table is not the strictest arm
(the m = 5 table beats it by 1.0-6%), so this saving does not reach the incumbent. This is a constant-factor result for a generic square-root-class method; no exponent is touched.

**Why (mechanism, measured).** A free row needs a genuine collision of two distinct signed k-sums. At the incumbent sizes the table has E = 2^(m-1) C(N, m) entries and about E^2/(2r) genuine collisions
(B27, m = 4, N = 16: E = 14,560, r = 22.4 M, about 4.7), against N + 1 = 17 independent rows required. The harness emits 57 free rows there, but 54 are linearly dependent
(`matrix_dependent`; mean rank gain about 3), so the target needs 13.8 hits instead of 16.6 and the trial count falls 17% (12,730 -> 10,612); the relations phase is only about half of S, so S falls 6%.
Growing N for more collisions costs more in build than it saves (k5-pf at N + 2 on B27: 9.48 against 6.13). My preregistered model (cost E/2 + (N+1) r/E, best 0.71x) counted every colliding
pair as an independent row; the measured rank is the genuine-collision count, so the optimum does not move and the model's 0.71x is not available (P4, P5 falsified).

### Distance to matched rho (never a pass/fail gate): S(strictest) / S(rho-neg), same targets, rho from 432 runs per curve

| curve | S rho-neg (w = 0 / 0.1 / 1) | incumbent S (strictest at each w) | incumbent / rho | candidate / rho (strictest free=1 arm) |
|:--|:--|:--|:--|:--|
| B21 | 1.250 / 1.359 / 2.337 | 6.409 / 7.140 / 13.710 (k5-p-N0) | 5.13 / 5.25 / 5.87 | 5.13 / 5.25 / 5.87 |
| B24 | 1.205 / 1.314 / 2.293 | 6.391 / 7.081 / 13.211 (k5-p-N0, N0+1, N0+1) | 5.30 / 5.39 / 5.76 | 5.25 / 5.38 / 5.76 |
| B27 | 1.028 / 1.123 / 1.977 | 6.127 / 6.890 / 13.759 (k5-p-N0) | 5.96 / 6.13 / 6.96 | 5.89 / 6.13 / 6.96 |

The incumbent is 5.1-7.0 times rho across the band; free=1 closes at most 1.2% of that gap (B27, w = 0) and is not distinguishable from zero. Native work priced at w = 0.1 and 1 GAE per operation
(lookups, target-guard probes, row operations and counted table inserts: the unpriced native set of these arms): the incumbent's native work is about 1.2 times its group-addition cost at B27 (7.6 against 6.1 per sqrt r), so
rho's lead widens from 5.96x at w = 0 to 6.96x at w = 1; nothing in this lane reorders any arm against rho.

### Incumbent identification and cost decomposition (item 1 and 2)

Best one-target IC arm on binary curves with n <= 29, from `docs/ic/LEADERBOARD.md` tables A-C and the component sections (GHS-weak section excluded) and `research/`: the pruned negation-folded k-sum table on random binary curves
(`research/ic_tableb_binary_ksum_20261115` + `research/ic_ksum_prune_20261115`), 4.97-6.59x the matched negation-map rho at w = 0-1 (as published). Koblitz: the best claimed arm
(`research/koblitz_ksum_fold_study_20261115`, K1 n23, 0.88 / 1.1 / 2.26 of rho-strong) failed independent validation (D1: wrong incumbent, gain is the generator-orbit base; D2: rho v5 is 3-4x cheaper than rho-strong), and the validated K1 row
is 5.6-5.95x rho at n23. Random binary is therefore the incumbent lane. Decomposition of the strictest arm (this lane's measurement, GAE / sqrt r at w = 0, native work / sqrt r in brackets; build = oracle_setup):

| curve, arm | table build | relations (walk + probes) | linear algebra | factor base + verify | trials per solve | hits needed |
|:--|:--|:--|:--|:--|:--|:--|
| B21 k5-p-N0 | 2.68 [1.98] | 3.69 [5.03] | 0.00 [0.30] | 0.04 | 2,574 | 9-10 |
| B24 k5-p-N0 | 1.90 [1.40] | 4.45 [6.92] | 0.00 [0.21] | 0.04 | 5,012 | 10 |
| B27 k5-p-N0 | 3.35 [2.68] | 2.76 [4.84] | 0.00 [0.11] | 0.02 | 11,450 | 13 |

Build 30-55% of S at w = 0, relations 45-70%, linear algebra and verification negligible. This is the structure the free-row lever acts on (it can only remove hits, and only as many as the table has
genuine collisions).

### Predictions (PROTOCOL section 7)

| id | prediction | outcome |
|:--|:--|:--|
| P1 | all runs verified | **held**: 22,032 runs, 0 failures, 0 unverified ([d]G = Q recomputed in the example for every run) |
| P2 | collisions >= 1 and >= 0.5 free rows per run at n27 N0 | **falsified for m = 5** (0 free rows at N0 on all three curves); held for m = 4 (n24: 10, n27: 57 emitted, mostly dependent) |
| P3 | k5 free/no-free R_0 in [0.90, 1.00] on all three, saving growing with n | **not held**: R = 1.000 exactly (no collisions), no trend; k4 shows 1.000 / 0.978 / 0.939 (grows with n, but m = 4 was not the predicted arm) |
| P4 | strictest R*_w upper end < 1 at all w on >= 1 curve, R*_0 in [0.75, 0.95] | **falsified**: R*_0 = 1.000 / 0.991 / 0.988, intervals contain 1 |
| P5 | strictest candidate at N above the incumbent's on >= 2 of 3 curves | **falsified** (candidate N is N0 or N0 - 1, or the same arm) |
| P6 | abs(R*_1 - R*_0) < 0.06 | held, trivially (differences 0, 0.009, 0.012) |
| P7 | distance >= 3.5 at all w and curves, shrinking <= 25% | **held** (5.1-7.0; shrink <= 1.2%) |
| P8 | example unit within 8% of the committed prune-study values | **held**: k4-p / k5-p / rho-neg within -3.8% to +1.5% on all curves and weights (`results.json`, `P8_crosscheck`) |

## What this lane could not do with the tools present

- **ecbench at HEAD cannot run the k-sum oracle on binary curves.** `ecbench run` on `ksum:m=5` over a `binary_explicit` curve returns `oracle ksum does not run on a binary curve`. The dispatch line
  `"ksum" => Box::new(KSumOracle::new(ms))` that commit d87eb39548 added to `solve_ic_binary` (src/cryptanalysis/ecbench/methods.rs) is absent at HEAD 1fa1a5193c (lost in a merge repair), so the committed binary k-sum
  sessions (`research/ic_tableb_binary_ksum_20261115`, `research/ic_ksum_prune_20261115`) no longer replay with the current binary. `src/` is frozen for this lane; the one-line fix is the coordinator's call. The same
  loss affects `KSumTable::build_negation_folded_split` (`split=1`, commit fc0babc73e): it is not in HEAD's `plugins.rs`/`ic_boundary.rs` (grep finds no `split` parameter), so that lever cannot be measured either.
- Consequence: the IC arms were run by `examples/icbin_ksum_free.rs` (the one new file this lane adds), which calls the same library entry points as `solve_ic_binary`. Cost is counted in the same unit, but it is not an
  `ecbench` record, so `ecbench verify --replay`, `ecbench compare --weight` and `ecbench band` apply only to the rho reference. Paired intervals are this lane's own cluster bootstrap (`analyze.py`, 4000 resamples,
  clusters = (replicate, target), 3 rounds inside a cluster). Replay evidence for IC arms: counts are deterministic; the duplicate arm `k5-p-p0-aa` equals `k5-p-p0` exactly (A/A 1.0000 at every w and curve).

## Deviations from the protocol and disclosures

1. Pre-study IC smoke (not data): the example was run on 3 targets of B21-r1, 1 round, arms k5-p-N0, k5-pf-N0, k5-pf-N+2, to debug it (it showed `verified` false because the planted log was unknown to the library: fixed by
   recomputing [d]G = Q; it also showed 6 free rows at k5 N+2 and S 10.9). After the protocol freeze, before the study; `smoke/` holds the config only.
2. The rho reference was first run with 1 round per spec (`superseded_1round_sessions/`, B24 rho-neg 1.03 against the committed 1.21 from only 48 runs); before any IC arm was analysed it was re-run with 3 rounds (432 runs per
   curve, same target seeds, same targets). Only the 3-round sessions are used.
3. The ecbench sessions use `--lock /tmp/crypto-bench-laneB.lock --allow-busy --cpus none`, not the default shared lock: the other lane held `/tmp/crypto-bench.lock` for its timed sessions and my `--wait` stalled.
   The runs are L0 count-only, so the lock cannot change a count.
4. The unit of IC arms: S = `RunReport.total_gae / sqrt r` with `Calibration::pin(regime, slug)` (as `ecbench` does), and the weighted term U = unpriced native operations of these arms (`lookups`, `target_guard_probes`,
   `row_ops`) plus counted `table_insert_attempts`, per `ecbench band` since 2026-11-08. I identified this U by matching the committed prune-study mean (native 7,429 per run at B21 k5-p against 7,472 here); the match is the P8 check, not a proof of identity with every ecbench counter.
5. Winner's-curse symmetry: both strictest arms are chosen on the same pooled data, so any selection advantage is shared (B24 and B27 improved 0.991 and 0.988 by point estimate only).
6. `analysis/` is empty (the script lives at the lane root as `analyze.py`).

## Reproduce

```
cd research/ic_binary_generic_incumbent_20261119
python3 make_specs.py rho && ./run_rho.sh               # matched rho sessions (ecbench, binary sha256 in BINARY.sha256) and the 48 public targets x 3 replicates
python3 make_specs.py configs                           # example configs: targets read from the rho sessions
CARGO_INCREMENTAL=0 cargo build --release --example icbin_ksum_free   # from the repo root; sha256 in EXAMPLE.sha256
./run_ic.sh                                             # 9 example invocations (3 curves x 3 replicates), about 70 s total, raw/ (gitignored)
for n in 21 24 27; do for r in 1 2 3; do ecbench band --dir sessions/B$n-r$r --json band_B$n-r$r.json; done; done
python3 analyze.py                                      # results.json, figure.svg
```

Raw records (gitignored): `raw/B<n>-r<rep>.jsonl` (41 MB), `sessions/*/records.jsonl`; SHA-256 of every raw file, session record and config in `RAW.sha256`.
Derived: `results.json` (33 KB: arm means, T1 and T2 with intervals, P8, decomposition), `figure.svg` (S at w = 0 per arm, incumbent vs free=1, with rho).
Example binary sha256 `17d70407df1f92884400e80a268f9f867b6a52971bc413ee98f02437b75b2ac9`; ecbench `77c716dd12e072a053e3f140ff6c5512ddbb2c74ba43b8bda1656ad52eefeab8`.
