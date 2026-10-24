# N83 19-base grid on one AWS Spot Tier 1 allocation

This provider variant executes the exact 84-event schedule, public point,
candidate manifests, source files, IC flags, rho seeds and process caps of
`research/n83_nearby_grid_tier1_20261024`. It is a fresh measurement on an
AWS `g5g.metal` Spot instance in `us-east-1b`; it does not combine timings
with the queued GitHub-hosted run. The comparison remains one target per
child, with cold IC and one-target online times recorded separately.

The host must expose its IMDSv2 instance identity document and PKCS7
signature, `instance-life-cycle=spot`, ARM64 architecture, the expected git
commit, at least two affinity CPUs, 5 GiB memory and 4 GiB free disk. The
controller later binds an independent AWS `DescribeInstances` response to
the archived host record. Without that response the aggregate stays
`BOUNDED_UNKNOWN`.

The CPU quota chain is recorded from the process cgroup through the cgroup
v2 root before and after each child. Every level must be unbounded. When
that is true, the kernel may omit `nr_throttled`; an explicit nonzero
throttle counter still fails. The per-child steal-time limit is 0.5% of
visible CPU wall time, measured with the recorded clock tick rate. OOM,
unstable quota or memory limits, over-cap RSS, missing telemetry, failed
replay, or any K1107/rho A/A spread over 20% leaves the screen
`BOUNDED_UNKNOWN`. All children and reasons remain in the raw output.

The approved launch envelope is one `g5g.metal` Spot instance for at most
165 minutes and $5. `launch_spot.py` fixes a $1.70/hour maximum bid,
one-time Spot request, encrypted 80 GiB root volume, IMDSv2, no inbound
network, and a 160-minute shutdown deadline. The bootstrap also shuts down
when it finishes. The instance role reads only the task's S3 prefix and
has SSM access. An interrupted or capped run preserves its partial rows;
it is never silently restarted as the same run.

From the repository root, the local fixture preflight and host gate tests are:

```sh
python3 -m research.n83_nearby_grid_aws_tier1_20261024.run --preflight
python3 -m unittest research.n83_nearby_grid_aws_tier1_20261024.test_host
```

After local validation, archive the exact committed source and run the
read-only launch preflight. `--execute` uploads the bundle once and launches
the capped host. Keep the launch receipt outside the source tree:

```sh
git archive --format=tar HEAD | zstd -T4 -q -o /absolute/path/source.tar.zst
python3 -m research.n83_nearby_grid_aws_tier1_20261024.launch_spot \
  --bundle-path /absolute/path/source.tar.zst --commit <40hex-commit> \
  --receipt /absolute/path/launch_receipt.json
python3 -m research.n83_nearby_grid_aws_tier1_20261024.launch_spot \
  --bundle-path /absolute/path/source.tar.zst --commit <40hex-commit> \
  --receipt /absolute/path/launch_receipt.json --execute
```

The bootstrap verifies the archive SHA-256 and embedded git commit,
rebuilds the exact source natively for ARM64, runs the release library and
example tests, and uploads raw results and logs. Its measured command is:

```sh
python3 -m research.n83_nearby_grid_aws_tier1_20261024.run \
  --binary /absolute/path/koblitz_orbit_dlp_fast \
  --rho-binary /absolute/path/n83_rho_optimization_eval \
  --output /absolute/path/results
```

After the raw results are back on the controller, bind the provider receipt
and replay without changing the measurements. Verify the instance has
terminated and preserve the S3 and local artifact hashes:

```sh
python3 -m research.n83_nearby_grid_aws_tier1_20261024.bind_provider \
  /absolute/path/results --profile ecc-challenges-new \
  --bundle-path /absolute/path/source.tar.zst
python3 -m research.n83_nearby_grid_aws_tier1_20261024.verify \
  /absolute/path/results
```

The result remains a screen of 19 contiguous signed-Frobenius orbit bases.
Fresh 16-point confirmation and a wider factor-base family comparison are
separate acceptance steps.
