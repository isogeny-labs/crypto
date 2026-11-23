# Prepared Prime31 image and execution route

## Status

`check-prepared-inputs.sh` performs **metadata checks only**. It recognizes
`prime31-isogeny-choice.cell/v3` and `prime31-complete-ic-incumbent.cell/v1`, checks
necessary tracked source/selection/protocol hashes and limits, and prints
`launch_admitted:false` and `experiment_executed:false` even on success.
It is not a launcher, does not authenticate execution receipts, does not call a
scientific binary or checkpoint uploader, and creates no output directory.
The older frozen
`isogeny-choice/run-choice.sh` and `sparse-h2-followup/run-staged.sh` are unchanged.

The two native interfaces already exist:

| Input | Native binary | Existing commands |
| --- | --- | --- |
| ChoiceCell/v3 | `prime31-isogeny-choices` | `validate-cell`, `source-check`, `source-manifest`, `run`, `replay` |
| Complete-IC incumbent cell/v1 | `prime31-ic-incumbent` | `validate-cell`, `source-check`, `source-manifest`, `run`, `replay` |

Recognizing a schema does not prove that its arithmetic input is valid. Native
input validation and independent result replay remain required in any future
admitted runner. The metadata checker does not execute them.

## Existing infrastructure, without a Prime31 payload

The maintained AWS bridge is
`taskq/deploy/eks-karpenter/remote/launch-remote-command REQUEST.json`.
It accepts `isolab.eks.command-request/v1`; the implemented request handling is
in `remote/run-command`. The envelope has `job` and `provenance`; the job carries
`job_name`, `image`, `command`, `cpu`, `memory`, `seconds`, `isolation_profile`,
`instance_type`, `retries` and `wait_seconds`. This bridge uploads the request,
starts the existing CodeBuild launcher and reconciles its S3 receipt. It is a
paid submission route, not a local validation command. No submission was made.

The current `run-command` requires an image digest under the configured IsoLab
ECR repository. Its CPU field accepts only even values from 2 to 28; `cpu=1`
is rejected. A future command job must therefore reserve at least **2 vCPUs**,
keep the computation on **one affinity CPU and one thread**, and charge the full
reservation. This does not authorize changing the protocol or exceeding capacity.

The current `isolab-experiment.yml` workflow builds an immutable source revision
and publishes an image, but `build-bundle` packages `ecbench` and
`isolab-fresh-owner`. Its additional command mode packages the separately frozen
norm-branch census. It does not package either Prime31 binary, a Prime31 execution
wrapper, selected Prime31 inputs or their protocol. A source SHA in that workflow's
provenance cannot make the default executable become the Prime31 executable.
The source-checkpoint and read-only budget-telemetry workflows also exist;
neither supplies a Prime31 image or establishes this lane's execution admission.

## Missing packaging and verified execution evidence

A metadata/source constructor now exists in `prepare-runtime-bundle.sh`; see
`RUNTIME_BUNDLE.md`. It consumes existing binaries, preserves a real sparse Git
checkout and records actual ELF dependencies. Its fixed contracts use tiny real
Git fixtures; actual Prime31 end-to-end packaging and the container runtime remain
unqualified. It neither builds nor publishes an image and is not a launcher.

A future source-bound build must package the selected Prime31 binary and runtime,
embedded source manifest, selected input, execution wrapper and required
protocol/selection material. Preserve source revision, binary SHA-256 and OCI
digest as distinct identities, then verify the actual submitted Job/Pod image.
The existing command-job interface can execute that packaged command.

The execution wrapper for the new cell schemas is **not implemented here**. It
must preserve the existing publication/checkpoint gates, original deadline,
zero retries, fresh output directory, native source/input checks, independent
result replay and complete failure retention. The metadata checker cannot replace
it. No new provider, account, IAM identity, queue or quota is proposed.

The old `prime31-isogeny-choice.selection/v1` fields can express a stage, exact
cell and protocol hashes, pilot hash, original deadline, zero retries and the
USD5 aggregate ceiling. They do **not** define how to authenticate/reconcile
publication, checkpoint, lane reservations, capacity and image execution.
The older wrapper checks only that publication and budget receipt files are
nonempty. Copying that test would not verify their contents or applicability.

`BUDGET_TELEMETRY.md` explicitly says `launch_admitted=false`; delayed account
budget values do not supply current lane reservations or capacity. A positive
number in a selection is a necessary limit, not verified spending headroom.
The canonical protocol checker must also be available and source-pinned; an
archived schema or the `experiment.protocol/v1` header cannot substitute for it.

The existing execution requirements still need actual evidence for these facts:

1. Exact published source revision and all input, wrapper, selection and protocol
   bytes; current canonical checker identity, successful validation and freeze.
2. Successful `scripts/checkpoint.sh gate` and upload/readback evidence for that
   immutable source and protocol. A file named `checkpoint.json` is insufficient.
3. The inherited lane's charged and reserved costs, including earlier attempts,
   build/idle/failure charges, and the remaining balance under **USD5 total**.
   Current daily/monthly headroom and a reservation for this selected attempt
   must be reconciled; unknown spend remains unknown.
4. Fresh all-tenant pool/namespace capacity from the maintained pool-check route,
   bound to the requested placement. No quota or provider changes are implied.
5. OCI digest, actual job/pod image identity, binary SHA-256 and embedded source
   manifest. Package the precise selected binary and required runtime alongside
   source/protocol/selection custody; do not add a mutable clone/build at launch.
6. One selected attempt, one computational thread and affinity CPU, **4096 MiB**,
   **120 seconds** outer bound (TERM at 115, KILL after 5), zero retries, and the
   existing **original absolute deadline**. Provisioning/builds cannot reset it.
7. Immutable new result location and complete preservation of receipts, stdout,
   stderr, interrupted/failed outcomes and replay. Timing remains diagnostic
   until separately qualified L2/A/A admission exists.

These facts may be verified individually through legitimate existing tools when
their real interfaces and credentials are available. This document does not
require a new centralized verifier or admission service. It does not define a
substitute receipt schema, IAM grant, launch reservation or frozen protocol.

## Using the metadata checker

Run `bash check-prepared-inputs.sh ABSOLUTE_CELL.json
ABSOLUTE_PROPOSED_NEW_OUTPUT_DIR`. The proposed output is inspected for freshness;
it is never created or modified. These variables identify **metadata files only**:

| Path variable | Metadata read |
| --- | --- |
| `PRIME31_SELECTION` | Exact tracked selection using the existing selection/v1 structure |
| `PRIME31_CANONICAL_PROTOCOL` | Exact tracked protocol bytes named by that selection |
| `PRIME31_PILOT_RECEIPT` | Required for complete recovery; only its selected hash is checked |

The checker rejects missing, empty, ambiguous JSON or symlink metadata files,
modified/untracked source inputs, duplicate cell selections, altered limits,
unknown declared balance and an expired/too-short deadline. Success says only
that these necessary metadata checks passed. It explicitly lists execution
prerequisites it did **not** verify. An `admitted` string in the selection or a
positive declared balance does not confer launch authority.

## Evidence inspected

- `AGENTS.md`, checkpoint and IsoLab execution/cost requirements.
- `isogeny-choice/run-choice.sh`, `sparse-h2-followup/run-staged.sh` and
  `sparse-h2-followup/selection.template.json`.
- `taskq/deploy/eks-karpenter/{ACCESS.md,BUDGET_TELEMETRY.md,build-bundle}` and
  `remote/{launch-remote-command,run-command}`.
- `.github/workflows/isolab-{experiment,source-checkpoint,budget-telemetry}.yml`.

This is development and packaging analysis. It creates no run, budget balance,
performance result or end-to-end advantage claim.
