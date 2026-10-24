# Product proof map

Every theorem proves an unconditional natural-number product equality.
For partial rows the product explicitly includes the composite residual.
Lean does not prove primality; the separate PARI certificates do.

| Target | Status | Lean declaration in `SkippedNistFactors` | Identity certificate |
|---|---|---|---|
| `p256-4p-minus1` | complete | `p256_4p_minus1_product` | [IDC1h31218b677856b8b9](identity-certificates/p256-4p-minus1.json) |
| `published_224-twist2` | partial | `published_224_twist2_product` | [IDC1hdb2af39b925576a1](identity-certificates/published_224-twist2.json) |
| `published_256-6n-plus1` | complete | `published_256_6n_plus1_product` | [IDC1hdecb8fce71e2eab7](identity-certificates/published_256-6n-plus1.json) |
| `published_256-7n-minus1` | complete | `published_256_7n_minus1_product` | [IDC1h512190b8fcf5d7fe](identity-certificates/published_256-7n-minus1.json) |
| `published_256-twist2` | complete | `published_256_twist2_product` | [IDC1h969e7da713aca14e](identity-certificates/published_256-twist2.json) |
| `skipped_224_1485-conductor4` | complete | `skipped_224_1485_conductor4_product` | [IDC1h7c5ecb55bf5e2915](identity-certificates/skipped_224_1485-conductor4.json) |
| `skipped_224_5736-twist2` | partial | `skipped_224_5736_twist2_product` | [IDC1he5e12c8967667144](identity-certificates/skipped_224_5736-twist2.json) |
| `skipped_256_876-4n-minus1` | complete | `skipped_256_876_4n_minus1_product` | [IDC1h0ba14f42c133d8da](identity-certificates/skipped_256_876-4n-minus1.json) |
| `skipped_256_876-twist2` | partial | `skipped_256_876_twist2_product` | [IDC1hb6ab9163ca2cf443](identity-certificates/skipped_256_876-twist2.json) |

Checked with Lean 4.35.0 and the repository proof gate: nine theorems,
zero failing, zero trusted; all axiom lists are empty. `lean4checker` was
unavailable. See `runs/lean-gate-final/` after archive extraction.
