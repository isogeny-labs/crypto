# Primary N83 K600 six-summand access pilot

This directory freezes one bounded 4+2 relation-access experiment on the
primary `a=0` Koblitz curve. It holds the K600 sequential base fixed and
compares the K600 Gray-prefix and hash residual pools. The earlier hosted 3+2
pair consumed 5,000 conflicts in each arm with zero models; that result does
not establish the behavior of the 4+2 search. The signed-orbit support counts
in `protocol.json` are combinatorial counts, not measured relation yields.

The exporter and worker build stages request two CPUs to fit the hosted ARM
runner. The worker checks the first public corpus trial under a 20,000-conflict,
1,800-second, 32-model cap per arm. The two arms use separate cold builds and
imports. The hosted image is pinned by digest, runs on Linux ARM, and has a
1-CPU/4-GiB/zero-swap search cgroup. Both archives are regenerated from the
pinned source checkout and checked against their existing S3 object identities
and compressed SHA-256 values. The S3 objects live under
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/a0/objects/`.

The first dispatch [40149326167](https://github.com/aburan28/crypto/actions/runs/40149326167)
stopped before regeneration because Docker rejected the original four-CPU
build request on a two-CPU runner. `attempt-1.json` binds the provider job,
the retained eight-member `attempt-1.zip`, and the exact stderr bytes.
Protocol v2 changes only the
hosted build CPU limit; the factor-base objects, curve, target, and search
shape remain frozen.

The corrected [dispatch 40150464971](https://github.com/aburan28/crypto/actions/runs/40150464971)
completed its producer job and uploaded artifact `12584236596`. Its ZIP is
10,283,974 bytes with SHA-256
`cc24715f7db609c05751e66d858d0162009c4f11f7b80a86f82c13be8a5240ae`;
the provider reports expiration at `2027-01-23T01:23:51Z`. The independent
`replay_artifact.py` audit passed for all 51 ZIP members, the pinned source and
protocol, stage commands and cgroups, worker results, and fresh S3 downloads
of all three archives. Its committed `attempt-2/replay-receipt.json` binds the
provider responses and S3 HEAD receipts. Both arms reached the 20,000-conflict
cap on the first public corpus trial with zero models and status
`UNKNOWN_search_cap`. The Gray arm used 97 residual signed orbits (16,102
points), 1,209,935.811 ms of worker wall time, and 380,829,696 peak cgroup
bytes. The hash arm used 600 residual signed orbits (99,600 points),
1,110,506.489 ms, and 639,139,840 bytes. Both used the same 99,600-point
sequential base and byte-identical worker binaries. These measurements do not
rank the factor bases because neither arm yielded a relation.

The frozen accounting envelope is 1,800 seconds for public base preparation
and 3,100 seconds per arm for build, import, search, and cleanup: 8,000 seconds
in total. The corrected run consumed 3,351.969 seconds of observed combined
preparation and arm elapsed time, including 341.534 seconds of preparation;
the conservative charged total remains 8,000 seconds. The workflow timeout
is 150 minutes to preserve artifacts if an
envelope is exceeded. Actual elapsed times and conservative charges are both
recorded. A bounded access result cannot select a best total-runtime factor
base: relation generation, rank, linear algebra, unseen-target online work,
and matched rho accounting remain required.

To resume from the PR branch after reviewing the workflow and protocol:

```sh
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-sixsum-access-20261025/protocol.py
gh workflow run n83-hosted-paired-gate.yml \
  --ref codex/n83-aws-resume-20261024 -f experiment=six_sum_access
gh run list --workflow n83-hosted-paired-gate.yml --branch codex/n83-aws-resume-20261024 --limit 5
```

The run uploads `n83-hosted-sixsum-<run_id>-<attempt>` even on stage failure.
It includes `n83-sixsum-setup/`, all three compressed bases,
`n83-hosted-sixsum/config.json`, each arm's raw stage receipts and worker
`search/config.json` and `search/result.json`, and `outer.json`. A `PASS` or
`UNKNOWN` worker status is only usable after independent replay of the
artifact, including archive bytes, cgroup/cost receipts, and any candidate
group equation. Preserve failed and censored outcomes. The machine-readable
best-total-runtime field remains `null` until the full claim gates pass.

To replay the corrected run from an AWS checkout of this PR, obtain the exact
GitHub artifact ZIP and fetch the three objects anew from S3. The committed
provider files can be passed directly to the replay command. For each object
digest in `protocol.json`, save the S3 GET as `s3-<digest>.jsonl.gz` and the
`aws s3api head-object` JSON as `s3-head-<digest>.json` in one directory. Then
run the following from the repository root with `gh`, `aws`, Python 3.14, and
`blake3==1.0.8` available:

```sh
mkdir -p /tmp/n83-sixsum-replay
gh api repos/aburan28/crypto/actions/artifacts/12584236596/zip \
  > /tmp/n83-sixsum-replay/artifact.zip
for digest in \
  3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0 \
  966a0493c96674f0af6d184615ed05309d08fba5c7342957b4186cd50366dc47 \
  b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e
do
  aws s3 cp \
    "s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/a0/objects/$digest.jsonl.gz" \
    "/tmp/n83-sixsum-replay/s3-$digest.jsonl.gz"
  aws s3api head-object --bucket crypto-autoresearcher \
    --key "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/a0/objects/$digest.jsonl.gz" \
    > "/tmp/n83-sixsum-replay/s3-head-$digest.json"
done
sha256sum /tmp/n83-sixsum-replay/artifact.zip
python3.14 research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-sixsum-access-20261025/replay_artifact.py \
  --artifact-zip /tmp/n83-sixsum-replay/artifact.zip \
  --provider-jobs research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-sixsum-access-20261025/attempt-2/provider_jobs.json \
  --artifact-metadata research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-sixsum-access-20261025/attempt-2/artifact_metadata.json \
  --s3-dir /tmp/n83-sixsum-replay \
  --output /tmp/n83-sixsum-replay/new-replay-receipt.json
```

The auditor fails on archive-byte, source, protocol, provider, resource,
outcome, or candidate-group-equation mismatch. `attempt-2/replay-receipt.json`
is the exact receipt from the retained ZIP and S3 responses; the ZIP remains
at the linked GitHub artifact rather than in this Git tree.

The pinned mathematical source commit is
`cb98e85d1c1c1afdd5000bc70eea00969ee86c6c`; the protocol checkout is the
dispatch commit. The two checkouts are independent so the frozen-source git
cleanliness gate remains meaningful. `protocol.py` reproduces the exact
`protocol.json` bytes; `audit.py` independently validates the 4+2 worker
receipt; `run.py` validates source, archives, preparation, image/cgroup,
per-stage output, and cleanup before it summarizes either arm.
