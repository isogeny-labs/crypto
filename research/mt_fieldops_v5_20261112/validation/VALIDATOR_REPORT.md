# Validator report: multi-target IC v5, icbest vs rhobest with native work priced (2026-11-12)

Validator: independent `validator` subagent (Opus 5.5), 2026-11-12. All validator code is in
`scripts/`; none of it is the producer's.

**Disposition: `validated` (status `pass`), with non-blocking amendments.** `replay-certificate.json`
is `verified`, `validator-report.json` is `pass`, and `claim-report.json` is `pass`: the claim
statement is endorsed exactly as worded.

- **The measurement is sound.**
  - All 3366 rows replay exactly on an instrumented build.
  - A nonce sample of 51 rows on a pristine build of the frozen source also matches, and so does
    a sample of 17 rows on the producer's binary.
  - 1,096,704 recovered logs from 153 runs were checked with the validator's own EC code. There
    were 0 failures.
  - All 972 entries of `summary.json` and all 972 of `summary_v4rule.json` recompute exactly.
  - The protocol was frozen before the data, and the curves are fresh and regenerate
    byte-for-byte.
- **The headline is reproduced.** icbest against rhobest is unranked in all 27 (cell, I/M)
  decisions under section 5a. The order flips in every one of them: the 95% lower bound is
  above 1 at h = 0 and the 95% upper bound is below 1 at h = 1. At I = 10 M:

  | h | icbest/rhobest | cells |
  |---|---|---|
  | 0 | 1.0633–1.1586 | dearer in 9/9 |
  | 0.1 | 0.9983–1.0539 | lower bound above 1 in 8/9 |
  | 1 | 0.7135–0.7995 | cheaper in 9/9 |

  Under the v4 rule (N unpriced) the IC is dearer in all 27, at 1.0475–2.0184. Against bsgs2 it
  is 1.1114–1.4709, dearer in all 27.
- **The harness is what the protocol says.** v5 is the v4 harness plus the v4 validator's
  `ic5w`/`ic5rw`/`rhodpu` patches plus the counter N, and nothing else.
  - I, M, S, H and L are unchanged on 72 replayed v4 rows.
  - N counts exactly the defined native work in every row.
  - The IC trials have no uncounted native work, and there is no free information.
- **What to disclose (V5-F5).** The IC's h = 1 advantage holds under the pre-registered
  convention, in which a distinguished-point rho pays two native operations per point. That
  convention matches section 5a and ecbench's counters. It is robust to every other variant
  tried. It is **not** robust to charging a distinguished-point rho one native operation per
  point. Then:
  - at h = 1 the IC is cheaper in only 19 of 27 (cell, I/M), and 3 decisions become "dearer";
  - at I = 10 M the h = 1 ratio is 0.83–1.06.
- **RESULTS.md prose (V5-F4).** Three sentences are wrong:
  - "rhodpu never": rhodpu wins in 4 of 198 configurations at I = 10 M, h = 1.
  - "best kappa interior at h = 1": κ = 0.5, the top of the grid, wins in 36 of 198.
  - noverify at h = 1 should read 0.70–0.73, not 0.71–0.73.

  None of them touches a headline number.

## 0. What was validated, and from where

- **Frozen artifact set:** commit `2729e252e2e66196cd48bb1a3b58cb3dc1cc8cf0` (branch
  `aburan28/eloquent-bell-2jerrf`).
  - Extracted with `git archive` into the validator scratchpad.
  - The study directory and `examples/mt_fieldops.rs` were clean in the working tree. The only
    dirty file was the bench record `benchmarks/records/20261112T0311Z-mt-fieldops-v5.json`,
    which is not part of the study.
  - HEAD moved during validation to `1f85ca3678` (bench metrics and leaderboard builder). That
    commit touches neither the study nor the harness, and it is not validated here.
- **Protocol commit:** `5aab996e55`, at 03:11:47Z.
  - It contains PROTOCOL.md, run.py, analyze.py, noverify.py, mk_runs.py, plot.py, the 33
    instances and the harness.
  - The harness is `examples/mt_fieldops.rs`, sha256 `1dfdd511…e999c`, and it is identical at
    `2729e252e2`.
  - From the protocol commit to `2729e252e2` only results, RESULTS.md, the figures and
    REPORT.pdf were added.
  - The rows record commit `59e4e892b3`, which is bench-record only. The bench record says the
    holdout started at 03:12:01Z.
- **Measured binary:** `target/release/examples/mt_fieldops`, sha256 `c55c87dd…ca64`. This equals
  `bin_sha256` in all 3366 rows.
- **Validator builds:** standalone crates from `git show 5aab996e55:examples/mt_fieldops.rs`,
  built with rustc 1.97.0, rand 0.8.8 and serde_json 1.0.151 (`arbitrary_precision`,
  `raw_value`), the versions in `Cargo.lock`.

  | build | sha256 | what it adds |
  |---|---|---|
  | pristine | `e7ac3c45…48f5` | nothing |
  | instrumented (`scripts/patch_instr.py`) | `bc86e6dd…9104` | no counted operation. A separate atomic tally counts HashMap calls, step indices, canonicalisations, hashes, DP tests, W-step indices and restart RNG draws, and the build dumps targets under `MTFO_DUMP` |

- **Command sequence:** `scripts/run_all.sh`. The nonce `fc12f5fd8a83efeadcac54dc5d6c889f` was
  drawn with `os.urandom(16)` at 03:31:42Z, before any sampling.
- **Host and scope:** local, 4 cores. Only counts are used; there is no wall-clock figure.

## 1. Replay (`replay-certificate.json`)

| replay | rows | mismatching | ok ≠ targets |
|---|---|---|---|
| every holdout row, instrumented build | 3366 | 0 | 0 |
| nonce sample, pristine build (1 per arm-parameter × bits; 17 at 32 bits) | 51 | 0 | 0 |
| nonce sample, producer binary (1 per arm-parameter) | 17 | 0 | 0 |
| one nonce-drawn configuration per cell, all 17 runs, with dumps | 153 | 0 | 0 |

- **Fields compared:** every field the binary prints except `ms`: I, M, S, H, L, N, ok, every
  phase snapshot and every arm extra.
- **Phases:** monotone in every row, and the last phase equals the totals.
- **`runs.json`:** an exact projection of `rows.jsonl`. Its `source_sha256` matches.

## 2. Certificates (scalars)

- **Script:** `scripts/ecverify.py` with `scripts/ec.py`. It uses the validator's own Jacobian
  double-and-add code, which was cross-checked against an affine implementation on 300 random
  scalars.
- **Negative controls:** off-by-one and negated logs are rejected.
- **Scope:** 153 runs covering every arm, every parameter and all three sizes, with 1,096,704
  targets.
- **Checks for every target:**
  - Q_t is on the curve;
  - 0 ≤ log < n;
  - log = planted x_t;
  - [log]G = Q_t (64,504 distinct scalar multiplications; T²/2n duplicate plantings explain the
    rest).
- **Planted lists:** identical across the 17 runs of each configuration, so the comparison is
  paired.
- **Result:** 0 failures. Every row of the holdout also has ok = targets under the harness's own
  counted check.

## 3. Harness and accounting (check 1)

### 3.1 Provenance
- **Source comparison.** The validator applied the v4 validator's `patch_ic5rw.py` and
  `patch_rhodpu.py` to the v4 harness (`bb3d2bc40e`) and compared the result with the v5 harness,
  whitespace-normalised.
  - The only differences are the `cn` field, `fn native()` and the `native()` calls: rho-family
    `hidx` ×4, `canon` ×2 (two calls each) and the ic5w/ic5rw W-step ×1 each.
  - The `N` output field is also new, and rustfmt reflowed the closures it touched.
  - So `ic5w`, `ic5rw` and `rhodpu` are exactly the v4 validator's arms.
- **v4 rows (`v4_crosscheck.json`).** The v5 binary was run on 72 nonce-drawn v4 rows on the v4
  instances, 9 per arm:
  - from the v4 holdout: ic5, ic5ks, rho2xks, rhodp, bsgs2;
  - from the v4 validator's runs: ic5w, ic5rw, rhodpu.

  It reproduces I, M, S, H, L and ok in all 72. Its N matches the v4 validator's instrumented
  counts in 18 of 18 rows that have them: the step indices of rho2xks and 2× the
  canonicalisations of rhodp.

### 3.2 N and H in every row (`native_work.json`)

| arm | N counts | identity (all rows) | holdout total |
|---|---|---|---|
| rho2xks | one per step index (`x mod 32`) | N = hidx calls | 932,642,597 |
| rhodpu | 2 per generated point (canonicalisation + one hash of x) | N = 2 × canon calls | 2 × 2,770,912,573 |
| ic5w, ic5rw | one per W-step index | N = W steps | 287,272 / 239,813 |
| ic5, ic5ks, bsgs2 | none | N = 0 | 0 |

- **H.** H equals the HashMap probes and inserts in all 2970 rows of ic5, ic5w, rho2xks, rhodpu
  and bsgs2.
  - In ic5ks and ic5rw, one H covers both the retained-table probe and the table probe of the
    same x. On a retained hit the table probe is skipped, so 0 ≤ H − probes ≤ targets in all 396
    rows.
  - This is the merged-table count the v4 validator accepted. It favours the IC, and counting
    both probes changes no decision (§6, V11).
- **"Two per generated point" for rhodpu is right for this code.** Each generated point is
  canonicalised once. Its x is mixed into one 64-bit value, whose bits give the step index, the
  look-ahead test and the DP test.
  - The code evaluates that mix three times per point: 5.25e9 index evaluations and 2.62e9 DP
    tests. An implementation needs it only once.
  - The v4 validator's count was 4 per step.
  - ecbench counts `canonicalisations_uncharged` per point and `partition_hashes_uncharged` per
    step, priced separately. Its `negation_strong` also charges `dp_tests_uncharged`.
  - So the frozen count matches ecbench's convention, or is lighter on rho.
- **The IC side.**
  - **Trials:** `xpair` (counted I, M and S) plus table probes (counted H). There is no
    per-trial index, canonicalisation or other native step.
  - **W offsets:** shared by every target. ic5 and ic5ks use an arithmetic progression with no
    index. ic5w and ic5rw use one counted splitmix index per W.
  - **Solver:** mod-n products and inverses are counted as M and I. Mod-n additions and
    zero tests are not counted, the same convention as uncounted F_p additions.
- **Uncounted rho work:** the walk-restart RNG draws. That is 32,384,938 for rho2xks (3.5% of
  steps) and 144,400,826 for rhodpu (5.2% of points). Leaving them out favours rho.
- **No free information.**
  - The planted logs are used only for planting (an uncounted field copy) and in the final
    counted check.
  - ic5rw's retained hits use only logs that it has already solved.
- **Medians per target at I = 10 M, in GAE:**

  | arm | field (incl. verification) | H | N |
  |---|---|---|---|
  | ic5 | 247.0 | 364.9 | 0 |
  | ic5w | 247.5 | 365.4 | 0.18 |
  | ic5rw | 230.0 | 466.4 | 0.15 |
  | rho2xks | 206.8 | 503.6 | 138.0 |
  | rhodpu | 321.8 | 53.9 | 559.8 |
  | bsgs2 | 188.5 | 270.6 | 0 |

## 4. Recompute (check 3)

- **Script:** `scripts/recompute.py`, the validator's own code, working from `runs.json`. It
  uses the protocol's bootstrap: `random.Random(1)`, B = 4000, order statistics 100 and 3899.
- **summary files:** 972 of 972 `summary.json` entries and 972 of 972 `summary_v4rule.json`
  entries match, with a maximum relative difference of 0.
- **Decisions:** every decision line of `analysis.txt` and `analysis_v4rule.txt` is reproduced.
  An independent numpy bootstrap (B = 20000, seeded from the nonce) gives the same decisions.

  | candidate | vs rhobest | vs rho2xks | vs rhodpu | vs bsgs2 |
  |---|---|---|---|---|
  | icbest (5a) | unranked 27 | unranked 27 | cheaper 27 | dearer 27 |
  | ic5rw (5a) | unranked 27 | unranked 27 | cheaper 27 | dearer 27 |
  | icbest (v4 rule) | dearer 27 | unranked 27 | unranked 27 | dearer 27 |

- **RESULTS and noverify (`results_check.json`, 167 checks).**
  - All 108 numbers of the RESULTS table match, and so do all 27 of `noverify.txt`.
  - The ranges match (1.06–1.16, 1.00–1.05, 0.71–0.80, v4 rule 1.05–2.02, BSGS 1.11–1.47), and
    so do the cell counts (9, 8, 9).
  - The IC winners match: ic5rw 198 / 197, and ic5w 100 or ic5 98.
  - The κ winners match: 126/69, 111/63, 147/36.
- **Two mismatches:**
  - RESULTS says the rho winner at h = 1 is rho2xks in all 198. It is rho2xks in 194, because
    rhodpu wins in 4.
  - The noverify minimum at h = 1 is 0.70499. It rounds to 0.70, not 0.71.
- **Shared verification phase:** 34.5–47.4 GAE per target at I = 10 M in every arm.

## 5. Protocol (check 4)

- **Freeze.** The protocol was frozen before the data, as set out in §0. No change was made to
  PROTOCOL, the runner, the analysis, noverify, mk_runs or the harness after the freeze.
- **Grid.** The grid is complete: 9 cells × 22 configurations × 17 runs = 3366. Seeds 41 and 42
  are used as frozen, and no row was dropped.
- **Instances (`instances_check.json`):**
  - All 33 are valid: p and n prime, the curve nonsingular, nG = O, n the only multiple of n in
    the Hasse interval, and the fixture log correct.
  - They regenerate byte-identically (33 files plus `MANIFEST.json`) from
    `gen_instances.py --families W --bits 24,28,32 --per 11 --seed mtfo-v5-20261112`.
  - There is no (p, a, b) overlap with 116,860 curve records in 107,880 JSON files under
    `research/` and `docs/curves`.
  - One curve, W32-4, shares (p, n) with v4's W32-4, but its j-invariant differs. The curves are
    not isomorphic: it is a different curve with different targets.
- **Grid edges.**
  - At h = 0, rho2xks's best κ is the floor, 0.05, in 126 of 198 configurations. A smaller κ can
    only make rho cheaper at h = 0. That widens the IC's h = 0 loss and cannot change an
    unranked decision.
  - At h = 1, κ = 0.5 (the top of the grid) wins in 36 of 198. A validator panel at κ = 0.75 and
    1.0 (396 runs) never wins at any price, so the h = 1 result does not rest on the edge.
- **Undocumented smoke runs.** PROTOCOL's predictions come from smoke runs whose instances,
  seeds and outputs are not committed.

## 6. Robustness of "IC cheaper at h = 1 in all 9 cells" (check 5; `sensitivity.json`)

This section uses the protocol's decision rule and bootstrap. None of these variants was
pre-registered. Decisions are counts out of 27 (cell, I/M).

**Pricing variants:**

| variant | decisions | I = 10 M medians, h = 0 / 0.1 / 1 | h = 1 cheaper (of 27) |
|---|---|---|---|
| V0 as frozen | unranked 27 | 1.063–1.159 / 0.998–1.054 / 0.714–0.799 | 27 |
| V1 rho2xks step index free (it could reuse the per-point table hash) | unranked 27 | … / 1.048–1.103 / 0.870–0.918 | 27 |
| V1b rho2xks: only restart starts charged | unranked 27 | … / 1.046–1.101 / 0.863–0.908 | 27 |
| V5 rho2xks index ×2 | unranked 27 | … / 0.947–1.009 / 0.643–0.716 | 27 |
| V4 rhodpu 4 per point (v4 validator's count) | unranked 27 | as V0 | 27 |
| **V2 rhodpu 1 per point** | **unranked 24, dearer 3** | … / 0.998–1.054 / **0.833–1.059** | **19** |
| **V3 minimal: V2 + V1 + IC N = 0** | **unranked 20, dearer 7** | … / 1.048–1.103 / 0.901–1.059 | **19** |
| V6 = v4 rule (N unpriced) | dearer 27 | … / 1.048–1.103 / 1.215–2.018 | 0 |

**Reference and selection variants:**

| variant | decisions | I = 10 M medians, h = 0 / 0.1 / 1 | h = 1 cheaper (of 27) |
|---|---|---|---|
| V7 cross-fit (parameters chosen on the other seed) | unranked 27 | 1.063–1.152 / 0.995–1.042 / 0.715–0.802 | 27 |
| V8 + rho2xks κ 0.75, 1.0 and rho2x κ 0.5, 1.0 | unranked 27 | as V0 | 27 |
| V12 V8 with rho2xks/rho2x index free | unranked 27 | … / 1.048–1.103 / 0.875–0.919 | 27 |
| V10 + modelled DP rho without negation map (1 N/point, √2 walk) | unranked 27 | as V0 | 27 |
| V11 ic5ks/ic5rw two probes per trial class | unranked 27 | … / 1.035–1.077 / 0.714–0.799 | 27 |

What the tables show:

- **Robust.** Under the frozen convention, which is the section 5a and ecbench reading, the h = 1
  result survives:
  - pricing rho2xks's step index at 0;
  - κ above the grid;
  - rho2xks without retention (rho2x);
  - a modelled non-negation DP rho;
  - cross-fitting;
  - a stricter hash count for the IC's retention arms.

  No standard pseudo-random-walk rho avoids a per-step partition decision. The generic method
  with no per-step native work in this panel is BSGS, and the IC does not beat it at any price.
- **Not robust.** Charge the distinguished-point rho one native operation per point, taking the
  canonicalisation or the point hash as free. Then rhodpu is the cheapest rho at h = 1 in all 198
  configurations. At I = 10 M, h = 1 the IC/rho ratio is:

  | cell | V2 | frozen |
  |---|---|---|
  | 28 b, T = 1024 | 0.991 [0.984, 1.002] | 0.731 |
  | 32 b, T = 4096 | 1.010 [1.008, 1.013] | 0.717 |
  | 32 b, T = 1024 | 1.059 [1.043, 1.065] | 0.728 |

  - The ratio also grows with the group size: 0.908, 0.991 and 1.059 at 24, 28 and 32 b
    (T = 1024). Under the frozen convention it is 0.748, 0.731 and 0.728.
  - Three decisions become "dearer": 32 b with T = 1024 at I = 10 M and 30 M, and 32 b with
    T = 4096 at I = 10 M.

## 7. Findings
Full text with numbers is in `validator-report.json`.

| id | severity | finding |
|---|---|---|
| V5-F1 | pass | Exact replay (3366 + 51 + 17 + 153). 1,096,704 logs certified with the validator's EC code |
| V5-F2 | pass | Harness = v4 + the v4 validator's arms + N only. I, M, S, H, L unchanged on v4 rows. N and H match their definitions in every row. No uncounted IC native work. No free information |
| V5-F3 | pass | 972 + 972 summary entries, all decisions, the RESULTS table and noverify recompute exactly |
| V5-F4 | minor | RESULTS prose: "rhodpu never" (4/198 at I = 10 M, h = 1; 27/198 at I = 3 M); "best κ interior at h = 1" (36/198 at κ = 0.5); noverify h = 1 range 0.70–0.73 |
| V5-F5 | major (disclosure, non-blocking) | The h = 1 advantage needs two native operations per DP-rho point, which is the frozen and section 5a/ecbench convention. At one per point: 19/27 cheaper at h = 1, 3 dearer decisions, 0.83–1.06 at I = 10 M |
| V5-F6 | pass | Grid edges and stronger references do not move the result. Cross-fit is harmless |
| V5-F7 | info | Uncounted work: rho restart RNG (favours rho). The IC's merged-table probe count favours the IC, but changes no decision |
| V5-F8 | process | rows.jsonl (2.5 MB) is in Git, not the artifact store. The bench record was open at the validated commit. The leaderboard builder was pointed at v5 before validation. The smoke runs are undocumented |
| V5-F9 | info | One (p, n) coincidence with a non-isomorphic v4 curve |

## 8. Required amendments (none blocking)
1. **V5-F4.** In RESULTS "Which arm wins", correct three statements:
   - Replace "rhodpu never" with: rho2xks in 198/198 at h = 0 and 0.1 and in 194 at h = 1;
     rhodpu in 4 (32 b, T = 1024), and in 27 at I = 3 M.
   - Replace "the best kappa is interior" with: κ = 0.5, the top of the grid, wins in 36; the
     validator's κ = 0.75 and 1.0 runs never win.
   - Change noverify at h = 1 to 0.70–0.73.
2. **V5-F5.** Disclose in Limits and in the Answer that the h = 1 advantage holds under the
   pre-registered native-work convention, and is robust to rho2xks's index priced at 0. With one
   native operation per distinguished-point rho point:
   - the IC is not cheaper at h = 1 in 8 of 27 (cell, I/M);
   - 3 decisions are dearer (32 b, T ≤ 4096);
   - the ratio is 0.83–1.06 at I = 10 M.

   Replace "a choice that favours rho over the v4 validator's count" with this two-sided
   statement.
3. **V5-F8.** Clean up the process gaps:
   - Move rows.jsonl to the artifact store and close the bench record.
   - Keep the leaderboard's v5 multi-target section "not validated" until a `VALIDATION.json`
     passes the gate and names this report.
   - Record the pre-freeze smoke runs in a dated note. Do not edit the frozen PROTOCOL.

**Accepted RESULTS headline** (the producer's headline plus the V5-F5 sentence):

> On 33 fresh prime-order curves (group orders about 2^24-2^32, T = 1024-16384, 22 paired
> configurations per cell), with counted field and mod-n operations and every arm's hash-table
> and native work priced at h = 0, 0.1 and 1 GAE each, the cheapest of four multi-target IC
> variants is unranked against the cheapest of ten runs of this harness's multi-target rho in
> all 27 (cell, I/M) decisions because the order flips with h: at I = 10 M it is 1.06-1.16x at
> h = 0 (dearer in all 9 cells), 1.00-1.05x at h = 0.1 and 0.71-0.80x at h = 1 (cheaper in all 9
> cells). With native work unpriced it is dearer in all 27 (1.05-2.02x at I = 10 M); it is
> 1.11-1.47x a paired BSGS at every price. The h = 1 advantage rests on charging a
> distinguished-point rho two native operations per point: at one per point the IC is not
> cheaper at h = 1 at 28-32 bits with T = 1024-4096 (0.83-1.06x at I = 10 M) and 3 of the 27
> decisions become dearer.

## 9. The claim statement (`claim-report.json`)
**Status `pass`: endorsed exactly as worded** (statement sha256 `123e66c3…199a`). All 12
clauses recompute, and all 14 headline numbers agree with the validator's recompute.

The statement names its convention: canonicalisations, point hashes and step indices are each
priced at h. Under that convention every clause holds. The report's notes record four points:
- the one-per-point sensitivity (V5-F5);
- rho2xks's "step-index hash" is `x mod 32`; the result holds with it free;
- the (p, n) coincidence (V5-F9);
- L = 0 in every row.

## 10. Limits of this validation
- **Sensitivity panels:** rho2xks at κ 0.75 and 1.0, and rho2x, are validator panels. They were
  not pre-registered, and only the harness's counted check verified their targets.
- **Modelled rho:** the non-negation DP rho is a model built from rhodpu's phase counts, not a
  run.
- **Pricing:** charging each native operation at h is the section 5a convention, not a measured
  cost. The band does not say what a canonicalisation or a hash costs relative to a field
  multiplication on any platform.
- **Scope:** counts only. Memory is unbounded for every arm. Groups are about 2^24 to 2^32, on
  one local host. Nothing here bears on single-target ECDLP or on ECC2K-130 / m = 83.
