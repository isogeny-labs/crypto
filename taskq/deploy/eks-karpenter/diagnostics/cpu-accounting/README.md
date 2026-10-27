# Native CPU-accounting diagnostic

This bounded probe records `/proc/stat` alongside `wait4` and monotonic wall
timestamps. It diagnoses the excess busy ticks seen in the retained failed
periodic-recovery qualification. It does not grade, admit, retry, or modify an
ecbench result.

Build with the lockfile and pinned toolchain:

```sh
cargo +1.99.1 build --locked --release \
  --manifest-path taskq/deploy/eks-karpenter/diagnostics/cpu-accounting/Cargo.toml \
  --target x86_64-unknown-linux-musl
```

Run the binary in a Restricted EKS pod with at least two assigned CPUs and a new
output directory. Freeze the image digest, source/binary hashes, node bootstrap,
instance type, resources, deadline, and protocol before submission. The binary
pins its observer to the first assigned CPU and each child to the last. With a
two-vCPU SMT allocation these can be siblings; that is diagnostic placement,
not L2 benchmark isolation. A four-vCPU allocation leaves separate whole cores.

Exactly 2,000 child processes alternate between a 20 ms spin and touching 20 MiB
of memory followed by the same spin. Every record retains raw per-component CPU
ticks, the timestamps bracketing both reads, child user/system time, faults and
involuntary switches. Metadata records the kernel, boot arguments, affinity,
thread siblings, tick frequency and workqueue mask. Histograms use the existing
ecbench foreign-tick calculation for comparison; they are not an admission gate.

Upload the full directory and a SHA-256 manifest to the Pod UID's S3 prefix,
including negative results. Verify every downloaded hash. A cleaner counter
diagnostic still requires a separately frozen, unchanged crypto workload with
native L2 grading, replay and A/A checks before a bootstrap change is qualified.
