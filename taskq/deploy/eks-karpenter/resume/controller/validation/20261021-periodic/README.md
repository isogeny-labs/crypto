# Periodic checkpoint intent repair

Review baseline: `bfd01bb9b6c635703b11da9ee225d41c7f5c51d6` (PR #2377).
This is a controller correctness repair, with no performance claim.

After generation 1 committed and the supervisor continued, its running status
retained generation 1. On the next scheduled checkpoint the previous condition
matched any committed generation and returned to Running without requesting
generation 2. The controller now persists `checkpoint_generation` on entry to
Checkpointing and retains it through commit/continue reconciliation. The saved
next-generation counter can advance without changing the pending intent.

The regression serializes and reloads controller state before the checkpoint
RPC, after committing a generation, and after continuing before saving Running.
It requires a second checkpoint despite the supervisor reporting generation 1,
then commits generation 2. Substituting the original any-committed-generation
condition into the reviewed method fails precisely at that second interval;
`regression-old-condition.log` retains this deliberate failing run. This is a
condition-level negative control, not a build of the entire original revision.

Validation ran in the cached `rust:1.93.1-bookworm` Linux ARM64 Docker image,
with two CPUs available to the container. These are correctness checks on a
shared macOS ARM64 host through a Linux VM, not isolated timing measurements.

- `cargo test --locked --manifest-path taskq/deploy/eks-karpenter/resume/controller/Cargo.toml`:
  18 passed, none failed or ignored; `linux-tests.log`.
- `cargo clippy --locked --manifest-path taskq/deploy/eks-karpenter/resume/controller/Cargo.toml --all-targets -- -D warnings`:
  passed after installing the image's missing Clippy component; `linux-clippy.log`.
- `cargo fmt --manifest-path taskq/deploy/eks-karpenter/resume/controller/Cargo.toml -- --check`
  and patch whitespace checks passed.

Earlier attempts remain available: macOS compilation rejected the Linux-only
supervisor syscalls; the first Linux suite passed 17 tests but its relative-file
durability test encountered the read-only source mount. The final Linux run used
a writable isolated checkout. The first Clippy attempt lacked the component.
No code was changed to hide these environment errors.

Older Running states acquire the intent at their next checkpoint. Older states
already in Checkpointing without an intent are rejected explicitly because the
controller cannot distinguish an unissued checkpoint from a completed continue.
The original deadline, request identity, instance fence, readback requirements,
and fresh benchmark epoch rules remain enforced.

A live multi-generation CRIU run and current-head hosted CI are still required
for their respective qualification claims. This repair did not launch an AWS
worker or duplicate the existing owner's Action dispatch. No throughput,
recovery latency, or physical x86/CUDA result is inferred from these unit tests.
