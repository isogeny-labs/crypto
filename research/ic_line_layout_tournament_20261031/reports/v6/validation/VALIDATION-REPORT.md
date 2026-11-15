# Validation report VAL-20261106-9ce3e6: IC line-layout tournament v6 (fp28), as amended by V6-AMENDMENT-1

Validator: independent Validator role (claude-opus-5-5), 2026-11-06. Protocol
`IC-LINE-LAYOUT-E2E-FP28-V6-20261106`, amendment V6-AMENDMENT-1 (merged in PR #3531).
Frozen contract read at main `e87fa18a`. The checkout is now at `4a3461a7f`, and the study
directory is byte-identical at both revisions. Machine-readable results:
`validation.json` (one entry per check), `recompute.json`, `verdict-rederived.json`,
`descriptive.json`; scripts in `scripts/`.

**Terminal verdict: `passed`, with caveats.** The receipts are admissible evidence. The
reported outcome is confirmed number for number, and the reported verdicts follow from
the frozen rules. Like any passed validation, this supports no ECDLP claim, speedup or
promotion. Here the evidence shows the candidate is slower.

## Bottom line in plain words

The data are genuine and intact, and the analysis was done correctly. Every session replays
exactly. My own independent recomputation reproduces every reported number, to the last
floating-point digit. The candidate (`layout=lines`) does exactly the same counted work as
the incumbent: every record agrees field by field. It is also about 12% *slower* in wall
time in all three sessions, on both curves and on both hosts (ratio 1.121–1.123, 95% CI
inside [1.118, 1.126]).

Session s1 fails the frozen A/A gate by a hair. Its incumbent-vs-twin interval
[0.99687, 0.99992] stops 0.00008 short of 1. Under the frozen rule that makes s1
`invalid_measurement`, and so the combined v6 verdict is `invalid_measurement`. Sessions s2
and s3 are valid and map to `inconclusive`, because the frozen vocabulary has no
"regression" label. Their direction (slower) must be stated descriptively. The gap to
matched rho is 22.3–23.0x in counted S across the GAE band.

## 1. Artifacts, hashes, binary, specs

| Check | Result |
|---|---|
| `SHA256SUMS` of s1, s2, s3, counted | pass. All files verify, and the file set equals the list (19/19/19/18 files). |
| S3 anchor (validator read S3 directly, read-only) | pass. `SHA256SUMS` and `job-exit.json` of all four jobs are byte-equal to the S3 originals. Each job prefix has one pod dir. Only five v6 jobs exist (qual, s1, s2, s3, counted); there is no `-r2` or `-nr`. |
| Binary | pass. `bin-pinned/ecbench` sha256 is `73362eed…31dd01`, the same value as every `binary.sha256` and every record's `binary_sha256`. |
| Image → binary extraction | caveat. Executor-attested (`run/analysis-step-E.out`, layer 5 of image `3c4ad8bf…`). Not redone here. |
| Spec equality | pass. `input-spec.json` and `session/spec.json` are `jq -S -c`-equal to `spec-fp28-v6.json` (`0116954f…`) for s1/s2/s3 and to `spec-fp28-v6-counted.json` (`ac231ee6…`) for counted. Each request's embedded spec is equal too. |
| Build | pass. `build-info.json` has source `ea8e9ef7…` and ecbench `73362eed…`, equal to each request's `job.build`. |
| Requests | pass. `request-session-3.json` differs from session 1 only in `job.job_name` and the provenance `harness` label (s2 likewise). |
| Freeze | pass. `experiment_protocol.py hash` reproduces `protocol_sha256` `2aedaaf8…`, and the prior `12415faa…` with `amendments=[]` and the extension removed. `validate`: 0 findings. Amendment sha `6383bbfc…` and request-session-3 sha `9302e585…` match the `amendments[]` entry. |
| Pre-data timing | pass. Original freeze 04:25:05Z precedes s2's start (04:51:55Z). PR #3531 merged at 05:32:59Z, before s3 ran (06:23:19–06:23:57Z). |
| Session records `records.jsonl` (sha256) | s1 `bc60833b…`, s2 `ab4be90e…`, s3 `9aa25936…`, counted `1a7f3faa…`. Each has 576 records, a `complete` session and 576/576 verified. |

## 2. Replay (pinned binary, validator's own copies and output dir)

`ecbench verify --dir <copy>/session --replay-all --exit-code --out validation/replay/<s>-replay-all.json`

| Session | exit | ok | verified | replays reproduced | receipt sha256 |
|---|---|---|---|---|---|
| s1 | 0 | true | 576/576 | 512/512 identical | `0da9886c…` |
| s2 | 0 | true | 576/576 | 512/512 identical | `7a9f3f52…` |
| s3 | 0 | true | 576/576 | 512/512 identical | `3529c263…` |
| counted | 0 | true | 576/576 | 512/512 identical | `8ae131c4…` |

The 64 warm-ups per session are not replayed; `--replay-all` covers measured runs. The
untrusted input directories were hashed before and after and are unchanged.

## 3. Independent recomputation (`scripts/recompute.py`, Python, no shared code)

Definitions, derived and then confirmed by agreement to machine precision:

- Total wall ratio R = Σ candidate `time.solve_wall_ns` / Σ incumbent, over 128 measured
  pairs keyed by (workload, round).
- CI: delta-method SE of log R, se = sd(b_i/mean b − a_i/mean a)/√n, with
  CI = R·exp(±t₀.₉₇₅,₁₂₇·se). The interval is symmetric on the log scale. The symmetric
  ratio-scale form misses the reported endpoints by about 4e-6, so the comparison
  discriminates between the two forms.
- A/A = Σ `incumbent-aa` / Σ `incumbent`, same CI.
- Corrected = R/AA, with variances added on the log scale and a Welch–Satterthwaite dof.
- Ops: ratio of Σ S, with S = (total_gae + w·unpriced)/√r. "Unpriced" is the
  `*_uncharged` operations: IC phase natives, rho `counters`. The CI is a workload-cluster
  linearised t with t₁₅.
- t quantiles: my own incomplete-beta implementation (t₁₂₇ = 1.97882, t₁₅ = 2.13145).

Tolerance stated in advance: 1e-9 relative. Observed maximum over all 51 headline values:
**4.7e-16**. Over every compared field, including the 4 control-comparison and band
fields: 6.3e-16.

| Session | cand/inc total wall [95% CI] | A/A inc-aa/inc [95% CI] | corrected [95% CI] (dof) | pc-fb2x/inc wall [95% CI] | pc-fb2x ops S |
|---|---|---|---|---|---|
| s1 (H1) | 1.121440 [1.118285, 1.124603] | 0.998393 [0.996871, **0.999918**] | 1.123244 [1.119663, 1.126837] (195.6) | 1.381905 [1.342757, 1.422193] | 2.396098 |
| s2 (H1) | 1.121552 [1.118256, 1.124858] | 0.999641 [0.997293, 1.001995] | 1.121954 [1.117755, 1.126170] (242.2) | 1.380374 [1.341381, 1.420500] | 2.396098 |
| s3 (H2) | 1.122522 [1.119261, 1.125792] | 0.998931 [0.997223, 1.000642] | 1.123722 [1.119950, 1.127507] (205.5) | 1.392984 [1.353891, 1.433206] | 2.396098 |

The control-comparison.json A/A (`pc-fb2x` vs `incumbent-aa`, `aa_selection: first
alphabetical`) uses the same `incumbent`/`incumbent-aa` pair, so its `aa_ci95` is identical
to the column above in each session. I recomputed it and it agrees. The job's
`control-comparison.json` is byte-identical to the copy the analysis read.

Counted companion (`ic-line-v6-counted-20261106`, exit 42):

| w (GAE per unpriced op) | candidate/incumbent S | IC/rho S [95% CI] | pooled mean S, IC / rho |
|---|---|---|---|
| 0 | 1 exactly, CI [1, 1] | 22.998 [20.153, 26.244] | 20.771 / 0.9032 |
| 0.1 | 1 exactly, CI [1, 1] | 22.869 [20.011, 26.136] | 22.661 / 0.9909 |
| 1 | 1 exactly, CI [1, 1] | 22.281 [19.346, 25.662] | 39.668 / 1.7803 |

Per-curve band, recomputed exactly, including `native_sum` and `charged_sum`:

- tm10425: IC 20.970 / 22.751 / 38.778 against rho 0.960 / 1.053 / 1.893.
- t15757: IC 20.572 / 22.571 / 40.557 against rho 0.846 / 0.928 / 1.668.

The order rho < IC holds at every weight on both curves (AGENTS.md §5a). Incumbent vs
candidate is "unresolved", correctly, because they are equal. Raw (unnormalised) cost
ratios are 23.08 / 22.96 / 22.41, shown for reference; ecbench reports the S ratio.

## 4. Counted parity

- **Ops level:** in every wall session, all 128 measured pairs have identical `total_gae`
  and the same `algorithm_seed`, with 0 unpaired runs. The S ratio is exactly 1.0 and the
  CI is [1, 1].
- **Record level:** this uses the frozen projection (recovered log, `total_gae`,
  `cost.unpriced`, per-phase adds/doubles/scalar_mults/gae/native/work_native, with
  `ALLOWED_NATIVE = []`). 0 of 144 cells mismatch in each of s1, s2, s3, and 0 of 144 in
  the counted session.
- **Beyond the projection:** outside timing and identity, the only paths that ever differ
  between the twins are `detail.label`, `fbm_id` and `phases[].wall_ns`. Counters,
  `work.*` bitops and outcomes are identical.
- **Cross-session:** the counted IC records equal each wall session's on the projection,
  288/288 cells for each of s1, s2, s3.

**Counted parity holds.**

## 5. Gates and verdicts re-derived from the frozen rule text (`scripts/derive_verdict.py`)

Inputs: my recompute, my replay receipts and each `job-exit.json`. Sources: PROTOCOL-V6 §6,
`decision_rule.text`, Amendment §2.3/§2.4/§3. Mapping per session: a failed
replay/parity/admission/A-A/positive gate gives `invalid_measurement`; otherwise below-1 and
outside noise gives `supported_within_scope`; not outside noise gives
`within_control_distribution`; anything else gives `inconclusive`.

| Gate | s1 | s2 | s3 |
|---|---|---|---|
| replay and audit (576 records) | pass | pass | pass |
| counted parity, ops | pass | pass | pass |
| counted parity, records | pass | pass | pass |
| admission (128 pairs, 0 below L2, job exit 0) | pass | pass | pass |
| A/A both intervals contain 1 within [0.9, 1.1] | **FAIL** (upper 0.999918 < 1) | pass | pass |
| positive control (ops > 1, wall > 1, CI low > 1) | pass | pass | pass |
| total and corrected CI below 1 | false | false | false |
| outside noise | true | true | true |
| **verdict** | **invalid_measurement** | **inconclusive** | **inconclusive** |

- My gates equal the producer's `gates` objects exactly for all three sessions.
- `outside_noise` was evaluated under two readings: the candidate CI is disjoint from the
  A/A CI, and the corrected CI excludes 1. Both are true, so no label depends on which
  reading ecbench uses.
- **Host epoch (Amendment §3 command, run by me):** exit 0. H1 is s1 and s2 on
  `ip-10-84-67-216`; H2 is s3 on `ip-10-84-75-83`, with distinct pod UIDs and s3 exit 0.
- **s1 determinism:** `{verify_exit, verdict, gates}` for `dir` =
  `…/s1-untrusted/session` is byte-equal in run (i) and run (ii). The s1 candidate, positive
  and control-comparison files are also byte-identical across the two runs.
- **Combined (Amendment §3):** (invalid_measurement, inconclusive, inconclusive) gives
  **`invalid_measurement`**. The host-epoch rule could only lower support and does not
  apply. **The reported verdicts follow.**
- **Counted acceptance (§7):**
  - audit OK, every execution verified, 512/512 replays identical;
  - `invalid_outcomes` [], spec and build equal to the freeze;
  - exit 42 is accepted. The 97 below-L2 executions are all rho arms at L1 (49 rho-neg,
    48 rho-neg-aa), and every IC execution is L2.

## 6. Controls and the null object

- **The null object is a same-shape twin.** `incumbent-aa` is the same method, seeds and
  workloads as `incumbent`, measured in the same session. Under `alternate_measured` it
  sits at block positions {1, 2}, exactly like the candidate. The incumbent and `pc-fb2x`
  sit at {0, 3}, 64 pairs each. That is the frozen layout, tabulated in `recompute.json`.
- **The A/A therefore measures the positional bias the candidate inherits.** All three A/A
  ratios are slightly below 1 (0.9984, 0.9996, 0.9989). The corrected ratio removes this
  bias and changes the effect only in the fourth digit. Against that null, the candidate's
  +12% is 52 to 80 A/A half-widths away from 1 (s2 to s1).
- **The effect vanishes when its cause is removed.** Without `layout=lines` (the A/A), the
  ratio returns to 1 within ±0.2%.
- **Positive control.** `pc-fb2x` is detected in every session (wall ~1.38, CI low > 1.34;
  ops S 2.396).

## 7. Heuristic-validation, ladder and cost-model checks

- **Heuristic validation:** not applicable. This is not a heuristic-validation experiment.
- **Inventor-protocol §6 ladder:** not applicable. No speedup is claimed, and the measured
  direction is a slowdown.
- **Cost unit:** declared. `ecbench.gae` is a lower bound; S = GAE/√r. Unpriced native
  operations are priced at w = 0, 0.1 and 1, and ranks are given only where the order holds.
- **Memory alongside time:**

  | Arm | median max RSS | median solve |
  |---|---|---|
  | incumbent | ~49.9 MB | 28.9 ms |
  | candidate | ~49.3 MB | 32.3 ms |
  | pc-fb2x | ~63 MB | 40 ms |

  The candidate does not trade memory for time.
- **Phase diagnostic (descriptive only):** the candidate's extra wall time is about 44% in
  `oracle_setup` (+25% phase wall) and about 55% in `relations` (+9%). The per-curve ratio
  is 1.120–1.126.

## 8. Limitations and caveats

1. **Not yet a committed snapshot.** The frozen contract is committed, but the four
   sessions' raw `records.jsonl` and the analysis outputs are not. They are untracked in
   the shared working tree, and `records.jsonl` is not in the repository at all. I anchored
   the receipts to the S3 results URIs, which the frozen `analysis_plan.inputs` name. The
   Coordinator's archive must commit the records at the sha256 values above. Until then the
   draft `VALIDATION.json` is refused by `validation_gate.py evidence`. I tested it: it
   returns "valid" with the four files staged at their pinned paths and hashes, and fails
   without them.
2. **The replay uses the producer's binary.** The protocol requires the pinned binary, so
   the replay certifies determinism and record binding, not an independent implementation.
   The metric recomputation is independent.
3. **Two ecbench labels are not re-implemented:** `wall.status: admitted` and
   `outside_noise`. Their inputs (128 pairs, all L2) and two readings of `outside_noise`
   were checked; no verdict depends on them.
4. **Executor-attested items, not redone:**
   - image→binary extraction;
   - the s3 pre-launch EC2 node check (0 and 0 at 06:20:27Z);
   - the budget gate line (USD 1.14 ≤ 5.00);
   - the qualification's minimum solve ≥ 5 ms condition. Its `records.jsonl` is not
     committed; the other 4 of 5 qualification conditions re-check from committed files.
5. **Output paths.** The analysis was written to scratch `analysis/run-i|run-ii` instead of
   `reports/v6/analysis-a|b`. The later `reports/v6/analysis-a|b` copies are byte-identical
   (26/26 non-empty files; the empty `.stderr` files are omitted). This is cosmetic.
6. **The s1 A/A failure is narrow (8.2e-5) and is not evidence about the candidate.** The
   frozen rule has no tolerance, so the label stands. The other two sessions are valid and
   agree with s1 on the effect.
7. **The counted job's CodeBuild launcher reports `FAILED`** (`run/gate-line-s3.txt`). Its
   `job-exit` is 42, which the frozen counted rule accepts.
8. **Concurrent edits to the shared worktree.** During this validation, an uncommitted
   `DECISION-V6-REDESIGN-20261106.md` §16 ("Outcome of v6 … provisional") and a producer
   report `reports/v6/V6-RESULTS.md` appeared. I read only §16's headings and did not read
   either body. My §14–15.5 reading matches the committed blob.
9. **Scope.** The result covers fp28 only, two public prime-order curves, c8a.2xlarge,
   executable `ea8e9ef7` / `73362eed`, and two host epochs (instances, not necessarily
   physical machines).
10. **ID allocation.** `tools/allocate_id.py` has no `VAL` type, so the report ID uses a
    random 6-hex suffix.

## 9. `validation_report`

```yaml
validation_report:
  id: VAL-20261106-9ce3e6
  task_id: null   # task card supplied inline by the caller; no TASK-* id given
  run_ids: [ic-line-v6-s1-20261106, ic-line-v6-s2-20261106, ic-line-v6-s3-20261106, ic-line-v6-counted-20261106]
  artifact_checks:
    - {check: sha256sums_all_sessions, result: pass}
    - {check: s3_anchor_sha256sums_and_job_exit, result: pass}
    - {check: binary_73362eed, result: pass}
    - {check: image_extraction, result: caveat_executor_attested}
    - {check: spec_and_build_equal_freeze, result: pass}
    - {check: freeze_hash_reproduces, result: pass}
    - {check: committed_snapshot, result: caveat_raw_data_not_committed}
  metric_recomputations:
    - {metric: total_wall_ratio_and_ci, sessions: [s1, s2, s3], max_rel_diff: 0.0, result: pass}
    - {metric: aa_ratio_and_ci, max_rel_diff: 2.2e-16, result: pass}
    - {metric: corrected_ratio_ci_dof, max_rel_diff: 3.0e-16, result: pass}
    - {metric: counted_ops_ratio_w0_0.1_1, value: 1.0, result: pass}
    - {metric: ic_over_rho_S_w0_0.1_1, values: [22.998, 22.869, 22.281], max_rel_diff: 6.3e-16, result: pass}
  control_checks:
    - {control: aa_null_twin, s1: fail, s2: pass, s3: pass}
    - {control: pc_fb2x_positive, result: pass_all_sessions}
    - {control: counted_parity_ops_and_records, result: pass}
    - {control: host_epoch_H1_H2, result: pass}
    - {control: s1_determinism, result: pass}
  heuristic_validation_checks: [not_applicable]
  cost_model_checks:
    - {check: unit_declared, result: pass, unit: "ecbench.gae (lower bound), S = GAE/sqrt(r)"}
    - {check: gae_band_0_0.1_1_with_rho_priced, result: pass, order_holds_across_band: true}
    - {check: memory_alongside_time, result: pass}
  proof_architecture_checks: [not_applicable]
  per_session_verdicts: {s1: invalid_measurement, s2: inconclusive, s3: inconclusive}
  combined_verdict_rederived: invalid_measurement
  reported_outcome_confirmed: true
  verdict: passed
  limitations: [see section 8]
  artifact_paths: [validation.json, recompute.json, verdict-rederived.json, descriptive.json, VALIDATION.json, gate-draft/, replay/, s3-anchor/, scripts/]
review_attestation:
  joints_owned: [evidence admissibility, replay, metric recompute, counted parity, gate and verdict derivation]
  sources_read:
    - repo e87fa18a: PROTOCOL-V6.md, PROTOCOL-V6-AMENDMENT-1.md, protocol-v6.json (freeze, amendments, decision_rule, analysis_plan, stopping_rules), analyze-v6.sh, DECISION-V6-REDESIGN-20261106.md sections 14-15.5, spec-fp28-v6*.json, launch-v6/request-*.json, eks-results-v6/qualification/*, docs/VALIDATION_GATE.md (origin/main), scripts/fixtures/validation_gate/positive/*
    - untrusted: s1/s2/s3/counted-untrusted (all files by hash; records.jsonl via own scripts), analysis/run-i and run-ii (compare outputs, verdict.json), run/analysis-step-*.out, run/analysis-s1-det-*.txt, run/analysis-host-epoch.out, run/dl_session.sh, run/analysis-env.sh, run/node-gone-check.txt, run/gate-line-s3.txt, stubbin/aws (head), synth/ (listing and hashes)
    - S3 (read-only): SHA256SUMS and job-exit.json of the four jobs, v6 job-prefix listing
  read_sibling_reports: false   # saw only the headings of the uncommitted DECISION section 16; did not read it or reports/v6/V6-RESULTS.md
  verdict: holds   # on the owned joints
```
