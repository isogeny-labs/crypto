# Validation review: `research/ic_la_m3_20261115/REPORT.md`

Validator (independent; did not write the code, the spec, the run or the report), 2026-11-15.
Standing of this review: `[pcc:unverified]` (no pcc claim is registered for it).

**Disposition: `validated-scoped`.**

The linear-algebra result holds. All 42 certificates pass when I re-verify them with my own
elliptic-curve code. The 24 solver comparisons are identical in every field outside the
linear-algebra phase. Every `row_ops` and wall number in the table recomputes from the raw records.
The `row_ops` saving is a real algorithmic saving, not an accounting artefact. Three things show
this: the code counts the same unit in both solvers, a complete unit (`bitop/v1`) gives the same
ratio, and my own re-implementation of both solvers gives the same size of saving.

The §5a band numbers are not validated as written. `analyze.py` computes the band under a reading
that matches neither the §5a rule nor `band.rs`:

- it charges rho's unpriced canonicalisations at 0 at every weight;
- it leaves out the counted `table_insert_attempts`.

The direction holds under the default reading: the echelon arm is cheaper than the gauss arm at
w = 0.1 and w = 1, and rho is ahead at every weight. But the ranges change, and the w = 1 column of
the REPORT table is off by up to 1.9× (finding 1).

What holds:

- **Certificates.** For all 42 records, `[recovered]G = Q` and `[planted]G = Q` hold on curves I
  checked independently. Each planted scalar reproduces from the declared target law and seed.
- **Identity.** All 24 echelon-vs-gauss and structured-vs-gauss pairs are equal on the seven fields
  the report names. They are also equal on every phase except linear algebra: adds, doubles,
  scalar mults, GAE, every `native` and `work_native` counter, every counter except `matrix_work`
  (which mirrors `row_ops`), `cost`, `boundaries`, `outcome` and the factor base. Inside linear
  algebra, only `row_ops` and `gauss_entries_scanned` differ, plus the `bitop/v1` LA cost
  derived from them.
- **Counts.** The `row_ops` medians and ratios (0.10–0.15×, 0.25–0.30×) and the LA wall ratios
  (0.06–0.15×) recompute exactly. So does S/rho at w = 0 (52–1 230).
- **Pin equivalence.** The equivalence holds by construction, under the target-pinned completion
  rule, with the target as the last column. It also holds empirically: 24 of 24 runs here, and
  my own re-implementation, row by row.

Findings, in brief:

- **§5a band reading** (finding 1). Rho is priced at 0, and table work is left out. The REPORT
  ranges and the benchmark-record metric `ic_la_echelon_band_w1_r37_f4096` (975 vs 2 683) do not
  hold under the default reading, where the values are 537 vs 1 392. Every echelon/gauss pair is
  "unresolved inside the band" by `ecbench band`, because they tie at w = 0. The REPORT does not
  state that.
- **Not supported as written** (findings 2–4).
  - "dominant in the §5a band at 0.1": in this run linear algebra is the largest priced part at
    w = 0.1 in only 2 of 6 gauss runs at |F_x| = 4096.
  - "a seventh to a tenth … of the linear-algebra time": it is 0.06–0.15×, a seventh to a
    seventeenth.
  - The structured-gauss reading leaves out its wall (0.50–0.88×) and its scans (2.7–7.8×).
- **Omissions** (findings 5–6).
  - The unpriced counters of each arm are not listed with their counts.
  - There is no `w_cal` reading.
  - There is no §3 class or §3a chain.
- **Process gaps** (finding 7).
  - The raw data are not in S3.
  - The record's `reports.pdf` names `REPORT.md`.
  - The replay-audit command is not in the record.

## What was validated

| item | value |
|---|---|
| validated objects | `12dfc56f68c8641ce643b73c85158ecb30fdd3be` (origin head at dispatch), read with `git archive` / `git show`; the working tree was not used. Origin has since moved to `5d618524e2`, which adds only `research/ic_gpu_rootfind_la_20261115/` and `.resume/RESUME.md`. It does not touch this study, `src/` or the benchmark record |
| run commit | `9f8feeace6a1f69c23c21c82506cac67d2ff8921`, `git_dirty: false` in the session. `src/` is identical at `9f8feeace6` and `12dfc56f68` (`git diff --stat` empty). The code reviewed is the code at `9f8feeace6` |
| raw tarball | `raw/out_la.tgz`: 43 587 B, sha256 `8510cd97a8462085f203c64264a7233ababf7cbc5bbd1f9baa20a647725c877e`. This matches `.sha256`, REPORT.md and the record. It has 12 entries and no unsafe paths |
| session | `ECBS1h96f04c2a546f`, status `complete`, 42/42 planned and written, 42 `verified`. `records_sha256` `68ca67ea…`, `plan_sha256` `f68f99df…` and `host_sha256` `e105930c…` match the files. `binary_sha256` is `7193e9b9…` |
| binary | `target/release/ecbench` in this container hashes to `7193e9b9…`. That equals the session's and the auditor's `binary_sha256`. Its mtime is 16:55, the minute the run started. The session reads the commit at run time (`source: runtime_worktree`), so the binary-to-source binding is not independent (see "Not checked") |
| host | 4-core Intel Xeon @ 2.30 GHz VM, `--cpus none --allow-busy`, L0 on every record. `perf_event_open` is unavailable, so there are no hardware counters |

## Requirement checks

| # | requirement | result | evidence |
|---|---|---|---|
| 1a | Raw archive hash | **pass** | `sha256sum raw/out_la.tgz` = `.sha256` = record `raw[0].sha256` |
| 1b | One ecbench session, 42 records | **pass** | `la/session/{session,plan,spec,host}.json`, `records.jsonl` (42 lines, seq 0–41). `audit.json`/`audit.log`: `ok`, 0 problems, 42/42 replays "identical", `replay_selection: all`. `run.log`: 42 lines, `RUN rc=0`. `host.json`, `nproc.txt` = 4 |
| 1c | Replay reproduces | **pass, same machine type** | I ran `ecbench verify --replay-all` with a copy of the exact binary on a copy of the session: `audit OK: 42 records, 42 verified, 42 replays reproduced, 0 problems`. Receipt sha256 `3fdee25b…`. My auditor env class is `ECBENV3hdd6c4f5c7267` and hardware class `ECBHW1h0e053363c48a`. The producer's are `ECBENV3h0addad1d0728` and `ECBHW1hd713976c9605`. The CPU model and core count are the same, so this is a determinism check, not a §12 cross-host audit |
| 2a | Protocol and spec committed and pushed before the run | **pass** | `9f8feeace6` (PROTOCOL.md, RESUME.md, specs/la.json) was committed at 16:47:14Z. GitHub REST `activity` shows `branch_creation … 9f8feeace6` at 16:47:26Z; the local reflog says 16:47:27. Session `started_unix_ms` 1794761720207 = 16:55:20.207Z |
| 2b | Session spec byte-identical to the committed spec | **pass** | `cmp la/session/spec.json specs/la.json` gives identical bytes, sha256 `9421ff36…`, equal at `9f8feeace6` and `12dfc56f68`. PROTOCOL.md and the spec are unchanged after `9f8feeace6`. The session's `spec_sha256` `d93f68b3…` is ecbench's canonical identity hash of the spec, not the file hash; the audit's `spec_id` check passed |
| 2c | Benchmark record on a clean commit with the command | **pass, with gaps** | `benchmarks/records/20261115T1647Z-ic-la-m3.json`: `commit` `9f8feeace6`, `commit_dirty: false`, `created_utc` 16:47:27Z. `commands[0]` holds the full `ecbench run` argv, exit 0, 184.2 s. Gaps are in finding 7 |
| 3a | Certificates, re-verified with independent code | **pass, 42/42** | My own affine short-Weierstrass code, using nothing from the repository. For each of the 3 curves: p and r prime, `group_order = r` (cofactor 1), non-singular, Hasse bound, G on the curve, `[r]G = O`, and the record's generator equals the spec's. For each record: Q on the curve, `[recovered]G = Q`, `[planted]G = Q`, `status = verified` |
| 3b | Planted targets drawn by the declared law | **pass, 42/42** | `planted = 1 + derive_u64("uniform_scalar_sha256_v1", [model, r, 20261125, index]) mod (r−1)`, reimplemented from `workload.rs` and `canonical.rs`. It reproduces every record's `planted`, so the targets were not chosen |
| 3c | Identity on the report's fields (trials, relations, logarithm, status, S, total GAE, relation-phase adds) | **pass, 24/24, 0 mismatches** | My `identity.py`, grouped by (curve, workload, \|F_x\|) |
| 3d | Identity beyond the report: every phase except linear_algebra | **pass, 24/24** | Every differing path is: (i) the solver's own identity (`method.*`, `detail.label`, `detail.linear_algebra.name`, `fbm_id`); (ii) `counters.matrix_work`, which equals LA `row_ops`; (iii) LA `native.row_ops` and LA `work_native.gauss_entries_scanned`; (iv) the `bitop/v1` LA phase and the totals that sum it. The rest is equal: the `factor_base`, `oracle_setup`, `relations` and `verify` phases (charged counts, GAE, every `native` and `work_native` counter); all other counters; `cost`; `boundaries`; `outcome`; `factor_base`; `algorithm_seed`; and the other LA counters (rank, rows, columns, `pinned_by_repeated_row`, `two_column_rows`, `pivot_inversion_steps`, `row_build_*`) |
| 4a | LA `row_ops` medians and ratios | **pass** | See "Confirmed numbers". Echelon/gauss is 0.1017–0.1481 on medians (0.1015–0.1555 per target). Structured/gauss is 0.2477–0.2967 (0.198–0.323 per target) |
| 4b | LA wall ratios | **pass (L0, indicative)** | Echelon/gauss is 0.0586–0.1486 on medians (0.053–0.153 per target) |
| 4c | §5a band as computed in `analyze.py` | **reproduced, but the reading does not conform** | Finding 1 |
| 4d | S/rho at w = 0 | **pass** | 51.6–1 229.8 |
| 4e | "Rho stays ahead at every weight" | **pass under every reading** | analyze.py's, `band.rs` default and pre-2026-11-08. `ecbench band --ranks` gives `rho-neg < <IC arm>` for all 18 IC arms × curves |
| 4f | RESULTS.md, results.json, fig_la.svg regenerate | **pass, byte-identical** | `analyze.py` on the extracted session; `make_figure.py` on its `results.json` |
| 5a | `IncrementalEchelon` `row_ops` comparable to `IncrementalGauss` | **pass** | "Code review" |
| 5b | Same pin at the same row by construction | **pass** | "Code review"; also checked independently in `sim.py` |
| 6 | Wording | **findings 2–6** | |

## Confirmed numbers

Medians over the 2 targets per cell. All values come from `records.jsonl`. Columns:

- **row_ops**: LA `row_ops` for gauss, echelon and structured, with the ratio to gauss.
- **wall**: LA wall in seconds, gauss → echelon, L0 and indicative only.
- **cost/rho at w = 1**: gauss → echelon, first as the REPORT computes it (analyze.py), then
  under `band.rs`'s default reading. Finding 1 explains the difference.

| log2 r | \|F_x\| | gauss row_ops | echelon row_ops | structured row_ops | LA wall (s) | cost/rho w=1 (REPORT) | cost/rho w=1 (default) |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 33.45 | 2048 | 46 595 764 | 6 900 110 (0.148) | 12 373 965 (0.266) | 0.194 → 0.020 (0.103) | 314.4 → 116.3 | 167.9 → 68.7 |
| 33.45 | 4096 | 108 899 824 | 11 069 696 (0.102) | 27 848 620 (0.256) | 0.635 → 0.037 (0.059) | 640.7 → 152.3 | 362.6 → 118.2 |
| 35.18 | 2048 | 43 602 814 | 5 790 094 (0.133) | 12 938 796 (0.297) | 0.214 → 0.026 (0.123) | 4 170.1 → 2 525.0 | 2 195.7 → 1 365.8 |
| 35.18 | 4096 | 293 352 270 | 41 575 257 (0.142) | 72 659 014 (0.248) | 1.219 → 0.115 (0.094) | 14 122.3 → 3 167.9 | 7 492.1 → 1 966.2 |
| 37.39 | 2048 | 42 327 114 | 5 906 238 (0.140) | 12 533 656 (0.296) | 0.246 → 0.037 (0.149) | 1 646.2 → 1 434.8 | 836.3 → 730.5 |
| 37.39 | 4096 | 340 654 663 | 46 505 346 (0.137) | 99 389 693 (0.292) | 1.519 → 0.135 (0.089) | 2 682.9 → 975.5 | 1 391.8 → 537.1 |

Every `row_ops`, wall and REPORT-reading cell of the REPORT table matches these to its printed
precision. Further confirmed values:

- **REPORT reading (analyze.py).** At w = 0.1, gauss/echelon is 1.027–1.870. At w = 1 it is
  1.147–4.458. Echelon/rho is 58.1–1 359.3 at w = 0.1 and 116.3–3 167.9 at w = 1.
- **Benchmark record.** The metric values (46 505 346 vs 340 654 663, i.e. 0.137×; max ratio
  0.148; 975 vs 2 683, i.e. 2.75×) equal the REPORT reading.
- **Complete unit (`bitop/v1`, `work.complete: true` on all 36 IC records).**
  - LA bitops, echelon/gauss: 0.104–0.149, the same as the `row_ops` ratio.
  - LA bitops, structured/gauss: 0.254–0.304.
  - Whole-run bitops, echelon/gauss: 0.38–0.95. The REPORT does not give these.
- **Rho reference.** Mean rho S per curve is 1.852, 0.116 and 0.406, which is 2.09×, 0.13× and
  0.46× the negation floor 0.886.
  - The 1 230× (35.18 bits) therefore mostly reflects two lucky rho runs (S = 0.092 and 0.141).
    The REPORT's "noisy" caveat is correct.
  - Against the floor, IC S is 105–333×.

## Findings

1. **The §5a band in `analyze.py` does not price the reference arm, and it leaves out the counted
   table work.**

   **What `analyze.py` does.** `unpriced_total` sums, for each name in `cost.unpriced`, only the
   phases' `native` maps.
   - Rho records list `canonicalisations_uncharged`, but their phase `native` maps are empty. The
     count is in the top-level `counters`: 272 317, 127 502, 17 778, 27 380, 196 061 and
     147 666 per run. So `U_rho = 0` at every weight.
   - It also ignores `table_insert_attempts` (4 194 304 at |F_x| = 2048 and 16 777 216 at 4096),
     which `oracle_setup` counts behind `counted:table_insert_attempts`.

   **What the rule and the tool do.**
   - §5a requires "every arm, the reference included".
   - `band.rs::unpriced_counts` looks in the top-level counters first. `table_work_counts` adds the
     marked table work by default since 2026-11-08.
   - The earlier reviews computed the band the same way: `ic_gpu_rootfind_big_20261115`,
     `ic_gpu_rootfind_m3_20261115` and `ic_gpu_rootfind_pairs_20261115` (their finding 10). Each
     says it priced rho's `canonicalisations_uncharged` and the counted `table_insert_attempts`.
   - PROTOCOL.md says the band is "computed per arm … against the matched rho". As implemented,
     the reference arm carries no unpriced work.

   **Recomputed.** My own implementation of `band.rs`'s rule agrees to the last printed digit with
   `ecbench band` from the exact binary, in both readings. Under the default reading:
   - **Echelon vs gauss.** Gauss/echelon is 1.03–1.82× at w = 0.1, against the REPORT's 1.03–1.87.
     At w = 1 it is 1.15–3.81×, against 1.15–4.46. The pre-2026-11-08 reading, which prices rho
     but not the table work, gives the REPORT's solver ratios exactly. Those ratios do not involve
     rho.
   - **Echelon vs rho.** 54.7–1 254 at w = 0.1, against the REPORT's 58–1 359. 68.7–1 966 at
     w = 1, against 116–3 168.
   - **Gauss vs rho at w = 1.** 168–7 492, against the REPORT table's 314–14 122.
   - **Band verdict.** `ecbench band --ranks` gives **"unresolved inside the band"** for all 6
     echelon/gauss pairs and all 6 structured/gauss pairs. They tie at w = 0, because S is equal.
     The REPORT gives per-weight ratios, which is allowed, but does not state the band verdict.
     The correct statement is: equal at w = 0, echelon cheaper at w = 0.1 and 1, unresolved under
     the §5a ranking rule.
   - **Benchmark record.** The metric `ic_la_echelon_band_w1_r37_f4096` (value 975, baseline
     2 683) is in `benchmarks/progress/` under the non-conforming reading. Under the default
     reading it is 537.1 vs 1 391.8, a 2.59× solver ratio, not 2.75×.

2. **"dominant in the §5a band at 0.1 and 1 GAE per operation"** (Intro; also implied by "the gain
   is largest at |F_x| = 4096, where linear algebra dominates").
   - This is attributed to round 4, but these records carry the same counts on every arm. They
     support it at w = 1 only. In the 6 gauss runs at |F_x| = 4096, linear algebra is 71–88% of
     the priced cost at w = 1.
   - At w = 0.1 it is 31–56%, and it is the largest part in only 2 of the 6 runs (35.18 bits).
     `oracle_setup` is largest at 33.45 bits and `relations` at 37.39 bits. These shares use the
     default reading; analyze.py's reading gives the same ordering.
   - "the largest part of an m = 3 solve at |F_x| = 4096" holds on round 4's device-table arm, in
     wall time. It does not hold for this CPU run's wall: `oracle_setup` takes 2.7–3.6 s and
     `relations` 0.35–6.9 s, against 0.60–1.59 s of linear algebra. The sentence should say which
     arm and which unit.

3. **"a seventh to a tenth of the row operations and of the linear-algebra time"** (What this
   changes).
   - The row operations are 0.10–0.15×, so "a seventh to a tenth" fits.
   - The LA time is 0.059–0.149× (0.053–0.153 per target), i.e. a seventh to a seventeenth.
   - The wall ratio is also lower than the `row_ops` ratio at |F_x| = 4096 (0.06–0.09 against
     0.10–0.14). Part of the wall gain therefore comes from the kernel, not from fewer row operations. The
     echelon arm uses a Shoup `FixedMul` against `u128 %` and stores sparse rows [pcc:unverified].
   - The wall is L0 and indicative. This sentence does not carry that qualifier, though the
     Standing and Wall bullets do.

4. **structured-gauss is presented by its `row_ops` alone.** The report and the record give "0.25–
   0.30×". The same records show more:
   - Its `gauss_entries_scanned` is 2.7–7.8× the gauss arm's. These scans are counted, but are not
     in `cost.unpriced` and not in the band.
   - Its LA wall is 0.50–0.88× (up to 1.05× per target).
   - The `row_ops` statement is correct as a `row_ops` statement. The REPORT does not call
     structured-gauss better.

5. **The §5a counter list is missing.** §5a requires listing every unpriced counter of every arm,
   with its count.
   - IC arms leave unpriced `sqrt_solves`, `lookups`, `row_ops`, `legendre_symbols`, `inversions`
     and `target_guard_probes`, plus the marked `table_insert_attempts`.
   - The rho arm leaves unpriced `canonicalisations`.
   - The REPORT names none of them.
   - Counted work outside both `cost.unpriced` and the band is not mentioned. It is the same on
     all IC arms except `gauss_entries_scanned`. It includes `sweep_field_muls`: 17 884 860 in
     seq 1.

6. **No `w_cal` reading, and no §3 class or §3a chain.**
   - §5a (2026-11-12) asks for a calibrated weight for "the mod p or mod r mulmods of linear
     algebra". `row_ops` are mod-r multiply-subtracts. `band.rs` does not class
     `row_ops_uncharged` as field arithmetic, so the tooling does not enforce this, and the REPORT
     gives no `w_cal` reading.
   - Illustration only, not a measurement on this host: at the 0.005–0.013 GAE per mulmod
     measured by the IC positive control, gauss/echelon priced cost is 1.001–1.059 at w = 0.005
     and 1.004–1.149 at w = 0.013, per workload, under the default reading.
   - The REPORT does not classify the change under §3. The correct class is a linear-algebra
     component change in `row_ops`, scoped (§2), with fewer `row_ops` [pcc:unverified]. S at
     w = 0 is unchanged.
   - It is not relabelling. No other phase moved, and the scans and the `bitop/v1` LA cost fell
     with it.
   - It is not accounting. The unit is unchanged (see "Code review").

7. **Process gaps.**
   - The record has `raw[0].uploaded: false`. `head-object` on its `s3_uri` returns HTTP 403 from
     here, so I could not check S3. The 43.6 KB archive is under §13's 64 KiB threshold, so Git is
     allowed. But the benchmark-record rule asks for the raw data in S3, and CI's §12 cross-host
     re-audit cannot see a session that exists only inside a tarball.
   - `reports.pdf` in the record names `REPORT.md`, not `REPORT.pdf`. The same gap was raised in
     earlier rounds.
   - `ecbench verify --replay-all` ran (audit.json and audit.log are in the tarball), but it is
     not among the record's `commands`.
   - The record's `host.isolation` is "unstated", although every record says L0.
   - `run.log` warns that one measured round with `order: alternate` cannot be balanced: arms ran
     in spec order within each workload. This does not affect counts. It is one more reason the
     wall is indicative.

## Code review (claim 5)

I read these at `9f8feeace6`:

- `IncrementalEchelon` in `src/cryptanalysis/ic_framework/linalg.rs`, lines 980–1135;
- `IncrementalGauss` in `src/cryptanalysis/ic_boundary.rs`, lines 1217–1385, and its
  `RelationSolver` impl;
- the relation loop's completion check in `ic_boundary.rs`, lines 14 080–14 160;
- solver construction in `ic_framework/mod.rs`, lines 480–600;
- `FixedMul` in `koblitz_sparse_la.rs`.

**`row_ops` are the same unit in both solvers.**

- `IncrementalGauss` counts:
  - one per non-zero entry of each pivot row applied, pivot entry included, plus one for the
    right-hand side;
  - the same for the normalisation (every non-zero of the new row, plus the right-hand side);
  - the same for the back-elimination of the new pivot column from each stored row that has it.
- `IncrementalEchelon` counts `p.cols.len() + 2` per pivot row applied: the non-zeros right of the
  pivot, plus the pivot entry (which it never computes), plus the right-hand side. It counts
  `cols.len() + 2` per normalisation. Every multiply-subtract it performs is counted.
- The difference is algorithmic. Echelon does no back-elimination, and it applies only the stored
  tail of each pivot row, so its count includes any fill it creates in later pivot columns.

**Work outside `row_ops`.**

- Echelon's scan of the arriving row is counted in `gauss_entries_scanned`: exactly `cols` per
  row (1 862 × 2 049 = 3 815 238 in seq 2, as recorded). Its inversions are counted in
  `pivot_inversion_steps`.
- One `FixedMul::new` per applied pivot row is not counted separately. It is one u128 division.
  The +1 pivot-entry over-count offsets it in count, though not in time.
- Gauss's dense scans are counted in its own `gauss_entries_scanned`, which is about 5× echelon's.
- The ratio is not an artefact of the count:
  - in `bitop/v1`, which also prices scans and inversions, the LA ratio is 0.104–0.149, the same
    as the `row_ops` ratio;
  - the wall falls by a similar or larger factor.

**Independent re-implementation (`sim.py`).**

- I wrote both algorithms myself under the same counting convention. They ran on synthetic
  m = 3-shaped rows (3 signed factor-base columns plus a non-zero target column) modulo the
  34-bit r, until the target pinned.
- The two gave the same status, the same pin row and the same value on every row (value =
  planted).

The last column gives the records' per-target values: rows, gauss `row_ops`, echelon `row_ops` and
their ratio.

| \|F\| | seeds | rows | RREF ops | echelon ops | ratio | records, per target |
|--:|--:|--:|--:|--:|--:|--:|
| 2048 | 3 | 1 865–1 890 | 41.5–49.2M | 5.6–6.9M | 0.134–0.140 | 1 848–1 899 rows; 39.7–46.9M; 4.9–7.2M; 0.121–0.156 |
| 4096 | 2 | 3 738–3 781 | 323.5–342.1M | 40.3–44.3M | 0.125–0.129 | at 35/37 bits: 3 589–3 752 rows; 247.8–344.9M; 31.7–51.5M; 0.128–0.152 |

**The pin is the same by construction.**

- The pipeline calls `matrix.pinned(d_col)` after every `add_row` and, under `TargetPinned`,
  stops at the first `Some`. `d_col` is the last column: `matrix_by_name(…, fb.columns + 1, …)`.
- Echelon's new pivot column equals Gauss's. The forward-reduced and the fully reduced row differ
  by a vector of the row span. Any non-zero vector of the span leads at a pivot column. If the two
  rows led at different columns, their difference would lead at the leftmost of the two, which is
  pivot-free: a contradiction.
- By induction both solvers accept the same rows, so their augmented spans are equal and
  Dependent/Inconsistent agree.
- Because d is last, a pivot at d is `c·e_d`. In both solvers `pinned(d)` is therefore exactly
  "d is a pivot column", and its value is the system's unique `x_d`.
- The argument holds only for the last column. Echelon's `pinned(col)` for any other column is
  sufficient, not necessary, so it is not a drop-in where other columns are queried. That
  includes the full-rank rules and `check_full_rank_base_columns`. The REPORT restricts it to the
  target-pinned rule, and `mod.rs` assigns `TargetPinned` to it.

## Not checked or not checkable

- **Binary provenance.** The binary was not rebuilt. ecbench reads the commit from the working
  tree at run time. So the link between `7193e9b9…` and `9f8feeace6`'s sources rests on
  `git_dirty: false`, on `src/` being unchanged across `9f8feeace6^..12dfc56f68`, and on the
  binary's mtime. I did not run the Rust test `echelon_matches_incremental_gauss_row_for_row`;
  `sim.py` stands in for it.
- **Cross-host audit.** My replay used the same binary on the same CPU model.
- **S3.** HTTP 403 from here.
- **Wall time.** I recomputed the wall numbers from the records. Their quality cannot be validated
  at L0.
- **research-kb.** Not reachable here either. There is no `crypto-autoresearcher` checkout under
  `/home/user`, which is consistent with the REPORT's stated gap.
- **The GPU-pod extrapolation.** "would fall to roughly a tenth" is labelled as not measured.
  Nothing here tests it.

## Method

All checks were run on `git archive 12dfc56f68` output and the extracted tarball. The scripts are
in the validator's scratch directory and are not committed, by the dispatch's "edit nothing else"
rule:

- `certs.py`: own EC arithmetic;
- `planted.py`: target law;
- `identity.py`: full-record diff per pair;
- `tables.py`: medians, ratios, three band readings;
- `sim.py`: independent solver re-implementation.

`ecbench band` (default and `--no-price-table-work`) and `ecbench verify --replay-all` were run
from a copy of the exact binary on copies of the session.
