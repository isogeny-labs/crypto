# Validator report: research/quartic_diem_la_20261123

Cheaper linear algebra for Diem double-large-prime index calculus on prime-order plane
quartics. The arms are `2lp-sfb` (scalar Wiedemann on AᵀDA) and `2lp-sfb-sge` (trim, then SGE,
then coordinate Wiedemann on a square subsystem, with a fully counted fallback).

- Validator role, Claude Opus 5.5, 2026-11-24.
- Every number below is `[pcc:unverified]`. No pcc claim references this study, and the
  validator created none.
- **Validated commit:** `23384701f18747d4ab225143549b574ca731a789` (results). The freeze commit
  is `612f8dee20e`. Branch: `claude/hyperelliptic-index-calculus-td1fdn`.
  - The study tree there is `f5fd68d162451664f40f11e98d6c5d873612ccca`.
  - At dispatch, the study directory was clean against that commit. Only `.resume/RESUME.md`
    was dirty in the checkout.
- **Changes made by others during the validation.** Another session committed WIP snapshots of
  this `validation/` directory while it was being written, from `f2840e8c0e9` to `a6e8fdcfc68`
  and later. They are not authoritative; every output here was regenerated at the end.
  - The only change outside `validation/` after the validated commit is `720044c7f91`. It adds
    a `[pcc:unverified]` tag to RESULTS.md line 22. No number changes, and the errata's line
    numbers hold in both versions.
  - To stay off the moving tree, every input was read from `git archive 23384701f18` (the study
    plus the parent's `raw/`) or from `git show`.
  - The three `src/` files read for the audit equal `source.sha256` and the validated commit.
- **Binary:** `genus_ladder.qla`, sha256 `adddfbfa…d6d1f6`. It was hashed and copied at
  dispatch, before anything ran.
  - The hash equals `binary.sha256` and `dev/binary.sha256`.
  - Filesystem birth = mtime = 2026-11-24 06:47:21 UTC.
  - Not rebuilt: the dispatch allowed no cargo builds.
- **Environment:** Python 3.13.18 and numpy 2.5.3. The host has 4 cores and about 15 GB RAM.
  The replays used at most 2 threads, with peak RSS around 75 MB.

## Verdict: **validated-with-errata** (gate disposition `validated`)

The frozen rule gives **IMPROVEMENT** [pcc:unverified], and this reproduces.

- **Pooled paired GAE₁ sge/sfb:** 0.371439. The validator's exact cluster-bootstrap 95%
  interval is [0.3398, 0.4063]; the producer's 4000-replicate interval is [0.3401, 0.4057].
- **w = 0.1:** 0.371758, exact [0.3401, 0.4066].
- **w = 0:** 1.002819, exact [1.0010, 1.0048]. This is within the preregistered ≤ 1.01.

What reproduced:

- all 80 certificates, checked with solver-independent arithmetic;
- all 80 runs, by replay with the frozen binary (148 of 148 rows byte-identical in their
  counted fields);
- every point estimate in `analysis.json`, by an independent re-implementation, to 8e-14;
- `analysis.json` itself, which the frozen `analyze.py` regenerates byte-identically.

Six errata (E1–E6) correct wording or binding statements. None changes a number in
`analysis.json` or the frozen decision. E2 matters for how the result may be cited: the
order of the two arms is **not** a strict §5a ranking across the band, because it reverses at
w = 0. Per lesson L-8d86e8fc352c, the corrections belong in a sibling `ERRATA.md`, and
RESULTS.md, PROTOCOL.md and `analysis.json` stay byte-equal (all three are hash-pinned).

## Requirements

| # | requirement (dispatch) | result | evidence |
|--:|---|---|---|
| 1 | `frozen.sha256`, `data.sha256`; freeze before every grid-prime row | pass | F1; `checks_extra.json` (manifests, timing) |
| 2 | binary hash vs `binary.sha256` | pass (binary-to-commit binding not verified: no rebuild) | F2; `checks_extra.json` (manifests.binary) |
| 3 | replay with the frozen binary, ≥ 2 instances at 4099/16411/65537 incl. a fallback, ≥ 1 at 131101 | pass: **all** 20 curves; 80 of 80 runs; 148 of 148 rows byte-identical with wall masked (80 runs, 40 skips, 20 instances, 8 cells); 8 of 8 fallback runs, incl. 16411/252 seeds 1 and 2 | F3; `replay-certificate.json`, `replay_compare.json` |
| 4 | independent section-6 analysis | pass: points to 8e-14; intervals within bootstrap noise; decision and 8 verdicts equal | F4; `recompute.json` |
| 5 | integrity: certified, `recovered == t_secret`, same log, C0 | pass: 80 of 80 certificates by independent arithmetic; C0 40 of 40 on all 55 shared fields | F5; `certificates.json`, `recompute.json` (integrity) |
| 6 | pre-freeze amendment vs `dev/`; parameter pick by the stated rule | pass: the coincidence claim is true in all 12 settings; pick reproduced; the mechanism wording is imprecise (E6) | F6; `checks_extra.json` (dev_amendment), `diag_check.json` |
| 7 | every number in RESULTS.md and `report.typ` | pass with errata E3–E5 | F7 |
| – | §5a band and ranking | the frozen rule holds; **no strict §5a ranking** (E2); no `w_cal` (P2) | F8 |
| – | accounting fairness of the new arm | pass; structural identities exact; stress test | F9 |
| – | repository policy | process findings P1–P5 | below |
| – | `VALIDATION.json` | `python3 scripts/validation_gate.py evidence research/quartic_diem_la_20261123/validation` prints `valid` | `VALIDATION.json` |

## Findings

**F1. The freeze precedes every grid-prime row.**

- `sha256sum -c frozen.sha256` and `sha256sum -c data.sha256` report every file OK.
- Freeze commit `612f8dee20e` is dated 2026-11-24 07:19:09 UTC.
- The main grid's stamp `raw/main_start.txt` reads 07:19:22, 13 s later. The 131101 run's
  stamp `raw/r131101_start.txt` reads 07:23:33.
- Filesystem birth of `raw/run.jsonl` is 07:19:22.82, and of `raw/run131101.jsonl` 07:23:33.71.
- The rows' own `done` totals (250.88 s and 930.79 s) reproduce both end stamps to within
  0.21 s.
- Git history: the only commit that touches `raw/` is `23384701f18` (07:42:18).
- `analyze.py` has had one hash (`3166cc75…`) since the draft snapshot `5299e594a29`
  (2026-11-23 17:56). Section 6 of PROTOCOL.md is unchanged by the freeze diff. So the decision
  rule predates the development grid (06:54 to 07:18 on 2026-11-24) as well as the grid.
- These times are self-reported by the producer's host. They agree with each other but are not
  tamper-proof.

**F2. Bindings.**

- `source.sha256`: all three files match their Git blobs at `4faca82a6f0`, at `612f8dee20e` and
  at `23384701f18`.
- The binary's hash matches.
- The binary's link to a commit is **not established**. It predates `4faca82a6f0` (06:50:13) by
  about 3 minutes, and the bench record names `4faca82a6f0` with `commit_dirty: true`. Other
  compilation inputs (`plane_quartic_pic.rs`, `chained_transfer/ic_control`, `ic_boundary`,
  `residual_walk`, …) are not hashed. See E5.
- Mitigations:
  - C0 ties the `2lp-sfb` arm, row for row on 55 fields, to the separately validated parent
    binary `f0243cf8…`.
  - The sge arm's dominant counter is an exact function of the reported dimensions (F9).

**F3. Replay.**

- `validation/replay.sh` runs the frozen flags of `run_main.sh` (2 threads, all 15 curves) and
  `run_131101.sh` (1 thread, all 5 curves). Output went to the validator's scratch directory,
  never to `raw/`.
- `compare_replay.py` matches rows by (kind, family, arm, p, curve_seed, seed). Each committed
  and replayed JSON line is byte-identical after masking only `wall_ns`, `search_wall_s` and
  `wall_s`.
- Result: **148 of 148** rows equal (80 runs, 40 skips, 20 instances and 8 cells; `config` and
  `done` excluded). No replayed key is duplicated, and every committed row is covered.
- All 8 fallback runs were replayed.
- Every replayed run is certified with `recovered == t_secret`.
- Replayed wall time over committed wall time ranged from 0.77 to 1.45. The host is shared, and
  wall time is not part of any claim.
- Scratch hashes: `replay_main.jsonl` sha256 `1a9744f9…ca27`; `replay_131101.jsonl` sha256
  `0c193d47…bec0`. The files are not committed (§13); `replay_compare.json` holds the per-row
  result.

**F4. Independent section-6 analysis.**

- `recompute.py` is written from PROTOCOL.md alone; `analyze.py` is neither imported nor copied.
  - GAE_w = group_ops + w·(rel_mul + rel_add + la_mul + la_add + idx_lookups + idx_inserts).
  - The reference is R = sqrt(πr/4).
  - Curve level is the geometric mean over seeds; per-prime is the geometric mean over 5
    curves; pooled is equal weight per prime; the slope is in ln p.
- Every point estimate agrees with `analysis.json` within 8.2e-14 relative: the paired ratios
  (GAE at three weights, la_add, la_mul, la total, all native), the 24 per-prime IC/rho values,
  the pooled values and the slopes.
- Intervals: the validator computes the **exact** cluster-bootstrap law, by enumerating the
  126 multisets per prime with multinomial weights and convolving the four primes for the
  pooled interval.
  - It also ran 400 independent 4000-replicate Monte Carlo bootstraps.
  - Every producer pooled endpoint lies inside the range of those 400 runs.
  - The largest relative difference from the exact law is 1.7% pooled (la total, upper end) and
    1.6% per prime (sge IC/rho, w = 0, 65537, lower end, at a jump of the discrete law).
  - This is bootstrap noise, not a disagreement.
- The decision is IMPROVEMENT [pcc:unverified] under both the Monte Carlo and the exact intervals.
- All 8 IC/rho cells (2 arms × 4 primes) are UNRESOLVED. In each, IC/rho is below 1 at
  w = 0 and above 1 at w = 1.
- For reference, the exact interval for sge IC/rho at 131101, w = 1, is [2.0226, 3.8201].
  RESULTS shows [2.02, 3.80], which is Monte Carlo noise.

**F5. Integrity.**

- `verify_certificates.py` uses `picarith.py`, copied byte-identical from the parent validation
  (sha256 `b6b60f41…3b16`). It is pure Python and shares no code with the solver. Rows are read
  with `git show` at the validated commit.
- Results for 20 of 20 curves and 80 of 80 rows:
  - F is smooth and P0 is on C.
  - r is prime, h = 1, and the Hasse-Weil bounds hold exactly.
  - G and T are valid class records, G ≠ 0, and [r]G = 0.
  - **[t]G = T**, and every row has `recovered == t_secret` with `certified = ok = true`.
  - Negative controls: [t+1]G ≠ T, and a tampered T is rejected.
  - [r]X = 0 for two random classes.
  - #C(F_p), recounted independently, equals `n_points`.
- Both arms recover the same logarithm on all 40 pairs, with the same instance (f, p0, r,
  g_class, target_class, t_secret).
- C0 against `../quartic_diem_ladder_20261121/raw/`: 40 of 40 `2lp-sfb` rows are identical on
  all **55** shared fields (all but `wall_ns`). The producer's check compared 20 of them.
- All 20 instance rows equal the parent's.
- The la split sums to `la_add`/`la_mul` on all 80 rows.
- The relation phase is identical across arms on all 40 pairs: rel_mul, rel_add, idx_lookups,
  idx_inserts, relations, trials, attempts, core_rows and core_cols.

**F6. The pre-freeze amendment.**

- `checks_extra.py` re-derives the amendment from the 12 `dev/dev_W*_F*_E*.jsonl` files.
- Every setting has exactly 4 `la_fallbacks`, on exactly the 4 pairs where `2lp-sfb` needed a
  second attempt: 4093/89/2, 8191/106/2, 32771/217/1 and 32771/217/2. The match holds in both
  directions in every setting. **The coincidence claim is true.**
- FILL changes no field except `wall_ns`, in either arm, at any (W, EXCESS).
- All 12 settings meet the amended rule. The lowest geometric-mean `la_add + la_mul` at 32771
  is (W 8, EXCESS 64) at 12,532,139.7, against 12,700,356.2 for EXCESS 16. The FILL tie goes to
  1.5, so the pick reproduces (it is the same under a curve-level geometric mean).
- Under the original zero-fallback rule no setting was eligible (`dev/choice.json` pick =
  null), as the amendment says.
- The amendment changes eligibility only. It cannot steer the pick, because every setting is
  eligible under it.
- The dev primes (4093, 8191, 32771) are disjoint from the grid.
- Dev C0: the 10 `2lp-sfb` rows at 32771 equal the parent's on every shared field, as the
  freeze commit message states.
- In the study, the 8 fallbacks are again exactly the 8 pairs where `2lp-sfb` needed a second
  attempt. On the other 32 pairs, group_ops are equal.
- **Mechanism (lesson L-4c4c2ac6ef29).** `replay.sh diag` and `replay.sh diagdev` rerun every
  grid fallback curve, and the three dev curves at the picked setting, with the binary's
  uncounted `--quartic-diag`. `diag_check.py` reads the result.
  - The flag leaves every committed field unchanged in 40 of 40 runs.
  - In all 16 grid and 8 dev multi-attempt runs, attempt 1 has rank = rank without `t`, and
    that rank is below the column count. So `t` is **not determined**.
  - The outcome of attempt 1 is 2 (a wrong `t` from the solver), not 1 (no solution).
  - The final attempt determines and certifies `t` in all 40 runs.
  - So the fallback does repeat a solve that fails its certificate. But the system is
    consistent and underdetermined in `t`; it does not have "no solution". See E6.
- **Not checked:** the protocol's `cargo test --release --lib quartic_ladder` requirement. The
  dispatch allowed no builds.

**F7. RESULTS.md and `reports/run-20261124T0719Z/report.typ`.**

- Every number in both files agrees with `analysis.json` or with the validator's recompute
  from raw rows.
- The PDF text matches `report.typ`. `fig_la.svg` equals the copy under `reports/`, and
  `plot.py` plots `analysis.json` values only.
- Specific values checked:
  - the decision table;
  - per-prime GAE₁ 0.376, 0.431, 0.327 and 0.359, with intervals;
  - la-total ratios 0.204, 0.307, 0.198 and 0.253;
  - pooled la_add 0.236 [0.205, 0.270];
  - slope −0.037 [−0.090, 0.014];
  - the IC/rho table, all 16 cells;
  - "about 2e-4" at w = 0;
  - "2.78×, down from 7.75×";
  - la share 42–77% (sge) and 78–89% (sfb);
  - the fallback shares 83%, 46% and 77% (82.5, 45.8, 77.1);
  - the fallback pair ratio ~0.66 (gm 0.660);
  - single-attempt la ratios 0.204, 0.184, 0.173 and 0.168 (the report's "0.17–0.20");
  - 8 fallback runs with 61–77 extra group operations;
  - "0.9% at 16411" (0.86%);
  - the bindings (`6d07d316…`, `aa8fade5…`, the bench-record path, and 07:19 to 07:39);
  - the bench record's two metrics.
- PROTOCOL §2's pre-run model of the core at 131101 ("about 1075 × 744, about 49,500
  non-zeros") matches the measured `2lp-sfb` means of 1074.7 × 742.5 and 49,858.7.
- The exceptions are E3, E4 and E5.

**F8. §5a band.**

- Every counted operation is in the GAE sum, or is a sub-split of a summed counter (checked):
  `la_*` sub-splits sum to `la_add`/`la_mul`, and `rel_enum_*` plus `rel_decomp_*` stay within
  `rel_*`.
- `cantor_mul`/`cantor_inv` are the F_p work *inside* the counted Pic^0 operations
  (`quartic_ladder.rs` module doc, lines 24–26). Leaving them out avoids double counting.
- The remaining integer fields are sizes, counts or identifiers.
- The rho reference is ideal and charges nothing unpriced at any w. The protocol discloses
  this, and it is conservative toward rho. Every skip row's `expected_steps` equals
  sqrt(πr/4).
- The paired order is **not** a strict §5a ranking (E2):
  - At w = 0 the pooled interval lies wholly above 1.
  - At w = 0.1 and w = 1 it lies wholly below 1.
  - The pooled point ratio crosses 1 at w ≈ 1.6e-7 GAE per native operation. Per fallback
    pair, the crossing lies between 4.3e-7 and 4.6e-6.
- Calibration, labelled as a calculation and not a measurement: the counter ratio
  `cantor_mul / group_ops` is 3,570–3,880 F_p multiplications per Pic^0 operation. That
  suggests a per-multiplication weight of about 2.6–2.8e-4 GAE, three orders of magnitude above
  the crossover. A measured `w_cal` is missing (P2).
- `ecbench band` does not apply: these are `genus_ladder` records, not ecbench sessions.

**F9. Accounting fairness of `2lp-sfb-sge`.**

- Source audit, at the hashed `quartic_ladder.rs` and `genus_ladder.rs`:
  - The square solve's candidate is certified by a counted `cmul`.
  - On failure, the full-core Wiedemann reruns with the cloned draws, charged to the same
    `la_split`. The sge arm therefore pays one more failed certificate than sfb on each
    fallback pair, which is the w = 0 excess.
  - `select_rows` and `sge_reduce` charge their arithmetic and index work.
  - Heap operations and the sort/dedup of SGE holder lists are not charged per comparison.
- Structural identities on the rows:
  - In all 32 single-attempt sge runs, `la_spmv_add` equals (2·sq_dim + 2)·sq_nnz *exactly*,
    which is the closed form of the counted code.
  - In all 32, `sge_rows_in` = min(core_rows, core_cols + 64), and `sq_dim` = `sge_cols_out`.
  - In the 32 single-attempt sfb runs, `la_spmv_add` / (2·core_nnz·(3·core_cols + 2)) lies in
    [0.9989, 0.9998].
- Stress test: multiplying every trim and SGE charge in the sge arm by 10 moves the pooled
  GAE₁ ratio from 0.371 to 0.391. Multiplying by 100 moves it to 0.580. The decision holds.

## Errata (file and line, at the validated commit)

- **E1. `PROTOCOL.md:1`.** The title still reads "Protocol (DRAFT, NOT FROZEN)", while line 3
  says "FROZEN 2026-11-24". The file is hash-pinned (`frozen.sha256`), so the correction goes to
  `ERRATA.md`.
- **E2. `analysis.json:1073`** [pcc:unverified], written by the frozen `analyze.py:242`: "IMPROVEMENT (ranked
  below 2lp-sfb across the band)".
  - The frozen rule's IMPROVEMENT is correct [pcc:unverified].
  - The parenthetical is not a §5a ranking. At w = 0 the pooled paired ratio is 1.0028, exact
    95% [1.0010, 1.0048], wholly above 1. So the order of the arms reverses inside the band.
    §5a reports such a pair as unresolved at that boundary.
  - Cite it as: "IMPROVEMENT [pcc:unverified] under the frozen rule (w = 0 within the preregistered 1.01
    tolerance; +0.28% group operations from the failed square-solve certificates of the 8
    fallback runs). Not ranked across the §5a band; the pooled point ratio is below 1 for
    w > 1.6e-7."
  - RESULTS.md, `report.typ` and the bench record do not repeat "ranked". Any leaderboard or
    benchmarks row must use the corrected wording.
- **E3. `RESULTS.md:32`.** "about one failed `[t]G = T` check of ~log₂ r operations" is off by
  about half.
  - The 61–77 extra operations are 1.37–1.55 × log₂ r, with log₂ r = 42.0, 48.0 and 51.0.
  - A double-and-add check costs about 1.5·log₂ r.
  - Read: "~1.5 log₂ r operations".
- **E4. `RESULTS.md:59`.** "Sparse products are still 85–96% of what remains" holds for
  `la_add` (85.2–96.3%). For `la_add + la_mul` the share is 81.6–95.1%. Read: "of the remaining
  `la_add`".
- **E5. `RESULTS.md:8` and `reports/run-20261124T0719Z/report.typ:8`.** "built from
  4faca82a6f0" is stronger than the evidence.
  - The binary (born 06:47:21 UTC) predates that commit (06:50:13).
  - The bench record says `commit_dirty: true`.
  - Only three sources are hashed.
  - Read: "built from a working tree whose three hashed sources (`source.sha256`) equal
    4faca82a6f0's; the binary-to-commit binding is not verified by a rebuild".
- **E6. `PROTOCOL.md:56`.** "the first attempt's system has no solution for either arm" is
  imprecise.
  - The binary's diagnosis, on all 4 dev pairs and all 8 grid fallback pairs, shows a
    consistent system that leaves `t` undetermined: rank = rank without `t` < columns.
  - The solver returns a wrong `t`, whose certificate fails (F6).
  - Read: "the first attempt's core does not determine t for either arm". The conclusion (the
    fallback repeats a failing solve) stands.

## Process findings (no number changes)

- **P1. §13 artifact store.** 13 data files over 64 KiB are committed to Git: `raw/run.jsonl`
  (125,829 B) and the 12 `dev/dev_W*.jsonl` files (about 75.9 KB each). There is no
  `.artifacts.json`, and the bench record lists its raw files with `"uploaded": false`.
- **P2. §5a calibrated weight (owner, 2026-11-12).** `la_*` and `rel_*` count field arithmetic,
  but no `w_cal` (method, host) is recorded. The pair therefore cannot be called "ranked at
  calibrated price". The band verdict stands as in E2.
- **P3. §3a cost chain.** RESULTS.md does not write the chain. The validator checked its
  premise: one replace line (the linear algebra), with the relation phase identical on all 40
  pairs (F5).
- **P4. Frozen `analyze.py` has latent weaknesses, with no effect here.**
  - It reads `recovered`/`t_secret` and the 20 C0 fields with `.get` (lesson L-99ee952350b2).
  - Its integrity test needs only `C0 compared > 0`, and it does not check that the grid is
    complete.
  - The validator confirmed that every such key exists in every study row and parent row, that
    C0 compared 40 of 40, and that the 4 × 5 × 2 grid is complete with both arms.
- **P5. Not run, outside the dispatch.**
  - `cargo test --release --lib quartic_ladder`: no builds were allowed.
  - A clean rebuild to bind the binary to `4faca82a6f0`.

## Files

| file | what |
|---|---|
| `replay.sh` | replay with the frozen flags (`BIN OUT main`, `BIN OUT 131101 SEEDS`), and the uncounted rank diagnosis (`BIN OUT diag`, `BIN OUT diagdev`) |
| `compare_replay.py` → `replay_compare.json` | byte-level row comparison |
| `verify_certificates.py`, `picarith.py` → `certificates.json` | independent certificate re-verification |
| `recompute.py` → `recompute.json` | independent section-6 analysis, exact bootstrap, noise study, band crossover, breakdown |
| `checks_extra.py` → `checks_extra.json` | manifests, freeze timing, dev amendment, `analyze.py` byte reproduction, stress test, structural identities |
| `diag_check.py` → `diag_check.json` | the amendment's mechanism on grid and dev rows |
| `build_gate_artifacts.py` → `replay-certificate.json`, `validator-report.json`, `VALIDATION.json` | gate artifacts; every flag derived from the files above |

Reproduce, from the repository root, after making a snapshot with
`git archive 23384701f18 research/quartic_diem_la_20261123 research/quartic_diem_ladder_20261121/raw | tar -x -C SNAP`:

```sh
V=research/quartic_diem_la_20261123/validation; ST=SNAP/research/quartic_diem_la_20261123
sh $V/replay.sh BIN OUT main && sh $V/replay.sh BIN OUT 131101 53/177/193/228/288
sh $V/replay.sh BIN DIAG diag && sh $V/replay.sh BIN DIAG diagdev
python3 -I $V/compare_replay.py $ST OUT $V/replay_compare.json
python3 -I $V/diag_check.py $ST DIAG $V/diag_check.json
python3 -I $V/verify_certificates.py . 23384701f18 $V/certificates.json --count-points
python3 -I $V/recompute.py $ST SNAP/research/quartic_diem_ladder_20261121/raw $V/recompute.json
python3 -I $V/checks_extra.py . 23384701f18 SNAP BIN $V/checks_extra.json
python3 -I $V/build_gate_artifacts.py . 23384701f18 $V
python3 scripts/validation_gate.py evidence $V
```
