# Jacobian-cover component counts: admission pending

The [corrected study](../../research/ic_cover_leads_20261018/README.md) and [report](report.pdf) preserve the component census and proposed follow-up leads. They make no measured speedup claim. The current report corrects the F4 denominator to 100 systems, labels columns with more than one nonzero entry, and treats the proposed F5 estimate as a heuristic.

The component census lacks a matched end-to-end rho reference and a complete `VALIDATION.json`; it is therefore not admitted to the benchmark results table. Admission would require the independently replayed pipeline workload, complete accounting, and the matched reference required by `docs/VALIDATION_GATE.md`. These remain unresolved; no substitute measurement was introduced during review.

The former branch row is retained below verbatim as superseded provenance. Its F4 means, column label, and bound language are superseded by the linked correction and must not be read as the current result.

```text
| 2026-10-18 | [`20261018-jv-cover-cost-breakdown-leads`](20261018-jv-cover-cost-breakdown-leads/) | counted field multiplications of one Nagao decomposition test of `examples/jv_cover.rs` on main after #1717, split by part (p = 101, 120 tests), F4 rows per degree, non-unit columns of the multiplication matrices, and the `--trace` F4 replay against the stock engine (p = 53, 101) | C_cov 1.223M = F4 737k (60 %) + characteristic polynomial 245k (20 %) + combination and eigenvectors 160k (13 %) + normal forms 57k + roots 19k; `M_x6` has 20 of 64 non-unit columns; 124 of 1,014 F4 rows reduce to zero; `--trace` costs 31–35 % more F4 multiplications; leads only, no speedup claimed | 4 vCPU shared VM, L0 (counted cost, load-independent) | [`research/ic_cover_leads_20261018`](../research/ic_cover_leads_20261018/), run-20261018T1900Z |
```
