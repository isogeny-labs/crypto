# NIST BEST2 seeded validation

Blocked, not running. The card retains the original validation scope without
claiming an unexecuted measurement. The card's executable is the native ecbench
model session; the protocol also specifies the distinct exact-curve CPU/GPU
panels, remaining replay implementation, resource bounds and acceptance gates.
Source/image hashes must be pinned before the card becomes ready. Cost is
unmeasured; no monetary or CPU-hour estimate is asserted.

See [the problem](../../problems/nist-best2-dp-bias-and-gpu-validation.md) and
[the audit](../../research/nist_best2_small_batch_20261027/seeded-audit-v1/README.md).
The model has hashed predicates and two full additions; it does not supply
NIST GPU solve costs. Independent seeded GPU observations, successful-collision
replay and complete accounting remain required. BEST2 stays disabled.


Canonical backlog verifier templates use `{out}`, the runner-chosen shard
output directory. `outputs.verify` now addresses that ecbench session directory. The separate cairn checker retains its `{artifact}` interface.
This template correction changes neither readiness nor the frozen protocol;
no experiment, journal upload or checkpoint action is performed.
