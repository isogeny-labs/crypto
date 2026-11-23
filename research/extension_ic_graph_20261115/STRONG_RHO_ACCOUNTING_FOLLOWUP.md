# Strong rho accounting after the collection trial

The existing comparison must retain its actual strongest reference options
and public input. Improving the two-LP collector does not by itself settle
reference pricing. This source audit records the next accounting dependency;
it selects no new run and changes no published measurement. [pcc:unverified]

| Source path | Existing behavior | Required follow-up |
| --- | --- | --- |
| `jv_rho_strong.rs::measure_fq3_prims` | Measures the IC harness's `Fq3` primitives and labels them as harness prices. | Preserve that reference and add a separately named price for the actual native arithmetic. |
| `Arith::mul_raw`, `Arith::inv`, `Arith::fq_inv` | Native rho uses Barrett arithmetic and its own adjugate inversion. The inversion implementation differs from the IC harness. | Check primitive counts against the actual source, use hand and differential arithmetic controls, and bind the counted formula to that source. |
| `TUNED` | Legacy floating multiplication credits for Karatsuba, specialized squaring and inversion, including an assumed base-inverse credit. | Retain as explicitly scoped legacy credits; do not silently present them as a complete measured bitop/v1 implementation. |
| `rho_tally` | Reconstructs phase counts from explicit walk/setup/verification counters. Some reused arithmetic already emits hooks. | Avoid adding the whole process hook snapshot to reconstructed counts; prove each additional contribution is disjoint. |
| `rng_draws`, `scalar_ops` | Existing counters describe selected high-level operations. | Resolve actual selected uniform draws/rejections and native event coverage under the same declared convention as IC. |

The next implementation should first add meaningful arithmetic and counting
controls, then publish a frozen reference-only validation before another
full primary comparison. Native instruction audits can diagnose missing
counters; they do not turn instruction counts into bitop prices or establish
an independent signed execution verdict. [pcc:unverified]

Sources checked (KB): wrapper status and exact/mismatch rho-pricing queries
are unavailable in this runtime; receipts are
`validation/rho-native-kb-{status,search1,search2}.log`. Read the current native
arithmetic, `measure_fq3_prims`, `rho_tally` and `TUNED`, along with
`docs/units/BITOPS.md` and its explicit distinction between model prices and
instruction audits. No new arithmetic price or parity result is asserted.
[pcc:unverified]
