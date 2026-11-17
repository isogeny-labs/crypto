# Split-graph secant: native exact extractor and a frozen complete-cost comparison

2026-11-07. Slug `sgsecant`. Lead:
`knowledge/leads/ecdlp/ic-prime/LEAD-ICP-split-graph-secant-presentation.yaml`
(crypto-autoresearcher checkout). All numbers below are `[pcc:unverified]`; every
number from a run is a development check, not a measurement for a claim.
No frozen-seed data exist.

## Goal

Advance the lead to a frozen, launch-ready complete-cost comparison: implement a
native exact triangular/FGLM extractor with no assignment search and with sharing
across the four ordinate-sign branches, check it against the saved-basis
situation and the historical caps, freeze the prospective protocol (>= 3 growing
cells, all arms and charges, decision rule), and deliver the command and cost
estimate for the EKS command route. The panel was **not** run.

## Sources checked

* research-kb: **not available** (`scripts/research-kb` is not executable here).
* Atlas tools: `lead_atlas.py --match` and `build_frontier_map.py --match` both fail
  under `python3 -I` with `ModuleNotFoundError: orchestration` (the sparse checkout of
  crypto-autoresearcher lacks that package; `schemas/` is missing too). Fallback: read
  the lead YAML and `grep -rliE "secant|fglm|triangular|split-graph|boolean-offset"`
  over `knowledge/leads/ecdlp/`. Hits: the lead itself,
  `LEAD-ICE-prime-field-semaev-solving-degree` (closed_known: naive prime-field Semaev
  Groebner costs beta_work 0.65 to 0.80 against a 0.30 gate; "revisit when a bounded-degree
  membership base is measured flat across >= 3 field sizes"),
  `LEAD-ICP-gfp5-symmetrized-gaudry` (mentions FGLM as a finish), `LEAD-TRF-jv-nagao-genus2-cover-decomp`.
  No hit records a triangular/FGLM extractor for split-graph bases, so "not tried" is
  supported only to the extent of this grep; not a literature-wide novelty statement.
* Prior work read: `research/prime_factor_base_degree_20261031/SECANT_GRAPH.md`,
  `parallel_20261031/{RESULTS.md,HANDOFF.md,results-summary.json}`,
  `pencil_search_20261101/README.md`, and the code that builds the systems
  (`parallel_20261031/f4/main.rs` `expected_model` and `roots`, `ladder/{core.rs,main.rs,spec.json}`).
* Route documents read: `.claude/skills/isolab-eks/SKILL.md`, `taskq/deploy/eks-karpenter/{ACCESS.md,remote/launch-remote-command,remote/run-command,image.json,Dockerfile.ecbench}`.
* Conductor: not installed (not attempted).

## What was done

* `native/` (dependency-free Rust, 2,882 lines, files `sgsecant_*.rs`): counted Buchberger
  (DRL), FGLM to lex, triangular rational-root extraction by univariate gcd and
  equal-degree splitting, candidate verification, exact per-phase ledgers (mul, add, inv,
  native), the system builders (secant branch, epsilon-shared, dense S3), group replay,
  relation rows with online rank mod n and final log recovery, an internal Pollard rho
  reference, the historical ladder reproduction (own SHA-256), the panel driver and the
  decision logic. `sgsecant_recheck.py` recomputes every ratio from `records.jsonl`.
* `DESIGN.md`: the algorithm, the sign-branch sharing proof (E1 four solves, E2 two
  epsilon-shared solves with eta classified by evaluation), counters, limits.
* `PROTOCOL.md`: frozen protocol (6 cells, 4 instances, 30 runs, arms, charges, controls,
  decision rule, disclosure of what development runs preceded it, Stage 2 outline).
* `eks/`: request template (passes the launcher's own `jq` structural validator with
  placeholder digest/revision), `build_and_publish.sh`, `launch.sh`. Not run.
* `dev_logs/`: logs of the development checks below.

## Results (development checks; evidence class in brackets)

* [measured] Univariate root finding equals brute force on 1,200 random polynomials over
  F17, F83, F127, F491.
* [measured] Historical ladder reproduced with my SHA-256 and recipe (seed 315): curves
  a,b = (59,30) F83, (3,58) F101, (20,61) F127 and graph proposals 1/4/23, 3/10/260, 1/2/62
  for d = 3/4/5, equal to `results-summary.json`.
* [measured] Exact root sets on all finite targets of those nine cells and the F17 fixture,
  graph / low-x / hash-x bases, E1 / E2 / dense: 87 rows, 0 mismatches, 0 censored, 0
  triangular fallbacks (`dev_logs`). Handled degree d+1 (graph secant) and 2d+2 (dense).
* [measured] Historical caps: a faithful port of the 2,000,000-assignment extractor on my
  reduced bases caps on exactly 12 candidate-secant, 6 low-x and 8 hash-x systems of
  F127/d=5 (26), and the new extractor returns the exact root set for each. The historical
  panel stays censored; nothing was rewritten. The saved bases themselves are not in this
  checkout (`runs.tar.xz` is absent), so the check is against recomputed reduced bases, which
  are canonical.
* [measured, extractor-only, F127 d=5, all 138 targets, graph base] field ops / native ops:
  E1 1.33M / 4.6M, E2 3.93M / 1.07M, dense 1.41M / 10.4M. With relation collection included
  (development seeds), `cs1/cd` was 1.0 to 1.4x in field ops and below 1 only with native work
  priced at 1 per operation; `cs2/cd` about 2 to 4x. The graph construction cost grows fast
  with d: at d=8 (development seed) construction is about 17.5M field ops per relation-equivalent
  (about 3e8 in total, mostly rejected proposals) against about 8e4 per relation of solving,
  so including it dominates at d >= 7.
* [measured] Determinism: identical `records.jsonl` (wall field removed) for 1 and 4 threads
  and for glibc and musl builds. Independent Python recomputation: 0 mismatches.
* [derived, from the dev numbers] The expected frozen outcome is REJECTED on field operations
  with a native-weight-dependent order (UNRANKED under section 5a). Written into PROTOCOL
  section 10 before any frozen data. This is a prediction, not a result.
* Not measured: any frozen-seed number, any timing, cross-engine (msolve/F4) cost, higher arity,
  256-bit fields, the matched ecbench rho, anything about the Boolean-offset successor.

## Decisions made (and why)

* Own counted engine instead of `f4_fp.rs`: that module cannot be built standalone
  (imports `groebner_f4`, `fx_hash`, `jv_bitops`, `glv_invariant_base` and through them
  `crate::ecc::field`), the library needs a full dependency set with no local registry
  cache and a contended cargo lock. The engine is Buchberger, not matrix F4, used for every
  arm; conclusions are about this engine at toy scale.
* Primary decision contrast is same-base excl. construction (construction cancels), with
  incl. construction reported; cross-base contrasts are primary incl. construction.
* Two pre-registered candidates (`cs1` unshared, `cs2` shared) instead of picking the
  shared one after the fact; the rule uses a 5% materiality margin and requires the A/A,
  self and exact-2x positive controls.
* Frozen seed never used in development; the early smoke seed 20261107 is declared burned.
* No schema-validated `protocol.json`: the schema and checker are missing from the sparse
  checkout.

## Open threads

1. Main session: wrap PROTOCOL.md in `experiment.protocol/v1`, `freeze --commit auto`,
   fill `.resume/RESUME.md`, open a `bench.record/v1` if the hooks require it (counts are
   not timings, but the hooks classify launchers), Conductor task, commit and push `native/`.
2. Build and publish the layer, fill the request template, launch, then recheck.
3. Cross-engine check of the primary comparison through `f4_fp::solve` or msolve (needs the
   library build or an msolve binary; neither is present here).
4. Swap-symmetry (3 solves instead of 4) and z-elimination (z = m^2 - x - r) are untested
   cheaper variants of E1; not in the frozen arms.
5. Boolean-offset builder (7 variables at k=2) and its own frozen protocol (PROTOCOL section 12).
6. Matched ecbench rho with the section 5a band for any leaderboard-grade gap statement.

## Errors

* `python3 -I tools/lead_atlas.py` and `build_frontier_map.py` fail (missing `orchestration`);
  handled by grep fallback above.
* First `cargo build` waited behind other agents' cargo lock; killed and replaced by direct
  `rustc` for development, then a separate-target `cargo build` (scratchpad target dir, not
  the shared one) for the lockfile and musl build. `rustup target add
  x86_64-unknown-linux-musl` installed a rust-std component in the user toolchain (outside the repo).
* `rho` panicked once on an identity table point (fixed before the freeze); the first
  smoke ran under seed 20261107 before the frozen seed was separated (declared in PROTOCOL).
* Nothing else failed. Nothing was launched on EKS or AWS.

## Ready-to-run

Artifacts: `native/` (source lock in `native/SHA256SUMS`), `PROTOCOL.md` (sha256
`20acbf94d87b87f47fa20b4866c79143e9df6154e64cc1a07de9e56e10a84ed1`), `eks/request.template.json`,
`eks/build_and_publish.sh`, `eks/launch.sh`.

```sh
# after Conductor/RESUME/benchrec/commit+push (see Open threads 1):
bash research/split_graph_secant_cost_20261107/eks/build_and_publish.sh <40-hex-commit>
# fill job.image digest, provenance.source_revision, provenance.protocol_sha256 in a copy of the template
ISOLAB_CLIENT_OUTPUT=/abs/unique/receipts \
  bash research/split_graph_secant_cost_20261107/eks/launch.sh filled-request.json
# after completion: download runs/<job>/<pod-uid>/, sha256sum -c SHA256SUMS,
python3 -I research/split_graph_secant_cost_20261107/sgsecant_recheck.py out/panel
```

Panel command (inside the Job): `sgsecant panel --threads 4 --out $ISOLAB_OUTPUT_DIR/panel`
(frozen config printed by `panel --describe`).

Fit to the EKS command route: yes. One `isolab.eks.command-request/v1` Job, 4 vCPU, 8 Gi,
`seconds` 3600, `standard` profile, `retries` 0, `wait_seconds` 60, digest-pinned image in
`isolab/tasks` (base image is the AWS-CLI lab image, so `aws s3 cp` to `ISOLAB_RESULTS_URI`
works; verify the base image still has `aws`). The binary is static, so no glibc concern.
Command Jobs have no L2 gate, replay or A/A admission and no resume: acceptable because the
measurand is exact operation counts (wall time is diagnostic); a Spot interruption means a
full rerun under a new job name with both attempts retained. Cost estimate [heuristic from
development timing, same dimensions at a non-frozen seed, about 1,100 CPU-seconds of job work,
dominated by the d=8 cell, graph search about 150 s per instance]: 5 to 15 minutes wall on
4 vCPU, well under US$0.10 of Spot compute; launcher/CodeBuild minutes and the image build
dominate the real cost. Records are about 8 MB.

## Shared files edited

None. New files only, all under `research/split_graph_secant_cost_20261107/`.
