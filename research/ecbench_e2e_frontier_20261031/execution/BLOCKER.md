# Execution status (2026-10-31 UTC)

The current-source frontier refresh is being prepared, but no measurement has
been submitted. Missing evidence remains `blocked`, never zero cost or a
performance loss.

## Route now verified

- GitHub CLI authentication works for `isogeny-labs/crypto`.
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

## Design issues found before freeze

The nine-spec draft is not ready to execute unchanged:

1. Its source, static binary, and image are now current and exact, but the
   workload roster is not prospectively frozen.
2. The 74-method statement inventories top-level registry IDs, not every
   parameterized implementation. Current main adds the orbit-layout table,
   generator-orbit/stride configurations, and the two-word F_(2^m)^4 IC/rho
   path without necessarily adding a new method ID.
3. It has no fresh random-binary cell and no dedicated cubic/quartic symmetry
   cells, so it cannot establish the claimed boundary in those domains.
4. `rho.signed_frobenius_strong_escape` is now present in each draft Koblitz
   spec, but its common-accounting selection rule against strong-v2 still has
   to be frozen before measurement.
5. The nine specs plan 4,704 process executions. A trusted benchmark job has a
   3,600-second whole-job deadline, while draft per-process timeouts reach
   3,600 seconds. The campaign must be split into candidate/reference/A-A
   shards with bounded censoring before freeze.
6. The one-column generator-orbit result is a separate toy fixed-setup
   engineering crossover. It does not support the unqualified sentence that
   every Koblitz cell is rho-led, nor does it overturn the medium/scaling
   Koblitz frontier.

## Unblock and resume

Provision a checkpoint upload token for this workspace. Then:

1. run `scripts/checkpoint.sh push` and `scripts/checkpoint.sh gate`;
2. replace the draft with a shardable, prospectively frozen protocol and
   closed implementation roster;
3. merge the preregistration to `main` before any warmup;
4. submit each queue shard at most once, replay all records, and update the
   ranked leaderboards/PDF only from admitted evidence.
