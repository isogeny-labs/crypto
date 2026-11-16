# C1 terminal checkpoint diagnostic boundary

Read-only source and saved-artifact review, 2026-10-30. No cloud calls, mounts, resources, controllers, payloads, trials, or replay were performed.

## Finding

The retained status establishes a failed CRIU checkpoint at attempted generation 19, with generation 18 the last committed checkpoint. It does **not** establish CRIU exit 137 or an out-of-memory cause. The `exit_code:137` in supervisor status belongs to the scientific payload: `supervisor.rs:637–648` obtains it through `waitpid` on `s.pid`. At `693–698`, a failed checkpoint operation explicitly sends SIGKILL to that payload's process group. Consequently 137 is consistent with controller/supervisor cleanup after checkpoint failure. CRIU's actual exit status and stderr are a separate receipt, not this payload status.

## Exact source and identities

Reviewed source root: `/home/ec2-user/pencil-controller-review-20261029/source/taskq/deploy/eks-karpenter/resume`.

| File | SHA-256 |
|---|---|
| controller/src/supervisor.rs | 7cae58e46359ff72955ee52ac3e87c82987c68f20ffdf8778492e0e0a606f939 |
| controller/src/cloud.rs | ec633a6af0e9475731f0b7f9757828bfdd6b6baab6918ec6d082803a6cd0ec91 |
| controller/src/main.rs | 1460d79c9697f750a4a51b36830acfbc48a221cc3139bae7d861b96840bc9be5 |
| run-controller | 59fd9e1337af8560309cbb52dbb49821ebd2f0a10b19b3b69caef0f84e1d602b |

Prior reviewed publication identities: explicit package client `/home/ec2-user/pencil-controller-review-20261029/package/bin/isolab-eks-recovery`, SHA `ac7d57132617775679770dc55b6f8b3acc83f4454e31b025a1f35e9b915abc21`; controller ZIP SHA `0bd84d4009064968365f03518adbca1bf6fd6c27eeed24721721e20018a9e85a`; runtime image digest `sha256:e82333a5e77a03270335589848783e1d0cf55e42c7c2f5f35732ee8ba54523fd`. These identify the retained reviewed deployment lineage; this review makes no fresh remote image attestation.

## Where diagnostics are written

`supervisor.rs:313–357` creates `/work/generations/g19/capture-N/`, where N is the actual capture iteration (1 through 8; do not assume 1). CRIU runs `dump --tree <payload-pid>`, uses that directory as its images directory, and receives `--log-file dump.log` with verbosity 4 (`168–182`, `347–353`). Immediately after `.output()`, the supervisor writes `capture-N/dump-result.json` containing CRIU's own `result.status.code()` and captured stderr. Signal termination may produce a null `status.code()`.

A failed CRIU command returns the generic error immediately. It does not rename that capture to `dump`, write the successful generation-level receipt, or commit the generation. Successful capture follows `373–374`; generation manifest creation occurs later at `397–412`. The diagnostic candidates are therefore `capture-N/dump.log` and `capture-N/dump-result.json`, not an assumed g19 manifest or g19 top-level result.

## What terminal upload preserves

`cloud.rs:868–872` records generic supervisor status and fails the tick on failed phase. `1012–1027` records failure, best-effort stop, and publishes durable failure state. `run-controller:15–26` uploads only top-level controller `receipts/*.json` to `recovery/runs/<run>/controllers/<build-id>/`. It does not recursively retrieve or upload worker generation files.

Checkpoint archive capture (`cloud.rs:355–391`, success path `487–518`) requires a successful generation manifest first. The native `archive N` command (`main.rs:14`) includes only `compute` and `generations/gN`. Thus the retained g18 archive cannot contain later g19 capture stderr. The g18 snapshot also predates that failed capture. `archive-computation` (`main.rs:15`) includes compute/control, not generations, and its upload path (`cloud.rs:923–958`) is for completed verified computation.

The local controller-3 receipt directory and private takeover directory were searched narrowly for dump/terminal filenames. No saved `dump.log` or `dump-result.json` was located. The supplied terminal CloudWatch tail and saved supervisor terminal supplement contain only the generic checkpoint error and payload exit status. This is an inventory of those inspected local paths, not a claim about every possible external log or volume.

## Recovery boundary and next read-only options

No existing committed archive identified here contains g19 diagnostic files. They may remain on the worker's existing EBS filesystem, but that retention and current Pod/PVC accessibility have not been independently checked. If an existing, accessible Pod still exposes the volume, the cloud owner can inspect `/work/generations/g19/capture-*/dump-result.json` and `dump.log` through the existing access path. This review does not authorize or perform that access. If no such accessible Pod exists, creating a mount, Pod, controller, or restore is beyond the requested read-only/no-resource-mutation scope. A new mount must not be portrayed as an ordinary read-only retrieval.

The already available controller receipt objects or CloudWatch log can be read again without creating resources, but this source contains no automatic stderr upload to recover by merely fetching the g18 archive. Preserve current terminal receipts and report the underlying CRIU failure cause as unknown unless actual capture diagnostics or independent system evidence become available. Do not attribute it to OOM, scientific algorithm failure, or checkpoint-image size from exit 137 alone.
