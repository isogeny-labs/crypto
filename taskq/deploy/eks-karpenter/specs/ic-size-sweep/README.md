# Frozen one-target index-calculus curve-size panel

The four `n*.json` specs are independent public synthetic elliptic-curve jobs at
20, 24, 28 and 32 bits. Each uses `prime_search` seed 59297 and target seed
20261002, with exactly one target per curve. The candidate is `ic.pipeline`
with a 32-element prime-abscissa factor base and negation-folded MITM oracle;
the matched reference and A/A control are both `rho.negation`. Each job has
one warmup and five measured alternating rounds, requires L2 isolation, and
checks its recovered scalar. The panel validates plumbing and correctness;
online timing excludes setup, so it makes no end-to-end speed claim.

`n32-resumable.json` is the same 32-bit curve, target and arms with a
600-second per-round timeout. CRIU can pause computation during an EBS
checkpoint, so this separate frozen fixture budgets that pause without
changing the ordinary panel's protocol. Its own validation receipt is pending.

After this directory lands on `main`, dispatch **Iso Lab experiment** from
`main` with `size_sweep=true`, `instance_type=c8a.2xlarge`, and `seconds=900`.
For CRIU computation checkpoint and restore, also set `resumable=true`,
`checkpoint_seconds=30`, and `max_restores=2`. The workflow builds the selected
source commit into an immutable ECR image, fetches each frozen spec from that
same commit, submits one uniquely named Job per size to the Spot-only
`cpu-l2-spot` pool, and saves the request and result receipts as an Actions
artifact. A size failure does not suppress the other sizes; the workflow fails
after collecting their outcomes. A resumable request checkpoints computation
and then measures a fresh L2 epoch after any recovery. Its recovery receipt
must show a restore before claiming that worker-loss recovery was exercised.

The first live panel and raw receipts are in
[`../../validation/ic-size-sweep-20261026/`](../../validation/ic-size-sweep-20261026/README.md).
