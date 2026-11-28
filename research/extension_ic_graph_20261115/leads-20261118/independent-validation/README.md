# Independent validation: nagao-e2e, Run50 deferred pair, descent-finder control

Lane key `independent-validation`, 2026-11-18, validator role. Every statement
here is a validator observation, [pcc:unverified]. No pcc claim was written, no
paid compute was used, nothing was pushed, and no producer file was changed.

## Question

Re-verify three committed artifact sets with code that the producers did not write:

- (a) `nagao-e2e/`: PCC-20261117-feaf31, 6432aa and e9f39d.
- (b) `runpod-deferred-pair-20261117-50/`: PCC-20261117-7defbb, 6224b9 and 853115.
- (c) the round-1 `leads-20261118/descent-finder/` control.

For each set, check these things:

- retained files against their manifests
- the frozen commits against the source
- the recorded scalars against the elliptic relation
- the statistics against the raw rows
- the README claims against the data

## Binding

- **Validated commit:** `6016f362e8e1c14fc8eddb080f642c72d953e694`.
  - Every validated path was byte-equal to that commit and had no untracked
    files (`results/hashes.tsv`, row `binding`).
  - HEAD later moved to `1f1397f525` and then `f3c22ede8c` because other lanes committed, but the
    validated paths did not change.
- **Validator tools:**
  - rustc 1.97.0.
  - `validate.rs` uses only the standard library. It has its own JSON reader,
    its own F_p/F_{p²}/F_{p⁶} tower, a Jacobian short-Weierstrass group law
    with a left-to-right ladder, Miller–Rabin, prices derived from the
    bitop/v1 rules, and its own statistics.
  - Hash and binding checks use `sha256sum`, `git show`, `jq` and `zcat`.
- **Producer code that was run, for reproduction only:**
  - `descent-finder/traced_descent_check.rs` and `descent_breakdown.rs`.
  - The nagao `analyze.rs` bootstrap rule, replayed in `bootstrap-replay/`
    with the frozen lock's `rand 0.8.8`.
  - None of these counts as independent evidence. They show that the committed
    sources reproduce the committed outputs.

## Dispositions

| Set | Disposition | Reason |
|---|---|---|
| (a) nagao-e2e | `insufficient_evidence` | Every recomputable number holds: the endpoint certificate, 56/56 statistics checks, the bit-exact bootstrap replay, and the input, freeze and qualifier pins. However, 104 of 109 raw payload members and all 6 binaries exist only in LOCAL custody that is absent here. None of the three claims is in `pcc/ledger.jsonl`. No external fastest rho was measured. |
| (b) Run50 | `insufficient_evidence` | All 8 arms reprice exactly from the retained primitives (114/114 checks). The 210-path source manifest holds at the measured commit. Every retained file matches its manifest. However, the three graph-arm raw records, the source tar, the binaries and the 64 MB archive are LOCAL only. N = 1. None of the claims is in the ledger. |
| (c) descent-finder | `insufficient_evidence` for the relation claims; the counted-work claims are confirmed and reproduced | The traced-control outputs reproduce byte for byte from the committed sources (both streams, the rows and `breakdown.json`), and every ratio recomputes. The decomposition certificates (factor-base relations) were not re-verified with independent cover/Jacobian code. The build is not runnable as committed. |

No certificate failed, so no set is `invalid_measurement`. Every recorded
scalar, in every retained record, is 242624768583643540. It satisfies
Q = [d]G for all three public inputs, which have identical values.

## Per-claim verdicts

Verdicts are confirmed (C), discrepancy (D) or not checkable (N). Evidence
files are under `results/`.

### (a) nagao-e2e

| # | Claim (source) | Verdict | Evidence |
|---|---|---|---|
| a1 | Every recovery returns the endpoint; Q = [d]G (README, 6432aa) | C | `ec.txt`: L is prime; [L]G = O; [d]G = Q for d = 242624768583643540. Negative controls [d+1]G and [L−d]G do not equal Q. All 24 rows carry this d. |
| a2 | 24 planned calls: 3 warmup, 6 A/A, 15 measured, all valid (README, export) | C | `nagao-stats.txt`: the key set is exact, all rows are valid and isolation_admitted, and the IC rows record 1506 residuals. |
| a3 | Medians 20.71449 / 21.10177 / 111.40204 s; minima 20.60457 / 20.84097 / 111.02453 s | C | Recomputed from `runs[]` rows. Each equals the recorded f64. |
| a4 | A/A max/min below 1.10 | C | 1.03048, 1.00988 and 1.00951. |
| a5 | unit/baseline paired geometric ratio 0.99832, bootstrap 95% 0.96725–1.02717; no E2E gain | C | Recomputed paired ratios and geometric mean. Producer bootstrap replayed bit-exact (`reproduction.txt`). Exact enumeration of all 3125 resamples gives [0.965851, 1.027166], which also contains 1. Each recorded endpoint is the geometric mean of a 5-multiset of the ratios. |
| a6 | Ratio of medians 1.01870 differs because round 1's baseline was slow | C | 1.018696. Round 1 has the slowest baseline (22.183 s) and a paired ratio of 0.942. |
| a7 | unit/rho 0.18701 (0.18372–0.18928); baseline/rho 0.18732 (0.18325–0.19370) | C | Replayed bit-exact. |
| a8 | Preregistered order: IC order reversed on odd rounds, rho rotated | C | Rounds 0–4 ran rho,baseline,unit / unit,rho,baseline / baseline,unit,rho / rho,unit,baseline / baseline,rho,unit. |
| a9 | Workers receive public input only; input sha 48584910…, 250 B, commit c1b9bd4e | C | `public.json` keys are alpha,g,l,p,q. The hash and size match `public-manifest.json` and `git show c1b9bd4e`. |
| a10 | Frozen science source 7802b7f8; no source change after the freeze | C | No file present at 7802b7f8 was modified, apart from `SESSION.md`, which only had lines appended (0 removed). The freeze commit also adds PLAN.md and the feaf31 claim. |
| a11 | Historical library fcda1e22 plus three overlay files exposing a fallible callback, with the default path unchanged | C | The overlays differ from fcda1e22 by +33/−3, +14/−2 and +4/−1 lines. The diff only adds `NagaoSolver`/`*_using` and keeps `None` → old solver. |
| a12 | Seven qualifier files keep their original bytes | C | 7/7: the git blob at fcda1e22 equals the snapshot blob, and the sha256 matches. |
| a13 | 6 worker tests and 55 qualifier tests pass; clippy clean | C, producer logs only | `validation/*.log`: 6 passed, 55 passed, `clippy-3.log` empty. The pins in feaf31 match. |
| a14 | All 109 raw members held in a hash-bound LOCAL archive, 19,071,290 B | N | The manifest lists 109 members. Five members retained in the repo match: `summary.json`, `analysis.stdout`, `binaries-before`, `binaries-after` and `runner.exit`. The archive is absent (`/workspace` does not exist on this host). |
| a15 | All 24 isolation records uncontended; within-method work signatures equal; foreign hooks zero; independent qualifier replay passes | N | The per-call raw, isolation and qualifier outputs are LOCAL only. The preflight on CPU0 shows `contended: false`, but user threads stayed on the reserved CPU. |
| a16 | The six binaries are unchanged after the run | N | The binaries are not retained. Only the producer's `sha256sum -c` log exists. |
| a17 | The data postdate the preregistration | C (weak) | feaf31 created 05:33:50Z; freeze commit 05:38:19Z; input commit 05:41:02Z; 6432aa 06:05:29Z. The 24 walls sum to 1240.46 s, which fits the runner's 1311.20 s and the 1467 s window. All timestamps are set by the producer. |
| a18 | "Canonical registration appends to actual main" (e9f39d) | D | The three claims are not in `pcc/ledger.jsonl` at 6016f362. e9f39d was appended at e7efc801 and dropped by merge bdd757487a, which resolved the ledger to main's side (`ledger.txt`). The two-entry fork file is internally valid: the entry hashes and the link to main's seq 1351 verify with `pcc.entry_hash`, and the claim digests match the claim files. |
| a19 | 6432aa producer commit 86189420 | N | That commit is not in this clone. |
| a20 | Rho reference is "the best previously observed" of four configurations | C, with caveat | In stage41, plain/b16 took 78.8 s against 217–243 s for the others. That run was seed 389, and its walk took 249,417,689 steps, 0.387× the generic expectation (see Observations). |

### (b) Run50 deferred pair

| # | Claim (source) | Verdict | Evidence |
|---|---|---|---|
| b1 | All 8 workers exit 0 and recover 242624768583643540 | C | The exit codes and `summary.jsonl` files retained in the repo match the post-call manifest. The scalar was re-verified independently (a1). |
| b2 | Original qualifier exits 0; 60,793 B qualification preserved byte for byte | C | `qualified-original.json` hashes to the `summary.json` entry in `qualified-output.sha256`. The analysis exit code is 0. This is an unsigned producer record. |
| b3 | The 8-arm table: observed bitops, inner seconds, process seconds | C | `run50.txt`: every phase reprices exactly at bitop/v1 prices derived from the formulas (field 10 bits, r 58 bits). The phase sums equal the subtotals, `run-study.json` and the README. Inner and process times match the qualified record and `*.time.txt`. |
| b4 | Saving 322,112,932,750 bitops = 5.165387153%; relations −385,877,619,300; LA +63,764,686,550 | C | Recomputed exactly. All other phases are equal between the two graph arms. |
| b5 | Descent 3,426,280,619,436 in all three graph arms | C | Recomputed. |
| b6 | Original IC < table < deferred subtotal; table has the lowest elapsed time; graph process faster than all four rho processes | C | 27.829 s against a minimum of 36.908 s. N = 1, fixed order, no warmup (`runtime-plan.json`). |
| b7 | Fastest/cheapest observed rho is plain/b16 | C, with caveat | Its walk is 0.387× the generic expectation and is identical to stage41's seed-389 walk. |
| b8 | Graph statistics 125491 / 20510 / 77994 / 8145 / 69849, witnesses 16/13, zero refusals, rank 508 at 155,648 with 518 rows | C (internal) / N (raw) | Consistent within `run-study.json` and `qualified-original.json` (77994 − 8145 = 69849; 7796 + 12714 = 20510). `pair-deferred.raw.json` is LOCAL. |
| b9 | Execution source 11c9a031 keeps 210 science paths from a8622863; published before allocation | C | 210/210 hashes match at 11c9a031 and at a8622863. The commit time is 10:35:45Z and the pod was created at 10:37:16Z. |
| b10 | Format-only successor (0ecee3) | C | The successor manifest matches 210/210 at 48d6e39c. The only change is the 4-line `pub use` relayout in `jv_sieve_two_large_prime.rs`, which equals `CI_FORMAT_ORIGINAL.log`. |
| b11 | Pre-call, pre-workers, post-call, raw and report manifests pass | C for retained files, N for the rest | 82 MATCH and 0 MISMATCH. 37 entries are LOCAL or absent: 3 graph raw records, stderr files, fixture files, cpu-stat, driver, analyzer, wrapper, source tars, bundle and `artifacts.tar.gz` (`hashes.tsv`). |
| b12 | Archive collected before DELETE 204 / GET 404 at 10:49:49Z; no owned pod remains | C (records) / N (live state) | collected-at is 10:49:48Z and deleted-at is 10:49:49Z. The Runpod API was not queried. |
| b13 | 7defbb/6224b9/853115 recorded in the ledger | D | They were appended at 48d6e39c and dropped by merge 96e50a56d3. Its first parent's ledger had 1391 lines and the merge has 1358 (`ledger.txt`). |
| b14 | Complete cost false, ratios null, no rho parity | C | `run-study.json`. Each rho arm names three `known_uncounted` counters with `count: null`, so under §5a no counted ranking is possible, and none is claimed. |

### (c) descent-finder control

| # | Claim (README) | Verdict | Evidence |
|---|---|---|---|
| c1 | Breakdown table: residuals, descent bitops, per-residual cost, shares 38.3/65.5/54.9/57.9%; reprices exact | C | `descent.txt`. Recomputed independently. `breakdown.json` reproduces byte-equal on rerun. |
| c2 | Primitive shares (field_mul 88.32%, …) and legacy stages 77.43/21.87/0.30/0.23/0.17% | C | Recomputed from `original-ic.raw.json`. The stage sum equals `descent_muls`. |
| c3 | Hooked field_mul 1,245,470,049 against 1,243,056,697 (0.19%); gap 345,496,566 | C | Recomputed. |
| c4 | The recorded descent subtotal "omits at least about 24.6% of descent work" (also "≥ ~25%" in next steps) | D (minor) | The unhooked lower bound 837,749,046,680 is 24.45% of the recorded subtotal, but 19.65% of descent work (recorded plus omitted). The 24.6% figure is the full `lin_muls × fp_mul` (844,371,046,980) over the recorded subtotal. That double-counts the 6,622,000,300 bitops the hooks already saw. The table's priced columns use the correct lower bound. |
| c5 | F4 multiplications 2.29x / 2.30x; hooked 0.451 / 0.448; with linear stage 0.559 / 0.556; plus learning bound 0.586 (1.71x) / 0.617 (1.62x) | C | Recomputed from the summaries: 2.2861, 0.4509, 0.5590, 0.5856 (1.708x) and 2.3016, 0.4475, 0.5563, 0.6165 (1.622x). The summaries reproduce byte-equal on rerun. |
| c6 | Decomposing indices 263, 3414 and 997, 1505; 0 disagreements; replays 3414 (33) and 1505 (11) | C | `rows-*.txt`: aggregated from the 3415 + 1506 rows. The sums equal the summary for all three arms, and the decomposition lists are identical across arms. |
| c7 | Stream reconstruction agrees with the recorded runs (3415 and 1506 residuals, 2 accepted) | C (counts) / N (identity) | The counts match Run50 and nagao. The streams cannot be compared element by element because the raw files are LOCAL. |
| c8 | Production hooked subtotal is 0.30% below Run50's | C | 0.2976%. |
| c9 | Oracle about 4% dearer than the staircase stop | C | 4.33% and 4.37%. |
| c10 | Every solution satisfies the six quadrics; every decomposition passes `verify_dec`; R = aG_J + bQ_J | N | The rows do not carry quadrics, solutions or factor-base points. No independent cover or Jacobian code exists in this lane. The relation certificates were not re-verified. |
| c11 | SHA-256s in BUILD_MANIFEST and RAW.sha256 | C | RAW 14/14. Frozen sources 15/15 equal `git show fcda1e22`. Run50 inputs 5/5 match. The 18 assembled source files are byte-identical after path-substituted reassembly. The Cargo.toml hash reproduces with bin path `/dev/shm/df-lane/…`, and the Cargo.lock hash reproduces by offline resolution. |
| c12 | `mini-build/build.sh OUT` rebuilds the control | D | Not runnable as committed. It copies `$here/Cargo.lock`, which was never committed, under `set -e`, and `fetch_frozen.sh`/`assemble.py` hard-code an absent worktree path. BUILD_MANIFEST does not hash the compiled `traced_descent_check.rs` (a `/dev/shm/df-lane` copy). This validation closes that gap by rerunning the committed source to byte-identical outputs. |

### Lab checks

- `python3 tools/pcc/pcc.py canary`: 69/69 classes behave as required.
- `python3 tools/pcc/pcc.py ledger verify`: 1367 entries, chain ok, exit 0.
- `pcc verify` gives `not_covered` for all six claims, because they carry
  attestation proofs only.
- `pcc standing` reports all six claims as unverified.
- `ledger verify` passes even though 2588 of the 3584 claim files at 6016f362
  have no ledger entry (`ledger.txt`).

## Controls and reporting contract

| Requirement | (a) nagao-e2e | (b) Run50 |
|---|---|---|
| Null / A-A control | Present: A/A ≤ 1.031 | Absent: N = 1, no warmup (stated) |
| Cheaper/unmodified baseline | Present: six-variable IC | Present: grounded pairs, original IC, table |
| Reference arm | Registered native rho, priced in wall time | Four native rho configurations |
| External fastest implementation | Absent (disclaimed); blocks any parity claim | Absent (disclaimed) |
| §5a band | Not computed by the producer: complete counts are null and the claim is wall-only. Lesson L-546d37ca7750 does not apply. | Not computed: ratios are null and rho `known_uncounted` counts are null, so the arms cannot be ranked by count. |
| One table, one unit, ratio | Yes: complete process wall seconds, paired ratio | One table, but it shows incomplete bitops next to wall seconds. No ratio is claimed. |
| Leaderboard / benchmarks gate | No row and no VALIDATION.json (correct) | Report copied to `benchmarks/20261117-runpod-deferred-pair/` with no README row and no VALIDATION.json (correct) |

## Observations (interpretation, labelled)

### The rho reference was chosen on one lucky walk

Run50's raw rho records give each walk's length as a multiple of the generic
expectation (√(πL/2) plain, √(πL/4) with negation):

| Configuration | Walk length vs expectation |
|---|---:|
| plain/b1 | 0.913 |
| neg/b1 | 1.176 |
| plain/b16 | 0.387 |
| neg/b16 | 1.356 |

The plain/b16 walk has the same seed (389) and the same 249,417,689 steps as
stage41's. That one short walk is the basis for "best previously observed" and
for nagao's reference choice.

Combining the observed seconds per step on the Run50 host with the generic
expected walk gives these expected walls (a model, not a measurement):

| Configuration | Expected wall |
|---|---:|
| neg/b16 | about 75.9 s |
| plain/b16 | 95.2 s |
| neg/b1 | 99.9 s |
| plain/b1 | 131.2 s |

The cheapest configuration in expectation is probably negation/b16, not
plain/b16. nagao's rho used seed 401, whose walk length is in LOCAL raw only.

### The nagao bootstrap is discrete

With 5 paired rounds the bootstrap has only 126 attainable values. The
10000-draw interval [0.96725, 1.02717] differs at its lower end from the exact
[0.96585, 1.02717]. The conclusion (the interval contains 1) does not change.

## Commands run

All commands ran from the repo root.

```
git rev-parse HEAD                      # 6016f362e8…
python3 tools/lessons.py brief --scope validator
python3 tools/pcc/pcc.py canary ; python3 tools/pcc/pcc.py ledger verify
python3 tools/pcc/pcc.py standing|verify PCC-20261117-{feaf31,6432aa,e9f39d,7defbb,6224b9,853115}
bash research/extension_ic_graph_20261115/leads-20261118/independent-validation/run_all.sh \
     /home/user/crypto 6016f362e8e1c14fc8eddb080f642c72d953e694 SCRATCH --repro
bash research/extension_ic_graph_20261115/leads-20261118/independent-validation/ledger_trace.sh \
     /home/user/crypto 6016f362e8e1c14fc8eddb080f642c72d953e694 > …/results/ledger.txt
```

`run_all.sh` does the following:

- builds `validate.rs` with `rustc -O`
- runs `ec`, `nagao`, `run50`, `descent` and `rows`
- runs `check_hashes.sh`

With `--repro` it also:

- reassembles the mini-build into scratch with paths substituted, and builds it
  offline into `CARGO_TARGET_DIR=/home/user/cargo-target-round2/independent-validation`
  (since deleted)
- reruns `traced_descent_check` on seed 389 / n 3415 and seed 401 / n 1506
- reruns `descent_breakdown`
- builds and runs `bootstrap-replay/`

The run took about 98 s in total, about 46 s of it in the two descent runs.
That timing is indicative and is not reported as a result. The fork-chain check
used `pcc.entry_hash` and `pcc.claim_digest` imported from `tools/pcc/pcc.py`.

## Failures and deviations

- **research-kb unavailable.** `scripts/research-kb` reports "no executable
  found", and no research-kb MCP tool is loaded. No literature claim is made
  here.
- **LOCAL custody is absent.** `/workspace/nagao-e2e-runs` and
  `/workspace/ic-runpod-runs` do not exist on this host, so every
  LOCAL-only artifact is not checkable.
- **Producer code was run.** The descent-finder reproduction and the bootstrap
  replay use producer code. They establish reproducibility, not independence.
- **Lessons ledger.** Outcome, test and new-lesson records went to
  `lessons/` in this lane (`tools/lessons.py --dir`), because this lane may not
  write `docs/lessons/entries/`. The orchestrator can move them.

## Next steps and leads

1. **Ledger repair (owner or orchestrator).** Append fresh ledger entries for
   the six lane claims and for 0ecee3. Never rewrite existing entries.
   - Separately, add a check to `pcc ledger verify` that every
     `pcc/claims/*.json` has a ledger entry (2588 orphans now), and a
     canary for a "take main" merge that drops ledger lines.
   - `pcc.py` is under the cross-repo shared-code lock, so this needs an owner
     decision.
2. **Custody.** Push the nagao (19 MB) and Run50 (64 MB) LOCAL archives to the
   artifact store while they still exist on the producing hosts. Until then,
   claims a14–a16, b8 (raw) and b11 (absent files) stay not checkable.
3. **Rho reference by expected cost, not by best walk.** Draft a protocol that
   measures each configuration's per-step cost and multiplies it by the
   expected walk length, or runs K seeds per configuration. Run50's per-step
   data point to negation/b16.
4. **Independent relation verifier.** Have descent rows also emit the
   factor-base points (or their Mumford coordinates) together with G_J and Q_J.
   Then write a validator-owned Rust cover/Jacobian check so that c10 becomes
   checkable.
5. **descent-finder build hygiene.** Commit `Cargo.lock` and relative paths, and
   hash the compiled `traced_descent_check.rs` in BUILD_MANIFEST. Also correct
   the "24.6% of descent work" wording (c4).
6. **Exact bootstrap.** For designs with five or fewer rounds, report the exact
   enumerated bootstrap beside the Monte Carlo interval.

## Sources read

- `AGENTS.md`: sections "Preserve scope and report evidence", "The rule:
  boundary, table, ratio", §5a, §12, §13, "Model tiers" and "Sabotage is
  assumed".
- The lane READMEs, PLAN, SESSION, manifests and claim files named above.
- `nagao-e2e/native/analyze.rs`.
- `library-overlay/*` diffs against fcda1e22.
- `src/cryptanalysis/bitops.rs` (bitop/v1 rules).
- `descent-finder/{README,BUILD_MANIFEST,mini-build/*,descent_breakdown.rs,traced_descent_check.rs}`.
- stage41 `runpod-extension-pipeline-20261116-41/` rho raw records and fixture.
- `validation/deferred50-*.sha256`.
