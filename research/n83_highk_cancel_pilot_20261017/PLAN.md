# N83 high-K cooperative cancellation feasibility pilot

This pilot measures the same previously exposed public point with the
K2816/root11 and K3072/root11 bases under the independently confirmed
two-worker cooperative cancellation implementation. It checks whether both
large bases still construct, reach full rank, produce a four-point target
relation, and recover the same scalar when cancellation is enabled. The
one-target online intervals and full cold process times prioritize a fresh
Tier 1 factorial comparison of base size and target-worker policy.

The fixed run order is K2816 then K3072, each in a new process with empty
caches. The source is `examples/koblitz_orbit_dlp_fast.rs`, SHA-256
`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`.
The target file is
`research/n83_cold_factor_base_20261016/targets.jsonl`, SHA-256
`2c068d2f659588edcdbaa685fc73bb146e2bfee3be123d377d537658f8021c6d`;
the independent answer sidecar is
`research/n83_cold_factor_base_20261016/answers.jsonl`, SHA-256
`b71c5a8bfdae603931fa6c03fc6f6faa3b261288cf787ac682bf8a3e3afe262d`.
The point was already used by the earlier local K3072 source control, so
this pilot is an ordinary-host implementation diagnostic. It does not enter
the fresh-point factor-base selector.

Use the exact root11 dynamic index flags from the cancellation confirmation
plus `KIC_TARGET_THREADS=2` and `KIC_TARGET_CANCEL=1`. Each IC child has a
900-second cap and the existing 15.25 GiB RSS cap. Save the release binary,
its hash, host receipt, exact command and environment, full stdout, stderr,
base dump, target result, process timing and resource receipt. Independent
verification must recompute both enumerated bases, rank, four-point relation,
scalar, packed-index layout, and five exclusive target-online phases.

If either arm times out, exceeds memory, or fails replay, preserve its status
and raw files. No missing value becomes a zero-cost measurement. A later
fresh-point Tier 1 comparison will use new public points and alternate both
factor-base and worker-policy arms with same-point rho.
