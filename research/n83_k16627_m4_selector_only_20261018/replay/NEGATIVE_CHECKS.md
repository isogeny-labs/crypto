# Separate relation-replay negative checks

The Rust verifier read the retained compressed object and each arm's original worker receipt, returning `PASS_RELATION_REPLAY` for A1, B and A2 in [a1.json](a1.json), [b.json](b.json) and [a2.json](a2.json). For each arm, `jq` changed the first `x_decimal` entry to `"0"` in a copied worker receipt, and separately changed `relation.dense_decimal_blake3` to 64 zero hex digits. Neither alteration changed the original worker receipt.

| Arm | Changed x result | Changed row result |
| --- | --- | --- |
| A1 full choice | exit 1, `worker x block differs from independent group addition` | exit 1, `relation row hash differs from independent phase-zero row` |
| B selectors only | exit 1, `worker x block differs from independent group addition` | exit 1, `relation row hash differs from independent phase-zero row` |
| A2 full choice | exit 1, `worker x block differs from independent group addition` | exit 1, `relation row hash differs from independent phase-zero row` |

The six altered receipts and corresponding stderr/stdout files are retained in this directory. Each rejected invocation left no success JSON at its requested output path. These checks exercise the verifier's x and row guards; they do not replay the worker's SAT assignment bits, which the worker did not export. The verifier shares the library's field and curve arithmetic with the worker and has not been run on a second host.
