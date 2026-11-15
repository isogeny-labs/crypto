# Validator report: multi-target IC v3, ic5 (k-term table) vs rho, bsgs2 and ic4 (2026-11-11)

Validator: independent `validator` subagent (Opus 5.5), 2026-11-11.

**Verdict: `valid_with_amendments` for the measurement. Disposition of the headline "ic5 is cheaper than the strongest rho in all 36 (cell, c_I)": `insufficient_evidence` (blocking).**

- **The measurement is sound.**
  - All 4488 holdout rows replay exactly. Recovered logs were verified independently: 108 runs, 774,144 targets.
  - Every number in `summary.json`, `analysis.txt`, `noverify.txt` and the RESULTS.md table recomputes exactly from `runs.json`/`rows.jsonl`.
  - The protocol was frozen before the data. Amendment A1 is legitimate in substance.
  - ic5's cost accounting is complete: no uncounted or free work was found.
- **What is validated:**
  - ic5 is cheaper than ic4, the multi-target IC baseline, in all 36 (cell, c_I) decisions (median 0.47 to 0.99). That is a measured improvement over the IC baseline.
  - ic5 is dearer than bsgs2 in all 36 (1.11 to 1.49).
- **What is not validated:**
  - The comparison with rho holds only against this harness's anchored, fixed-database rho family. That family lacks the standard multi-target rule, Kuhn–Struik retention of solved walks.
  - The validator added retention to the harness's strongest rho arm (`rho2xks`, 1056 runs, all verified) and measured ic5 against it:
    - **dearer at h = 0 in all 36** (1.07 to 1.34, every 95% lower bound above 1);
    - **dearer at h = 0.1 in all 36** (1.05 to 1.18);
    - cheaper only at h = 1 (0.91 to 0.98).
  - Under the frozen section-5a rule all 36 decisions become **unranked**.
  - A distinguished-point rho is estimated (model, not measured) to be cheaper than ic5 at h = 1 too.
- **Gate status:** `validator-report.json` has `status: "insufficient_evidence"`, so a gate manifest that pins it does not admit a vs-rho row. The replay certificate is `verified`.

## 0. What was validated, and from where

- **Frozen artifact set:** commit `70d0b40bc365c1269dfc3c1251153cbc44cc589e` (branch `aburan28/eloquent-bell-2jerrf`).
  - I extracted these paths with `git archive` into the validator scratchpad: `research/mt_fieldops_v3_20261111`, `research/mt_rho_start_fix_20261111`, `research/mt_fieldops_v2_20261111`, `examples/mt_fieldops.rs` and the two bench records. Every check reads that copy.
  - The study directories and `examples/mt_fieldops.rs` in the working tree were identical to HEAD (`git diff --quiet`).
- **The working tree was dirty.** `benchmarks/records/20261111T0551Z-mt-fieldops-v3.json` and `…T0800Z-mt-rho-start-fix.json` had uncommitted changes, so they are outside the validated set. The data rows, `ms` aside, are deterministic.
- **Measured binary:** `target/release/examples/mt_fieldops`.
  - sha256 `773fb2ca…bfd6`, equal to `bin_sha256` in all 4488 rows. The rows record commit `e821c5fbaf`, which has the same `mt_fieldops.rs` as `75dbc70b36` and HEAD (file sha256 `1f1b86a1…2dd2`).
  - I copied it to scratch and did not rebuild it in place.
- **Validator builds**, each a standalone crate from `git show 70d0b40bc3:examples/mt_fieldops.rs`. Toolchain: rustc 1.97.0, rand 0.8.8, rand_chacha 0.3.1, serde_json 1.0.151 with `arbitrary_precision`, the versions in `Cargo.lock`.
  - a pristine build;
  - an instrumented build (`scripts/patch_dbg.py`, no counted operation added);
  - a sensitivity build with two extra arms (`scripts/patch_ks.py`).
- **Host:** local, 4 cores, Linux 6.18.44, Python 3.13 with numpy. Counts only; no wall-clock figure is used.
- **Scratchpad:** shared with the producing session. I used only files I created under its `v3val/` subdirectory.
- **HEAD moved during validation**, from `70d0b40bc3` to `a9907c9254` (A-F13). Neither later commit touches the study directories or `examples/mt_fieldops.rs`, and neither is validated here.
  - `866d8b9d0a` records the bench command, metrics and raw hashes. The hashes match this certificate, and the records stay open with `uploaded: false`.
  - `a9907c9254` points the leaderboard's multi-target section at the v3 ic5 rows, to be labelled "validated" from `benchmarks/20261111-mt-fieldops-v3/VALIDATION.json`. That directory does not exist yet. This report does not support a validated vs-rho row (A-F1).
- **Files written:** only the files in this `validation/` directory and in `../../mt_rho_start_fix_20261111/validation/`. Nothing was committed or pushed, and no `VALIDATION.json` was written.

## 1. Dispositions

| # | requirement | disposition | evidence |
|---|---|---|---|
| 1 | Cost accounting of ic5: field, sqrt, hash, mod-n LA all counted; no free information; consistent with ic4, rho*, bsgs2 | **pass** | §3 |
| 1b | Restart fix: correct, no free work for rho, does not hurt rho | **pass** (correct, counted). The A1 wording is inaccurate: not monotone per run, and the chained starts add a 1/64 repeat rate | §3, §6 |
| 2 | Replay from argv; counts match exactly | **pass**: 4488/4488 (instrumented build), 336/336 nonce sample (pristine build), 72/72 nonce sample (producer binary), 160/160 tune rows | §4 |
| 2b | Independent EC verification of recovered logs | **pass**: 108 runs (every arm, every cell), 774,144 targets, 0 failures. Negative control caught in 7/7 arms | §4 |
| 3 | Own recompute of medians, CIs and decisions from runs.json | **pass (exact)**: 540/540 summary entries bit-identical; decisions identical; numpy bootstrap 0 disagreements; RESULTS table 84/84; noverify 36/36 | §5 |
| 4 | Protocol: freeze order, tune/holdout disjointness, frozen_ic5 before holdout, A1 legitimacy | **pass**; A1 wording amendments | §2 |
| 5 | Fairness of rhobest: kappa grid, standard multi-target rho | **fail as "strongest rho"**: the kappa grid brackets the optimum (0.7 wins 2310/2376), but the reference lacks Kuhn–Struik retention and distinguished points; measured retention makes all 36 decisions unranked | §6 |
| - | Reproducibility bindings | **partial**: commit and binary bound per row; no dirty flag or toolchain in rows; bench record open without command, raw or metrics at the commit | §7 |
| - | Reporting contract | **partial**: one table, one unit, ratios, verification disclosed; "strongest rho" wording, phases only in prose, two prose numbers from single runs, artifact-store statement false, no ECC2K-130 line | §7, §8 |

The findings (A-F1 to A-F13) are in `validator-report.json` under `findings`. A-F1 blocks.

## 2. Protocol and freeze order (check 4)

| commit | time (Z) | content |
|---|---|---|
| `f71b42cc4f` | 05:49:38 | PROTOCOL.md, run.py, analyze.py, 48 instances + MANIFEST, the ic5 arm in `examples/mt_fieldops.rs` (plus the p < 2^60 assert; no other non-ic5 line changed) |
| `79b405304f` | 05:51:25 | tune rows (160; commit f71b42cc4f, binary 58bc395a), `frozen_ic5.json` (mu 1.0, k 4) |
| `c400640e87` | 05:51:30 | bench record opened (commit_dirty false) |
| `75dbc70b36` | 08:00:17 | rho restart fix (five hunks, rho family only), amendment A1 (appended; no earlier PROTOCOL line changed), Study B protocol |
| `e821c5fbaf` | 08:00:52 | bench record for Study B; **this is the commit recorded in every holdout row** |
| `70d0b40bc3` | 08:27:52 | holdout rows, runs.json, summary, analysis, noverify, RESULTS, figures |

- **Freeze:** run.py, analyze.py, the instances and frozen_ic5.json are unchanged from their freeze commits to HEAD.
- **Tune recompute** (`scripts/tune_check.py`):
  - The frozen objective (mean over 16 configurations of cost(c_I = 10, h = 0.1) relative to the best grid point) gives mu = 1.0, k = 4 at 1.0043, then k = 3 at 1.0139. This matches `frozen_ic5.json`.
  - Tune uses idx 0, seeds 101/102; the holdout uses idx 1..11, seeds 21/22. The two are disjoint by index, seed and curve (p, j).
  - The tune CPU time (147 s) fits the two minutes between the protocol and tune commits, and the tune rows bind the protocol commit.
- **Amendment A1:**
  - The defect is real. At 32 b, idx 1, T = 1024, kappa 0.7, `db_target` = 1,468,007 points. The fixed-bank triple sums allow at most 45,760 × 32 = 1,464,320 phase-1 points, so the run could not finish.
  - Six rows were written before the hang, which matches the job order (ic5, ic4, then the four arms at kappa 0.5).
  - The fix touches only the rho-family restarts. The ic5 parameters, instances, seeds and decision rule are unchanged, and the rerun regenerates those six configurations deterministically.
  - So A1 is legitimate. Its rationale has two false statements (A-F3):
    - "every rho arm cheaper or equal" fails per run: in Study B, 3.5% to 15.7% of runs per arm became dearer, by up to 4.3%.
    - "no repeated walks" is false: chained starts repeat the previous walk with probability 1/64 per restart (§6).

## 3. Cost accounting (check 1)

ic5 (lines 542–793) was read in full against ic4, the rho arms and bsgs2:

- **Factor base:** the smallest-x points, with counted x³ + ax + b and a counted Tonelli–Shanks square root, as in ic4.
- **(k−1)-term full points:** `padd_with_sibling` per pair, I + 4M + 2S. At 32 b, T = 1024, m = 48: 35,720 inversions = C(48,2) + 2·C(48,3).
- **Table:** one `xpair` (I + 2M + 2S) and 2 hash inserts per pair of x-classes. At 32 b: 778,320 = 4·C(48,4) pairs and 1,556,640 hashes; 1,555,415 distinct classes.
- **Relations:**
  - trials x(V ± W_k) cost an `xpair` plus 1 hash per class looked up;
  - each table hit adds 1 hash for the dedupe and 2 counted `padd` for the sign;
  - incremental elimination mod n counts every product as M and every inverse as I (one run at 32 b, T = 1024: 4,577 M and 48 I);
  - back substitution counts its products.
- **Shared progressions:** W_k and V are counted additions, and every scalar multiplication is counted.
- **Targets:** each trial x(Q_t ± W_k) costs an `xpair` plus 1 hash per class. A hit adds 2 counted `padd` (`entry_pt`, `resolve`). The log is a ±sum of known factor-base logs (≤ 4 mod-n additions, uncounted like every addition in the harness).
- **No free information:**
  - the planted logs `xs` are used only to plant Q_t (an uncounted field copy, the same for every arm) and in the final check `found == xs mod n` (lines 270–285, 1243);
  - no arm reads the planted logs.
- **Verification:** a counted double-and-add per target, identical across arms (31.4–47.6 GAE/target at c_I = 10).
- **Uncounted in every arm:** F_p and mod-n additions, subtractions, negations and comparisons. The source header discloses this.
- **Same conventions in the other arms:** bsgs2 and ic4 use the same `xpair` and hash conventions. The rho arms count every step, sibling, restart and hit recompute. The restart fix (`git diff f71b42cc4f 75dbc70b36`) replaces two additions per restart with one counted addition on a running start. Its bookkeeping is right: every target verifies, and 774,144 targets were verified independently.
- **ic5 phase shares** (median, c_I = 10, h = 0.1):

| cell | table | targets | verify | relations + LA | combos |
|---|---|---|---|---|---|
| 22 b, T = 1024 | 30.8% | 31.5% | 32.6% | 0.95% | 3.6% |
| 22 b, T = 16384 | 15.0% | 19.6% | 64.1% | 0.08% | 1.2% |
| 32 b, T = 1024 | 48.4% | 44.5% | 2.4% | 2.46% | 2.1% |
| 32 b, T = 16384 | 45.9% | 43.4% | 9.1% | 0.21% | 1.4% |

## 4. Replay and certificates (check 2)

- **Replay:** `scripts/replay.py` compares every field the binary emits except `ms`: all counters, every phase snapshot and every arm extra.
  - The instrumented build replayed **all 4488 holdout rows** with 0 mismatches. Phase snapshots are monotone and end at the totals.
  - The tune rows replayed 160/160 (ic5 is unchanged since the tune binary).
  - Two nonce-drawn samples used nonce `7bc8adf926d31ea205a47b36172336e8` (`os.urandom`, drawn after the freeze):
    - the pristine source build replayed 336 rows (4 per arm × cell; 84 at 32 b) with 0 mismatches;
    - the producer's binary replayed 72 rows (17 at 32 b) with 0 mismatches.
  - `runs.json` is an exact projection of `rows.jsonl`, and its `source_sha256` matches.
- **Certificates:** the instrumented build dumps (t, planted x_t, recovered log, Q_t). My own Python checks each target: Q_t on the curve, 0 ≤ log < n, log = x_t, and [log]G = Q_t. It uses projective coordinates, a Montgomery ladder and a Fermat inversion, and shares no code with the harness. It agrees with an independent affine implementation on 300 random multiples.
  - Sample: one run per (arm, bits, T), three for ic5 = 108 runs. All 108 pass (774,144 targets).
  - Planted target lists are identical across the arms of each configuration.
- **Negative control:** with `found[0]` corrupted, the binary reports ok = targets − 1 and my checker flags exactly target 0, in all 7 arms.
- **Instances** (`scripts/instances_check.py`):
  - 48/48 valid: p and n prime, nonsingular, G on the curve, [n]G = O, n in the Hasse interval with cofactor 1, fixture point = [known_log]G, MANIFEST hashes match, a and b nonzero.
  - Repository scan for freshness: no (a, b, p) reuse and no (p, n) reuse under `research/` or `docs/curves`.

## 5. Recompute (check 3)

`scripts/recompute.py` is my own code and does not import `analyze.py`. It computes, from `runs.json`:

- cost per target = [(c_I I + M + S)/(c_I + 3) + h(H + L)]/T;
- rhobest = min over the 12 rho runs, and bsgs2 = min over 3 mf, per configuration and price;
- the median of 22 per-configuration ratios;
- CIs by a re-implementation of the frozen procedure (B = 4000, `random.Random(1)`) and by an independent numpy bootstrap (B = 20000, seeds 7 and 2026).

Result:

- **Summary:** 540/540 `summary.json` entries match. The median relative difference is 0, the frozen-procedure CI difference is 0, and the ic/ref medians and win counts are equal.
- **Decisions** (the numpy bootstrap gives the same decisions):

| comparison | cheaper | unranked | dearer |
|---|---|---|---|
| ic5 vs rhobest | 36 | 0 | 0 |
| ic5 vs bsgs2 | 0 | 0 | 36 |
| ic5 vs ic4 | 36 | 0 | 0 |
| ic4 vs rhobest | 7 | 7 | 22 |
| ic4 vs bsgs2 | 0 | 0 | 36 |

- **RESULTS.md main table:** 84/84 values match (four ranges × 12 cells, three decisions × 12 cells). The noverify table matches 36/36 lines × 3 values.
- **Global ranges:** ic5/rhobest 0.848–0.982, ic5/bsgs2 1.112–1.491, ic5/ic4 0.473–0.992, ic4/rhobest 0.944–2.060.
- **Prose numbers from a single run** (A-F5):
  - "about 61 relations" — the cell has 48 to 111, median 53.5;
  - "1396 measured vs 1380 predicted" — the cell has 1317 to 1495, median 1389.

## 6. Fairness of the rho reference (check 5)

**Kappa grid.** kappa 0.7 wins 2310 of 2376 settings, which is the first-order optimum sqrt(1/2) of the anchored design. The grid brackets the optimum.

**Residual overheads after the fix.** The instrumented build measured these with every counter unchanged:

- One addition per 32-step restart whose start point is neither stored nor looked up. This is 2.2% to 2.6% of rho2x's cost, and 2.5% to 3.0% of rho's.
- Repeated phase-2 lookups: 1.5% to 3.5% of rho2/rho2x lookups. Two causes:
  - sibling self-duplicates;
  - a duplicate the chained starts introduced: with probability 1/64 the increment B_k equals the previous walk's first arm, and the new walk retraces the old one. Plain rho at 32 b shows 1.5%, which is the 1/64 × 31/32 rate.
- Phase-1 walks keep stepping along stored paths after a merge: 0.519 to 0.626 steps per new point for rho2x, against about 0.5.

With the first two removed from every rho run, ic5 vs rhobest is cheaper in 30 of 36 and unranked in 6, all six at c_I = 30 (largest median 1.011).

**Kuhn–Struik retention (measured).** In standard multi-target rho, every point of a solved target's walk has a known log and is kept for the later targets. The anchored harness never does this, so its walk cost is about sqrt(2nT) against about sqrt(nT) with retention.

- `rho2xks` is the harness's rho2x plus one counted hash insert per retained point. Everything else is unchanged, including the restart overhead.
- It ran on all 264 holdout configurations at kappa 0.05/0.1/0.25/0.5: 1056 runs, all targets verified.
- `ic5ks` gives ic5 the same retention.

Table at c_I = 10:

| bits | T | ic5/rhobest h = 0.1, as measured | same, restart + repeats removed | ic5 / (rhobest ∪ rho2xks), h = 0 / 0.1 / 1 | 5a with rho2xks (c_I 3/10/30) | ic5 / DP-rho, h = 1 (**estimate**) |
|---|---|---|---|---|---|---|
| 22 | 1024 | 0.947 [0.939, 0.954] | 0.974 [0.968, 0.982] | 1.171 / 1.106 / 0.949 | unranked ×3 | 1.49 |
| 22 | 4096 | 0.953 [0.951, 0.958] | 0.979 [0.977, 0.984] | 1.124 / 1.079 / 0.947 | unranked ×3 | 1.30 |
| 22 | 16384 | 0.957 [0.956, 0.960] | 0.983 [0.982, 0.986] | 1.081 / 1.056 / 0.944 | unranked ×3 | 1.15 |
| 24 | 1024 | 0.954 [0.935, 0.956] | 0.986 [0.966, 0.987] | 1.218 / 1.131 / 0.965 | unranked ×3 | 1.67 |
| 24 | 4096 | 0.944 [0.936, 0.947] | 0.972 [0.963, 0.976] | 1.157 / 1.093 / 0.946 | unranked ×3 | 1.46 |
| 24 | 16384 | 0.949 [0.948, 0.951] | 0.974 [0.973, 0.976] | 1.110 / 1.068 / 0.942 | unranked ×3 | 1.28 |
| 28 | 1024 | 0.926 [0.917, 0.939] | 0.962 [0.954, 0.976] | 1.260 / 1.144 / 0.947 | unranked ×3 | 1.93 |
| 28 | 4096 | 0.917 [0.913, 0.924] | 0.951 [0.947, 0.958] | 1.223 / 1.120 / 0.935 | unranked ×3 | 1.78 |
| 28 | 16384 | 0.922 [0.919, 0.926] | 0.953 [0.950, 0.956] | 1.176 / 1.095 / 0.934 | unranked ×3 | 1.60 |
| 32 | 1024 | 0.933 [0.923, 0.942] | 0.972 [0.961, 0.980] | 1.293 / 1.155 / 0.963 | unranked ×3 | 2.11 |
| 32 | 4096 | 0.910 [0.903, 0.916] | 0.947 [0.940, 0.952] | 1.252 / 1.127 / 0.939 | unranked ×3 | 1.99 |
| 32 | 16384 | 0.902 [0.900, 0.905] | 0.937 [0.934, 0.940] | 1.219 / 1.106 / 0.929 | unranked ×3 | 1.88 |

- **h = 0:** across all c_I, ic5 is 1.069 to 1.337 times the retention reference, and every 95% lower bound is above 1 (36/36).
- **h = 0.1:** 1.046 to 1.181, every lower bound above 1.
- **h = 1:** 0.908 to 0.975. The retention arm pays a hash insert per stored point, so the anchored rho2x stays the reference there.
- **Section-5a decisions:** all 36 are unranked, with or without the verification phase, and with the restart cost removed.
- **The retention arm is not fully tuned.** kappa 0.05, the smallest value tried, was best in 181 of 264 configurations, so the measured gap is conservative.
- **ic5 cannot catch up with the same rule.** ic5ks is 0.980 to 1.000 times ic5. Its trial classes Q_t ± W_k come from one shared progression, so targets collide only along differences (k − j)D.

**Distinguished points (estimate, not measured).** `scripts/dp_estimate.py` models a Kuhn–Struik distinguished-point rho on {P, −P}:

- sqrt(n/T) + θ steps per target at 1 GAE each, with θ optimised;
- 2 hash operations per θ steps;
- the measured verification phase added;
- fruitless cycles not charged.

At h = 1 it estimates ic5 at 1.14 to 2.17 times that rho. With it, ic5 would be dearer at every h. ecbench already implements this method as `rho.multitarget_pooled`, but it was not used here (AGENTS.md §12).

**What does hold.** ic5's lead over the anchored harness family is a constant factor of about one multiplication per two x-classes. That is the same generic lever bsgs2 uses: x-only paired trials against a shared progression. It is not specific to index calculus:

- The relation search and elimination cost 0.08% to 2.5% of ic5.
- A known-log factor base with the same table and trials (as ecbench's `ic.known_log_ksum_v1`) would cost 0.977 to 1.0005 times ic5 (`scripts/generic_ic5.py`, a model on the measured phases).
- bsgs2 stays 1.11 to 1.49 times cheaper than ic5.

## 7. Reproducibility and reporting

- **Rows:** every row binds commit `e821c5fbaf` and binary `773fb2ca…`. The rows carry no dirty-tree flag and no toolchain.
- **Bench record:** at the validated commit it is open, with no command, raw entries or metrics. The command is only in the uncommitted working tree: started 08:03:15Z, 1372 s.
- **Missing entry:** the `benchmarks/20261111-mt-fieldops-v3` entry that the record names does not exist.
- **Artifact store:** RESULTS.md says `rows.jsonl` is in the artifact store. It is committed to Git (3.16 MB, no `.artifacts.json`), against AGENTS.md §13. The tune rows (138 KB) are also over the 64 KiB data-file limit.
- **Isolation and harness:** local host, not isolab-eks; not ecbench. Limits says both.
- **Visual report:** REPORT.pdf and the figures exist, and `plot.py` reads only `summary.json`.

## 8. Required amendments to RESULTS.md (and the record)

1. **BLOCKING:** drop "the strongest rho" from the title, headline, section-5a bullet and the Q1 answer. Say instead: "the cheapest of 12 pre-registered runs of this harness's anchored, fixed-database rho family (no retention of solved walks, no distinguished points)".
2. **BLOCKING:** state next to the ic5/rhobest ratio that a standard multi-target rho is cheaper:
   - with Kuhn–Struik retention added to rho2x (validator, 1056 runs), ic5 is 1.07–1.34× that rho at h = 0 and 1.05–1.18× at h = 0.1, and cheaper only at h = 1 (0.91–0.98);
   - the 5a decision is therefore unranked in all 36 (cell, c_I);
   - a distinguished-point rho is estimated (not measured) to be cheaper at h = 1 too.

   Do not report a crossover against rho, at T = 1024 or anywhere else, until a Kuhn–Struik/DP rho arm has been measured under the frozen rule (ecbench `rho.multitarget_pooled`, or retention in this harness).
3. Report the residual reference overheads (restart cost 2.2–3.0%, repeated lookups 1.5–3.5%, phase-1 merge waste). Say that removing the first two leaves 30 cheaper and 6 unranked even against the harness family.
4. Amend A1. The fix is not monotone per run: 3.5%–15.7% of Study B runs are dearer, by up to 4.3%. Chained starts repeat a walk with probability 1/64 per restart.
5. Headline the validated result:
   - ic5 is cheaper than ic4, the multi-target IC baseline, in all 36 (cell, c_I), at 0.47–0.99;
   - it is class *engineering* under AGENTS.md §3;
   - keep the bsgs2 ratio (1.11–1.49, dearer everywhere) as the distance remaining to the best generic method in the harness.
6. State that ic5's lead over the harness rho is generic. The IC-specific phases are 0.08–2.5% of its cost, and a known-log k-sum table would cost the same.
7. Correct the single-run numbers:
   - "about 61 relations" → 48–111, median 53.5;
   - give the cell range for the trials check (1317–1495, median 1389, against 1380).

   Label "Why ic5 wins" as interpretation.
8. Correct "rows.jsonl (artifact store)":
   - push `rows.jsonl` (sha256 `46ddf9ec…62bd`) and `tune/rows.jsonl` through `tools/artifacts` with a manifest;
   - commit the bench record with its command, raw hashes and metrics;
   - add the `benchmarks/` entry.
9. Add one line saying ECC2K-130 / m = 83 is not engaged.

## 9. Validator scripts (`validation/scripts/`)

| script | role |
|---|---|
| `patch_dbg.py` | instrumentation: per-target dump, `MTFO_CORRUPT` negative control, restart cost vectors, phase-2 lookups and repeats, ic5 LA counters |
| `patch_ks.py` | sensitivity arms `rho2xks` and `ic5ks` (retention); scratch only |
| `replay.py` | replay from the recorded argv, comparing every emitted field |
| `eccheck.py`, `eccheck_lib.py` | independent EC certificate check (projective ladder; affine cross-check) |
| `negctl.py` | negative control |
| `instances_check.py` | instance validity, manifest hashes, tune/holdout disjointness |
| `recompute.py` | own recompute of all medians, CIs and decisions from runs.json |
| `results_check_v3.py` | RESULTS.md table, noverify.txt, prose numbers, phase shares |
| `tune_check.py` | tune objective and frozen choice |
| `sens.py` | overhead counterfactuals and the retention reading under the frozen rule |
| `run_ks.py`, `run_ks_cfg.py` | the retention runs |
| `dp_estimate.py` | distinguished-point rho model (estimate) |
| `theory.py`, `generic_ic5.py` | first-order models; known-log factor-base counterfactual |
| `run_all*.sh`, `findings.py`, `make_artifacts.py` | orchestration and artifact assembly |

Two compact evidence files are in this directory:

- `ks_runs.json`: the 1584 retention-arm runs, with counters and the verification-phase vector.
- `rho_instrumentation.json`: per rho-family row, the restart cost vectors and counts, the phase-2 lookups and the repeated lookups.

The raw outputs (replay records, per-target dumps) stay in the validator scratchpad. Every number above can be regenerated from the committed rows with these scripts.
