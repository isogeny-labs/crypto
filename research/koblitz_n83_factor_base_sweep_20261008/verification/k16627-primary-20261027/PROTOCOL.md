# Primary N83 K16627 factor-base construction gate

Freeze one cell of the existing `v2-size-frontier` before execution: curve
`a=0`, subgroup order `2417851639230796216685689`, `public_x_hash` policy,
seed `2026100801`, and 16,627 signed-Frobenius orbit columns. A completed
full-orbit base has exactly 2,760,082 distinct point records. The prior
K4096 object, built under the same curve, policy and seed, is the construction
control; this gate compares object correctness and resource use, not relation
yield or a complete IC runtime.

Use the K4096 control's checked-out source commit
`f61b33c1b1bb98efa22177a615270c160859a605`, whose exporter SHA-256 is
`cf31a5e9e8d5aad35aeb3c73be352992bc4e40ca6cbd425ad0ccf8e14de89340`.
Record the executable SHA-256, immutable container image ID, exact command,
and the `size-frontier-v2.json` digest. Construct into a fresh
external directory with `v2_object_supervisor.py`, one CPU, a 6 GiB Docker
memory limit with zero swap, no network, a read-only repository mount, and a
3,600-second outer and worker wall cap. Preserve stdout, stderr, config,
outer receipt, manifest or cap, container status and compressed object.
Retain a censored or failed attempt under its original status.

Only if construction passes, run `v2-replay-one` on that exact object under a
separate 1,800-second cap. Generic multi-limb arithmetic must verify every
point, subgroup and Frobenius label, orbit closure, and the point-set digest.
Keep the replay receipt and independent compressed-object SHA-256/BLAKE3
hashes. Only after replay passes, publish to the versioned S3 prefix and
verify a fresh download against those hashes. A storage failure leaves the
local construction and replay result intact with publication pending.

The expected full-pair multiset count is 3,809,027,703,403 and the
four-summand smoothness ceiling first reaches one at this K in the frozen
support model. Neither quantity estimates solver throughput. Report
`total_index_calculus_runtime_ms` and `selected_best_total_runtime` as null.
Selection requires natural relation/rank yield, matrix and target stages,
independent scalar replay, all preparation costs, and a same-point rho pair
under the declared benchmark tier.
