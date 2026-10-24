# N83 K1107 root-layout local diagnostic

This is a single disjoint public-point, exploratory check of the current-source
K1107/root11 and K1107/root22 implementations. The fixed order is root11 A,
root22 A, root22 B, root11 B, then one same-point rho run. Every IC child
starts with an empty directory and constructs its own factor base. This probe
tests execution and measures whole-process time, setup, relation, rank and
target phases where the executable reports them. It cannot select a global
factor-base winner or enter the frozen five-arm confirmation selector.

The point is SHA256(`n83-modal-root-layout-probe-20261024-v1` || u32le(0))
reduced to a nonzero subgroup scalar and multiplied by the public generator.
`generate.py` independently checks subgroup membership and exclusion from the
frozen 16-point panel. The target and answer files are sealed before execution.

Build the IC and rho examples from source commit `8bf07aefb3fb2a65248ce379da7284cb58a5581d`
with `cargo build --release --locked`, then pin their SHA-256 values in the run
receipt. Run the four IC children
with `construct:83:1:1107`, rank seed 29, one target, 300-second per-child cap, and the
current-source root-batch flags. Run rho with the same point and the archived
rho configuration, seed 2026102401, 120-second cap. Retain raw stdout, stderr,
result files, base dumps, hashes, process receipts, and local host evidence,
then independently inspect them before reporting any timing. A failed/timeout
child is retained as such. This macOS diagnostic is exploratory under the
existing Linux cgroup host gate even if all children succeed.

The originally prepared Modal launch was rejected by automatic approval review
because it would send project-derived binaries and data to a third-party
service. This protocol runs locally and does not use Modal or its volume.
