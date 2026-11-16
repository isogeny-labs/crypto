# Prime IC relation-solving tournament, evidence version 3

Status: frozen before all version-3 infrastructure and candidate data. The
machine-readable authority is `protocol-v3.json`, protocol SHA-256
`f5a8c9e7bad8e7f87200b614b4c1898fd0acf1c1c3b18b41d1c1a3001af8796a`.
It must be merged before the first dispatch.

## Request and evidence map

| Requirement | Fixed evidence |
|---|---|
| Run the index-calculus relation-solving tournament | Two independent `ic.pipeline` full-solve sessions under `spec-fp20-v3.json` |
| Measure a complete solve | Cold interval includes factor-base construction, oracle setup, relation collection, matrix solving, logarithm recovery and verification |
| Preserve correctness | Every scalar verifies; every measured row independently replays; candidate/incumbent complete GAE is exactly equal |
| Distinguish signal from noise | Same-session incumbent A/A plus the rho plain/negation positive timing control |
| Do not repeat the consumed evidence version | Fresh curve, target and measurement seeds; new protocol identifier and source-bound jobs |
| Do not repeat the known L2 failure blindly | One fixed, non-resumable L2 qualification must pass before either scientific job is dispatched |

## Claim boundary

The incumbent is `prime-abscissa:size=45` with
`mitm:negation_folded=1`. The candidate changes only the native pair index and
repeat guard to `layout=lines`. A supported result is a constant-factor native
engineering improvement in the generic-decomposition track on the measured
fp20 workload and AMD `c8a.2xlarge` class. It is not an asymptotic result, an
algebraic-track result or a claim to beat matched Pollard rho.

Relation-yield pruning from PR #3091 remains selection instrumentation. This
tournament does not turn a yield, coverage, rank or phase-only change into a
pipeline speedup: only the complete successful recovery boundary can decide the
comparison.

## Fresh fixed design

- Curve: public `prime_search`, 20 bits, seed `20261101141`.
- Targets: eight public targets, seed `20261101142`.
- Algorithm plan seed: `20261101143`.
- Arms: incumbent, identical incumbent A/A, line-layout candidate, matched
  `rho.negation`, and `rho.plain` positive control.
- Per scientific job: one warmup and eight measured rounds, interleaved.
- Replication: two concurrent, non-resumable EKS Spot jobs on distinct L2
  nodes; both sessions must independently pass every decision gate.

The v1 curve and targets were exercised by the excluded local diagnostic. The
v2 scientific inputs were never dispatched, but this separately authorized
evidence version still uses new seeds so no future reader can pool versions.

## Staged stopping rule

1. After this protocol is frozen and merged, dispatch exactly one build-bound
   `taskq/deploy/eks-karpenter/l2-validation.json` job. It qualifies routing,
   manifest verification and accepted L2 only; it is not candidate evidence.
2. If that job fails, is below L2, or lacks a manifest-verified receipt, retain
   it and stop version 3 without dispatching the scientific sessions.
3. If it passes, dispatch exactly two concurrent jobs using
   `spec-fp20-v3.json`, then stop. No GitHub rerun, replacement seed, selective
   host replacement or scientific retry is allowed.

The consumed version-1 workflows 41260778029 and 41260778856 and version-2
qualification workflow 41271383665 remain failures. Nothing in version 3
rewrites, retries or pools them.

## Decision rule

The candidate is supported within scope only if both independent sessions:

- pass native audit and full independent replay with no wrong or unpaired row;
- have candidate/incumbent complete GAE ratio and interval exactly `[1, 1]`;
- earn L2 on every measured pair;
- retain an incumbent A/A interval containing 1 inside `[0.9, 1.1]`;
- detect `rho.plain` as slower than `rho.negation`; and
- put both the complete total-wall 95% interval and the A/A-corrected wall
  interval wholly below 1.

One passing session, a stage-only ratio, a successful Kubernetes job or an S3
receipt is not the verdict. The frozen `analyze.sh` applies the exact rule to
authenticated session directories.

## Sources checked

- `research/relation_yield_pruning_20261031/REPORT.md`, including its
  source-linked PKM 2016 and Gaudry 2009 relation-probability boundary.
- `research/ic_linalg_pipeline_20261019/PROTOCOL.md` for the separation between
  relation collection, matrix solving and end-to-end accounting.
- `research/ic_candidate_tournament_20260915/MEASUREMENT.md` for complete cold
  solve and paired-control requirements.
- `docs/ecbench/README.md` and the Iso Lab harness, submission and recovery
  contracts for L2, replay, identity and non-resumable timing rules.
- `TERMINAL-OBSERVATION-20261101.md` and
  `V2-PREFLIGHT-OBSERVATION-20261101.md` for the exact consumed version-1 and
  version-2 failures and their evidence limits.
