# Execution status (2026-10-31 UTC)

The current-source frontier refresh is being prepared, but no measurement has
been submitted. Missing evidence remains `blocked`, never zero cost or a
performance loss.

## Route history and current access

- The trusted main-branch `isolab-experiment.yml` route accepted a
  publication-only request for source
  `ff7ad61cfed11908fb79de9dd485b696d0ada779`.
- Publication run
  [`41241104247`](https://github.com/isogeny-labs/crypto/actions/runs/41241104247)
  is an exact build/image publication only. It requested no EKS experiment.
- The repository's experiment queue is the appropriate execution route after
  preregistration: it permanently claims each frozen shard, runs native L2
  ECBench on Spot, retains failures, and returns result PRs without treating
  the queue as a scientific decision maker.
- In the current runtime, `gh auth status` reports the injected `GH_TOKEN` as
  invalid. There is also no direct Iso Lab/EKS MCP tool, AWS CLI, kubectl or
  Conductor client. The earlier publication proves the route, but it does not
  give this runtime current submission authority.

## Blocking admission gate

Repository policy requires `scripts/checkpoint.sh gate` before any experiment.
The handoff is complete in `.resume/RESUME.md`, but this workspace exposes none
of the three checkpoint transports:

- `CHECKPOINT_UPLOAD_TOKEN` is unset;
- the default checkpoint token file is absent;
- no AWS credentials or configured AWS CLI profile are available.

Consequently `scripts/checkpoint.sh push` cannot upload the resume checkpoint
and the gate cannot pass. No benchmark workflow or paid queue entry will be
launched until a checkpoint upload token (or equivalent approved transport) is
provisioned.

## Sharding completed; design gates still open

The original nine-spec draft remains planning history and will not be submitted.
It has now been expanded into 92 candidate/reference/A-A shards:

1. The exact pinned binary accepted all 92 specs, expanding them to 13,888
   processes with zero planner failures.
2. They cover all 13 top-level `current_primary` methods and all 11 nested
   solver backends, plus the factor-base, root-finding, symmetry,
   generator-orbit and F_((2^m)^4) parameterized panels named in the closed
   roster.
3. Every shard fixes at most 2,380 seconds of aggregate child timeout ceilings,
   leaving at least 1,220 seconds of the one-hour EKS envelope for workload
   construction, provisioning, audit and upload.
4. Koblitz shards carry strong-v2 and strong-escape plus an A/A copy of each;
   the paired minimum common-accounting `S` and union A/A envelope are fixed as
   the reference rule before data.
5. The extension-field public target constructor exhausted its fixed bound.
   Those eight implementation specs therefore use planted known-answer targets
   with scalars hidden from solvers and are excluded from public-target claims.

The deterministic analysis driver is now SHA-256-bound in `protocol.json` and
passes its 92-spec check-only gate. The remaining design blockers are
exact-image runtime preflight for each arm class, protocol freeze, and merge of
the exact spec IDs to `main` before the first warmup. The 92-spec set remains
draft until those are complete.

## Unblock and resume

Provision a checkpoint upload token for this workspace. Then:

1. restore an authorized GitHub/AWS/Iso Lab submission identity;
2. run exact-image runtime preflight and preserve the bound analysis-script bytes;
3. run `scripts/checkpoint.sh push` and `scripts/checkpoint.sh gate`;
4. freeze and merge the exact preregistration to `main` before any warmup;
5. reconcile existing receipts, submit each queue shard at most once, replay
   all records, and update the
   ranked leaderboards/PDF only from admitted evidence.
