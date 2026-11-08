# Prime medium-tier rho reference bound (run complete 2026-11-08; verdict inconclusive)

> **Result (2026-11-08).** The run is complete. See [`RESULTS.md`](RESULTS.md).
> The `rho.negation` medium constant over the floor at w = 0 is 1.093 [1.033, 1.156].
> The PROTOCOL.md §9 verdict is `inconclusive`: the positive control P4 misses its
> threshold in every job. On M alone the verdict would be `setup_excess`.
> Every number is `[pcc:unverified]`.


Slug `prime_medium_tier_rho`. Lead `LEAD-RHO-matched-rho-calibration`
(priority 2 of the 2026-11-07 prime-field lead assessment). **Nothing here has
been measured on EKS.** The only executions were local development checks,
labelled as such below. Every number quoted from the repositories is
`[pcc:unverified]`. Evidence classes: *cited* (a committed file), *derived*
(arithmetic on cited numbers), *development check* (local, not a result),
*prediction* (written before data).

## Goal

Make the sealed prime-field medium-tier rho reference bound launch-ready. Every
prime-field IC gap is divided by this reference. The bound covers
`rho.negation`, `rho.plain` and `bsgs.negation` at log2 r 32-49, with at least
24 runs per size, replay certificates, the §5a band, and a preregistered
R(N) = 1 + c/N^beta fit. That fit decides whether the toy excess of
`rho.negation` (1.453 x floor) is set-up or a walk shortfall.

## Sources checked

- **research-kb**: not available here (`scripts/research-kb` has no executable), as
  PARALLEL_RULES says. I ran the substitutes in the crypto-autoresearcher
  checkout. Their `orchestration` package is missing from the sparse
  checkout, so I ran them through a scratch YAML shim
  (`/tmp/claude-0/-home-user/354dbe26-6a93-506a-a404-2fc6ef137fdc/scratchpad/prime_medium_atlas.py`):
  - `lead_atlas.py --match "prime medium-tier rho bound sealed ecbench rho.negation 32-48 bits"`
    returned: `LEAD-RHO-matched-rho-calibration` (21), `KR-RHO-cd0049` (17,
    toy prime negation-BSGS 1.160 x floor), `KR-RHO-1c2928`, `KR-RHO-825bca`,
    `LEAD-RHO-bsgs-family-memory` (closed_known).
  - `lead_atlas.py --match "rho finite-size bias toy set-up excess vs negation map fruitless cycle walk shortfall at medium sizes"`
    returned: `KR-RHO-fb88e6` (negation map, sqrt 2 and fruitless cycles),
    `LEAD-RHO-negation-map-fruitless-cycles` (closed_known),
    `LEAD-RHO-pool-class-quotient`, `LEAD-STR-automorphism-quotient-rho`.
  - `build_frontier_map.py --match` for both queries returned `KR-RHO-cd0049`,
    `KR-RHO-1c2928`, `KR-RHO-825bca`, `KR-RHO-fb88e6`, `KR-RHO-482ed3`.
    None of them is a sealed prime medium-tier rho bound.
- **ecdb**: unreachable. I read the token from Secrets Manager `ecdb/api` into the
  process only (never printed). `scripts/ecdb.sh get /v1/meta` returned
  HTTP 503 `database unavailable ... error connecting to server` three times
  (about 20:00-20:26 UTC). I fell back to repository files.
- **Sealed bounds** (`docs/bounds/records/`, cited):
  - The prime toy bounds `calibration-prime-{rho-neg,rho-plain,bsgs-neg}-v2.json`:
    `rho.negation` 1.453 [1.184, 1.772] x floor, alpha 0.398 [0.315, 0.475],
    six curves at log2 r 15.7-25.3, 24 runs each.
  - The Koblitz medium records. `calibration-koblitz-medium-*` covers log2 r
    27.8, 32.1, 39.0, and the pair-claw records add 36.6, 44.3, 44.5 and 47.2.
    The lead calls the Koblitz ladder "32-48"; in fact it spans 27.8-47.2.
  - The prime **oracle-domain** medium references `cheon-ecbench-medium-*-rho*`
    (`static_dh` domains, r +- 1 shaped curves, 8 runs per size): 1.11-1.19 x
    floor at w = 0, and about 2.2-2.4 at w = 1.
- **Committed sessions**: more than a hundred sessions with prime `rho.negation` at 33-50
  field bits exist, mostly special curves (volcano, conductor-gap, CM,
  shaped). I pooled them as a prior only (`sources/prior-pool.txt`):
  `rho.negation` 1.062 x floor over n = 18 735 runs, `rho.plain` about
  1.50, `bsgs.negation` 1.10-1.16. The pool is unsealed, in-sample and heterogeneous.
- **Read**: the skills `ecbench-bounds`, `ecbench-measure`, `controlled-experiment`,
  `experiment-protocol` (pointer) and `isolab-eks` (with its submission reference);
  `docs/bounds/README.md`, `docs/ecbench/README.md` §§1-5 and the Accounting v2
  table, `docs/benchmark-records.md`, `docs/VALIDATION_GATE.md`; AGENTS.md §5a,
  §11, §12, "Resume checkpoints" and "Benchmark records";
  `taskq/deploy/eks-karpenter/{HARNESS.md,COST_CONTROLS.md,run-ecbench,remote/validate-request}`;
  `research/ecbench_calibration_20261002/PROTOCOL.md` and specs (the toy
  ladder this mirrors); `research/cheon_ecbench_medium_20261106/fit.sh`.

## What was done

1. **Curves.** `prime_search` (the `find_prime_order_curve` source of the toy
   `icv1-fp*` ladder) refuses `bits > 32`, and the bench roster stops at 20
   bits. I therefore used the harness's own deterministic generator for
   larger fields, `prime_orbit_shape {shape: generic}`
   (`prime_orbit_index_calculus::generate_instance`: random p, random a and b,
   certified order, cofactor 1). The steps:
   - `specs/curve-generation.json` (plan only) uses field bits {33, 37, 41, 45, 49}
     with two curves per size. The seeds come from SHA-256 by the rule in
     `seeds.json`.
   - `ecbench plan` resolved it, and `ecbench profile-input` exported the
     explicit `prime_explicit` parameters into `curves.json`.
   - Every size is medium tier (field bits > 32; a 32-bit field is toy).
   - All ten curves have A = 2, and none is in `docs/curves/registry.json` yet.

   | slug | bits | # | p | a | b | r (= #E) | G | gen. seed | job |
   |:--|--:|--:|--:|--:|--:|--:|:--|--:|:--|
   | `icv1-fp33-t65189-eee04389` | 33 | 1 | 5557729739 | 2036002406 | 2773982840 | 5557664551 | (3, 898702012) | 201624291843241 | J1 |
   | `icv1-fp33-tm43469-ac81a6df` | 33 | 2 | 7183145327 | 4546663450 | 440691422 | 7183188797 | (3, 6515878363) | 24642267360510 | J1 |
   | `icv1-fp37-tm218953-61268c9f` | 37 | 1 | 70093240733 | 34866272438 | 47397029180 | 70093459687 | (9, 33541631670) | 18561872162765 | J1 |
   | `icv1-fp37-t160405-998ed0ab` | 37 | 2 | 92849827471 | 41216192137 | 53376697970 | 92849667067 | (2, 43964737334) | 135799598603144 | J1 |
   | `icv1-fp41-tm405553-81153b6e` | 41 | 1 | 1495523448539 | 640197488525 | 566237084442 | 1495523854093 | (1, 949085523714) | 119518173357916 | J1 |
   | `icv1-fp41-tm886935-baa24912` | 41 | 2 | 1930433675293 | 28146981299 | 897732060440 | 1930434562229 | (1, 969460461504) | 181458334271012 | J1 |
   | `icv1-fp45-tm6458695-fe3d5bd2` | 45 | 1 | 27611383581911 | 12878670601399 | 4914515096856 | 27611390040607 | (4, 7577605687942) | 185150999512726 | J2 |
   | `icv1-fp45-t1999407-5563cb34` | 45 | 2 | 25361112161527 | 20787366627629 | 12019143787601 | 25361110162121 | (1, 11221611220376) | 196853410870112 | J2 |
   | `icv1-fp49-tm10234315-a355a6d8` | 49 | 1 | 365857381347911 | 265803930896159 | 186608017781445 | 365857391582227 | (2, 209161832331426) | 40016877808970 | J3, J4 |
   | `icv1-fp49-tm34125029-11a5de23` | 49 | 2 | 561006233595917 | 429062284978109 | 409501054401560 | 561006267720947 | (2, 13191095788455) | 203571419754608 | J5, J6 |

   log2 r = 32.37, 32.74, 36.03, 36.43, 40.44, 40.81, 44.65, 44.53, 48.38 and
   49.00. EC1 aliases are assigned at registration.
2. **Protocol.** I wrote the frozen-to-be protocol `PROTOCOL.md` and
   `protocol.json` (`experiment.protocol/v1`, `freeze.status: draft`):
   - Arms: `rho.negation` (reference), an A/A `rho.negation`, `rho.plain`
     (positive control, sqrt 2), `bsgs.negation` (exact constant 1.0) and
     `rho.negation_v2` (secondary).
   - Size: 8 targets x 6 measured rounds + 1 warm-up, `alternate` order,
     L2. That is 96 runs per size per arm and 480 per arm.
   - Seeds are derived (`seeds.json`).
   - The §5a band: every bound is fitted at w = 0, 0.1 and 1.
   - The R(N) models: A, `1 + c N^-beta`, and B, `R_inf + c N^-beta`.
   - The decision table: `setup_excess` / `within_1.10_strict` /
     `walk_shortfall` / `intermediate_excess` / `unresolved` /
     `inconclusive`.
   - Predictions P1-P9 were stated before data, together with the data
     already seen, and the stop and rerun rules are written in.
   - Validation: the crypto-autoresearcher checker (`experiment_protocol.py
     validate`, HEAD 477a3273) printed `OK ... (0 finding(s))`. I ran it from
     a scratch copy with its schemas taken by `git show`, because the sparse
     checkout has no `schemas/`.
3. **Specs.** `build_specs.sh` (jq only) writes `specs/J1..J6.json`, the
   `specs/smoke-dev.json` development spec and `requests/J1..J6.request.json`.
   - `ecbench plan` (binary sha256
     `2522feb865b254d6e8847f97f6b0aa5dfaae0421b417336cf9d27bae3b969c49`,
     built here from the working tree at `413114389d`) accepted all six with
     no warning. Their spec ids:
     - J1 `ECS1h95ff8c1ab689`, 1 680 executions;
     - J2 `ECS1h33628e74730d`, 560 executions;
     - J3 `ECS1h9fb5ed79eaed`, J4 `ECS1hed99ca6aaf5a`, J5 `ECS1h5c47da15a3f2`
       and J6 `ECS1h8736acba2d03`, 140 executions each.
   - I first used 5 rounds, but `alternate` warned that an odd count is
     unbalanced, so I moved to 6.
4. **Development checks**, local, not results:
   - **Smoke.** `specs/smoke-dev.json`: 33-bit curve 1, 2 targets, 1 round, all
     five arms, `--allow-busy`. Session `ECBS1h565b74345e03`: 10/10 verified,
     and `verify --replay-all` gave `audit OK: 10 records, 10 verified, 10
     replays reproduced` (receipt sha256 `e6fd5afa50fa...`). The A/A counts
     were identical. `ecbench bound fit --tier medium` on it gave an
     admissible constant bound with domain tier `medium`, and `bound check`
     re-derived it. The repeated `--dir/--audit` form of `fit.sh` worked on
     two dev sessions. The unpriced counters seen were:
     - `rho.negation`: `canonicalisations_uncharged`, about 1 per GAE, so S
       roughly doubles at w = 1;
     - `bsgs.negation`: `inserts_uncharged` and `lookups_uncharged`;
     - `rho.negation_v2`: canonicalisations plus the DP-table queries and inserts;
     - `rho.plain`: none.

     `rho.negation_v2` is not trajectory-identical to `rho.negation`: in one
     pair its GAE was 142 921 against 80 535.
   - **Pre-flight timing.** One 49-bit target (`icv1-fp49-tm34125029-11a5de23`),
     one round, four arms, on this shared and unisolated 4-core Xeon at
     2.1 GHz (L1): `rho.negation` 213, `rho.plain` 205 and `rho.negation_v2`
     217 ns/GAE; `bsgs.negation` 414 ns/GAE with 412 MiB RSS. At 33 bits it
     was 147-176 ns/GAE. All four verified, and the replay-all audit was OK.
   - **R(N) fit.** `examples/prime_medium_tier_rho_rfit.rs` (new, native) on
     the toy record alone gave model A beta 0.826 [0.408, 1.748] and c 101.7,
     with chi2 1.65 on 4 dof (p 0.80). Model B's `R_inf` is not determined by
     toy data alone: 1.13 [-1.16, 1.33] (`analysis/rfit-toy-only-rho-neg.json`).
     This is derived from cited data.
5. **Requests.** `requests/J1..J6.request.json` (`isolab.eks.request/v1`:
   c8a.2xlarge, 3 600 s, 4 Gi, the spec inline, 2.2-3.6 KB) pass
   `taskq/deploy/eks-karpenter/remote/validate-request`, the JSON schema
   `contracts/request.schema.json`, and the pod's own spec gate in
   `run-ecbench` (L2, at least 5 rounds, a warm-up, a reference, a control).
   `finalize_requests.sh IMAGE SOURCE_SHA ECBENCH_SHA` adds `job.build`, binds
   `provenance.source_revision`, checks the inline spec against the committed
   spec, and revalidates. I dry-tested it on a scratch copy with dummy values.
6. **Analysis scripts.** `fit.sh` contains the bounds for 4 arms x 3 weights,
   `bound check`, `frontier build` and the three R(N) fits. The
   `prime_medium_tier_rho_rfit` example has sha256 `abcdb0263ef1...`, and
   `fit.sh` has `9fd4e3860933...`; both are pinned in `protocol.json`.

## Results (with evidence class)

There is no measured result; the study has not run.

- Development checks only: the harness executes every arm on these curves,
  replays exactly and fits a medium-tier prime bound.
- Prediction, written before data: `rho.negation` will be about 1.06 x floor
  at medium sizes, inside [1.00, 1.10], which gives the verdict
  `setup_excess`. The prediction rests on the unsealed prior pool (1.062)
  and on toy-only model A, which predicts R of about 1.01 at log2 r 32.

Derived consequence, if the prediction holds:
- A prime IC gap normalised to a toy rho at 1.2-1.5 x floor understates the
  distance to a medium-tier rho by that factor. This is the lead's "gaps grow".
- Gaps measured against a matched rho in the same session do not change.

## Decisions made (and why)

- **Field bits 33/37/41/45/49, not 32-48.** A 32-bit field is toy tier, and
  a fit that spans tiers is refused. b = target + 1 puts log2 r in
  [32, 33), [36, 37) and so on.
- **`prime_orbit_shape` generic, then frozen as `prime_explicit`.** It is the
  only in-harness deterministic generic generator above 32 bits. Freezing the
  curves explicitly mirrors the toy calibration, so no audit repeats the
  search.
- **Two curves per size** so that the curve effect is estimated, as a
  diagnostic.
- **Arms.** I added `rho.negation_v2` because `docs/ecbench/README.md` says
  the `_v2` ids are "the ones to read new work against". The primary stays
  the frozen `rho.negation`, so that toy and medium are one method id for
  R(N). I did not add `bsgs.negation_v2`: it differs only by one counted
  final check of about 1.5 log2 r GAE.
- **Six jobs.** EKS caps a job at 3 600 s, and the pod's `--replay-all`
  doubles the solve time. Each 49-bit curve is therefore split into two
  four-target blocks with different target seeds, which makes them
  different workloads, so the bound fit accepts both.
- **6 measured rounds**, because `alternate` with 5 was flagged as unbalanced
  and EKS needs at least 5.
- **Count admission despite an L2-gate failure.** A complete, audited session
  is admitted for counts even if the pod's L2 gate fails; only wall time is
  dropped (levels gate wall time, never counts). An incomplete session gets
  one rerun.
- **Decision thresholds 1.10 and 1.30** come from the lead's discriminator.
  "Strictly within 1.10" is a separate line because 480 runs will not reach
  an upper bound of 1.10 or below if the truth is about 1.06.

## Open threads

- **Register the ten slugs** in the PR that lands this (AGENTS.md §11). Once
  `curves.json` and `specs/*.json` are tracked, run
  `python3 scripts/build_curve_registry.py`, which harvests tracked JSON with
  `p, a, b, group_order` (I did not run it: it rewrites shared registry
  files). Then rerun `ecbench plan` and check `registered=true`.
- **Freeze `protocol.json`** after the image exists: fill
  `arms[].instrument.commit` and `binary_sha256` (currently `null`), then
  `freeze`.
- **An image with this commit's binary is required.** The deployed default
  image is `cb2ec6873f30` from 2026-10-19. I could not check whether it has
  `rho.negation_v2` because the history is shallow. Use a commit-built image.
- **Ask for a review-run** of the export after the run.
- Make the research-visuals report, graph and PDF after the run (the
  research-visuals skill); none exists now.
- **Not done on purpose:** `scripts/checkpoint.sh init`, `tools/benchrec.py
  init`, any EKS launch and any commit. The main session does these
  (PARALLEL_RULES).

## Errors

- ecdb returned 503 three times. I fell back to repository files.
- `lead_atlas.py` and `build_frontier_map.py` failed with
  `ModuleNotFoundError: orchestration` (sparse checkout). A scratch YAML shim
  worked around it.
- `experiment_protocol.py validate` first refused because
  `schemas/experiment-protocol.v1.json` was missing from the sparse checkout.
  A scratch copy with the schemas from `git show HEAD:schemas/...` worked.
- The first `protocol.json` had 27 schema findings and then 8 more: enum
  values, null instrument fields, the negative control and derived-metric
  formulas. All are fixed; the validator now prints 0 findings.
- One jq syntax error in the `curves.json` assembly; fixed.
- The ecbench build waited about 20 minutes on the shared cargo lock.

## Ready-to-run

The artifacts are under `research/prime_medium_tier_rho_20261107/` plus
`examples/prime_medium_tier_rho_rfit.rs`:
- `PROTOCOL.md`, `protocol.json`, `curves.json`, `seeds.json`;
- `specs/J1..J6.json`, plus `specs/curve-generation.json` and `specs/smoke-dev.json`
  (dev only);
- `requests/J1..J6.request.json`;
- the scripts `build_specs.sh`, `finalize_requests.sh` and `fit.sh`;
- `analysis/rfit-toy-only-rho-neg.json` and `sources/prior-pool.txt`.

**Compute estimate.** This is derived from this host's development timing,
which is conservative for c8a. The solve model per job is 7 rounds x targets x
sum over arms of (S x sqrt(r) x ns/GAE): the rho arms at 215 ns/GAE with
S = 0.94 (neg, aa, v2) and 1.32 (plain), and bsgs at 415 ns/GAE with S = 1.0.
That is about 1 305 ns x sqrt(r) per workload and round. The pod's replay-all
doubles it, plus a 60 s settle and at most about 0.3 s of overhead per
execution.

| job | solve + replay | executions | with overhead | 
|:--|--:|--:|--:|
| J1 | 486 s | 1 680 | about 1 100 s |
| J2 | 1 500 s | 560 | about 1 750 s |
| J3, J4 | 1 396 s each | 140 | about 1 550 s each |
| J5, J6 | 1 732 s each | 140 | about 1 900 s each |

Totals:
- About 1.6e10 GAE solved, and 3.2e10 counting the replays.
- About 2.3 h of solve and replay in total, and about 3-3.5 c8a.2xlarge
  instance-hours with provisioning.
- The jobs run one at a time, since the L2 pool is 8 vCPU.
- Every job fits under 3 600 s with 30-50 % margin, provisioning included.

Proposed ceiling: USD 10. It covers 6 jobs plus up to 2 reruns at at most
1 h each, CodeBuild launchers and the image build. The price per hour is an
assumption, so check it against COST_CONTROLS.md before spending.

**Launch sequence** (main session; the experiment director freezes it into a
card):

```bash
cd /home/user/crypto
# 0. Land the inputs on a pushed branch: protocol, specs, requests, scripts,
#    curves, seeds, example. Register the curves first:
python3 scripts/build_curve_registry.py
python3 taskq/deploy/eks-karpenter/check-experiment-git --repository aburan28/crypto \
  --path research/prime_medium_tier_rho_20261107/protocol.json \
  --path research/prime_medium_tier_rho_20261107/PROTOCOL.md \
  --path research/prime_medium_tier_rho_20261107/curves.json \
  --path research/prime_medium_tier_rho_20261107/seeds.json \
  $(for j in 1 2 3 4 5 6; do printf -- '--path research/prime_medium_tier_rho_20261107/specs/J%s.json ' $j; done) \
  --path research/prime_medium_tier_rho_20261107/finalize_requests.sh \
  --path research/prime_medium_tier_rho_20261107/fit.sh \
  --path examples/prime_medium_tier_rho_rfit.rs > research/prime_medium_tier_rho_20261107/publication-receipt.json
# 1. Commit-built image without running anything:
gh workflow run isolab-experiment.yml --repo aburan28/crypto --ref main \
  -f source_revision="$(git rev-parse HEAD)" -f publish_only=true
#    -> read the image URI (@sha256 digest), the source SHA and the ecbench sha256 from its receipt.
# 2. Fill protocol.json arms[].instrument.{commit,binary_sha256}, then freeze. The sparse
#    crypto-autoresearcher checkout has no schemas/, so use a full checkout:
python3 ../isogeny-labs/crypto-autoresearcher/tools/experiment_protocol.py freeze \
  research/prime_medium_tier_rho_20261107/protocol.json --commit auto --filed-before-affected-data
# 3. Bind the requests to that image and revalidate:
research/prime_medium_tier_rho_20261107/finalize_requests.sh IMAGE_URI SOURCE_SHA ECBENCH_SHA256
#    Commit and push requests/final/*.json and the frozen protocol.json, then rerun
#    check-experiment-git with those paths added (COST_CONTROLS.md: inputs pushed before spending).
# 4. Resume checkpoint (fill .resume/RESUME.md, every section), then push:
scripts/checkpoint.sh init && $EDITOR .resume/RESUME.md && scripts/checkpoint.sh push && scripts/checkpoint.sh gate
# 5. Benchmark record (one for the study):
python3 tools/benchrec.py init --slug prime-medium-rho-bound --problem rho:prime_medium_tier_bound
#    -> REC=benchmarks/records/<YYYYMMDD>T<HHMM>Z-prime-medium-rho-bound.json
# 6. Launch J1..J6 one at a time (the L2 pool holds one c8a.2xlarge), each under the record:
export AWS_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 \
       ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher
for j in J1 J2 J3 J4 J5 J6; do
  ISOLAB_CLIENT_OUTPUT=$PWD/research/prime_medium_tier_rho_20261107/receipts/$j \
  python3 tools/benchrec.py run "$REC" -- \
    taskq/deploy/eks-karpenter/remote/launch-remote research/prime_medium_tier_rho_20261107/requests/final/$j.request.json
done
```

If GitHub dispatch is preferred instead of the bridge, run one dispatch per
spec:

```bash
gh workflow run isolab-experiment.yml --ref main -f source_revision=SHA \
  -f spec_path=research/prime_medium_tier_rho_20261107/specs/J1.json \
  -f instance_type=c8a.2xlarge -f seconds=3600
```

After the runs:
1. Download each session to `sessions/J1..J6`.
2. Re-audit each with `ecbench verify --dir sessions/Jn --replay-all --out
   sessions/Jn.audit.json`.
3. Run `ecbench table`, `ecbench band`, and `ecbench compare --save` with
   `--a rho-neg --b X` for X = rho-plain, bsgs-neg, rho-neg-v2 and
   rho-neg-aa.
4. Run `sh research/prime_medium_tier_rho_20261107/fit.sh`.
5. Build the export with `experiment_protocol.py from-ecbench` and check it
   with `check-export --verify-files`.
6. Record each raw file and metric in the bench record with `benchrec
   add-raw` and `benchrec metric`, giving every arm's medium constant in `S`
   at w = 0, 0.1 and 1. Then run `benchrec close`.

**`benchrec init` fields.** It requires `--slug`
(`prime-medium-rho-bound`) and `--problem` (`rho:prime_medium_tier_bound`).
`--curve` (a curve UID or EC1) is optional; the study spans ten curves, so
omit it. `--entry benchmarks/<date>-<slug>` applies only if a `benchmarks/`
row is added: a bound is not a speedup, so it is probably not needed, and a
row would need a `VALIDATION.json`. `close` then needs `--status`,
`--summary`, the raw files (records.jsonl per session, the audits) and the
report/SVG once they exist.

**Checkpoint `RESUME.md` content to fill:**
- **Goal:** this README's Goal.
- **Lead:** LEAD-RHO-matched-rho-calibration.
- **Spec/protocol:** this directory, with the protocol and spec sha256s.
- **State:** launch-ready and unrun.
- **Next steps:** the launch sequence above.
- **In flight:** each job's name `prime-medium-rho-20261107-jN`, its
  CodeBuild id, Job/Pod UID and S3 prefix.
- **Stop/success:** PROTOCOL.md §9 and §10.

## Shared files edited

None. New file outside this directory: `examples/prime_medium_tier_rho_rfit.rs`
(slug-prefixed, auto-discovered example; it adds no `Cargo.toml` entry).
The local `ecbench` build and the example build wrote only to the shared
`target/`.

## Amendment note (2026-11-07 21:05 UTC, before any data)

`examples/prime_medium_tier_rho_rfit.rs` was reformatted with
`rustfmt --edition 2021` to satisfy the repository's `rustfmt (changed files)`
check. The change is layout only. Its sha256 moved from `abcdb026…cdd1` to
`b79e7d00…0837`, and `protocol.json` records the new hash. The protocol still
validates with 0 findings. No measurement existed at either hash.
