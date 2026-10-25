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
build request on a two-CPU runner. `attempt-1.json` binds the provider job and
retained artifact to the exact stderr bytes. Protocol v2 changes only the
hosted build CPU limit; the factor-base objects, curve, target, and search
shape remain frozen.

The frozen accounting envelope is 1,800 seconds for public base preparation
and 3,100 seconds per arm for build, import, search, and cleanup: 8,000 seconds
in total. The workflow timeout is 150 minutes to preserve artifacts if an
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

The pinned mathematical source commit is
`cb98e85d1c1c1afdd5000bc70eea00969ee86c6c`; the protocol checkout is the
dispatch commit. The two checkouts are independent so the frozen-source git
cleanliness gate remains meaningful. `protocol.py` reproduces the exact
`protocol.json` bytes; `audit.py` independently validates the 4+2 worker
receipt; `run.py` validates source, archives, preparation, image/cgroup,
per-stage output, and cleanup before it summarizes either arm.
