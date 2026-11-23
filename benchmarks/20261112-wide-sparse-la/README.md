# 20261112-wide-sparse-la

Wide-modulus sparse linear algebra (filter + block Wiedemann / block Lanczos +
reconstruction). The cost per column was measured against modulus width
(61–255 bits, u64…U256) and size (D = 2¹²…2²⁰) on three RunPod CPU pods, one
per seed, on 2026-11-12/13. Protocol, raw data and analysis:
[`research/ic_techniques_20261012/wide_sparse_la/`](../../research/ic_techniques_20261012/wide_sparse_la/README.md).
The bench record is [`records/20261112T2153Z-wide-sparse-la.json`](../records/20261112T2153Z-wide-sparse-la.json).

Outcome `[pcc:unverified]`:
- The counted size law holds: β = 1.96–1.97 for every arm.
- The frozen width-law test failed. Within one word, the time per operation
  depends on the modulus: for example, U128 `full-bw` at D = 2¹⁶ costs
  3.6 / 4.6 / 7.0 ns at 61 / 89 / 127 bits.
- So the D = 10⁹ extrapolation for the ECC2K-130 order is unsupported.

This entry is not in the admitted `benchmarks/README.md` table. It is a phase
cost with no matched rho, replay certificate or validator report, so the
validation gate does not admit it.

Files: [`REPORT.pdf`](REPORT.pdf), `row_ops_per_col_vs_D.{svg,png}`, `ns_per_row_op_vs_limbs.{svg,png}`.
