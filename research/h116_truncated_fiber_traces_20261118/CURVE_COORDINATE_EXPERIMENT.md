# H116 curve-coordinate experiment

Protocol `H116-CURVE-COORDINATE-001` was source-gated at commit
`57b5bfc003` before execution. The producer constructed 24 coefficients with a
fixed-point iteration; the independent verifier reconstructed them with a
coefficient recurrence from the retained TSV only. [pcc:unverified]

## Result

| fixture | p | scale `trace-1` | identities and rescaling | wrong scaling | wrong parameter | field operations (add/mul/inv) |
| --- | ---: | ---: | --- | --- | --- | ---: |
| fp31-r37 | 31 | 25 | 24/24 coefficients | first differs at 4 | first differs at 4 | 81,732 / 81,183 / 5 |
| fp211-r197 | 211 | 14 | 24/24 coefficients | first differs at 4 | first differs at 4 | 81,732 / 81,183 / 5 |
| fp241-r257 | 241 | 225 | 24/24 coefficients | first differs at 4 | first differs at 4 | 81,732 / 81,183 / 5 |
| fp431-r397 | 431 | 34 | 24/24 coefficients | first differs at 4 | first differs at 4 | 81,732 / 81,183 / 5 |
| fp587-r547 | 587 | 40 | 24/24 coefficients | first differs at 4 | first differs at 4 | 81,732 / 81,183 / 5 |

All five cells satisfy the source and target curve equations, target rescaling,
double substitution and the complete normalized x-ratio equality through the
frozen precision. Both precommitted controls are rejected in all five cells.
The operation counts are deterministic fixed-precision diagnostics, not timing
or asymptotic measurements. [pcc:unverified]

The independent verifier reports:

```json
{"verified_cells":5,"curve_identity_cells":5,"rescale_matches":5,"double_substitution_matches":5,"normalized_ratio_matches":5,"wrong_scaling_rejected":5,"wrong_parameter_rejected":5}
```

A temporary copy with the first row's `curve_identity` field changed from
`true` to `false` was rejected with exit code 101. The retained result was not
edited. [pcc:unverified]

## Reproducibility

```text
source-gate commit  57b5bfc003
result TSV SHA-256  693ebaedc341bf0c3c632de7bc7545a0c02e55d8ffc312fe1e211c494ad9681c
verify JSON SHA-256 b18f93d439920a6b532ee07b5838dd3861f165e27a741453d20ca81e4fd6a95f
identity certificate IDC1h692b3e04c6a4f0cd
certificate SHA-256 351f35d72483e906391aa2f2e17e68eb849b3af5ac06f7414d349361de5f42d2
```

The identity certificate seals the polynomial target-rescaling step and points
to the separately audited Lean theorem. It was checked at 32 issuer points and
32 domain-separated checker points over the certificate field. The exact
finite-field arrays remain in `CURVE_COORDINATE_RESULT.tsv`.
[pcc:unverified]

## Evidence boundary and next formal target

This panel instantiates the coordinate equation and scaling laws consumed by
`WeierstrassCoordinateUnit.normalizedXRatio_prefix`. It does not prove that the
two inputs to that theorem are the local parameters of `V-1`. [pcc:unverified]
The next exact
obligation is to derive

```text
a * zNormalized = zScalar  modulo the required prefix
```

from `V+pi=[t]`, the local Frobenius expansion and formal-group subtraction,
then compose that equality with the checked curve-coordinate theorem.
[pcc:unverified]

Sources: Bostan--Morain--Salvy--Schost,
[arXiv:cs/0609020](https://arxiv.org/abs/cs/0609020); Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`; study prior-art receipt
`PRIOR_ART.md`.

