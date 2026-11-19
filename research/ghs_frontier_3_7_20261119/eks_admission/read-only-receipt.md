# Read-only admission receipt

Observed on 2026-11-19 UTC from branch
`codex/ghs-frontier-3-7-20261119`. Secret values are not recorded.

## Identity and local clients

```text
aws sts get-caller-identity --output json
Unable to locate credentials. You can configure credentials by running "aws configure".

codex mcp get isolab-eks
Error: No MCP server named 'isolab-eks' found.

command -v kubectl
(no output; command absent)

gh auth status
authenticated as aburan28
```

The GitHub repository API returned admin/maintain/push/pull/triage permissions
for the authenticated identity. Listing Actions variables returned HTTP 403;
their values and presence are not inferred.

## Source custody

```text
git rev-parse HEAD
2b88bdd84b2c102da686d4c589b2d6a68cea07d3

git ls-remote origin codex/ghs-frontier-3-7-20261119
2b88bdd84b2c102da686d4c589b2d6a68cea07d3 refs/heads/codex/ghs-frontier-3-7-20261119
```

That records the tracking commit as published. It does not freeze the final
experiment source; concurrent implementation changes remain in the worktree.

## Checkpoint and pool gates

```text
scripts/checkpoint.sh gate
checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /home/agent/.config/checkpoint/upload-token
checkpoint: cannot upload, so nothing was checkpointed
checkpoint_gate_exit=1

scripts/isolab_pool_check.sh --cpu 8 --memory 8Gi --pool cpu-spot --save /tmp/ghs-frontier-pool-check.json
isolab_pool_check: kubectl not found (use --from-json)
exit=3
```

No saved pool bundle was created. A hand-written or historical bundle would not
be a live all-tenant receipt.

## GitHub workflow

Workflow `390723163` is active on main commit
`ad2a014c0022cf000845014499a9649b6c2cef2d`. Its exact YAML has Git blob
`c66a56fe4ab501293ed766ede670d9121b30fac2` and SHA-256
`82f6065c5fa2839eede0947b5e505564447d59ebf19fcf4357dc5a286ec70369`.

Latest manual run:

```text
run: 44374607134
event: workflow_dispatch
conclusion: failure
contracts / Pull store data this workflow reads (tools/artifacts): failure
build: skipped
experiment: skipped
```

The workflow accepts an exact `source_revision`, builds and publishes an
`ecbench` executable, and launches an audited ECBench request. Its only custom
command payload is the separately hard-coded norm-branch census. It does not
bundle or launch the standalone GHS driver.

## Standalone request controls

```text
python3 -m unittest research.quartic_ghs_strong_rho_20261114.test_make_requests
Ran 7 tests in 0.003s
OK
```

These tests validate the generator's fail-closed preflight and image bindings.
They do not establish a live operator receipt, image, request, capacity, L2, or
measurement.

