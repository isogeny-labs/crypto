# VALIDATOR.md: independent validation of the counted-S v2 results

Protocol `crypto-hashed-ecdl-counts-20261031` v2, approval `DEC-20261031-77e2b3`,
results merged in isogeny-labs/crypto PR #3131 (merge `053714ca09e5288c2cca0ab93e22c75b1492b9cc`).
Validated on 2026-11-01 by the Validator role (agents/validator.md), independent of the producer.
Snapshot read: `research/hashed_ecdl_counts_20261031/` at `053714ca`. `origin/main` at
`7fd570bd38` has no change under that directory since the merge
(`git diff 053714ca origin/main -- research/hashed_ecdl_counts_20261031` is empty).

This report validates the receipt. It does not support an ECDLP claim, a speedup or a
promotion, and it does not change research status. No raw artifact was edited.

## 0. Verdict

**Receipt: `passed`** (the recorded counts are admissible as counted-S evidence), **with
one FAIL on a sentence of `results/RESULTS.md`, two INCONCLUSIVE items, and a
`VALIDATION.json` that cannot be issued today.**

- Every number in the RESULTS tables recomputes from `records.jsonl` without `decide.py`
  (max relative difference to `decision.json`: 3.3e-16 over 45 ratios).
- Every execution replays bit-exactly in my own runs: C-10-14 96/96, C-16 32/32, B 288/288 (416 of 416, plain `identical`).
- **FAIL (F1):** "The gap to rho widens roughly 4x per 2 bits" is not what the data show.
  Measured: 5.0x per 2 bits of log2 r (log-log slope 1.17), successive steps 6.3x, 6.3x, 4.0x.
- **INCONCLUSIVE (F2):** the source commit and build of the binaries that produced the
  C-10-14 and B sessions cannot be recovered; the sessions record `git_commit: null`.
  I did not rebuild (no toolchain, root disk full). Not material to the validity of the
  counts (384 of 384 bit-exact replays by rebuilt, not original, binaries), but it blocks a
  clean replay certificate.
- **INCONCLUSIVE (F8):** no source-independent rebuild of the replay binaries was done.
- `VALIDATION.json` is not issuable: the raw records are not in the repository, there is
  no replay certificate or validator report in gate schema, and one `rho_reference` field
  needs a decision (section 9).

## 1. Check table

| # | Check | Result | Key evidence |
|---|---|---|---|
| 1a | `SOURCES.sha256` verifies | PASS | `sha256sum -c SOURCES.sha256`: 9 of 9 OK |
| 1b | protocol hash equals the approval record | PASS | canonical sha256 of `protocol.json` without `freeze` = `8a794febbd17fd388ac286ca2df98329e24c5b6696af870b10a07785a306ca95` = `freeze.protocol_sha256` = hash in `DEC-20261031-77e2b3` |
| 1c | `decide.py` not altered after freeze | PASS | `3b6c69908ad6da040a214b1a4008807dc70d7a5ecfac30e26ae02b44e4a49519` on main, at freeze commit `ab3b148a4`, in `analysis_plan.script.sha256` and in `SOURCES.sha256`; `protocol.json`, 3 specs, `README.md`, `SOURCES.sha256` byte-identical at `ab3b148a4` and `053714ca` |
| 1d | frozen originals unchanged | PASS | `docs/ecbench/specs/tensor-ic-vs-rho*.json`, `research/ecdl_tensor_ic_speed_20261030/protocol.json` identical at `4df337e64d` and `origin/main`; `frozen-source/*` identical to `3ae1ac2bd` |
| 1e | specs differ from originals only in `measurement` | PASS | structured diff: `arms`, `workloads`, seeds equal; `measurement` differs as README states (rounds, warmup, order, L2 to L0) |
| 1f | sessions ran the frozen specs, after approval | PASS | `spec.json` in each session byte-equal to the frozen spec (`4bf13a20`, `1ec5dcb4`, `9bf4e7b1`). Freeze `2026-10-31T06:58:46Z`, DEC `07:40Z`, ledger commit `16:27Z`, sessions started `17:06:40Z` (C-10-14), `17:17:16Z` (B), `20:01:42Z` (C-16) |
| 2a | session files are bound to the receipts | PASS | all 15 file hashes in `results/raw/verify-*.txt` equal `sha256sum` of the S3 files (host, plan, records, session, spec for each session) |
| 2b | full replay C-10-14 and B (validator) | PASS | `ecbench-C verify --replay-all`: 96/96 identical, 0 problems; `ecbench-B verify --replay-all`: 288/288 identical (incl. 110 exhausted rows), 0 problems |
| 2c | replay C-16 (validator) | PASS | all 32 rows re-executed and identical: 24 non-tensor rows plus the 8 tensor-m2 rows (the heavy ones), 0 problems |
| 2d | scalars verified independently | PASS | 306 verified rows: `[d]G == Q` recomputed in plain Python (no ecbench) on 4 curves `y^2=x^3+x+b` (b = 15, 3, 12, 16 over p = 827, 3907, 16187, 65353), `[r]G = O`; 0 failures |
| 2e | A/A determinism | PASS (weak control, see F6) | rho-neg vs rho-neg-aa: 24+8+32 pairs; native-selectors vs -aa: 32 pairs (19 exhausted); status, seed, S, total_gae, counters, phases all identical; 0 `other_status_mismatch`, 0 timeouts |
| 2f | positive control | PASS | B `rho-neg-v2` verified 32 of 32 targets |
| 3 | S ratio tensor-m2 / rho-neg, and ic-mitm ratios, at w = 0, 0.1, 1, without `decide.py` | PASS | section 2; matches RESULTS to rounding (max 2.4%) and `decision.json` to 3.3e-16 |
| 4 | Track B complete vs incomplete cells | PASS | section 4 |
| 5 | gap growth "about 4x per 2 bits" | **FAIL** | section 5 |
| 6 | caveats complete, no overclaim | **FAIL (partial)** | section 7: F3 to F7 |
| 7 | Holm family, P1, P4 follow from `decision.json` | PASS | section 6 |
| 8 | provenance of binaries | INCONCLUSIVE | F2, F8 |

## 2. Recomputation of the S ratios (check 3)

Method (`validator/recompute.py`, shares no code with `decide.py` or ecbench). For a record,
`priced_gae(w) = total_gae - C + w (U + n_C)`, `S = priced_gae / sqrt(r)`, as in the
module comment of `src/cryptanalysis/ecbench/band.rs` and AGENTS.md section 5a, where
`U` is the sum of the counts behind the record's `cost.unpriced` counters and `C, n_C` the
GAE and operation count a pinned calibration put on native work (only `ic-mitm`; table
`docs/ic/calibration.json`, identical blob at `4df337e` and `3ae1ac2`, copied to
`validator/calibration-pinned-4df337e.json`). Ratio `R = sum S(B) / sum S(A)` over the 8
paired targets of a curve, rows with `status == verified` and `warmup == false` only.

Tensor-m2 over rho-neg (w = 0 / 0.1 / 1), 8 pairs on every curve:

| curve | log2 r | recomputed | RESULTS.md |
|---|---|---|---|
| fp10 | 9.685 | 4318 / 5008 / 9543 | 4.3e3 / 5.0e3 / 9.5e3 |
| fp12 | 11.925 | 27039 / 30522 / 49612 | 2.7e4 / 3.1e4 / 5.0e4 |
| fp14 | 13.987 | 169644 / 190460 / 301042 | 1.7e5 / 1.9e5 / 3.0e5 |
| fp16 | 15.988 | 673279 / 756951 / 1206158 | 6.7e5 / 7.6e5 / 1.2e6 |

Tensor-m2 over ic-mitm (w = 0 / 0.1 / 1): fp10 258.9 / 303.9 / 630.2; fp12 2988 / 3505 / 7241;
fp14 21014 / 24349 / 46389; fp16 121098 / 136676 / 222417. All equal the RESULTS column
"vs ic-mitm" to its two significant figures.

ic-mitm over rho-neg, Track B session (RESULTS: "14.8 down to 5.2 x rho"): fp10 14.84 / 14.53 / 12.57;
fp12 9.756 / 9.407 / 7.469; fp14 8.528 / 8.258 / 6.809; fp16 5.194 / 5.166 / 5.017. The "14.8 down
to 5.2" is the w = 0 column; at w = 1 it is 12.6 down to 5.0 (F7).

Order across the band: for each complete cell the mean S ordering is the same at 0, 0.1 and 1
(tensor-m2 above rho-neg and above ic-mitm on every curve). "Distance remaining 3.6 to 6.1
orders of magnitude": log10 4318 = 3.64 (fp10, w=0), log10 1.206e6 = 6.08 (fp16, w=1). Confirmed.

Cross-checks that do use ecbench: all 39 `ecbench compare ... --weight w --resamples 4000 --seed 20261002 --json`
commands in `results/raw/compare-*.json` re-run with the rebuilt binaries give identical JSON
(`validator/rerun_compare.py`); `ecbench band ... --ranks` output (json and txt, both tracks) is
byte-identical to `results/raw/band-{B,C}.*`. The independent linearised-t interval and
p-value for the 15 complete cells reproduce `decision.json` (e.g. C-rho fp10 CI at w=0
[2965, 6289] vs [2964.7, 6289.2]; p = 2.77e-10 both) (`validator/pvals.py`; my t(7) tail is
Gauss-Legendre integration, not the incomplete-beta code of `decide.py`).
`validator/redecide.py` runs the frozen `decide.py` functions on `results/raw/*.json` plus the
sessions and reproduces all 32 cells of `decision.json` (0 field differences) and P1, P4.

## 3. Replay and controls (check 2)

Producer receipts (`results/raw/verify-*.txt`, S3 `v2/receipts/`): C-10-14 96/96, C-16 32/32,
B 288/288 replays `identical`, 0 problems, auditor binaries `5cec72d2...` (C) and `603c6c86...` (B).

Validator runs (cwd any; binaries restored from S3 `v2/bin/`):

```
taskset -c 0 ecbench-C verify --dir C-10-14 --replay-all --out C-10-14.audit.json
    -> audit OK: 96 records, 96 verified, 96 replays reproduced, 0 problems (receipt e164862d91dd...)
taskset -c 1 ecbench-B verify --dir B --replay-all --out B.audit.json
    -> audit OK: 288 records, 178 verified, 288 replays reproduced, 0 problems (receipt 5856bf359ff2...)
ecbench-C verify --dir C-16 --replay-seq <24 non-tensor seqs>
    -> audit OK: 32 records, 32 verified, 24 replays reproduced, 0 problems (receipt 98a7b1c60efc...)
ecbench-C verify --dir C-16 --replay-seq 2,6,10,14,18,22,26,30   (the 8 tensor-m2 rows, ~12 min each)
    -> run as two processes (seqs 2,6,10,14 on CPU 2; 18,22,26,30 on CPU 3), 41 to 46 min:
       audit OK: 32 records, 32 verified, 4 + 4 replays reproduced, 0 problems (receipts f6eca561921b..., 096134ee3d9e...)
```

The sample for C-16 is deliberately larger than a nonce draw: all 8 tensor rows (the only
expensive ones, 646 to 794 s each in the session) plus the other 24 rows, i.e. a full replay
split over CPUs. What `compare_replay` (audit.rs) requires to be identical: recovered scalar,
exhaustion flag, `total_gae` and per-phase gae (bit-exact), phase counts and native maps,
counters, factor base, field ops, unpriced list, determinism flag, automorphisms. All
replays report plain `identical`; none needed a legacy-accounting profile.

Census (all three sessions, from `records.jsonl`): 416 records; 306 verified, 110 exhausted
(all in B), 0 timeout, 0 wrong_answer, 0 error; all `L0`, 0 warm-ups, round 0 only, all
`deterministic: true`, no `nondeterminism` entry. 96, 32 and 288 executions planned and
recorded (`executions_planned == records_written == last_seq + 1`).

Null object: the study compares costs of two methods on the same targets; the protocol
declares no negative control (`not_applicable_reason` given) and none is needed for a
cost ratio. The A/A arms share the algorithm seed with their partner, so identical S is
guaranteed by construction for a deterministic function; the A/A control checks determinism
and the harness, not seed-to-seed noise (F6). The one quantity that the protocol says
should be flat and is not is rho's S (P3, section 5).

## 4. Track B cells (check 4)

Rows per (backend, curve), verified / exhausted of 8 (`validator/inv.py`; matches
`decision.json` and session `status_counts` 178 verified, 110 exhausted):

| arm | fp10 | fp12 | fp14 | fp16 |
|---|---|---|---|---|
| native-source | 0 / 8 | 0 / 8 | 0 / 8 | 0 / 8 |
| native-width | 0 / 8 | 0 / 8 | 0 / 8 | 0 / 8 |
| native-selectors | 8 / 0 | 5 / 3 | 0 / 8 | 0 / 8 |
| native-partial | 8 / 0 | 8 / 0 | 7 / 1 | 1 / 7 |

Complete cells (all 8 + 8 rows verified): native-selectors fp10, native-partial fp10,
native-partial fp12. Exactly the three stated in RESULTS. Exhausted rows were not dropped
into a ratio and not counted as success: `decide.py` marks every other cell
`incomplete_unknown` with p = 1.

Ratios of the stated cells, recomputed (w = 0 / 0.1 / 1):

| cell | recomputed | RESULTS.md |
|---|---|---|
| native-selectors fp10 / rho-neg | 1306.5 / 1450.5 / 2371.5 | 1.3e3 / 1.5e3 / 2.4e3 |
| native-partial fp10 / rho-neg | 495.6 / 550.5 / 901.7 | 5e2 / 5.5e2 / 9e2 |
| native-partial fp12 / rho-neg | 1751.8 / 1911.9 / 2801.5 | 1.8e3 / 1.9e3 / 2.8e3 |

All exhausted statuses are budget exhaustion inside the method (replayed identically),
not wall-clock timeouts: maximum solve wall of an exhausted row 6.6 s against the 30 s limit.
In this run the "complete cells depend on the host" caveat therefore did not bite; the set is
a deterministic function of the code.

Descriptive verified-only ratios in `decision.json` for incomplete cells (selectors fp12, 5
pairs: 8003 / 8693 / 12455; partial fp14, 7 pairs: 7976 / 8608 / 12009; partial fp16, 1 pair:
613 / 730 / 1328) recompute exactly. They are conditioned on the cheap verified targets, so
they are not unbiased estimates of the cell ratio. RESULTS.md claims no ratio for them.

## 5. Gap growth (check 5): FAIL as stated

RESULTS.md: "The gap to rho widens roughly 4x per 2 bits of group size (exponential toy, as
predicted by H1)."

OLS of `log2 R` on the actual `log2 r` (9.685, 11.925, 13.987, 15.988), four points, df 2;
bootstrap = resample the 8 targets within each size, 4000 draws, seed 20261101
(`validator/slope.py`). Descriptive; the frozen `decide.py` computes none of this.

| quantity | slope | se | 95% t(2) | 95% bootstrap |
|---|---|---|---|---|
| ratio tensor/rho, w = 0 | 1.169 | 0.039 | [1.00, 1.34] | [1.096, 1.242] |
| ratio tensor/rho, w = 0.1 | 1.162 | 0.038 | [1.00, 1.32] | [1.088, 1.242] |
| ratio tensor/rho, w = 1 | 1.123 | 0.033 | [0.98, 1.27] | [1.037, 1.211] |
| P2: tensor-m2 median S | 1.037 | 0.027 | [0.92, 1.16] | [0.968, 1.093] |
| P3: rho-neg median total GAE | 0.373 | 0.024 | [0.27, 0.47] | [0.284, 0.425] |
| rho-neg median S | -0.127 | 0.024 | [-0.23, -0.03] | [-0.215, -0.076] |

- Growth factor of the ratio per 2 bits of log2 r: 4^1.169 = **5.06x** (w = 0), 4.74x (w = 1).
  Successive steps at w = 0: 6.26x (2.24 bits), 6.27x (2.06 bits), 3.97x (2.00 bits); local
  slopes 1.18, 1.29, 0.99. "Roughly 4x" holds for the last step only; a slope of 1.0 lies below
  the bootstrap interval at all three weights and at the lower edge of the t(2) interval.
- The "4x per 2 bits, as predicted by H1" is true of the tensor arm's own S (slope 1.04,
  inside the pre-registered P2 window 0.7 to 1.3; local slopes 1.14, 1.00, 0.97). The ratio
  grows faster because rho's S is not flat: median 3.12, 2.68, 1.95, 1.87 (slope -0.13), i.e.
  rho's total-GAE slope is 0.37 (P3 window 0.35 to 0.65: point estimate inside, bootstrap interval
  reaches 0.28). Fixed overheads in rho's S at small r (setup, verification) inflate it at 10 bits.
- Neither P2 nor P3 is emitted by `decide.py` and RESULTS.md says so; the protocol's
  `analysis_plan` and claim C4 call for medians and log-log slopes with target-resampled
  intervals. They were not reported; the sentence above stands in for them. The measured
  growth is 26% larger than 4x on average (57% larger on each of the first two steps).
- The qualitative statements ("gap widens", "growing", ordered 10 < 12 < 14 < 16 at all three
  weights, which is what P1 needs) are correct.

Suggested wording: "The gap to rho (tensor-m2 S over rho-neg S) grows by 6.3x, 6.3x and 4.0x
over the three steps (log-log slope 1.17, bootstrap 95% 1.10 to 1.24); the tensor arm's own
S grows with slope 1.04 (H1: 1), and rho's S falls from 3.1 to 1.9 over the range."

## 6. Holm family, P1, P4, and the gate (checks 6 and 7)

- Family: 8 comparisons (C-rho, C-ic, B-rho-{source, selectors, width, partial, ic}, B-source-selectors)
  times 4 curves = 32 cells in `decision.json`, equal to `tests_in_family` and to the protocol.
  Holm levels in `decision.json` are `0.05/(32 - k + 1)` for k = 1..32.
- 15 cells are complete and ranked: C-rho 4, C-ic 4, B-rho-selectors fp10, B-rho-partial fp10 and
  fp12, B-rho-ic 4. Their p-values (max over the three weights) recompute independently
  (section 2), largest 3.96e-5 (B-rho-ic fp16) against the strictest Holm level 0.05/32 = 1.56e-3.
  Holm therefore equals Bonferroni here and no classification depends on the step-down. The 17
  other cells have p = 1 by rule and stay `incomplete_unknown`.
- P1 (decisive): all four C-rho cells supported, ratio strictly increasing 10 to 16 bits at all
  three weights (4318 < 27039 < 169644 < 673279 at w = 0, same order at 0.1 and 1), interval lower
  ends 2965, 19844, 139834, 524571 (w = 0). Follows from `decision.json`.
- P4: judged on exactly the 3 complete native cells, all supported. Follows from `decision.json`.
  Scope: 3 of 16 native (backend, curve) cells; native-source and native-width contribute none.
- C-ic (tensor vs ic-mitm) was expected to be underpowered and is not: all 4 cells are ranked,
  tensor-m2 worse by 259x to 2.2e5x. No cell shows an arm cheaper than rho or than ic-mitm.
- Power: `sample_size.power.passes` is false as stated; unchanged.

## 7. Discrepancies and overclaims (check 6)

| id | finding | severity |
|---|---|---|
| F1 | "roughly 4x per 2 bits" is 5.0x measured (slope 1.17); section 5 | FAIL, wording; qualitative claim stands |
| F2 | `session.json`/`host.json` of all three sessions have `git_commit: null`, `git_dirty: null`, empty `build_env`. The C-10-14 binary (`679cdc2f`, `/dev/shm/target-main/release/ecbench`) and the B binary (`a133714d`, `/dev/shm/target-b/release/ecbench`, glibc 2.39, rustc 1.97.0) no longer exist. The replay binaries are rebuilds: C `5cec72d2` (rustc 1.97.0, source `4df337e64d`, ancestor of main), B `603c6c86` (rustc **1.99.1**, **musl** target, **`--no-default-features`**, source `3ae1ac2bd`). RESULTS.md says only "not bit-identical" | INCONCLUSIVE for provenance; see below |
| F3 | The S3 prefix cited in RESULTS.md (`.../v2/`) holds only session C-16, binaries, receipts and decision output. Sessions C-10-14 and B are under `.../20261031/snapshot-1730/sessions/`. The raw `records.jsonl` files are not in git at all; only their hashes (in `results/raw/verify-*.txt`) are | process gap |
| F4 | The mislabelled `descriptive_note` ("includes exhausted pairs at the S they spent") is still in `decision.json` for 17 incomplete cells; the figures are verified pairs only (e.g. 5 of 8). DECISION.md required the report to footnote this; RESULTS.md does not | minor; affects no ranking |
| F5 | Pre-registered descriptive outputs not delivered: medians and log-log slopes (C4, P2, P3), census graphs, `experiment.export/v1`, `reports/` directory (results are in `results/`). `P2/P3 are not emitted` is stated, but C4 is a protocol claim | protocol deviation, documented |
| F6 | "A/A determinism" is guaranteed by construction (shared algorithm seed, deterministic counting); it cannot detect seed-to-seed spread, which at one round and 8 targets is the only source of uncertainty in the linearised intervals. RESULTS.md does not say so, nor that there is one round, 8 targets per cell, `power.passes = false`, L0 isolation, and that C-16 ran on another host class (`ECBENV3hac4e...` vs `ECBENV3h13e3...`; counted S is unaffected) | omission |
| F7 | "ic-mitm ... 14.8 down to 5.2 x rho" is the w = 0 column only (12.6 to 5.0 at w = 1). "P1 and P4 supported_within_scope" does not repeat that P4 rests on 3 of 16 native cells. "Verdict: no tested arm beats rho at any size" reads as a universal statement: for native-source and native-width there is no verified solve at all (64 of 64 exhausted), and their exhausted rows spent only 1.0x to 5.9x rho's S at w = 0 (one fp16 row 0.99x), because nearly all their work is unpriced; the lower bound only separates from rho at w = 1 (3.7e3x and up). The supportable sentence is "no complete cell shows an arm cheaper than rho or than ic-mitm at the four tested sizes" | overclaim in wording; the underlying data are correct |
| F8 | The replay and the producer's audits used the same producer-supplied binaries; I did not rebuild from source | INCONCLUSIVE |

Does F2 matter for validity? Not for the counts. Counted S is a deterministic function of
(workload, algorithm seed, code). A build that differs in toolchain, libc, features and path, and
reproduces `recovered`, `total_gae` (bit-exact), every phase count, counter, native map and the
exhaustion flag on 96 + 288 recorded executions of the original sessions, shows the recorded
counts are properties of the algorithm at the named commits and not of one binary. C-16 was run
by the rebuilt C binary itself (record `binary_sha256 == 5cec72d2...`), so it is unaffected;
that same binary also reproduces C-10-14, so the three C-session arms are the same code on the
executed workloads. What F2 does cost: (a) the binary-to-commit chain of custody for the original
sessions (the README's "built from 4df337e" and "3ae1ac2bd" are claims; replay equality is
consistent with them but does not prove them); (b) a replay certificate must name the rebuilt
binaries, not the session binaries; (c) `3ae1ac2bd` is reachable only from branch
`codex/hashed-ecdl-ecbench-e2e` (3 ahead, 2180 behind main), so B's source can disappear with
the branch; (d) `pipeline.sh` (the only record of the C-16 command and the B build flags) is in
S3, not in git. A rebuild with `--remap-path-prefix` and the recorded toolchains could close
(a); I could not run it.

## 8. Heuristic-validation and cost-model checks (agents/validator.md)

- Pre-registered prediction: PASS. H1/P1/P2/P3, H2/P4 are in the frozen protocol
  (`2026-10-30`/`2026-10-31`), `sample_status: out_of_sample`, with the one disclosed exposure
  (a 2-target smoke session on 10 and 12 bits seen after P1 and P2 were written).
- Sample integrity: PASS. 8 targets per curve, target seeds 20261030 (C) and 2026103001 (B),
  measurement seeds 20261030 and 2026103002, in specs and records; ratios recompute from raw rows.
- Correspondence validity: not applicable (direct sampling of planted/public targets).
- Scale binding: PASS with limitation. Prime curves of 827, 3907, 16187, 65353 elements of field,
  group orders 823, 3889, 16231, 65003 (all prime, cofactor 1, `a = 1`, so only the automorphism
  +-1 exists and negation is the strongest rho symmetry: `automorphisms_available == 2` in
  every rho record, confirmed by the on-curve fit above). Toy sizes only; `lambda <= 64`,
  `root_u = 0`, finite-size fallback; the recursive subexponential kernel was not exercised.
- Cost-unit honesty: PASS with gaps. Unit is `ecbench.gae` (group-addition equivalents), `S = GAE / sqrt(r)`,
  priced at w = 0, 0.1, 1 per unpriced operation on every arm including rho (§5a). Native S and
  rho S are lower bounds (`lower_bound: true`). Memory is not reported in RESULTS.md (records carry
  `max_rss_kib`); the study is counted-S only. ecbench's own verdict string in all 39 `compare`
  outputs labels each ratio "diagnostic only (unpriced work; ratio does not bound full cost)", with
  `quality.score` 0 (L0, 0 wall-admitted pairs); RESULTS.md's "counted units only" caveat is
  consistent with this but does not quote it.
- Cost bookkeeping: each execution runs to a verified scalar (S per solve); there is no
  retry or success-probability factor to apply. Exhausted rows are unknown, not failures.
- Ladder (docs/inventor-protocol.md section 6): no speedup is claimed (all ratios above 1),
  so steps 2 to 4 are not triggered; the recorded result is negative on every complete cell.

## 9. VALIDATION.json (docs/VALIDATION_GATE.md): not issuable yet

The gate decides what may be shown on the IC leaderboard or in `benchmarks/README.md`. This
study found no improvement, so nothing here must be shown; the results may stay in the research
directory without the gate. If a row is to be shown, three artifacts are needed:

| gate item | state | what is missing |
|---|---|---|
| replay certificate (`status` verified/pass, `independent: true`, `scalars_verified: true`, non-empty `runs`, hash-pinned) | `ecbench.audit/v1` receipts exist (producer's in `results/raw/verify-*.txt`; mine listed in section 3) but not in gate schema; replay binaries differ from session binaries (F2) | write `replay-certificate.json`: `runs` = the 416 records, `independent: true` for the validator replays, `scalars_verified: true` (306 `[d]G == Q` checked, section 1 row 2d), naming binaries `5cec72d2...` and `603c6c86...` with rustc and target; ideally rebuild from the named commits first |
| validator report (`status: pass`, `independent: true`, `raw_data` = **repository files**, `headline` with `claimed`/`recomputed`) | this markdown only; **the raw records are not in the repository** (F3) | commit `records.jsonl`, `session.json`, `host.json`, `plan.json`, `spec.json` of the three sessions (about 3.4 MB; hashes in section 3) under the evidence directory; then `validator-report.json` with headline entries from section 2 (claimed = `decision.json` ratios, recomputed = `validator/recompute.py`, `rel_tol` 1e-9; the RESULTS table needs `rel_tol` 0.03 because it is rounded to two figures) |
| rho reference matched to the protocol, with the 5a band | filled in below, with one open field | decide `strongest_available` for Track B, see below |

Candidate `rho_reference` (all values from this report): `kind: negation`; `protocol`: matched
`rho.negation` (arm `rho-neg`) of `protocol.json` v2; `unit`: `ecbench.gae` counted S; `binary_or_process`:
rho and candidate arms of every comparison ran in one session and one binary (C-10-14: `679cdc2f`;
C-16: `5cec72d2`; B: `a133714d`); `matches_protocol: true`; `same_unit: true`;
`same_binary_or_process: true` per comparison; `order_holds_across_band: true` for the 15 complete
cells (`results/raw/band-C.txt`, `band-B.txt`); `gae_band` from those two files.
**Open:** `strongest_available`. Track C has only `rho.negation`. Track B also ran `rho.negation_v2`
(control `rho-neg-v2`), which is cheaper than `rho-neg` on fp14 and fp16: 0.951 and 0.889 of rho-neg's
S at w = 0 (1.014 and 0.901 at w = 1), and 1.015, 1.034 on fp10, fp12. For B on fp16 the negation
reference is therefore not the cheapest rho that was run, by up to 11%. No ranking changes (the
smallest complete-cell gap is 5.0x), but the descriptor must either use `rho-neg-v2` for B or
state this.

Missing, in order of importance: (1) raw sessions in git, (2) replay certificate and validator
report in gate schema, (3) a superseding RESULTS record fixing F1 and F4 to F7 (RESULTS.md is
immutable; file a new record, do not edit), (4) the `strongest_available` decision, (5) pin the
B source commit (tag or merge) and add `pipeline.sh` plus build receipts (rustc, target,
features) to git, (6) mirror sessions B and C-10-14 under the `v2/` S3 prefix or cite
`snapshot-1730`.

## 10. Reproduction

Layout used by `validator/*.py` (edit the constants at the top of `recompute.py` if different):
`/dev/shm/restore/snapshot-1730/sessions/{B,C-10-14}`, `/dev/shm/restore/v2/sessions/C-16`,
`/dev/shm/restore/v2/bin/ecbench-{B,C}`, symlinks `/dev/shm/restore/sessions/{C-10-14,B}`
and `/dev/shm/sessions/C-16` for `rerun_compare.py` and `redecide.py`.

```
export AWS_SECRET_ACCESS_KEY="$AWS_SECRET_KEY"
aws s3 sync s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/v2/ /dev/shm/restore/v2
aws s3 sync s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/snapshot-1730/ /dev/shm/restore/snapshot-1730
python3 -I validator/recompute.py     # section 2 and 4 ratios, A/A rows of ratios
python3 -I validator/pvals.py         # CI and p-values of the 15 complete cells vs decision.json
python3 -I validator/maxdiff.py       # max relative difference to decision.json and RESULTS
python3 -I validator/slope.py         # section 5
python3 -I validator/inv.py           # arms, unpriced counters, status census per curve
python3 -I validator/aa.py            # A/A, seeds, status census
python3 -I validator/exh.py           # F7: exhausted native rows against paired rho S
python3 -I validator/scalars.py       # [d]G == Q for the 306 verified rows
python3 -I validator/rerun_compare.py # 39 ecbench compare commands vs results/raw
python3 -I validator/redecide.py      # frozen decide.py functions on raw -> decision.json
ecbench-C verify --dir <session> --replay-all --out <receipt>      # sections 3
```

Scratch and logs lived under `/dev/shm/val/` (not preserved). The `validator/` directory is the
only addition to the snapshot besides this file.

## 11. validation_report

```yaml
validation_report:
  id: VAL-20261101-counts-v2
  task_id: validator task for PR #3131 (merge 053714ca)
  run_ids:
    - ECBS1hb5ca4da84455   # C-10-14, 96 records
    - ECBS1hab6ef45acb59   # C-16, 32 records
    - ECBS1he2397c126e47   # B, 288 records
  artifact_checks:
    - {id: 1a-1f, result: pass, note: frozen bytes, approval hash, decide.py hash, originals, spec diff, timeline}
    - {id: 2a, result: pass, note: 15 of 15 session file hashes bound to the receipts}
    - {id: provenance, result: inconclusive, note: git_commit null in all sessions; original C-10-14 and B binaries unrecoverable (F2, F8)}
    - {id: archive, result: fail_minor, note: raw records only in S3; v2 prefix lacks 2 of 3 sessions (F3)}
  metric_recomputations:
    - {id: S ratios tensor/rho, ic-mitm, native, w in 0,0.1,1, result: pass, max_rel_diff_to_decision_json: 3.3e-16, max_rel_diff_to_RESULTS_rounded: 0.024}
    - {id: p-values and CIs of 15 complete cells, result: pass}
    - {id: gap growth, result: fail, claimed: about 4x per 2 bits, recomputed: 5.06x per 2 bits (slope 1.169, bootstrap 95% 1.096 to 1.242)}
  control_checks:
    - {id: A/A rho-neg vs rho-neg-aa, result: pass, note: identical by construction, F6}
    - {id: A/A native-selectors vs -aa, result: pass, pairs: 32, exhausted_pairs: 19}
    - {id: positive rho-neg-v2, result: pass, verified: 32 of 32}
    - {id: replay C-10-14, result: pass, reproduced: 96 of 96}
    - {id: replay B, result: pass, reproduced: 288 of 288}
    - {id: replay C-16, result: pass, reproduced: 32 of 32 (24 non-tensor + 8 tensor), problems: 0}
    - {id: scalars, result: pass, checked: 306, failures: 0}
  heuristic_validation_checks:
    - {id: pre-registration, result: pass}
    - {id: sample integrity, result: pass}
    - {id: scale binding, result: pass_with_limitation, note: toy 10-16 bits, root_u = 0}
  cost_model_checks:
    - {id: unit and band, result: pass_with_gaps, note: memory not reported}
  proof_architecture_checks: []
  verdict: passed
  limitations:
    - one FAIL on RESULTS.md wording (F1) and wording overclaims F6, F7; the data are admissible
    - provenance of original binaries not recoverable; no source rebuild done
    - replay binaries are producer-supplied
    - validator-computed slopes (section 5) are descriptive, not part of the frozen analysis
    - VALIDATION.json not issuable (section 9)
  artifact_paths:
    - research/hashed_ecdl_counts_20261031/VALIDATOR.md
    - research/hashed_ecdl_counts_20261031/validator/
  review_attestation:
    joints_owned: all (no review_plan on the task card)
    sources_read:
      - research/hashed_ecdl_counts_20261031/{README.md,DECISION.md,protocol.json,SOURCES.sha256,spec-*.json,analysis/decide.py,results/RESULTS.md,results/decision.json,results/raw/*}
      - crypto-autoresearcher ledger/decisions/DEC-20261031-77e2b3.yaml (via gh api)
      - src/cryptanalysis/ecbench/{band.rs,compare.rs,stats.rs,audit.rs,record.rs,methods.rs (PRICED_NATIVE)}, src/cryptanalysis/ic_boundary.rs, docs/ic/calibration.json, docs/VALIDATION_GATE.md
      - S3 validation/hashed-ecdl-counts/20261031/{v2,snapshot-1730}/** (sessions, bin, receipts, decision, pipeline.sh)
    read_sibling_reports: false   # DECISION.md is the Coordinator's pre-data protocol re-review, read for the advisories; no other reviewer or validator report was read
    verdict: passed
```
