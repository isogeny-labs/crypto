# Validator report: multi-target IC v4, icbest vs standard multi-target rho (2026-11-12)

Validator: independent `validator` subagent (Opus 5.5), 2026-11-12. All validator code is in
`scripts/`; none of it is the producer's.

**Verdict: `valid_with_amendments` for the measurement. Disposition of the headline "the
multi-target IC is dearer than standard multi-target rho in all 27 section-5a decisions":
`insufficient_evidence` (blocking).** `validator-report.json` and `claim-report.json` have
status `insufficient_evidence`, so the gate refuses them. `replay-certificate.json` is
`verified`.

- **The measurement is sound.**
  - All 3960 rows replay exactly. The validator also replayed a nonce sample of 60 rows on a
    pristine build and 15 rows on the producer's binary: every one matches.
  - 1,290,240 recovered logs were verified with the validator's own EC code. So were 693,769
    logs that the retention arms claim for stored points. There were no failures.
  - All 972 `summary.json` entries and all 153 numbers in the RESULTS table and `noverify.txt`
    recompute exactly.
  - The protocol was frozen before the data. The curves are fresh and regenerate byte-for-byte.
- **Reproduced:** under the frozen rule, icbest is dearer than rhobest in all 27 decisions.
  The frozen rule charges h only on hash-table and solver operations.
- **Not validated as stated.** The result depends on two choices. Each one changes the
  decisions.
  1. **§5a pricing.** rhodp is the h = 1 reference. Its per-step canonicalisation, step-index
     hash and DP test are neither counted nor priced: 7.27e9 canonicalisations against
     7.66e8 priced hash operations. AGENTS.md §5a and ecbench (`band.rs`) price rho's
     canonicalisations in the band. With them priced at h, all **27 decisions become
     unranked**. At h = 1 and I = 10 M, the IC is 0.888–0.935x the cheapest rho.
  2. **The IC's retention arm is degenerate.** ic5ks's trials use one shared
     arithmetic-progression W. Its retained classes therefore cover O(K) target logs, not
     O(K·K'). A one-line validator variant (ic5rw: non-progression W, 32 counted scalar
     multiplications) costs 0.79–0.93x of icbest at I = 10 M, h = 0. With it the IC is
     unranked against the frozen rhobest in all 27.

  Against stronger, better-tuned references the frozen-rule gap widens. Even so, under the §5a
  reading no ranking against rho is established.

## 0. What was validated, and from where

- **Frozen artifact set:** commit `bb3d2bc40ee75de9b0623f919391832f4a1d5669` (branch
  `aburan28/eloquent-bell-2jerrf`).
  - Extracted with `git archive` into the validator scratchpad: `research/mt_fieldops_v4_20261112`
    and `examples/mt_fieldops.rs` (sha256 `c6b694bc…e639a`, the same as at the protocol commit
    `25a2540781`).
  - The study directory was clean in the working tree.
  - HEAD moved during validation to `7c33638efc`, through `f96fce6b81` (bench record) and
    `7c33638efc` (leaderboard builder). Neither later commit touches the study or the harness,
    and neither is validated here.
- **Measured binary:** `target/release/examples/mt_fieldops`, sha256 `d8fdc720…612c`. This
  equals `bin_sha256` in all 3960 rows. The rows record commit `9ce79930e7`, which added only
  the bench record after the freeze.
- **Validator builds:** standalone crates from `git show bb3d2bc40e:examples/mt_fieldops.rs`,
  built with rustc 1.97.0, rand 0.8.8 and serde_json 1.0.151 (`arbitrary_precision`,
  `raw_value`), the versions in `Cargo.lock`.

  | build | sha256 | what it adds |
  |---|---|---|
  | pristine | `2e28a6e1…a52b` | nothing |
  | instrumented (`scripts/patch_instr.py`) | `32712a8a…2c31` | no counted operation. It counts HashMap calls, canonicalisations, step-index hashes and DP tests, and dumps targets and retention-log samples to stderr |
  | sensitivity | `c0295664…a6f` | arms `ic5rw` and `ic5w` |
  | sensitivity | `68ef13ec…205` | arm `rhodpu` |

- **Host:** local, 4 cores, Python 3 with numpy. Only counts are used; there is no wall-clock
  figure.
- **Command sequence:** `scripts/run_all.sh`. The nonce `6f8349fedd9cf3d9041ce040e08eb849` was
  drawn with `os.urandom` before any sampling.

## 1. Replay (`replay-certificate.json`)

| replay | rows | mismatching | ok ≠ targets | phases non-monotone | last phase ≠ totals |
|---|---|---|---|---|---|
| every holdout row, instrumented build | 3960 | 0 | 0 | 0 | 0 |
| nonce sample, pristine build (4 per arm × bits) | 60 | 0 | 0 | 0 | 0 |
| nonce sample, producer binary (1 per arm × bits) | 15 | 0 | 0 | 0 | 0 |
| one configuration per (bits, T) cell, all 20 arms, with dumps | 180 | 0 | 0 | 0 | 0 |

- **Fields compared:** every field the binary prints except `ms`: I, M, S, H, L, ok, every
  phase snapshot and every arm extra.
- **Inputs:** runs were rebuilt from the `argv` in `rows.jsonl`. Instance paths were mapped to
  the frozen copy, and every instance hashes to `MANIFEST.json`.
- **`runs.json`:** an exact projection of `rows.jsonl`. Its `source_sha256` matches. Field `g`
  is always null.

## 2. Scalars and retained logs (certificate discipline)

- **Recovered logs** (`scalar_check.json`, `scripts/ecverify.py`, `scripts/ecv.py`):
  - Scope: 180 runs with 1,290,240 targets, covering every arm and all of 24, 28 and 32 bits.
  - Each target was checked for:
    - Q_t on the curve;
    - 0 ≤ log < n;
    - log = planted x_t;
    - [log]G = Q_t, using the validator's Jacobian double-and-add code, which shares nothing
      with the harness.
  - Result: 0 failures. The planted lists are identical across the 20 runs of each
    configuration.
- **Logs the retention arms claim for stored points** (`retention_log_check.json`,
  `scripts/verify_samples.py`):
  - Sample: x % 211 = 0, at most 300 per run, from every run.
  - Checks:
    - rho2xks retention: [log]G = (x, y), or the stored sibling prev − A_h.
    - rhodp precomputation and retention: [log]G = the canonical point, with y ≤ (p−1)/2.
    - ic5ks retention: x([log]G) = x.
  - Result: 693,769 logs checked, 0 failures.

## 3. Cost accounting of the new arms (code review plus instrumented counts)

### 3.1 `rho2xks`
- **What it is:** `rho2x` plus Kuhn–Struik retention.
- **Counted work:**
  - Every step is I + 3M + 2S, plus two counted lookups (x and the sibling x).
  - After a solve, every trail point is inserted with log = off + v, at one counted hash each.
  - On a hit, the sibling y is recomputed by a counted addition.
- HashMap calls equal H in all 594 runs.
- No free information was found.
- **Weak spot:** the restart overhead (one counted addition per 32 steps) is kept, which
  favours the IC.

### 3.2 `rhodp`
- **Verdict:** a correct multi-target distinguished-point rho.
  - Negation-map walk on canonical classes. The step is a deterministic function of the class,
    look-ahead included, so walks merge.
  - DP database. Own-walk collisions with the opposite sign are solved as q = (a2 − a)/(2b),
    with one counted M mod n.
  - Pending DPs are inserted with log a + b·v after the solve.
  - Optional known-log precomputation.
  - Every addition is counted (look-ahead and abandoned walks included), and so is every
    lookup and insert.
  - Canonicalisation is correct: y > (p−1)/2 is mapped to p − y, and there is no 2-torsion.
- **Uncounted work (V4-F1):** per computed point there is one canonicalisation; per step there
  are two step-index hashes and one DP test.
  - Over 2376 runs these total 7,266,086,859 canonicalisations, 14,018,374,537 step-index
    hashes and 7,009,187,296 DP tests. The priced hash operations total 766,313,717.
  - That is 37 to 5886 canonicalisations per target.
- **Reference handicap (V4-F4):** fruitless cycles are not fully caught.
  - The 1024 steps are B_j + B_k with j and k uniform over 64 points. About 39% of the steps
    (simulated) share their value with another index.
  - A sign flip followed by such an index is a fruitless 2-cycle that the same-index
    look-ahead misses. The walk is then abandoned after 20·2^d counted steps.
  - Abandoned walks take 1–34% of rhodp's steps: 5–20% at 32 bits for the offsets that win at
    h = 1.
  - The validator's `rhodpu` (distinct step pairs, nothing else changed) cuts abandonment to
    4.5e-5 per walk. Its best h = 1 cost is 0.81–0.99x of rhodp's (median 0.92).
- **Reporting defect (no cost effect):** the `target_steps` field omits each target's solving
  walk. Costs come from the field counters, so they are unaffected.

### 3.3 `ic5ks` (is the IC handicapped?)
- **Accounting:**
  - It is ic5, plus insertion of the solved targets' trial classes (one hash each).
  - A hit on a retained class costs a counted recomputation of Q_t2 ± W_k2.
  - It does two lookups per trial class and counts one: 1.17e9 HashMap calls against
    H = 8.88e8. One merged table, with retained entries overwriting table entries, gives the
    same answers with one probe, so the count is defensible. It favours the candidate.
- **The retention is degenerate (V4-F2).** All targets try the same progression
  W_k = W_0 + kD.
  - A retained class x(Q_t ± W_k) meets a later trial x(Q' ± W_k') only if Q' ∓ Q_t lies in
    {jD} ∪ {±(2W_0 + jD)}.
  - That is O(K) target logs per solved target, against O(K·K') for a spread W.
  - Exact count (`coverage.json`, K = K' = 1000): 11,994 distinct solvable target logs against
    3,292,090.
  - So retention gains ic5 at most 1.5%. RESULTS' explanation ("ic5's table is fixed after
    phase 1") names the wrong mechanism.
- **Validator arm `ic5rw`:** ic5ks with W_(k+1) = W_k + D_(splitmix64(k) mod 32) over 32 random
  known-log points (32 counted scalar multiplications); nothing else changes.
  - Run at mu ∈ {1.0, 0.85, 0.7, 0.6, 0.5}: 990 runs, every target verified by the harness.
  - At I = 10 M, h = 0 it costs 0.791–0.928x of icbest (median 0.834).
  - The control `ic5w` (same W, no retention) is 1.001x ic5, so the gain comes from retention.
  - mu = 0.5 is at the grid floor, so this arm is not fully tuned either.
  - **First attempt (validator bug):** the W index was driven by x(W). That W is itself a rho
    walk, so it cycles after about √n terms, and 9 of 1584 runs hit the trial budget. The index
    was changed to depend on the position k, and the whole panel was rerun: 0 errors.
- No free information was found in any arm. The planted logs are used only for planting
  (uncounted field copy) and in the final check.

## 4. Recompute (`recompute.json`, `results_check.json`)
- **Method:** the validator's own script (`scripts/recompute.py`) works from `runs.json`. It uses
  the bootstrap that analyze.py states, plus an independent numpy bootstrap (B = 20000, seeded
  from the nonce).
- **summary.json:** 972 of 972 entries match, with a maximum relative difference of 0. No
  decision depends on the bootstrap draw.
- **Decisions under the frozen rule:**

  | candidate | vs rhobest | vs rho2xks | vs rhodp | vs bsgs2 |
  |---|---|---|---|---|
  | icbest | dearer 27 | unranked 27 | unranked 27 | dearer 27 |
  | ic5 | dearer 27 | unranked 27 | unranked 27 | dearer 27 |
  | ic5ks | dearer 27 | dearer 27 | unranked 27 | dearer 27 |

- **Ranges:**

  | quantity | recomputed | as stated |
  |---|---|---|
  | icbest/rhobest, all cells, c_I and h | 1.0032–1.8101 | matches |
  | I = 10 M, h = 0 | 1.0394–1.1557 | matches |
  | I = 10 M, h = 0.1 | 1.0280–1.0830 | matches |
  | I = 10 M, h = 1 | 1.1973–1.7565 | matches |
  | icbest/bsgs2 | 1.1431–1.4946 | wrong: should read **1.14–1.49**, not 1.50 |
  | noverify, I = 10 M, h = 0 | 1.0946–1.1646 | wrong: should read **1.09–1.16**, not 1.10–1.17 |
  | noverify, I = 10 M, h = 0.1 | 1.0474–1.0900 | matches |
  | noverify, I = 10 M, h = 1 | 1.3203–1.7702 | matches |

  - The Answer's "4% to 76%" at I = 10 M should be **3% to 76%**, since h = 0.1 reaches 1.028.
- **Which rho wins (I = 10 M), reproduced:**
  - rho2xks at κ = 0.5 wins at h = 0 and h = 0.1 in all 198 configurations.
  - rhodp wins at h = 1 in all 198 (κ 0 in 96 and 0.25 in 102; offsets −1 to 1).
- **Shared verification phase:** 34.5–47.5 GAE per target at I = 10 M, for every arm.

## 5. Protocol (V4-F7)
- **Freeze order.** PROTOCOL.md, run.py, analyze.py, the instances and the harness are
  committed in `25a2540781` and unchanged through `bb3d2bc40e`.
  - The data rows record `9ce79930e7`, a bench-record-only commit made after the freeze.
  - Only plot.py changed after the data, and only the figure layout.
- **The grid** is complete: 3960 = 9 cells × 22 configurations × 20 runs. Seeds 31 and 32 are
  as frozen, and no row was dropped.
- **Instances:**
  - All 33 curves regenerate byte-identically from `gen_instances.py --families W --bits
    24,28,32 --per 11 --seed mtfo-v4-20261112`.
  - All are valid: p prime, n prime, cofactor 1, nG = O, the fixture log is correct.
  - There is no overlap with 3312 curve records under `research/` and `docs/curves`.
  - PROTOCOL's "one 32-bit-field curve has an order just above 2^32" is wrong: 7 of 11 do, and
    6 of 11 at 24 and at 28 bits are just above 2^24 and 2^28.
- **"Symmetric favour to the candidate" is inaccurate.** The IC gets 2 runs per configuration
  and rho gets 15. Choosing the rho parameters on the other seed (cross-fitting) still leaves
  all 27 dearer under the frozen rule, so this asymmetry does not drive the result.

## 6. Robustness (`sensitivity.json`; not pre-registered)

The decision rule and bootstrap are exactly the protocol's. Each count is a (cell, c_I)
decision count out of 27.

**Pricing:**
- **P0:** the frozen rule.
- **P1:** P0, plus h per rhodp canonicalisation (the ecbench §5a convention).
- **P2:** P1, plus h per step-index hash and DP test.

**Candidate sets:**
- **C0:** icbest, the cheaper of ic5 and ic5ks.
- **C1:** C0 plus ic5rw.

**Reference sets:**
- **R0:** rhobest, the cheapest of the 15 frozen runs.
- **R1:** R0 plus rho2xks at κ 0.05, 0.1 and 0.25 (594 runs). At I = 10 M and h = 0, κ ≤ 0.25
  beats κ = 0.5 in all 198 configurations; κ 0.05 or 0.1 is best in 195 of them.
- **R2:** R1 plus rhodpu on rhodp's full grid (2376 runs).
- **x:** parameters cross-fitted on the other seed.

| candidate vs reference, pricing | decisions (27) | I = 10 M medians h = 0 / 0.1 / 1 |
|---|---|---|
| C0 vs R0, P0 (as published) | dearer 27 | 1.04–1.16 / 1.03–1.08 / 1.20–1.76 |
| C0 vs R0, **P1** | **unranked 27** | 1.04–1.16 / 1.03–1.08 / **0.89–0.94** |
| C0 vs R0, P2 | unranked 27 | 1.04–1.16 / 0.98–1.02 / 0.71–0.79 |
| C0 vs R0x, P0 | dearer 27 | 1.04–1.16 / 1.03–1.08 / 1.19–1.72 |
| C0 vs R2, P0 | dearer 27 | 1.09–1.28 / 1.07–1.15 / 1.21–2.03 |
| C0 vs R2, P1 | unranked 20, dearer 7 | 1.09–1.28 / 1.07–1.15 / 0.90–1.06 |
| C1 vs R0, P0 | unranked 27 | 0.90–0.96 / 0.93–0.98 / 1.20–1.76 |
| C1 vs R2, P0 | dearer 14, unranked 13 | 0.99–1.05 / 0.99–1.03 / 1.21–2.03 |
| C1 vs R2, P1 | unranked 19, cheaper 6, dearer 2 | 0.99–1.05 / 0.99–1.03 / 0.90–1.06 |
| C1x vs R2x, P1 | unranked 19, cheaper 6, dearer 2 | 0.99–1.05 / 0.985–1.035 / 0.90–1.04 |
| C0 vs rho2xks alone, P0 | unranked 27 | 1.04–1.16 / 1.03–1.08 / 0.87–0.91 |
| C0 vs rhodp alone, P0 | unranked 27 | 0.71–0.85 / 0.83–0.89 / 1.20–1.76 |

What the table shows:
- **Robust:**
  - Under the frozen rule, the frozen IC arms are dearer than this harness's rho envelope.
  - Stronger rho arms (R2) widen that gap.
  - Cross-fitting does not change it.
- **Not robust:**
  - "The IC is dearer than standard multi-target rho in all 27 §5a decisions" fails once
    rhodp's canonicalisations are priced, and fails again once the IC is given a working
    retention.
  - With the strongest references and the §5a reading, the outcome is mixed: 19 unranked,
    6 cheaper, 2 dearer.
- **Single rho arms:** no single rho arm beats the IC across the band. The published
  "dearer" verdict is against the per-price envelope of two different algorithms: rho2xks at
  h ≤ 0.1 and rhodp at h = 1 (V4-F3).

## 7. Findings
Full text with numbers is in `validator-report.json`.

| id | severity | finding |
|---|---|---|
| V4-F1 | blocking | rhodp's canonicalisations, step-index hashes and DP tests are uncounted and unpriced, contrary to §5a. Priced, all 27 decisions are unranked, and the IC is 0.89–0.94x rho at h = 1 |
| V4-F2 | blocking | ic5ks's retention is degenerate (shared progression W). ic5rw is 0.79–0.93x icbest. C1 vs R0 is unranked in all 27 |
| V4-F3 | major | Against either rho arm alone the IC is unranked in all 27. rhobest is a per-price envelope, which RESULTS does not say |
| V4-F4 | major (direction favours the IC) | rho2xks's κ grid has its best at the floor. rhodp loses 1–34% of its steps to abandoned walks (duplicate steps). Stronger references widen the frozen-rule gap |
| V4-F5 | info | No free information. Retained logs are correct. HashMap calls equal H (ic5ks counts two lookups as one: defensible, and it favours the IC) |
| V4-F6 | minor | Prose rounding: BSGS should read 1.14–1.49 (not 1.50), noverify h = 0 should read 1.09–1.16, and the Answer should read "3% to 76%" |
| V4-F7 | minor | Protocol curve-order remark is wrong (7 of 11). "Symmetric favour" is inaccurate, but cross-fitting shows it is harmless |
| V4-F8 | process | rows.jsonl (2.9 MB) is in Git, not the artifact store. The bench record was empty at the validated commit. runs.json and noverify.txt have no committed generator. The leaderboard builder was switched to v4 before validation |

## 8. Required amendments
1. **BLOCKING (V4-F1).** Change how the 27 decisions are described:
   - Stop calling them section-5a decisions. Describe them as "dearer in all 27 decisions of
     the frozen rule (h on hash-table and solver operations only)".
   - Report the §5a reading next to them: with rhodp's canonicalisations priced at h, all 27
     are unranked, and the IC is 0.89–0.94x the cheapest rho at h = 1, I = 10 M.
   - Emit the counters from the harness: canonicalisation, step-index-hash and DP-test counts
     for rhodp, and step-index-hash counts for rho2xks.
2. **BLOCKING (V4-F2).** Retitle RESULTS and replace three passages with "no ranking against rho
   is established":
   - "the IC is dearer than standard multi-target rho in every cell";
   - "rho wins";
   - "Beating rho here would need a non-generic property of the curve".

   Also correct the "Why" paragraph: it should name the degenerate retention (shared
   arithmetic-progression W), not an IC table fixed after phase 1.
3. **V4-F3.** State that against either rho arm alone the decision is unranked in all 27, and
   that rhobest is the per-price envelope.
4. **V4-F4.** Disclose the handicaps of both reference arms and which way each one pushes the
   result.
5. **V4-F6 and V4-F7.** Fix the numbers:
   - 1.14–1.49x BSGS ("14% to 49%");
   - noverify at h = 0: 1.09–1.16;
   - the Answer at I = 10 M: "3% to 76%";
   - the curve-order remark: 7 of 11 curves.
6. **V4-F8.** Clean up the process gaps:
   - Move rows.jsonl to the artifact store.
   - Commit the generator for runs.json and noverify.txt.
   - Keep the leaderboard's v4 multi-target section "not validated" until a passing validator
     report exists.

**Accepted RESULTS headline:**

> Under the frozen rule (h on hash-table and solver operations only), this harness's
> multi-target IC (the cheaper of ic5 and ic5ks) is dearer than the cheapest of 15 runs of its
> multi-target rho in all 27 (cell, I/M) decisions. At I = 10 M it is 1.04–1.16x at h = 0,
> 1.03–1.08x at h = 0.1 and 1.20–1.76x at h = 1 (1.003–1.810x overall). It is 1.14–1.49x a
> paired BSGS. Against either rho arm alone it is unranked in all 27. With rhodp's per-step
> canonicalisations priced at the same h, as AGENTS.md §5a requires, all 27 decisions are
> unranked: the IC is 0.89–0.94x the cheapest rho at h = 1, I = 10 M. With a non-degenerate
> retention for the IC (validator arm ic5rw), the IC is unranked against the frozen rhobest in
> all 27, and near parity at h = 0 and 0.1 against a better-tuned rho. No ranking against rho
> is established.

## 9. The claim statement (`claim-report.json`)
**Status `insufficient_evidence`: not endorsed as worded.** Three clauses fail:
- "dearer in all 27 … section-5a decisions" (V4-F1);
- "the multi-target index calculus (cheaper of ic5 and ic5 with retention)" (V4-F2), because
  that retention arm is degenerate;
- "1.14–1.50x a paired BSGS": the range is 1.14–1.49.

Every other number in the statement recomputes. The validator would endorse this accepted
wording:

> On 33 fresh prime-order Weierstrass curves (group orders about 2^24, 2^28 and 2^32), T = 1024/4096/16384, 22 paired configurations per cell, counted F_p and mod-n operations (I = 3/10/30 M, S = M) with hash-table and solver operations at 0/0.1/1 GAE: this harness's multi-target index calculus (the cheaper of ic5 and ic5ks per configuration and price; ic5ks's retention of solved trial classes along the shared arithmetic-progression W saves at most 1.5%) costs 1.003-1.810x per target of the cheapest of 15 runs of this harness's multi-target rho per configuration and price (Kuhn-Struik retention rho rho2xks at h = 0 and 0.1, distinguished-point rho rhodp at h = 1) and is dearer in all 27 cell x I/M decisions of the frozen rule; at I = 10M it is 1.04-1.16x at h = 0, 1.03-1.08x at h = 0.1 and 1.20-1.76x at h = 1; it is 1.14-1.49x a paired BSGS. The frozen rule prices h on hash-table operations only: with rhodp's per-step canonicalisations priced at the same h (AGENTS.md section 5a) the IC is 0.89-0.94x the cheapest rho at h = 1 and I = 10M and all 27 decisions are unranked, so no ranking against rho is established.

## 10. Limits of this validation
- The ic5rw, ic5w, rhodpu and low-κ rho2xks arms are validator sensitivity arms. They were not
  pre-registered, and only one validator has checked them. Their targets were verified by the
  harness's counted check (the same check whose answers the validator verified independently
  for the producer's arms). Their own scalars were not re-verified with the validator's EC code.
- rhodpu's canonicalisation count comes from its walk-phase inversions minus the setup
  inversions of rhodp on the same curve and seed. That identity holds to ±2 per run on the
  replayed rhodp rows.
- Pricing a canonicalisation, a step-index hash or a DP test at h GAE is the §5a band's
  convention, not a measured cost. P1 prices only canonicalisations, which is the minimal
  reading; P2 is the maximal one.
- Counts only. Memory is unbounded for every arm. Groups are about 2^24 to 2^32. Nothing here
  bears on single-target ECDLP or on ECC2K-130 / m = 83.
