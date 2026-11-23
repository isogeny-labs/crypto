# Validator report: research/genus_ladder_scaling_20261119 (lead C)

**Disposition: validated-with-errata.** Every number in `analysis.json`, `tables.md` and `RESULTS.md`
recomputes from the raw rows. 38 sampled run rows replay identically, and 16 recovered logarithms pass
an independent certificate. The errata are interpretive: the main one is that the tuned genus-5 arm
x5\* is not a double-large-prime arm (F1). The VALIDATION.json gate cannot pass in a clean clone yet,
because the raw rows are not in Git (F4). Every number here is `[pcc:unverified]`. Nothing in this
report is a pcc claim.

## Binding

| item | value |
|---|---|
| validated commit | `469f5bf73199cfb69e01426df46170bd976bdb88` (branch `claude/hyperelliptic-index-calculus-td1fdn`). The study's tracked files are byte-identical at branch head `f0ac380b21`, re-checked 2026-11-23. The tracked tree under the study is clean. |
| freeze / D1 / selection commits | `b5971d86dd` (2026-11-19T06:40:49Z) / `d0ac2cd27b` (2026-11-20T19:08:35Z) / `2a3fde9e0f` (2026-11-21T05:01:41Z) |
| binary | `scratchpad/bin/genus_ladder.po`, sha256 `493ee294…6b921`, equal to `binary.sha256` (built from `ca72267899`; the validated prime-order study's rebuild had identical code sections) |
| raw rows | 550 untracked files (457,528 bytes): b4, b5, tune5, rho5, search `.jsonl`/`.log` and `failed_p60013/`. Each is hashed in `validation/out/raw_manifest.json` (the manifest's own sha256 is `02e18928061ba966380ac6728d447f786179cb35fa50b3f8647668c5acefb4db`). They were re-hashed unchanged on 2026-11-23. |
| reused rows | `research/genus4_prime_order_20261118/b_p{16411,32771}_{x,xstar}.jsonl.gz` (tracked). Their hashes equal that study's validator pins. |
| replay nonce | `f22ec7a6c4996832db2d625dd608ea77` (drawn 2026-11-21T06:46:03Z, `validation/out/nonce.txt`) |
| host | the local shared 4-core host, single-threaded jobs only, no Rust build |

## Requirements

| # | requirement | result | evidence |
|---|---|---|---|
| 1 | D1 is only the 60013 → 54293 substitution, and it was written before any 54293 data | **pass** | `out/freeze_check.json` |
| 2 | selection follows the frozen rule, and every admitted curve has prime #J | **pass** | `out/curves_check.json` |
| 3 | sampled rows replay with the binary (`--threads 1`), with all non-timing fields equal | **pass** (38/38 sampled rows, plus 5 extra rows) | `out/replay_compare.json`, `out/compare_extra.json` |
| 4 | every ratio, interval, slope, C2 ratio, reference and band recomputed | **pass** | `out/recompute.json`, `out/compare_claims.json` |
| 5 | certificates: logs or only booleans, and whether scalars can be verified | rows hold **booleans only**. Scalars were verified on a sample via side builds (16/16). | `out/certs.json` |
| 6 | genus-5 tuning used only 1973 / 2254 and never confirmation curves | **pass** | `out/recompute.json` (`tune`) |
| 7 | overstatements in RESULTS.md | **errata F1–F3, F5, F6** | this report |

### 1. D1

- **The edits are exactly the substitution.** Replacing `54293` with `60013` in the current `run.sh`,
  `curves.py` and `analyze.py` reproduces the frozen sha256 of each file. The reverse also holds:
  substituting the other way in the freeze-commit blobs reproduces the current files byte for byte
  (5, 1 and 6 occurrences).
- **The other frozen files are unchanged.** `PROTOCOL.md`, `search.sh` and `tune.py` still match
  `frozen.sha256`.
- **D1 came before the 54293 data.** `DEVIATIONS.md` first appears in `d0ac2cd27b`, committed at
  19:08:35Z. The first p = 54293 file has an mtime of 19:08:36Z, 1.0 s later. The b4 blocks started at
  2026-11-21T05:12:57Z. `DEVIATIONS.md` has not changed since. The mtimes are producer-controlled, so
  they are consistent with the record but weak evidence on their own.
- **The guard explains the p = 60013 failure.** The 56 kept chunks (112 files with their logs) cover seeds 1..1500. Their 165
  `other` rejects are exactly the 165 seeds whose f the validator's Rabin test finds irreducible, and
  the reducible count matches the remaining 1335. At g = 4 the guard value is
  (√p + 1)^8 = 1.340e19 ≥ 9.0e18 at p = 60013, and 8.992e18 < 9.0e18 at p = 54293. So 54293 is the
  largest prime the guard admits.
- **The driver fix is not committed.** The "driver fix" that D1 mentions lives only in
  `scratchpad/drive_search.sh` (see F8).

### 2. Selection and group orders

- **Coverage.** The search covers seeds exactly 1..1500 at g4 p54293, 1..2500 at g5 p2357 and
  1..1500 at g5 p4099. No seed lies in two chunks, and every chunk has a cell row with
  `tried` = its seed count. The p2357 extensions follow the rule: 2 seeds were admitted by 1500,
  3 by 2000 and 5 by 2500.
- **Reject counts.** In every chunk, the validator's irreducibility test reproduces the
  `f_reducible_even_order` count. Over the cells that is 1317, 2299 and 1366 reducible seeds. The
  irreducible seeds equal admitted plus order rejects: 183 = 4 + 179, 201 = 5 + 196, 134 = 4 + 130.
- **Selection.** The recomputed selection matches `curves.json`:
  - g4: 381, 1206, 1375 (1453 is 4th);
  - g5 p2357: confirm 466, 1215, 1520; tune 1973, 2254;
  - g5 p4099: 330, 538, 574.
- **1375 against 1453.** Chunk `w0_c336` (holds 1453) finished at 01:53:37Z, and chunk `w2_c336`
  (holds 1375) at 04:52:48Z. The range 1..1500 was complete only at 05:01:11Z, and `curves.json` was
  committed at 05:01:41Z. The frozen rule (smallest seeds over the full covered range) therefore
  gives 1375. No run row uses 1453.
- **Group-order certificates, all 13 admitted curves (the validator's own Cantor arithmetic,
  `scripts/jac.py`).** For each curve:
  - f = `random_f(seed)`, monic of degree 2g+1, squarefree and irreducible;
  - r is prime (deterministic Miller–Rabin);
  - r lies in the Hasse–Weil interval and exceeds the interval's width;
  - [r]D = 0 for 3 random nonzero divisors.

  Together these give #J = r. Run files and search rows carry the same instance.
- **Only negation applies.** The only F_p-rational affine automorphism is the identity, so negation
  is the strongest symmetry available to rho.
- **Not verified.** The 505 `order_not_near_prime` rejections rest on the solver's point count. They
  can only have skipped a smaller admissible seed. They cannot have changed a selected curve, whose
  orders are certified above.

### 3. Replay

- **Sample and method.** 17 nonce-drawn jobs, each one (curve, target) pair with both solver seeds,
  run single-threaded with `run.sh`'s flags. Every field except `wall_ns` is equal to the committed
  row, and so is the instance row.

| block | pairs replayed | frozen binary rows | export build rows |
|---|---|---|---|
| b4_p54293_cc2.0 (x) | 1206 k5 | 2/2 | – |
| b4_p54293_cc1.4 (x\*) | 1375 k4, 1375 k2 | 4/4 | – (logs via val_po) |
| b5_p2357_x5 | 1215 k1 | 2/2 | 2/2 |
| b5_p2357_x5star | 466 k1, 1520 k0 | 4/4 | 4/4 |
| b5_p4099_x5 | 330 k1 | 2/2 | 2/2 |
| b5_p4099_x5star | 330 k5, 330 k0 | 4/4 | 4/4 |
| tune5_cc2.8, tune5_cc1.0 | 2254 k0 (both SGE arms) | 8/8 | – |

- **Extra replays.**
  - The tune key (2254, k0) at **cc 4.0** gives rows identical to the committed cc 2.8 rows (4/4);
    see F1.
  - The G3 rho run (466, k0, seed 1) replays identically: 141,805,483 steps.

### 4. Recompute

- **Point estimates.** The validator's own script (`scripts/recompute.py`, sharing no code with
  `analyze.py`, keys read with `[]`) recomputes all 114 point estimates in `analysis.json` to within
  1.7e-13 relative. These cover:
  - per-cell ratios at w = 0, 0.1 and 1, mean r, mean cost, LA share and per-curve ratios;
  - the C1 slopes, exponents and residuals;
  - the C2 ratios and N values;
  - the tune cost and the G3 ratio.
- **Bounds and decisions.** With an independent hierarchical bootstrap (numpy PCG64, 4000
  resamples), all 76 bootstrap bounds agree within 6.3e-4. Every decision flag agrees: below at all
  w, the C2 verdicts, G1–G3, the tune choice, and the C1 rule.
- **Producer scripts reproduce.** `analyze.py`, `tune.py` and `curves.py`, re-run in a sandbox on the
  same rows, reproduce `analysis.json`, `tables.md`, `tune.json` and `curves.json` byte for byte.
- **Every RESULTS.md string matches.** Each number RESULTS.md prints equals the recomputed value
  rounded to the printed digits.

One unit: IC cost per key over the ideal negation bound R = √(πr/4) per key, both as a mean over the
36 keys (60 for the reused rows), in GAE of the curve's own genus. Native work is priced at w GAE per
counted operation. The reference charges nothing.

| genus | p | arm | keys | w = 0 | w = 0.1 | w = 1 [95% bootstrap] | below at all w | LA share (w = 1) |
|---|---|---|---|---|---|---|---|---|
| 4 | 16411 | x | 60 | 0.00178 | 0.10147 | 0.99866 [0.9949, 1.0024] | no | 0.766 |
| 4 | 16411 | x\* | 60 | 0.00295 | 0.09024 | 0.87592 [0.8716, 0.8798] | yes | 0.560 |
| 4 | 32771 | x | 60 | 0.00113 | 0.07795 | 0.76929 [0.7665, 0.7720] | yes | 0.806 |
| 4 | 32771 | x\* | 60 | 0.00193 | 0.06572 | 0.63987 [0.6383, 0.6414] | yes | 0.604 |
| 4 | 54293 | x | 36 | 0.00082 | 0.06521 | 0.64476 [0.6425, 0.6466] | yes | 0.819 |
| 4 | 54293 | x\* | 36 | 0.00142 | 0.05369 | 0.52417 [0.5227, 0.5256] | yes | 0.616 |
| 5 | 2357 | x5 (cc 2.0, double LP) | 36 | 0.00064 | 0.04426 | 0.43690 [0.4269, 0.4450] | yes | 0.130 |
| 5 | 2357 | x5\* (cc 2.8, full factor base, F1) | 36 | 0.00062 | 0.04250 | 0.41945 [0.4076, 0.4295] | yes | 0.115 |
| 5 | 4099 | x5 (cc 2.0, double LP) | 36 | 0.00029 | 0.02270 | 0.22435 [0.2219, 0.2268] | yes | 0.188 |
| 5 | 4099 | x5\* (cc 2.8, full factor base, F1) | 36 | 0.00026 | 0.02038 | 0.20144 [0.1988, 0.2041] | yes | 0.182 |

- **Paired per-key ratios** at w = 1 (lesson L-26895912385b) span:
  - x\* at 54293: [0.517, 0.529];
  - x5\* at 2357: [0.379, 0.444];
  - x5\* at 4099: [0.194, 0.212].

  Per-curve ratios agree to within 0.03.
- **C1 (x\*).**
  - Slope −0.4306 [−0.4351, −0.4258].
  - Cost exponent 1.5685 [1.5640, 1.5733].
  - Middle residual −0.0114.
  - For x: slope −0.3664 and exponent 1.6327.
- **C2.**
  - x5\* / x\* = 0.4789 [0.4652, 0.4906] at N ≈ 2^56 and 0.3148 [0.3106, 0.3191] at N ≈ 2^60.
  - The slopes in ln N are −0.2644 (g5 x5\*) and −0.1137 (g4 x\*).
  - The N values match to −0.24% and +1.03%.
- **G3.** The measured plain rho comes to 0.8204 × √(πr/2) as a mean of 2 runs. The per-run values
  are 0.421 and 1.220. The rho rows list `hash_uncharged`, `coef_adds_uncharged` and
  `compares_uncharged` (0, window-free).
- **§5a band.** The reference is a closed form with no counted work, so there is no rho-side
  unpriced counter to price. On the IC side, the priced counters are `rel_mul + rel_add + la_mul +
  la_add`. `la_mul = la_wied_mul + la_sge_mul`. `la_add` additionally contains the union-find
  operations. `group_ops = walk_ops + setup_ops + verify_ops` on every row. The unpriced IC work is
  listed in F5. Lesson L-546d37ca7750 has no ecbench session to apply to here; its point (rho's
  per-step work vs the IC table work) is covered by F5. Every one of the six new cells is below the
  reference at all three weights, by its upper bound.

### 5. Certificates

**Rows carry only booleans.** The run rows carry `ok` (recovered = planted t, computed inside the
binary) and `certified` (the binary's own [t]G = T check). They hold no recovered logarithm, no G
and no T. `src/bin/genus_ladder.rs` at `ca72267899` emits nothing more, so the committed rows alone
cannot be certified (F7). The validator obtained the logs without a Rust build as follows.

- **Genus 4.** The prior validator's harness `val_po` (sha256 `81bbb745…`, source
  `research/genus4_prime_order_20261118/validation/scripts/val_po.rs`, genus 4 only) prints f, G, T,
  t and the recovered log.
- **Genus 5.** The export build `scratchpad/bin/genus_ladder.blpt` (sha256 `e210549a…`, used by
  `research/block_lanczos_pretest_20261120`) writes each solve's target coordinate when
  `GENUS_LADDER_EXPORT_CORE` is set. Its 12 run rows are identical to the committed rows, which binds
  it to the committed computation. The recovered log is `coord_out · h_inv mod r`.
- **The validator's own check.** For every sampled key, `jac.instance_spec` rebuilds (G, t, T) from
  (p, g, curve seed, target seed, h, r) with its own Cantor arithmetic. It then checks:
  - G is valid, G ≠ 0 and [r]G = 0;
  - T = [t]G;
  - L = t and [L]G = T for the solver's log L;
  - counts equal to the committed row.

  Tamper controls (L + 1, one T coefficient, count + 1) are caught.
- **Result.** 16 of 16 logs pass:
  - 4 genus-4 logs, x\* at p = 54293 (1375, k2 and k4);
  - 12 genus-5 logs: x5 at 2357 and 4099; x5\* at 2357 (466 k1, 1520 k0) and at 4099 (330 k5,
    330 k0).
- **The reconstruction matches the solver.** It equals the solver's own G, T and t on 12 instances of
  the prime-order study (p ≡ 3 mod 4) and on both p = 54293 keys here (p ≡ 1 mod 4, Tonelli–Shanks).
- **The solver cannot read the secret.** At `ca72267899`, `run_ic_ext` never reads `t_secret`; only
  `instance_from` and tests do.
- **Scope.** This is a sample: 16 of the 216 IC confirmation runs and 0 of the 96 tuning runs.

### 6. Tuning

- **Only the tune curves.** Every tune row is on curve 1973 or 2254. Each of the 8 configurations
  has exactly the frozen 12 keys (targets k = 0..2, seeds 1–2), and no confirmation curve of either
  prime appears.
- **Order in time.** The tune files were finished by 05:12:57Z. b5 started at 05:45:14Z, and its
  x5star config is cc 2.8 / SGE (8, 2), the reproduced choice.
- **The choice and its margin.** The choice costs 100,290,947.6; the runner-up, 2.8 / (4, 1.5),
  costs 101,699,591.6, which is +1.4%.
- **No earlier genus-5 runs.** No genus-5 p = 2357 run of another study was found in `research/`.

## Findings

1. **F1 (major, interpretation): x5\* is the full-factor-base algorithm, not double large prime.**
   - **What the rows show.**
     - **The factor base is every place.** At cc 2.8 the factor-base target ⌈2.8·p^0.8⌉ (1397 at
       p = 2357, 2175 at p = 4099) exceeds the number of degree-1 places. Those are the x with f(x) a
       nonzero square: 1165–1190 at p = 2357 and 2053–2077 at p = 4099, counted by the validator.
       So every place is in the base, and `fb_cols` equals the place count on every curve.
     - **No large prime is ever used.** `rels_by_lp` sums to [n, 0, 0] in every x5\* run (and in the
       cc 2.8 tune runs).
     - **A larger cc changes nothing.** A replay at cc 4.0 is row-identical to cc 2.8.
   - **What is wrong in the text.**
     - `PROTOCOL.md` §4 and `RESULTS.md` describe x5\* as the double-large-prime walk.
     - `RESULTS.md` reads P3's grid edge as "the genus-5 optimum may lie beyond it … x5\* is therefore
       an upper bound on the tuned cost". That is wrong: every cc above 2.34–2.39 at p = 2357 and
       2.64–2.68 at p = 4099 (the range is over the curves) is this same algorithm. The untested range is cc between 2.0 and
       saturation.
     - The theory exponents quoted beside C2, (2 − 2/g)/g = 0.32 for genus 5, belong to the
       double-large-prime algorithm, not to the arm that produced 0.419 and 0.201.
   - **The C2 direction holds with the genuine double-large-prime genus-5 arm.** For x5 (cc 2.0,
     85% and 75% of the places in the base, large-prime relations present), x5 / x\* = 0.4988
     [0.4870, 0.5083] at 2^56 and 0.3506 [0.3467, 0.3545] at 2^60. The upper bounds are below 1 at
     every w.
   - **Correction.** Label x5\* as "full factor base (cc 2.8 saturates; no large primes)". State
     the double-large-prime genus-5 reading next to it. Drop the "beyond the grid edge" sentence.
2. **F2 (moderate): the genus-4 three-point fit is curved; the "sits on the line" sentence and the
   interval understate it.**
   - **The curvature is resolved.** The middle-point residual is −0.0114 for x\*, with bootstrap
     interval [−0.0137, −0.0091], and −0.0053 for x, with [−0.0083, −0.0025]. Both exclude 0.
   - **The local exponent of x\* rises.** It is 1.542 [1.535, 1.550] from 16411 to 32771 and 1.608
     [1.601, 1.615] from 32771 to 54293. The difference, 0.065 [0.054, 0.077], is about 7 times the
     width of the reported interval.
   - **The interval covers sampling error only.** The three-point exponent interval
     [1.564, 1.573] covers key sampling, not model misfit, and it excludes 1.5.
   - **"Consistent with 1.5" is the frozen window, not agreement.** The C1 verdict comes only from
     the frozen [1.4, 1.6] overlap rule, and the latest local exponent is already outside that
     window.
   - **Correction.** Replace "the middle points sit on the line" with the residual and the two
     local exponents. Call 1.568 the fitted exponent over p = 16411–54293.
3. **F3 (moderate): the genus-5 slopes are two-point and below the asymptotic theory.**
   - **The implied exponents are below theory.** The measured two-point cost exponents are p^1.18
     for x5\* (N^0.236) and p^1.30 for x5 (N^0.260). Both are below the double-large-prime theory of
     p^1.6 (N^0.32).
   - **The regime is pre-asymptotic.** The LA share is 0.11–0.19, so relation search dominates.
     The slopes cannot be extrapolated.
   - **What P4 rests on.** P4 ("holds") rests on two point estimates.
   - **Correction.** RESULTS.md puts −0.264 next to the theory exponents without saying this. It
     should, and it should state that x5\*'s advantage over x5 grows from 4% to 10% between the two
     primes. That growth is part of the steeper slope.
4. **F4 (major for the gate, provenance): the raw rows are not in the repository.**
   - **None of the 550 raw files is tracked.** The study's `.gitignore` ignores `*.jsonl` and
     `*.log` everywhere. Unlike the prior study, there is no `.jsonl.gz` copy and no artifact
     manifest.
   - **No clean clone can reproduce the numbers.** A clone has no evidence for any number in
     `analysis.json`. The validation gate's `raw_data` check passes in this worktree only because
     the files exist on disk.
   - **The rows are small enough for Git.** The largest file is 38.5 KB, below the 64 KiB data
     threshold.
   - **Remedy.** Commit them byte-identical (sha256 in `validator-report.json` `raw_data`), as
     tracked files or `.jsonl.gz`.
5. **F5 (minor, wording): "every native operation charged" overstates.**
   - **What is counted.** The priced counters are the F_p and mod-r arithmetic of the relation and
     linear-algebra phases.
   - **What is not.** These are not counted:
     - the per-step partition hash (`mix64(key_of)`; one per walk step, count ≈ `walk_ops`);
     - the large-prime `HashMap` and union-find lookups;
     - the `col[]` lookups.
   - **The effect is small.** The ideal reference charges nothing either, and an implemented rho
     would pay a per-step hash too (lesson L-10ef4b64c2a6). Charging the IC hash at 1 GAE per walk
     step raises the w = 1 ratios by at most 0.3% (e.g. x\* 54293: 0.5242 → 0.5256), so no ranking
     changes.
   - **The calibration is genus 4 only.** The cited calibration (about 0.003 GAE) was measured
     against genus-4 group operations. A genus-5 Cantor step costs 695 F_p multiplications against
     394 for genus 4 in this implementation, so w = 1 is even more conservative at genus 5.
6. **F6 (minor, clarity): C2 is relative to each Jacobian's own rho, in its own group operations.**
   "Genus 5 ahead" should name that unit. In a shared F_p-multiplication unit (lesson L-e388a81efa37;
   `scripts/unit_sensitivity.py`), genus 5 is further ahead: x5\* / x\* = 0.237 and 0.156 at w = 1.
   The verdict does not depend on the unit.
7. **F7 (minor, instrument): rows carry no recovered logarithm.**
   - **Why it matters.** `ok` and `certified` are computed by the solver binary itself. A
     certificate independent of the solver needed a harness built for another study (genus 4) and
     an export build (genus 5).
   - **Remedy.** Emit `recovered` (and the target in Mumford form) in the run rows, so that every row
     can be certified from Git alone.
8. **F8 (minor, process): some operational records are missing.**
   - **The driver is uncommitted.** The search driver with the D1 stop rule is only in the
     scratchpad (`drive_search.sh`).
   - **Events are not in DEVIATIONS.md.** The out-of-memory kill of one search worker, the two
     2-hour stops and the container restart appear only in RESULTS.md. They are verified harmless:
     coverage is exact, and no chunk was duplicated or left without a cell row.
9. **F9 (information): G3 is thin.** Its two runs read 0.42 and 1.22 of √(πr/2). It is a sanity
   check only, as RESULTS.md says. Seed 1 replays identically, but like the IC rows it carries only booleans.
10. **F10 (information): routing and branch.**
    - **Local host.** The study ran on the local host, not isolab-eks, as declared in the frozen
      protocol. The counts do not depend on the host and replay identically here.
    - **Moving branch.** The branch receives automated "WIP snapshot" commits. Two of them
      (`3b2b980d617`, 01:42:52Z, and `f0ac380b21c`, 01:50:18Z on 2026-11-23) committed this
      `validation/` directory while it was still in progress. The first was made while the rho
      replay was half-written and during the main session's disk-full window (01:36–01:45Z). The
      validator made neither commit.
      - **Effect.** Those snapshots may hold partial outputs. The authoritative files are the
        working-tree files at handback. All of them were regenerated after the window, and every
        JSON / JSONL file parses.
      - **The validation pins `469f5bf731`.** The producer files are unchanged from it to `f0ac380b21`.
    - **Reporting.** No PDF, `bench.record` or `benchmarks/` entry exists yet; RESULTS.md says so.

## Predictions, as graded

| # | RESULTS.md says | validator |
|---|---|---|
| P1 | holds | **Agrees.** 0.5242 is in [0.42, 0.58], graded at 54293 per D1. |
| P2 | holds | **Agrees.** The slope is −0.431 and the exponent 1.568. See F2 for what the interval means. |
| P3 | fails, grid edge | **Fails, but the stated reason is wrong.** The cc grid's top point is the saturated (full) factor base (F1). |
| P4 | holds | **Holds on point estimates.** The same holds with x5: 0.499 → 0.351. See F3. |
| P5 | holds | **Agrees.** G1 (all 456 rows of the 10 cells certified and ok, 216 of them new), G2 (h = 1, coverage complete) and G3 (0.82) all pass. |

## Gate

- **Draft manifest.** `validation/VALIDATION.json` is a draft with `validator-report.json` and
  `replay-certificate.json` beside it.
  - Its band is the binding cell of RESULTS.md's "below at all w": g4 p54293 x. Per key, IC is
    2.14e6, 1.71e8 and 1.69e9 GAE against the reference's 2.62e9 at w = 0, 0.1 and 1, and the order
    holds across the band.
  - `scalars_verified` is true for the sample above.
- **The gate can pass only when all three of these hold:**
  - the 550 raw files named in `raw_data` are committed byte-identical (F4);
  - the `validation/` outputs are committed;
  - the manifest sits in the evidence directory of the row it backs.
- **Before any row is shown, the producer should fix the x5\* label (F1).** Put the correction in a
  sibling `ERRATA.md`, not in the pinned RESULTS.md (lesson L-8d86e8fc352c).
- **What the gate script says now.** `python3 scripts/validation_gate.py evidence research/genus_ladder_scaling_20261119/validation` prints `valid` in this worktree. It finds the untracked rows on disk. In a clean clone, 219 of the `raw_data` paths do not exist. So the gate **cannot pass yet**.

## Reproduce

```
cd research/genus_ladder_scaling_20261119/validation
python3 -I scripts/freeze_check.py      # freeze, D1, binary, raw manifest, timeline
python3 -I scripts/curves_check.py      # coverage, selection, #J = r certificates, automorphisms
python3 -I scripts/plan.py              # nonce -> sample (out/plan.json, out/jobs.txt)
python3 -I scripts/run_jobs.py          # replays, one process at a time (JOBS_FILE=jobs_extra.txt for the extras)
python3 -I scripts/compare_replay.py; python3 -I scripts/compare_extra.py; python3 -I scripts/certs.py
python3 -I scripts/recompute.py; python3 -I scripts/unit_sensitivity.py; python3 -I scripts/compare_claims.py
python3 -I scripts/make_report.py       # validator-report.json, replay-certificate.json, VALIDATION.json (draft)
```

- **Not committed.** The export dumps `out/replay/*_core.jsonl` are 0.3–0.6 MB each and are ignored.
  `out/certs.json` keeps their sha256 and the scalar fields of every solve.
- **Lessons applied:** L-99ee952350b2, L-26895912385b, L-3f39e1d8b3a9, L-546d37ca7750, L-1a5a6b64022c,
  L-d12b64af75ed, L-e388a81efa37, L-10ef4b64c2a6, L-81737122cefc, L-f6351b15d194, L-8d86e8fc352c.
