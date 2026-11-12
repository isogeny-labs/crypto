# `fes-f2` vs `inherited-f4` at degree 13: the validated Iso Lab run (prepared, not submitted)

Status, 2026-11-12: **frozen, not yet run.** The main session froze the
protocol with fresh seeds (`/a1/` tags; the smoke's pairs are not in the
measured set) and the reference rho and band rule recorded in
`protocol.json` (`amendments_before_freeze`). Prepared by a subagent; nothing
was submitted by it. Submission goes through the director/executor cards.

## What is measured

The local, unisolated Run 2 of
[`../inherited_f4_next_20261112/`](../inherited_f4_next_20261112/README.md)
found `[pcc:unverified]`: on `icv1-f2m13-t181-515ee569` (K_0 / F_(2^13),
r = 2003), `binary-subspace[dimension=7] + descent-algebraic[m=2] + <solver> +
walk` has S = 94.7 (83.7 to 105.7) with `fes-f2` and 123.1 (117.8 to 128.4)
with `inherited-f4`; pair table 197.7; negation-map rho 2.76.

[`spec.json`](spec.json) measures exactly those arms in one isolated L2
ecbench session (one target per workload, 8 public targets, 1 warm-up and
6 measured rounds, arm order `alternate_measured`, seeds derived from SHA-256
of the study path and the attempt tag `a1`):

| arm | role | method |
|:--|:--|:--|
| `rho-sfstrong-v2` | reference | `rho.signed_frobenius_strong_v2` (A = 2n, the strongest Koblitz rho) |
| `rho-negation` | baseline | `rho.negation` (A = 2, the local "matched rho" kind) |
| `ic-pairtable-mitm2` | baseline | `ic.pipeline` binary-subspace:dimension=7, mitm:m=2, walk |
| `ic-descent-inherited-f4` | baseline | `ic.pipeline` binary-subspace:dimension=7, descent-algebraic:m=2, solver inherited-f4, walk |
| `ic-descent-fes-f2` | candidate | the same with solver fes-f2 |
| `rho-sfstrong-v2-aa` | control | A/A of the reference (run-ecbench compares the first reference with the first control) |

`max_trials` is 2,000,000 as in the local sweep. `solver_budget_seconds` is 0
(the local sweep used 60 s and hit it 0 times): a nonzero budget makes the
records nondeterministic and the replay audit would not reproduce them.

Hypotheses, expectation and falsifier: [`protocol.json`](protocol.json)
(H1: fes-f2 S below inherited-f4 S, paired 95 % interval below 1, order holding
at 0, 0.1 and 1 GAE; expected ratio about 0.77).

## Local smoke (not the measurement) `[pcc:unverified]`

2026-11-12 16:35 Z, this container (shared, 4 vCPU, other workers' builds
running), binary `8f9576b5…e5130f4` built from `git archive e9d5dbdccd`
([`smoke/build.json`](smoke/build.json)). Reduced spec
[`smoke/spec.json`](smoke/spec.json): 2 targets, 2 measured rounds, 1 warm-up,
otherwise identical.

```
ecbench run --spec smoke/spec.json --out <scratch>/smoke-session --allow-busy --lock <scratch>/bench.lock
  -> session ECBS1hf36aaddb1ae3 complete (36 records: {"verified": 36}), 3 s wall
ecbench verify --dir <scratch>/smoke-session --replay-all --exit-code
  -> audit OK: 36 records, 36 verified, 24 replays reproduced, 0 problems
```

Every arm runs and verifies; every descent record has `[pcc:unverified]`
`solver.priced_by = pinned` and an empty `cost.unpriced`; the plan resolves the
workload to `icv1-f2m13-t181-515ee569` (r = 2^10.97, A = 26). Grades are L1
(this host is not isolated), so no wall figure here means anything. Mean S
over 4 measured runs: rho-sfstrong-v2 2.48, rho-negation 2.70, pair table
197.1, inherited-f4 117.6, fes-f2 75.4
([`smoke/table.md`](smoke/table.md)). Paired fes/inherited GAE ratio per run:
0.75, 0.43, 0.73, 0.70.

Three things the smoke shows that the main session must know before freezing:

1. **The band changes what the descent rows mean.** `ecbench band` re-prices
   the pinned solver word XORs at w GAE each, like every other native counter
   ([`smoke/band.md`](smoke/band.md)): fes-f2 S is 14.1 / 2,665 / 26,520 and
   inherited-f4 14.6 / 4,474 / 44,612 at w = 0 / 0.1 / 1. At w = 0 the solver
   costs nothing and the two arms differ only through the group work of the
   targets each tried (fes/inherited 0.97 in the smoke); at w = 0.1 and 1 the
   solver dominates (0.60, 0.59). The order held at all three weights in the
   smoke, but the w = 0 margin is about 3 % and rests on trial counts, not on
   solver cost. H1 as pre-stated requires the order at w = 0 too; it could
   come out "unresolved inside the band" on that point alone.
2. **S is a deterministic count.** Counts depend on code, workload and seed,
   not on the host: the isolated run will reproduce, bit for bit, the S a local
   run of the same spec would give. The isolated run adds the L2 grade, the
   admitted A/A wall figures, image/binary provenance and the gate artifacts;
   it does not add sampling noise to S. The smoke used the full spec's
   `target_seed` and `measurement.seed`, so its 2 targets and rounds 1 to 2 are
   the first 2 of 8 workloads and 2 of 6 rounds of the full spec: 4 of the 48
   measured pairs per arm are already seen (disclosed here before the freeze).
   The full spec was not run locally.
3. With 4 pairs the paired ops interval is 0.64 (0.14 to 2.86): the per-target
   spread is wide (which targets fail to decompose changes the trial count),
   so 48 pairs are needed for an interval clear of 1; at the per-run ratios
   seen (0.43 to 0.75) that is expected but not guaranteed.

## Source and image

- Source revision: `e9d5dbdccdeb55b6d8527ebae1113135b42b26bf` (on `main`;
  12 commits behind remote `main` 6010b681, whose only `src/` change since is
  `src/cryptanalysis/koblitz_index_calculus.rs`). It contains 43171a66c1
  (solver calibration keys) and ca15f345e1 (`inherited-f4-next`), the code
  of the local Run 2. It does not contain branch commit 4840612771
  (inherited-f4 folds in the next kernels; fes-then-inherited size rule).
- No image for that revision exists in ECR `isolab/tasks`. The newest
  commit image is `gh-42948358222-1-8dda1b2d…` (source 8dda1b2d1d04, pushed
  2026-11-11 03:08 Z, digest
  `sha256:296c699530748395726136f394b0dbe349bb292c392dd56e2fd04ff23cc52640`); 25 relevant source files (ic_framework solvers and
  solver_units, ecbench methods and band, inherited_f4, koblitz_groebner,
  ic_boundary) changed between it and e9d5dbdccd, so it does not run the code
  of the local finding. The default image (`image.json`, source cb2ec6873f30,
  2026-10-19) is older still.
- A commit image must be built: dispatch `isolab-experiment.yml` on `main`
  with `source_revision=e9d5dbdccdeb55b6d8527ebae1113135b42b26bf` and
  `publish_only=true` (see "Main-session steps").

## Main-session steps (in order)

1. Commit and push this directory on a focused branch/PR; run
   `taskq/deploy/eks-karpenter/check-experiment-git --repository
   isogeny-labs/crypto --path research/fes_k13_validated_20261112/spec.json
   --path research/fes_k13_validated_20261112/protocol.json --path
   research/fes_k13_validated_20261112/request.template.json` and keep the
   receipt. Fill `.resume/RESUME.md` (`scripts/checkpoint.sh init`) and
   `tools/benchrec.py init` the bench record.
2. Publish the commit image without running anything:

   ```bash
   gh workflow run isolab-experiment.yml --repo isogeny-labs/crypto --ref main \
     -f source_revision=e9d5dbdccdeb55b6d8527ebae1113135b42b26bf \
     -f publish_only=true
   ```

   Record the run ID; download artifact
   `isolab-publication-<run_id>-<attempt>` (`isolab-image.json`,
   `build-info.json`). Check `build-info.json` `.source_revision` equals the
   SHA above.
3. Fill the request from the receipt and validate it:

   ```bash
   jq --arg img "$(jq -r .image_uri isolab-image.json)" \
      --arg bin "$(jq -r .ecbench_sha256 build-info.json)" \
      --arg wf "<main SHA the publish-only run used>" \
      --arg url "https://github.com/isogeny-labs/crypto/actions/runs/<run_id>" \
      '.job.build.image_uri=$img | .job.build.ecbench_sha256=$bin
       | .provenance.workflow_revision=$wf | .provenance.run_url=$url' \
      research/fes_k13_validated_20261112/request.template.json \
      > research/fes_k13_validated_20261112/request.json
   taskq/deploy/eks-karpenter/remote/validate-request research/fes_k13_validated_20261112/request.json
   ```

   (Field names in `isolab-image.json` per `publish-image`; check them.)
   Commit `request.json` before launch.
4. Director card (AWS bridge): pool check, then validate and launch, with
   `AWS_REGION=us-east-2`, `ISOLAB_BUCKET=isolab-results-104186811093-us-east-2`,
   `ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher`, a fresh
   `ISOLAB_CLIENT_OUTPUT` receipt directory (`must_not_exist`), client timeout
   at least 1800 + 3600 + 600 s (detached step).

   ```bash
   scripts/isolab_pool_check.sh --cpu 4 --memory 4Gi --pool cpu-l2-spot --save <receipt>/pool.json
   taskq/deploy/eks-karpenter/remote/validate-request research/fes_k13_validated_20261112/request.json
   taskq/deploy/eks-karpenter/remote/launch-remote research/fes_k13_validated_20261112/request.json
   ```

   This host has no `kubectl` or kube config: the pool check cannot run here;
   the executor host needs cluster read access or the card says it cannot claim
   the pool is free.

## After the run: replay certificate, validator report, `VALIDATION.json`

Inputs: the launcher's `result.json` and the S3 prefix it names
(`runs/<job>/…`: `session/`, `audit.json`, `l2-gate.json`,
`control-comparison.json`, `build-info.json`, `binary.sha256`,
`input-spec.json`, `SHA256SUMS`).

1. Download and check the receipt (director):

   ```bash
   aws s3 cp --recursive s3://isolab-results-104186811093-us-east-2/runs/fes-k13-validated-20261112-a1/ R/
   (cd R && sha256sum -c SHA256SUMS)
   jq -e '.state=="succeeded"' <receipt>/result.json
   jq -e '.ok and .regraded and .records_bound and all(.replays[]; .reproduced)' R/audit.json
   jq -e '.accepted' R/l2-gate.json
   jq -e '.wall.status=="admitted" and .wall.aa_pairs>=5' R/control-comparison.json
   cmp <(jq -S . R/input-spec.json) <(jq -S .job.spec research/fes_k13_validated_20261112/request.json)
   ```

   Also: receipt Job/Pod UIDs equal the pod's `job-exit.json`; `binary.sha256`
   equals `request.job.build.ecbench_sha256`; `build-info.json`
   `.source_revision` equals e9d5dbdccd. Copy the session to
   `research/fes_k13_validated_20261112/session/` (raw tree as one archive per
   AGENTS.md §13a, through the store; `records.jsonl` and `session.json` in Git).

2. **Replay certificate (independent).** The in-pod `verify --replay-all` is
   the same host as the measurement and is not the independent replay. Replay
   on a different host class with a binary of the session's commit:

   ```bash
   # the exact pod binary: the GitHub build artifact isolab-build-<run_id>-<attempt>
   gh run download <run_id> --repo isogeny-labs/crypto -n isolab-build-<run_id>-<attempt> -D BIN/
   sha256sum BIN/ecbench    # must equal build.ecbench_sha256
   BIN/ecbench verify --dir research/fes_k13_validated_20261112/session --replay-all --exit-code \
     --out research/fes_k13_validated_20261112/validation/replay-receipt.json
   BIN/ecbench host --json > research/fes_k13_validated_20261112/validation/replay-host.json
   ```

   Require every replay `identical`/reproduced and an `env_class_id`
   different from the session's `host.json`. Then write `[pcc:unverified]`
   `validation/replay-certificate.json` (`replay-certificate/v1`, as in
   `research/ecbench_generator_orbit_20261022/validation/replay-certificate.json`):
   `status: verified`, `independent: true`, `scalars_verified: true` (each
   record's answer re-checked `[d]G = Q` by validator-written code, not
   repository code), non-empty `runs`, the receipt SHA-256, both host classes
   and the binary SHA-256. Produced by an agent other than the measuring one.
3. **Validator report.** Independent recompute from `session/records.jsonl`
   with validator-written jq (not `ecbench table`/`band` output except as the
   claimed side): mean S per arm, the paired ratio fes/inherited, S/rho for
   both IC descent arms against both rho arms, and the band at w = 0, 0.1, 1
   (U = sum over phases of `cost.unpriced` counts plus `work_native`
   `table_insert_attempts` where a phase carries `counted:table_insert_attempts`,
   per `src/cryptanalysis/ecbench/band.rs`). Claimed side:

   ```bash
   ecbench table   --dir research/fes_k13_validated_20261112/session
   ecbench compare --dir research/fes_k13_validated_20261112/session --a ic-descent-inherited-f4 --b ic-descent-fes-f2 --save
   ecbench compare --dir research/fes_k13_validated_20261112/session --a rho-sfstrong-v2 --b ic-descent-fes-f2 --save
   ecbench compare --dir research/fes_k13_validated_20261112/session --a rho-negation --b ic-descent-fes-f2 --save
   ecbench band    --dir research/fes_k13_validated_20261112/session --weights 0,0.1,1 --ranks \
     --json research/fes_k13_validated_20261112/band.json
   ```

   `validation/validator-report.json`: `status: pass`, `independent: true`,
   `raw_data` = [`session/records.jsonl` with SHA-256], `headline` =
   `{name, claimed, recomputed}` for each number above (default rel_tol 1e-9).
   Also check H2: `priced_by == "pinned"` on every descent record and no
   `solver_*_uncharged` in any `cost.unpriced`.
4. **Matched rho band check.** The protocol names `rho-sfstrong-v2`
   (`kind: frobenius`, strongest available on a Koblitz curve; same session,
   same binary, unit `ecbench.gae`, S = GAE / sqrt r). `gae_band` gives mean S
   of the IC row (`ic-descent-fes-f2`) and of the rho at w = 0, 0.1, 1;
   `order_holds_across_band` is true only if the IC-vs-rho order is the same
   at all three weights (expected: rho ahead at every weight; the gap is the
   distance remaining). H1 (fes vs inherited) is ranked only where
   `ecbench band --ranks` says the order holds at all three weights.
5. `validation/VALIDATION.json` (schema `validation-gate/v1`, paths relative
   to the evidence directory), then:

   ```bash
   python3 tools/validate_validation_json.py research/fes_k13_validated_20261112/VALIDATION.json
   python3 scripts/validation_gate.py evidence research/fes_k13_validated_20261112
   python3 tools/pcc/pcc.py canary
   ```

   until `valid`; record the claim with `pcc`; then the leaderboard,
   `benchmarks/<date>-…/`, bench record and progress report per AGENTS.md.

## Cost estimate and ceiling

Spot price read 2026-11-12 (us-east-2, c8a.2xlarge Linux): 0.094 to
0.146 USD/h; On-Demand 0.431 USD/h. The run is short (see the smoke):
provisioning 2 to 5 min, the harness's fixed 60 s sleep, the session and the
replay-all. Deadline 1800 s, so the worst-case EKS node time is about 0.5 h
(plus Karpenter's empty-node consolidation, 2 min): at most 0.08 USD Spot,
0.22 USD at On-Demand rates. CodeBuild launcher (small Linux) waits up to
the deadline plus one hour: at most 90 build-minutes, about 0.45 USD at
0.005 USD/min. The publish-only GitHub workflow build runs on GitHub-hosted
or the configured runner (no AWS compute) plus about 150 MB of ECR storage
(under 0.02 USD/month). S3 results are a few MB.

**Recorded ceiling: 5 USD for this experiment, all attempts included**
(builds, failed attempts, a Spot loss and one re-submission), against the
200 USD/day IsoLab allowance and the 1,000 USD monthly budget
(COST_CONTROLS.md). Concurrency 1; 4 vCPU / 4 GiB on `cpu-l2-spot`; deadline
1800 s; retry allowance: 0 automatic, at most one labelled manual
re-submission after a reconciled Spot loss. The current daily and monthly
spend was not read here (no billing call); the main session must read the
budget remainder before launch and stop if it is unknown.
