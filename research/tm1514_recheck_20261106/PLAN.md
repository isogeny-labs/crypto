# tm1514 recheck: dispatch plan (draft, nothing launched)

> Superseded in part by PROTOCOL-AMENDMENT-1.md (counted-only, local L0, replicates A3/B3); where they disagree the amendment governs.

Study directory once committed: `research/tm1514_recheck_20261106/`.
Protocol: `PROTOCOL.json` (experiment.protocol/v1, status `draft`; validates with
`crypto-autoresearcher/tools/experiment_protocol.py validate`, 0 findings).
Specs: `spec-tm1514.json` (replicate A) and `spec-tm1514-B.json` (replicate B).
They are byte-identical except `label` and `measurement.seed`. `label` and
`description` are outside the spec id (`spec.rs` `identity_view`), so the
workload IDs match and `ecbench compare --b-dir` pairs A with B by
(workload, round).

Every number below that does not come from a committed file is marked as an
assumption.

## 1. What the specs contain

| field | value | why |
|:--|:--|:--|
| curves | the five `prime_explicit` curves of `multicurve/spec-k24.json`, byte-identical (checked by diff) | tm1514 is the subject; the four siblings are the comparison population |
| arms, in this order | `rho-neg` (reference), `rho-neg-aa` (control), `inc` (baseline), `inc-aa` (control), `rho-plain` (candidate) | `run-ecbench` admits wall A/A on the first reference against the first control, so `rho-neg-aa` goes first among the controls. The MCP/launcher check needs a control whose `method` equals a non-control arm's. Both twins satisfy it. |
| inc params | identical to spec-k24: `prime-projected-abscissa:size=2,scan_cap=32`, `wwalk:x=0.9,g=1,grow=2,seed=run`, `stride-generator` | same incumbent as before |
| targets | `target_kind: public`, `target_seed 1406057239440944`, `targets_per_curve 96` | new targets; the seed is derived as in section 2 |
| measurement | `L2`, `order alternate_measured`, `rounds 6`, `warmup 1`, `timeout_seconds 1800` | isolab requires L2, at least 5 rounds and at least 1 warm-up. With an even number of rounds and `alternate_measured`, each arm leads and trails equally often (no order warning). |
| seed | A `960559345608509`, B `4971825529176011` | B is the independent-randomness A/A, explained below |

Solves per arm per curve: 96 × 6 = 576 per replicate, so tm1514 gets 1152
solves per arm over A and B (the requirement was at least 256).

### Why there are two replicates

Within one session every arm of a (workload, round) gets the same algorithm
seed (`spec.rs:358-376`). The seed goes unchanged into `solve_measured`
(`record.rs:759-765`). `seed=run` swaps the run seed into the wwalk table
(`methods.rs:6213-6219`). So same-method twins in one session repeat their
partner exactly. That is a determinism check, and it is also the wall A/A that
isolab admission needs.

No arm parameter can salt the seed:

- `rho.negation` has only `cap_multiple` (`RHO_PARAMS`, `methods.rs:293-297`).
- For wwalk, `seed=<n>` fixes one table for every run. That is a different
  method, not an A/A.

The two ways to get independent draws are a second session with another
`measurement.seed`, or the different seed of each round within an arm. A
`seed_salt` method parameter would need an ecbench-extend change plus a new
method id. It is not proposed here.

### Correction to the brief

`p = 16777213`, the first curve of spec-k24.json, is
`icv1-fp24-tm1578-e83ed1b2` (trace −1578). The subject curve
`icv1-fp24-tm1514-dce9313c` is the fifth curve: `p = 16776317`, `a = 16524018`,
`b = 15225076`, `#E = 16777832`, `r = 2097229`. This was checked with
`ecbench plan` and the registry. The spec keeps all five curves, so nothing
changes in it.

### Local validation (development check, not a measurement)

The binary was built at origin/main b354dfef (rustc 1.97 locally; the
workflow uses 1.99.1). `ecbench plan` accepts both specs:

- A: `ECS1h298a06a2f06d`. B: `ECS1h864cbf25c1f2`.
- 16,800 executions each, 480 workloads, every curve `registered=true`, no
  order warning.
- The workload IDs are identical in A and B.
- The method IDs equal the k = 24 plan's: rho.negation `ECM1hefea4e0ebe79`,
  ic.pipeline `ECM1h3b0ad3332667`. rho.plain is `ECM1hb367476fce6c`.

The request envelope also passes `remote/validate-request`, `run-ecbench`'s
admission `jq` and the MCP A/A rule. Its first reference and first control are
`rho-neg` and `rho-neg-aa`.

One warm-up input per arm (seqs 1920, 1922 and 1924, tm1514 target 0) was run
through `ecbench profile-input` and `ecbench exec` to read the record shape.
The figures below are single L0 solves on a non-isolated host, for shape only:

- **rho.negation:** `cost.unpriced = [canonicalisations_uncharged]`. There
  were 2,727 canonicalisations for 2,899 GAE (2,419 steps). There is no
  per-phase wall time, only `solve_wall_ns`.
- **ic.pipeline (inc):**
  - `cost.unpriced` lists `sqrt_solves`, `lookups`, `row_ops`,
    `legendre_symbols` and three small `projected_base_*` counters, each with
    the `_uncharged` suffix.
  - Lookups dominate: 368 in the relations phase, one per stride trial.
  - The walk table's `table_insert_attempts` (1,530) sit in
    `oracle_setup.work_native`, which the band does not read.
  - Every phase has `wall_ns`.
- **rho.plain:** `unpriced = []`, 871 GAE.
- **Solve wall** (L0, one sample each, not evidence): rho-neg 0.43 ms, inc
  0.57 ms, rho-plain 0.19 ms. Per-solve fixed cost is visible: plain costs
  more ns per GAE than neg in this one pair. So the C3 estimator fits an
  intercept on rho-plain.
- **`time.hw_solve`:** `perf_event_open: No such file or directory`. This
  host has no PMU (`/sys/bus/event_source/devices` has no `cpu`).

## 2. Seeds (fixed before any data of this study)

Each seed is `int(sha256(D)[0:16 hex]) mod 2^53`, where D is
`crypto:research/tm1514_recheck_20261106:` followed by a suffix:

| suffix | value |
|:--|--:|
| `measurement.seed:A` | 960559345608509 |
| `measurement.seed:B` | 4971825529176011 |
| `workloads.target_seed` | 1406057239440944 |
| `analysis.bootstrap_seed` | 244596609222896 |

All values are below 2^53, so `jq`, which reads numbers as doubles in
`run-ecbench`, reads them exactly. None of them reuses 20261324 or 20261325.

## 3. Budget and time

Per replicate: 5 curves × 96 targets × (1 + 6) rounds × 5 arms = 16,800
executions.

Per-execution wall on the EKS L2 `c8a.2xlarge`, measured in committed sessions:

- `research/p20_large_n_20261029/eks-results`: 4,704 executions in 88.1 s,
  18.7 ms each. `verify --replay-all` took about 47 s, about 10 ms per record.
- `research/ic_packed_filter_20261029/P24/wall/eks-results`: 1,344 executions
  in 47.2 s, 35.1 ms each.
- The earlier k = 24 session on this family was a local L0 Firecracker VM, not
  EKS: 720 executions in 5.32 s, 7.4 ms each including the child process
  (`sessions/k24/session.json` timestamps). Per-arm solve times are only in
  its `records.jsonl`, which is in S3 (sha256 `02490440…e73b`) and not in Git.
  rho-neg costs about 1,815 GAE per solve and inc about 2,755 GAE (band-k24:
  charged_sum / 24). The solves are well under the per-execution harness
  overhead.

Estimate per replicate, budgeting 50 ms per execution:

| item | seconds |
|:--|--:|
| 16,800 executions × 50 ms | 840 |
| replay-all, 16,800 × 20 ms | 336 |
| `sleep 60` in `run-ecbench` | 60 |
| two `compare` calls | about 30 |
| total in the pod | about 1,270 (21 min) |

Spot provisioning adds about 3 to 5 minutes (assumption). The Kubernetes
deadline is `seconds = 3600` and includes provisioning, so the margin is about
2.5×.

If the per-execution cost turns out to be 150 ms (4× the worst observed), the
run would not fit. Splitting it into one job per curve would then be the
pre-registered fallback: five frozen specs, each with the same seed rule. It
is not needed at the observed rates.

Dollar ceiling, as assumptions to check against live prices before
submitting:

- Node: `c8a.2xlarge` Spot, at most 1 h each, at or below an assumed $0.50/h
  upper bound (the On-Demand order of magnitude; Spot is lower).
- CodeBuild launcher: at most 100 min × about $0.005/min ≈ $0.50.
- The GitHub-hosted build job is free. S3 is negligible.

That is about $1 per replicate. **Ceiling: USD 5 in total** (two replicates
plus one failed attempt). The daily allowance is USD 200 (COST_CONTROLS.md,
2026-11-03). Read the lane's charged and reserved spend before each dispatch,
and stop if the remainder is unknown.

L2 capacity is 8 vCPU, which is one `c8a.2xlarge`. Dispatch B only after A
reaches a terminal state, so the two never contend and B waits for nothing.

## 4. Before dispatch (all are rules; none is done yet)

1. Conductor task for `research/tm1514_recheck_20261106/**`: run
   `conductor check --summary ... --scope path:research/tm1514_recheck_20261106`.
2. Write `PROTOCOL.md`, the narrative the controlled-experiment skill
   requires.
3. Write the analysis in Rust (no Python, AGENTS.md), for example
   `examples/tm1514_recheck_analyze.rs` reading both sessions' `records.jsonl`.
   Put its path and SHA-256 in `PROTOCOL.json` `analysis_plan.script`.
   **Dry-run it** on the earlier k = 24 records (fetch `records.jsonl` from
   S3 by its SHA-256). That dry run measures the per-solve CV that the power
   note assumes (0.5), and checks that `phases[relations].wall_ns`,
   `phases[relations].native` and `time.hw_solve` are present in inc and rho
   records.
4. Commit the specs, `PROTOCOL.json`, `PROTOCOL.md`, `PLAN.md` and the analysis
   source to a focused branch, and push it. Then:
   - Run `python3 tools/experiment_protocol.py freeze PROTOCOL.json`
     (crypto-autoresearcher tool) so `freeze.protocol_sha256`, `commit` and
     `frozen_at` are set. Commit that and push it.
   - Run `taskq/deploy/eks-karpenter/check-experiment-git --repository aburan28/crypto --path <each file>`
     and keep the publication receipt.
5. Run `scripts/checkpoint.sh init`, fill in `.resume/RESUME.md`, and run
   `scripts/checkpoint.sh push` (the gate must pass).
6. Open a benchmark record with
   `python3 tools/benchrec.py init ...` for each replicate (bench.record/v1).
7. Confirm that the workflow revision on `main` has commit-image support. Then
   `SRC=<40-hex commit of step 4>`.

## 5. Dispatch inputs (one replicate at a time)

```bash
gh workflow run isolab-experiment.yml --repo aburan28/crypto --ref main \
  -f source_revision="$SRC" \
  -f spec_path=research/tm1514_recheck_20261106/spec-tm1514.json \
  -f instance_type=c8a.2xlarge -f seconds=3600 -f resumable=false
```

Replicate B runs the same command after A is terminal, with
`-f spec_path=research/tm1514_recheck_20261106/spec-tm1514-B.json`.

- Leave `publish_only` and `size_sweep` at their defaults (false).
- The job name is generated (`gh-RUN_ID-RUN_ATTEMPT`). Record the dispatch
  time and run ID. Before any second dispatch, reconcile an ambiguous one
  against the existing runs.
- Under the Claude model tiers, `experiment-director` writes the card,
  `executor-mechanical` runs it verbatim, and the director assesses the run.
  Record `tools/token_usage.py session --units 2 --unit-name card` after the
  relay.

## 6. Receipts to verify (per replicate)

Download every object under the receipt's `results_uri`, following
pagination. Then check:

1. `result.json`:
   - `state: succeeded` and `exit_code 0`;
   - `request_sha256` matches the dispatched request;
   - `job_uid` and `pod_uid` match the objects;
   - the `build` object (image digest, source revision = `$SRC`,
     `ecbench_sha256`) matches.
2. `build-info.json` and `binary.sha256` show the same source and binary
   hash.
3. `input-spec.json`'s SHA-256 equals the committed spec file at `$SRC`.
4. `SHA256SUMS`: every payload matches after download.
5. `job-exit.json` has `exit_code 0` and the same Pod UID.
6. `audit.json`: `ok`, `regraded` and `records_bound` are true, the record
   count is 16,800, and every replay has `reproduced: true` (replay-all).
7. `l2-gate.json`: `accepted` is true, `below_l2` and `invalid_outcomes` are
   empty, and censored outcomes are listed.
8. `control-comparison.json`: `wall.status admitted`, `pairs_below_level 0`,
   `aa_pairs >= 5`, and `aa_ci95` present.
9. `session/session.json`: `records_sha256` equals the SHA-256 of
   `records.jsonl`, `status complete`, and `executions_planned` equals
   `records_written`, which equals 16,800.
10. Twin determinism: every (workload, round) of `rho-neg` against `rho-neg-aa`,
    and of `inc` against `inc-aa`, has identical `cost.total_gae` and counters.

## 7. Analysis (frozen script, labels masked)

1. Run `ecbench band --dir A` and `--dir B` (per-arm `U` and its counters,
   mean S at 0 / 0.1 / 1).
2. Run `ecbench compare --dir A --a rho-neg --b inc --save`, the same for B,
   and the same per curve.
3. Run `ecbench compare --dir A --a inc --b-dir B --b inc`, and the same for
   rho-neg. This is the seed-independent A/A. It gives counts only; wall is
   refused across sessions.
4. Run `ecbench compare --dir A --a rho-neg --b rho-plain`. This is the
   positive control. Stop as invalid if it misses its expected detection.
5. Run `ecbench work --dir A`, the bitop/v1 model price of canonicalisations
   and lookups. This is a host-free cross-check.
6. Run the Rust analysis:
   - D(w) per weight with 98.33% intervals;
   - R_tm1514(0) against 1.52;
   - `w_meas_canon` and `w_meas_lookup` from L2 wall, and from `hw_solve`
     cycles if the node exposed a PMU;
   - `w_insert`;
   - R at the measured prices, with each arm at its own price and with both
     at the larger price.
7. Export with `experiment_protocol.py from-ecbench` for each session (plus
   `blind`), then run `check-export --verify-files`. Unmask after the
   intervals exist.

## 8. What lands in the same PR

- `research/tm1514_recheck_20261106/`:
  - specs, `PROTOCOL.json` and `PROTOCOL.md` (frozen before the data);
  - `PLAN.md`;
  - `launch/` (requests, cards, `result.json`, launch logs);
  - `eks-results/{A,B}/`, with the session directory whose `records.jsonl`
    sits in S3 behind `.artifacts.json`;
  - `audit.json`, the saved comparisons and the band files;
  - `analysis/` (source SHA, `result.json`);
  - `export.json`;
  - `reports/` (report, SVG graphs, PDF, per `research-visuals`);
  - `reviews/` (a `review-run` record by another runner, or its absence
    stated);
  - `README.md` with the table, the verdicts in the fixed vocabulary, and the
    scope sentence.
- `benchmarks/records/` with two `bench.record/v1` records, closed, raw data
  in S3 by SHA-256. Regenerate `benchmarks/progress/` (`tools/benchrec.py`).
- `benchmarks/20261106-tm1514-recheck/` (PDF, SVG, PNG) and a row in
  `benchmarks/README.md` **only if** `VALIDATION.json` names a replay
  certificate, a validator report and the matched rho reference with the 5a
  band. Otherwise the row renders "unvalidated (pre-gate)".
- A `pcc` claim for each verdict (`tools/pcc/pcc.py`), cited by claim ID and
  standing.
- The multicurve README gets a dated correction if C1b finds the earlier 1.52
  was a draw. The frozen evidence stays as it is.
- IC leaderboard: not touched. A re-measurement of the incumbent is not an
  improvement, and a relabelling or a pricing change cannot be one (success
  criterion).
- `docs/session-logs/2026-11-06.md` entry.

## 9. Rules not met yet, or gaps

- `PROTOCOL.md` (narrative) is not written. `PROTOCOL.json` is a validated
  draft, not frozen.
- The analysis script does not exist yet. The dry run on the earlier k = 24
  records was not done here, because the records are in S3 and this session
  has no AWS access. The CV of 0.5 in the power note is an assumption.
- No prime-curve native-unit microbenchmark exists. `ecbench price-units` takes
  Koblitz `a:n` only (`src/bin/ecbench.rs` `Cmd::PriceUnits`), and
  `ic_boundary::run_prime_instance` calibrates add, sqrt, Legendre, inversion,
  row op and pair-table lookup but not a negation canonicalisation. The
  wall-based estimators in C3 are indirect: per-step hashing and the DP test
  are uncounted on both rho arms and get absorbed into `t_add`. A direct
  measurement would be an `ecbench price-units --prime` extension
  (ecbench-extend). It could run on the L2 node only as an `isolab_k8s_run`
  command job, which needs the same-host MCP. The portable request and the
  GitHub workflow run specs only.
- Whether the c8a L2 node exposes a PMU is not established by anything in Git.
  `host.json` `perf_levels` is `[]` on both earlier hosts. Each record's
  `time.hw_solve` (or its `error`) answers it, and the run records it for
  free.
- Band asymmetry to report:
  - inc's walk-table inserts (`table_insert_attempts`, about T to 3T of them
    per solve) sit in `work_native`, which `band.rs` does not read, so the
    band never charges them.
  - rho-neg's per-step partition hash, DP test and DP table work are not
    counted at all. `rho.negation_v2` counts the table queries and inserts.
  - Both gaps are reported beside C3. Neither is repriced here, because that
    would be an accounting change.
- Session-log, Conductor and checkpoint steps belong to the main session.
  Nothing was written to the repository here.

## 10. Approval and retrieval (2026-11-06)

- The owner approved this run and the USD 5 ceiling on 2026-11-06 ("Yes" to the
  ceiling; "Go ahead and dispatch when ready").
- Protocol frozen at 2026-11-06T17:52:16Z on base 363306c646, protocol_sha256
  8cd5f856…9d645; analysis program `examples/tm1514_analyze.rs` sha256
  59b83670…3b6f2.
- The dispatching session has no AWS read access. The workflow verifies the S3
  receipts itself; the records reach Git only when someone with lab S3 access
  syncs `runs/<job>/` into `sessions/A` (and `B`) and pushes, or through the
  experiment queue once that route is used live.
