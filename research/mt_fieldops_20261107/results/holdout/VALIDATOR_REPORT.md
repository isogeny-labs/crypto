# Validator report: multi-target IC in field operations (holdout, 2026-11-07)

Validator: independent `validator` subagent (Opus 5.5), 2026-11-07. Standing of the work after this report:
the measurement is **not** an `invalid_measurement`. Its central numbers reproduce exactly. Two of its
comparative claims need amendment (sections 4 and 6). It is still pre-gate: there is no ecbench replay
certificate and no `VALIDATION.json`, and the counters are the producer's own.

## 0. What was validated, and from where

- **Frozen artifact set: commit `a5957000f8d0963cc48daf376ff4efa099208d9f`** (branch `aburan28/eloquent-bell-2jerrf`),
  extracted with `git archive a5957000f8` into a scratch directory. Every check below reads that copy.
- **The working tree moved during validation.** HEAD became `2c27049f64`: `de42468d2d` (github-actions bot) moved
  `results/holdout/rows.jsonl`, `results/tune/rows.jsonl` and `ratios.png` to the artifact store, and `2c27049f64`
  ran rustfmt on `examples/mt_fieldops.rs`. I checked both changes against the frozen commit:
  - All 14 evidence files, and the restored git-ignored copies in the working tree, hash identically to the
    `a5957000f8` blobs. Holdout `rows.jsonl` is `a0946510...46fc8b`, tune `rows.jsonl` is `5a353da9...87d983b`
    and `ratios.png` is `1256a69a...29f0a9`. These equal the `.artifacts.json` manifests written by `de42468d2d`.
  - The rustfmt commit is **not whitespace-only**, as its message says, but it is semantically identical. With
    all whitespace removed, the only differences are 8 added trailing commas, and the sources are equal after
    those are removed. I validated the pre-rustfmt source in `a5957000f8`. Its blob `22a3e5ae` is unchanged
    since `449d6f14e6`.
  - While this report was being written, HEAD moved again to `3434954e47` ("clippy manual_div_ceil"). It
    changes `(q + 1) / 2` to `q.div_ceil(2)` in Tonelli-Shanks; q is odd there, so the value is the same. The
    measured binary (`97d5e32e...`) corresponds to the `a5957000f8` source, not to these later edits. A rebuild
    from HEAD would have a different SHA and should be re-replayed before anyone cites it as the measured
    binary.
- Validator environment: local host, 4 cores, Linux 6.18.44; rustc 1.97.0 / cargo 1.97.0; Python 3.13.18,
  numpy 2.5.3. Counts are deterministic, so host isolation does not affect them, and no wall-clock figure is
  used. I made no edits to any repo file other than this report, did not commit, did not rebuild
  `target/release`, and changed no evidence or status.

## 1. Dispositions

| # | claim / requirement | disposition | evidence (section) |
|---|---|---|---|
| 1 | Freeze order: PROTOCOL.md, run.py, analyze.py and the harness committed before any data; `frozen.json` committed before the holdout; `frozen.json` = argmin of the frozen tune objective | **validated** | §2 |
| 2 | Replays: rows reproduce from their argv with the recorded binary; binary SHA in rows = built binary | **validated** (all 1320 holdout + 135 tune rows, not a sample) | §3 |
| 3 | Counting fairness: all I/M/S/H counted in every arm, no uncounted work, no target leakage, correct shared-inversion arithmetic, `rho` = `ic_batch_rho.rs` | **validated with amendments**: no leakage and no material uncounted work found; formulas correct; `rho` is the same algorithm. Two asymmetries matter for the conclusions: the `rho2` control computes a y it does not need, and the `rho` reference is a known-weak variant. The verification cost is identical across arms but undisclosed. | §4 |
| 4 | Correctness: recovered logs are true discrete logs; `ok == targets` is meaningful | **validated** | §5 |
| 5a | Headline table (RESULTS.md) and analysis.txt reproduce from rows.jsonl | **validated** (exact) | §6 |
| 5b | "IC4 cheaper than `rho` in 17/18 (cell, I/M); unranked at 24 b, T = 1024, I = 3M; pre-registered headline narrowly fails" | **validated** against the pre-registered `rho` arm; **amendment**: against a waste-free sequential-database rho it is 14/18 | §6, §4.5 |
| 5c | vs `rho2`: "cheaper at T = 4096 and 16384 for I = 3M ... dearer at T = 1024; unranked otherwise" | **validated with amendments**: "dearer at T = 1024" is false for 22 b at I = 3M (unranked). With an x-only-sibling rho2 (the lever as IC uses it), I = 3M changes in 3 of 6 cells, all against IC. | §6, §4.4 |
| 5d | vs `bsgs2`: "IC4 dearer in all 18 combinations, 1.18x to 1.80x" | **validated** as measured; **amendment**: 1.50x to 2.05x once the verification constant shared by all arms is removed | §6, §4.3 |
| 5e | "Distance remaining": rho "passed, 0.62x to 0.83x at h <= 0.1"; rho2 "0.86x to 1.11x"; bsgs2 "1.18x to 1.80x" | **validated with amendments**: "passed" is pass/fail wording the success criterion forbids, and it contradicts the failed pre-registered headline. The rho2 range covers h = 0 only; across the band it is 0.86x to 1.19x. | §6 |
| 5f | "What changed": rho 142.6 here vs 142 there; IC4 about 0.69x IC v3 (104 vs 151); "the same lever was offered to both generic arms and is counted the same way for all four" | numbers **validated** (one cell, I = 3M, h = 0, cross-instrument); the lever sentence is **refuted as worded** | §4.4, §6 |
| 6 | Reproducibility bindings, reporting contract, gate | **partial**: listed in §7 | §7 |

Overall disposition in the validator vocabulary: **validated (with amendments)** for the measured tables and the
pre-registered ranking against `rho` and `bsgs2`. The rho2-based and "distance remaining" wording needs the
amendments in §8 before it is cited. This is not leaderboard-eligible (§7).

## 2. Freeze order (check 1)

- `git log` / `git ls-tree` (evidence: the commit trees):
  - `449d6f14e6` (06:17:38Z) contains only `PROTOCOL.md`, `analyze.py`, `run.py`, plus `examples/mt_fieldops.rs`.
  - `91766533a8` (06:26:10Z) adds `frozen.json` with `results/tune/{rows.jsonl, analysis.txt}`.
  - `a5957000f8` (06:28:55Z) adds the holdout rows.
  - The blobs of PROTOCOL.md (`d32aa0a2`), run.py (`8e411242`), analyze.py (`df8d6eaa`) and mt_fieldops.rs
    (`22a3e5ae`) are identical in all three commits, so protocol, runner, analysis and harness were not edited
    after the data.
- The reflog is linear (`commit`, `commit`, `commit`, then a fast-forward and the rustfmt commit), with no amend,
  reset or rebase. Every tune row records `commit = 449d6f14...` and every holdout row records
  `commit = 91766533...`, so the holdout ran with `frozen.json` already committed.
- The binary's mtime (06:25:36Z) falls between the protocol and tune commits. The source build in §3 shows that
  the binary behaves exactly as the committed source.
- **Tune objective recomputed independently** (my code, formula from PROTOCOL "Cost unit"). The data has 9
  configurations x 15 grid points = 135 rows, all verified. The mean relative cost at c_I = 10, h = 0.1 has its
  argmin at (mu, g) = (1.0, 2.0), score 1.0000, followed by (1.0, 1.5) at 1.0139. This matches `frozen.json` and
  `results/tune/analysis.txt` to 4 decimals.
  (1.0, 2.0) is the per-configuration best in all 9 tune configurations at every (c_I, h) in the band, so the
  choice does not depend on the objective's pricing point.
- Caveats:
  - Commit times, reflog and recorded commit fields are producer-controlled. They are consistent, but they are
    not third-party timestamps; the branch was pushed only after `a5957000f8`.
  - g = 2.0 is the upper edge of the grid. This is disclosed and conservative for IC.
  - The holdout instances are the ones earlier IC versions were designed on (disclosed).

## 3. Replays and binary identity (check 2)

- `sha256(target/release/examples/mt_fieldops)` = `97d5e32ef99d915e3c7c55ccae19061f0d14c0c2d7e67a7d746b0ad7fca820a0`.
  That equals `bin_sha256` in all 1320 holdout and all 135 tune rows.
- **Full replay, not a sample:** I re-ran every row's argv (1320 + 135) with that binary and compared every output
  field except `ms`: I, M, S, H, L, ok, all phase snapshots, and all arm extras such as m, table, relations,
  trials, db and giants. **0 mismatches.**
- **Source binding:** I built `a5957000f8:examples/mt_fieldops.rs` in a standalone scratch crate. The deps were
  pinned to the repo's Cargo.lock versions (rand 0.8.8, serde_json 1.0.151 with the same features), the only
  changes were the validator instrumentation listed in §5, and the binary SHA was `9559c012...`. It reproduces
  all 1455 rows exactly (0 mismatches), so the measured binary implements the committed source.
- The argv of every row matches the protocol design: arm, instance `W{pb}-{idx}`, T, seed, param, and g for ic4.
  The grid is complete: 6 cells x 11 instances x 2 seeds = 132 configurations, each with 1 ic4 (1.0, 2.0),
  3 rho, 3 rho2 and 3 bsgs2 runs, and no duplicates. Each row's n and p match its instance file. The instances
  were committed in `6c1f3094b7` (before this work) and are unchanged.

## 4. Counting fairness (check 3)

I read `mt_fieldops.rs` line by line, every arm, at `a5957000f8`.

### 4.1 What is counted, per arm

- Every `inv`, `mul` and `sqr` goes through the counted `F` and every hash insert or lookup calls `f.hash()`, in
  every arm and phase:
  - ic4: factor-base search including counted Tonelli-Shanks, table, W progression, E and V points, relation
    search, recomputation on hit, signed-graph solve (L), target trials and resolve.
  - rho and rho2: bank of 64 scalar multiplications, restarts, steps, inserts and lookups.
  - bsgs2: baby lists, `ugen`, sum-set table, `spt`, giants, `baby_pt` and resolve.
  - All arms: verification.
- The phase snapshots sum to the totals in every row (asserted in my phase script).
- **Uncounted work found:**
  1. ic4 `seen_pairs.insert` (phase 1) is a hash operation without `f.hash()`. Instrumented count: 432 to 1450 per
     run, about `relations`. Priced generously at h (1.5 x relations), it changes no ratio by more than 0.003 and
     no ranking.
  2. F_p additions, subtractions and negations are uncounted in every arm. This is disclosed in the source
     header but not in RESULTS. Per point, `padd` and `xpair` use a similar handful, so this is immaterial.
  3. Mod-n log bookkeeping is uncounted in every arm and not priced in the §5a band:
     - rho and rho2 do one mod-n addition per step (rho2 two);
     - bsgs2 does a mod-n product per giant (`hl`), which is a coding choice and not needed;
     - ic4 does a few per W extension and relation, and none per trial.

     Pricing this would penalise rho and rho2 more than IC, so its omission is conservative for the IC-vs-rho
     claims.
- **No planted-target leakage:** `xs`, `plant` and `f0` are used only to plant the targets with uncounted
  `f0.smul` (instance generation) and in the final check (lines 268-273, 706). No arm reads `xs` or a planting
  RNG. The solver RNG (`rng`) is seeded independently of the targets.
- **No hidden y recomputation:**
  - ic4's x-only table and trials recompute full points only on a hit (`entry_pt`, `resolve`), and both are
    counted.
  - rho and rho2 store y in the database (memory, uncounted for every arm by protocol).
  - bsgs2 recomputes with the counted `baby_pt` and `resolve`.

### 4.2 Shared-inversion arithmetic

- `xpair` gives x(P+Q) = lambda_+^2 - x1 - x2 with lambda_+ = (y2-y1)/(x2-x1), and x(P-Q) with
  lambda_- = (-y2-y1)/(x2-x1), at a cost of I + 2M + 2S.
- `padd_with_sibling` gives the full P+Q and the full P-Q, at a cost of I + 4M + 2S.
- Affine doubling uses lambda = (3x^2 + a)/(2y).
- All of these are correct short-Weierstrass formulas. I also checked them numerically: transliterated into
  Python and compared with (i +- j)G and 2iG from my own Jacobian-coordinate Montgomery ladder, on about 15,000
  random pairs (5,000 drawn per instance) across W22-3, W24-5 and W25-9, there were **0 mismatches**.
- The baby set covers 1..Mx with Mx = (2u+1)(v-1) + u, and paired giants cover 2(2Mx+1) logs per stride. The
  sign resolution in `resolve` is correct: Q + s_w W = eps D gives log Q = eps dl - s_w w.

### 4.3 The verification constant (undisclosed in RESULTS)

The counted double-and-add check of every recovered log is part of every arm's I/M/S. It is **identical across
the 10 runs of each configuration** (checked: 0 of 132 configurations differ), because it depends only on the
recovered logs, which equal the planted ones. It costs **33.4 GAE/target (22 b), 36.7 (24 b) and 38.3 (25 b)**,
which is **15% to 64% of the reported GAE/target** (64% for bsgs2 at 24 b, T = 16384). This is consistent with
the earlier campaign, where `ic_batch_rho.rs` also priced verification. But it is not a cost of any attack, and
adding the same constant to both arms pulls every ratio toward 1. Solver-only ratios, with the verify phase
removed from every arm (validator computation from the recorded phase snapshots):

| vs | median ratio range over (cell, I/M, h) | rankings |
|---|---|---|
| rho | 0.476 to 1.007 (reported: 0.623 to 1.006) | unchanged (17/18) |
| rho2 | 0.770 to 1.212 (reported: 0.861 to 1.191) | unchanged |
| bsgs2 | **1.498 to 2.051** (reported: 1.177 to 1.802) | unchanged (dearer, 18/18) |

The protocol discloses that verification is counted; RESULTS.md does not say that a third to two thirds of each
reported cost is this shared constant.

### 4.4 Asymmetry: the `rho2` control is given a costlier form of the lever than IC uses

- ic4's lever is x-only: two x-classes for I + 2M + 2S.
- `rho2` needs the full main point R + A, because the walk continues from it, but it also computes the
  **sibling's y** (`ym`, +1M per step), and it uses that y only to resolve the sign on the rare hit.
- An x-only sibling costs I + 3M + 2S per step instead of I + 4M + 2S. That saves 11% of the step cost at
  I = 3M, 6% at 10M and 3% at 30M, for at most two counted recomputations per target. The source's own comment
  (line 527, "y or u64::MAX for a sibling without y") anticipates that design.
- I implemented it as a scratch validator control, `rho2x`. It is identical to rho2, except that the sibling is
  x-only and a hit on an x-only entry recomputes the needed point in full with a counted `padd`. I ran it on all
  132 configurations x 3 kappa values (396 runs, all targets verified).

| I/M | 22 b T1024 | 24 b T1024 | 24 b T4096 | 24 b T16384 | 25 b T4096 | 25 b T16384 |
|---|---|---|---|---|---|---|
| 3M, vs rho2 (as produced) | unranked | dearer | **cheaper** | cheaper | **cheaper** | cheaper |
| 3M, vs rho2x | **dearer** (1.047/1.057/1.105) | dearer | **unranked** (0.967/0.975/1.013) | cheaper | **unranked** (0.968/0.977/1.017) | cheaper |
| 10M and 30M, vs rho2x | dearer | dearer | unranked | cheaper | unranked | cheaper |

(medians at h = 0/0.1/1; ranks under the pre-registered rule, 95% bounds as the union over three bootstrap seeds of 20,000 resamples)

- Against the x-only-sibling rho, IC4 is **cheaper only at T = 16384, dearer at T = 1024 and unranked at
  T = 4096, at every I/M**. The IC4 / rho2x range is 0.909 to 1.198.
- The RESULTS sentence "the same lever was offered to both generic arms and is counted the same way for all four"
  is therefore not accurate:
  - counting is uniform (one field layer), but the lever is not implemented the same way;
  - `rho`, the headline reference, is not offered the lever at all (by design).
- The sentence "IC4's remaining edge over rho2 is that its target trials need no y-coordinate ..." is partly an
  artifact of rho2 computing the sibling's y.

### 4.5 `rho` = `ic_batch_rho.rs`, and that reference is a known-weak rho

- **Same algorithm (validated):**
  - Constants: ARMS = 32, RESTART = 32, BANK = 64.
  - Bank of random multiples of G, with the r-adding arms drawn from the bank.
  - Phase 1 restarts from 3-bank sums (2 additions); the step adds `arms[x mod 32]`; x -> (log, y) is stored
    with first-insert-wins.
  - Phase 2 restarts from Q + 2-bank sums (2 additions); lookups by x with the sign taken from y; verification
    is included.
  - Differences that do not change the algorithm:
    - the cost unit: exact field operations here, against group additions with a scalar multiplication priced
      at ceil(1.5 bits) in `ic_batch_rho`;
    - RNG streams;
    - scalars drawn from 1..n rather than 0..n.
- **But this reference is weak.** The earlier campaign's validator (F2 in
  `research/ic_e222_m221_20261106/results/multi_holdout_v3/VALIDATOR_REPORT.md`) found its phase 1 wasteful and a
  sequential-database anchored rho 8% to 14% cheaper. It asked for that arm "before any statement about parity
  with generic multi-target attacks".
- This campaign added bsgs2 but not that arm, and RESULTS.md calls `rho` "the matched reference" without the
  caveat. I implemented it as the scratch control `rhoseq`: the phase-1 database is {G, 2G, ..., NG}, built with
  N counted additions and N hashes, and phase 2 is byte-identical to `rho`. All 396 runs verified. `rhoseq / rho`
  is 0.880 to 0.953.
- **IC4 is cheaper than rhoseq in 14 of 18 (cell, I/M) combinations, against 17/18 vs rho.** The 4 unranked ones
  are 22 b T1024 at I = 3M, and 24 b T1024 at every I/M, where h = 1 gives 1.011 to 1.061. The IC4 / rhoseq
  median range is 0.681 to 1.061. IC4 stays cheaper than rhoseq at T >= 4096 at every I/M.
- Both validator controls are sensitivity analyses, not pre-registered arms.

### 4.6 Parameter selection favouring the generic arms

The per-configuration best-of-3 kappa/mf selection ("favours them") turns out to have **no effect**. Fixing one
best parameter per cell gives the same medians to 3 decimals for rho, rho2 and bsgs2. The cost is smooth in the
parameter, so the oracle never picks a lucky draw.

## 5. Correctness and the meaning of `ok == targets` (check 4)

- The scratch build of the frozen source was instrumented in three ways:
  - (a) a stderr dump of (t, planted x_t, recovered log, Q_t) after verification;
  - (b) a counter for `seen_pairs`;
  - (c) an env-gated corruption of one recovered log, applied before the verifier.

  None of these touches the counters; §3 shows the build reproduces all rows.
- 24 runs were drawn at random (seed 20261107), one per (cell, arm). For them, my own Python code checks every
  target, **172,032 recovered logs**. It uses Jacobian coordinates and a Montgomery ladder, with no affine slope
  formulas, and Miller-Rabin primality.
  - **0** cases where found_t·G != Q_t, and **0** cases where x_t·G != Q_t.
  - Every instance is valid: p and n prime, G on the curve, n·G = O, nonsingular.
  - Every Q_t is on the curve.
- **Negative control:** with one recovered log corrupted, the binary's verifier reports `ok = targets - 1` in all
  24 runs. So `ok == targets` is a live check, requiring `smul(g, found) == Q_t` and `found == x_t`.
- Since no arm reads x_t (§4.1), `ok == targets` in all 1320 rows certifies 1320 complete, correct multi-target
  solves.
- Duplicate planted targets occur (for example 16376 distinct of 16384, as the birthday bound expects). No arm
  caches solved targets, so duplicates are neither exploited nor penalised differently.

## 6. Recomputed headline, ranking and wording (check 5)

My own script (numpy bootstrap, 3 seeds x 20,000 resamples, conservative union of bounds; cost formula from
PROTOCOL.md) reproduces:

- all 162 `summary.json` medians and the IC and reference GAE medians **exactly**, and the per-cell win counts
  exactly. My bootstrap bounds differ from the producer's by at most 0.0017;
- all 54 table lines of `analysis.txt` (to the printed precision), and every number in the RESULTS.md headline
  table;
- the §5a ranking, for all 54 (I/M, reference, cell) verdicts. No verdict depends on the bootstrap seed. The
  closest calls, for example 24 b T1024 I = 3M h = 1 vs rho at 1.006 [0.985, 1.022], keep their verdict under
  every seed;
- vs rho: cheaper in 17/18. The single unranked one is 24 b T1024 at I = 3M. vs bsgs2: dearer in 18/18, median
  1.177 to 1.802. The pre-registered "IC beats rho" headline fails, as RESULTS.md says.

Wording and number discrepancies in RESULTS.md (none is a fabricated number):

1. **vs rho2, "dearer at T = 1024"**: at I = 3M, 22 b T1024 is **unranked** (0.981/0.996/1.067; h = 0.1 interval
   [0.992, 1.003]). The statement holds for 5 of the 6 (T = 1024, I/M) combinations.
2. **"To a rho with the same arithmetic: 0.86x to 1.11x"** is the h = 0 range only. Across the pre-registered
   band it is **0.861 to 1.191** (1.125 at h <= 0.1). Against the x-only-sibling control it is 0.909 to 1.198
   (§4.4).
3. **"To the matched (anchored) rho: passed, by 0.62x to 0.83x at h <= 0.1"** has three problems:
   - "passed" is the pass/fail framing that the success criterion says not to use;
   - it sits beside the statement that the pre-registered headline failed;
   - it omits h = 1, where the band reaches 1.006.

   Suggested wording: "distance to the anchored rho: 0.62x to 1.01x across the band (0.62x to 0.83x at h <= 0.1);
   cheaper in 17/18 under the pre-registered rule; 14/18 against a sequential-database rho (validator)".
4. **"the same lever was offered to both generic arms and is counted the same way for all four"**: not accurate
   (§4.4).
5. **"IC4 dearer than bsgs2 by 1.18x to 1.80x"** is correct as measured, but the shared verification cost
   compresses the solver-only gap of 1.50x to 2.05x (§4.3).
6. **"What changed" numbers**: rho 142.6 GAE/target here against 142 there, IC v3 at 151, and the 1.03-1.07 and
   1.40-1.53 figures all match `research/ic_e222_m221_20261106/RESULTS_multi_target.md`. 104.1/151 = 0.689.
   This is a one-cell, I = 3M, h = 0 cross-instrument comparison: IC v3 was not re-counted in this field-op
   harness, and at I = 30M rho here is 139.6.
7. The pre-registered predictions are not compared with the outcome. PROTOCOL.md predicted:
   - IC4/rho 0.55 to 0.75 (measured 0.62 to 1.01);
   - IC4/rho2 0.85 to 1.0 (measured 0.86 to 1.19);
   - IC4/bsgs2 1.3 to 1.5 (measured 1.18 to 1.80).

   The rho2 prediction fails at T = 1024, and the rho upper end is missed.
8. The commit message of `a5957000f8` says "17/18 cells". There are 6 cells; it is 17 of 18 (cell, I/M)
   combinations. RESULTS.md itself is correct here.
9. `REPORT.pdf` page 1 is `RESULTS.md[:4200]`, but RESULTS.md has 4422 bytes, so the PDF drops the end of
   "Limits", including the "no independent validator yet" disclosure.

## 7. Reproducibility bindings, controls, reporting contract, gate

| requirement | status | evidence / gap |
|---|---|---|
| exact commit per row | pass | `commit` in all rows; equal to the freeze commits (§2) |
| binary hash per row = binary | pass | §3 |
| dirty-tree state recorded | **fail** | run.py records `git rev-parse HEAD` only. Mitigated: the frozen-source rebuild reproduces all rows (§3) |
| toolchain / host / environment recorded | **fail** | not in rows or RESULTS; the validator's are in §0 |
| seeds and commands | pass | argv per row; seeds from PROTOCOL |
| declared artifacts exist and hash | pass | §0; the manifests match the frozen blobs |
| large-file policy (AGENTS §13) | **fail at the frozen commit** | `results/holdout/rows.jsonl` (962,823 B jsonl) and `results/tune/rows.jsonl` (126,267 B) were committed with `git add` in `91766533a8` / `a5957000f8`. The bot moved them in `de42468d2d`; the blobs stay in history |
| reference arm and cheaper baseline priced | pass, with §4.4/§4.5 caveats | rho, rho2 and bsgs2 all priced at h in {0, 0.1, 1}; rho2 handicapped by the sibling's y; rho known-weak (earlier F2) |
| §5a band on all arms' unpriced work | partial | H and L priced for all arms; mod-n bookkeeping and F_p additions unpriced in all arms (§4.1, conservative for IC vs rho) |
| one table, one unit, ratio | pass | GAE/target, with ratio and correctness columns in analysis.txt |
| phases separated | **fail in RESULTS** | phases are in the rows only; see the table below |
| external fastest implementation / ecbench (AGENTS §12) | **not met** | own counters, no `ecbench.spec`, no `ecbench verify --replay` receipt (disclosed in Limits) |
| validation gate (VALIDATION.json, replay certificate, matched rho with band) | **not met** | no `VALIDATION.json`, no ecbench receipt; RESULTS correctly says pre-gate `[pcc:unverified]`, and no leaderboard or `benchmarks/README.md` row was added (checked: the commits touch only this directory, the example and a session log) |
| m = 83 / ECC2K-130 disclosure | not engaged | prime-field toy groups; no ECC2K-130 transfer is claimed |

Phase breakdown (validator computation), GAE/target at I = 3M, h = 0. Each entry is the median of that phase, so
entries need not sum exactly to the total. Parameters: rho and rho2 at kappa = 0.7, bsgs2 at mf = 1.0.

| cell | IC4 fb / table / relations / targets / verify = total | rho db / targets | rho2 db / targets | bsgs2 baby / targets | verify (all arms) |
|---|---|---|---|---|---|
| 22 b T1024 | 7.9 / 26.7 / 12.4 / 29.6 / 33.4 = 110.2 | 54.3 / 49.5 | 39.8 / 39.3 | 18.9 / 20.0 | 33.4 |
| 24 b T1024 | 12.2 / 53.2 / 33.3 / 57.1 / 36.7 = 192.3 | 104.6 / 99.0 | 77.0 / 76.1 | 37.7 / 38.6 | 36.7 |
| 24 b T4096 | 4.3 / 26.5 / 6.2 / 30.3 / 36.7 = 104.1 | 55.8 / 50.2 | 39.4 / 38.6 | 18.8 / 20.4 | 36.7 |
| 24 b T16384 | 1.5 / 13.2 / 1.2 / 16.4 / 36.7 = 69.0 | 33.1 / 25.5 | 21.3 / 20.3 | 9.4 / 11.3 | 36.7 |
| 25 b T4096 | 6.5 / 37.4 / 10.1 / 41.5 / 38.3 = 133.8 | 79.6 / 70.3 | 55.7 / 54.6 | 26.6 / 28.2 | 38.3 |
| 25 b T16384 | 2.3 / 18.7 / 1.9 / 22.0 / 38.3 = 83.2 | 48.4 / 35.7 | 30.3 / 28.0 | 13.2 / 15.2 | 38.3 |

The solve phase does no field operations. Its L count (for example 860 per run at 22 b T1024, 0.84 GAE/target
at h = 1) enters only through h. IC4's precomputation (fb + table + relations) is 16 to 99 GAE/target. IC4's
target phase alone is below bsgs2's baby + target cost in every cell, but its table plus target phase is above
it in every cell (for example 56.8 against 39.2 at 24 b T4096).

## 8. Required amendments before citation

1. Replace "passed" in "Distance remaining" with the distance across the full band (0.62x to 1.01x vs rho). Add
   the sequential-database sensitivity: 14/18, 0.68x to 1.06x. Cite the earlier F2 finding that the anchored rho
   is a weak reference.
2. Correct the rho2 sentence (22 b T1024 at I = 3M is unranked) and the rho2 range (0.86x to 1.19x across the
   band). Either add an x-only-sibling rho2 arm or disclose that rho2 computes the sibling's y. With an x-only
   sibling, IC4 is cheaper only at T = 16384.
3. Withdraw "the same lever ... counted the same way for all four" or rescope it to "every arm's field operations
   are counted by the same layer".
4. Disclose that verification (33 to 38 GAE/target, identical in every arm, 15% to 64% of each reported cost) is
   included. Report the solver-only distance to bsgs2 (1.50x to 2.05x) beside the measured 1.18x to 1.80x.
5. Separate phases in RESULTS (table above). Compare the outcome with the pre-registered predictions. Fix the
   truncated PDF. In RESULTS, record that the rows lack dirty-tree and toolchain bindings.
6. For the gate: add an ecbench spec and `ecbench verify --replay` receipt, a `VALIDATION.json`, and route data
   through the artifact store before the commit. Until then the work stays pre-gate.

## 9. Not checked

- Wall-clock behaviour: not used by the claims.
- A distinguished-point rho, other rho families, and arm-count or bank-size sweeps.
- Whether IC would improve with g > 2.0, which is outside the frozen grid.
- Montgomery batching of inversions is assumed, not implemented, for c_I = 3 in every arm.
- A re-count of IC v3 inside this field-op harness.
- isolab-eks, ecdb, research-kb and pcc ledger entries: none was available or engaged in this validation.

## 10. Reproduction (validator scratch, not committed)

The scripts are in the session scratchpad
`/tmp/claude-0/-home-user/aeefaead-5838-5329-9e0a-13b7d93fe240/scratchpad/`. That directory is ephemeral, so
the descriptions above are sufficient to rebuild them.

| script | purpose |
|---|---|
| `tools/replay.py`, `tools/replay_bin.py` | full replays |
| `tools/tune_check.py` | tune argmin |
| `tools/holdout_check.py` | tables and ranking |
| `tools/phases.py` | phase breakdown |
| `tools/eccheck.py` | independent EC verification and negative control |
| `tools/patch.py`, `tools/patch2.py` | instrumentation, `rho2x` and `rhoseq` arms on the frozen source |
| `tools/sens.py`, `tools/sens2.py` | sensitivity tables |

Outputs: `replay_*.json`, `eccheck.jsonl`, `rho2x_rows.jsonl`, `rhoseq_rows.jsonl`, `sens.txt`, `sens2.txt`.
