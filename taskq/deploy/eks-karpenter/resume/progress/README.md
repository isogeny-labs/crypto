# Native computation phase for CRIU

`isolab-criu-progress` runs the existing native `ecbench` binary as an explicitly
untimed computation phase. It derives a separate L0 specification from the
original request, preserves its workload/arms/seeds/rounds, forces `--cpus none`,
and denies `perf_event_open` in the payload and descendants. It records the
original spec, derived spec and executable hashes in an adjacent recovery intent.
The original L2 input is not modified. Every record from this phase is ineligible
for L2 benchmark admission, including records produced before the checkpoint.

The filter is required because a real CRIU 4.2.1 dump of the measured native
harness failed on `anon_inode:[perf_event]`. It is a workload compatibility
mechanism, **not a security sandbox**. Normal benchmark jobs do not use this
launcher. The CRIU supervisor remains outside the filter, and the workload must
already run as UID 1000 with no effective capabilities. No raw memory is included
in this crate or its test fixtures.

```bash
cargo +1.99.1 test --locked --release --manifest-path \
  taskq/deploy/eks-karpenter/resume/progress/Cargo.toml
cargo +1.99.1 build --locked --release --target x86_64-unknown-linux-musl \
  --manifest-path taskq/deploy/eks-karpenter/resume/progress/Cargo.toml

# Inside the qualified trusted-supervisor pod, after dropping payload identity
# and capabilities; all output, locks and mutable files must be on the PVC:
isolab-criu-progress /usr/local/bin/ecbench /work/input.json /work/computation
```

The launcher execs the native harness. It does not checkpoint on its own, perform
Spot failover, create Pods, or certify restored hardware. Use CRIU plus a coherent
EBS snapshot under the [recovery procedure](../README.md); never rerun this
launcher to simulate restoring an existing computation. It refuses existing
output or intent files.

After a successful restore and native correctness replay, start a distinct normal
benchmark job from the original L2 spec. Re-earn warmup, L2 observations and A/A
admission, and verify that attempt's S3 outputs. Do not copy an earlier host's
admission or combine the restored computation's timing with the new epoch.
