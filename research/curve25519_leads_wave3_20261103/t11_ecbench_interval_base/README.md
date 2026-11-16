# T11: an interval factor base in ecbench, and selected `bits` frames priced end to end (results, 2026-11-04)

**Class:** engineering change (new factor-base plug-in `prime-interval:t=T[,lo=L]`)
plus an end-to-end ecbench comparison in counted units on the k = 20 Curve25519
miniature. Exploratory, constant-factor. **Not a leaderboard row, not a
`benchmarks/` row, no validation-gate submission.** Rho remains ahead.

Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen at `505ecbb154` before any code,
Addendum C (restart-safe one-target chunks, pooled paired compare) appended and
frozen at `4eddc74edc` before any chunk data. Every number below is read from
`data/` by the commands named; nothing was retuned after data.

## Headline

- The argmax-J frame `L976503` halves the end-to-end S of the `pkm-semaev`
  bits arm against `lo = 0`: pooled ratio **0.437 [0.300, 0.637]** at w = 0.1
  and w = 1, **0.458 [0.314, 0.668]** at w = 0, 6 of 6 targets won (model
  0.463). The J ÷ cols frame `L541434` gives 0.483 [0.329, 0.711] (model 0.443).
- On the native-priced track the incumbent isogeny arm stays far ahead:
  S(L976503) ÷ S(iso-s1093) = **7.71 [5.06, 11.76]** at w = 0.1 (model about 6).
  At w = 0 the two are not separated (1.15 [0.88, 1.50]); P9 (< 1 at w = 0)
  **failed**.
- Distance remaining for the selected frame: 4.04·10⁶× behind `rho-neg` and
  5.17·10⁵× behind `proj-a` at w = 0.1; 2.54·10⁷× and 3.23·10⁶× at w = 1;
  17.7× and 2.27× at w = 0.
- 13 of 14 predictions held (C1, C2, P1–P8, P10–P12 held; P9 failed). All 84
  executions verified; every chunk audited with a nonce-sampled replay, 0
  problems.

## 1. Code change (commit `2993713af8`)

`ecbench: prime-interval factor base (every point with x in [L, L + 2^T)) for pkm-semaev bits frames`,
3 files, +339/−2:

| file | change |
|:--|:--|
| `src/cryptanalysis/ic_framework/pkm_semaev.rs` | `PrimeIntervalBase<'i> { instance }` with `impl FactorBaseBuilder<PrimeCurve>`; shares its point list with `bits_factor_base`; charges `abscissae_scanned = 2^t`, one Legendre symbol per scanned x, `sqrt_solves` = carriers, index build through `count_index_build`. `PkmSemaevOracle::from_spec` gains the arm `"prime-interval" => new_bits(m, fb.t, fb.lo)`; an oracle `bits=`/`lo=` that differs from the base is an error, an equal one is accepted. Unit tests U1–U6 (five `#[test]`s; U3 and U4 share one). |
| `src/cryptanalysis/ecbench/methods.rs` | registered in `prime_base_for`, in the factor-base dump match and in the `ic.pipeline` `factor_base` help string (+8/−1). |
| `src/cryptanalysis/ic_boundary.rs` | `count_index_build` made `pub(crate)` (1 line). |

Gate results (`build/`, each log ends `RC=<code>`):

| gate | result | evidence |
|:--|:--|:--|
| `cargo build --release --bin ecbench` | RC=0 | `build/build.log` (13 m 53 s) |
| `rustfmt --edition 2021 --check` on the changed files | RC=0 | `build/rustfmt.log` |
| `cargo clippy --release -- -D warnings` | RC=0 | `build/clippy.log` (no lint in the changed files; the only diagnostic is the pre-existing unknown-lint note from `Cargo.toml`) |
| `cargo test --release --lib prime_interval` | RC=0, **5 passed, 0 failed** (`prime_interval_u1_frames_equal_bits_factor_base`, `_u2_lo0_equals_prime_abscissa_below`, `_u3_u4_explicit_x_and_charging`, `_u5_rejections`, `_u6_pipeline_solves`) | `build/test.log` |
| binary identity | `82bcad823fffcc7067eee11a38905ced4c1244b998741289155101a6bba19031 /home/user/ctarget-t11/release/ecbench` | `build/ecbench.sha256` = `data/binary.sha256` = `binary_sha256` in all six chunk `session.json`; re-hashed 2026-11-04 08:12Z, equal |
| source identity | `pkm_semaev.rs cdb05e78…`, `methods.rs ec8fe422…`, `ic_boundary.rs a811558a…` | `build/source.sha256` = `data/source.sha256` |

**Instrument-provenance erratum (validator R8, `validator-report.json`).** The
measuring binary was built from `505ecbb154` + the T11 patch. Commit
`2993713af8` landed on a tree whose `ic_boundary.rs` (`7b0ac308…`) carries
+153/−6 upstream lines that are not in the build's `ic_boundary.rs`
(`a811558a…`); the three-file T11 diff is the same. So no single commit
reproduces the measuring binary byte for byte; the C1 logs certify the lead's
build tree (`build/source.sha256`), and the validator independently re-ran
rustfmt and the five unit tests on `2993713af8` (RC=0, 5 passed).

## 2. What was run (Addendum C)

- Spec `ecbench/spec.json`, sha256
  `630f7722270399ce05ffcd585ef12ecd48f98e7f7401ace4c88d235b6becdbc2`; the t11
  binary plans it as `spec ECS1hd4f9ace982f5 … 84 executions`, no warning
  (`data/attempt1_refused_busy/ecbench/plan.txt`, `data/run.log`; the
  validator reproduced it byte for byte with the t11 and the shared binary).
- `scripts/chunked_run.py` split the frozen spec into six one-target chunks
  (`targets_per_curve` 6 → 1, `target_seed` 3110311101 + c, measurement seed
  3110311102 + c, description suffixed `[chunk c of 6, Addendum C]`; a
  spec-to-chunk diff shows no other key changed). All 7 arms × 2 rounds,
  `alternate_measured`, 0 warm-up, 3600 s timeout, L0, `--allow-busy`.
- `data/chunks/c00 … c05`: six complete sessions, 14 records each, 84 in all,
  status counts `{"verified": 84}`, driver log 05:02Z → 08:10Z
  (`data/chunks/driver.log`), no chunk interrupted
  (`data/chunks/interrupted/` is empty).

| chunk | session | spec id | workload | records |
|:--|:--|:--|:--|--:|
| c00 | ECBS1h5833502582a1 | ECS1h2000df4b82f8 | Wc57904853fc3 | 14/14 verified |
| c01 | ECBS1hc09a6d59ae44 | ECS1hd43256240ed2 | W19ebde08c4de | 14/14 verified |
| c02 | ECBS1h79af8bbb86f8 | ECS1h8cbd4c092763 | W80349ad23ee7 | 14/14 verified |
| c03 | ECBS1ha5c32877bafa | ECS1h04ab932ac7f5 | W0c95e5dd8255 | 14/14 verified |
| c04 | ECBS1h97007f83d88b | ECS1hedda76907cc8 | W6eeb087a8db8 | 14/14 verified |
| c05 | ECBS1hdadeecd4a6c9 | ECS1h869ab373d423 | We659ae7efffe | 14/14 verified |

Analysis commands (this file's numbers):

```
ecbench band --dir data/chunks/c00 … c05 --ranks --json data/band_pooled.json > data/band_pooled.txt
python3 -I scripts/pooled_compare.py A B w data/chunks/c00 … c05  >> data/pooled_compare.jsonl   (8 pairs × w ∈ {0, 0.1, 1})
ecbench verify --dir data/chunks/cNN --replay 1 --out data/verify/cNN.json  (each behind flock /tmp/crypto-bench.lock)
```

Positive control of `pooled_compare.py` on this data: on chunk c00 alone it
gives 0.49095 for L976503 ÷ lo0 at w = 0.1, and `ecbench compare --dir
data/chunks/c00` prints 0.4909 (band w = 0: 0.5176 both). Addendum C's T9
control (6 decimals) stands.

## 3. Band (pooled, `data/band_pooled.txt`; S = mean over 12 runs, √r = 362.2195)

| arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) |
|:--|--:|--:|--:|--:|
| pkm-int-t5-g3-L541434 | 12/12 | 28.226 | 7001937.597 | 70019121.945 |
| pkm-int-t5-g3-L976503 | 12/12 | 25.941 | 6333651.186 | 63336278.397 |
| pkm-int-t5-g3-lo0 | 12/12 | 56.628 | 14491575.449 | 144915244.835 |
| pkm-iso-t6-g4 | 12/12 | 29.170 | 1618179.641 | 16181533.886 |
| pkm-iso-t6-g4-s1093 | 12/12 | 22.633 | 821161.110 | 8211407.400 |
| proj-a | 12/12 | 11.437 | 12.253 | 19.590 |
| rho-neg | 12/12 | 1.466 | 1.569 | 2.491 |

Band verdicts (`--ranks`, order holding at every weight): `rho-neg <` every
arm; `proj-a <` every PKM arm; `pkm-iso-t6-g4-s1093 <` all three interval
arms and `< pkm-iso-t6-g4`; `pkm-int-t5-g3-L976503 < pkm-int-t5-g3-L541434 <
pkm-int-t5-g3-lo0`; `pkm-iso-t6-g4 < pkm-int-t5-g3-lo0`; L976503 and L541434
against `pkm-iso-t6-g4` are **unresolved inside the band** (they win at w = 0,
lose at w ≥ 0.1).

## 4. Frozen paired compares (`data/pooled_compare.jsonl`; ratio B ÷ A, 95 % ops interval, K = 6 targets, 12 pairs)

| B ÷ A | w = 0 | w = 0.1 | w = 1 | B wins (targets) |
|:--|:--|:--|:--|:--|
| L976503 ÷ lo0 | 0.458085 [0.314212, 0.667836] | 0.437057 [0.299979, 0.636775] | 0.437057 [0.299979, 0.636775] | 6/6, 6/6, 6/6 |
| L541434 ÷ lo0 | 0.498436 [0.337251, 0.736657] | 0.483173 [0.328572, 0.710518] | 0.483173 [0.328572, 0.710518] | 6/6, 6/6, 6/6 |
| L541434 ÷ L976503 | 1.08809 [0.927254, 1.27681] | 1.10551 [0.918283, 1.33092] | 1.10551 [0.918283, 1.33092] | 2/6, 2/6, 2/6 |
| L976503 ÷ iso-s1093 | 1.14614 [0.878171, 1.49588] | 7.71304 [5.0571, 11.7639] | 7.71321 [5.05719, 11.7641] | 2/6, 0/6, 0/6 |
| L976503 ÷ iso-s0 | 0.889297 [0.616865, 1.28205] | 3.91406 [2.53584, 6.04133] | 3.91411 [2.53587, 6.04141] | 4/6, 0/6, 0/6 |
| iso-s1093 ÷ iso-s0 | 0.775905 [0.466933, 1.28932] | 0.50746 [0.248682, 1.03552] | 0.507455 [0.248679, 1.03552] | 4/6, 5/6, 5/6 |
| L976503 ÷ proj-a | 2.26805 [1.9264, 2.67029] | 516921 [428537, 623533] | 3.23304·10⁶ [2.60072·10⁶, 4.01909·10⁶] | 0/6, 0/6, 0/6 |
| L976503 ÷ rho-neg | 17.6925 [11.9782, 26.1327] | 4.0375·10⁶ [2.67526·10⁶, 6.09338·10⁶] | 2.54224·10⁷ [1.61909·10⁷, 3.99175·10⁷] | 0/6, 0/6, 0/6 |

The w = 0.1 and w = 1 ratios of PKM-against-PKM pairs coincide to 5–6 digits
because the unpriced F4 multiply-adds (10⁹–10¹¹ per run) dwarf the charged
GAE (10³–10⁴ per run) at both weights.

## 5. Predictions (decided strictly by the frozen rules)

| id | rule | evidence | verdict |
|:--|:--|:--|:--|
| C1 | build, rustfmt, clippy `-D warnings`, `prime_interval` tests all pass | all four logs RC=0; 5 passed 0 failed (§1) | **held** (on the lead's build tree; see erratum R8) |
| C2 | t11 binary plans the frozen spec to `ECS1hd4f9ace982f5`, 84 executions, no warning | plan.txt of attempts 1–3 and the validator's run: exact | **held** |
| P1 | U1–U4: 11/24/16 columns, frozen abscissa lists, same points as `bits_factor_base`, `prime-abscissa:below=32`, `explicit-x`; `abscissae_scanned = 32` | tests U1–U4 pass; validator recomputed the carriers independently | **held** |
| P2 | 12/12 per arm, 84/84 verified, `ecbench verify` 0 problems | band: every arm 12/12; status counts `{"verified": 84}`; six receipts `ok: true, problems: []`, every replay `identical` (§6). Addendum C equivalent of the single-session 12-replay audit: one nonce-sampled replay per chunk session, six in all | **held** |
| P3 | S(L976503) ÷ S(lo0) ≤ 0.65 at w = 0.1 and w = 1 | 0.437057 and 0.437057 | **held** |
| P4 | upper 95 % end of that ratio < 1 at w = 0.1 and w = 1 | 0.636775 and 0.636775 | **held** |
| P5 | S(L976503) ÷ S(lo0) ≤ 0.75 at w = 0 | 0.458085 | **held** |
| P6 | S(L541434) ÷ S(L976503) ∈ [0.70, 1.30] at w = 0.1 | 1.10551 (interval [0.918, 1.331] includes 1, reported not predicted) | **held** |
| P7 | S(L541434) ÷ S(lo0) ≤ 0.65 at w = 0.1 and w = 1 | 0.483173 and 0.483173 | **held** |
| P8 | S(L976503) ÷ S(iso-s1093) ≥ 2.0 at w = 0.1 and w = 1, and band orders s1093 below L976503 at both | 7.71304 and 7.71321; band S 821161 < 6333651 (w = 0.1), 8211407 < 63336278 (w = 1), verdict `pkm-iso-t6-g4-s1093 < pkm-int-t5-g3-L976503` | **held** |
| P9 | S(L976503) ÷ S(iso-s1093) < 1.0 at w = 0 | **1.14614** [0.878, 1.496]; L976503 won 2 of 6 targets | **failed** |
| P10 | S(iso-s1093) ÷ S(iso-s0) ≤ 0.75 at w = 0.1 | 0.50746 (T5: 0.641) | **held** |
| P11 | all five PKM arms behind `proj-a` and `rho-neg` at w = 0.1 and w = 1; the three interval arms behind `proj-a` at w = 0 | band verdicts `proj-a < ` and `rho-neg < ` each PKM arm across the band; at w = 0 interval S 25.9/28.2/56.6 > proj-a 11.437 | **held** |
| P12 | per-arm `abscissae_scanned` totals 384 and factor-base `sqrt_solves` 12 × cols for each interval arm, in the compare arm lines or `band.json` | Records reading (neither `ecbench compare` arm lines nor `band.json` print `abscissae_scanned`; Addendum C has no P12 provision): the session records (`records.jsonl`, phase `factor_base.native`), which the pooled compare reads. lo0: scanned 384, sqrt 132 = 12 × 11; L976503: 384, 288 = 12 × 24; L541434: 384, 192 = 12 × 16; Legendre 384 each. `ecbench compare` arm lines print them as `legendre_symbols_uncharged`/`sqrt_solves_uncharged` (c00: 64 and 48 for L976503); `band_pooled.json` carries only the per-arm sums | **held (records reading)** |

Score: 13 held, 1 failed (P9), 0 not applicable. P9's model ("the tower build
is a fixed cost that dominates at w = 0") is contradicted: at w = 0 the isogeny
seed-1093 arm already has the lowest charged S of the five PKM arms (22.633).

## 6. Replay audit (`data/verify/`)

`ecbench verify --dir data/chunks/cNN --replay 1 --out data/verify/cNN.json`,
auditor binary `82bcad82…9031`, auditor seed from OS entropy (in each receipt).

| chunk | replayed seq | arm | algorithm seed | result | receipt |
|:--|--:|:--|--:|:--|:--|
| c00 | 13 | `rho-neg` | 13444998791494616123 | identical | ok: True, problems: 0 |
| c01 | 0 | `rho-neg` | 3779343707889731215 | identical | ok: True, problems: 0 |
| c02 | 10 | `pkm-iso-t6-g4-s1093` | 6199571022284097130 | identical | ok: True, problems: 0 |
| c03 | 7 | `pkm-int-t5-g3-L541434` | 13516223990770497100 | identical | ok: True, problems: 0 |
| c04 | 10 | `pkm-iso-t6-g4-s1093` | 8349483065760404266 | identical | ok: True, problems: 0 |
| c05 | 11 | `pkm-iso-t6-g4` | 1746814522755126439 | identical | ok: True, problems: 0 |

The six producer replays cover one interval-arm run (`L541434`, c03) and neither headline arm. The validator replayed c03 seq 4 (`lo0`), seq 5 (`L976503`) and seq 10 (`iso-s1093`) independently: all identical, 0 problems (`validator-report.json`).

## 7. The key comparison

| question | answer (pooled, this session) |
|:--|:--|
| argmax J (`L976503`) against `lo = 0` | 0.437 [0.300, 0.637] at w ≥ 0.1, 0.458 at w = 0; 6/6 targets. Model 0.463; the measured gain is at the model, not below it (T5's 1.07 inflation did not appear). |
| argmax J ÷ cols (`L541434`) against `lo = 0` | 0.483 [0.329, 0.711]; 6/6 targets. Model 0.443. |
| `L541434` against `L976503` | 1.106 [0.918, 1.331]; `L976503` better on 4 of 6 targets; the model (0.957) predicted the opposite sign, inside the stated no-difference window. |
| `L976503` against isogeny seed 1093 | 7.71× behind at w ≥ 0.1 (model 6.0); even at w = 0 (1.15×). The incumbent algebraic arm keeps the native-priced track. |
| `L976503` against isogeny seed 0 | 3.91× behind at w ≥ 0.1 (model 3.57); 0.89 [0.62, 1.28] at w = 0, unresolved. |
| `proj-a` distance | 2.27× (w = 0), 5.17·10⁵× (w = 0.1), 3.23·10⁶× (w = 1) |
| `rho-neg` distance | 17.7× (w = 0), 4.04·10⁶× (w = 0.1), 2.54·10⁷× (w = 1) |

## 8. Costs

- Measured S per solve (band, amortised, the frame search not charged): table
  in §3. Charged GAE per run for `L976503`: 5016–13217; unpriced F4 work
  1.04·10¹⁰–3.36·10¹⁰ multiply-adds per run (`records.jsonl`).
- **Frame search** (`research/curve25519_frame_search_20261103/analysis/summary.json`,
  cell k = 20, t = 5): 2,000 translation frames, `search_legendre` 64,000,
  `search_group_ops` 814,513.54. Charged to a single target it adds
  814,514 ÷ √r = 2,249 to S at every weight, plus 17.7 (w = 0.1) or 177
  (w = 1) for the Legendre symbols. Charged S of `L976503`: **2,275** (w = 0),
  6,335,918 (w = 0.1), 63,338,704 (w = 1); of `L541434`: 2,277 (w = 0). So at
  w = 0 the search costs 40× the lo0 arm's whole solve (56.6) and the selected
  frames lose; at w ≥ 0.1 the ratio against lo0 is unchanged to three digits
  (0.4371 → 0.4372 at w = 0.1, 0.4371 → 0.4371 at w = 1). The `lo0` arm needs no search.
- Wall time (diagnostic only, L0, shared 4-core host under 3–5 other heavy
  processes): chunks took 21–57 min each, 3 h 08 min in all.
- Earlier attempts consumed about 1 h 55 min (attempt 1, refused at the end of
  the lock wait), 17 and 13 executions (attempts 2 and 3) of wall time; none
  of their counts is used.

## 9. Deviations and disclosures

1. **`--allow-busy`.** Attempt 1 ran the frozen `run.sh` verbatim; after
   queuing on `/tmp/crypto-bench.lock` from 22:39Z to 00:34Z the busy preflight
   refused it (`host is not quiet (PSI cpu some avg10 55.66 > 5.00)`), rc=2,
   no records (`data/attempt1_refused_busy/`). Every later launch added
   `--allow-busy` (`data/run_allowbusy.sh.used`, `data/run_s2_allowbusy.sh.used`,
   `scripts/chunked_run.py`). S is a counted unit; no wall-time claim is made;
   every run is L0 as the spec permits.
2. **Chunking (Addendum C).** The frozen single 84-execution session could not
   survive the container restarts (about 01:30Z and 02:22Z), so the spec ran
   as six complete one-target sessions with derived seeds; the band and the
   paired compares are pooled by `pooled_compare.py` (positive control §2).
   The per-chunk spec ids differ from `ECS1hd4f9ace982f5` because the chunk
   specs differ in `targets_per_curve`, `target_seed`, `measurement.seed` and
   `description`; C2 was decided on the frozen spec.
3. **Three earlier attempts, none cited:** attempt 1 (refused, above);
   attempt 2, `data/interrupted_attempt2/s1`, 17 of 84 records, session
   ECBS1h4b7ae23e07d3, killed by the container restart; attempt 3,
   `data/interrupted_attempt3/s2`, 13 of 84, ECBS1h7165cac69326, killed by the
   next restart. Both keep `session.interrupted.json`. The validator noted
   that the first 10 s2 records equal attempt 2's records 0–9 in every count
   field (same binary, same seeds): the counts are deterministic. Chunk c00 is
   the same workload as the first target of attempts 2 and 3, and its round-0
   counts (seq 0–6) equal attempt 2's, which the producer had seen before
   Addendum C was frozen (validator erratum E7). Nothing was retuned: the
   frames, arms and thresholds are those frozen at `505ecbb154`.
4. **`SHA256SUMS` and `run.log`.** The frozen `run.sh` writes `SHA256SUMS`
   with `find ! -name run.log`, which (as the validator noted) also skips
   `ecbench/run.log`. This README's `data/SHA256SUMS` covers **every** file
   under `data/` except `SHA256SUMS` itself, the earlier attempts' logs
   included.
5. **Pricing of the interval scan.** The protocol says the Legendre symbols
   and square roots of the scan are "charged inside the harness, nothing moved
   to uncharged". They are counted inside the harness (P12), but ecbench prices
   the factor-base phase's `legendre_symbols` and `sqrt_solves` of **every**
   abscissa base (`prime-abscissa`, `prime-projected-abscissa`,
   `prime-isogeny`, now `prime-interval`) as `*_uncharged` native work, i.e.
   at 0 / 0.1 / 1 GAE each in the band, not inside `total_gae`. `prime-interval`
   is therefore charged exactly like the other abscissa bases, which is what
   the protocol's first clause asked; the second clause overstated it. For
   the interval arms this is 32 symbols and 11–24 roots per run (≤ 56 native
   ops against 10¹⁰ F4 multiply-adds), so no ratio changes at any weight.
6. **Instrument provenance** (erratum R8, §1): the measuring binary's tree is
   `505ecbb154` + patch, not `2993713af8`.
7. **Host.** Shared 4-core container; during the chunks and the verifies 2–4
   other ecbench sessions (wave-4 leads) were running. The counts are
   deterministic (every replay identical); load affected wall time only.
8. **Verify serialisation.** `ecbench verify` takes no lock of its own; each
   per-chunk verify was wrapped in `flock -x /tmp/crypto-bench.lock`
   (`data/verify/run_verify.sh`, `verify_driver.log`), one chunk per hold.

## 10. Scope and distance remaining

One curve (k = 20, r = 131203), one width (t = 5), one guess size (G = 3), six
targets, two rounds. The result is a constant factor on a bits frame choice;
no exponent changes and nothing extrapolates to 255 bits. The selected frame
is 7.7× behind the incumbent isogeny arm on the native-priced track and
4.0·10⁶× (w = 0.1) to 2.5·10⁷× (w = 1) behind `rho-neg`. **Rho remains ahead.**

## Files

- `PROTOCOL.md` — frozen protocol + Addendum C. `validator-report.json` —
  the independent pre-data validation (refused the moving tree; code review,
  C1/C2/P1 recomputed, erratum R8).
- `build/` — gate logs and identities. `ecbench/spec.json` — frozen spec.
  `selection/frames.json` — frozen frames. `scripts/` — `run.sh` (frozen),
  `chunked_run.py`, `pooled_compare.py`, `select_frames.py`.
- `data/chunks/c00…c05` — the six sessions; `data/chunks/specs/`, `driver.log`,
  `cNN.log`. `data/band_pooled.{txt,json}`, `data/pooled_compare.jsonl`,
  `data/verify/cNN.{json,txt}`. `data/attempt1_refused_busy/`,
  `data/interrupted_attempt2/`, `data/interrupted_attempt3/` — not cited.
  `data/SHA256SUMS` — every data file.

## Validation (2026-11-04)

`validator-report.json`: **pass_with_errata**, disposition validated (independent recompute of
§3 and §4, 0 mismatches; 84/84 certificates [d]G = T; three extra replays identical). Errata
applied to this README: E1 (replay table in §6), E2 (tally 13 of 14), E3 (`data/SHA256SUMS`
generated), E4 (§8 four-digit values), E5 (P12 labelled a records reading), E7 (c00 overlap with
the killed attempts disclosed in §9). E6 (tree binding) is handled at commit: the data tree is
committed with the PR. Pre-data erratum R8 stands: the measuring binary is not reproducible from a
single commit.
