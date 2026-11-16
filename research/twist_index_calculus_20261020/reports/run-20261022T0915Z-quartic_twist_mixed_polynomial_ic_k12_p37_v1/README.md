# run-20261022T0915Z-quartic_twist_mixed_polynomial_ic_k12_p37_v1

Raw record: [quartic_twist_mixed_polynomial_ic_k12_p37_v1.json](../../quartic_twist_mixed_polynomial_ic_k12_p37_v1.json) (SHA-256 `e5d8f52ba26180fdf8bbf4103f573f8b53391bb99aceee97375ae041160f7507`).

Source: [quartic_twist_mixed_polynomial_ic.rs](../../../../examples/quartic_twist_mixed_polynomial_ic.rs) (current SHA-256 `cb1caa67733660b09271eca8b0fbc6a72307f28c59a8eafb1bf79e3432f1d24e`). Measurement source: frozen source commit ade6c1916; current source SHA below is a rendering-time snapshot.

## Workload and execution

Frozen protocol: [quartic twist line protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md).

Host and isolation: Intel Xeon Platinum 6975P-C; shared host; L0 exploratory run.

One public known-answer target [271]G; p=37, b0=2, r=1,873,633, seed=0. For the pilot and diagnostic, the first four registered residuals or index 2 are used; for the full producer the existing 61-residual stream is the cap.

Command (with an 8 GiB address-space cap and 1,200 s watchdog): `target/release/examples/quartic_twist_mixed_polynomial_ic research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson --patterns=1,2`.

## Recorded observations

- Status: pass; anchored rank 4/4; recovered scalar 271.
- Processed 7 frozen residuals and retained 4 rank-raising signed relations; all factor logs verified.
- Counted IC: 154,272,224 Fp multiplications; 7,619 group operations. Same-process twist24 rho: 348,941 Fp multiplications; 2,745 group operations.

![Recorded field-work graph](graph.svg)

The graph plots native recorded counts. It does not include native hash, allocator or partition CPU. L0 wall time in the raw record is not isolated. No priced IC/rho or fastest-solver speed claim follows from this run.

[PDF copy](report.pdf).
