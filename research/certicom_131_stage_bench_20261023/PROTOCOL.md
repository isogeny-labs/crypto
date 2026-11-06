# Certicom 131-bit stage benchmark protocol

Frozen before the timed run on 2026-10-23. Source revision:
`d7daaa6b42e7b5209e78c2407b2eaf35da25fbe4` plus
`examples/certicom_131_stage_bench.rs` in this branch.

## Requested scope

Measure our rho and elliptic-curve index-calculus code on ECC2K-130,
ECC2-131, and ECCp-131. Keep the target identities and workload sizes
fixed. Report complete solver throughput separately from arithmetic stages.

## Executable measurements

The new Rust example uses the published curve parameters in the registry.
It checks each generator is on curve and has the advertised subgroup order.
It uses a planted `Q = [73]G` for a repeatable 16-way rho walk. One counted
step partitions by the low four x bits, adds the selected jump, and updates
both scalar coefficients. One index-calculus query constructs the third
Semaev polynomial in its final x coordinate and solves its quadratic in the
full 131-bit field. Inputs are built before timing. Positive-control
three-point relations must recover the expected root on each curve.

Run five samples with 10,000 rho steps and 1,000 S3 root queries per
sample and target, in the example's fixed order. No result is a complete
rho or index-calculus discrete-log solve: distinguished-point storage,
collision recovery, factor-base construction, relation yield, linear
algebra, descent, and final logarithm verification are outside these timed
stages.

Use `tools/isolated_bench.py run` with a full physical-core CPU pair, and
retain its admission and contention record. Refuse wall-time promotion if
the isolation preflight fails or the run is contended. The resulting CSV
contains every sample, operation count, seconds, and final checksum. Report
median operations/s and the full sample range. An unisolated correctness
smoke may run first; it is not timing evidence.

The GPU arm uses the existing ECC2K-130 client on an ephemeral G7 GPU host,
with `make test` before five complete-iteration samples using
`benchmarks/rtx-pro4500/run.sh`. ECC2-131 and ECCp-131 have no matching GPU
walk clients in this revision. The GPU job is independently gated: no GPU
rate is imputed from CPU results or from the archived ECC2K-130 receipt.

## Pre-run access outcomes

- Spot `g7.2xlarge` launch returned `MaxSpotInstanceCountExceeded`.
- On-demand `g7.2xlarge` launch returned `UnauthorizedOperation`: an
  explicit deny in the `SpotOnlyEC2Launch` identity policy.

Both launches failed before an instance ran. The GPU arm remains open.
The Spot command, issued from this worktree, was:

```sh
/home/ec2-user/crypto/taskq/deploy/ephemeral/ephemeral run --detach \
  --name ecc2k130-g7-benchmark-20261023 --type g7.2xlarge --spot \
  --ttl 90m --toolchain build --paths ecc2k130 --ref origin/main \
  --artifact ecc2k130/benchmarks/rtx-pro4500/result.json -- \
  'cd ecc2k130 && make test && REPEATS=5 bash benchmarks/rtx-pro4500/run.sh'
```

The on-demand retry used the identical command without `--spot`. The
launcher failed at `RunInstances` in both attempts; there is no job ID or
remote output to fetch.

## Pre-timing amendment

The unisolated correctness smoke `certicom_131_stage_bench 5 5 1` passed
all three target fixtures and positive S3 controls. Its five-operation
durations were 0.025–0.518 ms, so the initially stated 10,000/1,000
counts would yield subsecond samples for most rows. Before any admitted
timing run, increase the fixed workload to **200,000 rho steps and 50,000
S3 root queries per sample**, keeping five samples and every other rule
unchanged. The smoke is not performance evidence.

## Isolation diagnostic amendment

The default isolation preflight refused the first timed run: other processes
used 0.95 CPU seconds in 2 seconds, above its 0.20 CPU-second limit. It
created no samples. A separate diagnostic run may use the tool's
`--max-other-cpu 0.5` option, with the threshold and actual other-process
usage kept in the record. Its rates remain exploratory and do not satisfy
the strict isolation gate frozen above.

After restoring test fixtures, the same strict command was retried once.
It again failed before samples: 2.84 other-process CPU seconds in the
2-second window versus the unchanged 0.20 limit. The raw rejection is
`strict-admission.stderr`; `strict-samples.csv` is empty. No strict rate
has been promoted.
